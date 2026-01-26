import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorTestEscrow } from "../target/types/anchor_test_escrow";
import { expect } from "chai";
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getMint,
  getAccount,
  createMint,
  createAccount,
  mintTo
} from "@solana/spl-token";
import { 
  Keypair, 
  PublicKey,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL
} from "@solana/web3.js";

describe("anchor_test_escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorTestEscrow as Program<AnchorTestEscrow>;
  
  let maker: Keypair;
  let taker: Keypair;
  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA: PublicKey;
  let makerAtaB: PublicKey;
  let takerAtaA: PublicKey;
  let takerAtaB: PublicKey;
  let escrowPda: PublicKey;
  let vault: PublicKey;
  let bump: number;
  
  const seed = new anchor.BN(12345);
  const receiveAmount = new anchor.BN(100);
  const depositAmount = new anchor.BN(50);

  before(async () => {
    maker = Keypair.generate();
    taker = Keypair.generate();

    await airdrop(provider.connection, maker.publicKey, 2 * LAMPORTS_PER_SOL);
    await airdrop(provider.connection, taker.publicKey, 2 * LAMPORTS_PER_SOL);

    mintA = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9
    );

    mintB = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9
    );

    makerAtaA = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mintA,
      maker.publicKey
    );

    makerAtaB = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mintB,
      maker.publicKey
    );

    takerAtaA = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mintA,
      taker.publicKey
    );

    takerAtaB = await createAccount(
      provider.connection,
      provider.wallet.payer,
      mintB,
      taker.publicKey
    );

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintA,
      makerAtaA,
      provider.wallet.publicKey,
      1000 * 10 ** 9
    );

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mintB,
      takerAtaB,
      provider.wallet.publicKey,
      1000 * 10 ** 9
    );

    const [escrowPdaKey, bumpValue] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    escrowPda = escrowPdaKey;
    bump = bumpValue;

    const vaultKey = await getAssociatedTokenAddress(
      mintA,
      escrowPda,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    vault = vaultKey;
  });

  it("Creates an escrow", async () => {
    const tx = await program.methods
      .make(seed, receiveAmount, depositAmount)
      .accounts({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA,
        escrow: escrowPda,
        vault: vault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

    console.log("Transaction signature (make):", tx);

    const escrowAccount = await program.account.escrow.fetch(escrowPda);
    
    expect(escrowAccount.seed.toNumber()).to.equal(seed.toNumber());
    expect(escrowAccount.maker.toString()).to.equal(maker.publicKey.toString());
    expect(escrowAccount.mintA.toString()).to.equal(mintA.toString());
    expect(escrowAccount.mintB.toString()).to.equal(mintB.toString());
    expect(escrowAccount.receive.toNumber()).to.equal(receiveAmount.toNumber());
    expect(escrowAccount.bump).to.equal(bump);

    const vaultAccount = await getAccount(provider.connection, vault);
    expect(vaultAccount.amount.toString()).to.equal(depositAmount.toString());
    expect(vaultAccount.owner.toString()).to.equal(escrowPda.toString());
  });

  it("Takes an escrow", async () => {
    // Get initial balances
    const initialTakerAtaABalance = await getAccount(provider.connection, takerAtaA);
    const initialMakerAtaBBalance = await getAccount(provider.connection, makerAtaB);

    const tx = await program.methods
      .take()
      .accounts({
        taker: taker.publicKey,
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        vault: vault,
        takerAtaA: takerAtaA,
        takerAtaB: takerAtaB,
        makerAtaB: makerAtaB,
        escrow: escrowPda,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([taker])
      .rpc();

    console.log("Transaction signature (take):", tx);

    // Check final balances
    const finalTakerAtaABalance = await getAccount(provider.connection, takerAtaA);
    const finalMakerAtaBBalance = await getAccount(provider.connection, makerAtaB);

    expect(finalTakerAtaABalance.amount.toString()).to.equal(
      depositAmount.add(new anchor.BN(initialTakerAtaABalance.amount.toString())).toString()
    );

    expect(finalMakerAtaBBalance.amount.toString()).to.equal(
      receiveAmount.add(new anchor.BN(initialMakerAtaBBalance.amount.toString())).toString()
    );

    // Check if escrow account was closed
    try {
      await program.account.escrow.fetch(escrowPda);
      expect.fail("Escrow account should have been closed");
    } catch (err) {
      expect(err.toString()).to.include("Error");
    }
  });

  it("Fails with invalid amount", async () => {
    try {
      await program.methods
        .make(seed, new anchor.BN(0), depositAmount)
        .accounts({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaA: makerAtaA,
          escrow: escrowPda,
          vault: vault,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([maker])
        .rpc();
      
      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err.toString()).to.include("Error");
    }
  });

  it("Refunds an escrow", async () => {
    // Create a new escrow for refund test
    const refundSeed = new anchor.BN(54321);
    const refundReceiveAmount = new anchor.BN(200);
    const refundDepositAmount = new anchor.BN(75);

    const [refundEscrowPda, refundBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        refundSeed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const refundVault = await getAssociatedTokenAddress(
      mintA,
      refundEscrowPda,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    // Create the escrow
    await program.methods
      .make(refundSeed, refundReceiveAmount, refundDepositAmount)
      .accounts({
        maker: maker.publicKey,
        mintA: mintA,
        mintB: mintB,
        makerAtaA: makerAtaA,
        escrow: refundEscrowPda,
        vault: refundVault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker])
      .rpc();

    // Get initial maker ATA balance
    const initialMakerAtaABalance = await getAccount(provider.connection, makerAtaA);

    // Refund the escrow
    const tx = await program.methods
      .refund()
      .accounts({
        maker: maker.publicKey,
        escrow: refundEscrowPda,
        mintA: mintA,
        vault: refundVault,
        makerAtaA: makerAtaA,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Transaction signature (refund):", tx);

    // Check final maker ATA balance
    const finalMakerAtaABalance = await getAccount(provider.connection, makerAtaA);
    expect(finalMakerAtaABalance.amount.toString()).to.equal(
      refundDepositAmount.add(new anchor.BN(initialMakerAtaABalance.amount.toString())).toString()
    );

    // Check if escrow account was closed
    try {
      await program.account.escrow.fetch(refundEscrowPda);
      expect.fail("Escrow account should have been closed");
    } catch (err) {
      expect(err.toString()).to.include("Error");
    }

    // Check if vault account was closed (should not exist anymore)
    try {
      await getAccount(provider.connection, refundVault);
      expect.fail("Vault account should have been closed");
    } catch (err) {
      expect(err.toString()).to.include("Error");
    }
  });
});

async function airdrop(
  connection: any,
  address: PublicKey,
  amount: number
): Promise<void> {
  const signature = await connection.requestAirdrop(address, amount);
  await connection.confirmTransaction(signature);
}

async function getAssociatedTokenAddress(
  mint: PublicKey,
  owner: PublicKey,
  allowOwnerOffCurve: boolean = false,
  programId: PublicKey = TOKEN_PROGRAM_ID,
  associatedTokenProgramId: PublicKey = ASSOCIATED_TOKEN_PROGRAM_ID
): Promise<PublicKey> {
  const [address] = PublicKey.findProgramAddressSync(
    [
      owner.toBuffer(),
      programId.toBuffer(),
      mint.toBuffer(),
    ],
    associatedTokenProgramId
  );
  return address;
}
