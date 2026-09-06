//! Automatically rewritten from C to Rust
//! Source: drivers/tty/tty_mutex.c
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

// Legacy tty mutex glue
//
// Getting the big tty mutex.
//
#[no_mangle]
pub unsafe extern "C" fn tty_lock(tty: *mut tty_struct) {
    void tty_lock(struct tty_struct *tty)
    {
    tty_kref_get(tty);
    mutex_lock(&tty.legacy_mutex);
    }
    EXPORT_SYMBOL(tty_lock);
#[no_mangle]
pub unsafe extern "C" fn tty_lock_interruptible(tty: *mut tty_struct) -> c_int {
    int tty_lock_interruptible(struct tty_struct *tty)
    {
    int ret;
    tty_kref_get(tty);
    ret = mutex_lock_interruptible(&tty.legacy_mutex);
    if (ret)
    tty_kref_put(tty);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn tty_unlock(tty: *mut tty_struct) {
    void tty_unlock(struct tty_struct *tty)
    {
    mutex_unlock(&tty.legacy_mutex);
    tty_kref_put(tty);
    }
    EXPORT_SYMBOL(tty_unlock);
#[no_mangle]
pub unsafe extern "C" fn tty_lock_slave(tty: *mut tty_struct) {
    void tty_lock_slave(struct tty_struct *tty)
    {
    if (tty && tty != tty.link)
    tty_lock(tty);
    }
#[no_mangle]
pub unsafe extern "C" fn tty_unlock_slave(tty: *mut tty_struct) {
    void tty_unlock_slave(struct tty_struct *tty)
    {
    if (tty && tty != tty.link)
    tty_unlock(tty);
    }
#[no_mangle]
pub unsafe extern "C" fn tty_set_lock_subclass(tty: *mut tty_struct) {
    void tty_set_lock_subclass(struct tty_struct *tty)
    {
    lockdep_set_subclass(&tty.legacy_mutex, TTY_LOCK_SLAVE);
    }
