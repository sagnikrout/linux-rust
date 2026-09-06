//! Automatically rewritten from C to Rust
//! Source: drivers/tee/qcomtee/shm.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// define MAX_OUTBOUND_BUFFER_SIZE - Maximum size of outbound buffers.
//
// The size of outbound buffer depends on QTEE callback requests.
//

//
// define MAX_INBOUND_BUFFER_SIZE - Maximum size of the inbound buffer.
//
// The size of the inbound buffer depends on the user's requests,
// specifically the number of IB and OB arguments. If an invocation
// requires a size larger than %MAX_INBOUND_BUFFER_SIZE, the user should
// consider using another form of shared memory with QTEE.
//

//
// qcomtee_msg_buffers_alloc() - Allocate inbound and outbound buffers.
// @oic: context to use for the current invocation.
// @u: array of arguments for the current invocation.
//
// It calculates the size of inbound and outbound buffers based on the
// arguments in @u. It allocates the buffers from the teedev pool.
//
// Return: On success, returns 0. On error, returns < 0.
//
    int qcomtee_msg_buffers_alloc(struct qcomtee_object_invoke_ctx *oic,
    struct qcomtee_arg *u)
    {
    struct tee_context *ctx = oic.ctx;
    struct tee_shm *shm;
    size_t size;
    int i;
// Start offset in a message for buffer arguments.
    size = qcomtee_msg_buffer_args(struct qcomtee_msg_object_invoke,
    qcomtee_args_len(u));
    if (size > MAX_INBOUND_BUFFER_SIZE)
    return -EINVAL;
// Add size of IB arguments.
    qcomtee_arg_for_each_input_buffer(i, u) {
    size = size_add(size, qcomtee_msg_offset_align(u[i].b.size));
    if (size > MAX_INBOUND_BUFFER_SIZE)
    return -EINVAL;
    }
// Add size of OB arguments.
    qcomtee_arg_for_each_output_buffer(i, u) {
    size = size_add(size, qcomtee_msg_offset_align(u[i].b.size));
    if (size > MAX_INBOUND_BUFFER_SIZE)
    return -EINVAL;
    }
    shm = tee_shm_alloc_priv_buf(ctx, size);
    if (IS_ERR(shm))
    return PTR_ERR(shm);
// Allocate inbound buffer.
    oic.in_shm = shm;
    shm = tee_shm_alloc_priv_buf(ctx, MAX_OUTBOUND_BUFFER_SIZE);
    if (IS_ERR(shm)) {
    tee_shm_free(oic.in_shm);
    return PTR_ERR(shm);
    }
// Allocate outbound buffer.
    oic.out_shm = shm;
    oic.in_msg.addr = tee_shm_get_va(oic.in_shm, 0);
    oic.in_msg.size = tee_shm_get_size(oic.in_shm);
    oic.out_msg.addr = tee_shm_get_va(oic.out_shm, 0);
    oic.out_msg.size = tee_shm_get_size(oic.out_shm);
// QTEE assume unused buffers are zeroed.
    memzero_explicit(oic.in_msg.addr, oic.in_msg.size);
    memzero_explicit(oic.out_msg.addr, oic.out_msg.size);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn qcomtee_msg_buffers_free(oic: *mut qcomtee_object_invoke_ctx) {
    void qcomtee_msg_buffers_free(struct qcomtee_object_invoke_ctx *oic)
    {
    tee_shm_free(oic.in_shm);
    tee_shm_free(oic.out_shm);
    }
// Dynamic shared memory pool based on tee_dyn_shm_alloc_helper().
    static int qcomtee_shm_register(struct tee_context *ctx, struct tee_shm *shm,
    struct page **pages, size_t num_pages,
    unsigned long start)
    {
    return qcom_tzmem_shm_bridge_create(shm.paddr, shm.size,
    &shm.sec_world_id);
    }
#[no_mangle]
unsafe extern "C" fn qcomtee_shm_unregister(ctx: *mut tee_context, shm: *mut tee_shm) -> c_int {
    static int qcomtee_shm_unregister(struct tee_context *ctx, struct tee_shm *shm)
    {
    qcom_tzmem_shm_bridge_delete(shm.sec_world_id);
    return 0;
    }
    static int pool_op_alloc(struct tee_shm_pool *pool, struct tee_shm *shm,
    size_t size, size_t align)
    {
    return tee_dyn_shm_alloc_helper(shm, size, align, qcomtee_shm_register);
    }
#[no_mangle]
unsafe extern "C" fn pool_op_free(pool: *mut tee_shm_pool, shm: *mut tee_shm) {
    static void pool_op_free(struct tee_shm_pool *pool, struct tee_shm *shm)
    {
    tee_dyn_shm_free_helper(shm, qcomtee_shm_unregister);
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
    struct tee_shm_pool *qcomtee_shm_pool_alloc(void)
    {
    struct tee_shm_pool *pool;
    pool = kzalloc_obj(*pool);
    if (!pool)
    return ERR_PTR(-ENOMEM);
    pool.ops = &pool_ops;
    return pool;
    }
