//! Automatically rewritten from C to Rust
//! Source: drivers/greybus/svc_watchdog.c
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
// SVC Greybus "watchdog" driver.
//
// Copyright 2016 Google Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_watchdog {
    pub work: delayed_work,
    pub svc: *mut gb_svc,
    pub enabled: bool,
    pub pm_notifier: notifier_block,
}

    static struct delayed_work reset_work;
    static int svc_watchdog_pm_notifier(struct notifier_block *notifier,
    unsigned long pm_event, void *unused)
    {
    struct gb_svc_watchdog *watchdog =
    container_of(notifier, struct gb_svc_watchdog, pm_notifier);
    switch (pm_event) {
    case PM_SUSPEND_PREPARE:
    gb_svc_watchdog_disable(watchdog.svc);
    break;
    case PM_POST_SUSPEND:
    gb_svc_watchdog_enable(watchdog.svc);
    break;
    default:
    break;
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn greybus_reset(work: *mut work_struct) {
    static void greybus_reset(struct work_struct *work)
    {
    static char const start_path[] = "/system/bin/start";
    static char *envp[] = {
    "HOME=/",
    "PATH=/sbin:/vendor/bin:/system/sbin:/system/bin:/system/xbin",
    core::ptr::null_mut(),
    };
    static char *argv[] = {
    (char *)start_path,
    "unipro_reset",
    core::ptr::null_mut(),
    };
    pr_err("svc_watchdog: calling \"%s %s\" to reset greybus network!\n",
    argv[0], argv[1]);
    call_usermodehelper(start_path, argv, envp, UMH_WAIT_EXEC);
    }
#[no_mangle]
unsafe extern "C" fn do_work(work: *mut work_struct) {
    static void do_work(struct work_struct *work)
    {
    struct gb_svc_watchdog *watchdog;
    struct gb_svc *svc;
    int retval;
    watchdog = container_of(work, struct gb_svc_watchdog, work.work);
    svc = watchdog.svc;
    dev_dbg(&svc.dev, "%s: ping.\n", __func__);
    retval = gb_svc_ping(svc);
    if (retval) {
//
// Something went really wrong, let's warn userspace and then
// pull the plug and reset the whole greybus network.
// We need to do this outside of this workqueue as we will be
// tearing down the svc device itself.  So queue up
// yet-another-callback to do that.
//
    dev_err(&svc.dev,
    "SVC ping has returned %d, something is wrong!!!\n",
    retval);
    if (svc.action == GB_SVC_WATCHDOG_BITE_PANIC_KERNEL) {
    panic("SVC is not responding\n");
    } else if (svc.action == GB_SVC_WATCHDOG_BITE_RESET_UNIPRO) {
    dev_err(&svc.dev, "Resetting the greybus network, watch out!!!\n");
    INIT_DELAYED_WORK(&reset_work, greybus_reset);
    schedule_delayed_work(&reset_work, HZ / 2);
//
// Disable ourselves, we don't want to trip again unless
// userspace wants us to.
//
    watchdog.enabled = false;
    }
    }
// resubmit our work to happen again, if we are still "alive"
    if (watchdog.enabled)
    schedule_delayed_work(&watchdog.work, SVC_WATCHDOG_PERIOD);
    }
#[no_mangle]
pub unsafe extern "C" fn gb_svc_watchdog_create(svc: *mut gb_svc) -> c_int {
    int gb_svc_watchdog_create(struct gb_svc *svc)
    {
    struct gb_svc_watchdog *watchdog;
    int retval;
    if (svc.watchdog)
    return 0;
    watchdog = kmalloc_obj(*watchdog);
    if (!watchdog)
    return -ENOMEM;
    watchdog.enabled = false;
    watchdog.svc = svc;
    INIT_DELAYED_WORK(&watchdog.work, do_work);
    svc.watchdog = watchdog;
    watchdog.pm_notifier.notifier_call = svc_watchdog_pm_notifier;
    retval = register_pm_notifier(&watchdog.pm_notifier);
    if (retval) {
    dev_err(&svc.dev, "error registering pm notifier(%d)\n",
    retval);
    goto svc_watchdog_create_err;
    }
    retval = gb_svc_watchdog_enable(svc);
    if (retval) {
    dev_err(&svc.dev, "error enabling watchdog (%d)\n", retval);
    unregister_pm_notifier(&watchdog.pm_notifier);
    goto svc_watchdog_create_err;
    }
    return retval;
    svc_watchdog_create_err:
    svc.watchdog = core::ptr::null_mut();
    kfree(watchdog);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_svc_watchdog_destroy(svc: *mut gb_svc) {
    void gb_svc_watchdog_destroy(struct gb_svc *svc)
    {
    struct gb_svc_watchdog *watchdog = svc.watchdog;
    if (!watchdog)
    return;
    unregister_pm_notifier(&watchdog.pm_notifier);
    gb_svc_watchdog_disable(svc);
    svc.watchdog = core::ptr::null_mut();
    kfree(watchdog);
    }
#[no_mangle]
pub unsafe extern "C" fn gb_svc_watchdog_enabled(svc: *mut gb_svc) -> bool {
    bool gb_svc_watchdog_enabled(struct gb_svc *svc)
    {
    if (!svc || !svc.watchdog)
    return false;
    return svc.watchdog.enabled;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_svc_watchdog_enable(svc: *mut gb_svc) -> c_int {
    int gb_svc_watchdog_enable(struct gb_svc *svc)
    {
    struct gb_svc_watchdog *watchdog;
    if (!svc.watchdog)
    return -ENODEV;
    watchdog = svc.watchdog;
    if (watchdog.enabled)
    return 0;
    watchdog.enabled = true;
    schedule_delayed_work(&watchdog.work, SVC_WATCHDOG_PERIOD);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn gb_svc_watchdog_disable(svc: *mut gb_svc) -> c_int {
    int gb_svc_watchdog_disable(struct gb_svc *svc)
    {
    struct gb_svc_watchdog *watchdog;
    if (!svc.watchdog)
    return -ENODEV;
    watchdog = svc.watchdog;
    if (!watchdog.enabled)
    return 0;
    watchdog.enabled = false;
    cancel_delayed_work_sync(&watchdog.work);
    return 0;
    }
