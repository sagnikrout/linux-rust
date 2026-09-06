//! Automatically rewritten from C to Rust
//! Source: drivers/base/faux.c
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
// Copyright (c) 2025 Greg Kroah-Hartman <gregkh@linuxfoundation.org>
// Copyright (c) 2025 The Linux Foundation
//
// A "simple" faux bus that allows devices to be created and added
// automatically to it.  This is to be used whenever you need to create a
// device that is not associated with any "real" system resources, and do
// not want to have to deal with a bus/driver binding logic.  It is
// intended to be very simple, with only a create and a destroy function
// available.
//

//
// Internal wrapper structure so we can hold a pointer to the
// faux_device_ops for this device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct faux_object {
    pub faux_dev: faux_device,
    pub faux_ops: *const faux_device_ops,
    pub groups: *const attribute_group,
}

    static struct device *faux_bus_root;
#[no_mangle]
unsafe extern "C" fn faux_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int faux_match(struct device *dev, const struct device_driver *drv)
    {
// Match always succeeds, we only have one driver
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn faux_probe(dev: *mut device) -> c_int {
    static int faux_probe(struct device *dev)
    {
    struct faux_object *faux_obj = to_faux_object(dev);
    struct faux_device *faux_dev = &faux_obj.faux_dev;
    const struct faux_device_ops *faux_ops = faux_obj.faux_ops;
    int ret;
    if (faux_ops && faux_ops.probe) {
    ret = faux_ops.probe(faux_dev);
    if (ret)
    return ret;
    }
//
// Add groups after the probe succeeds to ensure resources are
// initialized correctly
//
    ret = device_add_groups(dev, faux_obj.groups);
    if (ret && faux_ops && faux_ops.remove)
    faux_ops.remove(faux_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn faux_remove(dev: *mut device) {
    static void faux_remove(struct device *dev)
    {
    struct faux_object *faux_obj = to_faux_object(dev);
    struct faux_device *faux_dev = &faux_obj.faux_dev;
    const struct faux_device_ops *faux_ops = faux_obj.faux_ops;
    device_remove_groups(dev, faux_obj.groups);
    if (faux_ops && faux_ops.remove)
    faux_ops.remove(faux_dev);
    }
    static const struct bus_type faux_bus_type = {
    .name		= "faux",
    .match		= faux_match,
    .probe		= faux_probe,
    .remove		= faux_remove,
    };
    static struct device_driver faux_driver = {
    .name		= "faux_driver",
    .bus		= &faux_bus_type,
    .probe_type	= PROBE_FORCE_SYNCHRONOUS,
    .suppress_bind_attrs = true,
    };
#[no_mangle]
unsafe extern "C" fn faux_device_release(dev: *mut device) {
    static void faux_device_release(struct device *dev)
    {
    struct faux_object *faux_obj = to_faux_object(dev);
    kfree(faux_obj);
    }
//
// faux_device_create_with_groups - Create and register with the driver
// core a faux device and populate the device with an initial
// set of sysfs attributes.
// @name:	The name of the device we are adding, must be unique for
// all faux devices.
// @parent:	Pointer to a potential parent struct device.  If set to
// NULL, the device will be created in the "root" of the faux
// device tree in sysfs.
// @faux_ops:	struct faux_device_ops that the new device will call back
// into, can be NULL.
// @groups:	The set of sysfs attributes that will be created for this
// device when it is registered with the driver core.
//
// Create a new faux device and register it in the driver core properly.
// If present, callbacks in @faux_ops will be called with the device that
// for the caller to do something with at the proper time given the
// device's lifecycle.
//
// Note, when this function is called, the functions specified in struct
// faux_ops can be called before the function returns, so be prepared for
// everything to be properly initialized before that point in time.  If the
// probe callback (if one is present) does NOT succeed, the creation of the
// device will fail and NULL will be returned.
//
// Return:
// * NULL if an error happened with creating the device
// * pointer to a valid struct faux_device that is registered with sysfs
//
    struct faux_device *faux_device_create_with_groups(const char *name,
    struct device *parent,
    const struct faux_device_ops *faux_ops,
    const struct attribute_group **groups)
    {
    struct faux_object *faux_obj;
    struct faux_device *faux_dev;
    struct device *dev;
    int ret;
    if (!faux_bus_root)
    return core::ptr::null_mut();
    faux_obj = kzalloc_obj(*faux_obj);
    if (!faux_obj)
    return core::ptr::null_mut();
// Save off the callbacks and groups so we can use them in the future
    faux_obj.faux_ops = faux_ops;
    faux_obj.groups = groups;
// Initialize the device portion and register it with the driver core
    faux_dev = &faux_obj.faux_dev;
    dev = &faux_dev.dev;
    device_initialize(dev);
    dev.release = faux_device_release;
    if (parent)
    dev.parent = parent;
    else
    dev.parent = faux_bus_root;
    dev.bus = &faux_bus_type;
    dev_set_name(dev, "%s", name);
    device_set_pm_not_required(dev);
    ret = device_add(dev);
    if (ret) {
    pr_err("%s: device_add for faux device '%s' failed with %d\n",
    __func__, name, ret);
    put_device(dev);
    return core::ptr::null_mut();
    }
//
// Verify that we did bind the driver to the device (i.e. probe worked),
// if not, let's fail the creation as trying to guess if probe was
// successful is almost impossible to determine by the caller.
//
    if (!dev.driver) {
    dev_dbg(dev, "probe did not succeed, tearing down the device\n");
    faux_device_destroy(faux_dev);
    faux_dev = core::ptr::null_mut();
    }
    return faux_dev;
    }
    EXPORT_SYMBOL_GPL(faux_device_create_with_groups);
//
// faux_device_create - create and register with the driver core a faux device
// @name:	The name of the device we are adding, must be unique for all
// faux devices.
// @parent:	Pointer to a potential parent struct device.  If set to
// NULL, the device will be created in the "root" of the faux
// device tree in sysfs.
// @faux_ops:	struct faux_device_ops that the new device will call back
// into, can be NULL.
//
// Create a new faux device and register it in the driver core properly.
// If present, callbacks in @faux_ops will be called with the device that
// for the caller to do something with at the proper time given the
// device's lifecycle.
//
// Note, when this function is called, the functions specified in struct
// faux_ops can be called before the function returns, so be prepared for
// everything to be properly initialized before that point in time.
//
// Return:
// * NULL if an error happened with creating the device
// * pointer to a valid struct faux_device that is registered with sysfs
//
    struct faux_device *faux_device_create(const char *name,
    struct device *parent,
    const struct faux_device_ops *faux_ops)
    {
    return faux_device_create_with_groups(name, parent, faux_ops, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(faux_device_create);
//
// faux_device_destroy - destroy a faux device
// @faux_dev:	faux device to destroy
//
// Unregisters and cleans up a device that was created with a call to
// faux_device_create()
//
#[no_mangle]
pub unsafe extern "C" fn faux_device_destroy(faux_dev: *mut faux_device) {
    void faux_device_destroy(struct faux_device *faux_dev)
    {
    struct device *dev = &faux_dev.dev;
    if (!faux_dev)
    return;
    device_del(dev);
// The final put_device() will clean up the memory we allocated for this device.
    put_device(dev);
    }
    EXPORT_SYMBOL_GPL(faux_device_destroy);
#[no_mangle]
pub unsafe extern "C" fn faux_bus_init() -> int __init {
    int __init faux_bus_init(void)
    {
    struct device *root;
    int ret;
    root = root_device_register("faux");
    if (IS_ERR(root))
    return PTR_ERR(root);
    ret = bus_register(&faux_bus_type);
    if (ret)
    goto err_deregister_root;
    ret = driver_register(&faux_driver);
    if (ret)
    goto err_deregister_bus;
    faux_bus_root = root;
    return 0;
    err_deregister_bus:
    bus_unregister(&faux_bus_type);
    err_deregister_root:
    root_device_unregister(root);
    return ret;
    }
