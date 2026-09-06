//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/isc.c
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
// Functions for registration of I/O interruption subclasses on s390.
//
// Copyright IBM Corp. 2008
// Authors: Sebastian Ott <sebott@linux.vnet.ibm.com>
//

    static unsigned int isc_refs[MAX_ISC + 1];
    static DEFINE_SPINLOCK(isc_ref_lock);
//
// isc_register - register an I/O interruption subclass.
// @isc: I/O interruption subclass to register
//
// The number of users for @isc is increased. If this is the first user to
// register @isc, the corresponding I/O interruption subclass mask is enabled.
//
// Context:
// This function must not be called in interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn isc_register(isc: c_uint) {
    void isc_register(unsigned int isc)
    {
    if (isc > MAX_ISC) {
    WARN_ON(1);
    return;
    }
    spin_lock(&isc_ref_lock);
    if (isc_refs[isc] == 0)
    system_ctl_set_bit(6, 31 - isc);
    isc_refs[isc]++;
    spin_unlock(&isc_ref_lock);
    }
    EXPORT_SYMBOL_GPL(isc_register);
//
// isc_unregister - unregister an I/O interruption subclass.
// @isc: I/O interruption subclass to unregister
//
// The number of users for @isc is decreased. If this is the last user to
// unregister @isc, the corresponding I/O interruption subclass mask is
// disabled.
// Note: This function must not be called if isc_register() hasn't been called
// before by the driver for @isc.
//
// Context:
// This function must not be called in interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn isc_unregister(isc: c_uint) {
    void isc_unregister(unsigned int isc)
    {
    spin_lock(&isc_ref_lock);
// check for misuse
    if (isc > MAX_ISC || isc_refs[isc] == 0) {
    WARN_ON(1);
    goto out_unlock;
    }
    if (isc_refs[isc] == 1)
    system_ctl_clear_bit(6, 31 - isc);
    isc_refs[isc]--;
    out_unlock:
    spin_unlock(&isc_ref_lock);
    }
    EXPORT_SYMBOL_GPL(isc_unregister);
