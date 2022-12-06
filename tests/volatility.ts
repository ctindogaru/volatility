import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Volatility } from "../target/types/volatility";
import { AggregatorAccount } from "@switchboard-xyz/switchboard-v2";
import { SwitchboardTestContext } from "@switchboard-xyz/sbv2-utils";

export const AGGREGATOR_PUBKEY: anchor.web3.PublicKey =
  new anchor.web3.PublicKey("GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR");

export const sleep = (ms: number): Promise<any> =>
  new Promise((s) => setTimeout(s, ms));

describe("volatility", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Volatility as Program<Volatility>;

  let switchboard: SwitchboardTestContext;

  before(async () => {
    try {
      switchboard = await SwitchboardTestContext.loadDevnetQueue(
        program.provider as anchor.AnchorProvider
      );
      console.log("devnet detected");
      return;
    } catch (error: any) {
      console.log(`Error: SBV2 Devnet - ${error.message}`);
    }
    // If fails, throw error
    throw new Error(`Failed to load the SwitchboardTestContext from devnet`);
  });

  it("Reads an aggregator history buffer", async () => {
    const aggregatorAccount = new AggregatorAccount({
      program: switchboard.program,
      publicKey: AGGREGATOR_PUBKEY,
    });
    const aggregator = await aggregatorAccount.loadData();

    const tx = await program.methods
      .calculateVolatility()
      .accounts({
        historyAccountInfo: aggregator.historyBuffer,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    await sleep(5000);

    const confirmedTxn = await program.provider.connection.getParsedTransaction(
      tx,
      "confirmed"
    );

    console.log(JSON.stringify(confirmedTxn?.meta?.logMessages, undefined, 2));
  });
});
