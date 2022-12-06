#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{execute_join_swap_extern, execute_exit_swap_share};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
// use crate::query::{query_owner, query_route, test_twap};
// use crate::state::{State, STATE, SWAP_REPLY_STATES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:yf-test-on-osmosis";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Msg Reply IDs
pub const SWAP_REPLY_ID: u64 = 1u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _: InstantiateMsg,
) -> Result<Response, ContractError> {
    // set contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // return OK
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::JoinSwapExtern {
            pool_id,
            token_in,
            share_our_min_amount,
            // slipage,
        } => execute_join_swap_extern(deps, env, info, pool_id, token_in, share_our_min_amount),
        ExecuteMsg::ExitSwapShare {
            pool_id,
            token_out_denom,
            share_in_amount,
            token_out_min_amount,
        } => execute_exit_swap_share(deps, env, info, pool_id, token_out_denom, share_in_amount, token_out_min_amount),
    }
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     // match msg {
//     // }
// }

