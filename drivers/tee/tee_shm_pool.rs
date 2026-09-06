//! Automatically rewritten from C to Rust
//! Source: drivers/tee/tee_shm_pool.c
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
// Copyright (c) 2015, 2017, 2022 Linaro Limited
//

    static int pool_op_gen_alloc(struct tee_shm_pool *pool, struct tee_shm *shm,
    size_t size, size_t align)
    {
    unsigned long va;
    struct gen_pool *genpool = pool.private_data;
    let mut a: usize = max_t(size_t, align, BIT(genpool.min_alloc_order));
    let mut data: genpool_data_align = { .align = a };
    let mut s: usize = roundup(size, a);
    va = gen_pool_alloc_algo(genpool, s, gen_pool_first_fit_align, &data);
    if (!va)
    return -ENOMEM;
    memset((void *)va, 0, s);
    shm.kaddr = (void *)va;
    shm.paddr = gen_pool_virt_to_phys(genpool, va);
    shm.size = s;
//
// This is from a static shared memory pool so no need to register
// each chunk, and no need to unregister later either.
//
    shm.flags &= ~TEE_SHM_DYNAMIC;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pool_op_gen_free(pool: *mut tee_shm_pool, shm: *mut tee_shm) {
    static void pool_op_gen_free(struct tee_shm_pool *pool, struct tee_shm *shm)
    {
    gen_pool_free(pool.private_data, (unsigned long)shm.kaddr,
    shm.size);
    shm.kaddr = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pool_op_gen_destroy_pool(pool: *mut tee_shm_pool) {
    static void pool_op_gen_destroy_pool(struct tee_shm_pool *pool)
    {
    gen_pool_destroy(pool.private_data);
    kfree(pool);
    }
    static const struct tee_shm_pool_ops pool_ops_generic = {
    .alloc = pool_op_gen_alloc,
    .free = pool_op_gen_free,
    .destroy_pool = pool_op_gen_destroy_pool,
    };
    struct tee_shm_pool *tee_shm_pool_alloc_res_mem(unsigned long vaddr,
    phys_addr_t paddr, size_t size,
    int min_alloc_order)
    {
    let mut page_mask: usize = PAGE_SIZE - 1;
    struct tee_shm_pool *pool;
    int rc;
// Start and end must be page aligned
    if (vaddr & page_mask || paddr & page_mask || size & page_mask)
    return ERR_PTR(-EINVAL);
    pool = kzalloc_obj(*pool);
    if (!pool)
    return ERR_PTR(-ENOMEM);
    pool.private_data = gen_pool_create(min_alloc_order, -1);
    if (!pool.private_data) {
    rc = -ENOMEM;
    goto err;
    }
    rc = gen_pool_add_virt(pool.private_data, vaddr, paddr, size, -1);
    if (rc) {
    gen_pool_destroy(pool.private_data);
    goto err;
    }
    pool.ops = &pool_ops_generic;
    return pool;
    err:
    kfree(pool);
    return ERR_PTR(rc);
    }
    EXPORT_SYMBOL_GPL(tee_shm_pool_alloc_res_mem);
