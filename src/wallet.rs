// Ultraviolet: post-blockchain P2P smart contracts
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2024 by Dr Maxim Orlovsky <orlovsky@uviolet.net>
// Written in 2024 by Dr Maxim Orlovsky <orlovsky@uviolet.net>
//
// Copyright (C) 2024-2025 Ultraviolet Alliance. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use std::collections::BTreeSet;

/////
pub struct ContractId;

pub struct TxPrefab {
    pub layer1: L1Prefab,
    pub sonic: OpPrefab,
}
/////

pub trait ContractReader {}

pub trait ContractIface {}

pub trait Stockpile {
    fn contracts_list(&self) -> impl Iterator<Item = ContractInfo>;
    fn has_contract(&self, contract_id: ContractId) -> bool;
    fn contract(&self, contract_id: ContractId) -> Option<impl ContractReader>;

    fn import(&mut self, prefab: VerifiedKit);
    fn consume(&mut self, consignment: Consignment);
}

pub trait Possessor {
    
}

pub struct Portfolio<S: Stockpile, P: Possessor> {
    pub possessor: P,
    pub stockpile: S,
}

impl<Stock: Stockpile, Owner: OwnerAccount> Portfolio<Stock, Owner> {
    pub fn owned_assets(&self, iface: impl Into<IfaceName>) -> BTreeSet<ContractId, ContractAssets> {}
    pub fn prepare(&mut self, invoice: Invoice) -> Result<TxPrefab, FullfilmentError> {}
    pub fn consign(&mut self, op: Operation) -> Result<Consignment, ConsignError> {}

    /// Stores information about operation in stockpile, once the operation is signed and the witness is mined.
    pub fn store(&mut self, witness: Witness) -> Result<(), CompletionError> {}
}
