// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
 
import PackageDescription
 
let package = Package(
    name: "Keycode",
    products: [
        .library(
            name: "Keycode",
            type: .static,
            targets: ["Keycode"]),
    ],
    dependencies: [],
    targets: [
        .target(
            name: "Keycode",
            dependencies: []
        ),
    ]
)