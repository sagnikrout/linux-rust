//! Automatically rewritten from C to Rust
//! Source: mm/debug_page_ref.c
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

// Macro flag: #define CREATE_TRACE_POINTS

#[no_mangle]
pub unsafe extern "C" fn __page_ref_set(page: *mut page, v: c_int) {
    void __page_ref_set(struct page *page, int v)
    {
    trace_page_ref_set(page, v);
    }
    EXPORT_SYMBOL(__page_ref_set);
    EXPORT_TRACEPOINT_SYMBOL(page_ref_set);
#[no_mangle]
pub unsafe extern "C" fn __page_ref_mod(page: *mut page, v: c_int) {
    void __page_ref_mod(struct page *page, int v)
    {
    trace_page_ref_mod(page, v);
    }
    EXPORT_SYMBOL(__page_ref_mod);
    EXPORT_TRACEPOINT_SYMBOL(page_ref_mod);
#[no_mangle]
pub unsafe extern "C" fn __page_ref_mod_and_test(page: *mut page, v: c_int, ret: c_int) {
    void __page_ref_mod_and_test(struct page *page, int v, int ret)
    {
    trace_page_ref_mod_and_test(page, v, ret);
    }
    EXPORT_SYMBOL(__page_ref_mod_and_test);
    EXPORT_TRACEPOINT_SYMBOL(page_ref_mod_and_test);
#[no_mangle]
pub unsafe extern "C" fn __page_ref_mod_and_return(page: *mut page, v: c_int, ret: c_int) {
    void __page_ref_mod_and_return(struct page *page, int v, int ret)
    {
    trace_page_ref_mod_and_return(page, v, ret);
    }
    EXPORT_SYMBOL(__page_ref_mod_and_return);
    EXPORT_TRACEPOINT_SYMBOL(page_ref_mod_and_return);
#[no_mangle]
pub unsafe extern "C" fn __page_ref_mod_unless(page: *mut page, v: c_int, u: c_int) {
    void __page_ref_mod_unless(struct page *page, int v, int u)
    {
    trace_page_ref_mod_unless(page, v, u);
    }
    EXPORT_SYMBOL(__page_ref_mod_unless);
    EXPORT_TRACEPOINT_SYMBOL(page_ref_mod_unless);
#[no_mangle]
pub unsafe extern "C" fn __page_ref_freeze(page: *mut page, v: c_int, ret: c_int) {
    void __page_ref_freeze(struct page *page, int v, int ret)
    {
    trace_page_ref_freeze(page, v, ret);
    }
    EXPORT_SYMBOL(__page_ref_freeze);
    EXPORT_TRACEPOINT_SYMBOL(page_ref_freeze);
#[no_mangle]
pub unsafe extern "C" fn __page_ref_unfreeze(page: *mut page, v: c_int) {
    void __page_ref_unfreeze(struct page *page, int v)
    {
    trace_page_ref_unfreeze(page, v);
    }
    EXPORT_SYMBOL(__page_ref_unfreeze);
    EXPORT_TRACEPOINT_SYMBOL(page_ref_unfreeze);
