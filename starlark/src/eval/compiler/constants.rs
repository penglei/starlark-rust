/*
 * Copyright 2019 The Starlark in Rust Authors.
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use gazebo::dupe::Dupe;
use once_cell::sync::Lazy;

use crate::{environment::Globals, values::FrozenValue};

#[derive(Clone, Copy, Dupe)]
pub(crate) struct Constants {
    pub(crate) fn_len: FrozenValue,
    pub(crate) fn_type: FrozenValue,
}

impl Constants {
    pub fn new() -> Self {
        static RES: Lazy<Constants> = Lazy::new(|| {
            let g = Globals::standard();
            Constants {
                fn_len: g.get_frozen("len").unwrap(),
                fn_type: g.get_frozen("type").unwrap(),
            }
        });
        *Lazy::force(&RES)
    }
}