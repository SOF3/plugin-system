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

// This function can have any name, but argument names matter.
// All arguments must be of type &mut FeatureMap or Option<&mut FeatureMap>.
// The argument "this" refers to the FeatureMap of this plugin.
// Other argument names are the crate names (in underscore).
#[declare_plugin(for = "plugin_system_example_core")]
fn init(this: &mut FeatureMap) {
    //put_feature!(this, Command, MyCommand);
    this.put::<Box<dyn Command>>(Box::new(MyCommand));
}

#[derive(Debug)]
struct MyCommand;

impl Command for MyCommand {
    fn name(&self) -> String {
        "foo-cmd".into()
    }

    fn execute(&self) {
        println!("foo-cmd is executed");
    }
}
