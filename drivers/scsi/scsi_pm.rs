//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_pm.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// scsi_pm.c	Copyright (C) 2010 Alan Stern
//
// SCSI dynamic Power Management
// Initial version: Alan Stern <stern@rowland.harvard.edu>
//

#[no_mangle]
unsafe extern "C" fn do_scsi_suspend(dev: *mut device, pm: *const dev_pm_ops) -> c_int {
    static int do_scsi_suspend(struct device *dev, const struct dev_pm_ops *pm)
    {
    return pm && pm.suspend ? pm.suspend(dev) : 0;
    }
#[no_mangle]
unsafe extern "C" fn do_scsi_freeze(dev: *mut device, pm: *const dev_pm_ops) -> c_int {
    static int do_scsi_freeze(struct device *dev, const struct dev_pm_ops *pm)
    {
    return pm && pm.freeze ? pm.freeze(dev) : 0;
    }
#[no_mangle]
unsafe extern "C" fn do_scsi_poweroff(dev: *mut device, pm: *const dev_pm_ops) -> c_int {
    static int do_scsi_poweroff(struct device *dev, const struct dev_pm_ops *pm)
    {
    return pm && pm.poweroff ? pm.poweroff(dev) : 0;
    }
#[no_mangle]
unsafe extern "C" fn do_scsi_resume(dev: *mut device, pm: *const dev_pm_ops) -> c_int {
    static int do_scsi_resume(struct device *dev, const struct dev_pm_ops *pm)
    {
    return pm && pm.resume ? pm.resume(dev) : 0;
    }
#[no_mangle]
unsafe extern "C" fn do_scsi_thaw(dev: *mut device, pm: *const dev_pm_ops) -> c_int {
    static int do_scsi_thaw(struct device *dev, const struct dev_pm_ops *pm)
    {
    return pm && pm.thaw ? pm.thaw(dev) : 0;
    }
#[no_mangle]
unsafe extern "C" fn do_scsi_restore(dev: *mut device, pm: *const dev_pm_ops) -> c_int {
    static int do_scsi_restore(struct device *dev, const struct dev_pm_ops *pm)
    {
    return pm && pm.restore ? pm.restore(dev) : 0;
    }
    static int scsi_dev_type_suspend(struct device *dev,
    int (*cb)(struct device *, const struct dev_pm_ops *))
    {
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    int err;
    err = scsi_device_quiesce(to_scsi_device(dev));
    if (err == 0) {
    err = cb(dev, pm);
    if (err)
    scsi_device_resume(to_scsi_device(dev));
    }
    dev_dbg(dev, "scsi suspend: %d\n", err);
    return err;
    }
    static int
    scsi_bus_suspend_common(struct device *dev,
    int (*cb)(struct device *, const struct dev_pm_ops *))
    {
    if (!scsi_is_sdev_device(dev))
    return 0;
    return scsi_dev_type_suspend(dev, cb);
    }
    static int scsi_bus_resume_common(struct device *dev,
    int (*cb)(struct device *, const struct dev_pm_ops *))
    {
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    int err;
    if (!scsi_is_sdev_device(dev))
    return 0;
    err = cb(dev, pm);
    scsi_device_resume(to_scsi_device(dev));
    dev_dbg(dev, "scsi resume: %d\n", err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn scsi_bus_prepare(dev: *mut device) -> c_int {
    static int scsi_bus_prepare(struct device *dev)
    {
    if (scsi_is_host_device(dev)) {
// Wait until async scanning is finished
    scsi_complete_async_scans();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scsi_bus_suspend(dev: *mut device) -> c_int {
    static int scsi_bus_suspend(struct device *dev)
    {
    return scsi_bus_suspend_common(dev, do_scsi_suspend);
    }
#[no_mangle]
unsafe extern "C" fn scsi_bus_resume(dev: *mut device) -> c_int {
    static int scsi_bus_resume(struct device *dev)
    {
    return scsi_bus_resume_common(dev, do_scsi_resume);
    }
#[no_mangle]
unsafe extern "C" fn scsi_bus_freeze(dev: *mut device) -> c_int {
    static int scsi_bus_freeze(struct device *dev)
    {
    return scsi_bus_suspend_common(dev, do_scsi_freeze);
    }
#[no_mangle]
unsafe extern "C" fn scsi_bus_thaw(dev: *mut device) -> c_int {
    static int scsi_bus_thaw(struct device *dev)
    {
    return scsi_bus_resume_common(dev, do_scsi_thaw);
    }
#[no_mangle]
unsafe extern "C" fn scsi_bus_poweroff(dev: *mut device) -> c_int {
    static int scsi_bus_poweroff(struct device *dev)
    {
    return scsi_bus_suspend_common(dev, do_scsi_poweroff);
    }
#[no_mangle]
unsafe extern "C" fn scsi_bus_restore(dev: *mut device) -> c_int {
    static int scsi_bus_restore(struct device *dev)
    {
    return scsi_bus_resume_common(dev, do_scsi_restore);
    }

#[no_mangle]
unsafe extern "C" fn sdev_runtime_suspend(dev: *mut device) -> c_int {
    static int sdev_runtime_suspend(struct device *dev)
    {
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    struct scsi_device *sdev = to_scsi_device(dev);
    let mut err: c_int = 0;
    err = blk_pre_runtime_suspend(sdev.request_queue);
    if (err)
    return err;
    if (pm && pm.runtime_suspend)
    err = pm.runtime_suspend(dev);
    blk_post_runtime_suspend(sdev.request_queue, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn scsi_runtime_suspend(dev: *mut device) -> c_int {
    static int scsi_runtime_suspend(struct device *dev)
    {
    let mut err: c_int = 0;
    dev_dbg(dev, "scsi_runtime_suspend\n");
    if (scsi_is_sdev_device(dev))
    err = sdev_runtime_suspend(dev);
// Insert hooks here for targets, hosts, and transport classes
    return err;
    }
#[no_mangle]
unsafe extern "C" fn sdev_runtime_resume(dev: *mut device) -> c_int {
    static int sdev_runtime_resume(struct device *dev)
    {
    struct scsi_device *sdev = to_scsi_device(dev);
    const struct dev_pm_ops *pm = dev.driver ? dev.driver.pm : core::ptr::null_mut();
    let mut err: c_int = 0;
    blk_pre_runtime_resume(sdev.request_queue);
    if (pm && pm.runtime_resume)
    err = pm.runtime_resume(dev);
    blk_post_runtime_resume(sdev.request_queue);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn scsi_runtime_resume(dev: *mut device) -> c_int {
    static int scsi_runtime_resume(struct device *dev)
    {
    let mut err: c_int = 0;
    dev_dbg(dev, "scsi_runtime_resume\n");
    if (scsi_is_sdev_device(dev))
    err = sdev_runtime_resume(dev);
// Insert hooks here for targets, hosts, and transport classes
    return err;
    }
#[no_mangle]
unsafe extern "C" fn scsi_runtime_idle(dev: *mut device) -> c_int {
    static int scsi_runtime_idle(struct device *dev)
    {
    dev_dbg(dev, "scsi_runtime_idle\n");
// Insert hooks here for targets, hosts, and transport classes
    if (scsi_is_sdev_device(dev)) {
    pm_runtime_autosuspend(dev);
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scsi_autopm_get_device(sdev: *mut scsi_device) -> c_int {
    int scsi_autopm_get_device(struct scsi_device *sdev)
    {
    int	err;
    err = pm_runtime_get_sync(&sdev.sdev_gendev);
    if (err < 0 && err !=-EACCES)
    pm_runtime_put_sync(&sdev.sdev_gendev);
    else
    err = 0;
    return err;
    }
    EXPORT_SYMBOL_GPL(scsi_autopm_get_device);
#[no_mangle]
pub unsafe extern "C" fn scsi_autopm_put_device(sdev: *mut scsi_device) {
    void scsi_autopm_put_device(struct scsi_device *sdev)
    {
    pm_runtime_put_sync(&sdev.sdev_gendev);
    }
    EXPORT_SYMBOL_GPL(scsi_autopm_put_device);
#[no_mangle]
pub unsafe extern "C" fn scsi_autopm_get_target(starget: *mut scsi_target) {
    void scsi_autopm_get_target(struct scsi_target *starget)
    {
    pm_runtime_get_sync(&starget.dev);
    }
#[no_mangle]
pub unsafe extern "C" fn scsi_autopm_put_target(starget: *mut scsi_target) {
    void scsi_autopm_put_target(struct scsi_target *starget)
    {
    pm_runtime_put_sync(&starget.dev);
    }
#[no_mangle]
pub unsafe extern "C" fn scsi_autopm_get_host(shost: *mut Scsi_Host) -> c_int {
    int scsi_autopm_get_host(struct Scsi_Host *shost)
    {
    int	err;
    err = pm_runtime_get_sync(&shost.shost_gendev);
    if (err < 0 && err !=-EACCES)
    pm_runtime_put_sync(&shost.shost_gendev);
    else
    err = 0;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn scsi_autopm_put_host(shost: *mut Scsi_Host) {
    void scsi_autopm_put_host(struct Scsi_Host *shost)
    {
    pm_runtime_put_sync(&shost.shost_gendev);
    }
    const struct dev_pm_ops scsi_bus_pm_ops = {
    .prepare =		scsi_bus_prepare,
    .suspend =		scsi_bus_suspend,
    .resume =		scsi_bus_resume,
    .freeze =		scsi_bus_freeze,
    .thaw =			scsi_bus_thaw,
    .poweroff =		scsi_bus_poweroff,
    .restore =		scsi_bus_restore,
    .runtime_suspend =	scsi_runtime_suspend,
    .runtime_resume =	scsi_runtime_resume,
    .runtime_idle =		scsi_runtime_idle,
    };
