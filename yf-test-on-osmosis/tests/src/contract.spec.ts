import { CosmWasmSigner, Link, testutils } from "@confio/relayer";
import { assert } from "@cosmjs/utils";
import test from "ava";
// import { Order } from "cosmjs-types/ibc/core/channel/v1/channel";
import "./types/yfTestOnOsmosis.types";
const { osmosis: oldOsmo, setup, randomAddress } = testutils;
const osmosis = { ...oldOsmo, minFee: "0.025uosmo" };

import {
  setupContract,
  setupOsmosisClient,
} from "./utils";

let osmosisIds: Record<string, number> = {};

test.before(async (t) => {
  console.log("test before for creating osmo client with local osmo setup")
  console.debug("Upload contracts to osmosis...");
  const osmosisContract = {
    // this is used in m1
    yf_test: "../artifacts/yf_test_on_osmosis-aarch64.wasm",
  };
  const osmosisSigner = await setupOsmosisClient();
  osmosisIds = await setupContract(osmosisSigner, osmosisContract);

  t.pass();
});

interface SetupInfo {
  osmoClient: CosmWasmSigner;
  osmoContract: string;
}

async function demoSetup(): Promise<SetupInfo> {
  // instantiate ica host on osmosis
  const osmoClient = await setupOsmosisClient();
  const initHost = {};
  const { contractAddress: osmoContract } = await osmoClient.sign.instantiate(
    osmoClient.senderAddress,
    osmosisIds.yf_test,
    initHost,
    "simple yf test on osmosis",
    "auto"
  );
  
  return {
      osmoClient,
      osmoContract,
  };
}

test.serial("execute join_swap_extern msg", async (t) => {
  const { osmoClient, osmoContract } = await demoSetup();

  console.log(osmoContract)
  t.log(osmoClient);
});