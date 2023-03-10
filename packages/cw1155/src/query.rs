use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128};
use cw0::Expiration;

use crate::msg::TokenId;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Cw1155QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    Balance { owner: HumanAddr, token_id: TokenId },
    /// Returns the current balance of the given address for a batch of tokens, 0 if unset.
    /// Return type: BatchBalanceResponse.
    BatchBalance {
        owner: HumanAddr,
        token_ids: Vec<TokenId>,
    },
    /// List all operators that can access all of the owner's tokens.
    /// Return type: ApprovedForAllResponse.
    ApprovedForAll {
        owner: HumanAddr,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
    },
    /// Query approved status `owner` granted to `operator`.
    /// Return type: IsApprovedForAllResponse
    IsApprovedForAll {
        owner: HumanAddr,
        operator: HumanAddr,
    },

    /// With MetaData Extension.
    /// Query metadata of token
    /// Return type: TokenInfoResponse.
    TokenInfo { token_id: TokenId },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    /// Return type: TokensResponse.
    Tokens {
        owner: HumanAddr,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    /// Return type: TokensResponse.
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BatchBalanceResponse {
    pub balances: Vec<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: HumanAddr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ApprovedForAllResponse {
    pub operators: Vec<Approval>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IsApprovedForAllResponse {
    pub approved: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokenInfoResponse {
    /// Should be a url point to a json file
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_from` in future queries
    /// to achieve pagination.
    pub tokens: Vec<TokenId>,
}
