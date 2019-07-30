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

use plugin_runtime::*;
use plugin_system_example_core::*;

#[declare_plugin(for = "plugin_system_example_core")]
fn init(this: &mut FeatureMap, _plugin_system_example_plugins_foo: &FeatureMap) {
    this.put::<Box<dyn Command>>(Box::new(MyCommand));
}

#[derive(Debug)]
struct MyCommand;

impl Command for MyCommand {
    fn name(&self) -> String {
        "bar-cmd".into()
    }

    fn execute(&self) {
        println!("bar-cmd is executed");
    }
}
