//! Automatically rewritten from C to Rust
//! Source: drivers/tee/amdtee/shm_pool.c
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


// SPDX-License-Identifier: MIT
//
// Copyright 2019 Advanced Micro Devices, Inc.
//

    static int pool_op_alloc(struct tee_shm_pool *pool, struct tee_shm *shm,
    size_t size, size_t align)
    {
    let mut order: c_uint = get_order(size);
    unsigned long va;
    int rc;
//
// Ignore alignment since this is already going to be page aligned
// and there's no need for any larger alignment.
//
    va = __get_free_pages(GFP_KERNEL | __GFP_ZERO, order);
    if (!va)
    return -ENOMEM;
    shm.kaddr = (void *)va;
    shm.paddr = __psp_pa((void *)va);
    shm.size = PAGE_SIZE << order;
// Map the allocated memory in to TEE
    rc = amdtee_map_shmem(shm);
    if (rc) {
    free_pages(va, order);
    shm.kaddr = core::ptr::null_mut();
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pool_op_free(pool: *mut tee_shm_pool, shm: *mut tee_shm) {
    static void pool_op_free(struct tee_shm_pool *pool, struct tee_shm *shm)
    {
// Unmap the shared memory from TEE
    amdtee_unmap_shmem(shm);
    free_pages((unsigned long)shm.kaddr, get_order(shm.size));
    shm.kaddr = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pool_op_destroy_pool(pool: *mut tee_shm_pool) {
    static void pool_op_destroy_pool(struct tee_shm_pool *pool)
    {
    kfree(pool);
    }
    static const struct tee_shm_pool_ops pool_ops = {
    .alloc = pool_op_alloc,
    .free = pool_op_free,
    .destroy_pool = pool_op_destroy_pool,
    };
    struct tee_shm_pool *amdtee_config_shm(void)
    {
    struct tee_shm_pool *pool = kzalloc_obj(*pool);
    if (!pool)
    return ERR_PTR(-ENOMEM);
    pool.ops = &pool_ops;
    return pool;
    }
