//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ath/wil6210/debug.c
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2013,2016 Qualcomm Atheros, Inc.
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//

#[no_mangle]
pub unsafe extern "C" fn __wil_err(wil: *mut wil6210_priv, fmt: *const c_char, ...) {
    void __wil_err(struct wil6210_priv *wil, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    netdev_err(wil.main_ndev, "%pV", &vaf);
    trace_wil6210_log_err(&vaf);
    va_end(args);
    }
#[no_mangle]
pub unsafe extern "C" fn __wil_err_ratelimited(wil: *mut wil6210_priv, fmt: *const c_char, ...) {
    void __wil_err_ratelimited(struct wil6210_priv *wil, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    if (!net_ratelimit())
    return;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    netdev_err(wil.main_ndev, "%pV", &vaf);
    trace_wil6210_log_err(&vaf);
    va_end(args);
    }
#[no_mangle]
pub unsafe extern "C" fn wil_dbg_ratelimited(wil: *const wil6210_priv, fmt: *const c_char, ...) {
    void wil_dbg_ratelimited(const struct wil6210_priv *wil, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    if (!net_ratelimit())
    return;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    netdev_dbg(wil.main_ndev, "%pV", &vaf);
    trace_wil6210_log_dbg(&vaf);
    va_end(args);
    }
#[no_mangle]
pub unsafe extern "C" fn __wil_info(wil: *mut wil6210_priv, fmt: *const c_char, ...) {
    void __wil_info(struct wil6210_priv *wil, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    netdev_info(wil.main_ndev, "%pV", &vaf);
    trace_wil6210_log_info(&vaf);
    va_end(args);
    }
#[no_mangle]
pub unsafe extern "C" fn wil_dbg_trace(wil: *mut wil6210_priv, fmt: *const c_char, ...) {
    void wil_dbg_trace(struct wil6210_priv *wil, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    trace_wil6210_log_dbg(&vaf);
    va_end(args);
    }
