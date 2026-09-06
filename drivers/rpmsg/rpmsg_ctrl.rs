//! Automatically rewritten from C to Rust
//! Source: drivers/rpmsg/rpmsg_ctrl.c
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
// Copyright (C) 2022, STMicroelectronics
// Copyright (c) 2016, Linaro Ltd.
// Copyright (c) 2012, Michal Simek <monstr@monstr.eu>
// Copyright (c) 2012, PetaLogix
// Copyright (c) 2011, Texas Instruments, Inc.
// Copyright (c) 2011, Google, Inc.
//
// Based on rpmsg performance statistics driver by Michal Simek, which in turn
// was based on TI & Google OMX rpmsg driver.
//

    static dev_t rpmsg_major;
    static DEFINE_IDA(rpmsg_ctrl_ida);
    static DEFINE_IDA(rpmsg_minor_ida);

//
// struct rpmsg_ctrldev - control device for instantiating endpoint devices
// @rpdev:	underlaying rpmsg device
// @cdev:	cdev for the ctrl device
// @dev:	device for the ctrl device
// @ctrl_lock:	serialize the ioctrls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpmsg_ctrldev {
    pub rpdev: *mut rpmsg_device,
    pub cdev: cdev,
    pub dev: device,
    pub ctrl_lock: mutex,
}

