//! Automatically rewritten from C to Rust
//! Source: arch/s390/crypto/phmac_s390.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright IBM Corp. 2025
//
// s390 specific HMAC support for protected keys.
//

    static struct crypto_engine *phmac_crypto_engine;
pub const MAX_QLEN: c_int = 10;
    static bool pkey_clrkey_allowed;
    module_param_named(clrkey, pkey_clrkey_allowed, bool, 0444);
    MODULE_PARM_DESC(clrkey, "Allow clear key material (default N)");
//
// A simple hash walk helper
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_walk_helper {
    pub walk: crypto_hash_walk,
    pub walkaddr: *const u8,
    pub walkbytes: c_int,
}

//
// Prepare hash walk helper.
// Set up the base hash walk, fill walkaddr and walkbytes.
// Returns 0 on success or negative value on error.
//
    static inline int hwh_prepare(struct ahash_request *req,
    struct hash_walk_helper *hwh)
    {
    hwh.walkbytes = crypto_hash_walk_first(req, &hwh.walk);
    if (hwh.walkbytes < 0)
    return hwh.walkbytes;
    hwh.walkaddr = hwh.walk.data;
    return 0;
    }
//
// Advance hash walk helper by n bytes.
// Progress the walkbytes and walkaddr fields by n bytes.
// If walkbytes is then 0, pull next hunk from hash walk
// and update walkbytes and walkaddr.
// If n is negative, unmap hash walk and return error.
// Returns 0 on success or negative value on error.
//
#[no_mangle]
pub unsafe extern "C" fn hwh_advance(hwh: *mut hash_walk_helper, n: c_int) -> c_int {
    static inline int hwh_advance(struct hash_walk_helper *hwh, int n)
    {
    if (n < 0)
    return crypto_hash_walk_done(&hwh.walk, n);
    hwh.walkbytes -= n;
    hwh.walkaddr += n;
    if (hwh.walkbytes > 0)
    return 0;
    hwh.walkbytes = crypto_hash_walk_done(&hwh.walk, 0);
    if (hwh.walkbytes < 0)
    return hwh.walkbytes;
    hwh.walkaddr = hwh.walk.data;
    return 0;
    }
//
// KMAC param block layout for sha2 function codes:
// The layout of the param block for the KMAC instruction depends on the
// blocksize of the used hashing sha2-algorithm function codes. The param block
// contains the hash chaining value (cv), the input message bit-length (imbl)
// and the hmac-secret (key). To prevent code duplication, the sizes of all
// these are calculated based on the blocksize.
//
// param-block:
// +-------+
// | cv    |
// +-------+
// | imbl  |
// +-------+
// | key   |
// +-------+
//
// sizes:
// part | sh2-alg | calculation | size | type
// -----+---------+-------------+------+--------
// cv   | 224/256 | blocksize/2 |   32 |  u64[8]
// | 384/512 |             |   64 | u128[8]
// imbl | 224/256 | blocksize/8 |    8 |     u64
// | 384/512 |             |   16 |    u128
// key  | 224/256 | blocksize   |   96 |  u8[96]
// | 384/512 |             |  160 | u8[160]
//

pub const PHMAC_MAX_KEYSIZE: c_int = 256;

// phmac protected key struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phmac_protkey {
    pub type: u32,
    pub len: u32,
    pub protkey: [u8; PHMAC_MAX_PK_SIZE],
}

pub const PK_STATE_NO_KEY: c_int = 0;
pub const PK_STATE_CONVERT_IN_PROGRESS: c_int = 1;
pub const PK_STATE_VALID: c_int = 2;
// phmac tfm context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phmac_tfm_ctx {
// source key material used to derive a protected key from
    pub keybuf: [u8; PHMAC_MAX_KEYSIZE],
    pub keylen: c_uint,
// cpacf function code to use with this protected key type
    pub fc: c_long,
// nr of requests enqueued via crypto engine which use this tfm ctx
    pub via_engine_ctr: core::sync::atomic::AtomicI32,
// spinlock to atomic read/update all the following fields
    pub pk_lock: spinlock_t,
// see PK_STATE* defines above, < 0 holds convert failure rc
    pub pk_state: c_int,
