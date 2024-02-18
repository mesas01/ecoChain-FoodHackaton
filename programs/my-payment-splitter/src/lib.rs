use anchor_lang::prelude::*;
use anchor_lang::solana_program::{system_instruction, program::invoke};
 
declare_id!("CZrt6RtXnwd6P2Z1kjSSwz8V9kojXtxDq78v4A95ZFA1");
 
#[program]
pub mod my_payment_splitter {
    use super::*;
    pub fn process_payment(
        ctx: Context<ProcessPayment>,
        total_payment: u64,
        cashback_percentage: u64,
        product_name: String,
        expiration_date: String
    ) -> Result<()> {
        msg!("Compra de producto: {}, Fecha de caducidad: {}", product_name, expiration_date);
 
        if cashback_percentage > 100 {
            return Err(ProgramError::InvalidArgument.into());
        }
 
        // Convierte el cashback_percentage a f64 para los cálculos
        let cashback_percentage_f64 = cashback_percentage as f64;
       
        // Calcula el monto del cashback y la porción para la empresa ajustando por 2.5%
        let adjusted_cashback_percentage = cashback_percentage_f64 - 2.5;
        let adjusted_store_percentage = 100.0 - cashback_percentage_f64 - 2.5;
        let ecochain_percentage = 5.0; // Suma de los 2.5% de cada parte
 
        let cashback_amount = (total_payment as f64) * (adjusted_cashback_percentage / 100.0);
        let store_share = (total_payment as f64) * (adjusted_store_percentage / 100.0);
        let ecochain_share = (total_payment as f64) * (ecochain_percentage / 100.0);
 
        // Transferencia a la empresa
        let transfer_instruction_to_store = system_instruction::transfer(
            &ctx.accounts.customer.key(),
            &ctx.accounts.store.key(),
            store_share as u64,
        );
        invoke(
            &transfer_instruction_to_store,
            &[
                ctx.accounts.customer.to_account_info(),
                ctx.accounts.store.to_account_info(),
            ],
        )?;
 
        // Cashback al cliente
        if cashback_amount > 0.0 {
            let transfer_instruction_to_customer = system_instruction::transfer(
                &ctx.accounts.customer.key(),
                &ctx.accounts.customer.key(),
                cashback_amount as u64,
            );
            invoke(
                &transfer_instruction_to_customer,
                &[
                    ctx.accounts.customer.to_account_info().clone(),
                    ctx.accounts.customer.to_account_info(),
                ],
            )?;
        }
 
        // Contribución a Ecochain
        if ecochain_share > 0.0 {
            let transfer_instruction_to_ecochain = system_instruction::transfer(
                &ctx.accounts.customer.key(),
                &ctx.accounts.ecochain.key(),
                ecochain_share as u64,
            );
            invoke(
                &transfer_instruction_to_ecochain,
                &[
                    ctx.accounts.customer.to_account_info(),
                    ctx.accounts.ecochain.to_account_info(),
                ],
            )?;
        }
 
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    #[account(mut)]
    pub customer: Signer<'info>, // Cliente que paga y recibe cashback
    #[account(mut)]
    pub store: AccountInfo<'info>, // Empresa que recibe el pago
    #[account(mut)]
    pub ecochain: AccountInfo<'info>, // Wallet de Ecochain
    pub system_program: Program<'info, System>,
}