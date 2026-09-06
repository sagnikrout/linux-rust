//! Automatically rewritten from C to Rust
//! Source: drivers/accel/drm_accel.c
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
// Copyright 2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

    DEFINE_XARRAY_ALLOC(accel_minors_xa);
    static const struct device_type accel_sysfs_device_minor = {
    .name = "accel_minor"
    };
    static char *accel_devnode(const struct device *dev, umode_t *mode)
    {
    return kasprintf(GFP_KERNEL, "accel/%s", dev_name(dev));
    }
    static const struct class accel_class = {
    .name = "accel",
    .devnode = accel_devnode,
    };
#[no_mangle]
unsafe extern "C" fn accel_sysfs_init() -> c_int {
    static int accel_sysfs_init(void)
    {
    return class_register(&accel_class);
    }
#[no_mangle]
unsafe extern "C" fn accel_sysfs_destroy() {
    static void accel_sysfs_destroy(void)
    {
    class_unregister(&accel_class);
    }
#[no_mangle]
unsafe extern "C" fn accel_name_info(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int accel_name_info(struct seq_file *m, void *data)
    {
    struct drm_info_node *node = (struct drm_info_node *) m.private;
    struct drm_minor *minor = node.minor;
    struct drm_device *dev = minor.dev;
    struct drm_master *master;
    mutex_lock(&dev.master_mutex);
    master = dev.master;
    seq_printf(m, "%s", dev.driver.name);
    if (dev.dev)
    seq_printf(m, " dev=%s", dev_name(dev.dev));
    if (master && master.unique)
    seq_printf(m, " master=%s", master.unique);
    if (dev.unique)
    seq_printf(m, " unique=%s", dev.unique);
    seq_puts(m, "\n");
    mutex_unlock(&dev.master_mutex);
    return 0;
    }
    static const struct drm_info_list accel_debugfs_list[] = {
    {"name", accel_name_info, 0}
    };

//
// accel_debugfs_register() - Register debugfs for device
// @dev: Pointer to the device instance.
//
// Creates common files for accelerators.
//
#[no_mangle]
pub unsafe extern "C" fn accel_debugfs_register(dev: *mut drm_device) {
    void accel_debugfs_register(struct drm_device *dev)
    {
    struct drm_minor *minor = dev.accel;
    minor.debugfs_root = dev.debugfs_root;
    drm_debugfs_create_files(accel_debugfs_list, ACCEL_DEBUGFS_ENTRIES,
    dev.debugfs_root, minor);
    }
//
// accel_set_device_instance_params() - Set some device parameters for accel device
// @kdev: Pointer to the device instance.
// @index: The minor's index
//
// This function creates the dev_t of the device using the accel major and
// the device's minor number. In addition, it sets the class and type of the
// device instance to the accel sysfs class and device type, respectively.
//
#[no_mangle]
pub unsafe extern "C" fn accel_set_device_instance_params(kdev: *mut device, index: c_int) {
    void accel_set_device_instance_params(struct device *kdev, int index)
    {
    kdev.devt = MKDEV(ACCEL_MAJOR, index);
    kdev.class = &accel_class;
    kdev.type = &accel_sysfs_device_minor;
    }
//
// accel_open - open method for ACCEL file
// @inode: device inode
// @filp: file pointer.
//
// This function must be used by drivers as their &file_operations.open method.
// It looks up the correct ACCEL device and instantiates all the per-file
// resources for it. It also calls the &drm_driver.open driver callback.
//
// Return: 0 on success or negative errno value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn accel_open(inode: *mut inode, filp: *mut file) -> c_int {
    int accel_open(struct inode *inode, struct file *filp)
    {
    struct drm_device *dev;
    struct drm_minor *minor;
    int retcode;
    minor = drm_minor_acquire(&accel_minors_xa, iminor(inode));
    if (IS_ERR(minor))
    return PTR_ERR(minor);
    dev = minor.dev;
    atomic_fetch_inc(&dev.open_count);
// share address_space across all char-devs of a single device
    filp.f_mapping = dev.anon_inode.i_mapping;
    retcode = drm_open_helper(filp, minor);
    if (retcode)
    goto err_undo;
    return 0;
    err_undo:
    atomic_dec(&dev.open_count);
    drm_minor_release(minor);
    return retcode;
    }
    EXPORT_SYMBOL_GPL(accel_open);
#[no_mangle]
unsafe extern "C" fn accel_stub_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int accel_stub_open(struct inode *inode, struct file *filp)
    {
    const struct file_operations *new_fops;
    struct drm_minor *minor;
    int err;
    minor = drm_minor_acquire(&accel_minors_xa, iminor(inode));
    if (IS_ERR(minor))
    return PTR_ERR(minor);
    new_fops = fops_get(minor.dev.driver.fops);
    if (!new_fops) {
    err = -ENODEV;
    goto out;
    }
    replace_fops(filp, new_fops);
    if (filp.f_op.open)
    err = filp.f_op.open(inode, filp);
    else
    err = 0;
    out:
    drm_minor_release(minor);
    return err;
    }
    static const struct file_operations accel_stub_fops = {
    .owner = THIS_MODULE,
    .open = accel_stub_open,
    .llseek = noop_llseek,
    };
#[no_mangle]
pub unsafe extern "C" fn accel_core_exit() {
    void accel_core_exit(void)
    {
    unregister_chrdev(ACCEL_MAJOR, "accel");
    accel_sysfs_destroy();
    WARN_ON(!xa_empty(&accel_minors_xa));
    }
#[no_mangle]
pub unsafe extern "C" fn accel_core_init() -> int __init {
    int __init accel_core_init(void)
    {
    int ret;
    ret = accel_sysfs_init();
    if (ret < 0) {
    DRM_ERROR("Cannot create ACCEL class: %d\n", ret);
    goto error;
    }
    ret = register_chrdev(ACCEL_MAJOR, "accel", &accel_stub_fops);
    if (ret < 0)
    DRM_ERROR("Cannot register ACCEL major: %d\n", ret);
    error:
//
// Any cleanup due to errors will be done in drm_core_exit() that
// will call accel_core_exit()
//
    return ret;
    }
