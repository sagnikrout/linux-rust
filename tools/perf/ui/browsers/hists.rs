//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/ui/browsers/hists.h
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
pub const _PERF_UI_BROWSER_HISTS_H_: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hist_browser {
    pub b: ui_browser,
    pub hists: *mut hists,
    pub he_selection: *mut hist_entry,
    pub selection: *mut map_symbol,
    pub hbt: *mut hist_browser_timer,
    pub pstack: *mut pstack,
    pub env: *mut perf_env,
    pub block_evsel: *mut evsel,
    pub print_seq: c_int,
    pub show_dso: bool,
    pub show_headers: bool,
    pub min_pcnt: float,
    pub nr_non_filtered_entries: u64,
    pub nr_hierarchy_entries: u64,
    pub nr_callchain_rows: u64,
    pub c2c_filter: bool,
// Get title string.
    pub size): *mut *mut char bf, size_t,
}

extern "C" {
    pub fn hist_browser__delete(browser: *mut hist_browser);
}
