use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use serde::{Deserialize, Serialize};

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(HelloResp)]
    HelloWorld {},
    #[returns(GetPriceResp)]
    GetPrice { pair: String },
    #[returns(GetCountResp)]
    GetCount {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HelloResp {
    pub greeting: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GetPriceResp {
    pub exchange_rate: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GetCountResp {
    pub count: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {}
