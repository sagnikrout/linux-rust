//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/etnaviv/etnaviv_gem_prime.c
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
// Copyright (C) 2014-2018 Etnaviv Project
//

    MODULE_IMPORT_NS("DMA_BUF");
    static struct lock_class_key etnaviv_prime_lock_class;
    struct sg_table *etnaviv_gem_prime_get_sg_table(struct drm_gem_object *obj)
    {
    struct etnaviv_gem_object *etnaviv_obj = to_etnaviv_bo(obj);
    let mut npages: c_uint = obj.size >> PAGE_SHIFT;
    if (WARN_ON(!etnaviv_obj.pages))  /* should have already pinned! */
    return ERR_PTR(-EINVAL);
    return drm_prime_pages_to_sg(obj.dev, etnaviv_obj.pages, npages);
    }
#[no_mangle]
pub unsafe extern "C" fn etnaviv_gem_prime_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int {
    int etnaviv_gem_prime_vmap(struct drm_gem_object *obj, struct iosys_map *map)
    {
    void *vaddr;
    vaddr = etnaviv_gem_vmap(obj);
    if (!vaddr)
    return -ENOMEM;
    iosys_map_set_vaddr(map, vaddr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn etnaviv_gem_prime_pin(obj: *mut drm_gem_object) -> c_int {
    int etnaviv_gem_prime_pin(struct drm_gem_object *obj)
    {
    if (!drm_gem_is_imported(obj)) {
    struct etnaviv_gem_object *etnaviv_obj = to_etnaviv_bo(obj);
    mutex_lock(&etnaviv_obj.lock);
    etnaviv_gem_get_pages(etnaviv_obj);
    mutex_unlock(&etnaviv_obj.lock);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn etnaviv_gem_prime_unpin(obj: *mut drm_gem_object) {
    void etnaviv_gem_prime_unpin(struct drm_gem_object *obj)
    {
    if (!drm_gem_is_imported(obj)) {
    struct etnaviv_gem_object *etnaviv_obj = to_etnaviv_bo(obj);
    mutex_lock(&etnaviv_obj.lock);
    etnaviv_gem_put_pages(to_etnaviv_bo(obj));
    mutex_unlock(&etnaviv_obj.lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn etnaviv_gem_prime_release(etnaviv_obj: *mut etnaviv_gem_object) {
    static void etnaviv_gem_prime_release(struct etnaviv_gem_object *etnaviv_obj)
    {
    let mut map: iosys_map = IOSYS_MAP_INIT_VADDR(etnaviv_obj.vaddr);
    if (etnaviv_obj.vaddr)
    dma_buf_vunmap_unlocked(etnaviv_obj.base.import_attach.dmabuf, &map);
// Don't drop the pages for imported dmabuf, as they are not
// ours, just free the array we allocated:
//
    kvfree(etnaviv_obj.pages);
    drm_prime_gem_destroy(&etnaviv_obj.base, etnaviv_obj.sgt);
    }
    static void *etnaviv_gem_prime_vmap_impl(struct etnaviv_gem_object *etnaviv_obj)
    {
    struct iosys_map map;
    int ret;
    lockdep_assert_held(&etnaviv_obj.lock);
    ret = dma_buf_vmap(etnaviv_obj.base.import_attach.dmabuf, &map);
    if (ret)
    return core::ptr::null_mut();
    return map.vaddr;
    }
    static int etnaviv_gem_prime_mmap_obj(struct etnaviv_gem_object *etnaviv_obj,
    struct vm_area_struct *vma)
    {
    int ret;
    ret = dma_buf_mmap(etnaviv_obj.base.dma_buf, vma, 0);
    if (!ret) {
// Drop the reference acquired by drm_gem_mmap_obj().
    drm_gem_object_put(&etnaviv_obj.base);
    }
    return ret;
    }
    static const struct etnaviv_gem_ops etnaviv_gem_prime_ops = {
// .get_pages should never be called
    .release = etnaviv_gem_prime_release,
    .vmap = etnaviv_gem_prime_vmap_impl,
    .mmap = etnaviv_gem_prime_mmap_obj,
    };
    struct drm_gem_object *etnaviv_gem_prime_import_sg_table(struct drm_device *dev,
    struct dma_buf_attachment *attach, struct sg_table *sgt)
    {
    struct etnaviv_gem_object *etnaviv_obj;
    let mut size: usize = PAGE_ALIGN(attach.dmabuf.size);
    int ret, npages;
    ret = etnaviv_gem_new_private(dev, size, ETNA_BO_WC,
    &etnaviv_gem_prime_ops, &etnaviv_obj);
    if (ret < 0)
    return ERR_PTR(ret);
    lockdep_set_class(&etnaviv_obj.lock, &etnaviv_prime_lock_class);
    npages = size / PAGE_SIZE;
    etnaviv_obj.sgt = sgt;
    etnaviv_obj.pages = kvmalloc_objs(struct page *, npages);
    if (!etnaviv_obj.pages) {
    ret = -ENOMEM;
    goto fail;
    }
    ret = drm_prime_sg_to_page_array(sgt, etnaviv_obj.pages, npages);
    if (ret)
    goto fail;
    etnaviv_gem_obj_add(dev, &etnaviv_obj.base);
    return &etnaviv_obj.base;
    fail:
    drm_gem_object_put(&etnaviv_obj.base);
    return ERR_PTR(ret);
    }
