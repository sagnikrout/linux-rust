//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/ttm_sys_manager.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT

    static int ttm_sys_man_alloc(struct ttm_resource_manager *man,
    struct ttm_buffer_object *bo,
    const struct ttm_place *place,
    struct ttm_resource **res)
    {
// res = kzalloc_obj(**res);
    if (!*res)
    return -ENOMEM;
    ttm_resource_init(bo, place, *res);
    return 0;
    }
    static void ttm_sys_man_free(struct ttm_resource_manager *man,
    struct ttm_resource *res)
    {
    ttm_resource_fini(man, res);
    kfree(res);
    }
    static const struct ttm_resource_manager_func ttm_sys_manager_func = {
    .alloc = ttm_sys_man_alloc,
    .free = ttm_sys_man_free,
    };
#[no_mangle]
pub unsafe extern "C" fn ttm_sys_man_init(bdev: *mut ttm_device) {
    void ttm_sys_man_init(struct ttm_device *bdev)
    {
    struct ttm_resource_manager *man = &bdev.sysman;
//
// Initialize the system memory buffer type.
// Other types need to be driver / IOCTL initialized.
//
    man.use_tt = true;
    man.func = &ttm_sys_manager_func;
    ttm_resource_manager_init(man, bdev, 0);
    ttm_set_driver_manager(bdev, TTM_PL_SYSTEM, man);
    ttm_resource_manager_set_used(man, true);
    }
