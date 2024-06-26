use crate::{
    contract::{
        execute::{increment, reset},
        query::get_count,
        query::get_price,
    },
    error::ContractError,
    msgs::{ExecuteMsg, HelloResp, InstantiateMsg, QueryMsg},
    state::{State, COUNT},
};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_ownable::get_ownership;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.clone().as_str()))?;

    let state = State { count: msg.count };
    COUNT.save(deps.storage, &state)?;

    let funds_attribute = if info.funds.is_empty() {
        "none".to_string()
    } else {
        info.funds
            .iter()
            .map(|coin| format!("{}:{}", coin.denom, coin.amount))
            .collect::<Vec<String>>()
            .join(", ")
    };

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("funds", funds_attribute)
        .add_attribute("count", msg.count.to_string()))
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        HelloWorld {} => to_json_binary(&query::hello_world()?),
        GetPrice { pair } => to_json_binary(&get_price(deps, pair)?),
        Ownership {} => to_json_binary(&get_ownership(deps.storage)?),
        GetCount {} => to_json_binary(&get_count(deps)?),
    }
}

mod query {
    use super::*;

    use nibiru_std::proto::{nibiru::oracle::QueryExchangeRateRequest, NibiruStargateQuery};

    use crate::msgs::{GetCountResp, GetPriceResp};

    pub fn hello_world() -> StdResult<HelloResp> {
        let response = HelloResp {
            greeting: "Hello Nibiru Developers".to_owned(),
        };

        Ok(response)
    }

    pub fn get_price(deps: Deps, pair: String) -> StdResult<GetPriceResp> {
        let query_request = QueryExchangeRateRequest { pair: pair.clone() };

        let query = query_request.into_stargate_query()?;

        let response: GetPriceResp = deps.querier.query(&query)?;

        Ok(response)
    }

    pub fn get_count(deps: Deps) -> StdResult<GetCountResp> {
        let state = COUNT.load(deps.storage)?;

        Ok(GetCountResp { count: state.count })
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Increment {} => increment(deps, info),
        Reset { count } => reset(deps, info, count),
    }
}

mod execute {
    use super::*;
    pub fn increment(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Load the current state
        let mut state = COUNT.load(deps.storage)?;

        // Increment the count by 1 (you can adjust this logic as needed)
        state.count += 1;

        // Save the updated state
        COUNT.save(deps.storage, &state)?;

        let funds_attribute = if info.funds.is_empty() {
            "none".to_string()
        } else {
            info.funds
                .iter()
                .map(|coin| format!("{}:{}", coin.denom, coin.amount))
                .collect::<Vec<String>>()
                .join(", ")
        };

        Ok(Response::new()
            .add_attribute("method", "increment")
            .add_attribute("owner", info.sender)
            .add_attribute("funds", funds_attribute)
            .add_attribute("count", state.count.to_string()))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        // Create a new state with the specified count
        let state = State { count };

        // Save the new state
        COUNT.save(deps.storage, &state)?;

        Ok(Response::new()
            .add_attribute("method", "reset")
            .add_attribute("caller", info.sender)
            .add_attribute("count", count.to_string()))
    }
}
