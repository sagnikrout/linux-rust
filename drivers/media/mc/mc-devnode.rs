//! Automatically rewritten from C to Rust
//! Source: drivers/media/mc/mc-devnode.c
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
// Media device node
//
// Copyright (C) 2010 Nokia Corporation
//
// Based on drivers/media/video/v4l2_dev.c code authored by
// Mauro Carvalho Chehab <mchehab@kernel.org> (version 2)
// Alan Cox, <alan@lxorguk.ukuu.org.uk> (version 1)
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//
// --
//
// Generic media device node infrastructure to register and unregister
// character devices using a dynamic major number and proper reference
// counting.
//

pub const MEDIA_NUM_DEVICES: c_int = 256;

    static dev_t media_dev_t;
//
// Active devices
//
    static DEFINE_MUTEX(media_devnode_lock);
    static DECLARE_BITMAP(media_devnode_nums, MEDIA_NUM_DEVICES);
// debugfs
    struct dentry *media_debugfs_root;
// Called when the last user of the media device exits.
#[no_mangle]
unsafe extern "C" fn media_devnode_release(cd: *mut device) {
    static void media_devnode_release(struct device *cd)
    {
    struct media_devnode *devnode = to_media_devnode(cd);
// Release media_devnode and perform other cleanups as needed.
    if (devnode.release)
    devnode.release(devnode);
    kfree(devnode);
    pr_debug("%s: Media Devnode Deallocated\n", __func__);
    }
    static const struct bus_type media_bus_type = {
    .name = MEDIA_NAME,
    };
    static ssize_t media_read(struct file *filp, char __user *buf,
    size_t sz, loff_t *off)
    {
    struct media_devnode *devnode = media_devnode_data(filp);
    if (!devnode.fops.read)
    return -EINVAL;
    if (!media_devnode_is_registered(devnode))
    return -EIO;
    return devnode.fops.read(filp, buf, sz, off);
    }
    static ssize_t media_write(struct file *filp, const char __user *buf,
    size_t sz, loff_t *off)
    {
    struct media_devnode *devnode = media_devnode_data(filp);
    if (!devnode.fops.write)
    return -EINVAL;
    if (!media_devnode_is_registered(devnode))
    return -EIO;
    return devnode.fops.write(filp, buf, sz, off);
    }
    static __poll_t media_poll(struct file *filp,
    struct poll_table_struct *poll)
    {
    struct media_devnode *devnode = media_devnode_data(filp);
    if (!media_devnode_is_registered(devnode))
    return EPOLLERR | EPOLLHUP;
    if (!devnode.fops.poll)
    return DEFAULT_POLLMASK;
    return devnode.fops.poll(filp, poll);
    }
    static long
    __media_ioctl(struct file *filp, unsigned int cmd, unsigned long arg,
    long (*ioctl_func)(struct file *filp, unsigned int cmd,
    unsigned long arg))
    {
    struct media_devnode *devnode = media_devnode_data(filp);
    if (!ioctl_func)
    return -ENOTTY;
    if (!media_devnode_is_registered(devnode))
    return -EIO;
    return ioctl_func(filp, cmd, arg);
    }
#[no_mangle]
unsafe extern "C" fn media_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long media_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
    {
    struct media_devnode *devnode = media_devnode_data(filp);
    return __media_ioctl(filp, cmd, arg, devnode.fops.ioctl);
    }

    static long media_compat_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    struct media_devnode *devnode = media_devnode_data(filp);
    return __media_ioctl(filp, cmd, arg, devnode.fops.compat_ioctl);
    }

// Override for the open function
#[no_mangle]
unsafe extern "C" fn media_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int media_open(struct inode *inode, struct file *filp)
    {
    struct media_devnode *devnode;
    int ret;
// Check if the media device is available. This needs to be done with
// the media_devnode_lock held to prevent an open/unregister race:
// without the lock, the device could be unregistered and freed between
// the media_devnode_is_registered() and get_device() calls, leading to
// a crash.
//
    mutex_lock(&media_devnode_lock);
    devnode = container_of(inode.i_cdev, struct media_devnode, cdev);
// return ENXIO if the media device has been removed
    already or if it is not registered anymore. */
    if (!media_devnode_is_registered(devnode)) {
    mutex_unlock(&media_devnode_lock);
    return -ENXIO;
    }
// and increase the device refcount
    get_device(&devnode.dev);
    mutex_unlock(&media_devnode_lock);
    filp.private_data = devnode;
    if (devnode.fops.open) {
    ret = devnode.fops.open(filp);
    if (ret) {
    put_device(&devnode.dev);
    filp.private_data = core::ptr::null_mut();
    return ret;
    }
    }
    return 0;
    }
