mod cdr_buffer;
mod node;
mod publisher;
mod subscription;

pub use cdr_buffer::*;
pub use node::*;
pub use publisher::*;
pub use subscription::*;

// // use env_logger::{Builder, Env};

// #[no_mangle]
// pub extern "C" fn rs_rmw_init() -> *const c_void {
//     let handle = task::spawn(async { event_loop().await });
//     Box::into_raw(Box::new(handle)) as *const c_void
// }
