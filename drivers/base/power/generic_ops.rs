//! Automatically rewritten from C to Rust
//! Source: drivers/base/power/generic_ops.c
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
// drivers/base/power/generic_ops.c - Generic PM callbacks for subsystems
//
// Copyright (c) 2010 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//

    ({ \
    struct device *_dev = (dev); \
    const struct dev_pm_ops *pm = _dev.driver ? _dev.driver.pm : core::ptr::null_mut(); \
    pm && pm.op ? pm.op(_dev) : 0; \
    })

//
// pm_generic_runtime_suspend - Generic runtime suspend callback for subsystems.
// @dev: Device to suspend.
//
// If PM operations are defined for the @dev's driver and they include
// ->runtime_suspend(), execute it and return its error code.  Otherwise,
// return 0.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_runtime_suspend(dev: *mut device) -> c_int {
    int pm_generic_runtime_suspend(struct device *dev)
    {
    return CALL_PM_OP(dev, runtime_suspend);
    }
    EXPORT_SYMBOL_GPL(pm_generic_runtime_suspend);
//
// pm_generic_runtime_resume - Generic runtime resume callback for subsystems.
// @dev: Device to resume.
//
// If PM operations are defined for the @dev's driver and they include
// ->runtime_resume(), execute it and return its error code.  Otherwise,
// return 0.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_runtime_resume(dev: *mut device) -> c_int {
    int pm_generic_runtime_resume(struct device *dev)
    {
    return CALL_PM_OP(dev, runtime_resume);
    }
    EXPORT_SYMBOL_GPL(pm_generic_runtime_resume);

//
// pm_generic_prepare - Generic routine preparing a device for power transition.
// @dev: Device to prepare.
//
// Prepare a device for a system-wide power transition.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_prepare(dev: *mut device) -> c_int {
    int pm_generic_prepare(struct device *dev)
    {
    struct device_driver *drv = dev.driver;
    let mut ret: c_int = 0;
    if (drv && drv.pm && drv.pm.prepare)
    ret = drv.pm.prepare(dev);
    return ret;
    }
//
// pm_generic_suspend_noirq - Generic suspend_noirq callback for subsystems.
// @dev: Device to suspend.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_suspend_noirq(dev: *mut device) -> c_int {
    int pm_generic_suspend_noirq(struct device *dev)
    {
    return CALL_PM_OP(dev, suspend_noirq);
    }
    EXPORT_SYMBOL_GPL(pm_generic_suspend_noirq);
//
// pm_generic_suspend_late - Generic suspend_late callback for subsystems.
// @dev: Device to suspend.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_suspend_late(dev: *mut device) -> c_int {
    int pm_generic_suspend_late(struct device *dev)
    {
    return CALL_PM_OP(dev, suspend_late);
    }
    EXPORT_SYMBOL_GPL(pm_generic_suspend_late);
//
// pm_generic_suspend - Generic suspend callback for subsystems.
// @dev: Device to suspend.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_suspend(dev: *mut device) -> c_int {
    int pm_generic_suspend(struct device *dev)
    {
    return CALL_PM_OP(dev, suspend);
    }
    EXPORT_SYMBOL_GPL(pm_generic_suspend);
//
// pm_generic_freeze_noirq - Generic freeze_noirq callback for subsystems.
// @dev: Device to freeze.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_freeze_noirq(dev: *mut device) -> c_int {
    int pm_generic_freeze_noirq(struct device *dev)
    {
    return CALL_PM_OP(dev, freeze_noirq);
    }
    EXPORT_SYMBOL_GPL(pm_generic_freeze_noirq);
//
// pm_generic_freeze - Generic freeze callback for subsystems.
// @dev: Device to freeze.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_freeze(dev: *mut device) -> c_int {
    int pm_generic_freeze(struct device *dev)
    {
    return CALL_PM_OP(dev, freeze);
    }
    EXPORT_SYMBOL_GPL(pm_generic_freeze);
//
// pm_generic_poweroff_noirq - Generic poweroff_noirq callback for subsystems.
// @dev: Device to handle.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_poweroff_noirq(dev: *mut device) -> c_int {
    int pm_generic_poweroff_noirq(struct device *dev)
    {
    return CALL_PM_OP(dev, poweroff_noirq);
    }
    EXPORT_SYMBOL_GPL(pm_generic_poweroff_noirq);
