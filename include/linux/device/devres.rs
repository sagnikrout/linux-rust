//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device/devres.h
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

// device resource management
extern "C" {
    pub fn void(dev: *mut *mut dr_release_t)(struct device, res: *mut c_void) -> typedef;
}
extern "C" {
    pub fn int(dev: *mut *mut dr_match_t)(struct device, res: *mut c_void, match_data: *mut c_void) -> typedef;
}

extern "C" {
    pub fn devres_free(res: *mut c_void);
}
extern "C" {
    pub fn devres_add(dev: *mut device, res: *mut c_void);
}
extern "C" {
    pub fn devres_destroy(dev: *mut device, release: dr_release_t, match: dr_match_t, match_data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn devres_release(dev: *mut device, release: dr_release_t, match: dr_match_t, match_data: *mut c_void) -> c_int;
}
// devres group
extern "C" {
    pub fn devres_open_group(dev: *mut device, id: *mut c_void, gfp: gfp_t) -> *mut void  __must_check;
}
extern "C" {
    pub fn devres_close_group(dev: *mut device, id: *mut c_void);
}
extern "C" {
    pub fn devres_remove_group(dev: *mut device, id: *mut c_void);
}
extern "C" {
    pub fn devres_release_group(dev: *mut device, id: *mut c_void) -> c_int;
}
// managed devm_k.alloc/kfree for device drivers
extern "C" {
    pub fn devm_kmalloc(_arg: dev, _arg: size, __GFP_ZERO: gfp |) -> return;
}
extern "C" {
    pub fn devm_kmalloc(_arg: dev, _arg: bytes, _arg: flags) -> return;
}
extern "C" {
    pub fn devm_kmalloc_array(_arg: dev, _arg: n, _arg: size, __GFP_ZERO: flags |) -> return;
}
extern "C" {
    pub fn devm_krealloc(_arg: dev, _arg: p, _arg: bytes, _arg: flags) -> return;
}
extern "C" {
    pub fn devm_kfree(dev: *mut device, p: *const c_void);
}
extern "C" {
    pub fn devm_kmemdup(_arg: dev, _arg: src, _arg: size_mul(size, _arg: n), _arg: flags) -> return;
}
//
// devm_alloc_percpu - Resource-managed alloc_percpu
// @dev: Device to allocate per-cpu memory for
// @type: Type to allocate per-cpu memory for
//
// Managed alloc_percpu. Per-cpu memory allocated with this function is
// automatically freed on driver detach.
//
// RETURNS:
// Pointer to allocated memory on success, NULL on failure.
//

extern "C" {
    pub fn devm_get_free_pages(dev: *mut device, gfp_mask: gfp_t, order: c_uint) -> c_ulong;
}
extern "C" {
    pub fn devm_free_pages(dev: *mut device, addr: c_ulong);
}

extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}

// allows to add/remove a custom action to devres stack
extern "C" {
    pub fn devm_remove_action_nowarn(dev: *mut device, ): *mut *mut void (action)(void, data: *mut c_void) -> c_int;
}
//
// devm_remove_action() - removes previously added custom action
// @dev: Device that owns the action
// @action: Function implementing the action
// @data: Pointer to data passed to @action implementation
//
// Removes instance of @action previously added by devm_add_action().
// Both action and data should match one of the existing entries.
//
extern "C" {
    pub fn devm_release_action(dev: *mut device, ): *mut *mut void (action)(void, data: *mut c_void);
}
extern "C" {
    pub fn __devm_add_action(dev: *mut device, ): *mut *mut void (action)(void, data: *mut c_void, name: *const c_char) -> c_int;
}

extern "C" {
    pub fn devm_is_action_added(dev: *mut device, ): *mut *mut void (action)(void, data: *mut c_void) -> bool;
}
