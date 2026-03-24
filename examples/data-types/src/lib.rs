#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

/// Data Types Example Contract
///
/// This contract demonstrates the various data types available in Soroban,
/// including primitives, collections, and custom types.
#[contract]
pub struct DataTypesContract;

#[contractimpl]
impl DataTypesContract {
    /// Demonstrates integer types
    pub fn integer_example(env: Env) -> i128 {
        // Soroban supports i32, i64, i128, u32, u64, u128
        let value: i128 = 42;
        env.events().publish(("integer_example",), value);
        value
    }

    /// Demonstrates boolean type
    pub fn boolean_example(env: Env) -> bool {
        let is_active = true;
        env.events().publish(("boolean_example",), is_active);
        is_active
    }

    /// Demonstrates string/symbol type
    pub fn symbol_example(env: Env) -> Symbol {
        let symbol = Symbol::new(&env, "hello");
        env.events().publish(("symbol_example",), symbol.clone());
        symbol
    }

    /// Demonstrates vector/collection type
    pub fn vector_example(env: Env) -> Vec<i32> {
        let mut vec = Vec::new(&env);
        vec.push_back(1);
        vec.push_back(2);
        vec.push_back(3);
        env.events().publish(("vector_example",), vec.clone());
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_integer_example() {
        let env = Env::default();
        let contract = DataTypesContract;
        let result = contract.integer_example(env);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_boolean_example() {
        let env = Env::default();
        let contract = DataTypesContract;
        let result = contract.boolean_example(env);
        assert!(result);
    }

    #[test]
    fn test_symbol_example() {
        let env = Env::default();
        let contract = DataTypesContract;
        let result = contract.symbol_example(env);
        assert_eq!(result, Symbol::new(&env, "hello"));
    }

    #[test]
    fn test_vector_example() {
        let env = Env::default();
        let contract = DataTypesContract;
        let result = contract.vector_example(env);
        assert_eq!(result.len(), 3);
    }
}
