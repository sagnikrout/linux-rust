//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/remoteproc_cdev.c
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
// Character device interface driver for Remoteproc framework.
//
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
//

pub const NUM_RPROC_DEVICES: c_int = 64;
    static dev_t rproc_major;
#[no_mangle]
unsafe extern "C" fn rproc_cdev_write(filp: *mut file, buf: *const char __user, len: usize, pos: *mut loff_t) -> isize {
    static ssize_t rproc_cdev_write(struct file *filp, const char __user *buf, size_t len, loff_t *pos)
    {
    struct rproc *rproc = container_of(filp.f_inode.i_cdev, struct rproc, cdev);
    let mut ret: c_int = 0;
    char cmd[10];
    if (!len || len > sizeof(cmd))
    return -EINVAL;
    ret = copy_from_user(cmd, buf, len);
    if (ret)
    return -EFAULT;
    if (!strncmp(cmd, "start", len)) {
    ret = rproc_boot(rproc);
    } else if (!strncmp(cmd, "stop", len)) {
    ret = rproc_shutdown(rproc);
    } else if (!strncmp(cmd, "detach", len)) {
    ret = rproc_detach(rproc);
    } else {
    dev_err(&rproc.dev, "Unrecognized option\n");
    ret = -EINVAL;
    }
    return ret ? ret : len;
    }
#[no_mangle]
unsafe extern "C" fn rproc_device_ioctl(filp: *mut file, ioctl: c_uint, arg: c_ulong) -> c_long {
    static long rproc_device_ioctl(struct file *filp, unsigned int ioctl, unsigned long arg)
    {
    struct rproc *rproc = container_of(filp.f_inode.i_cdev, struct rproc, cdev);
    void __user *argp = (void __user *)arg;
    s32 param;
    switch (ioctl) {
    case RPROC_SET_SHUTDOWN_ON_RELEASE:
    if (copy_from_user(&param, argp, sizeof(s32)))
    return -EFAULT;
    rproc.cdev_put_on_release = !!param;
    break;
    case RPROC_GET_SHUTDOWN_ON_RELEASE:
    param = (s32)rproc.cdev_put_on_release;
    if (copy_to_user(argp, &param, sizeof(s32)))
    return -EFAULT;
    break;
    default:
    dev_err(&rproc.dev, "Unsupported ioctl\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rproc_cdev_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int rproc_cdev_release(struct inode *inode, struct file *filp)
    {
    struct rproc *rproc = container_of(inode.i_cdev, struct rproc, cdev);
    let mut ret: c_int = 0;
    if (!rproc.cdev_put_on_release)
    return 0;
    if (rproc.state == RPROC_RUNNING)
    rproc_shutdown(rproc);
#[no_mangle]
pub unsafe extern "C" fn if(RPROC_ATTACHED: rproc->state ==) -> else {
    else if (rproc.state == RPROC_ATTACHED)
    ret = rproc_detach(rproc);
    return ret;
    }
    static const struct file_operations rproc_fops = {
    .write = rproc_cdev_write,
    .unlocked_ioctl = rproc_device_ioctl,
    .compat_ioctl = compat_ptr_ioctl,
    .release = rproc_cdev_release,
    };
#[no_mangle]
pub unsafe extern "C" fn rproc_char_device_add(rproc: *mut rproc) -> c_int {
    int rproc_char_device_add(struct rproc *rproc)
    {
    int ret;
    cdev_init(&rproc.cdev, &rproc_fops);
    rproc.cdev.owner = THIS_MODULE;
    rproc.dev.devt = MKDEV(MAJOR(rproc_major), rproc.index);
    cdev_set_parent(&rproc.cdev, &rproc.dev.kobj);
    ret = cdev_add(&rproc.cdev, rproc.dev.devt, 1);
    if (ret < 0)
    dev_err(&rproc.dev, "Failed to add char dev for %s\n", rproc.name);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rproc_char_device_remove(rproc: *mut rproc) {
    void rproc_char_device_remove(struct rproc *rproc)
    {
    cdev_del(&rproc.cdev);
    }
#[no_mangle]
pub unsafe extern "C" fn rproc_init_cdev() -> void __init {
    void __init rproc_init_cdev(void)
    {
    int ret;
    ret = alloc_chrdev_region(&rproc_major, 0, NUM_RPROC_DEVICES, "remoteproc");
    if (ret < 0)
    pr_err("Failed to alloc rproc_cdev region, err %d\n", ret);
    }
