import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PayContracts } from "../target/types/pay_contracts";
import { PublicKey } from "@solana/web3.js";
import {
  createMint,
  mintTo,
  createAssociatedTokenAccount,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { rpcConfig } from "./test_config";

describe("pay_contracts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  // Configure the client to use the local cluster.

  const program = anchor.workspace.PayContracts as Program<PayContracts>;
  const { web3 } = anchor;
  const {
    provider: { connection },
  } = program;

  const botPublicKey = new PublicKey(
    "55kBY9yxqSC42boV8PywT2gqGzgLi5MPAtifNRgPNezF"
  );

  let global= {}
  global["botPublicKey"] = botPublicKey;

  async function create_keypair() {
    const keypair = web3.Keypair.generate();
    await connection.confirmTransaction(
      {
        signature: await connection.requestAirdrop(
          keypair.publicKey,
          3 * web3.LAMPORTS_PER_SOL
        ),
        ...(await connection.getLatestBlockhash()),
      },
      "confirmed"
    );
    return keypair;
  }
  
  it("Is initialized!", async () => {
        // Add your test here.
        const initializeSigner = await create_keypair();

        const mintAddress = await createMint(
          connection,
          initializeSigner,
          initializeSigner.publicKey,
          initializeSigner.publicKey,
          6,
          undefined,
          undefined,
          TOKEN_2022_PROGRAM_ID
        );
    
        const signerTokenAddr = await createAssociatedTokenAccount(
          connection,
          initializeSigner,
          mintAddress,
          initializeSigner.publicKey,
          undefined,
          TOKEN_2022_PROGRAM_ID
        );
    
        await mintTo(
          connection,
          initializeSigner,
          mintAddress,
          signerTokenAddr,
          initializeSigner,
          30 * Math.pow(10, 12),
          undefined,
          undefined,
          TOKEN_2022_PROGRAM_ID
        );
    
    const tx = await program.methods.initialize(global["botPublicKey"], global["botPublicKey"], global["botPublicKey"])
    .accounts({initializer: initializeSigner.publicKey,tokenProgram: TOKEN_2022_PROGRAM_ID, usdcMint: mintAddress}).signers([initializeSigner]).rpc(rpcConfig);
    global["initializeSigner"] = initializeSigner;
    console.log("Your transaction signature", tx);
  });

  it("Create borrower!", async () => {
    await program.methods.createBorrower().accounts({borrowerSigner: global["initializeSigner"].publicKey}).signers([global["initializeSigner"]]).rpc(rpcConfig);
  });

  it("Edit initialize", async () => {
    await program.methods.editInitialize(global["botPublicKey"], global["botPublicKey"], global["botPublicKey"]).accounts({editor: global["initializeSigner"].publicKey}).signers([global["initializeSigner"]]).rpc(rpcConfig);
  })
});
