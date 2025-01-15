// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub burn_event_list: ::prost::alloc::vec::Vec<BurnEvent>,
    #[prost(message, repeated, tag="2")]
    pub mint_event_list: ::prost::alloc::vec::Vec<MintEvent>,
    #[prost(message, repeated, tag="3")]
    pub stake_event_list: ::prost::alloc::vec::Vec<StakeEvent>,
    #[prost(message, repeated, tag="4")]
    pub unstake_event_list: ::prost::alloc::vec::Vec<UnstakeEvent>,
    #[prost(message, repeated, tag="5")]
    pub wrap_event_list: ::prost::alloc::vec::Vec<WrapEvent>,
    #[prost(message, repeated, tag="6")]
    pub init_asset_manager_list: ::prost::alloc::vec::Vec<InitAssetManager>,
    #[prost(message, repeated, tag="7")]
    pub stake_list: ::prost::alloc::vec::Vec<Stake>,
    #[prost(message, repeated, tag="8")]
    pub unstake_list: ::prost::alloc::vec::Vec<Unstake>,
    #[prost(message, repeated, tag="9")]
    pub wrap_asset_list: ::prost::alloc::vec::Vec<WrapAsset>,
    #[prost(message, repeated, tag="10")]
    pub unwrap_asset_list: ::prost::alloc::vec::Vec<UnwrapAsset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub asset_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint_account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub asset_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint_account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub asset_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub stake_account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub mint_account: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner_account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub authority_account: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub owner_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub staker_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnstakeEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub asset_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub stake_account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub mint_account: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner_account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub authority_account: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub owner_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub staker_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrapEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub asset_info: ::core::option::Option<AssetInfo>,
    #[prost(string, tag="3")]
    pub asset_account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitAssetManager {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub limit: u64,
    #[prost(string, tag="3")]
    pub contract_uri: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub start_time: i64,
    #[prost(int64, tag="5")]
    pub end_time: i64,
    #[prost(uint64, tag="6")]
    pub wrap_fee: u64,
    #[prost(uint64, tag="7")]
    pub unwrap_fee: u64,
    #[prost(string, tag="8")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_asset_manager: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stake {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub stake_no: u64,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_asset_manager: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_asset: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_stake: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_mint_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_owner_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_authority_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unstake {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub stake_no: u64,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_asset_manager: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_asset: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_stake: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_mint_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_owner_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_authority_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrapAsset {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_asset_manager: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_asset: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnwrapAsset {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub supply_no: u64,
    #[prost(string, tag="3")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_asset_manager: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_asset: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInfo {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub supply_no: u64,
    #[prost(message, repeated, tag="3")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(int64, tag="4")]
    pub start_time: i64,
    #[prost(string, tag="5")]
    pub mint_account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(string, tag="1")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
}
// @@protoc_insertion_point(module)
