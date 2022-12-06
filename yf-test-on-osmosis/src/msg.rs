use cosmwasm_std::{Coin, Decimal, Uint128, Uint64};
// use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Slipage {
    MaxSlipagePercentage(Decimal),
    MinOutputAmount(Uint128),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // let the contract send a MsgJoinSwapExternAmountIn to join a single
    // token into a pool with one msg
    JoinSwapExtern {
        pool_id: u64,
        token_in: Coin,
        share_our_min_amount: String,
        // slipage: Slipage,
    },
    // let the contract send a MsgExitSwapExternAmountOut to exit a single
    // token from a pool with one msg
    // ExitSwapShare {
        // pool_id: u64,
        // token_out_denom: String,
        // share_in_amount: String,
        // token-out-min-amount: String,
    // }

    // Deposit {
    //     pool: Uint64,
    //     share_min_out: Uint128,
    //     // duration: Uint64,
    // },
    // Lockup {
    //     timeout: Option<u64>,
    //     duration: Uint64,
    // }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // TestTwap {},
}

//  #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct TestTwapResponse {
//     pub price: String,
// }
