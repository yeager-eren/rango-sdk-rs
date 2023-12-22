# rango-sdk
Unofficial Rango SDK for Rust

## Usage 

1. Initialize a client:
```rust
let rango = Client::new(
        "put-a-device-id-for-your-client",
        "YOUR_API_KEY",
        None,
    );
```

2. Use the methods (e.g. Getting meta):
```rust
let result = rango.meta.chains().await;
```

## Examples

Examples can be found [here][examples].

[examples]: https://github.com/yeager-eren/rango-sdk-rs/tree/main/examples


## Roadmap

### Basic API Checklist


|Title| Status |
|--|--|
| Meta (chains,swappers,messaging_protocols) | ✅ |
| `/meta` | ❌ |
| `/quote` | ✅ |
| `/is-approved` | ❌ |
| `/status` | ❌ |
| `/balance` | ❌ |
| `/swap` | ❌ |
| `/report-tx` | ❌ |


### Multi API Checklist

TBA