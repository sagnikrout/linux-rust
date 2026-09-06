//! Automatically rewritten from C to Rust
//! Source: rust/helpers/drm.c
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

#[no_mangle]
pub unsafe extern "C" fn rust_helper_drm_gem_object_get(obj: *mut drm_gem_object) -> __rust_helper void {
    __rust_helper void rust_helper_drm_gem_object_get(struct drm_gem_object *obj)
    {
    drm_gem_object_get(obj);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_drm_gem_object_put(obj: *mut drm_gem_object) -> __rust_helper void {
    __rust_helper void rust_helper_drm_gem_object_put(struct drm_gem_object *obj)
    {
    drm_gem_object_put(obj);
    }
    __rust_helper __u64
    rust_helper_drm_vma_node_offset_addr(struct drm_vma_offset_node *node)
    {
    return drm_vma_node_offset_addr(node);
    }

    __rust_helper void
    rust_helper_drm_gem_shmem_object_free(struct drm_gem_object *obj)
    {
    return drm_gem_shmem_object_free(obj);
    }
    __rust_helper void
    rust_helper_drm_gem_shmem_object_print_info(struct drm_printer *p, unsigned int indent,
    const struct drm_gem_object *obj)
    {
    drm_gem_shmem_object_print_info(p, indent, obj);
    }
    __rust_helper int
    rust_helper_drm_gem_shmem_object_pin(struct drm_gem_object *obj)
    {
    return drm_gem_shmem_object_pin(obj);
    }
    __rust_helper void
    rust_helper_drm_gem_shmem_object_unpin(struct drm_gem_object *obj)
    {
    drm_gem_shmem_object_unpin(obj);
    }
    __rust_helper struct sg_table *
    rust_helper_drm_gem_shmem_object_get_sg_table(struct drm_gem_object *obj)
    {
    return drm_gem_shmem_object_get_sg_table(obj);
    }
    __rust_helper int
    rust_helper_drm_gem_shmem_object_vmap(struct drm_gem_object *obj,
    struct iosys_map *map)
    {
    return drm_gem_shmem_object_vmap(obj, map);
    }
    __rust_helper void
    rust_helper_drm_gem_shmem_object_vunmap(struct drm_gem_object *obj,
    struct iosys_map *map)
    {
    drm_gem_shmem_object_vunmap(obj, map);
    }
    __rust_helper int
    rust_helper_drm_gem_shmem_object_mmap(struct drm_gem_object *obj, struct vm_area_struct *vma)
    {
    return drm_gem_shmem_object_mmap(obj, vma);
    }

