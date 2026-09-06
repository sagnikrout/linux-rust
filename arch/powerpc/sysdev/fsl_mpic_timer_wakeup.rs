//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/fsl_mpic_timer_wakeup.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// MPIC timer wakeup driver
//
// Copyright 2013 Freescale Semiconductor, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mpic_timer_wakeup {
    pub timer: *mut mpic_timer,
    pub free_work: work_struct,
}

    static struct fsl_mpic_timer_wakeup *fsl_wakeup;
    static DEFINE_MUTEX(sysfs_lock);
#[no_mangle]
unsafe extern "C" fn fsl_free_resource(ws: *mut work_struct) {
    static void fsl_free_resource(struct work_struct *ws)
    {
    struct fsl_mpic_timer_wakeup *wakeup =
    container_of(ws, struct fsl_mpic_timer_wakeup, free_work);
    mutex_lock(&sysfs_lock);
    if (wakeup.timer) {
    disable_irq_wake(wakeup.timer.irq);
    mpic_free_timer(wakeup.timer);
    }
    wakeup.timer = core::ptr::null_mut();
    mutex_unlock(&sysfs_lock);
    }
#[no_mangle]
unsafe extern "C" fn fsl_mpic_timer_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t fsl_mpic_timer_irq(int irq, void *dev_id)
    {
    struct fsl_mpic_timer_wakeup *wakeup = dev_id;
    schedule_work(&wakeup.free_work);
    return wakeup.timer ? IRQ_HANDLED : IRQ_NONE;
    }
    static ssize_t fsl_timer_wakeup_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut interval: time64_t = 0;
    mutex_lock(&sysfs_lock);
    if (fsl_wakeup.timer) {
    mpic_get_remain_time(fsl_wakeup.timer, &interval);
    interval++;
    }
    mutex_unlock(&sysfs_lock);
    return sysfs_emit(buf, "%lld\n", interval);
    }
    static ssize_t fsl_timer_wakeup_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    time64_t interval;
    int ret;
    if (kstrtoll(buf, 0, &interval))
    return -EINVAL;
    guard(mutex)(&sysfs_lock);
    if (fsl_wakeup.timer) {
    disable_irq_wake(fsl_wakeup.timer.irq);
    mpic_free_timer(fsl_wakeup.timer);
    fsl_wakeup.timer = core::ptr::null_mut();
    }
    if (!interval)
    return count;
    fsl_wakeup.timer = mpic_request_timer(fsl_mpic_timer_irq,
    fsl_wakeup, interval);
    if (!fsl_wakeup.timer)
    return -EINVAL;
    ret = enable_irq_wake(fsl_wakeup.timer.irq);
    if (ret) {
    mpic_free_timer(fsl_wakeup.timer);
    fsl_wakeup.timer = core::ptr::null_mut();
    return ret;
    }
    mpic_start_timer(fsl_wakeup.timer);
    return count;
    }
    static struct device_attribute mpic_attributes = __ATTR(timer_wakeup, 0644,
    fsl_timer_wakeup_show, fsl_timer_wakeup_store);
#[no_mangle]
unsafe extern "C" fn fsl_wakeup_sys_init() -> int __init {
    static int __init fsl_wakeup_sys_init(void)
    {
    struct device *dev_root;
    let mut ret: c_int = -EINVAL;
    fsl_wakeup = kzalloc_obj(struct fsl_mpic_timer_wakeup);
    if (!fsl_wakeup)
    return -ENOMEM;
    INIT_WORK(&fsl_wakeup.free_work, fsl_free_resource);
    dev_root = bus_get_dev_root(&mpic_subsys);
    if (dev_root) {
    ret = device_create_file(dev_root, &mpic_attributes);
    put_device(dev_root);
    if (ret)
    kfree(fsl_wakeup);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fsl_wakeup_sys_exit() -> void __exit {
    static void __exit fsl_wakeup_sys_exit(void)
    {
    struct device *dev_root;
    dev_root = bus_get_dev_root(&mpic_subsys);
    if (dev_root) {
    device_remove_file(dev_root, &mpic_attributes);
    put_device(dev_root);
    }
    mutex_lock(&sysfs_lock);
    if (fsl_wakeup.timer) {
    disable_irq_wake(fsl_wakeup.timer.irq);
    mpic_free_timer(fsl_wakeup.timer);
    }
    kfree(fsl_wakeup);
    mutex_unlock(&sysfs_lock);
    }
    module_init(fsl_wakeup_sys_init);
    module_exit(fsl_wakeup_sys_exit);
    MODULE_DESCRIPTION("Freescale MPIC global timer wakeup driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Wang Dongsheng <dongsheng.wang@freescale.com>");
