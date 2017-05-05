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
    use std::ffi::CString;
    use std::ptr;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn sanity_check() {
        unsafe {
            let mut errp: *mut pg_error = ptr::null_mut();
            let args = CString::new("-c1 -n1 --no-huge --no-shconf --lcores 0,1 -l 0,1").unwrap();
            let ret = pg_start_str(args.as_ptr());
            assert_eq!(ret,8);

            let name = CString::new("nop").unwrap();
            let nop = pg_nop_new(name.as_ptr(), &mut errp);
            assert!(!pg_error_is_set(&mut errp));
            assert!(!nop.is_null());
            
            let name = CString::new("fw").unwrap();
            let fw = pg_firewall_new(name.as_ptr(), 0, &mut errp);
            assert!(errp.is_null());
            assert!(!fw.is_null());

            let tmp = CString::new("/tmp").unwrap();
            pg_vhost_start(tmp.as_ptr(), &mut errp);
            assert!(errp.is_null());

            let name = CString::new("vhost").unwrap();
            let vhost = pg_vhost_new(name.as_ptr(),
                                     PG_VHOST_USER_DEQUEUE_ZERO_COPY as u64,
                                     &mut errp);
            assert!(errp.is_null());
            assert!(!vhost.is_null());

            pg_brick_link(nop, fw, &mut errp);
            assert!(!pg_error_is_set(&mut errp));
            pg_brick_link(fw, vhost, &mut errp);
            assert!(!pg_error_is_set(&mut errp));

            let name = CString::new("graph").unwrap();
            let graph = pg_graph_new(name.as_ptr(), nop, &mut errp);
            assert!(!pg_error_is_set(&mut errp));
            pg_graph_poll(graph, &mut errp);
            assert!(!pg_error_is_set(&mut errp));

            let thread = pg_thread_init(&mut errp);
            assert!(thread >= 0);
            assert!(!pg_error_is_set(&mut errp));
            pg_thread_add_graph(thread, graph);
            pg_thread_run(thread);
            sleep(Duration::from_secs(5));
            assert!(pg_thread_state(thread) == pg_thread_state::PG_THREAD_RUNNING);

            pg_thread_stop(thread);
            sleep(Duration::from_millis(100));
            assert!(pg_thread_state(thread) == pg_thread_state::PG_THREAD_STOPPED);
            pg_thread_destroy(thread);
            pg_graph_destroy(graph);
            pg_stop();
        }
    }
}
