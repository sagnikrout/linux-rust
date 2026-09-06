//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/sort.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sort_mode {
    SORT_MODE__NORMAL,
    SORT_MODE__BRANCH,
    SORT_MODE__MEMORY,
    SORT_MODE__TOP,
    SORT_MODE__DIFF,
    SORT_MODE__TRACEPOINT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sort_type {
// common sort keys
    SORT_PID,
    SORT_COMM,
    SORT_COMM_NODIGIT,
    SORT_DSO,
    SORT_SYM,
    SORT_PARENT,
    SORT_CPU,
    SORT_SOCKET,
    SORT_SRCLINE,
    SORT_SRCFILE,
    SORT_LOCAL_WEIGHT,
    SORT_GLOBAL_WEIGHT,
    SORT_TRANSACTION,
    SORT_TRACE,
    SORT_SYM_SIZE,
    SORT_DSO_SIZE,
    SORT_CGROUP,
    SORT_CGROUP_ID,
    SORT_SYM_IPC_NULL,
    SORT_TIME,
    SORT_CODE_PAGE_SIZE,
    SORT_LOCAL_INS_LAT,
    SORT_GLOBAL_INS_LAT,
    SORT_LOCAL_PIPELINE_STAGE_CYC,
    SORT_GLOBAL_PIPELINE_STAGE_CYC,
    SORT_ADDR,
    SORT_LOCAL_RETIRE_LAT,
    SORT_GLOBAL_RETIRE_LAT,
    SORT_SIMD,
    SORT_ANNOTATE_DATA_TYPE,
    SORT_ANNOTATE_DATA_TYPE_OFFSET,
    SORT_SYM_OFFSET,
    SORT_ANNOTATE_DATA_TYPE_CACHELINE,
    SORT_PARALLELISM,
    SORT_TGID,

// branch stack specific sort keys
    __SORT_BRANCH_STACK,
    SORT_DSO_FROM = __SORT_BRANCH_STACK,
    SORT_DSO_TO,
    SORT_SYM_FROM,
    SORT_SYM_TO,
    SORT_MISPREDICT,
    SORT_ABORT,
    SORT_IN_TX,
    SORT_CYCLES,
    SORT_SRCLINE_FROM,
    SORT_SRCLINE_TO,
    SORT_SYM_IPC,
    SORT_ADDR_FROM,
    SORT_ADDR_TO,
    SORT_CALLCHAIN_BRANCH_PREDICTED,
    SORT_CALLCHAIN_BRANCH_ABORT,
    SORT_CALLCHAIN_BRANCH_CYCLES,

// memory mode specific sort keys
    __SORT_MEMORY_MODE,
    SORT_MEM_DADDR_SYMBOL = __SORT_MEMORY_MODE,
    SORT_MEM_DADDR_DSO,
    SORT_MEM_LOCKED,
    SORT_MEM_TLB,
    SORT_MEM_LVL,
    SORT_MEM_SNOOP,
    SORT_MEM_DCACHELINE,
    SORT_MEM_IADDR_SYMBOL,
    SORT_MEM_PHYS_DADDR,
    SORT_MEM_DATA_PAGE_SIZE,
    SORT_MEM_BLOCKED,
}

//
// configurable sorting bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sort_entry {
    pub se_header: *const c_char,
    pub ): *mut *mut *mut int64_t (se_cmp)(struct hist_entry , struct hist_entry,
    pub ): *mut *mut *mut int64_t (se_collapse)(struct hist_entry , struct hist_entry,
    pub ): *mut *mut *mut int64_t (se_sort)(struct hist_entry , struct hist_entry,
    pub width): c_uint,
    pub arg): *const *const *const int (se_filter)(struct hist_entry he, int type, void,
    pub he): *mut *mut void (se_init)(struct hist_entry,
    pub se_width_idx: u8,
}

extern "C" {
    pub fn setup_sorting(evlist: *mut evlist, env: *mut perf_env) -> c_int;
}
extern "C" {
    pub fn setup_output_field() -> c_int;
}
extern "C" {
    pub fn reset_output_field();
}
extern "C" {
    pub fn sort__setup_elide(fp: *mut FILE);
}
extern "C" {
    pub fn perf_hpp__set_elide(idx: c_int, elide: bool);
}
extern "C" {
    pub fn report_parse_ignore_callees_opt(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
}
extern "C" {
    pub fn is_strict_order(order: *const c_char) -> bool;
}
extern "C" {
    pub fn hpp_dimension__add_output(col: unsigned, implicit: bool) -> c_int;
}
extern "C" {
    pub fn reset_dimensions();
}
extern "C" {
    pub fn output_field_add(list: *mut perf_hpp_list, tok: *const c_char, level: *mut c_int) -> c_int;
}
extern "C" {
    pub fn sort__comm_nodigit_len(entry: *mut hist_entry) -> usize;
}
