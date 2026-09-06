//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/symbol_conf.h
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
pub const __PERF_SYMBOL_CONF: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unwind_style {
    UNWIND_STYLE_UNKNOWN = 0,
    UNWIND_STYLE_LIBDW,
    UNWIND_STYLE_LIBUNWIND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum a2l_style {
    A2L_STYLE_UNKNOWN = 0,
    A2L_STYLE_LIBDW,
    A2L_STYLE_LLVM,
    A2L_STYLE_LIBBFD,
    A2L_STYLE_CMD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol_conf {
    pub nanosecs: bool,
    pub priv_size: c_ushort,
// kallsyms_name,
// source_prefix,
// field_sep,
// graph_function;
// default_guest_kallsyms,
// default_guest_modules;
    pub guestmount: *const c_char,
// comm_list_str,
// pid_list_str,
// tid_list_str,
// sym_list_str,
// parallelism_list_str,
// col_width_list_str,
// bt_stop_list_str;
    pub addr2line_path: *const c_char,
    pub addr2line_style: [a2l_style; MAX_A2L_STYLE],
    pub addr2line_timeout_ms: c_int,
    pub unwind_style: [unwind_style; MAX_UNWIND_STYLE],
    pub time_quantum: c_ulong,
// comm_list,
// sym_list,
// dso_from_list,
// dso_to_list,
// sym_from_list,
// sym_to_list,
// bt_stop_list;
// tid_list,
// addr_list;
    pub symfs: *const c_char,
    pub symfs_layout_flat: bool,
    pub res_sample: c_int,
    pub pad_output_len_dso: c_int,
    pub group_sort_idx: c_int,
    pub addr_range: c_int,
    pub 1): DECLARE_BITMAP(parallelism_filter, MAX_NR_CPUS +,
}
