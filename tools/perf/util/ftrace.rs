//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/ftrace.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_ftrace {
    pub evlist: *mut evlist,
    pub target: target,
    pub tracer: *const c_char,
    pub filters: list_head,
    pub notrace: list_head,
    pub graph_funcs: list_head,
    pub nograph_funcs: list_head,
    pub event_pair: list_head,
    pub profile_hash: *mut hashmap,
    pub percpu_buffer_size: c_ulong,
    pub inherit: bool,
    pub use_nsec: bool,
    pub bucket_range: c_uint,
    pub min_latency: c_uint,
    pub max_latency: c_uint,
    pub bucket_num: c_uint,
    pub hide_empty: bool,
    pub graph_depth: c_int,
    pub func_stack_trace: c_int,
    pub func_irq_info: c_int,
    pub graph_args: c_int,
    pub graph_retval: c_int,
    pub graph_retval_hex: c_int,
    pub graph_retaddr: c_int,
    pub graph_nosleep_time: c_int,
    pub graph_noirqs: c_int,
    pub graph_verbose: c_int,
    pub graph_thresh: c_int,
    pub graph_tail: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_entry {
    pub list: list_head,
    pub name: [c_char; ],
}

extern "C" {
    pub fn perf_ftrace__latency_prepare_bpf(ftrace: *mut perf_ftrace) -> c_int;
}
extern "C" {
    pub fn perf_ftrace__latency_start_bpf(ftrace: *mut perf_ftrace) -> c_int;
}
extern "C" {
    pub fn perf_ftrace__latency_stop_bpf(ftrace: *mut perf_ftrace) -> c_int;
}
extern "C" {
    pub fn perf_ftrace__latency_cleanup_bpf(ftrace: *mut perf_ftrace) -> c_int;
}

