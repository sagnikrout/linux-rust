//! Automatically rewritten from C to Rust
//! Source: rust/helpers/clk.c
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

//
// The "inline" implementation of below helpers are only available when
// CONFIG_HAVE_CLK or CONFIG_HAVE_CLK_PREPARE aren't set.
//

    __rust_helper struct clk *rust_helper_clk_get(struct device *dev,
    const char *id)
    {
    return clk_get(dev, id);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_put(clk: *mut clk) -> __rust_helper void {
    __rust_helper void rust_helper_clk_put(struct clk *clk)
    {
    clk_put(clk);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_enable(clk: *mut clk) -> __rust_helper int {
    __rust_helper int rust_helper_clk_enable(struct clk *clk)
    {
    return clk_enable(clk);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_disable(clk: *mut clk) -> __rust_helper void {
    __rust_helper void rust_helper_clk_disable(struct clk *clk)
    {
    clk_disable(clk);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_get_rate(clk: *mut clk) -> __rust_helper unsigned long {
    __rust_helper unsigned long rust_helper_clk_get_rate(struct clk *clk)
    {
    return clk_get_rate(clk);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_set_rate(clk: *mut clk, rate: c_ulong) -> __rust_helper int {
    __rust_helper int rust_helper_clk_set_rate(struct clk *clk, unsigned long rate)
    {
    return clk_set_rate(clk, rate);
    }

#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_prepare(clk: *mut clk) -> __rust_helper int {
    __rust_helper int rust_helper_clk_prepare(struct clk *clk)
    {
    return clk_prepare(clk);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_unprepare(clk: *mut clk) -> __rust_helper void {
    __rust_helper void rust_helper_clk_unprepare(struct clk *clk)
    {
    clk_unprepare(clk);
    }

    __rust_helper struct clk *rust_helper_clk_get_optional(struct device *dev,
    const char *id)
    {
    return clk_get_optional(dev, id);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_prepare_enable(clk: *mut clk) -> __rust_helper int {
    __rust_helper int rust_helper_clk_prepare_enable(struct clk *clk)
    {
    return clk_prepare_enable(clk);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clk_disable_unprepare(clk: *mut clk) -> __rust_helper void {
    __rust_helper void rust_helper_clk_disable_unprepare(struct clk *clk)
    {
    clk_disable_unprepare(clk);
    }
