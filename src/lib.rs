/* Copyright 2017 Outscale SAS
 *
 * This file is part of packetgraph-sys.
 *
 * Butterfly is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3 as published
 * by the Free Software Foundation.
 *
 * Butterfly is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Butterfly.  If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    extern crate libc;

    use super::*;
    use self::libc::{c_char, c_int};
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn sanity_check() {
        let errp: *const pg_error = ptr::null();
        let errp = errp as *mut *mut pg_error;

        let mut args = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();

        args.push(CString::new("-c1").unwrap());
        args.push(CString::new("-n1").unwrap());
        args.push(CString::new("--no-huge").unwrap());
        args.push(CString::new("--no-shconf").unwrap());

        let c_args = args.iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        unsafe {
            let ret = pg_start(c_args.len() as c_int,
                               c_args.as_ptr() as *mut *mut c_char,
                               errp);
            assert!(errp.is_null());
            assert_eq!(ret, 4);
        }

        // simple nop creation / destruction
        let name = CString::new("nop").unwrap();
        unsafe {
            let nop = pg_nop_new(name.as_ptr(), errp);
            assert!(errp.is_null());
            assert!(!nop.is_null());
            pg_brick_destroy(nop);
            pg_stop();
        }
    }
}
