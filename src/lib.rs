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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_types;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;
