//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/callchain.h
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
pub enum perf_call_graph_mode {
    CALLCHAIN_NONE,
    CALLCHAIN_FP,
    CALLCHAIN_DWARF,
    CALLCHAIN_LBR,
    CALLCHAIN_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chain_mode {
    CHAIN_NONE,
    CHAIN_FLAT,
    CHAIN_GRAPH_ABS,
    CHAIN_GRAPH_REL,
    CHAIN_FOLDED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chain_order {
    ORDER_CALLER,
    ORDER_CALLEE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callchain_node {
    pub parent: *mut callchain_node,
    pub val: list_head,
    pub parent_val: list_head,
    pub /: *mut *mut rb_node rb_node_in; / to insert nodes in an rbtree,
    pub /: *mut *mut rb_node rb_node; / to sort nodes in an output tree,
    pub /: *mut *mut rb_root rb_root_in; / input tree of children,
    pub /: *mut *mut rb_root rb_root; / sorted output tree of children,
    pub val_nr: c_uint,
    pub count: c_uint,
    pub children_count: c_uint,
    pub hit: u64,
    pub children_hit: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callchain_root {
    pub max_depth: u64,
    pub node: callchain_node,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chain_key {
    CCKEY_FUNCTION,
    CCKEY_ADDRESS,
    CCKEY_SRCLINE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chain_value {
    CCVAL_PERCENT,
    CCVAL_PERIOD,
    CCVAL_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callchain_param {
    pub enabled: bool,
    pub defer: bool,
    pub record_mode: perf_call_graph_mode,
    pub dump_size: u32,
    pub mode: chain_mode,
    pub max_stack: u16,
    pub print_limit: u32,
    pub min_percent: double,
    pub sort: sort_chain_func_t,
    pub order: chain_order,
    pub order_set: bool,
    pub key: chain_key,
    pub branch_callstack: bool,
    pub value: chain_value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callchain_list {
    pub list: list_head,
    pub ip: u64,
    pub ms: map_symbol,
    pub srcline: *const c_char,
    pub branch_count: u64,
    pub from_count: u64,
    pub cycles_count: u64,
    pub iter_count: u64,
    pub iter_cycles: u64,
    pub brtype_stat: *mut branch_type_stat,
    pub predicted_count: u64,
    pub abort_count: u64,
    pub unfolded: bool,
    pub has_children: bool,
}

//
// A callchain cursor is a single linked list that
// let one feed a callchain progressively.
// It keeps persistent allocated entries to minimize
// allocations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callchain_cursor_node {
    pub ip: u64,
    pub ms: map_symbol,
    pub srcline: *const c_char,
// Indicate valid cursor node for LBR stitch
    pub valid: bool,
    pub branch: bool,
    pub branch_flags: branch_flags,
    pub branch_from: u64,
    pub nr_loop_iter: c_int,
    pub iter_cycles: u64,
    pub next: *mut callchain_cursor_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stitch_list {
    pub node: list_head,
    pub cursor: callchain_cursor_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct callchain_cursor {
    pub nr: u64,
    pub first: *mut callchain_cursor_node,
    pub last: *mut callchain_cursor_node,
    pub pos: u64,
    pub curr: *mut callchain_cursor_node,
}

extern "C" {
    pub fn callchain_register_param(param: *mut callchain_param) -> c_int;
}
extern "C" {
    pub fn callchain_cursor_reset(cursor: *mut callchain_cursor);
}
// Close a cursor writing session. Initialize for the reader
// Cursor reading iteration helpers
extern "C" {
    pub fn hist_entry__append_callchain(he: *mut hist_entry, sample: *mut perf_sample) -> c_int;
}
extern "C" {
    pub fn parse_callchain_record(arg: *const c_char, param: *mut callchain_param) -> c_int;
}
extern "C" {
    pub fn parse_callchain_record_opt(arg: *const c_char, param: *mut callchain_param) -> c_int;
}
extern "C" {
    pub fn parse_callchain_report_opt(arg: *const c_char) -> c_int;
}
extern "C" {
    pub fn parse_callchain_top_opt(arg: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_callchain_config(var: *const c_char, value: *const c_char) -> c_int;
}
// dest = *src;

extern "C" {
    pub fn arch_skip_callchain_idx(thread: *mut thread, chain: *mut ip_callchain) -> c_int;
}

extern "C" {
    pub fn free_callchain(root: *mut callchain_root);
}
extern "C" {
    pub fn callchain_cursor_cleanup(cursor: *mut callchain_cursor);
}
extern "C" {
    pub fn decay_callchain(root: *mut callchain_root);
}
extern "C" {
    pub fn callchain_node__make_parent_list(node: *mut callchain_node) -> c_int;
}
extern "C" {
    pub fn callchain_param_setup(sample_type: u64, e_machine: u16);
}
extern "C" {
    pub fn callchain_total_hits(hists: *mut hists) -> u64;
}
extern "C" {
    pub fn callchain_avg_cycles(cnode: *mut callchain_node) -> i64;
}
extern "C" {
    pub fn int(node: *mut *mut callchain_iter_fn)(struct callchain_cursor_node, data: *mut c_void) -> typedef;
}
