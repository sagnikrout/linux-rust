//! Automatically rewritten from C to Rust
//! Source: crypto/deflate.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Cryptographic API.
//
// Deflate algorithm (RFC 1951), implemented here primarily for use
// by IPCOMP (RFC 3173 & RFC 2394).
//
// Copyright (c) 2003 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2023 Google, LLC. <ardb@kernel.org>
// Copyright (c) 2025 Herbert Xu <herbert@gondor.apana.org.au>
//

pub const DEFLATE_DEF_WINBITS: c_int = 11;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct deflate_stream {
    pub stream: z_stream_s,
    pub workspace: [u8; ],
}

    static DEFINE_MUTEX(deflate_stream_lock);
    static void *deflate_alloc_stream(void)
    {
    size_t size = max(zlib_inflate_workspacesize(),
    zlib_deflate_workspacesize(-DEFLATE_DEF_WINBITS,
    DEFLATE_DEF_MEMLEVEL));
    struct deflate_stream *ctx;
    ctx = kvmalloc_flex(*ctx, workspace, size);
    if (!ctx)
    return ERR_PTR(-ENOMEM);
    ctx.stream.workspace = ctx.workspace;
    return ctx;
    }
#[no_mangle]
unsafe extern "C" fn deflate_free_stream(ctx: *mut c_void) {
    static void deflate_free_stream(void *ctx)
    {
    kvfree(ctx);
    }
    static struct crypto_acomp_streams deflate_streams = {
    .alloc_ctx = deflate_alloc_stream,
    .free_ctx = deflate_free_stream,
    };
    static int deflate_compress_one(struct acomp_req *req,
    struct deflate_stream *ds)
    {
    struct z_stream_s *stream = &ds.stream;
    struct acomp_walk walk;
    int ret;
    ret = acomp_walk_virt(&walk, req, true);
    if (ret)
    return ret;
    do {
    unsigned int dcur;
    dcur = acomp_walk_next_dst(&walk);
    if (!dcur)
    return -ENOSPC;
    stream.avail_out = dcur;
    stream.next_out = walk.dst.virt.addr;
    do {
    let mut flush: c_int = Z_FINISH;
    unsigned int scur;
    stream.avail_in = 0;
    stream.next_in = core::ptr::null_mut();
    scur = acomp_walk_next_src(&walk);
    if (scur) {
    if (acomp_walk_more_src(&walk, scur))
    flush = Z_NO_FLUSH;
    stream.avail_in = scur;
    stream.next_in = walk.src.virt.addr;
    }
    ret = zlib_deflate(stream, flush);
    if (scur) {
    scur -= stream.avail_in;
    acomp_walk_done_src(&walk, scur);
    }
    } while (ret == Z_OK && stream.avail_out);
    acomp_walk_done_dst(&walk, dcur);
    } while (ret == Z_OK);
    if (ret != Z_STREAM_END)
    return -EINVAL;
    req.dlen = stream.total_out;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn deflate_compress(req: *mut acomp_req) -> c_int {
    static int deflate_compress(struct acomp_req *req)
    {
    struct crypto_acomp_stream *s;
    struct deflate_stream *ds;
    int err;
    s = crypto_acomp_lock_stream_bh(&deflate_streams);
    ds = s.ctx;
    err = zlib_deflateInit2(&ds.stream, DEFLATE_DEF_LEVEL, Z_DEFLATED,
    -DEFLATE_DEF_WINBITS, DEFLATE_DEF_MEMLEVEL,
    Z_DEFAULT_STRATEGY);
    if (err != Z_OK) {
    err = -EINVAL;
    goto out;
    }
    err = deflate_compress_one(req, ds);
    out:
    crypto_acomp_unlock_stream_bh(s);
    return err;
    }
    static int deflate_decompress_one(struct acomp_req *req,
    struct deflate_stream *ds)
    {
    struct z_stream_s *stream = &ds.stream;
    let mut out_of_space: bool = false;
    struct acomp_walk walk;
    int ret;
    ret = acomp_walk_virt(&walk, req, true);
    if (ret)
    return ret;
    do {
    unsigned int scur;
    stream.avail_in = 0;
    stream.next_in = core::ptr::null_mut();
    scur = acomp_walk_next_src(&walk);
    if (scur) {
    stream.avail_in = scur;
    stream.next_in = walk.src.virt.addr;
    }
    do {
    unsigned int dcur;
    unsigned long avail_in;
    dcur = acomp_walk_next_dst(&walk);
    stream.avail_out = dcur;
    stream.next_out = walk.dst.virt.addr;
    avail_in = stream.avail_in;
    ret = zlib_inflate(stream, Z_NO_FLUSH);
    if (!dcur && avail_in == stream.avail_in) {
    out_of_space = true;
    break;
    }
    dcur -= stream.avail_out;
    acomp_walk_done_dst(&walk, dcur);
    } while (ret == Z_OK && stream.avail_in);
    if (scur)
    acomp_walk_done_src(&walk, scur);
    if (out_of_space)
    return -ENOSPC;
    } while (ret == Z_OK);
    if (ret != Z_STREAM_END)
    return -EINVAL;
    req.dlen = stream.total_out;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn deflate_decompress(req: *mut acomp_req) -> c_int {
    static int deflate_decompress(struct acomp_req *req)
    {
    struct crypto_acomp_stream *s;
    struct deflate_stream *ds;
    int err;
    s = crypto_acomp_lock_stream_bh(&deflate_streams);
    ds = s.ctx;
    err = zlib_inflateInit2(&ds.stream, -DEFLATE_DEF_WINBITS);
    if (err != Z_OK) {
    err = -EINVAL;
    goto out;
    }
    err = deflate_decompress_one(req, ds);
    out:
    crypto_acomp_unlock_stream_bh(s);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn deflate_init(tfm: *mut crypto_acomp) -> c_int {
    static int deflate_init(struct crypto_acomp *tfm)
    {
    int ret;
    mutex_lock(&deflate_stream_lock);
    ret = crypto_acomp_alloc_streams(&deflate_streams);
    mutex_unlock(&deflate_stream_lock);
    return ret;
    }
    static struct acomp_alg acomp = {
    .compress		= deflate_compress,
    .decompress		= deflate_decompress,
    .init			= deflate_init,
    .base.cra_name		= "deflate",
    .base.cra_driver_name	= "deflate-generic",
    .base.cra_flags		= CRYPTO_ALG_REQ_VIRT,
    .base.cra_module	= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn deflate_mod_init() -> int __init {
    static int __init deflate_mod_init(void)
    {
    return crypto_register_acomp(&acomp);
    }
#[no_mangle]
unsafe extern "C" fn deflate_mod_fini() -> void __exit {
    static void __exit deflate_mod_fini(void)
    {
    crypto_unregister_acomp(&acomp);
    crypto_acomp_free_streams(&deflate_streams);
    }
    module_init(deflate_mod_init);
    module_exit(deflate_mod_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Deflate Compression Algorithm for IPCOMP");
    MODULE_AUTHOR("James Morris <jmorris@intercode.com.au>");
    MODULE_AUTHOR("Ard Biesheuvel <ardb@kernel.org>");
    MODULE_AUTHOR("Herbert Xu <herbert@gondor.apana.org.au>");
    MODULE_ALIAS_CRYPTO("deflate");
    MODULE_ALIAS_CRYPTO("deflate-generic");