//
// pm_generic_poweroff_late - Generic poweroff_late callback for subsystems.
// @dev: Device to handle.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_poweroff_late(dev: *mut device) -> c_int {
    int pm_generic_poweroff_late(struct device *dev)
    {
    return CALL_PM_OP(dev, poweroff_late);
    }
    EXPORT_SYMBOL_GPL(pm_generic_poweroff_late);
//
// pm_generic_poweroff - Generic poweroff callback for subsystems.
// @dev: Device to handle.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_poweroff(dev: *mut device) -> c_int {
    int pm_generic_poweroff(struct device *dev)
    {
    return CALL_PM_OP(dev, poweroff);
    }
    EXPORT_SYMBOL_GPL(pm_generic_poweroff);
//
// pm_generic_thaw_noirq - Generic thaw_noirq callback for subsystems.
// @dev: Device to thaw.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_thaw_noirq(dev: *mut device) -> c_int {
    int pm_generic_thaw_noirq(struct device *dev)
    {
    return CALL_PM_OP(dev, thaw_noirq);
    }
    EXPORT_SYMBOL_GPL(pm_generic_thaw_noirq);
//
// pm_generic_thaw - Generic thaw callback for subsystems.
// @dev: Device to thaw.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_thaw(dev: *mut device) -> c_int {
    int pm_generic_thaw(struct device *dev)
    {
    return CALL_PM_OP(dev, thaw);
    }
    EXPORT_SYMBOL_GPL(pm_generic_thaw);
//
// pm_generic_resume_noirq - Generic resume_noirq callback for subsystems.
// @dev: Device to resume.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_resume_noirq(dev: *mut device) -> c_int {
    int pm_generic_resume_noirq(struct device *dev)
    {
    return CALL_PM_OP(dev, resume_noirq);
    }
    EXPORT_SYMBOL_GPL(pm_generic_resume_noirq);
//
// pm_generic_resume_early - Generic resume_early callback for subsystems.
// @dev: Device to resume.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_resume_early(dev: *mut device) -> c_int {
    int pm_generic_resume_early(struct device *dev)
    {
    return CALL_PM_OP(dev, resume_early);
    }
    EXPORT_SYMBOL_GPL(pm_generic_resume_early);
//
// pm_generic_resume - Generic resume callback for subsystems.
// @dev: Device to resume.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_resume(dev: *mut device) -> c_int {
    int pm_generic_resume(struct device *dev)
    {
    return CALL_PM_OP(dev, resume);
    }
    EXPORT_SYMBOL_GPL(pm_generic_resume);
//
// pm_generic_restore_noirq - Generic restore_noirq callback for subsystems.
// @dev: Device to restore.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_restore_noirq(dev: *mut device) -> c_int {
    int pm_generic_restore_noirq(struct device *dev)
    {
    return CALL_PM_OP(dev, restore_noirq);
    }
    EXPORT_SYMBOL_GPL(pm_generic_restore_noirq);
//
// pm_generic_restore_early - Generic restore_early callback for subsystems.
// @dev: Device to resume.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_restore_early(dev: *mut device) -> c_int {
    int pm_generic_restore_early(struct device *dev)
    {
    return CALL_PM_OP(dev, restore_early);
    }
    EXPORT_SYMBOL_GPL(pm_generic_restore_early);
//
// pm_generic_restore - Generic restore callback for subsystems.
// @dev: Device to restore.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_restore(dev: *mut device) -> c_int {
    int pm_generic_restore(struct device *dev)
    {
    return CALL_PM_OP(dev, restore);
    }
    EXPORT_SYMBOL_GPL(pm_generic_restore);
//
// pm_generic_complete - Generic routine completing a device power transition.
// @dev: Device to handle.
//
// Complete a device power transition during a system-wide power transition.
//
#[no_mangle]
pub unsafe extern "C" fn pm_generic_complete(dev: *mut device) {
    void pm_generic_complete(struct device *dev)
    {
    struct device_driver *drv = dev.driver;
    if (drv && drv.pm && drv.pm.complete)
    drv.pm.complete(dev);
    }
