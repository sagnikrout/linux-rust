//! Automatically rewritten from C to Rust
//! Source: drivers/spmi/spmi-devres.c
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
// Copyright 2023 Google LLC.
//

#[no_mangle]
unsafe extern "C" fn devm_spmi_controller_release(parent: *mut device, res: *mut c_void) {
    static void devm_spmi_controller_release(struct device *parent, void *res)
    {
    spmi_controller_put(*(struct spmi_controller **)res);
    }
    struct spmi_controller *devm_spmi_controller_alloc(struct device *parent, size_t size)
    {
    struct spmi_controller **ptr, *ctrl;
    ptr = devres_alloc(devm_spmi_controller_release, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return ERR_PTR(-ENOMEM);
    ctrl = spmi_controller_alloc(parent, size);
    if (IS_ERR(ctrl)) {
    devres_free(ptr);
    return ctrl;
    }
// ptr = ctrl;
    devres_add(parent, ptr);
    return ctrl;
    }
    EXPORT_SYMBOL_GPL(devm_spmi_controller_alloc);
#[no_mangle]
unsafe extern "C" fn devm_spmi_controller_remove(parent: *mut device, res: *mut c_void) {
    static void devm_spmi_controller_remove(struct device *parent, void *res)
    {
    spmi_controller_remove(*(struct spmi_controller **)res);
    }
#[no_mangle]
pub unsafe extern "C" fn devm_spmi_controller_add(parent: *mut device, ctrl: *mut spmi_controller) -> c_int {
    int devm_spmi_controller_add(struct device *parent, struct spmi_controller *ctrl)
    {
    struct spmi_controller **ptr;
    int ret;
    ptr = devres_alloc(devm_spmi_controller_remove, sizeof(*ptr), GFP_KERNEL);
    if (!ptr)
    return -ENOMEM;
    ret = spmi_controller_add(ctrl);
    if (ret) {
    devres_free(ptr);
    return ret;
    }
// ptr = ctrl;
    devres_add(parent, ptr);
    return 0;
    }
    EXPORT_SYMBOL_GPL(devm_spmi_controller_add);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("SPMI devres helpers");
