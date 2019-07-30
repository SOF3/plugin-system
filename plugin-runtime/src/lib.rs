// plugin-system
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use plugin_runtime_codegen::*;

use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Iterator;
use std::marker::PhantomData;

use typemap::{DebugMap, Key};

#[derive(Debug, Default)]
pub struct PluginList(pub HashMap<TypeId, FeatureMap>);

impl PluginList {
    pub fn require<'a, 'b, T: 'static, F>(&'a mut self, f: F) -> &'b mut FeatureMap
    where
        F: FnOnce(&mut Self) -> FeatureMap,
        'a: 'b,
    {
        let id = TypeId::of::<T>();
        if !self.0.contains_key(&id) {
            let map = f(self);
            self.0.insert(id, map);
        }
        self.0.get_mut(&id).unwrap()
    }

    pub fn list<'a, B: Debug + 'static>(&'a mut self) -> impl Iterator<Item = &'a mut B> {
        self.0
            .iter_mut()
            .filter_map(|(_, map)| map.list())
            .flatten()
    }
}

pub trait PluginManifest {
    fn init(deps: &mut PluginList) -> FeatureMap;
}

#[macro_export]
macro_rules! require_plugin {
    ($list:expr, $name:ident) => {
        $list.require::<'_, '_, $name::PluginManifestImpl, _>(|list| {
            use plugin_runtime::PluginManifest;
            $name::PluginManifestImpl::init(list)
        })
    };
}

#[derive(Debug)]
pub struct FeatureMap(pub DebugMap);

impl Default for FeatureMap {
    fn default() -> Self {
        FeatureMap(DebugMap::custom())
    }
}

#[derive(Debug)]
struct FeatureEntry<B>(PhantomData<B>);

impl<B: 'static> Key for FeatureEntry<B> {
    type Value = Vec<B>;
}

impl FeatureMap {
    pub fn put<B: Debug + 'static>(&mut self, value: B) {
        if !self.0.contains::<FeatureEntry<B>>() {
            self.0.insert::<FeatureEntry<B>>(Vec::new());
        }
        self.0.get_mut::<FeatureEntry<B>>().unwrap().push(value);
    }

    pub fn list<'a, B: Debug + 'static>(&'a mut self) -> Option<impl Iterator<Item = &'a mut B>> {
        self.0
            .get_mut::<FeatureEntry<B>>()
            .map_or(None, |vec| Some(vec.iter_mut()))
    }
}

#[macro_export]
macro_rules! put_feature {
    ($map:expr, $trait:ty, $v:expr) => {
        $map.put::<Box<dyn $trait>>(Box::new($v));
    };
}
