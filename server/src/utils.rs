//! Utility functions for StelloVault backend

use uuid::Uuid;

// Database utilities
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

// Validation utilities
pub fn is_valid_stellar_address(address: &str) -> bool {
    // TODO: Implement proper Stellar address validation
    address.starts_with('G') && address.len() == 56
}

// Error handling utilities
pub fn map_database_error(error: sqlx::Error) -> String {
    // TODO: Implement proper error mapping
    format!("Database error: {}", error)
}

// Pagination utilities
pub fn calculate_offset(page: i32, limit: i32) -> i64 {
    ((page - 1) * limit) as i64
}

pub fn calculate_total_pages(total: i64, limit: i32) -> i32 {
    ((total + limit as i64 - 1) / limit as i64) as i32
}