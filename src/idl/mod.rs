pub mod idl {
    use anchor_lang::prelude::*;

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct Asset {
        pub token_address: String,
        pub amount: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
    pub struct AssetInfo {
        pub owner: String,
        pub supply_no: u64,
        pub assets: Vec<Asset>,
        pub start_time: i64,
        pub mint_account: String,
        pub token_account: String,
        pub token_address: String,
        pub amount: u64,
    }

    declare_id!("E6qJKiFiq2f23Qf9Zx4cuqJTeJasi5XVrV1wvge6BZjc");

    pub mod program {
        pub mod types {
            use anchor_lang::prelude::*;

            #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
            pub struct Asset {
                pub token_address: String,
                pub amount: u64,
            }
        }

        pub mod events {
            use anchor_lang::prelude::*;
            use super::super::AssetInfo;

            #[event]
            pub struct MintEvent {
                pub asset_account: Pubkey,
                pub mint_account: Pubkey,
                pub token_account: Pubkey,
            }

            #[event]
            pub struct BurnEvent {
                pub asset_account: Pubkey,
                pub mint_account: Pubkey,
                pub token_account: Pubkey,
            }

            #[event]
            pub struct WrapEvent {
                pub asset_account: Pubkey,
                pub owner: Pubkey,
                pub asset_info: AssetInfo,
            }

            #[event]
            pub struct StakeEvent {
                pub asset_account: Pubkey,
                pub stake_account: Pubkey,
                pub mint_account: Pubkey,
                pub owner_account: Pubkey,
                pub authority_account: Pubkey,
                pub owner_token_account: Pubkey,
                pub staker_token_account: Pubkey,
            }

            #[event]
            pub struct UnstakeEvent {
                pub asset_account: Pubkey,
                pub stake_account: Pubkey,
                pub mint_account: Pubkey,
                pub owner_account: Pubkey,
                pub authority_account: Pubkey,
                pub owner_token_account: Pubkey,
                pub staker_token_account: Pubkey,
            }
        }

        pub mod client {
            pub mod args {
                use anchor_lang::prelude::*;

                #[derive(AnchorDeserialize)]
                pub struct InitAssetManager {
                    pub limit: u64,
                    pub contract_uri: String,
                    pub start_time: i64,
                    pub end_time: i64,
                    pub wrap_fee: u64,
                    pub unwrap_fee: u64,
                }
            }
        }
    }
}

