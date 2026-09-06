//! Automatically rewritten from C to Rust
//! Source: kernel/power/console.c
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
// Functions for saving/restoring console.
//
// Originally from swsusp.
//

    static int orig_fgconsole, orig_kmsg;
    static bool vt_switch_done;
    static DEFINE_MUTEX(vt_switch_mutex);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_vt_switch {
    pub head: list_head,
    pub dev: *mut device,
    pub required: bool,
}

    static LIST_HEAD(pm_vt_switch_list);
//
// pm_vt_switch_required - indicate VT switch at suspend requirements
// @dev: device
// @required: if true, caller needs VT switch at suspend/resume time
//
// The different console drivers may or may not require VT switches across
// suspend/resume, depending on how they handle restoring video state and
// what may be running.
//
// Drivers can indicate support for switchless suspend/resume, which can
// save time and flicker, by using this routine and passing 'false' as
// the argument.  If any loaded driver needs VT switching, or the
// no_console_suspend argument has been passed on the command line, VT
// switches will occur.
//
#[no_mangle]
pub unsafe extern "C" fn pm_vt_switch_required(dev: *mut device, required: bool) -> c_int {
    int pm_vt_switch_required(struct device *dev, bool required)
    {
    struct pm_vt_switch *entry, *tmp;
    let mut ret: c_int = 0;
    mutex_lock(&vt_switch_mutex);
    list_for_each_entry(tmp, &pm_vt_switch_list, head) {
    if (tmp.dev == dev) {
// already registered, update requirement
    tmp.required = required;
    goto out;
    }
    }
    entry = kmalloc_obj(*entry);
    if (!entry) {
    ret = -ENOMEM;
    goto out;
    }
    entry.required = required;
    entry.dev = dev;
    list_add(&entry.head, &pm_vt_switch_list);
    out:
    mutex_unlock(&vt_switch_mutex);
    return ret;
    }
    EXPORT_SYMBOL(pm_vt_switch_required);
//
// pm_vt_switch_unregister - stop tracking a device's VT switching needs
// @dev: device
//
// Remove @dev from the vt switch list.
//
#[no_mangle]
pub unsafe extern "C" fn pm_vt_switch_unregister(dev: *mut device) {
    void pm_vt_switch_unregister(struct device *dev)
    {
    struct pm_vt_switch *tmp;
    mutex_lock(&vt_switch_mutex);
    list_for_each_entry(tmp, &pm_vt_switch_list, head) {
    if (tmp.dev == dev) {
    list_del(&tmp.head);
    kfree(tmp);
    break;
    }
    }
    mutex_unlock(&vt_switch_mutex);
    }
    EXPORT_SYMBOL(pm_vt_switch_unregister);
//
// There are three cases when a VT switch on suspend/resume are required:
// 1) no driver has indicated a requirement one way or another, so preserve
// the old behavior
// 2) console suspend is disabled, we want to see debug messages across
// suspend/resume
// 3) any registered driver indicates it needs a VT switch
//
// If none of these conditions is present, meaning we have at least one driver
// that doesn't need the switch, and none that do, we can avoid it to make
// resume look a little prettier (and suspend too, but that's usually hidden,
// e.g. when closing the lid on a laptop).
//
#[no_mangle]
unsafe extern "C" fn pm_vt_switch() -> bool {
    static bool pm_vt_switch(void)
    {
    struct pm_vt_switch *entry;
    let mut ret: bool = true;
    mutex_lock(&vt_switch_mutex);
    if (list_empty(&pm_vt_switch_list))
    goto out;
    if (!console_suspend_enabled)
    goto out;
    list_for_each_entry(entry, &pm_vt_switch_list, head) {
    if (entry.required)
    goto out;
    }
    ret = false;
    out:
    mutex_unlock(&vt_switch_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_prepare_console() {
    void pm_prepare_console(void)
    {
    if (!pm_vt_switch())
    return;
    orig_fgconsole = vt_move_to_console(SUSPEND_CONSOLE, 1);
    if (orig_fgconsole < 0)
    return;
    vt_switch_done = true;
    orig_kmsg = vt_kmsg_redirect(SUSPEND_CONSOLE);
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn pm_restore_console() {
    void pm_restore_console(void)
    {
    if (!pm_vt_switch() && !vt_switch_done)
    return;
    if (orig_fgconsole >= 0) {
    vt_move_to_console(orig_fgconsole, 0);
    vt_kmsg_redirect(orig_kmsg);
    }
    vt_switch_done = false;
    }
