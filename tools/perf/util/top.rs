//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/top.h
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


// SPDX-License-Identifier: GPL-2.0
pub const __PERF_TOP_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_top {
    pub tool: perf_tool,
    pub sb_evlist: *mut *mut evlist evlist,,
    pub record_opts: record_opts,
    pub evswitch: evswitch,
//
// Symbols will be added here in perf_event__process_sample and will
// get out after decayed.
//
    pub drop_total: u64 samples, lost, lost_total, drop,,
    pub us_samples: u64 kernel_samples,,
    pub exact_samples: u64,
    pub guest_kernel_samples: u64 guest_us_samples,,
    pub delay_secs: int print_entries, count_filter,,
    pub max_stack: c_int,
    pub zero: bool hide_kernel_symbols, hide_user_symbols,,

    pub use_tui: bool,

    pub use_stdio: bool,
    pub vmlinux_warned: bool,
    pub dump_symtab: bool,
    pub stitch_lbr: bool,
    pub sym_filter_entry: *mut hist_entry,
    pub sym_evsel: *mut evsel,
    pub session: *mut perf_session,
    pub winsize: winsize,
    pub realtime_prio: c_int,
    pub sym_filter: *const c_char,
    pub min_percent: float,
    pub nr_threads_synthesize: c_uint,
    pub uid_str: *const c_char,
    pub in: *mut ordered_events,
    pub data: [ordered_events; 2],
    pub rotate: bool,
    pub mutex: mutex,
    pub cond: cond,
    pub qe: },
}

extern "C" {
    pub fn perf_top__header_snprintf(top: *mut perf_top, bf: *mut c_char, size: usize) -> usize;
}
extern "C" {
    pub fn perf_top__reset_sample_counters(top: *mut perf_top);
}
