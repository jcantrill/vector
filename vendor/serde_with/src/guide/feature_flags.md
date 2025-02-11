# Available Feature Flags

This crate has the following features which can be enabled.
Each entry will explain the feature in more detail.

1. [`base64`](#base64)
2. [`chrono`](#chrono)
3. [`guide`](#guide)
4. [`hex`](#hex)
5. [`json`](#json)
6. [`macros`](#macros)

## `base64`

The `base64` feature enables serializing data in base64 format.

This pulls in `base64` as a dependency.

## `chrono`

The `chrono` feature enables integration of `chrono` specific conversions.
This includes support for the timestamp and duration types.

This pulls in `chrono` as a dependency.

## `guide`

The `guide` feature enables inclusion of this user guide.
The feature only changes the rustdoc output and enables no other effects.

## `hex`

The `hex` feature enables serializing data in hex format.

This pulls in `hex` as a dependency.

## `json`

The `json` features enables JSON conversions from the `json` module.

This pulls in `serde_json` as a dependency.

## `macros`

The `macros` features enables all helper macros and derives.
It is enabled by default, since the macros provide a usability benefit, especially for `serde_as`.

This pulls in `serde_with_macros` as a dependency.
