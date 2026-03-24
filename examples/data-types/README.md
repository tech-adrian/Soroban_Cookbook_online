# Data Types Example

This example demonstrates the various data types available in Soroban smart contracts.

## Overview

Soroban supports a rich set of data types for building smart contracts:

- **Integers**: i32, i64, i128, u32, u64, u128
- **Booleans**: true/false values
- **Symbols**: String-like identifiers
- **Vectors**: Dynamic collections
- **Maps**: Key-value storage
- **Custom Types**: User-defined structures

## Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Testing

```bash
cargo test
```

## Contract Functions

### `integer_example()`
Demonstrates working with integer types in Soroban.

### `boolean_example()`
Demonstrates working with boolean values.

### `symbol_example()`
Demonstrates working with symbols (string identifiers).

### `vector_example()`
Demonstrates working with vectors (dynamic collections).

## Deployment

To deploy this contract to the Stellar testnet:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/data_types.wasm \
  --source <your-account> \
  --network testnet
```

## Resources

- [Soroban SDK Documentation](https://docs.rs/soroban-sdk)
- [Soroban Data Types](https://developers.stellar.org/docs/smart-contracts/learn/storing-data)
- [Stellar Developer Portal](https://developers.stellar.org/)
