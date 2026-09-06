//! Automatically rewritten from C to Rust
//! Source: tools/virtio/ringtest/noring.c
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
// Macro flag: #define _GNU_SOURCE

// stub implementation: useful for measuring overhead
#[no_mangle]
pub unsafe extern "C" fn alloc_ring() {
    void alloc_ring(void)
    {
    }
// guest side
#[no_mangle]
pub unsafe extern "C" fn add_inbuf(len: unsigned, buf: *mut c_void, datap: *mut c_void) -> c_int {
    int add_inbuf(unsigned len, void *buf, void *datap)
    {
    return 0;
    }
//
// skb_array API provides no way for producer to find out whether a given
// buffer was consumed.  Our tests merely require that a successful get_buf
// implies that add_inbuf succeed in the past, and that add_inbuf will succeed,
// fake it accordingly.
//
    void *get_buf(unsigned *lenp, void **bufp)
    {
    return "Buffer";
    }
#[no_mangle]
pub unsafe extern "C" fn used_empty() -> bool {
    bool used_empty()
    {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn disable_call() {
    void disable_call()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_call() -> bool {
    bool enable_call()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn kick_available() {
    void kick_available(void)
    {
    assert(0);
    }
// host side
#[no_mangle]
pub unsafe extern "C" fn disable_kick() {
    void disable_kick()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_kick() -> bool {
    bool enable_kick()
    {
    assert(0);
    }
#[no_mangle]
pub unsafe extern "C" fn avail_empty() -> bool {
    bool avail_empty()
    {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn use_buf(lenp: *mut unsigned, bufp: *mut c_void) -> bool {
    bool use_buf(unsigned *lenp, void **bufp)
    {
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn call_used() {
    void call_used(void)
    {
    assert(0);
    }
