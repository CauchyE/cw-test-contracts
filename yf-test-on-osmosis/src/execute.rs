use crate::state::{};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, coin, to_binary, has_coins, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, IbcMsg,
    IbcQuery, MessageInfo, Order, PortIdResponse, Response, StdResult, Storage, Uint128, WasmMsg, Uint64,
};

use crate::contract::SWAP_REPLY_ID;
use crate::error::ContractError;
use crate::helpers::{
   generate_swap_msg, calculate_min_output_from_twap,
};
use crate::msg::Slipage;

pub fn execute_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    input_token: Coin,
    output_denom: String,
    slipage: Slipage,
) -> Result<Response, ContractError> {
    if !has_coins(&info.funds, &input_token) {
        return Err(ContractError::InsufficientFunds {});
    }

    let min_output_token = match slipage {
        Slipage::MaxSlipagePercentage(percentage) => calculate_min_output_from_twap(
            deps.as_ref(),
            input_token.clone(),
            output_denom,
            env.block.time,
            percentage,
        )?,
        Slipage::MinOutputAmount(minimum_output_amount) => {
            coin(minimum_output_amount.u128(), output_denom)
        }
    };

    // generate the swap_msg
    let swap_msg = generate_swap_msg(
        deps.as_ref(),
        env.contract.address,
        input_token,
        min_output_token,
    )?;

    // // save intermediate state for reply
    // SWAP_REPLY_STATES.save(
    //     deps.storage,
    //     SWAP_REPLY_ID,
    //     &SwapMsgReplyState {
    //         original_sender: info.sender,
    //         swap_msg: swap_msg.clone(),
    //     },
    // )?;

    // TODO: Should we handle the error here?
    Ok(Response::new()
        .add_attribute("action", "trade_with_slippage_limit")
        .add_message(swap_msg),
    )
        // .add_submessage(SubMsg::reply_on_success(swap_msg, SWAP_REPLY_ID)))
}
