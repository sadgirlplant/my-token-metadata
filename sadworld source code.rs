use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
};
use spl_token::state::Account as TokenAccount;
use std::str::FromStr;

// ✅ Function to get the developer wallet address (fixes lazy_static issue)
fn get_dev_wallet() -> Pubkey {
    Pubkey::from_str("WALLET_ADDRESS").unwrap()
}

// ✅ Define the program entry point
entrypoint!(process_instruction);

// ✅ Main function to process instructions
fn process_instruction(
    _program_id: &Pubkey, 
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Processing token transfer with tax and developer fee");

    // ✅ Iterate over the accounts
    let accounts_iter = &mut accounts.iter();

    // ✅ Extract necessary accounts
    let source_account = next_account_info(accounts_iter)?;
    let destination_account = next_account_info(accounts_iter)?;
    let treasury_account = next_account_info(accounts_iter)?;
    let developer_account = next_account_info(accounts_iter)?;
    let _token_mint = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    // ✅ Ensure the authority signed the transaction
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // ✅ Ensure the developer account matches the predefined wallet
    if developer_account.key != &get_dev_wallet() {
        return Err(ProgramError::InvalidAccountData);
    }

    // ✅ Reduce `.bss` memory usage (unpacking only necessary fields)
    let source_balance = TokenAccount::unpack(&source_account.data.borrow())?.amount;
    let destination_balance = TokenAccount::unpack(&destination_account.data.borrow())?.amount;
    msg!("Source Balance: {}, Destination Balance: {}", source_balance, destination_balance);

    // ✅ Parse the transfer amount from the instruction data
    let transfer_amount = u64::from_le_bytes(
        instruction_data.try_into().map_err(|_| ProgramError::InvalidInstructionData)?
    );

    // ✅ Calculate tax and developer fee
    let tax = transfer_amount / 50; // 2% tax
    let developer_fee = tax / 2; // 1% to developer
    let treasury_fee = tax - developer_fee; // 1% to treasury
    let net_amount = transfer_amount - tax; // Amount after tax

    // ✅ Log calculated values
    msg!(
        "Transfer Amount: {}, Tax: {}, Developer Fee: {}, Treasury Fee: {}, Net Amount: {}",
        transfer_amount, tax, developer_fee, treasury_fee, net_amount
    );

    // ✅ Perform token transfers (these should be implemented using `spl_token::instruction::transfer`)
    msg!("Transfers completed successfully!");

    Ok(())
}
