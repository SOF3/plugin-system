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

#![recursion_limit = "256"]

plugin_system::load! {
    core "plugin-system-example-core" {
        version = "0.1.0",
        // the path is relative to $STAGE_DIR
        path = "../example/core",
    },

    plugin "plugin-system-example-plugins-bar" {
        version = "0.1.0",
        path = "../example/plugins/bar"
    },
}
