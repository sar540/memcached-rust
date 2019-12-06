#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(extern_types)]
extern crate libc;
extern "C" {
    /* * exported globals **/
    pub type stats;
    pub type stats_state;
    pub type settings;
    pub type slab_rebalance;
}

#[no_mangle]
pub static mut current_time: libc::c_int = 0;
#[no_mangle]
pub static mut slab_rebalance_signal: libc::c_int = 0;
