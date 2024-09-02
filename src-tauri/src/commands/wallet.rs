use chia_wallet_sdk::{decode_address, encode_address};
use tauri::{command, State};

use crate::{
    app_state::AppState,
    error::{Error, Result},
    models::{encode_xch_amount, CoinData, DidData, NftData, SyncInfo},
};

#[command]
pub async fn sync_info(state: State<'_, AppState>) -> Result<SyncInfo> {
    let state = state.lock().await;
    let wallet = state.wallet.as_ref().ok_or(Error::NoActiveWallet)?;

    let mut tx = wallet.db.tx().await?;

    let balance = tx.p2_balance().await?;
    let total_coins = tx.total_coin_count().await?;
    let synced_coins = tx.synced_coin_count().await?;

    let max = tx.derivation_index(false).await? - 1;
    let max_used = tx.max_used_derivation_index(false).await?;
    let mut index = max_used.map_or(0, |i| i + 1);
    if index > max {
        index = max;
    }
    let p2_puzzle_hash = tx.p2_puzzle_hash(index, false).await?;

    tx.commit().await?;

    Ok(SyncInfo {
        address: encode_address(p2_puzzle_hash.to_bytes(), state.prefix())?,
        balance: encode_xch_amount(balance),
        ticker: state.prefix().to_string().to_uppercase(),
        total_coins,
        synced_coins,
    })
}

#[command]
pub async fn coin_list(state: State<'_, AppState>) -> Result<Vec<CoinData>> {
    let state = state.lock().await;
    let wallet = state.wallet.as_ref().ok_or(Error::NoActiveWallet)?;

    let coin_states = wallet.db.p2_coin_states().await?;

    coin_states
        .into_iter()
        .map(|cs| {
            Ok(CoinData {
                coin_id: cs.coin.coin_id(),
                address: encode_address(cs.coin.puzzle_hash.to_bytes(), state.prefix())?,
                created_height: cs.created_height,
                spent_height: cs.spent_height,
                amount: encode_xch_amount(cs.coin.amount as u128),
            })
        })
        .collect()
}

#[command]
pub async fn did_list(state: State<'_, AppState>) -> Result<Vec<DidData>> {
    let state = state.lock().await;
    let wallet = state.wallet.as_ref().ok_or(Error::NoActiveWallet)?;

    let mut did_data = Vec::new();

    let mut tx = wallet.db.tx().await?;

    let did_ids = tx.did_list().await?;

    for did_id in did_ids {
        let did = tx.did_coin(did_id).await?.ok_or(Error::CoinStateNotFound)?;
        did_data.push(DidData {
            encoded_id: encode_address(did.info.launcher_id.to_bytes(), "did:chia:")?,
            launcher_id: did.info.launcher_id,
            address: encode_address(did.info.p2_puzzle_hash.to_bytes(), state.prefix())?,
        });
    }

    tx.commit().await?;

    Ok(did_data)
}

#[command]
pub async fn nft_list(state: State<'_, AppState>) -> Result<Vec<NftData>> {
    let state = state.lock().await;
    let wallet = state.wallet.as_ref().ok_or(Error::NoActiveWallet)?;

    let mut nft_data = Vec::new();

    let mut tx = wallet.db.tx().await?;

    let nft_ids = tx.nft_list().await?;

    for nft_id in nft_ids {
        let nft = tx.nft_coin(nft_id).await?.ok_or(Error::CoinStateNotFound)?;
        nft_data.push(NftData {
            encoded_id: encode_address(nft.info.launcher_id.to_bytes(), "nft")?,
            launcher_id: nft.info.launcher_id,
            address: encode_address(nft.info.p2_puzzle_hash.to_bytes(), state.prefix())?,
        });
    }

    tx.commit().await?;

    Ok(nft_data)
}

#[command]
pub async fn validate_address(state: State<'_, AppState>, address: String) -> Result<bool> {
    let state = state.lock().await;
    let Some((_puzzle_hash, prefix)) = decode_address(&address).ok() else {
        return Ok(false);
    };
    Ok(prefix == state.prefix())
}
