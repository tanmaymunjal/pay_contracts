import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PayContracts } from "../target/types/pay_contracts";
import {
  createMint,
  mintTo,
  createAssociatedTokenAccount,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { rpcConfig } from "./test_config";
import { rpc } from "@coral-xyz/anchor/dist/cjs/utils";

describe("pay_contracts", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  // Configure the client to use the local cluster.

  const program = anchor.workspace.PayContracts as Program<PayContracts>;
  const { web3 } = anchor;
  const {
    provider: { connection },
  } = program;

  let global = {};

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

  async function get_pda_from_seeds(seeds) {
    return web3.PublicKey.findProgramAddressSync(seeds, program.programId);
  }

  async function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  it("Is initialized!", async () => {
    // Add your test here.
    const initializeSigner = await create_keypair();
    const bot = await create_keypair();
    global["bot"] = bot;
    global["botPublicKey"] = bot.publicKey;

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

    const tx = await program.methods
      .initialize(
        global["botPublicKey"],
        global["botPublicKey"],
        global["botPublicKey"]
      )
      .accounts({
        initializer: initializeSigner.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        usdcMint: mintAddress,
      })
      .signers([initializeSigner])
      .rpc(rpcConfig);
    global["initializeSigner"] = initializeSigner;
    global["mintAddress"] = mintAddress;
    console.log("Your transaction signature", tx);
  });

  it("Create borrower!", async () => {
    const borrowerSigner = await create_keypair();

    const signerTokenAddr = await createAssociatedTokenAccount(
      connection,
      borrowerSigner,
      global["mintAddress"],
      borrowerSigner.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    await mintTo(
      connection,
      borrowerSigner,
      global["mintAddress"],
      signerTokenAddr,
      global["initializeSigner"],
      30 * Math.pow(10, 12),
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    await program.methods
      .createBorrower()
      .accounts({ borrowerSigner: borrowerSigner.publicKey })
      .signers([borrowerSigner])
      .rpc(rpcConfig);
    global["borrowerSigner"] = borrowerSigner;
  });

  it("Edit initialize", async () => {
    await program.methods
      .editInitialize(
        global["botPublicKey"],
        global["botPublicKey"],
        global["botPublicKey"]
      )
      .accounts({ editor: global["initializeSigner"].publicKey })
      .signers([global["initializeSigner"]])
      .rpc(rpcConfig);
  });

  it("Send borrow apppl", async () => {
    await program.methods
      .borrowAppl(
        "1234",
        new anchor.BN(10000),
        "https://www.google.com/search?q=monkey&oq=monkey&gs_lcrp="
      )
      .accounts({ borrowerSigner: global["borrowerSigner"].publicKey })
      .signers([global["borrowerSigner"]])
      .rpc(rpcConfig);
  });
  it("Appprove Appl", async () => {
    let [borrowerAcc] = await get_pda_from_seeds([
      Buffer.from("borrower"),
      global["borrowerSigner"].publicKey.toBuffer(),
    ]);

    let [borrowAppl] = await get_pda_from_seeds([
      Buffer.from("borrower_appl"),
      borrowerAcc.toBuffer(),
      Buffer.from("1234"),
    ]);
    await program.methods
      .approveAppl(new anchor.BN(9000))
      .accounts({
        borrowAppl: borrowAppl,
        lendingAgent: global["botPublicKey"],
      })
      .signers([global["bot"]])
      .rpc(rpcConfig);
    global["borrowerAcc"] = borrowerAcc;
    global["borrowAppl"] = borrowAppl;
  });
  it("Debarr borrower", async () => {
    await program.methods
      .debarrBorrower(true)
      .accounts({
        debarrer: global["initializeSigner"].publicKey,
        borrowerAcc: global["borrowerAcc"],
      })
      .signers([global["initializeSigner"]])
      .rpc(rpcConfig);
  });
  it("Collections borrower", async () => {
    await program.methods
      .collectionsBorrower(true)
      .accounts({
        collector: global["initializeSigner"].publicKey,
        borrowerAcc: global["borrowerAcc"],
      })
      .signers([global["initializeSigner"]])
      .rpc(rpcConfig);
  });
  it("Create staking vault", async () => {
    const fiften_days = { fifteenDays: {} };
    await program.methods
      .createStakingVault("1234", fiften_days)
      .accounts({ initializer: global["initializeSigner"].publicKey })
      .signers([global["initializeSigner"]])
      .rpc(rpcConfig);
  });
  it("Stake staking vault", async () => {
    const [stakingVault] = await get_pda_from_seeds([
      Buffer.from("staking_vault"),
      Buffer.from("1234"),
    ]);
    await program.methods
      .stakeStakingVault(new anchor.BN(1000))
      .accounts({
        staker: global["initializeSigner"].publicKey,
        stakingVault: stakingVault,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        usdcMint: global["mintAddress"],
      })
      .signers([global["initializeSigner"]])
      .rpc(rpcConfig);
    global["stakingVault"] = stakingVault;
  });
  it("Pay money", async () => {
    await sleep(5000);
    const instance = {
      req: [{ epoch: { fifteenDays: {} }, amount: new anchor.BN(99) }],
    };
    await program.methods
      .payMoney(instance)
      .accounts({
        fraudAgent: global["botPublicKey"],
        cardAgent: global["botPublicKey"],
        borrowAppl: global["borrowAppl"],
        fifteenStakingVault: global["stakingVault"],
        thirtyStakingVault: null,
        fortyFiveStakingVault: null,
        sixtyStakingVault: null,
        seventyFiveStakingVault: null,
        ninetyStakingVault: null,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        usdcMint: global["mintAddress"],
      })
      .signers([global["bot"]])
      .rpc(rpcConfig);
  });
  it("Pay back money", async () => {
    await program.methods
      .payBackMoney(new anchor.BN(99), new anchor.BN(1))
      .accountsPartial({
        fraudAgent: global["botPublicKey"],
        borrowerSigner: global["borrowerSigner"].publicKey,
        borrowAppl: global["borrowAppl"],
        usdcMint: global["mintAddress"],
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([global["bot"], global["borrowerSigner"]])
      .rpc(rpcConfig);
  });
  it("Allocate back money", async () => {
    const instance = {
      req: [{ epoch: { fifteenDays: {} }, amount: new anchor.BN(100) }],
    };
    await program.methods
      .allocateBackMoney(instance)
      .accounts({
        fraudAgent: global["botPublicKey"],
        fifteenStakingVault: global["stakingVault"],
        thirtyStakingVault: null,
        fortyFiveStakingVault: null,
        sixtyStakingVault: null,
        seventyFiveStakingVault: null,
        ninetyStakingVault: null,
      })
      .signers([global["bot"]])
      .rpc(rpcConfig);
  });
  it("Unstake Staking Vault", async () => {
    await sleep(75000);
    await program.methods
      .unstakeStakingVault()
      .accounts({
        staker: global["initializeSigner"].publicKey,
        stakingVault: global["stakingVault"],
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        usdcMint: global["mintAddress"],
      })
      .signers([global["initializeSigner"]])
      .rpc(rpcConfig);
  });
});
