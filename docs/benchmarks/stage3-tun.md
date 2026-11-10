# Stage 3: negotiated receive-side TUN coalescing

Evaluation in progress. No throughput or resource improvement is claimed yet.
Frozen baseline: merged PR #7, `f26fd3d741d125037797ec9953de1d50ebd93c8a`.

## Descriptor contract

Existing raw `IFF_TUN | IFF_NO_PI` descriptors keep their existing ABI and packet
delivery. An integrating Linux caller can create a **new, exclusively owned**
`IFF_TUN | IFF_NO_PI | IFF_VNET_HDR` descriptor, call
`linux::prepare_receive_coalescing(fd)`, then attach it normally. Discard the new
descriptor if preparation fails: its setup ioctls are not transactional. The
helper rejects raw descriptors before changing settings. It sets ten-byte,
little-endian virtio headers and disables kernel-to-userspace offloads, so
outbound packets remain ordinary fully checksummed IP. Attachment checks the
negotiated header ABI; reads fail closed on unexpected outbound GSO/checksum
metadata. Android, Apple and Windows retain ordinary packet delivery.

The implementation combines only consecutive checksum-valid TCP data with
matching flow, ACK, window, options and IP attributes; sequence numbers must be
contiguous. IPv4 requires a plain header and DF; IPv6 requires TCP without
extension headers. ECN, control packets, fragments, flow/option changes, invalid
checksums and other unsupported cases retain ordinary delivery. A PSH or short
segment ends a group. A group contains at most 32 original packets and 65,535
IP bytes. No reordering, delayed flush, extra packet storage or per-group heap
allocation is introduced.

One `writev` submits metadata, the updated header and immutable queued payloads.
Only an atomic successful write acknowledges originals. Backpressure preserves
all originals; unsupported aggregate submission disables aggregation and retries
ordinary delivery. New counters report TUN write attempts, accepted originals,
coalesced originals, maximum group and offload fallbacks. These diagnose behavior,
not performance by themselves.

## Controlled comparison

The existing download runners accept `tun_mode=compare`. Baseline and A/A clients
use raw TUN; the candidate uses explicitly negotiated vnet. Peer and load binaries
remain fixed baseline builds with raw peer TUN. The same fixture overlay is
injected into both source archives; no candidate API is linked into the baseline.
Recorded client descriptor flags must match the planned mode. Candidate samples
must actually coalesce, stay within the group bound and have zero offload fallback.
Candidate smoke coverage precedes timed samples. Normal release timing remains
separate from frame-pointer profiling.

The setup follows the Linux [TUN ioctl implementation](https://github.com/torvalds/linux/blob/v6.12/drivers/net/tun.c),
[virtio metadata conversion](https://github.com/torvalds/linux/blob/v6.12/include/linux/virtio_net.h)
and [virtio header ABI](https://github.com/torvalds/linux/blob/v6.12/include/uapi/linux/virtio_net.h).
Real supported-kernel downloads, rather than metadata unit tests, must establish
end-to-end checksum and segmentation correctness.
