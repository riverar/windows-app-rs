[![Build and Test](https://github.com/microsoft/windows-app-rs/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/microsoft/windows-app-rs/actions)

# Rust for the Windows App SDK
The `windows-app` crate lets you call any [Windows App SDK](https://github.com/microsoft/WindowsAppSDK) (formerly known as Project Reunion) API using code generated from the metadata describing the API. It is powered by the [windows](https://github.com/microsoft/windows-rs) crate.

## Release channel coverage
The Windows App SDK is delivered via [three release channels](https://docs.microsoft.com/en-us/windows/apps/windows-app-sdk/release-channels)—experimental, preview, and stable. The `windows-app` crate currently targets APIs available in the preview and stable channels.

## Getting started
Add the following to your Cargo.toml file:

```toml
[build-dependencies.windows]
version = "0.26"
features = ["build"]

[dependencies.windows-app]
version = "0.2"
features = [
    "Foundation",
    "WindowsAppSdk_Foundation",
    "Windows_System_Power"
]
```

Make use of any Windows App SDK APIs as needed.

```rust
use ::windows_app::*;
use ::windows_app::Microsoft::Windows::System::Power::*;

fn main() -> ::windows::runtime::Result<()> {
    bootstrap::initialize()
        .and_then(|_| {
            println!(
                "Remaining charge: {}%",
                PowerManager::RemainingChargePercent()?
            );
            Ok(())
        })
        .and_then(|_| bootstrap::uninitialize())
}
```
