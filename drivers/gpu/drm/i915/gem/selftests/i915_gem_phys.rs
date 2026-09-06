//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gem/selftests/i915_gem_phys.c
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


//
// SPDX-License-Identifier: MIT
//
// Copyright © 2016 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn mock_phys_object(arg: *mut c_void) -> c_int {
    static int mock_phys_object(void *arg)
    {
    struct drm_i915_private *i915 = arg;
    struct drm_i915_gem_object *obj;
    int err;
// Create an object and bind it to a contiguous set of physical pages,
// i.e. exercise the i915_gem_object_phys API.
//
    obj = i915_gem_object_create_shmem(i915, PAGE_SIZE);
    if (IS_ERR(obj)) {
    err = PTR_ERR(obj);
    pr_err("i915_gem_object_create failed, err=%d\n", err);
    goto out;
    }
    i915_gem_object_lock(obj, core::ptr::null_mut());
    if (!i915_gem_object_has_struct_page(obj)) {
    i915_gem_object_unlock(obj);
    err = -EINVAL;
    pr_err("shmem has no struct page\n");
    goto out_obj;
    }
    err = i915_gem_object_attach_phys(obj, PAGE_SIZE);
    i915_gem_object_unlock(obj);
    if (err) {
    pr_err("i915_gem_object_attach_phys failed, err=%d\n", err);
    goto out_obj;
    }
    if (i915_gem_object_has_struct_page(obj)) {
    pr_err("i915_gem_object_attach_phys did not create a phys object\n");
    err = -EINVAL;
    goto out_obj;
    }
    if (!atomic_read(&obj.mm.pages_pin_count)) {
    pr_err("i915_gem_object_attach_phys did not pin its phys pages\n");
    err = -EINVAL;
    goto out_obj;
    }
// Make the object dirty so that put_pages must do copy back the data
    i915_gem_object_lock(obj, core::ptr::null_mut());
    err = i915_gem_object_set_to_gtt_domain(obj, true);
    i915_gem_object_unlock(obj);
    if (err) {
    pr_err("i915_gem_object_set_to_gtt_domain failed with err=%d\n",
    err);
    goto out_obj;
    }
    out_obj:
    i915_gem_object_put(obj);
    out:
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn i915_gem_phys_mock_selftests() -> c_int {
    int i915_gem_phys_mock_selftests(void)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(mock_phys_object),
    };
    struct drm_i915_private *i915;
    int err;
    i915 = mock_gem_device();
    if (!i915)
    return -ENOMEM;
    err = i915_subtests(tests, i915);
    mock_destroy_device(i915);
    return err;
    }
