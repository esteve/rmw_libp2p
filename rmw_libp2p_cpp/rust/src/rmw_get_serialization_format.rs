// Copyright 2024 Esteve Fernandez All rights reserved.
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

use std::os::raw::c_char;
use stdext::function_name;

#[no_mangle]
pub extern "C" fn rmw_get_serialization_format() -> *const c_char {
    debug!(target: "rmw_libp2p_cpp", function_name!());
    unsafe { libp2p_serialization_format }
}
