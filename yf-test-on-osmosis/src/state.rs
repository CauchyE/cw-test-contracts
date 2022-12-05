use osmosis_std::types::osmosis::gamm::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ROUTING_TABLE: Map<(&str, &str), Vec<SwapAmountInRoute>> = Map::new("routing_table");
