// Copyright 2024 Esteve Fernandez
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

mod cdr_buffer;
mod node;
mod publisher;
mod subscription;
mod rmw_get_serialization_format;

pub use cdr_buffer::*;
pub use node::*;
pub use publisher::*;
pub use subscription::*;
pub use rmw_get_serialization_format::*;

#[no_mangle]
pub static mut libp2p_identifier: *const ::std::os::raw::c_char = "rmw_libp2p_cpp".as_ptr() as *const ::std::os::raw::c_char;

#[no_mangle]
pub static mut libp2p_serialization_format: *const ::std::os::raw::c_char = "cdr".as_ptr() as *const ::std::os::raw::c_char;