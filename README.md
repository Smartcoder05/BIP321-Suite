# bip321-suite

A composable Rust library for parsing, building, and resolving
Bitcoin payment URIs as defined in BIP-321.

Built for wallet developers who need a flexible, spec-compliant
foundation for handling Bitcoin payment instructions — without
being locked into a specific payment method or network.

---

## Why This Library

Most BIP-321 implementations are parsers and nothing more.
This library is different:

- **Composable by design** — two trait extension points let you
  plug in your own behaviour without modifying the library
- **Parser and builder** — parse URIs into structured data and
  build them from scratch with full round-trip support
- **Proof of Payment** — full PoP implementation including
  callback URI assembly and scheme validation
- **Extend at parse time** — implement `PaymentInstructionHandler`
  to handle any payment instruction type your wallet needs,
  including future ones not yet in the spec
- **Resolve on your terms** — implement `PaymentResolver` to
  process payment instructions exactly how your wallet needs them,
  with built-in network validation
- **rust-bitcoin integration** — proper address validation and
  type-safe address handling throughout

---

## Project Structure

```
bip321-suite/
├── Cargo.toml           ← workspace root
├── README.md
└── src/
    ├── bip321/          ← core library
    │   ├── Cargo.toml
    │   ├── src/
    │   │   ├── lib.rs
    │   │   ├── types.rs
    │   │   ├── parser.rs
    │   │   ├── builder.rs
    │   │   ├── pop.rs
    │   │   └── error.rs
    │   └── tests/       ← integration tests (31 test cases)
    │       ├── valid_uris.rs
    │       ├── invalid_uris.rs
    |       ├── network.rs
    │       └── pop_callbacks.rs
    ├── silent-payment/  ← BIP-352 address validation
    ├── payjoin/         ← PayJoin URL validation
    └── cli/             ← command line interface
```

Each crate is independently usable. A wallet that only needs
the core parser can depend on `bip321` alone without pulling
in silent payment or payjoin dependencies.

---

## Architecture

### Two Extension Points

This library is designed around two traits that give callers
full control over payment instruction handling.

#### `PaymentInstructionHandler`

Called during parsing when the parser encounters a query parameter
it doesn't recognise. Implement this trait to extend the parser
with new payment instruction types without modifying the library.

```rust
#[derive(Default)]
struct MyWalletHandler {
    ark_address: Option<String>,
}

impl PaymentInstructionHandler for MyWalletHandler {
    fn is_supported_key(&self, key: &str) -> bool {
        key == "ark"
    }
    fn handle(&mut self, key: &str, value: &str) -> Result<(), URIError> {
        self.ark_address = Some(value.to_string());
        Ok(())
    }
    fn is_empty(&self) -> bool {
        self.ark_address.is_none()
    }
}

// You can parse like this
let uri = "bitcoin:?ark=ark1p...".parse::<BitcoinUri<MyWalletHandler>>()?;
// or like this
let uri: BitcoinUri<MyWalletHandler> = "bitcoin:?ark=ark1p...".parse()?;
// uri.extras contains the ark address
```

#### `PaymentResolver`

Called after parsing when the caller needs to act on payment
instructions. Implement this trait to define exactly what your
wallet does with each instruction type.

```rust
struct MainnetResolver;

impl PaymentResolver for MainnetResolver {
    type Output = String;

    fn supports(instruction: &PaymentInstruction) -> bool {
        matches!(instruction, PaymentInstruction::Onchain(_))
    }

    fn resolve(instruction: &PaymentInstruction) -> Option<String> {
        if let PaymentInstruction::Onchain(addr) = instruction {
            Some(addr.assume_checked().to_string())
        } else {
            None
        }
    }

    fn network() -> Option<Network> {
        Some(Network::Bitcoin)
    }
}

let uri: BitcoinUri<NoopHandler> = "bitcoin:1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".parse()?;
let addresses = uri.resolve_instructions::<String, MainnetResolver>();
```

Both traits have a default no-op implementation — `NoopHandler` —
for callers who just need basic parsing without custom behaviour.

---

## Proof of Payment

This library fully implements the `pop=`.

Proof of Payment solves a real problem for wallet developers —
how does the app that initiated a payment know it actually went
through? For on-chain payments you can watch the blockchain, but
for Lightning there is no global ledger to check.

BIP-321 solves this with the `pop=` parameter — a callback URI
the wallet opens after payment completes, passing the proof back
to the initiating app.

This library handles the full PoP flow:

- Validates the callback URI scheme at parse time — forbidden
  schemes like `https` are rejected to protect sender privacy
- Handles both `pop=` (optional) and `req-pop=` (required)
- Assembles the final callback URI after payment with
  `build_callback(pop_value, method, proof)`

Supported proof formats:
- On-chain → full hex-encoded transaction including witness data
- Lightning → hex-encoded payment preimage

```rust
// After payment completes
let callback = build_callback(&uri.pop.unwrap(), "onchain", &hex_tx);
// open callback with OS
```

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bip321 = { path = "src/bip321" }

# optional adapters
silent-payment = { path = "src/silent-payment" }
payjoin = { path = "src/payjoin" }
```

---

## Usage

### Parse a URI

```
$ bip321 parse "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=20.3&label=Luke-Jr&lightning=lnbc420bogusinvoice"

Address:      175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W
Amount:       20.3 BTC
Label:        Luke-Jr
Message:      None
Pop:          None
Instructions:
  - OnChain:  175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W
  - Lightning: lnbc420bogusinvoice
Unknown:      None
```

### Validate a URI

```
$ bip321 validate "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=20.3&label=Luke-Jr"
Valid URI

$ bip321 validate "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=20.3&amount=50"
Error: DuplicateParam("amount")
```

### Build a URI

```
$ bip321 build --address 175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W --amount 20.3 --label Luke-Jr

bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?amount=20.3&label=Luke-Jr
```

### Proof of Payment Callback

```
$ bip321 pop "bitcoin:175tWpb8K1S7NmH4Zx6rewF9WQrcZv245W?pop=initiatingapp%3a" onchain 0200000001abc

initiatingapp:onchain=0200000001abc
```

---

## Running Tests

```
cargo test --all
```

---

## Supported Payment Instructions

| Parameter | Description |
|---|---|
| `lightning` | BOLT 11 Lightning invoices |
| `lno` | BOLT 12 offers |
| `sp` | BIP-352 Silent Payment addresses |
| `pj` | PayJoin endpoints (BIP-78) |
| `bc` / `tb` | Segwit addresses by network |

---

## Forward Compatibility

Query parameters prefixed with `req-` are required — if your
wallet doesn't understand them the entire URI is invalid.
Parameters without `req-` are safely ignored, preserving
forward compatibility as the spec evolves.

---

## Future Work

- [ ] `no_std` support for embedded wallets
- [ ] `serde` feature flag for JSON serialization
- [ ] Async `PaymentResolver` for network-dependent resolution
- [ ] Full BIP-352 Silent Payment sending via `silentpayments` crate

---

## License

MIT
