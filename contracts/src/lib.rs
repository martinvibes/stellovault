//! StelloVault Soroban Contracts
//!
//! This module contains the smart contracts for StelloVault, a trade finance dApp
//! built on Stellar and Soroban. The contracts handle collateral tokenization,
//! multi-signature escrows, and automated release mechanisms.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, Address, Env, Symbol};

/// Contract errors
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    Unauthorized = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
    EscrowNotFound = 4,
    EscrowAlreadyReleased = 5,
}

/// Collateral token data structure
#[contracttype]
#[derive(Clone)]
pub struct CollateralToken {
    pub owner: Address,
    pub asset_type: Symbol, // e.g., "INVOICE", "COMMODITY"
    pub asset_value: i128,
    pub metadata: Symbol, // Hash of off-chain metadata
    pub fractional_shares: u32,
    pub created_at: u64,
}

/// Escrow data structure for trade finance deals
#[contracttype]
#[derive(Clone)]
pub struct TradeEscrow {
    pub buyer: Address,
    pub seller: Address,
    pub collateral_token_id: u64,
    pub amount: i128,
    pub status: EscrowStatus,
    pub oracle_address: Address,
    pub release_conditions: Symbol, // e.g., "SHIPMENT_DELIVERED"
    pub created_at: u64,
}

/// Escrow status enum
#[contracttype]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum EscrowStatus {
    Pending = 0,
    Active = 1,
    Released = 2,
    Cancelled = 3,
}

/// Main contract for StelloVault trade finance operations
#[contract]
pub struct StelloVaultContract;

/// Contract implementation
#[contractimpl]
impl StelloVaultContract {
    /// Initialize the contract
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&symbol_short!("admin")) {
            return Err(ContractError::Unauthorized);
        }

        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("next_token_id"), &1u64);
        env.storage().instance().set(&symbol_short!("next_escrow_id"), &1u64);

        env.events().publish((symbol_short!("init"),), (admin,));
        Ok(())
    }

    /// Get contract admin
    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&symbol_short!("admin"))
            .unwrap()
    }

    /// Tokenize collateral (create a new collateral token)
    pub fn tokenize_collateral(
        env: Env,
        owner: Address,
        asset_type: Symbol,
        asset_value: i128,
        metadata: Symbol,
        fractional_shares: u32,
    ) -> Result<u64, ContractError> {
        owner.require_auth();

        if asset_value <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let token_id: u64 = env
            .storage()
            .instance()
            .get(&symbol_short!("next_token_id"))
            .unwrap_or(1);

        let collateral = CollateralToken {
            owner: owner.clone(),
            asset_type,
            asset_value,
            metadata,
            fractional_shares,
            created_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&token_id, &collateral);

        env.storage()
            .instance()
            .set(&symbol_short!("next_token_id"), &(token_id + 1));

        env.events().publish(
            (symbol_short!("tokenize"),),
            (token_id, owner, asset_value),
        );

        Ok(token_id)
    }

    /// Get collateral token details
    pub fn get_collateral(env: Env, token_id: u64) -> Option<CollateralToken> {
        env.storage().persistent().get(&token_id)
    }

    /// Create a trade escrow
    pub fn create_escrow(
        env: Env,
        buyer: Address,
        seller: Address,
        collateral_token_id: u64,
        amount: i128,
        oracle_address: Address,
        release_conditions: Symbol,
    ) -> Result<u64, ContractError> {
        buyer.require_auth();

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Verify collateral token exists
        if env.storage().persistent().get::<u64, CollateralToken>(&collateral_token_id).is_none() {
            return Err(ContractError::EscrowNotFound);
        }

        let escrow_id: u64 = env
            .storage()
            .instance()
            .get(&symbol_short!("next_escrow_id"))
            .unwrap_or(1);

        let escrow = TradeEscrow {
            buyer: buyer.clone(),
            seller: seller.clone(),
            collateral_token_id,
            amount,
            status: EscrowStatus::Pending,
            oracle_address,
            release_conditions,
            created_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&escrow_id, &escrow);

        env.storage()
            .instance()
            .set(&symbol_short!("next_escrow_id"), &(escrow_id + 1));

        env.events().publish(
            (symbol_short!("escrow_created"),),
            (escrow_id, buyer, seller, amount),
        );

        Ok(escrow_id)
    }

    /// Get escrow details
    pub fn get_escrow(env: Env, escrow_id: u64) -> Option<TradeEscrow> {
        env.storage().persistent().get(&escrow_id)
    }

    /// Activate an escrow (funded and ready)
    pub fn activate_escrow(env: Env, escrow_id: u64) -> Result<(), ContractError> {
        let mut escrow: TradeEscrow = env
            .storage()
            .persistent()
            .get(&escrow_id)
            .ok_or(ContractError::EscrowNotFound)?;

        if escrow.status != EscrowStatus::Pending {
            return Err(ContractError::Unauthorized);
        }

        escrow.status = EscrowStatus::Active;
        env.storage().persistent().set(&escrow_id, &escrow);

        env.events().publish((symbol_short!("escrow_activated"),), (escrow_id,));
        Ok(())
    }

    /// Release escrow funds (oracle-triggered)
    pub fn release_escrow(env: Env, escrow_id: u64) -> Result<(), ContractError> {
        let mut escrow: TradeEscrow = env
            .storage()
            .persistent()
            .get(&escrow_id)
            .ok_or(ContractError::EscrowNotFound)?;

        // Only oracle can trigger release
        escrow.oracle_address.require_auth();

        if escrow.status != EscrowStatus::Active {
            return Err(ContractError::EscrowAlreadyReleased);
        }

        escrow.status = EscrowStatus::Released;
        env.storage().persistent().set(&escrow_id, &escrow);

        env.events().publish((symbol_short!("escrow_released"),), (escrow_id,));
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Env as _};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let admin = Address::generate(&env);

        let contract = StelloVaultContract::initialize(env.clone(), admin.clone());
        assert!(contract.is_ok());

        assert_eq!(StelloVaultContract::admin(env), admin);
    }

    #[test]
    fn test_tokenize_collateral() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let owner = Address::generate(&env);

        StelloVaultContract::initialize(env.clone(), admin).unwrap();

        let result = StelloVaultContract::tokenize_collateral(
            env.clone(),
            owner.clone(),
            symbol_short!("INVOICE"),
            1000,
            symbol_short!("metadata_hash"),
            100,
        );

        assert!(result.is_ok());
        let token_id = result.unwrap();

        let collateral = StelloVaultContract::get_collateral(env, token_id).unwrap();
        assert_eq!(collateral.owner, owner);
        assert_eq!(collateral.asset_value, 1000);
    }
}