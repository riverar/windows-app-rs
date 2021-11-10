[![Build and Test](https://github.com/microsoft/windows-app-rs/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/microsoft/windows-app-rs/actions)

# Rust for the Windows App SDK
The `windows-app` crate lets you call any [Windows App SDK](https://github.com/microsoft/WindowsAppSDK) (formerly known as Project Reunion) API using code generated from the metadata describing the API. It is powered by the [windows](https://github.com/microsoft/windows-rs) crate.

It's early days, but this crate is meant to make it much easier to use the Windows App SDK from Rust. As this new set of APIs requires bootstrapping and various other hooks to get it up and running, using only the `windows` crate—while possible—is a little more cumbersome for these new APIs.

As [WinUI 3](https://microsoft.github.io/microsoft-ui-xaml/) is a large part of the Windows App SDK, one goal is to support WinUI 3 app development.

## Getting started
Add the following to your Cargo.toml file:

```toml
[dependencies.windows-app]
version = "TBD"
features = [
    "TBD"
]
```

TBD: Bootstrapping