import { CosmWasmSigner, Link, testutils } from "@confio/relayer";
import { assert } from "@cosmjs/utils";
import test from "ava";
import { Order } from "cosmjs-types/ibc/core/channel/v1/channel";

const { osmosis: oldOsmo, setup, wasmd, randomAddress } = testutils;
const osmosis = { ...oldOsmo, minFee: "0.025uosmo" };

import {
  setupContracts,
  setupOsmosisClient,
} from "./utils";

let wasmIds: Record<string, number> = {};
let osmosisIds: Record<string, number> = {};

test.before(async (t) => {
  console.log("test before for creating osmo client with local osmo setup")
  // console.debug("Upload contracts to osmosis...");
  // const osmosisContracts = {
    // host: "./internal/simple_ica_host.wasm",
    // whitelist: "./external/cw1_whitelist.wasm",
  // };
  const osmosisSign = await setupOsmosisClient();
  console.log(osmosisSign)
  // osmosisIds = await setupContracts(osmosisSign, osmosisContracts);

  t.pass();
});