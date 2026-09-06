//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/expr.h
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
pub const PARSE_CTX_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct expr_scanner_ctx {
    pub user_requested_cpu_list: *mut c_char,
    pub runtime: c_int,
    pub system_wide: bool,
    pub is_test: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expr_parse_ctx {
    pub ids: *mut hashmap,
    pub sctx: expr_scanner_ctx,
}

extern "C" {
    pub fn ids__free(ids: *mut hashmap);
}
extern "C" {
    pub fn ids__insert(ids: *mut hashmap, id: *const c_char) -> c_int;
}
//
// Union two sets of ids (hashmaps) and construct a third, freeing ids1 and
// ids2.
//
extern "C" {
    pub fn expr__ctx_clear(ctx: *mut expr_parse_ctx);
}
extern "C" {
    pub fn expr__ctx_free(ctx: *mut expr_parse_ctx);
}
extern "C" {
    pub fn expr__del_id(ctx: *mut expr_parse_ctx, id: *const c_char);
}
extern "C" {
    pub fn expr__add_id(ctx: *mut expr_parse_ctx, id: *const c_char) -> c_int;
}
extern "C" {
    pub fn expr__add_id_val(ctx: *mut expr_parse_ctx, id: *const c_char, val: double) -> c_int;
}
extern "C" {
    pub fn expr__add_ref(ctx: *mut expr_parse_ctx, ref: *mut metric_ref) -> c_int;
}
extern "C" {
    pub fn expr_id_data__value(data: *const expr_id_data) -> double;
}
extern "C" {
    pub fn expr_id_data__source_count(data: *const expr_id_data) -> double;
}
extern "C" {
    pub fn expr_id_data__aggr_nr(data: *const expr_id_data) -> double;
}
extern "C" {
    pub fn expr__get_literal(literal: *const c_char, ctx: *const expr_scanner_ctx) -> double;
}
extern "C" {
    pub fn expr__has_event(ctx: *const expr_parse_ctx, compute_ids: bool, id: *const c_char) -> double;
}
extern "C" {
    pub fn expr__strcmp_cpuid_str(ctx: *const expr_parse_ctx, compute_ids: bool, id: *const c_char) -> double;
}
