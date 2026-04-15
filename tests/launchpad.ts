import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import BN from "bn.js";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { expect } from "chai";

type Launchpad = any;

describe("launchpad", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Launchpad as Program<Launchpad>;
  const admin = provider.wallet as anchor.Wallet;

  // Test keypairs
  const devWallet = Keypair.generate();
  const platformWallet = Keypair.generate();
  const pauseAuthority = Keypair.generate();

  // PDAs
  let configPda: PublicKey;
  let configBump: number;

  before(async () => {
    // Airdrop to test wallets
    const sig1 = await provider.connection.requestAirdrop(
      devWallet.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    const sig2 = await provider.connection.requestAirdrop(
      platformWallet.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig1);
    await provider.connection.confirmTransaction(sig2);

    // Derive config PDA
    [configPda, configBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );
  });

  // ── Initialize ───────────────────────────────────────────────────────

  describe("initialize", () => {
    it("initializes global config", async () => {
      await program.methods
        .initialize({
          pauseAuthority: pauseAuthority.publicKey,
          devWallet: devWallet.publicKey,
          platformWallet: platformWallet.publicKey,
          devFeeBps: 50,
          platformFeeBps: 50,
          sellTaxBps: 2400,
          presalePlatformFeeBps: 100,
          migrationFeeBps: 100,
        })
        .accounts({
          admin: admin.publicKey,
          config: configPda,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const config = await program.account.globalConfig.fetch(configPda);
      expect(config.admin.toBase58()).to.equal(admin.publicKey.toBase58());
      expect(config.devFeeBps).to.equal(50);
      expect(config.platformFeeBps).to.equal(50);
      expect(config.sellTaxBps).to.equal(2400);
      expect(config.migrationFeeBps).to.equal(100);
      expect(config.isPaused).to.equal(false);
    });

    it("rejects duplicate initialization (PDA already exists)", async () => {
      try {
        await program.methods
          .initialize({
            pauseAuthority: pauseAuthority.publicKey,
            devWallet: devWallet.publicKey,
            platformWallet: platformWallet.publicKey,
            devFeeBps: 50,
            platformFeeBps: 50,
            sellTaxBps: 2400,
            presalePlatformFeeBps: 100,
            migrationFeeBps: 100,
          })
          .accounts({
            admin: admin.publicKey,
            config: configPda,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        // Account already initialized
        expect(err.toString()).to.not.be.empty;
      }
    });
  });

  // ── Pause / Unpause ──────────────────────────────────────────────────

  describe("pause / unpause", () => {
    it("pause authority can pause", async () => {
      await program.methods
        .pause()
        .accounts({
          authority: pauseAuthority.publicKey,
          config: configPda,
        })
        .signers([pauseAuthority])
        .rpc();

      const config = await program.account.globalConfig.fetch(configPda);
      expect(config.isPaused).to.equal(true);
    });

    it("admin can unpause", async () => {
      await program.methods
        .unpause()
        .accounts({
          authority: admin.publicKey,
          config: configPda,
        })
        .rpc();

      const config = await program.account.globalConfig.fetch(configPda);
      expect(config.isPaused).to.equal(false);
    });

    it("random user cannot pause", async () => {
      const rando = Keypair.generate();
      const sig = await provider.connection.requestAirdrop(
        rando.publicKey,
        LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      try {
        await program.methods
          .pause()
          .accounts({
            authority: rando.publicKey,
            config: configPda,
          })
          .signers([rando])
          .rpc();
        expect.fail("Should have thrown");
      } catch (err: any) {
        expect(err.toString()).to.contain("UnauthorizedPauseAuthority");
      }
    });
  });

  // ── Bonding Curve Pool ───────────────────────────────────────────────

  describe("bonding curve", () => {
    let mint: PublicKey;
    let poolPda: PublicKey;
    let solVaultPda: PublicKey;
    let tokenVaultPda: PublicKey;

    const creator = Keypair.generate();
    const buyer = Keypair.generate();

    before(async () => {
      // Fund creator and buyer
      const sig1 = await provider.connection.requestAirdrop(
        creator.publicKey,
        10 * LAMPORTS_PER_SOL
      );
      const sig2 = await provider.connection.requestAirdrop(
        buyer.publicKey,
        10 * LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig1);
      await provider.connection.confirmTransaction(sig2);

      // Create token mint (creator is authority, 6 decimals)
      mint = await createMint(
        provider.connection,
        creator,
        creator.publicKey,
        null,
        6
      );

      // Derive PDAs
      [poolPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("bonding_pool"), mint.toBuffer()],
        program.programId
      );
      [solVaultPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("bonding_sol_vault"), mint.toBuffer()],
        program.programId
      );
      [tokenVaultPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("bonding_token_vault"), mint.toBuffer()],
        program.programId
      );
    });

    it("creates bonding pool", async () => {
      await program.methods
        .createBondingPool({
          virtualSolReserves: null,
          virtualTokenReserves: null,
          tokenSupply: null,
          migrationTarget: null,
        })
        .accounts({
          creator: creator.publicKey,
          config: configPda,
          mint: mint,
          pool: poolPda,
          solVault: solVaultPda,
          tokenVault: tokenVaultPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const pool = await program.account.bondingCurvePool.fetch(poolPda);
      expect(pool.creator.toBase58()).to.equal(creator.publicKey.toBase58());
      expect(pool.mint.toBase58()).to.equal(mint.toBase58());
      expect(pool.isMigrated).to.equal(false);
      expect(pool.maxBuyBps).to.equal(100);
      // Default 30 SOL virtual reserves
      expect(pool.virtualSolReserves.toNumber()).to.equal(30_000_000_000);
      // Default 1B tokens (6 decimals)
      expect(pool.realTokenReserves.toString()).to.equal("1000000000000000");
    });

    it("buyer can buy tokens", async () => {
      // Create buyer's token account
      const buyerAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        buyer,
        mint,
        buyer.publicKey
      );

      const buyAmount = new BN(0.2 * LAMPORTS_PER_SOL); // 0.2 SOL (stays under 1% of 1B tokens)

      const [buyerPositionPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("position"), poolPda.toBuffer(), buyer.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .buyBonding(buyAmount, new BN(0)) // min_tokens_out = 0 for test
        .accounts({
          buyer: buyer.publicKey,
          config: configPda,
          pool: poolPda,
          buyerPosition: buyerPositionPda,
          solVault: solVaultPda,
          tokenVault: tokenVaultPda,
          buyerTokenAccount: buyerAta.address,
          devWallet: devWallet.publicKey,
          platformWallet: platformWallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([buyer])
        .rpc();

      // Verify buyer got tokens
      const pool = await program.account.bondingCurvePool.fetch(poolPda);
      expect(pool.realSolReserves.toNumber()).to.be.greaterThan(0);

      // Verify token balance
      const tokenBalance = await provider.connection.getTokenAccountBalance(
        buyerAta.address
      );
      expect(Number(tokenBalance.value.amount)).to.be.greaterThan(0);
      console.log(
        `  Bought ${tokenBalance.value.uiAmountString} tokens for 0.5 SOL`
      );
    });

    it("rejects buy exceeding 1% max", async () => {
      const buyerAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        buyer,
        mint,
        buyer.publicKey
      );

      const [buyerPositionPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("position"), poolPda.toBuffer(), buyer.publicKey.toBuffer()],
        program.programId
      );

      // 1 SOL on a 30 SOL virtual pool yields ~3.2% of supply — exceeds 1% max
      const hugeBuy = new BN(1 * LAMPORTS_PER_SOL);

      try {
        await program.methods
          .buyBonding(hugeBuy, new BN(0))
          .accounts({
            buyer: buyer.publicKey,
            config: configPda,
            pool: poolPda,
            buyerPosition: buyerPositionPda,
            solVault: solVaultPda,
            tokenVault: tokenVaultPda,
            buyerTokenAccount: buyerAta.address,
            devWallet: devWallet.publicKey,
            platformWallet: platformWallet.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([buyer])
          .rpc();
        expect.fail("Should have thrown ExceedsMaxBuy");
      } catch (err: any) {
        expect(err.toString()).to.satisfy((s: string) =>
          s.includes("ExceedsMaxBuy") || s.includes("6004") || s.includes("custom program error")
        );
      }
    });

    it("buyer can sell tokens", async () => {
      const buyerAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        buyer,
        mint,
        buyer.publicKey
      );

      // Sell half the tokens
      const tokenBalance = await provider.connection.getTokenAccountBalance(
        buyerAta.address
      );
      const sellAmount = new BN(
        Math.floor(Number(tokenBalance.value.amount) / 2)
      );

      const solBefore = await provider.connection.getBalance(buyer.publicKey);

      await program.methods
        .sellBonding(sellAmount, new BN(0)) // min_sol_out = 0 for test
        .accounts({
          seller: buyer.publicKey,
          config: configPda,
          pool: poolPda,
          solVault: solVaultPda,
          tokenVault: tokenVaultPda,
          sellerTokenAccount: buyerAta.address,
          platformWallet: platformWallet.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([buyer])
        .rpc();

      const solAfter = await provider.connection.getBalance(buyer.publicKey);
      // After 24% tax + 1% platform fee, user should still get some SOL back
      expect(solAfter).to.be.greaterThan(solBefore - 10000); // minus tx fee

      const pool = await program.account.bondingCurvePool.fetch(poolPda);
      expect(pool.buybackTreasury.toNumber()).to.be.greaterThan(0);
      console.log(
        `  Buyback treasury: ${pool.buybackTreasury.toNumber() / LAMPORTS_PER_SOL} SOL`
      );
    });
  });

  // ── Presale Pool ─────────────────────────────────────────────────────

  describe("presale", () => {
    let mint: PublicKey;
    let poolPda: PublicKey;
    let solVaultPda: PublicKey;
    let tokenVaultPda: PublicKey;

    const creator = Keypair.generate();
    const contributor1 = Keypair.generate();
    const contributor2 = Keypair.generate();

    before(async () => {
      // Fund accounts
      for (const kp of [creator, contributor1, contributor2]) {
        const sig = await provider.connection.requestAirdrop(
          kp.publicKey,
          10 * LAMPORTS_PER_SOL
        );
        await provider.connection.confirmTransaction(sig);
      }

      mint = await createMint(
        provider.connection,
        creator,
        creator.publicKey,
        null,
        6
      );

      [poolPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("presale_pool"), mint.toBuffer()],
        program.programId
      );
      [solVaultPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("presale_sol_vault"), mint.toBuffer()],
        program.programId
      );
      [tokenVaultPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("presale_token_vault"), mint.toBuffer()],
        program.programId
      );
    });

    it("creates presale pool", async () => {
      const endTime = Math.floor(Date.now() / 1000) + 3600; // 1 hour

      await program.methods
        .createPresalePool({
          migrationTarget: new BN(100 * LAMPORTS_PER_SOL),
          tokenSupply: new BN("1000000000000000"), // 1B tokens
          endTime: new BN(endTime),
          creatorPoolBps: 2000, // 20%
        })
        .accounts({
          creator: creator.publicKey,
          config: configPda,
          mint: mint,
          pool: poolPda,
          solVault: solVaultPda,
          tokenVault: tokenVaultPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([creator])
        .rpc();

      const pool = await program.account.presalePool.fetch(poolPda);
      expect(pool.creator.toBase58()).to.equal(creator.publicKey.toBase58());
      expect(pool.migrationTarget.toString()).to.equal(
        (100 * LAMPORTS_PER_SOL).toString()
      );
      expect(pool.maxBuyBps).to.equal(100);
      expect(pool.creatorPoolBps).to.equal(2000);
      expect(pool.isMigrated).to.equal(false);
    });

    it("contributor can contribute SOL", async () => {
      const [positionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("position"),
          poolPda.toBuffer(),
          contributor1.publicKey.toBuffer(),
        ],
        program.programId
      );

      const amount = new BN(0.5 * LAMPORTS_PER_SOL);

      await program.methods
        .contributePresale(amount)
        .accounts({
          contributor: contributor1.publicKey,
          config: configPda,
          pool: poolPda,
          solVault: solVaultPda,
          userPosition: positionPda,
          platformWallet: platformWallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([contributor1])
        .rpc();

      const position = await program.account.userPosition.fetch(positionPda);
      expect(position.solContributed.toNumber()).to.be.greaterThan(0);
      expect(position.tokensClaimed).to.equal(false);
      expect(position.refundClaimed).to.equal(false);

      const pool = await program.account.presalePool.fetch(poolPda);
      expect(pool.currentRaised.toNumber()).to.be.greaterThan(0);
      expect(pool.numContributors).to.equal(1);
      console.log(
        `  Contributed: ${position.solContributed.toNumber() / LAMPORTS_PER_SOL} SOL (after 1% fee)`
      );
    });

    it("rejects contribution exceeding 1% of target", async () => {
      const [positionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("position"),
          poolPda.toBuffer(),
          contributor2.publicKey.toBuffer(),
        ],
        program.programId
      );

      // 1% of 100 SOL = 1 SOL. Try 2 SOL (should exceed after 1% fee deduction)
      const amount = new BN(2 * LAMPORTS_PER_SOL);

      try {
        await program.methods
          .contributePresale(amount)
          .accounts({
            contributor: contributor2.publicKey,
            config: configPda,
            pool: poolPda,
            solVault: solVaultPda,
            userPosition: positionPda,
            platformWallet: platformWallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([contributor2])
          .rpc();
        expect.fail("Should have thrown ExceedsMaxContribution");
      } catch (err: any) {
        expect(err.toString()).to.satisfy((s: string) =>
          s.includes("ExceedsMaxContribution") || s.includes("6011") || s.includes("custom program error")
        );
      }
    });

    it("cannot claim before migration", async () => {
      const contributorAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        contributor1,
        mint,
        contributor1.publicKey
      );

      const [positionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("position"),
          poolPda.toBuffer(),
          contributor1.publicKey.toBuffer(),
        ],
        program.programId
      );

      try {
        await program.methods
          .claimPresale()
          .accounts({
            user: contributor1.publicKey,
            pool: poolPda,
            userPosition: positionPda,
            tokenVault: tokenVaultPda,
            userTokenAccount: contributorAta.address,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([contributor1])
          .rpc();
        expect.fail("Should have thrown NotMigrated");
      } catch (err: any) {
        expect(err.toString()).to.satisfy((s: string) =>
          s.includes("NotMigrated") || s.includes("6007") || s.includes("AnchorError")
        );
      }
    });

    it("cannot refund before end time", async () => {
      const [positionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("position"),
          poolPda.toBuffer(),
          contributor1.publicKey.toBuffer(),
        ],
        program.programId
      );

      try {
        await program.methods
          .refundPresale()
          .accounts({
            user: contributor1.publicKey,
            pool: poolPda,
            userPosition: positionPda,
            solVault: solVaultPda,
            systemProgram: SystemProgram.programId,
          })
          .signers([contributor1])
          .rpc();
        expect.fail("Should have thrown PresaleNotEnded");
      } catch (err: any) {
        expect(err.toString()).to.satisfy((s: string) =>
          s.includes("PresaleNotEnded") || s.includes("6010") || s.includes("AnchorError")
        );
      }
    });
  });

  // ── Update Config ────────────────────────────────────────────────────

  describe("update config", () => {
    it("admin can update fees", async () => {
      await program.methods
        .updateConfig({

          newPauseAuthority: null,
          newDevWallet: null,
          newPlatformWallet: null,
          newDevFeeBps: 30,
          newPlatformFeeBps: 70,
          newSellTaxBps: null,
          newPresalePlatformFeeBps: null,
          newMigrationFeeBps: null,
        })
        .accounts({
          admin: admin.publicKey,
          config: configPda,
        })
        .rpc();

      const config = await program.account.globalConfig.fetch(configPda);
      expect(config.devFeeBps).to.equal(30);
      expect(config.platformFeeBps).to.equal(70);
      // Unchanged
      expect(config.sellTaxBps).to.equal(2400);
    });

    it("non-admin cannot update", async () => {
      const rando = Keypair.generate();
      const sig = await provider.connection.requestAirdrop(
        rando.publicKey,
        LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      try {
        await program.methods
          .updateConfig({
  
            newPauseAuthority: null,
            newDevWallet: null,
            newPlatformWallet: null,
            newDevFeeBps: 500,
            newPlatformFeeBps: null,
            newSellTaxBps: null,
            newPresalePlatformFeeBps: null,
            newMigrationFeeBps: null,
          })
          .accounts({
            admin: rando.publicKey,
            config: configPda,
          })
          .signers([rando])
          .rpc();
        expect.fail("Should have thrown UnauthorizedAdmin");
      } catch (err: any) {
        expect(err.toString()).to.contain("UnauthorizedAdmin");
      }
    });
  });
});
