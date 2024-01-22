use namada_core::types::address::Address;
use namada_core::types::dec::Dec;
use namada_core::types::token;
use namada_core::types::token::Amount;
use namada_storage as storage;
use namada_storage::{StorageRead, StorageWrite};

use crate::storage_key::*;

/// Initialize parameters for the token in storage during the genesis block.
pub fn write_params<S>(
    params: &token::Parameters,
    storage: &mut S,
    address: &Address,
) -> storage::Result<()>
where
    S: StorageRead + StorageWrite,
{
    let token::Parameters {
        max_reward_rate: max_rate,
        kd_gain_nom,
        kp_gain_nom,
        locked_ratio_target: locked_target,
    } = params;
    storage.write_without_merkle_diffs(
        &masp_last_inflation_key(address),
        Amount::zero(),
    )?;
    storage.write_without_merkle_diffs(
        &masp_last_locked_ratio_key(address),
        Dec::zero(),
    )?;
    storage.write_without_merkle_diffs(
        &masp_max_reward_rate_key(address),
        max_rate,
    )?;
    storage.write_without_merkle_diffs(
        &masp_locked_ratio_target_key(address),
        locked_target,
    )?;
    storage
        .write_without_merkle_diffs(&masp_kp_gain_key(address), kp_gain_nom)?;
    storage
        .write_without_merkle_diffs(&masp_kd_gain_key(address), kd_gain_nom)?;
    Ok(())
}
