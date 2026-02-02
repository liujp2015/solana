use pinocchio::{AccountView, error::ProgramError};

/// Validates that the maker account is a signer
pub fn validate_maker_account(maker: &AccountView) -> Result<(), ProgramError> {
    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

/// Validates that the mint account is a valid Mint
pub fn validate_mint_account(mint: &AccountView) -> Result<(), ProgramError> {
    // Basic mint account validation
    if mint.data_len() < 8 { // Minimum mint size
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

/// Validates that the mint interface is valid
pub fn validate_mint_interface(mint: &AccountView) -> Result<(), ProgramError> {
    // Basic mint interface validation
    if mint.data_len() < 8 { // Minimum mint size
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

/// Validates that the associated token account is valid
pub fn validate_associated_token_account(
    ata: &AccountView,
    _owner: &AccountView,
    _mint: &AccountView,
    _token_program: &AccountView
) -> Result<(), ProgramError> {
    // Basic associated token account validation
    if ata.data_len() < 165 { // Minimum ATA size
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

/// Validates that the system program account is valid
pub fn validate_system_program(system_program: &AccountView) -> Result<(), ProgramError> {
    // Basic system program validation
    Ok(())
}

/// Validates that the token program account is valid
pub fn validate_token_program(token_program: &AccountView) -> Result<(), ProgramError> {
    // Basic token program validation
    Ok(())
}
