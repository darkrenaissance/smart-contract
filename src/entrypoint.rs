/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2023 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use darkfi_sdk::{
    crypto::{ContractId, PublicKey},
    error::{ContractError, ContractResult},
    msg,
    pasta::pallas,
    db::set_return_data, ContractCall,
};
use darkfi_serial::{deserialize, Encodable, WriteExt};

use crate::ContractFunction;

// We define the necessary functions.
darkfi_sdk::define_contract!(
    init: init_contract,
    exec: process_instruction,
    apply: process_update,
    metadata: get_metadata
);

/// This entrypoint function runs when the contract is (re)deployed and initialized.
/// We use this function to initialize all the necessary databases and prepare them
/// with initial data if necessary. This is also the place where we bundle the zkas
/// circuits that are to be used with functions provided by the contract.
fn init_contract(cid: ContractId, ix: &[u8]) -> ContractResult {
    Ok(())
}

/// This function is used by the wasm VM's host to fetch the necessary metadata for
/// verifying signatures and zk proofs. The payload given here are all the contract
/// calls in the transaction.
fn get_metadata(cid: ContractId, ix: &[u8]) -> ContractResult {
    let (call_idx, calls): (u32, Vec<ContractCall>) = deserialize(ix)?;
    if call_idx >= calls.len() as u32 {
        msg!("Error: call_idx >= calls.len()");
        return Err(ContractError::Internal);
    }

    match ContractFunction::try_from(calls[call_idx as usize].data[0])? {
        ContractFunction::Hello => {
            let zk_public_inputs: Vec<(String, Vec<pallas::Base>)> = vec![];
            let signature_pubkeys: Vec<PublicKey> = vec![];

            // Serialize and return
            let mut metadata = vec![];
            zk_public_inputs.encode(&mut metadata)?;
            signature_pubkeys.encode(&mut metadata)?;

            set_return_data(&metadata)?;
            Ok(())
        }
    }
}

/// This function should verify a state transition and produce a state update
/// if everything is successful. This step should happen **after** the host
/// has successfully verified the metadata from `get_metadata()`.
fn process_instruction(cid: ContractId, ix: &[u8]) -> ContractResult {
    let (call_idx, calls): (u32, Vec<ContractCall>) = deserialize(ix)?;
    if call_idx >= calls.len() as u32 {
        msg!("Error: call_idx >= calls.len()");
        return Err(ContractError::Internal);
    }

    match ContractFunction::try_from(calls[call_idx as usize].data[0])? {
        ContractFunction::Hello => {
            msg!("gm world");

            // Create a state update
            let mut update_data = vec![];
            update_data.write_u8(ContractFunction::Hello as u8)?;
            set_return_data(&update_data)?;
            Ok(())
        }
    }
}

/// This function attempts to write a given state update provided the previous steps
/// of the contract call execution were successful. It's the last in line, and assumes
/// that the transaction/call was successful. The payload given to the function is the
/// update data retrieved from `process_instruction()`.
fn process_update(cid: ContractId, update_data: &[u8]) -> ContractResult {
    match ContractFunction::try_from(update_data[0])? {
        ContractFunction::Hello => Ok(()),
    }
}
