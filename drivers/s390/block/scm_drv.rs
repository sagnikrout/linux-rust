//! Automatically rewritten from C to Rust
//! Source: drivers/s390/block/scm_drv.c
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
// Device driver for s390 storage class memory.
//
// Copyright IBM Corp. 2012
// Author(s): Sebastian Ott <sebott@linux.vnet.ibm.com>
//

#[no_mangle]
unsafe extern "C" fn scm_notify(scmdev: *mut scm_device, event: enum scm_event) {
    static void scm_notify(struct scm_device *scmdev, enum scm_event event)
    {
    struct scm_blk_dev *bdev = dev_get_drvdata(&scmdev.dev);
    switch (event) {
    case SCM_CHANGE:
    pr_info("%lx: The capabilities of the SCM increment changed\n",
    (unsigned long) scmdev.address);
    SCM_LOG(2, "State changed");
    SCM_LOG_STATE(2, scmdev);
    break;
    case SCM_AVAIL:
    SCM_LOG(2, "Increment available");
    SCM_LOG_STATE(2, scmdev);
    scm_blk_set_available(bdev);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn scm_probe(scmdev: *mut scm_device) -> c_int {
    static int scm_probe(struct scm_device *scmdev)
    {
    struct scm_blk_dev *bdev;
    int ret;
    SCM_LOG(2, "probe");
    SCM_LOG_STATE(2, scmdev);
    if (scmdev.attrs.oper_state != OP_STATE_GOOD)
    return -EINVAL;
    bdev = kzalloc_obj(*bdev);
    if (!bdev)
    return -ENOMEM;
    dev_set_drvdata(&scmdev.dev, bdev);
    ret = scm_blk_dev_setup(bdev, scmdev);
    if (ret) {
    dev_set_drvdata(&scmdev.dev, core::ptr::null_mut());
    kfree(bdev);
    goto out;
    }
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scm_remove(scmdev: *mut scm_device) {
    static void scm_remove(struct scm_device *scmdev)
    {
    struct scm_blk_dev *bdev = dev_get_drvdata(&scmdev.dev);
    scm_blk_dev_cleanup(bdev);
    dev_set_drvdata(&scmdev.dev, core::ptr::null_mut());
    kfree(bdev);
    }
    static struct scm_driver scm_drv = {
    .drv = {
    .name = "scm_block",
    .owner = THIS_MODULE,
    },
    .notify = scm_notify,
    .probe = scm_probe,
    .remove = scm_remove,
    .handler = scm_blk_irq,
    };
#[no_mangle]
pub unsafe extern "C" fn scm_drv_init() -> int __init {
    int __init scm_drv_init(void)
    {
    return scm_driver_register(&scm_drv);
    }
#[no_mangle]
pub unsafe extern "C" fn scm_drv_cleanup() {
    void scm_drv_cleanup(void)
    {
    scm_driver_unregister(&scm_drv);
    }
