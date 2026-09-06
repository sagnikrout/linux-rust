//! Automatically rewritten from C to Rust
//! Source: rust/helpers/xarray.c
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

#[no_mangle]
pub unsafe extern "C" fn rust_helper_xa_err(entry: *mut c_void) -> __rust_helper int {
    __rust_helper int rust_helper_xa_err(void *entry)
    {
    return xa_err(entry);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_xa_init_flags(xa: *mut xarray, flags: gfp_t) -> __rust_helper void {
    __rust_helper void rust_helper_xa_init_flags(struct xarray *xa, gfp_t flags)
    {
    return xa_init_flags(xa, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_xa_trylock(xa: *mut xarray) -> __rust_helper int {
    __rust_helper int rust_helper_xa_trylock(struct xarray *xa)
    {
    return xa_trylock(xa);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_xa_lock(xa: *mut xarray) -> __rust_helper void {
    __rust_helper void rust_helper_xa_lock(struct xarray *xa)
    {
    return xa_lock(xa);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_xa_unlock(xa: *mut xarray) -> __rust_helper void {
    __rust_helper void rust_helper_xa_unlock(struct xarray *xa)
    {
    return xa_unlock(xa);
    }
