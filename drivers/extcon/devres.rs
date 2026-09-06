//! Automatically rewritten from C to Rust
//! Source: drivers/extcon/devres.c
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
// drivers/extcon/devres.c - EXTCON device's resource management
//
// Copyright (C) 2016 Samsung Electronics
// Author: Chanwoo Choi <cw00.choi@samsung.com>
//

#[no_mangle]
unsafe extern "C" fn devm_extcon_dev_match(dev: *mut device, res: *mut c_void, data: *mut c_void) -> c_int {
    static int devm_extcon_dev_match(struct device *dev, void *res, void *data)
    {
    struct extcon_dev **r = res;
    if (WARN_ON(!r || !*r))
    return 0;
    return *r == data;
    }
#[no_mangle]
unsafe extern "C" fn devm_extcon_dev_release(dev: *mut device, res: *mut c_void) {
    static void devm_extcon_dev_release(struct device *dev, void *res)
    {
    extcon_dev_free(*(struct extcon_dev **)res);
    }
#[no_mangle]
unsafe extern "C" fn devm_extcon_dev_unreg(dev: *mut device, res: *mut c_void) {
    static void devm_extcon_dev_unreg(struct device *dev, void *res)
    {
    extcon_dev_unregister(*(struct extcon_dev **)res);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extcon_dev_notifier_devres {
    pub edev: *mut extcon_dev,
    pub id: c_uint,
    pub nb: *mut notifier_block,
}

#[no_mangle]
unsafe extern "C" fn devm_extcon_dev_notifier_unreg(dev: *mut device, res: *mut c_void) {
    static void devm_extcon_dev_notifier_unreg(struct device *dev, void *res)
    {
    struct extcon_dev_notifier_devres *this = res;
    extcon_unregister_notifier(this.edev, this.id, this.nb);
    }
#[no_mangle]
unsafe extern "C" fn devm_extcon_dev_notifier_all_unreg(dev: *mut device, res: *mut c_void) {
    static void devm_extcon_dev_notifier_all_unreg(struct device *dev, void *res)
    {
    struct extcon_dev_notifier_devres *this = res;
    extcon_unregister_notifier_all(this.edev, this.nb);
    }
//
// devm_extcon_dev_allocate - Allocate managed extcon device
// @dev:		the device owning the extcon device being created
// @supported_cable:	the array of the supported external connectors
// ending with EXTCON_NONE.
//
// This function manages automatically the memory of extcon device using device
// resource management and simplify the control of freeing the memory of extcon
// device.
//
// Returns the pointer memory of allocated extcon_dev if success
// or ERR_PTR(err) if fail
//
    struct extcon_dev *devm_extcon_dev_allocate(struct device *dev,
    const unsigned int *supported_cable)
    {
    struct extcon_dev **ptr, *edev;
    ptr = devres_alloc(devm_extcon_dev_release, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return ERR_PTR(-ENOMEM);
    edev = extcon_dev_allocate(supported_cable);
    if (IS_ERR(edev)) {
    devres_free(ptr);
    return edev;
    }
    edev.dev.parent = dev;
// ptr = edev;
    devres_add(dev, ptr);
    return edev;
    }
    EXPORT_SYMBOL_GPL(devm_extcon_dev_allocate);
//
// devm_extcon_dev_free() - Resource-managed extcon_dev_unregister()
// @dev:	the device owning the extcon device being created
// @edev:	the extcon device to be freed
//
// Free the memory that is allocated with devm_extcon_dev_allocate()
// function.
//
#[no_mangle]
pub unsafe extern "C" fn devm_extcon_dev_free(dev: *mut device, edev: *mut extcon_dev) {
    void devm_extcon_dev_free(struct device *dev, struct extcon_dev *edev)
    {
    WARN_ON(devres_release(dev, devm_extcon_dev_release,
    devm_extcon_dev_match, edev));
    }
    EXPORT_SYMBOL_GPL(devm_extcon_dev_free);
//
// devm_extcon_dev_register() - Resource-managed extcon_dev_register()
// @dev:	the device owning the extcon device being created
// @edev:	the extcon device to be registered
//
// this function, that extcon device is automatically unregistered on driver
// detach. Internally this function calls extcon_dev_register() function.
// To get more information, refer that function.
//
// If extcon device is registered with this function and the device needs to be
// unregistered separately, devm_extcon_dev_unregister() should be used.
//
// Returns 0 if success or negaive error number if failure.
//
#[no_mangle]
pub unsafe extern "C" fn devm_extcon_dev_register(dev: *mut device, edev: *mut extcon_dev) -> c_int {
    int devm_extcon_dev_register(struct device *dev, struct extcon_dev *edev)
    {
    struct extcon_dev **ptr;
    int ret;
    ptr = devres_alloc(devm_extcon_dev_unreg, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = extcon_dev_register(edev);
    if (ret) {
    devres_free(ptr);
    return ret;
    }
// ptr = edev;
    devres_add(dev, ptr);
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_extcon_dev_register);
//
// devm_extcon_dev_unregister() - Resource-managed extcon_dev_unregister()
// @dev:	the device owning the extcon device being created
// @edev:	the extcon device to unregistered
//
// Unregister extcon device that is registered with devm_extcon_dev_register()
// function.
//
#[no_mangle]
pub unsafe extern "C" fn devm_extcon_dev_unregister(dev: *mut device, edev: *mut extcon_dev) {
    void devm_extcon_dev_unregister(struct device *dev, struct extcon_dev *edev)
    {
    WARN_ON(devres_release(dev, devm_extcon_dev_unreg,
    devm_extcon_dev_match, edev));
    }
    EXPORT_SYMBOL_GPL(devm_extcon_dev_unregister);
//
// devm_extcon_register_notifier() - Resource-managed extcon_register_notifier()
// @dev:	the device owning the extcon device being created
// @edev:	the extcon device
// @id:		the unique id among the extcon enumeration
// @nb:		a notifier block to be registered
//
// This function manages automatically the notifier of extcon device using
// device resource management and simplify the control of unregistering
// the notifier of extcon device.
//
// Note that the second parameter given to the callback of nb (val) is
// "old_state", not the current state. The current state can be retrieved
// by looking at the third pameter (edev pointer)'s state value.
//
// Returns 0 if success or negaive error number if failure.
//
    int devm_extcon_register_notifier(struct device *dev, struct extcon_dev *edev,
    unsigned int id, struct notifier_block *nb)
    {
    struct extcon_dev_notifier_devres *ptr;
    int ret;
    ptr = devres_alloc(devm_extcon_dev_notifier_unreg, sizeof(*ptr),
    GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = extcon_register_notifier(edev, id, nb);
    if (ret) {
    devres_free(ptr);
    return ret;
    }
    ptr.edev = edev;
    ptr.id = id;
    ptr.nb = nb;
    devres_add(dev, ptr);
    return 0;
    }
    EXPORT_SYMBOL(devm_extcon_register_notifier);
//
// devm_extcon_unregister_notifier()
// - Resource-managed extcon_unregister_notifier()
// @dev:	the device owning the extcon device being created
// @edev:	the extcon device
// @id:		the unique id among the extcon enumeration
// @nb:		a notifier block to be registered
//
    void devm_extcon_unregister_notifier(struct device *dev,
    struct extcon_dev *edev, unsigned int id,
    struct notifier_block *nb)
    {
    WARN_ON(devres_release(dev, devm_extcon_dev_notifier_unreg,
    devm_extcon_dev_match, edev));
    }
    EXPORT_SYMBOL(devm_extcon_unregister_notifier);
//
// devm_extcon_register_notifier_all()
// - Resource-managed extcon_register_notifier_all()
// @dev:	the device owning the extcon device being created
// @edev:	the extcon device
// @nb:		a notifier block to be registered
//
// This function manages automatically the notifier of extcon device using
// device resource management and simplify the control of unregistering
// the notifier of extcon device. To get more information, refer that function.
//
// Returns 0 if success or negaive error number if failure.
//
    int devm_extcon_register_notifier_all(struct device *dev, struct extcon_dev *edev,
    struct notifier_block *nb)
    {
    struct extcon_dev_notifier_devres *ptr;
    int ret;
    ptr = devres_alloc(devm_extcon_dev_notifier_all_unreg, sizeof(*ptr),
    GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = extcon_register_notifier_all(edev, nb);
    if (ret) {
    devres_free(ptr);
    return ret;
    }
    ptr.edev = edev;
    ptr.nb = nb;
    devres_add(dev, ptr);
    return 0;
    }
    EXPORT_SYMBOL(devm_extcon_register_notifier_all);
//
// devm_extcon_unregister_notifier_all()
// - Resource-managed extcon_unregister_notifier_all()
// @dev:	the device owning the extcon device being created
// @edev:	the extcon device
// @nb:		a notifier block to be registered
//
    void devm_extcon_unregister_notifier_all(struct device *dev,
    struct extcon_dev *edev,
    struct notifier_block *nb)
    {
    WARN_ON(devres_release(dev, devm_extcon_dev_notifier_all_unreg,
    devm_extcon_dev_match, edev));
    }
    EXPORT_SYMBOL(devm_extcon_unregister_notifier_all);
