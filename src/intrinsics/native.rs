//! This module contains native implementations for intrinsics. These are used
//! when generated IR intrinsics are disabled.

mod add;
mod ctz;
mod divmod;
mod mul;
mod rot;
mod shl;
mod shr;
mod sub;
mod wide_int_div;
mod wide_int_div_nonconst;

pub use self::{add::*, ctz::*, divmod::*, mul::*, rot::*, shl::*, shr::*, sub::*};
