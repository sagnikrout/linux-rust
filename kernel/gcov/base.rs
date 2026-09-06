//! Automatically rewritten from C to Rust
//! Source: kernel/gcov/base.c
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
// This code maintains a list of active profiling data structures.
//
// Copyright IBM Corp. 2009
// Author(s): Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
//
// Uses gcc-internal data definitions.
// Based on the gcov-kernel patch by:
// Hubertus Franke <frankeh@us.ibm.com>
// Nigel Hinds <nhinds@us.ibm.com>
// Rajan Ravindran <rajancr@us.ibm.com>
// Peter Oberparleiter <oberpar@linux.vnet.ibm.com>
// Paul Larson
//

    int gcov_events_enabled;
    DEFINE_MUTEX(gcov_lock);
//
// gcov_enable_events - enable event reporting through gcov_event()
//
// Turn on reporting of profiling data load/unload-events through the
// gcov_event() callback. Also replay all previous events once. This function
// is needed because some events are potentially generated too early for the
// callback implementation to handle them initially.
//
#[no_mangle]
pub unsafe extern "C" fn gcov_enable_events() {
    void gcov_enable_events(void)
    {
    struct gcov_info *info = core::ptr::null_mut();
    mutex_lock(&gcov_lock);
    gcov_events_enabled = 1;
// Perform event callback for previously registered entries.
    while ((info = gcov_info_next(info))) {
    gcov_event(GCOV_ADD, info);
    cond_resched();
    }
    mutex_unlock(&gcov_lock);
    }
//
// store_gcov_u32 - store 32 bit number in gcov format to buffer
// @buffer: target buffer or NULL
// @off: offset into the buffer
// @v: value to be stored
//
// Number format defined by gcc: numbers are recorded in the 32 bit
// unsigned binary form of the endianness of the machine generating the
// file. Returns the number of bytes stored. If @buffer is %NULL, doesn't
// store anything.
//
#[no_mangle]
pub unsafe extern "C" fn store_gcov_u32(buffer: *mut c_void, off: usize, v: u32) -> usize {
    size_t store_gcov_u32(void *buffer, size_t off, u32 v)
    {
    u32 *data;
    if (buffer) {
    data = buffer + off;
// data = v;
    }
    return sizeof(*data);
    }
//
// store_gcov_u64 - store 64 bit number in gcov format to buffer
// @buffer: target buffer or NULL
// @off: offset into the buffer
// @v: value to be stored
//
// Number format defined by gcc: numbers are recorded in the 32 bit
// unsigned binary form of the endianness of the machine generating the
// file. 64 bit numbers are stored as two 32 bit numbers, the low part
// first. Returns the number of bytes stored. If @buffer is %NULL, doesn't store
// anything.
//
#[no_mangle]
pub unsafe extern "C" fn store_gcov_u64(buffer: *mut c_void, off: usize, v: u64) -> usize {
    size_t store_gcov_u64(void *buffer, size_t off, u64 v)
    {
    u32 *data;
    if (buffer) {
    data = buffer + off;
    data[0] = (v & 0xffffffffUL);
    data[1] = (v >> 32);
    }
    return sizeof(*data) * 2;
    }

// Update list and generate events when modules are unloaded.
    static int gcov_module_notifier(struct notifier_block *nb, unsigned long event,
    void *data)
    {
    struct module *mod = data;
    struct gcov_info *info = core::ptr::null_mut();
    struct gcov_info *prev = core::ptr::null_mut();
    if (event != MODULE_STATE_GOING)
    return NOTIFY_OK;
    mutex_lock(&gcov_lock);
// Remove entries located in module from linked list.
    while ((info = gcov_info_next(info))) {
    if (gcov_info_within_module(info, mod)) {
    gcov_info_unlink(prev, info);
    if (gcov_events_enabled)
    gcov_event(GCOV_REMOVE, info);
    } else
    prev = info;
    }
    mutex_unlock(&gcov_lock);
    return NOTIFY_OK;
    }
    static struct notifier_block gcov_nb = {
    .notifier_call	= gcov_module_notifier,
    };
#[no_mangle]
unsafe extern "C" fn gcov_init() -> int __init {
    static int __init gcov_init(void)
    {
    return register_module_notifier(&gcov_nb);
    }
    device_initcall(gcov_init);