// Override for the release function
#[no_mangle]
unsafe extern "C" fn media_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int media_release(struct inode *inode, struct file *filp)
    {
    struct media_devnode *devnode = media_devnode_data(filp);
    if (devnode.fops.release)
    devnode.fops.release(filp);
    filp.private_data = core::ptr::null_mut();
// decrease the refcount unconditionally since the release()
    return value is ignored. */
    put_device(&devnode.dev);
    return 0;
    }
    static const struct file_operations media_devnode_fops = {
    .owner = THIS_MODULE,
    .read = media_read,
    .write = media_write,
    .open = media_open,
    .unlocked_ioctl = media_ioctl,

    .compat_ioctl = media_compat_ioctl,

    .release = media_release,
    .poll = media_poll,
    };
    int __must_check media_devnode_register(struct media_device *mdev,
    struct media_devnode *devnode,
    struct module *owner)
    {
    int minor;
    int ret;
// Part 1: Find a free minor number
    mutex_lock(&media_devnode_lock);
    minor = find_first_zero_bit(media_devnode_nums, MEDIA_NUM_DEVICES);
    if (minor == MEDIA_NUM_DEVICES) {
    mutex_unlock(&media_devnode_lock);
    pr_err("could not get a free minor\n");
    kfree(devnode);
    return -ENFILE;
    }
    set_bit(minor, media_devnode_nums);
    mutex_unlock(&media_devnode_lock);
    devnode.minor = minor;
    devnode.media_dev = mdev;
// Part 1: Initialize dev now to use dev.kobj for cdev.kobj.parent
    devnode.dev.bus = &media_bus_type;
    devnode.dev.devt = MKDEV(MAJOR(media_dev_t), devnode.minor);
    devnode.dev.release = media_devnode_release;
    if (devnode.parent)
    devnode.dev.parent = devnode.parent;
    dev_set_name(&devnode.dev, "media%d", devnode.minor);
    dev_set_drvdata(&devnode.dev, mdev);
    device_initialize(&devnode.dev);
// Part 2: Initialize the character device
    cdev_init(&devnode.cdev, &media_devnode_fops);
    devnode.cdev.owner = owner;
    kobject_set_name(&devnode.cdev.kobj, "media%d", devnode.minor);
// Part 3: Add the media and char device
    set_bit(MEDIA_FLAG_REGISTERED, &devnode.flags);
    ret = cdev_device_add(&devnode.cdev, &devnode.dev);
    if (ret < 0) {
    clear_bit(MEDIA_FLAG_REGISTERED, &devnode.flags);
    pr_err("%s: cdev_device_add failed\n", __func__);
    goto cdev_add_error;
    }
    return 0;
    cdev_add_error:
    mutex_lock(&media_devnode_lock);
    clear_bit(devnode.minor, media_devnode_nums);
    devnode.media_dev = core::ptr::null_mut();
    mutex_unlock(&media_devnode_lock);
    put_device(&devnode.dev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn media_devnode_unregister_prepare(devnode: *mut media_devnode) {
    void media_devnode_unregister_prepare(struct media_devnode *devnode)
    {
// Check if devnode was ever registered at all
    if (!media_devnode_is_registered(devnode))
    return;
    mutex_lock(&media_devnode_lock);
    clear_bit(MEDIA_FLAG_REGISTERED, &devnode.flags);
    mutex_unlock(&media_devnode_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn media_devnode_unregister(devnode: *mut media_devnode) {
    void media_devnode_unregister(struct media_devnode *devnode)
    {
    mutex_lock(&media_devnode_lock);
// Delete the cdev on this minor as well
    cdev_device_del(&devnode.cdev, &devnode.dev);
    devnode.media_dev = core::ptr::null_mut();
    clear_bit(devnode.minor, media_devnode_nums);
    mutex_unlock(&media_devnode_lock);
    put_device(&devnode.dev);
    }
//
// Initialise media for linux
//
#[no_mangle]
unsafe extern "C" fn media_devnode_init() -> int __init {
    static int __init media_devnode_init(void)
    {
    int ret;
    pr_info("Linux media interface: v0.10\n");
    ret = alloc_chrdev_region(&media_dev_t, 0, MEDIA_NUM_DEVICES,
    MEDIA_NAME);
    if (ret < 0) {
    pr_warn("unable to allocate major\n");
    return ret;
    }
    ret = bus_register(&media_bus_type);
    if (ret < 0) {
    unregister_chrdev_region(media_dev_t, MEDIA_NUM_DEVICES);
    pr_warn("bus_register failed\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn media_devnode_exit() -> void __exit {
    static void __exit media_devnode_exit(void)
    {
    debugfs_remove_recursive(media_debugfs_root);
    bus_unregister(&media_bus_type);
    unregister_chrdev_region(media_dev_t, MEDIA_NUM_DEVICES);
    }
    subsys_initcall(media_devnode_init);
    module_exit(media_devnode_exit)
    MODULE_AUTHOR("Laurent Pinchart <laurent.pinchart@ideasonboard.com>");
    MODULE_DESCRIPTION("Device node registration for media drivers");
    MODULE_LICENSE("GPL");