#[no_mangle]
unsafe extern "C" fn rpmsg_ctrldev_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int rpmsg_ctrldev_open(struct inode *inode, struct file *filp)
    {
    struct rpmsg_ctrldev *ctrldev = cdev_to_ctrldev(inode.i_cdev);
    get_device(&ctrldev.dev);
    filp.private_data = ctrldev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpmsg_ctrldev_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int rpmsg_ctrldev_release(struct inode *inode, struct file *filp)
    {
    struct rpmsg_ctrldev *ctrldev = cdev_to_ctrldev(inode.i_cdev);
    put_device(&ctrldev.dev);
    return 0;
    }
    static long rpmsg_ctrldev_ioctl(struct file *fp, unsigned int cmd,
    unsigned long arg)
    {
    struct rpmsg_ctrldev *ctrldev = fp.private_data;
    void __user *argp = (void __user *)arg;
    struct rpmsg_endpoint_info eptinfo;
    struct rpmsg_channel_info chinfo;
    struct rpmsg_device *rpdev;
    let mut ret: c_int = 0;
    if (copy_from_user(&eptinfo, argp, sizeof(eptinfo)))
    return -EFAULT;
    memcpy(chinfo.name, eptinfo.name, RPMSG_NAME_SIZE);
    chinfo.name[RPMSG_NAME_SIZE - 1] = '\0';
    chinfo.src = eptinfo.src;
    chinfo.dst = eptinfo.dst;
    mutex_lock(&ctrldev.ctrl_lock);
    switch (cmd) {
    case RPMSG_CREATE_EPT_IOCTL:
    ret = rpmsg_chrdev_eptdev_create(ctrldev.rpdev, &ctrldev.dev, chinfo);
    break;
    case RPMSG_CREATE_DEV_IOCTL:
    rpdev = rpmsg_create_channel(ctrldev.rpdev, &chinfo);
    if (!rpdev) {
    dev_err(&ctrldev.dev, "failed to create %s channel\n", chinfo.name);
    ret = -ENXIO;
    }
    break;
    case RPMSG_RELEASE_DEV_IOCTL:
    ret = rpmsg_release_channel(ctrldev.rpdev, &chinfo);
    if (ret)
    dev_err(&ctrldev.dev, "failed to release %s channel (%d)\n",
    chinfo.name, ret);
    break;
    default:
    ret = -EINVAL;
    }
    mutex_unlock(&ctrldev.ctrl_lock);
    return ret;
    };
    static const struct file_operations rpmsg_ctrldev_fops = {
    .owner = THIS_MODULE,
    .open = rpmsg_ctrldev_open,
    .release = rpmsg_ctrldev_release,
    .unlocked_ioctl = rpmsg_ctrldev_ioctl,
    .compat_ioctl = compat_ptr_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn rpmsg_ctrldev_release_device(dev: *mut device) {
    static void rpmsg_ctrldev_release_device(struct device *dev)
    {
    struct rpmsg_ctrldev *ctrldev = dev_to_ctrldev(dev);
    ida_free(&rpmsg_ctrl_ida, dev.id);
    ida_free(&rpmsg_minor_ida, MINOR(dev.devt));
    kfree(ctrldev);
    }
#[no_mangle]
unsafe extern "C" fn rpmsg_ctrldev_probe(rpdev: *mut rpmsg_device) -> c_int {
    static int rpmsg_ctrldev_probe(struct rpmsg_device *rpdev)
    {
    struct rpmsg_ctrldev *ctrldev;
    struct device *dev;
    int ret;
    ctrldev = kzalloc_obj(*ctrldev);
    if (!ctrldev)
    return -ENOMEM;
    ctrldev.rpdev = rpdev;
    dev = &ctrldev.dev;
    device_initialize(dev);
    dev.parent = &rpdev.dev;
    dev.class = &rpmsg_class;
    mutex_init(&ctrldev.ctrl_lock);
    cdev_init(&ctrldev.cdev, &rpmsg_ctrldev_fops);
    ctrldev.cdev.owner = THIS_MODULE;
    ret = ida_alloc_max(&rpmsg_minor_ida, RPMSG_DEV_MAX - 1, GFP_KERNEL);
    if (ret < 0)
    goto free_ctrldev;
    dev.devt = MKDEV(MAJOR(rpmsg_major), ret);
    ret = ida_alloc(&rpmsg_ctrl_ida, GFP_KERNEL);
    if (ret < 0)
    goto free_minor_ida;
    dev.id = ret;
    dev_set_name(&ctrldev.dev, "rpmsg_ctrl%d", ret);
    ret = cdev_device_add(&ctrldev.cdev, &ctrldev.dev);
    if (ret)
    goto free_ctrl_ida;
// We can now rely on the release function for cleanup
    dev.release = rpmsg_ctrldev_release_device;
    dev_set_drvdata(&rpdev.dev, ctrldev);
    return ret;
    free_ctrl_ida:
    ida_free(&rpmsg_ctrl_ida, dev.id);
    free_minor_ida:
    ida_free(&rpmsg_minor_ida, MINOR(dev.devt));
    free_ctrldev:
    put_device(dev);
    kfree(ctrldev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rpmsg_ctrldev_remove(rpdev: *mut rpmsg_device) {
    static void rpmsg_ctrldev_remove(struct rpmsg_device *rpdev)
    {
    struct rpmsg_ctrldev *ctrldev = dev_get_drvdata(&rpdev.dev);
    int ret;
    mutex_lock(&ctrldev.ctrl_lock);
// Destroy all endpoints
    ret = device_for_each_child(&ctrldev.dev, core::ptr::null_mut(), rpmsg_chrdev_eptdev_destroy);
    if (ret)
    dev_warn(&rpdev.dev, "failed to nuke endpoints: %d\n", ret);
    mutex_unlock(&ctrldev.ctrl_lock);
    cdev_device_del(&ctrldev.cdev, &ctrldev.dev);
    put_device(&ctrldev.dev);
    }
    static struct rpmsg_driver rpmsg_ctrldev_driver = {
    .probe = rpmsg_ctrldev_probe,
    .remove = rpmsg_ctrldev_remove,
    .drv = {
    .name = "rpmsg_ctrl",
    },
    };
#[no_mangle]
unsafe extern "C" fn rpmsg_ctrldev_init() -> c_int {
    static int rpmsg_ctrldev_init(void)
    {
    int ret;
    ret = alloc_chrdev_region(&rpmsg_major, 0, RPMSG_DEV_MAX, "rpmsg_ctrl");
    if (ret < 0) {
    pr_err("failed to allocate char dev region\n");
    return ret;
    }
    ret = register_rpmsg_driver(&rpmsg_ctrldev_driver);
    if (ret < 0) {
    pr_err("failed to register rpmsg driver\n");
    unregister_chrdev_region(rpmsg_major, RPMSG_DEV_MAX);
    }
    return ret;
    }
    postcore_initcall(rpmsg_ctrldev_init);
#[no_mangle]
unsafe extern "C" fn rpmsg_ctrldev_exit() {
    static void rpmsg_ctrldev_exit(void)
    {
    unregister_rpmsg_driver(&rpmsg_ctrldev_driver);
    unregister_chrdev_region(rpmsg_major, RPMSG_DEV_MAX);
    }
    module_exit(rpmsg_ctrldev_exit);
    MODULE_DESCRIPTION("rpmsg control interface");
    MODULE_ALIAS("rpmsg:" KBUILD_MODNAME);
    MODULE_LICENSE("GPL v2");
