/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `net/if.h`

use crate::dyld::FunctionExports;
use crate::export_c_func;
use crate::mem::{ConstPtr, Ptr};
use crate::Environment;

// TODO: struct definition
#[allow(non_camel_case_types)]
struct if_nameindex {}

fn if_nameindex(_env: &mut Environment) -> ConstPtr<if_nameindex> {
    // TODO: implement
    Ptr::null()
}

fn if_nametoindex(_env: &mut Environment, _ifname: ConstPtr<u8>) -> u32 {
    // Pretend every interface (e.g. "en0") exists with index 1, so that
    // MAC-address lookups via sysctl() can continue.
    1
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(if_nameindex()),
    export_c_func!(if_nametoindex(_)),
];
