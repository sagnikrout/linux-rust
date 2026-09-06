//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/branch.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


pub const _PERF_BRANCH_H: c_int = 1;
//
// The linux/stddef.h isn't need here, but is needed for __always_inline used
// in files included from uapi/linux/perf_event.h such as
// /usr/include/linux/swab.h and /usr/include/linux/byteorder/little_endian.h,
// detected in at least musl libc, used in Alpine Linux. -acme
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_flags {
    pub value: u64,
    pub mispred:1: u64,
    pub predicted:1: u64,
    pub in_tx:1: u64,
    pub abort:1: u64,
    pub cycles:16: u64,
    pub type:4: u64,
    pub spec:2: u64,
    pub new_type:4: u64,
    pub priv:3: u64,
    pub not_taken:1: u64,
    pub reserved:30: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_info {
    pub from: addr_map_symbol,
    pub to: addr_map_symbol,
    pub flags: branch_flags,
    pub branch_stack_cntr: u64,
    pub srcline_from: *mut c_char,
    pub srcline_to: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_entry {
    pub from: u64,
    pub to: u64,
    pub flags: branch_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_stack {
    pub nr: u64,
    pub hw_idx: u64,
    pub entries: [branch_entry; ],
}

//
// The hw_idx is only available when PERF_SAMPLE_BRANCH_HW_INDEX is applied.
// Otherwise, the output format of a sample with branch stack is
// struct branch_stack {
// u64			nr;
// struct branch_entry	entries[0];
// }
// Check whether the hw_idx is available,
// and return the corresponding pointer of entries[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_type_stat {
    pub branch_to: bool,
    pub counts: [u64; PERF_BR_MAX],
    pub new_counts: [u64; PERF_BR_NEW_MAX],
    pub cond_fwd: u64,
    pub cond_bwd: u64,
    pub cross_4k: u64,
    pub cross_2m: u64,
}

extern "C" {
    pub fn branch_type_stat_display(fp: *mut FILE, st: *const branch_type_stat);
}
extern "C" {
    pub fn branch_type_str(st: *const branch_type_stat, bf: *mut c_char, bfsize: c_int) -> c_int;
}
