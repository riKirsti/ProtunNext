import Darwin
import Foundation
@preconcurrency import NetworkExtension

#if !arch(arm64)
#error("ProtunNext Apple builds require ARM64")
#endif

/// Public NetworkExtension packets cross an owned datagram socket pair. This
/// never extracts a private utun descriptor from NEPacketTunnelFlow.
/// All mutable state belongs to queue; @unchecked Sendable documents that boundary.
public final class PacketFlowBridge: @unchecked Sendable {
    private let flow: NEPacketTunnelFlow
    private let queue = DispatchQueue(label: "ProtunNext.packet-flow", qos: .userInitiated)
    private let descriptor: Int32
    private var native: Int32?
    private var reader: DispatchSourceRead!
    private var writer: DispatchSourceWrite!
    private var writerActive = false
    private var stopped = false
    private var started = false
    private var readPending = false
    private var pending: [NEPacket] = []
    private var next = 0
    private var scratch = [UInt8](repeating: 0, count: 65_540)
    private let failed: @Sendable () -> Void

    public init(flow: NEPacketTunnelFlow, failed: @escaping @Sendable () -> Void) throws {
        self.flow = flow
        self.failed = failed
        var pair: [Int32] = [-1, -1]
        guard socketpair(AF_UNIX, SOCK_DGRAM, 0, &pair) == 0 else { throw Self.posixError() }
        do {
            for fd in pair {
                guard fcntl(fd, F_SETFD, FD_CLOEXEC) == 0,
                    fcntl(fd, F_SETFL, O_NONBLOCK) == 0 else { throw Self.posixError() }
                var yes: Int32 = 1
                guard setsockopt(fd, SOL_SOCKET, SO_NOSIGPIPE, &yes, socklen_t(MemoryLayout<Int32>.size)) == 0
                    else { throw Self.posixError() }
                // Preserve the native adapter's maximum IP packet size even
                // when Darwin's default datagram send buffer is much smaller.
                var capacity: Int32 = 256 * 1024
                for option in [SO_SNDBUF, SO_RCVBUF] {
                    guard setsockopt(fd, SOL_SOCKET, option, &capacity, socklen_t(MemoryLayout<Int32>.size)) == 0
                        else { throw Self.posixError() }
                }
            }
        } catch {
            pair.forEach { _ = Darwin.close($0) }
            throw error
        }
        descriptor = pair[0]
        native = pair[1]
        reader = DispatchSource.makeReadSource(fileDescriptor: descriptor, queue: queue)
        writer = DispatchSource.makeWriteSource(fileDescriptor: descriptor, queue: queue)
        reader.setEventHandler { [weak self] in self?.receive() }
        writer.setEventHandler { [weak self] in self?.sendPending() }
        // Closing in the cancellation handler prevents descriptor-reuse races.
        let owned = descriptor
        reader.setCancelHandler { _ = Darwin.close(owned) }
        reader.resume()
    }

    /// Transfer exactly once to Connection.unixConnect; that API consumes the fd.
    /// Call before start. If never transferred, this bridge closes it on teardown.
    public func takeNativeDescriptor() -> Int32? {
        queue.sync {
            guard !started && !stopped else { return nil }
            let fd = native
            native = nil
            return fd
        }
    }
    public func start() {
        queue.async {
            guard !self.started && !self.stopped else { return }
            self.started = true
            self.readBatch()
        }
    }
    /// Stop the native Connection first, then stop this bridge. No router or
    /// system routes/settings are owned or modified here.
    public func stop() { queue.async { self.stopOnQueue() } }
    private static func posixError() -> NSError {
        NSError(domain: NSPOSIXErrorDomain, code: Int(errno))
    }
    private func stopOnQueue() {
        guard !stopped else { return }
        stopped = true
        pending.removeAll()
        if !writerActive { writer.resume(); writerActive = true }
        writer.cancel()
        reader.cancel()
        if let fd = native { _ = Darwin.close(fd); native = nil }
    }
    private func fail() {
        stopOnQueue()
        // Host recovery may join native threads. Keep it outside the packet queue.
        DispatchQueue.global(qos: .utility).async(execute: failed)
    }
    private func readBatch() {
        guard !stopped && !readPending && pending.isEmpty else { return }
        readPending = true
        // One outstanding public packet-flow read, and one bounded pending batch.
        flow.readPacketObjects { [weak self] packets in
            guard let self else { return }
            self.queue.async {
                guard !self.stopped else { return }
                self.readPending = false
                guard packets.count <= 4096,
                    packets.reduce(0, { $0 + $1.data.count }) <= 4 * 1024 * 1024
                    else { self.fail(); return }
                self.pending = packets
                self.next = 0
                self.sendPending()
            }
        }
    }
    private func sendPending() {
        guard !stopped else { return }
        while next < pending.count {
            let packet = pending[next]
            let family = packet.protocolFamily
            guard let first = packet.data.first, packet.data.count <= 65_535,
                (family == AF_INET && first >> 4 == 4 && packet.data.count >= 20)
                    || (family == AF_INET6 && first >> 4 == 6 && packet.data.count >= 40)
                else { fail(); return }
            var header = UInt32(family).bigEndian
            let sent = withUnsafeBytes(of: &header) { prefix in
                packet.data.withUnsafeBytes { bytes in
                    withUnsafeTemporaryAllocation(of: iovec.self, capacity: 2) { vectors in
                        vectors[0] = iovec(iov_base: UnsafeMutableRawPointer(mutating: prefix.baseAddress), iov_len: 4)
                        vectors[1] = iovec(iov_base: UnsafeMutableRawPointer(mutating: bytes.baseAddress), iov_len: bytes.count)
                        return Darwin.writev(descriptor, vectors.baseAddress!, 2)
                    }
                }
            }
            if sent == packet.data.count + 4 { next += 1; continue }
            if sent < 0 && errno == EINTR { continue }
            if sent < 0 && (errno == EAGAIN || errno == EWOULDBLOCK) {
                if !writerActive { writerActive = true; writer.resume() }
                return
            }
            fail(); return
        }
        pending.removeAll(keepingCapacity: true)
        if writerActive { writerActive = false; writer.suspend() }
        readBatch()
    }
    private func receive() {
        guard !stopped else { return }
        var packets: [NEPacket] = []
        packets.reserveCapacity(64)
        for _ in 0..<64 {
            let count = scratch.withUnsafeMutableBytes { Darwin.recv(descriptor, $0.baseAddress, $0.count, MSG_DONTWAIT) }
            if count < 0 && errno == EINTR { continue }
            if count < 0 && (errno == EAGAIN || errno == EWOULDBLOCK) { break }
            // A valid maximum IP datagram plus its internal AF header is 65539.
            guard count >= 5 && count <= 65_539 else { fail(); return }
            let family = scratch.prefix(4).reduce(UInt32(0)) { ($0 << 8) | UInt32($1) }
            guard (family == UInt32(AF_INET) && scratch[4] >> 4 == 4 && count >= 24)
                || (family == UInt32(AF_INET6) && scratch[4] >> 4 == 6 && count >= 44)
                else { fail(); return }
            let data = scratch.withUnsafeBytes { Data(bytes: $0.baseAddress!.advanced(by: 4), count: count - 4) }
            packets.append(NEPacket(data: data, protocolFamily: sa_family_t(family)))
        }
        if !packets.isEmpty && !flow.writePacketObjects(packets) { fail() }
    }
    deinit {
        if !stopped {
            if !writerActive { writer.resume() }
            writer.cancel()
            reader.cancel()
            if let fd = native { _ = Darwin.close(fd) }
        }
    }
}
