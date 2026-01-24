#[derive(Accounts)]
pub struct refund<'info> {

    #[account(mut)]
     pub maker: SystemAccount<'info>,

     #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
        has_one = mint_b @ EscrowError::InvalidMintB,
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    mint_a：maker 存入的代币

    vault：与 escrow 和 mint_a 关联的代币账户，代币已存入其中

    maker_ata_a：与 maker 和 mint_a 关联的代币账户，将从 vault 接收代币

    associated_token_program：用于创建关联代币账户的关联代币程序

    token_program：用于 CPI 转账的代币程序

    system_program：用于创建 Escrow 的系统程序
}