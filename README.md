# rango-sdk

Unofficial Rango SDK for Rust

## Usage

First you need to initialize your client:

```rust
let rango = Client::new(
        "put-a-device-id-for-your-client",
        "YOUR_API_KEY",
        None,
    );
```

Then you can use available methods to interact with rango.

Examples can be found [here](https://github.com/yeager-eren/rango-sdk-rs/tree/main/examples).

## Status

### Basic API Checklist

|Title| Status |
|--|--|
| Meta (chains,swappers,messaging_protocols) | ✅ |
| `/meta` | ✅ |
| `/quote` | ✅ |
| `/is-approved` | ✅ |
| `/status` | ✅ |
| `/balance` | ✅ |
| `/swap` | ✅ |
| `/report-tx` | ❌ |

### Multi API Checklist

Not implemented.
