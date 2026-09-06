//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_managed.h
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

extern "C" {
    pub fn void(dev: *mut *mut drmres_release_t)(struct drm_device, res: *mut c_void) -> typedef;
}
//
// drmm_add_action - add a managed release action to a &drm_device
// @dev: DRM device
// @action: function which should be called when @dev is released
// @data: opaque pointer, passed to @action
//
// This function adds the release @action with optional parameter @data to the
// list of cleanup actions for @dev. The cleanup actions will be run in reverse
// order in the final drm_dev_put() call for @dev.
//

//
// drmm_add_action_or_reset - add a managed release action to a &drm_device
// @dev: DRM device
// @action: function which should be called when @dev is released
// @data: opaque pointer, passed to @action
//
// Similar to drmm_add_action(), with the only difference that upon failure
// @action is directly called for any cleanup work necessary on failures.
//

//
// drmm_kzalloc - &drm_device managed kzalloc()
// @dev: DRM device
// @size: size of the memory allocation
// @gfp: GFP allocation flags
//
// This is a &drm_device managed version of kzalloc(). The allocated memory is
// automatically freed on the final drm_dev_put(). Memory can also be freed
// before the final drm_dev_put() by calling drmm_kfree().
//
extern "C" {
    pub fn drmm_kmalloc(_arg: dev, _arg: size, __GFP_ZERO: gfp |) -> return;
}
//
// drmm_kmalloc_array - &drm_device managed kmalloc_array()
// @dev: DRM device
// @n: number of array elements to allocate
// @size: size of array member
// @flags: GFP allocation flags
//
// This is a &drm_device managed version of kmalloc_array(). The allocated
// memory is automatically freed on the final drm_dev_put() and works exactly
// like a memory allocation obtained by drmm_kmalloc().
//
extern "C" {
    pub fn drmm_kmalloc(_arg: dev, _arg: bytes, _arg: flags) -> return;
}
//
// drmm_kcalloc - &drm_device managed kcalloc()
// @dev: DRM device
// @n: number of array elements to allocate
// @size: size of array member
// @flags: GFP allocation flags
//
// This is a &drm_device managed version of kcalloc(). The allocated memory is
// automatically freed on the final drm_dev_put() and works exactly like a
// memory allocation obtained by drmm_kmalloc().
//
extern "C" {
    pub fn drmm_kmalloc_array(_arg: dev, _arg: n, _arg: size, __GFP_ZERO: flags |) -> return;
}
extern "C" {
    pub fn drmm_kfree(dev: *mut drm_device, data: *mut c_void);
}
extern "C" {
    pub fn __drmm_mutex_release(dev: *mut drm_device, res: *mut c_void);
}
//
// drmm_mutex_init - &drm_device-managed mutex_init()
// @dev: DRM device
// @lock: lock to be initialized
//
// Returns:
// 0 on success, or a negative errno code otherwise.
//
// This is a &drm_device-managed version of mutex_init(). The initialized
// lock is automatically destroyed on the final drm_dev_put().
//

extern "C" {
    pub fn __drmm_workqueue_release(device: *mut drm_device, wq: *mut c_void);
}
//
// drmm_alloc_ordered_workqueue - &drm_device managed alloc_ordered_workqueue()
// @dev: DRM device
// @fmt: printf format for the name of the workqueue
// @flags: WQ_* flags (only WQ_FREEZABLE and WQ_MEM_RECLAIM are meaningful)
// @args: args for @fmt
//
// This is a &drm_device-managed version of alloc_ordered_workqueue(). The
// allocated workqueue is automatically destroyed on the final drm_dev_put().
//
// Returns: workqueue on success, negative ERR_PTR otherwise.
//

