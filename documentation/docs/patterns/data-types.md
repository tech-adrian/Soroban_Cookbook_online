---
sidebar_position: 2
title: Data Types
description: Understanding and working with Soroban data types
---

# Data Types in Soroban

Soroban provides a comprehensive set of data types for building smart contracts. This guide covers the available types and how to use them effectively.

## Primitive Types

### Integers

Soroban supports both signed and unsigned integers of various sizes:

- **Signed**: `i32`, `i64`, `i128`
- **Unsigned**: `u32`, `u64`, `u128`

```rust
#[contractimpl]
impl MyContract {
    pub fn work_with_integers(env: Env) -> i128 {
        let small_int: i32 = 100;
        let large_int: i128 = 9_223_372_036_854_775_807;
        large_int as i128
    }
}
```

### Booleans

Boolean values represent true/false conditions:

```rust
pub fn check_condition(env: Env) -> bool {
    let is_valid = true;
    is_valid
}
```

### Symbols

Symbols are string-like identifiers optimized for contract storage:

```rust
pub fn create_symbol(env: Env) -> Symbol {
    Symbol::new(&env, "token_balance")
}
```

## Collection Types

### Vectors

Vectors are dynamic, ordered collections:

```rust
pub fn create_vector(env: Env) -> Vec<i32> {
    let mut numbers = Vec::new(&env);
    numbers.push_back(1);
    numbers.push_back(2);
    numbers.push_back(3);
    numbers
}
```

### Maps

Maps store key-value pairs:

```rust
use soroban_sdk::Map;

pub fn create_map(env: Env) -> Map<Symbol, i128> {
    let mut balances = Map::new(&env);
    balances.set(Symbol::new(&env, "alice"), 1000);
    balances.set(Symbol::new(&env, "bob"), 500);
    balances
}
```

## Best Practices

1. **Choose appropriate integer sizes** - Use the smallest type that fits your needs
2. **Use Symbols for keys** - More efficient than strings for storage
3. **Validate input types** - Always check data before processing
4. **Consider storage costs** - Larger types consume more storage

## See Also

- [Storage Patterns](./storage.md)
- [Authorization Patterns](../concepts/authorization.md)
- [Soroban SDK Reference](https://docs.rs/soroban-sdk)
