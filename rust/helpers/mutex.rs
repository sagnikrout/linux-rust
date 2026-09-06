//! Automatically rewritten from C to Rust
//! Source: rust/helpers/mutex.c
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
pub unsafe extern "C" fn rust_helper_mutex_lock(lock: *mut mutex) -> __rust_helper void {
    __rust_helper void rust_helper_mutex_lock(struct mutex *lock)
    {
    mutex_lock(lock);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mutex_trylock(lock: *mut mutex) -> __rust_helper int {
    __rust_helper int rust_helper_mutex_trylock(struct mutex *lock)
    {
    return mutex_trylock(lock);
    }
    __rust_helper void rust_helper___mutex_init(struct mutex *mutex,
    const char *name,
    struct lock_class_key *key)
    {
    __mutex_init(mutex, name, key);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mutex_assert_is_held(mutex: *mut mutex) -> __rust_helper void {
    __rust_helper void rust_helper_mutex_assert_is_held(struct mutex *mutex)
    {
    lockdep_assert_held(mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mutex_destroy(lock: *mut mutex) -> __rust_helper void {
    __rust_helper void rust_helper_mutex_destroy(struct mutex *lock)
    {
    mutex_destroy(lock);
    }
