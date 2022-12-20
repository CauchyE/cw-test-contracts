use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, coin, to_binary, has_coins, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg,
    IbcQuery, MessageInfo, Order, PortIdResponse, Response, StdResult, Storage, Uint128, WasmMsg, Uint64, Reply, SubMsg,
};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountInResponse;

// use crate::contract::SWAP_REPLY_ID;
use crate::error::ContractError;
use crate::helpers::{
   generate_join_swap_extern_msg, generate_exit_swap_share_amount_in,
};
use crate::contract::{JOIN_SWAP_REPLY_ID};
use crate::state::{SWAP_REPLY_STATES, SwapMsgReplyState};

pub fn execute_join_swap_extern(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pool_id: u64,
    token_in: Coin,
    share_out_min_amount: String,
    // slipage: Slipage,
) -> Result<Response, ContractError> {
    // check if user send enough fund in a tx to swap and join in a pool
    if !has_coins(&info.funds, &token_in) {
        return Err(ContractError::InsufficientFunds {coins: info.funds});
    }

    // generate the join_swap_extern_amount_in_msg
    let join_swap_extern_amount_in_msg = generate_join_swap_extern_msg(
        env.contract.address,
        pool_id,
        token_in,
        share_out_min_amount,
    )?;

    // record original sender in the state for the information of the later recording
    // of share amount for sender
    SWAP_REPLY_STATES.save(
        deps.storage,
        JOIN_SWAP_REPLY_ID,
        &SwapMsgReplyState {
            original_sender: info.sender,
        }
    )?;

    // TODO: Should we handle the error here?
    Ok(Response::new()
        .add_attribute("action", "trade and join in a pool")
        .add_submessage(SubMsg::reply_on_success(join_swap_extern_amount_in_msg, JOIN_SWAP_REPLY_ID))
    )
    
        // .add_submessage(SubMsg::reply_on_success(swap_msg, SWAP_REPLY_ID)))
}

pub fn execute_exit_swap_share(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    pool_id: u64,
    token_out_denom: String,
    share_in_amount: String,
    token_out_min_amount: String,
) -> Result<Response, ContractError> {
    let exit_swap_share_amount_in_msg = generate_exit_swap_share_amount_in(
        env.contract.address,
        pool_id,
        token_out_denom,
        share_in_amount,
        token_out_min_amount,
    )?;

    Ok(Response::new()
        .add_attribute("action", "exit_swap_share_amount_in")
        .add_message(exit_swap_share_amount_in_msg)
    )   
}


pub fn handle_join_swap_reply(
    _deps: DepsMut,
    msg: Reply,
    swap_msg_reply_state: SwapMsgReplyState,
) -> Result<Response, ContractError> {
    let res  = msg.result.unwrap();
    // let joined_amount = Uint128::from_str(&res.token_out_amount)?;
    // let depositor = res.events[0].clone();
    Ok(Response::new())
}

// pub fn handle_exit_swap_reply(s
//     _deps: DepsMut,
//     msg: Reply,
// ) -> Result<Response, ContractError> {
//     Ok(Response::new())
// }
