use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

/*
This is the function transfer_tokens, which facilitates the transfer of tokens from one account to another. Here’s a breakdown of the parameters:

from: The source token account from which tokens will be transferred.
to: The destination token account where tokens will be received.
amount: The number of tokens to transfer.
mint: The mint address that represents the token being transferred.
authority: The account (usually a wallet) that has the authority to approve the token transfer (e.g., the signer of the transaction).
token_program: The token program on the Solana blockchain that facilitates the transfer and other token operations.
The function returns a Result<()>, which means it will either complete the transaction successfully (return Ok(())) or return an error if something goes wrong.
*/

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
/*
A CPI (Cross-Program Invocation) context is created here. Solana programs often call other programs (like the token program), and to do so, they need a CPI context.
*/
    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts_options);

    transfer_checked(cpi_context, *amount, mint.decimals)
}