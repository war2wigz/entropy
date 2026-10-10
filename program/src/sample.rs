use entropy_api::prelude::*;
use solana_program::{log::sol_log, slot_hashes::SlotHashes};
use steel::*;

pub fn process_sample(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, var_info, slot_hashes_sysvar] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let var = var_info
        .as_account_mut::<Var>(&entropy_api::ID)?
        .assert_mut(|v| clock.slot >= v.end_at)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Silent return.
    if var.slot_hash != [0; 32] {
        return Ok(());
    }

    // Deserialize the slot hashes.
    let slot_hashes =
        bincode::deserialize::<SlotHashes>(slot_hashes_sysvar.data.borrow().as_ref()).unwrap();

    // Record the sampled slot hash. This deployment differs from Regolith's here: when the
    // sysvar has no entry for `end_at` — the slot is too recent (a bank holds hashes up to the
    // previous slot, so the entry appears in slot `end_at + 1` at the earliest), was skipped, or
    // is older than the sysvar's window — Regolith's code records `keccak(end_at)`, a value
    // anyone can compute in advance, and the `Var` is sampled for good. A `Var` written that way
    // cannot be unwritten, and all three ways the lookup misses deserve a retry or a replacement,
    // never a sample; so this deployment fails instead and writes nothing.
    if let Some(slot_hash) = slot_hashes.get(&var.end_at) {
        var.slot_hash = slot_hash.to_bytes();
        sol_log(&format!(
            "Sampled hash at slot {:?}: {:?}",
            var.end_at,
            slot_hash.to_string()
        ));
    } else {
        sol_log(&format!(
            "No hash for slot {:?}: too early, skipped, or older than the sysvar window",
            var.end_at
        ));
        return Err(EntropyError::SlotHashUnavailable.into());
    }

    Ok(())
}
