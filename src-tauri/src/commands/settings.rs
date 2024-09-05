use std::net::IpAddr;

use indexmap::IndexMap;
use itertools::Itertools;
use sage_config::{NetworkConfig, WalletConfig};
use specta::specta;
use tauri::{command, State};

use crate::{
    app_state::AppState,
    error::Result,
    models::{NetworkInfo, PeerInfo},
};

#[command]
#[specta]
pub async fn peer_list(state: State<'_, AppState>) -> Result<Vec<PeerInfo>> {
    let state = state.lock().await;
    let peer_state = state.peer_state.lock().await;

    Ok(peer_state
        .peers()
        .sorted_by_key(|peer| peer.socket_addr().ip())
        .map(|peer| PeerInfo {
            ip_addr: peer.socket_addr().ip().to_string(),
            port: peer.socket_addr().port(),
            trusted: false,
        })
        .collect())
}

#[command]
#[specta]
pub async fn remove_peer(state: State<'_, AppState>, ip_addr: IpAddr, ban: bool) -> Result<()> {
    let state = state.lock().await;
    let mut peer_state = state.peer_state.lock().await;

    if ban {
        peer_state.ban(ip_addr);
    } else {
        peer_state.remove_peer(ip_addr);
    }

    Ok(())
}

#[command]
#[specta]
pub async fn network_list(state: State<'_, AppState>) -> Result<IndexMap<String, NetworkInfo>> {
    let state = state.lock().await;

    let mut networks = IndexMap::new();

    for (network_id, network) in &state.networks {
        let info = NetworkInfo {
            default_port: network.default_port,
            genesis_challenge: hex::encode(network.genesis_challenge),
            agg_sig_me: network.agg_sig_me.map(hex::encode),
            dns_introducers: network.dns_introducers.clone(),
        };
        networks.insert(network_id.clone(), info);
    }

    Ok(networks)
}

#[command]
#[specta]
pub async fn network_config(state: State<'_, AppState>) -> Result<NetworkConfig> {
    let state = state.lock().await;
    Ok(state.config.network.clone())
}

#[command]
#[specta]
pub async fn set_discover_peers(state: State<'_, AppState>, discover_peers: bool) -> Result<()> {
    let mut state = state.lock().await;

    if state.config.network.discover_peers != discover_peers {
        state.config.network.discover_peers = discover_peers;
        state.save_config()?;
        state.reset_sync_task(false)?;
    }

    Ok(())
}

#[command]
#[specta]
pub async fn set_target_peers(state: State<'_, AppState>, target_peers: u32) -> Result<()> {
    let mut state = state.lock().await;

    state.config.network.target_peers = target_peers;
    state.save_config()?;
    state.reset_sync_task(false)?;

    Ok(())
}

#[command]
#[specta]
pub async fn set_network_id(state: State<'_, AppState>, network_id: String) -> Result<()> {
    let mut state = state.lock().await;

    state.config.network.network_id = network_id;
    state.save_config()?;
    state.reset_sync_task(true)?;
    state.switch_wallet().await?;

    Ok(())
}

#[command]
#[specta]
pub async fn wallet_config(state: State<'_, AppState>, fingerprint: u32) -> Result<WalletConfig> {
    let state = state.lock().await;
    state.try_wallet_config(fingerprint).cloned()
}

#[command]
#[specta]
pub async fn set_derive_automatically(
    state: State<'_, AppState>,
    fingerprint: u32,
    derive_automatically: bool,
) -> Result<()> {
    let mut state = state.lock().await;

    let config = state.try_wallet_config_mut(fingerprint)?;

    if config.derive_automatically != derive_automatically {
        config.derive_automatically = derive_automatically;
        state.save_config()?;
    }

    Ok(())
}

#[command]
#[specta]
pub async fn set_derivation_batch_size(
    state: State<'_, AppState>,
    fingerprint: u32,
    derivation_batch_size: u32,
) -> Result<()> {
    let mut state = state.lock().await;

    let config = state.try_wallet_config_mut(fingerprint)?;
    config.derivation_batch_size = derivation_batch_size;
    state.save_config()?;

    // TODO: Only if needed.
    state.reset_sync_task(false)?;

    Ok(())
}

#[command]
#[specta]
pub async fn rename_wallet(
    state: State<'_, AppState>,
    fingerprint: u32,
    name: String,
) -> Result<()> {
    let mut state = state.lock().await;

    let config = state.try_wallet_config_mut(fingerprint)?;
    config.name = name;
    state.save_config()?;

    Ok(())
}