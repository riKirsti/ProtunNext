// swift-tools-version: 6.0
import PackageDescription
let package = Package(
    name: "ProtunNextPacketFlow",
    platforms: [.macOS("26.0"), .iOS("26.0")],
    products: [.library(name: "ProtunNextPacketFlow", targets: ["ProtunNextPacketFlow"])],
    targets: [.target(name: "ProtunNextPacketFlow")]
)
