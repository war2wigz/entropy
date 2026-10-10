# MyBarPool's deployment

This repository is [MyBarPool](https://mybarpool.com)'s fork of Regolith Labs' [Entropy](https://github.com/regolith-labs/entropy) at commit `f26ae03cccab6188effb0a170b8123cf4bb54c94`, the commit [verify.osec.io](https://verify.osec.io/status/3jSkUuYBoJzQPMEzTvkDFXCZUBksPamrVhrnHR9igu2X) reports for Regolith's mainnet deployment `3jSkUuYBoJzQPMEzTvkDFXCZUBksPamrVhrnHR9igu2X`. MyBarPool deploys it at **`ASo8r4EEFLPAMDk1w3XdKbEmq4c1GynbsHGa6RGG83fH`** because Regolith's deployment has the `Open` instruction disabled, so nobody else can create a `Var` on it.

The changes to the program are exactly four, each its own commit: `api/src/lib.rs` — `declare_id!` is the program id above; `program/src/lib.rs` — the `EntropyInstruction::Open` dispatcher arm is uncommented (the `Open` handler itself is Regolith's, unchanged, and the provider is still not required to sign, as Regolith left it); `program/src/lib.rs` — the `security.txt` block's `project_url`, `contacts`, `policy` and `source_code` point at MyBarPool and this repository; `program/src/sample.rs` (with the error in `api/src/error.rs`) — `Sample` fails with `SlotHashUnavailable` (`2`) when the `SlotHashes` sysvar has no entry for `end_at` (the slot is too recent — a bank's sysvar holds hashes up to the previous slot, so the entry appears in slot `end_at + 1` at the earliest —, was skipped, or is older than the sysvar's 512 entries), where Regolith's records `keccak(end_at)`, a value anyone can compute in advance, and the variable is sampled for good; here the variable stays unsampled for a retry or a replacement. The `Var` layout, `Reveal`, `Next`, `Close` and the keccak formulas are Regolith's code, unmodified; `Sample` differs in that one way. The repository also gains the Apache-2.0 licence text (declared in the Cargo manifests upstream but not shipped), a `NOTICE`, this README section, an updated `SECURITY.md` and CI workflows.

The provider — the party that picks each seed, publishes its commit and reveals it — is MyBarPool's keeper, and MyBarPool is also this deployment's upgrade authority. How the draw stays fair when the same party runs the provider and the program is explained in [`war2wigz/mybarpool`](https://github.com/war2wigz/mybarpool): [`docs/ARCHITECTURE.md` › Randomness](https://github.com/war2wigz/mybarpool/blob/main/docs/ARCHITECTURE.md) and [`docs/PROGRAM.md` §4.4](https://github.com/war2wigz/mybarpool/blob/main/docs/PROGRAM.md). In short, the consuming program records the commit on its own account before the end slot and later verifies the seed, the slot hash and the recomputed value itself, so this deployment can stop a draw but never steer one.

To verify: `cargo build-sbf --arch v3` from the workspace root with Agave 4.3.0 produces `target/deploy/entropy_program.so` with `readelf -h` showing `Flags: 0x3`; `solana-verify build --arch v3 --library-name entropy_program --base-image solanafoundation/solana-verifiable-build@sha256:12fd4c0a0790f0fc41ef74b0cdb6bccc167ba6137eb1adb749bac649481c86bd` reproduces it, and `solana-verify get-executable-hash target/deploy/entropy_program.so` prints the hash the [build workflow](.github/workflows/build.yml) prints and uploads for every commit. Once deployed, [verify.osec.io/status/ASo8r4EEFLPAMDk1w3XdKbEmq4c1GynbsHGa6RGG83fH](https://verify.osec.io/status/ASo8r4EEFLPAMDk1w3XdKbEmq4c1GynbsHGa6RGG83fH) shows the on-chain hash against this source.

Regolith's README follows unchanged.

---

# Entropy [WIP]

**Entropy** is a provably-fair random number generation protocol for Solana. It uses an commit-reveal scheme paired with slothash sampling strategy to generate random numbers onchain in a secure and cost-effective way.

## How it works

To create a new varaible, users must ping the Entropy API offchain and get a signed transaction from the provider. Users must sign this transaction and submit it to the chain to open the `Var` account. On the backend, the Entropy API will generate a set of N random numbers by hashing a psuedorandom number in a loop and returning the last value of the set. This value will be the first commit in the series, and the number N will represent the number of values the variable will take on over its lifetime.

The variable will be initialized with the commit provided by the Entropy API and the ending slot provided by the user. When that slot comes due, users should call `Sample` to sample the slothash from the chain and record it to the variable account. Only after the slothash has been sampled, the Entropy API will make the seed value available via a read interface. Users can fetch this value and submit it via the `Reveal` instruction to create the finalized variable value for end use. If the variable is initialized with `is_auto = true`, then the Entropy provider will automatically sample the slothash and reveal the seed without manual user action. After finalization, the variable can be read by any program via `value` property on the account.

The variable will then wait for the user to call `Next` to reset the variable for its next value. The slothash and finalized value will be reset to zero, and the recorded seed from the last value will become the commit for the next value. In this way, the Entropy API is "locked in" to all future seeds and cannot selectively manipulate specific outcomes. Likewise, the slothosh sampled at the ending slot is unknown to the Entropy API at the time of opening the variable, and thus the Entropy provider cannot know the results of future outcomes. Since the Entropy provider keeps future seed values secret until reveal, validators who provide the slothashes cannot favorably manipulate the outcome of the result either. Thus, as long as the Entropy API keeps its seed values secret (and does *not* run a Solana validator), the finalized variable values cannot be known to any party.


## API
- [`Consts`](api/src/consts.rs) – Program constants.
- [`Error`](api/src/error.rs) – Custom program errors.
- [`Event`](api/src/event.rs) – Custom program events.
- [`Instruction`](api/src/instruction.rs) – Declared instructions.

## Instructions
- [`Open`](program/src/open.rs) – Opens a new variable.
- [`Close`](program/src/close.rs) – Closes a variable account.
- [`Next`](program/src/next.rs) - Moves a variable to the next value.
- [`Reveal`](program/src/reveal.rs) – Reveals a seed.
- [`Sample`](program/src/sample.rs) - Samples the slothash.

## State
- [`Variable`](api/src/state/variable.rs) – Variable tracks a unique random variable.

## Get started

Compile your program:
```sh
steel build
```

Run unit and integration tests:
```sh
steel test
```
