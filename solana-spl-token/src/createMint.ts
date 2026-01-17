// START HERE

// Create the mint account
const createAccountIx = SystemProgram.createAccount({
    fromPubkey: feePayer.publicKey,
    newAccountPubkey: mint.publicKey,
    space: MINT_SIZE,
    lamports: mintRent,
    programId: TOKEN_PROGRAM_ID,
  });
  
  // Initialize the mint account
  // ✅ BOTH mint AND freeze authorities are feePayer (as per challenge)
  const initializeMintIx = createInitializeMint2Instruction(
    mint.publicKey,
    6, // decimals
    feePayer.publicKey, // mint authority
    feePayer.publicKey, // freeze authority ← was null, now fixed!
    TOKEN_PROGRAM_ID
  );
  
  // Create the associated token account
  const associatedTokenAccount = getAssociatedTokenAddressSync( // ✅ sync is fine
    mint.publicKey,
    feePayer.publicKey,
    false,
    TOKEN_PROGRAM_ID
  );
  const createAssociatedTokenAccountIx = createAssociatedTokenAccountIdempotentInstruction(
    feePayer.publicKey,
    associatedTokenAccount,
    feePayer.publicKey,
    mint.publicKey,
    TOKEN_PROGRAM_ID
  );
  
  // Mint 21,000,000 tokens (with 6 decimals)
  const mintAmount = 21_000_000n * 1_000_000n; // bigint: 21e12
  const mintToCheckedIx = createMintToCheckedInstruction( // ✅ use "Checked" version
    mint.publicKey,
    associatedTokenAccount,
    feePayer.publicKey,
    mintAmount,
    6 // must match mint decimals
  );
  
  const recentBlockhash = await connection.getLatestBlockhash();
  
  const transaction = new Transaction({
    feePayer: feePayer.publicKey,
    blockhash: recentBlockhash.blockhash,
    lastValidBlockHeight: recentBlockhash.lastValidBlockHeight,
  }).add(
    createAccountIx,
    initializeMintIx,
    createAssociatedTokenAccountIx,
    mintToCheckedIx
  );
  
  // Signers: feePayer (pays + authority) and mint (new account)
  const transactionSignature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [feePayer, mint] // ✅ correct
  );