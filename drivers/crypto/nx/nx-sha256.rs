//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/nx/nx-sha256.c
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
// SHA-256 routines supporting the Power 7+ Nest Accelerators driver
//
// Copyright (C) 2011-2012 International Business Machines Inc.
//
// Author: Kent Yoder <yoder1@us.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha256_state_be {
    pub 4]: __be32 state[SHA256_DIGEST_SIZE /,
    pub count: u64,
}

#[no_mangle]
unsafe extern "C" fn nx_crypto_ctx_sha256_init(tfm: *mut crypto_shash) -> c_int {
    static int nx_crypto_ctx_sha256_init(struct crypto_shash *tfm)
    {
    struct nx_crypto_ctx *nx_ctx = crypto_shash_ctx(tfm);
    int err;
    err = nx_crypto_ctx_sha_init(tfm);
    if (err)
    return err;
    nx_ctx_init(nx_ctx, HCOP_FC_SHA);
    nx_ctx.ap = &nx_ctx.props[NX_PROPS_SHA256];
    NX_CPB_SET_DIGEST_SIZE(nx_ctx.csbcpb, NX_DS_SHA256);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nx_sha256_init(desc: *mut shash_desc) -> c_int {
    static int nx_sha256_init(struct shash_desc *desc)
    {
    struct sha256_state_be *sctx = shash_desc_ctx(desc);
    sctx.state[0] = __cpu_to_be32(SHA256_H0);
    sctx.state[1] = __cpu_to_be32(SHA256_H1);
    sctx.state[2] = __cpu_to_be32(SHA256_H2);
    sctx.state[3] = __cpu_to_be32(SHA256_H3);
    sctx.state[4] = __cpu_to_be32(SHA256_H4);
    sctx.state[5] = __cpu_to_be32(SHA256_H5);
    sctx.state[6] = __cpu_to_be32(SHA256_H6);
    sctx.state[7] = __cpu_to_be32(SHA256_H7);
    sctx.count = 0;
    return 0;
    }
    static int nx_sha256_update(struct shash_desc *desc, const u8 *data,
    unsigned int len)
    {
    struct nx_crypto_ctx *nx_ctx = crypto_shash_ctx(desc.tfm);
    struct sha256_state_be *sctx = shash_desc_ctx(desc);
    struct nx_csbcpb *csbcpb = (struct nx_csbcpb *)nx_ctx.csbcpb;
    u64 to_process, leftover, total = len;
    struct nx_sg *out_sg;
    unsigned long irq_flags;
    let mut rc: c_int = 0;
    int data_len;
    u32 max_sg_len;
    spin_lock_irqsave(&nx_ctx.lock, irq_flags);
    memcpy(csbcpb.cpb.sha256.message_digest, sctx.state, SHA256_DIGEST_SIZE);
    NX_CPB_FDM(csbcpb) |= NX_FDM_INTERMEDIATE;
    NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;
    max_sg_len = min_t(u64, nx_ctx.ap.sglen,
    nx_driver.of.max_sg_len/sizeof(struct nx_sg));
    max_sg_len = min_t(u64, max_sg_len,
    nx_ctx.ap.databytelen/NX_PAGE_SIZE);
    data_len = SHA256_DIGEST_SIZE;
    out_sg = nx_build_sg_list(nx_ctx.out_sg, (u8 *)sctx.state,
    &data_len, max_sg_len);
    nx_ctx.op.outlen = (nx_ctx.out_sg - out_sg) * sizeof(struct nx_sg);
    if (data_len != SHA256_DIGEST_SIZE) {
    rc = -EINVAL;
    goto out;
    }
    do {
    struct nx_sg *in_sg = nx_ctx.in_sg;
    to_process = total & ~(SHA256_BLOCK_SIZE - 1);
    data_len = to_process;
    in_sg = nx_build_sg_list(in_sg, (u8 *) data,
    &data_len, max_sg_len);
    nx_ctx.op.inlen = (nx_ctx.in_sg - in_sg) * sizeof(struct nx_sg);
    to_process = data_len;
    leftover = total - to_process;
//
// we've hit the nx chip previously and we're updating
// again, so copy over the partial digest.
//
    memcpy(csbcpb.cpb.sha256.input_partial_digest,
    csbcpb.cpb.sha256.message_digest,
    SHA256_DIGEST_SIZE);
    if (!nx_ctx.op.inlen || !nx_ctx.op.outlen) {
    rc = -EINVAL;
    goto out;
    }
    rc = nx_hcall_sync(nx_ctx, &nx_ctx.op, 0);
    if (rc)
    goto out;
    atomic_inc(&(nx_ctx.stats.sha256_ops));
    total -= to_process;
    data += to_process;
    sctx.count += to_process;
    } while (leftover >= SHA256_BLOCK_SIZE);
    rc = leftover;
    memcpy(sctx.state, csbcpb.cpb.sha256.message_digest, SHA256_DIGEST_SIZE);
    out:
    spin_unlock_irqrestore(&nx_ctx.lock, irq_flags);
    return rc;
    }
    static int nx_sha256_finup(struct shash_desc *desc, const u8 *src,
    unsigned int nbytes, u8 *out)
    {
    struct nx_crypto_ctx *nx_ctx = crypto_shash_ctx(desc.tfm);
    struct sha256_state_be *sctx = shash_desc_ctx(desc);
    struct nx_csbcpb *csbcpb = (struct nx_csbcpb *)nx_ctx.csbcpb;
    struct nx_sg *in_sg, *out_sg;
    unsigned long irq_flags;
    u32 max_sg_len;
    let mut rc: c_int = 0;
    int len;
    spin_lock_irqsave(&nx_ctx.lock, irq_flags);
    max_sg_len = min_t(u64, nx_ctx.ap.sglen,
    nx_driver.of.max_sg_len/sizeof(struct nx_sg));
    max_sg_len = min_t(u64, max_sg_len,
    nx_ctx.ap.databytelen/NX_PAGE_SIZE);
// final is represented by continuing the operation and indicating that
// this is not an intermediate operation
// copy over the partial digest
    memcpy(csbcpb.cpb.sha256.input_partial_digest, sctx.state, SHA256_DIGEST_SIZE);
    NX_CPB_FDM(csbcpb) &= ~NX_FDM_INTERMEDIATE;
    NX_CPB_FDM(csbcpb) |= NX_FDM_CONTINUATION;
    sctx.count += nbytes;
    csbcpb.cpb.sha256.message_bit_length = (u64) (sctx.count * 8);
    len = nbytes;
    in_sg = nx_build_sg_list(nx_ctx.in_sg, (u8 *)src, &len, max_sg_len);
    if (len != nbytes) {
    rc = -EINVAL;
    goto out;
    }
    len = SHA256_DIGEST_SIZE;
    out_sg = nx_build_sg_list(nx_ctx.out_sg, out, &len, max_sg_len);
    if (len != SHA256_DIGEST_SIZE) {
    rc = -EINVAL;
    goto out;
    }
    nx_ctx.op.inlen = (nx_ctx.in_sg - in_sg) * sizeof(struct nx_sg);
    nx_ctx.op.outlen = (nx_ctx.out_sg - out_sg) * sizeof(struct nx_sg);
    if (!nx_ctx.op.outlen) {
    rc = -EINVAL;
    goto out;
    }
    rc = nx_hcall_sync(nx_ctx, &nx_ctx.op, 0);
    if (rc)
    goto out;
    atomic_inc(&(nx_ctx.stats.sha256_ops));
    atomic64_add(sctx.count, &(nx_ctx.stats.sha256_bytes));
    memcpy(out, csbcpb.cpb.sha256.message_digest, SHA256_DIGEST_SIZE);
    out:
    spin_unlock_irqrestore(&nx_ctx.lock, irq_flags);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn nx_sha256_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int nx_sha256_export(struct shash_desc *desc, void *out)
    {
    struct sha256_state_be *sctx = shash_desc_ctx(desc);
    union {
    u8 *u8;
    u32 *u32;
    u64 *u64;
    } p = { .u8 = out };
    int i;
    for (i = 0; i < SHA256_DIGEST_SIZE / sizeof(*p.u32); i++)
    put_unaligned(be32_to_cpu(sctx.state[i]), p.u32++);
    put_unaligned(sctx.count, p.u64++);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nx_sha256_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int nx_sha256_import(struct shash_desc *desc, const void *in)
    {
    struct sha256_state_be *sctx = shash_desc_ctx(desc);
    union {
    const u8 *u8;
    const u32 *u32;
    const u64 *u64;
    } p = { .u8 = in };
    int i;
    for (i = 0; i < SHA256_DIGEST_SIZE / sizeof(*p.u32); i++)
    sctx.state[i] = cpu_to_be32(get_unaligned(p.u32++));
    sctx.count = get_unaligned(p.u64++);
    return 0;
    }
    struct shash_alg nx_shash_sha256_alg = {
    .digestsize = SHA256_DIGEST_SIZE,
    .init       = nx_sha256_init,
    .update     = nx_sha256_update,
    .finup      = nx_sha256_finup,
    .export     = nx_sha256_export,
    .import     = nx_sha256_import,
    .init_tfm   = nx_crypto_ctx_sha256_init,
    .exit_tfm   = nx_crypto_ctx_shash_exit,
    .descsize   = sizeof(struct sha256_state_be),
    .statesize  = sizeof(struct sha256_state_be),
    .base       = {
    .cra_name        = "sha256",
    .cra_driver_name = "sha256-nx",
    .cra_priority    = 300,
    .cra_flags	 = CRYPTO_AHASH_ALG_BLOCK_ONLY,
    .cra_blocksize   = SHA256_BLOCK_SIZE,
    .cra_module      = THIS_MODULE,
    .cra_ctxsize     = sizeof(struct nx_crypto_ctx),
    }
    };