// if state is valid, pk holds the protected key
    pub pk: phmac_protkey,
}

    union kmac_gr0 {
    unsigned long reg;
    struct {
    unsigned long		: 48;
    unsigned long ikp	:  1;
    unsigned long iimp	:  1;
    unsigned long ccup	:  1;
    unsigned long		:  6;
    unsigned long fc	:  7;
    };
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmac_sha2_ctx {
    pub PHMAC_MAX_PK_SIZE]: u8 param[MAX_DIGEST_SIZE + MAX_IMBL_SIZE +,
    pub gr0: union kmac_gr0,
    pub buf: [u8; MAX_BLOCK_SIZE],
    pub buflen: [u64; 2],
}

    enum async_op {
    OP_NOP = 0,
    OP_UPDATE,
    OP_FINAL,
    OP_FINUP,
    };
// phmac request context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phmac_req_ctx {
    pub hwh: hash_walk_helper,
    pub kmac_ctx: kmac_sha2_ctx,
    pub async_op: enum async_op,
}

//
// Pkey 'token' struct used to derive a protected key value from a clear key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_clrkey_token {
    pub type: u8,
    pub res0: [u8; 3],
    pub version: u8,
    pub res1: [u8; 3],
    pub keytype: u32,
    pub len: u32,
    pub key: [u8; ],
    pub __packed: },
    static int hash_key(const u8 *in, unsigned int inlen,
    u8 *digest, unsigned int digestsize)
    {
    pub func: c_ulong,
    union {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha256_paramblock {
    pub h: [u32; 8],
    pub mbl: u64,
    pub sha256: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sha512_paramblock {
    pub h: [u64; 8],
    pub mbl: u128,
    pub sha512: },
    pub param: } __packed,

    pub \: param.sha##x.h[0] = SHA##y ## _H0;,
    pub \: param.sha##x.h[1] = SHA##y ## _H1;,
    pub \: param.sha##x.h[2] = SHA##y ## _H2;,
    pub \: param.sha##x.h[3] = SHA##y ## _H3;,
    pub \: param.sha##x.h[4] = SHA##y ## _H4;,
    pub \: param.sha##x.h[5] = SHA##y ## _H5;,
    pub \: param.sha##x.h[6] = SHA##y ## _H6;,
    pub \: param.sha##x.h[7] = SHA##y ## _H7;,
    param.sha##x.mbl = (z)
    switch (digestsize) {
    case SHA224_DIGEST_SIZE:
    pub CPACF_KLMD_SHA_256: func =,
    pub 8): *mut *mut PARAM_INIT(256, 224, inlen,
    case SHA256_DIGEST_SIZE:
    pub CPACF_KLMD_SHA_256: func =,
    pub 8): *mut *mut PARAM_INIT(256, 256, inlen,
    case SHA384_DIGEST_SIZE:
    pub CPACF_KLMD_SHA_512: func =,
    pub 8): *mut *mut PARAM_INIT(512, 384, inlen,
    case SHA512_DIGEST_SIZE:
    pub CPACF_KLMD_SHA_512: func =,
    pub 8): *mut *mut PARAM_INIT(512, 512, inlen,
    default:
    pub -EINVAL: return,
    }

    pub inlen): cpacf_klmd(func, &param, in,,
    pub digestsize): memcpy(digest, &param,,
    pub 0: return,
    }
//
// make_clrkey_token() - wrap the clear key into a pkey clearkey token.
//
    static inline int make_clrkey_token(const u8 *clrkey, size_t clrkeylen,
    unsigned int digestsize, u8 *dest)
    {
    pub )dest: *mut *mut hmac_clrkey_token token = (hmac_clrkey_token,
    pub blocksize: c_uint,
    pub rc: c_int,
    pub 0x00: token->type =,
    pub 0x02: token->version =,
    switch (digestsize) {
    case SHA224_DIGEST_SIZE:
    case SHA256_DIGEST_SIZE:
    pub PKEY_KEYTYPE_HMAC_512: token->keytype =,
    pub 64: blocksize =,
    case SHA384_DIGEST_SIZE:
    case SHA512_DIGEST_SIZE:
    pub PKEY_KEYTYPE_HMAC_1024: token->keytype =,
    pub 128: blocksize =,
    default:
    pub -EINVAL: return,
    }
    pub blocksize: token->len =,
    if (clrkeylen > blocksize) {
    pub digestsize): rc = hash_key(clrkey, clrkeylen, token->key,,
    if (rc)
    pub rc: return,
    } else {
    pub clrkeylen): memcpy(token->key, clrkey,,
    }
    pub 0: return,
    }
//
// phmac_tfm_ctx_setkey() - Set key value into tfm context, maybe construct
// a clear key token digestible by pkey from a clear key value.
//
    static inline int phmac_tfm_ctx_setkey(struct phmac_tfm_ctx *tfm_ctx,
    const u8 *key, unsigned int keylen)
    {
    if (keylen > sizeof(tfm_ctx.keybuf))
    pub -EINVAL: return,
    pub keylen): memcpy(tfm_ctx->keybuf, key,,
    pub keylen: tfm_ctx->keylen =,
    pub 0: return,
    }
//
// Convert the raw key material into a protected key via PKEY api.
// This function may sleep - don't call in non-sleeping context.
//
    static inline int convert_key(const u8 *key, unsigned int keylen,
    struct phmac_protkey *pk, bool tested)
    {
    pub PKEY_XFLAG_NOMEMALLOC: u32 xflags =,
    pub i: int rc,,
    if (tested && !pkey_clrkey_allowed)
    pub PKEY_XFLAG_NOCLEARKEY: xflags |=,
    pub sizeof(pk->protkey): pk->len =,
//
// In case of a busy card retry with increasing delay
// of 200, 400, 800 and 1600 ms - in total 3 s.
//
    pub {: for (rc = -EIO, i = 0; rc && i < 5; i++),
    if (rc == -EBUSY && msleep_interruptible((1 << i) * 100)) {
    pub -EINTR: rc =,
    pub out: goto,
    }
    rc = pkey_key2protkey(key, keylen,
    pk.protkey, &pk.len, &pk.type,
    }
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
//
// (Re-)Convert the raw key material from the tfm ctx into a protected
// key via convert_key() function. Update the pk_state, pk_type, pk_len
// and the protected key in the tfm context.
// Please note this function may be invoked concurrently with the very
// same tfm context. The pk_lock spinlock in the context ensures an
// atomic update of the pk and the pk state but does not guarantee any
// order of update. So a fresh converted valid protected key may get
// updated with an 'old' expired key value. As the cpacf instructions
// detect this, refuse to operate with an invalid key and the calling
// code triggers a (re-)conversion this does no harm. This may lead to
// unnecessary additional conversion but never to invalid data on the
// hash operation.
//
#[no_mangle]
unsafe extern "C" fn phmac_convert_key(tfm_ctx: *mut phmac_tfm_ctx, tested: bool) -> c_int {
    static int phmac_convert_key(struct phmac_tfm_ctx *tfm_ctx, bool tested)
    {
    pub pk: phmac_protkey,
    pub rc: c_int,
    pub PK_STATE_CONVERT_IN_PROGRESS: tfm_ctx->pk_state =,
    pub tested): rc = convert_key(tfm_ctx->keybuf, tfm_ctx->keylen, &pk,,
// update context
    if (rc) {
    pub rc: tfm_ctx->pk_state =,
    } else {
    pub PK_STATE_VALID: tfm_ctx->pk_state =,
    pub pk: tfm_ctx->pk =,
    }
    pub sizeof(pk)): memzero_explicit(&pk,,
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
//
// kmac_sha2_set_imbl - sets the input message bit-length based on the blocksize
//
    static inline void kmac_sha2_set_imbl(u8 *param, u64 buflen_lo,
    u64 buflen_hi, unsigned int blocksize)
    {
    pub SHA2_IMBL_OFFSET(blocksize): *mut *mut u8 imbl = param +,
    switch (blocksize) {
    case SHA256_BLOCK_SIZE:
// (u64 *)imbl = buflen_lo * BITS_PER_BYTE;
    case SHA512_BLOCK_SIZE:
// (u128 *)imbl = (((u128)buflen_hi << 64) + buflen_lo) << 3;
    default:
    }
    }
#[no_mangle]
unsafe extern "C" fn phmac_kmac_update(req: *mut ahash_request, maysleep: bool) -> c_int {
    static int phmac_kmac_update(struct ahash_request *req, bool maysleep)
    {
    pub crypto_ahash_reqtfm(req): *mut *mut crypto_ahash tfm =,
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx ctx =,
    pub &req_ctx->hwh: *mut *mut hash_walk_helper hwh =,
    pub crypto_ahash_blocksize(tfm): unsigned int bs =,
    pub crypto_ahash_tested(tfm): bool tested =,
    pub n: unsigned int offset, k,,
    pub 0: int rc =,
//
// The walk is always mapped when this function is called.
// Note that in case of partial processing or failure the walk
// is NOT unmapped here. So a follow up task may reuse the walk
// or in case of unrecoverable failure needs to unmap it.
//
    while (hwh.walkbytes > 0) {
// check sha2 context buffer
    pub bs: offset = ctx->buflen[0] %,
    if (offset + hwh.walkbytes < bs)
    pub store: goto,
    if (offset) {
// fill ctx buffer up to blocksize and process this block
    pub offset: n = bs -,
    pub n): memcpy(ctx->buf + offset, hwh->walkaddr,,
    pub 1: ctx->gr0.iimp =,
    pub {: for (;;),
    pub bs): k = _cpacf_kmac(&ctx->gr0.reg, ctx->param, ctx->buf,,
    if (likely(k == bs))
    if (unlikely(k > 0)) {
//
// Can't deal with hunks smaller than blocksize.
// And kmac should always return the nr of
// processed bytes as 0 or a multiple of the
// blocksize.
//
    pub -EIO: rc =,
    pub out: goto,
    }
// protected key is invalid and needs re-conversion
    if (!maysleep) {
    pub -EKEYEXPIRED: rc =,
    pub out: goto,
    }
    pub tested): rc = phmac_convert_key(tfm_ctx,,
    if (rc)
    pub out: goto,
    memcpy(ctx.param + SHA2_KEY_OFFSET(bs),
    pub tfm_ctx->pk.len): tfm_ctx->pk.protkey,,
    }
    pub n: ctx->buflen[0] +=,
    if (ctx.buflen[0] < n)
    pub n): rc = hwh_advance(hwh,,
    if (unlikely(rc))
    pub out: goto,
    pub 0: offset =,
    }
// process as many blocks as possible from the walk
    while (hwh.walkbytes >= bs) {
    pub bs: *mut *mut n = (hwh->walkbytes / bs),
    pub 1: ctx->gr0.iimp =,
    pub n): k = _cpacf_kmac(&ctx->gr0.reg, ctx->param, hwh->walkaddr,,
    if (likely(k > 0)) {
    pub k: ctx->buflen[0] +=,
    if (ctx.buflen[0] < k)
    pub k): rc = hwh_advance(hwh,,
    if (unlikely(rc))
    pub out: goto,
    }
    if (unlikely(k < n)) {
// protected key is invalid and needs re-conversion
    if (!maysleep) {
    pub -EKEYEXPIRED: rc =,
    pub out: goto,
    }
    pub tested): rc = phmac_convert_key(tfm_ctx,,
    if (rc)
    pub out: goto,
    memcpy(ctx.param + SHA2_KEY_OFFSET(bs),
    pub tfm_ctx->pk.len): tfm_ctx->pk.protkey,,
    }
    }
    store:
// store incomplete block in context buffer
    if (hwh.walkbytes) {
    pub hwh->walkbytes): memcpy(ctx->buf + offset, hwh->walkaddr,,
    pub hwh->walkbytes: ctx->buflen[0] +=,
    if (ctx.buflen[0] < hwh.walkbytes)
    pub hwh->walkbytes): rc = hwh_advance(hwh,,
    if (unlikely(rc))
    pub out: goto,
    }
    } /* end of while (hwh.walkbytes > 0) */
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_kmac_final(req: *mut ahash_request, maysleep: bool) -> c_int {
    static int phmac_kmac_final(struct ahash_request *req, bool maysleep)
    {
    pub crypto_ahash_reqtfm(req): *mut *mut crypto_ahash tfm =,
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx ctx =,
    pub crypto_ahash_digestsize(tfm): unsigned int ds =,
    pub crypto_ahash_blocksize(tfm): unsigned int bs =,
    pub crypto_ahash_tested(tfm): bool tested =,
    pub n: unsigned int k,,
    pub 0: int rc =,
    pub bs: n = ctx->buflen[0] %,
    pub 0: ctx->gr0.iimp =,
    pub bs): kmac_sha2_set_imbl(ctx->param, ctx->buflen[0], ctx->buflen[1],,
    pub {: for (;;),
    pub n): k = _cpacf_kmac(&ctx->gr0.reg, ctx->param, ctx->buf,,
    if (likely(k == n))
    if (unlikely(k > 0)) {
// Can't deal with hunks smaller than blocksize.
    pub -EIO: rc =,
    pub out: goto,
    }
// protected key is invalid and needs re-conversion
    if (!maysleep) {
    pub -EKEYEXPIRED: rc =,
    pub out: goto,
    }
    pub tested): rc = phmac_convert_key(tfm_ctx,,
    if (rc)
    pub out: goto,
    memcpy(ctx.param + SHA2_KEY_OFFSET(bs),
    pub tfm_ctx->pk.len): tfm_ctx->pk.protkey,,
    }
    pub ds): memcpy(req->result, ctx->param,,
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_init(req: *mut ahash_request) -> c_int {
    static int phmac_init(struct ahash_request *req)
    {
    pub crypto_ahash_reqtfm(req): *mut *mut crypto_ahash tfm =,
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx kmac_ctx =,
    pub crypto_ahash_blocksize(tfm): unsigned int bs =,
    pub 0: int rc =,
// zero request context (includes the kmac sha2 context)
    pub sizeof(*req_ctx)): *mut memset(req_ctx, 0,,
//
// setkey() should have set a valid fc into the tfm context.
// Copy this function code into the gr0 field of the kmac context.
//
    if (!tfm_ctx.fc) {
    pub -ENOKEY: rc =,
    pub out: goto,
    }
    pub tfm_ctx->fc: kmac_ctx->gr0.fc =,
//
// Copy the pk from tfm ctx into kmac ctx. The protected key
// may be outdated but update() and final() will handle this.
//
    memcpy(kmac_ctx.param + SHA2_KEY_OFFSET(bs),
    pub tfm_ctx->pk.len): tfm_ctx->pk.protkey,,
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_update(req: *mut ahash_request) -> c_int {
    static int phmac_update(struct ahash_request *req)
    {
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub crypto_ahash_reqtfm(req): *mut *mut crypto_ahash tfm =,
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx kmac_ctx =,
    pub &req_ctx->hwh: *mut *mut hash_walk_helper hwh =,
    pub rc: c_int,
// prep the walk in the request context
    pub hwh): rc = hwh_prepare(req,,
    if (rc)
    pub out: goto,
// Try synchronous operation if no active engine usage
    if (!atomic_read(&tfm_ctx.via_engine_ctr)) {
    pub false): rc = phmac_kmac_update(req,,
    if (rc == 0)
    pub out: goto,
    }
//
// If sync operation failed or key expired or there are already
// requests enqueued via engine, fallback to async. Mark tfm as
// using engine to serialize requests.
//
    if (rc == 0 || rc == -EKEYEXPIRED) {
    pub OP_UPDATE: req_ctx->async_op =,
    pub req): rc = crypto_transfer_hash_request_to_engine(phmac_crypto_engine,,
    if (rc != -EINPROGRESS)
    }
    if (rc != -EINPROGRESS) {
    pub rc): hwh_advance(hwh,,
    pub sizeof(*kmac_ctx)): *mut memzero_explicit(kmac_ctx,,
    }
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_final(req: *mut ahash_request) -> c_int {
    static int phmac_final(struct ahash_request *req)
    {
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub crypto_ahash_reqtfm(req): *mut *mut crypto_ahash tfm =,
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx kmac_ctx =,
    pub 0: int rc =,
// Try synchronous operation if no active engine usage
    if (!atomic_read(&tfm_ctx.via_engine_ctr)) {
    pub false): rc = phmac_kmac_final(req,,
    if (rc == 0)
    pub out: goto,
    }
//
// If sync operation failed or key expired or there are already
// requests enqueued via engine, fallback to async. Mark tfm as
// using engine to serialize requests.
//
    if (rc == 0 || rc == -EKEYEXPIRED) {
    pub OP_FINAL: req_ctx->async_op =,
    pub req): rc = crypto_transfer_hash_request_to_engine(phmac_crypto_engine,,
    if (rc != -EINPROGRESS)
    }
    out:
    if (rc != -EINPROGRESS)
    pub sizeof(*kmac_ctx)): *mut memzero_explicit(kmac_ctx,,
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_finup(req: *mut ahash_request) -> c_int {
    static int phmac_finup(struct ahash_request *req)
    {
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub crypto_ahash_reqtfm(req): *mut *mut crypto_ahash tfm =,
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx kmac_ctx =,
    pub &req_ctx->hwh: *mut *mut hash_walk_helper hwh =,
    pub rc: c_int,
// prep the walk in the request context
    pub hwh): rc = hwh_prepare(req,,
    if (rc)
    pub out: goto,
    pub OP_FINUP: req_ctx->async_op =,
// Try synchronous operations if no active engine usage
    if (!atomic_read(&tfm_ctx.via_engine_ctr)) {
    pub false): rc = phmac_kmac_update(req,,
    if (rc == 0)
    pub OP_FINAL: req_ctx->async_op =,
    }
    if (!rc && req_ctx.async_op == OP_FINAL &&
    !atomic_read(&tfm_ctx.via_engine_ctr)) {
    pub false): rc = phmac_kmac_final(req,,
    if (rc == 0)
    pub out: goto,
    }
//
// If sync operation failed or key expired or there are already
// requests enqueued via engine, fallback to async. Mark tfm as
// using engine to serialize requests.
//
    if (rc == 0 || rc == -EKEYEXPIRED) {
// req->async_op has been set to either OP_FINUP or OP_FINAL
    pub req): rc = crypto_transfer_hash_request_to_engine(phmac_crypto_engine,,
    if (rc != -EINPROGRESS)
    }
    if (rc != -EINPROGRESS)
    pub rc): hwh_advance(hwh,,
    out:
    if (rc != -EINPROGRESS)
    pub sizeof(*kmac_ctx)): *mut memzero_explicit(kmac_ctx,,
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_digest(req: *mut ahash_request) -> c_int {
    static int phmac_digest(struct ahash_request *req)
    {
    pub rc: c_int,
    pub phmac_init(req): rc =,
    if (rc)
    pub out: goto,
    pub phmac_finup(req): rc =,
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
    static int phmac_setkey(struct crypto_ahash *tfm,
    const u8 *key, unsigned int keylen)
    {
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub crypto_ahash_digestsize(tfm): unsigned int ds =,
    pub crypto_ahash_blocksize(tfm): unsigned int bs =,
    pub crypto_ahash_tested(tfm): bool tested =,
    pub tmpkeylen: c_uint,
    pub NULL: *mut *mut u8 tmpkey =,
    pub 0: int rc =,
    if (!tested) {
//
// selftest running: key is a raw hmac clear key and needs
// to get embedded into a 'clear key token' in order to have
// it correctly processed by the pkey module.
//
    pub bs: tmpkeylen = sizeof(struct hmac_clrkey_token) +,
    pub GFP_KERNEL): tmpkey = kzalloc(tmpkeylen,,
    if (!tmpkey) {
    pub -ENOMEM: rc =,
    pub out: goto,
    }
    pub tmpkey): rc = make_clrkey_token(key, keylen, ds,,
    if (rc)
    pub out: goto,
    pub tmpkeylen: keylen =,
    pub tmpkey: key =,
    }
// copy raw key into tfm context
    pub keylen): rc = phmac_tfm_ctx_setkey(tfm_ctx, key,,
    if (rc)
    pub out: goto,
// convert raw key into protected key
    pub tested): rc = phmac_convert_key(tfm_ctx,,
    if (rc)
    pub out: goto,
// set function code in tfm context, check for valid pk type
    switch (ds) {
    case SHA224_DIGEST_SIZE:
    if (tfm_ctx.pk.type != PKEY_KEYTYPE_HMAC_512)
    pub -EINVAL: rc =,
    else
    pub CPACF_KMAC_PHMAC_SHA_224: tfm_ctx->fc =,
    case SHA256_DIGEST_SIZE:
    if (tfm_ctx.pk.type != PKEY_KEYTYPE_HMAC_512)
    pub -EINVAL: rc =,
    else
    pub CPACF_KMAC_PHMAC_SHA_256: tfm_ctx->fc =,
    case SHA384_DIGEST_SIZE:
    if (tfm_ctx.pk.type != PKEY_KEYTYPE_HMAC_1024)
    pub -EINVAL: rc =,
    else
    pub CPACF_KMAC_PHMAC_SHA_384: tfm_ctx->fc =,
    case SHA512_DIGEST_SIZE:
    if (tfm_ctx.pk.type != PKEY_KEYTYPE_HMAC_1024)
    pub -EINVAL: rc =,
    else
    pub CPACF_KMAC_PHMAC_SHA_512: tfm_ctx->fc =,
    default:
    pub 0: tfm_ctx->fc =,
    pub -EINVAL: rc =,
    }
    out:
    pub rc): pr_debug("rc=%d\n",,
    pub rc: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_export(req: *mut ahash_request, out: *mut c_void) -> c_int {
    static int phmac_export(struct ahash_request *req, void *out)
    {
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx ctx =,
    pub sizeof(*ctx)): *mut memcpy(out, ctx,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_import(req: *mut ahash_request, in: *const c_void) -> c_int {
    static int phmac_import(struct ahash_request *req, const void *in)
    {
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx ctx =,
    pub sizeof(*req_ctx)): *mut memset(req_ctx, 0,,
    pub sizeof(*ctx)): *mut memcpy(ctx, in,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_init_tfm(tfm: *mut crypto_ahash) -> c_int {
    static int phmac_init_tfm(struct crypto_ahash *tfm)
    {
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub sizeof(*tfm_ctx)): *mut memset(tfm_ctx, 0,,
    pub phmac_req_ctx)): crypto_ahash_set_reqsize(tfm, sizeof(struct,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn phmac_exit_tfm(tfm: *mut crypto_ahash) {
    static void phmac_exit_tfm(struct crypto_ahash *tfm)
    {
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub sizeof(tfm_ctx->keybuf)): memzero_explicit(tfm_ctx->keybuf,,
    pub sizeof(tfm_ctx->pk)): memzero_explicit(&tfm_ctx->pk,,
    }
#[no_mangle]
unsafe extern "C" fn phmac_do_one_request(engine: *mut crypto_engine, areq: *mut c_void) -> c_int {
    static int phmac_do_one_request(struct crypto_engine *engine, void *areq)
    {
    pub ahash_request_cast(areq): *mut *mut ahash_request req =,
    pub crypto_ahash_reqtfm(req): *mut *mut crypto_ahash tfm =,
    pub crypto_ahash_ctx(tfm): *mut *mut phmac_tfm_ctx tfm_ctx =,
    pub ahash_request_ctx(req): *mut *mut phmac_req_ctx req_ctx =,
    pub &req_ctx->kmac_ctx: *mut *mut kmac_sha2_ctx kmac_ctx =,
    pub &req_ctx->hwh: *mut *mut hash_walk_helper hwh =,
    pub -EINVAL: int rc =,
//
// Three kinds of requests come in here:
// 1. req->async_op == OP_UPDATE with req->nbytes > 0
// 2. req->async_op == OP_FINUP with req->nbytes > 0
// 3. req->async_op == OP_FINAL
// For update and finup the hwh walk has already been prepared
// by the caller. For final there is no hwh walk needed.
//
    switch (req_ctx.async_op) {
    case OP_UPDATE:
    case OP_FINUP:
    pub true): rc = phmac_kmac_update(req,,
    if (rc == -EKEYEXPIRED) {
    pub pkey_handle_expired(): return,
    } else if (rc) {
    pub rc): hwh_advance(hwh,,
    pub out: goto,
    }
    if (req_ctx.async_op == OP_UPDATE)
    pub OP_FINAL: req_ctx->async_op =,
    case OP_FINAL:
    pub true): rc = phmac_kmac_final(req,,
    if (rc == -EKEYEXPIRED)
    pub pkey_handle_expired(): return,
    default:
// unknown/unsupported/unimplemented asynch op
    pub -EOPNOTSUPP: return,
    }
    out:
    if (rc || req_ctx.async_op == OP_FINAL)
    pub sizeof(*kmac_ctx)): *mut memzero_explicit(kmac_ctx,,
    pub rc): pr_debug("request complete with rc=%d\n",,
    pub rc): crypto_finalize_hash_request(engine, req,,
    pub rc: return,
    }

    {									\
    .base = {							\
    .init	  = phmac_init,					\
    .update	  = phmac_update,				\
    .final	  = phmac_final,				\
    .finup	  = phmac_finup,				\
    .digest	  = phmac_digest,				\
    .setkey	  = phmac_setkey,				\
    .import	  = phmac_import,				\
    .export	  = phmac_export,				\
    .init_tfm = phmac_init_tfm,				\
    .exit_tfm = phmac_exit_tfm,				\
    .halg = {						\
    .digestsize = SHA##x##_DIGEST_SIZE,		\
    .statesize  = sizeof(struct kmac_sha2_ctx),	\
    .base = {					\
    .cra_name = "phmac(sha" #x ")",		\
    .cra_driver_name = "phmac_s390_sha" #x,	\
    .cra_blocksize = SHA##x##_BLOCK_SIZE,	\
    .cra_priority = 400,			\
    .cra_flags = CRYPTO_ALG_ASYNC |		\
    CRYPTO_ALG_NO_FALLBACK,	\
    .cra_ctxsize = sizeof(struct phmac_tfm_ctx), \
    .cra_module = THIS_MODULE,		\
    },						\
    },							\
    },								\
    .op = {								\
    .do_one_request = phmac_do_one_request,			\
    },								\
    }
    static struct phmac_alg {
    pub fc: c_uint,
    pub alg: ahash_engine_alg,
    pub registered: bool,
    } phmac_algs[] = {
    {
    .fc = CPACF_KMAC_PHMAC_SHA_224,
    .alg = S390_ASYNC_PHMAC_ALG(224),
    }, {
    .fc = CPACF_KMAC_PHMAC_SHA_256,
    .alg = S390_ASYNC_PHMAC_ALG(256),
    }, {
    .fc = CPACF_KMAC_PHMAC_SHA_384,
    .alg = S390_ASYNC_PHMAC_ALG(384),
    }, {
    .fc = CPACF_KMAC_PHMAC_SHA_512,
    .alg = S390_ASYNC_PHMAC_ALG(512),
    }
}

    static struct miscdevice phmac_dev = {
    .name	= "phmac",
    .minor	= MISC_DYNAMIC_MINOR,
    };
#[no_mangle]
unsafe extern "C" fn s390_phmac_exit() {
    static void s390_phmac_exit(void)
    {
    struct phmac_alg *phmac;
    int i;
    if (phmac_crypto_engine) {
    crypto_engine_stop(phmac_crypto_engine);
    crypto_engine_exit(phmac_crypto_engine);
    }
    for (i = ARRAY_SIZE(phmac_algs) - 1; i >= 0; i--) {
    phmac = &phmac_algs[i];
    if (phmac.registered)
    crypto_engine_unregister_ahash(&phmac.alg);
    }
    misc_deregister(&phmac_dev);
    }
#[no_mangle]
unsafe extern "C" fn s390_phmac_init() -> int __init {
    static int __init s390_phmac_init(void)
    {
    struct phmac_alg *phmac;
    int i, rc;
// for selftest cpacf klmd subfunction is needed
    if (!cpacf_query_func(CPACF_KLMD, CPACF_KLMD_SHA_256))
    return -ENODEV;
    if (!cpacf_query_func(CPACF_KLMD, CPACF_KLMD_SHA_512))
    return -ENODEV;
// register a simple phmac pseudo misc device
    rc = misc_register(&phmac_dev);
    if (rc)
    return rc;
// with this pseudo device alloc and start a crypto engine
    phmac_crypto_engine =
    crypto_engine_alloc_init_and_set(phmac_dev.this_device,
    true, false, MAX_QLEN);
    if (!phmac_crypto_engine) {
    rc = -ENOMEM;
    goto out_err;
    }
    rc = crypto_engine_start(phmac_crypto_engine);
    if (rc) {
    crypto_engine_exit(phmac_crypto_engine);
    phmac_crypto_engine = core::ptr::null_mut();
    goto out_err;
    }
    for (i = 0; i < ARRAY_SIZE(phmac_algs); i++) {
    phmac = &phmac_algs[i];
    if (!cpacf_query_func(CPACF_KMAC, phmac.fc))
    continue;
    rc = crypto_engine_register_ahash(&phmac.alg);
    if (rc)
    goto out_err;
    phmac.registered = true;
    pr_debug("%s registered\n", phmac.alg.base.halg.base.cra_name);
    }
    return 0;
    out_err:
    s390_phmac_exit();
    return rc;
    }
    module_init(s390_phmac_init);
    module_exit(s390_phmac_exit);
    MODULE_ALIAS_CRYPTO("phmac(sha224)");
    MODULE_ALIAS_CRYPTO("phmac(sha256)");
    MODULE_ALIAS_CRYPTO("phmac(sha384)");
    MODULE_ALIAS_CRYPTO("phmac(sha512)");
    MODULE_DESCRIPTION("S390 HMAC driver for protected keys");
    MODULE_LICENSE("GPL");
