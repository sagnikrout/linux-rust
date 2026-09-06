//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/qxl/qxl_prime.c
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
// Copyright 2014 Canonical
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Andreas Pokorny
//

// Empty Implementations as there should not be any other driver for a virtual
// device that might share buffers with qxl
#[no_mangle]
pub unsafe extern "C" fn qxl_gem_prime_pin(obj: *mut drm_gem_object) -> c_int {
    int qxl_gem_prime_pin(struct drm_gem_object *obj)
    {
    struct qxl_bo *bo = gem_to_qxl_bo(obj);
    return qxl_bo_pin_locked(bo);
    }
#[no_mangle]
pub unsafe extern "C" fn qxl_gem_prime_unpin(obj: *mut drm_gem_object) {
    void qxl_gem_prime_unpin(struct drm_gem_object *obj)
    {
    struct qxl_bo *bo = gem_to_qxl_bo(obj);
    qxl_bo_unpin_locked(bo);
    }
    struct sg_table *qxl_gem_prime_get_sg_table(struct drm_gem_object *obj)
    {
    return ERR_PTR(-ENOSYS);
    }
    struct drm_gem_object *qxl_gem_prime_import_sg_table(
    struct drm_device *dev, struct dma_buf_attachment *attach,
    struct sg_table *table)
    {
    return ERR_PTR(-ENOSYS);
    }
#[no_mangle]
pub unsafe extern "C" fn qxl_gem_prime_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int {
    int qxl_gem_prime_vmap(struct drm_gem_object *obj, struct iosys_map *map)
    {
    struct qxl_bo *bo = gem_to_qxl_bo(obj);
    int ret;
    ret = qxl_bo_vmap_locked(bo, map);
    if (ret < 0)
    return ret;
    return 0;
    }
    void qxl_gem_prime_vunmap(struct drm_gem_object *obj,
    struct iosys_map *map)
    {
    struct qxl_bo *bo = gem_to_qxl_bo(obj);
    qxl_bo_vunmap_locked(bo);
    }
