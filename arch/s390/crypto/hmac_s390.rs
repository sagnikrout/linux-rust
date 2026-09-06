//! Automatically rewritten from C to Rust
//! Source: arch/s390/crypto/hmac_s390.c
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
// Copyright IBM Corp. 2024
//
// s390 specific HMAC support.
//

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
// cv	| 224/256 | blocksize/2 |   32 |  u64[8]
// | 384/512 |		|   64 | u128[8]
// imbl | 224/256 | blocksize/8 |    8 |     u64
// | 384/512 |		|   16 |    u128
// key	| 224/256 | blocksize	|   64 |  u8[64]
// | 384/512 |		|  128 | u8[128]
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_hmac_ctx {
    pub key: [u8; MAX_BLOCK_SIZE],
}

    union s390_kmac_gr0 {
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
pub struct s390_kmac_sha2_ctx {
    pub MAX_BLOCK_SIZE]: u8 param[MAX_DIGEST_SIZE + MAX_IMBL_SIZE +,
    pub gr0: union s390_kmac_gr0,
    pub buflen: [u64; 2],
}

//
// kmac_sha2_set_imbl - sets the input message bit-length based on the blocksize
//
    static inline void kmac_sha2_set_imbl(u8 *param, u64 buflen_lo,
    u64 buflen_hi, unsigned int blocksize)
    {
    u8 *imbl = param + SHA2_IMBL_OFFSET(blocksize);
    switch (blocksize) {
    case SHA256_BLOCK_SIZE:
// (u64 *)imbl = buflen_lo * BITS_PER_BYTE;
    break;
    case SHA512_BLOCK_SIZE:
// (u128 *)imbl = (((u128)buflen_hi << 64) + buflen_lo) << 3;
    break;
    default:
    break;
    }
    }
    static int hash_data(const u8 *in, unsigned int inlen,
    u8 *digest, unsigned int digestsize, bool final)
    {
    unsigned long func;
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
    pub CPACF_KIMD_SHA_256: func = final ? CPACF_KLMD_SHA_256 :,
    pub 8): *mut *mut PARAM_INIT(256, 224, inlen,
    if (!final)
    pub SHA256_DIGEST_SIZE: digestsize =,
    case SHA256_DIGEST_SIZE:
    pub CPACF_KIMD_SHA_256: func = final ? CPACF_KLMD_SHA_256 :,
    pub 8): *mut *mut PARAM_INIT(256, 256, inlen,
    case SHA384_DIGEST_SIZE:
    pub CPACF_KIMD_SHA_512: func = final ? CPACF_KLMD_SHA_512 :,
    pub 8): *mut *mut PARAM_INIT(512, 384, inlen,
    if (!final)
    pub SHA512_DIGEST_SIZE: digestsize =,
    case SHA512_DIGEST_SIZE:
    pub CPACF_KIMD_SHA_512: func = final ? CPACF_KLMD_SHA_512 :,
    pub 8): *mut *mut PARAM_INIT(512, 512, inlen,
    default:
    pub -EINVAL: return,
    }

    pub inlen): cpacf_klmd(func, &param, in,,
    pub digestsize): memcpy(digest, &param,,
    pub 0: return,
    }
    static int hash_key(const u8 *in, unsigned int inlen,
    u8 *digest, unsigned int digestsize)
    {
    pub true): return hash_data(in, inlen, digest, digestsize,,
    }
    static int s390_hmac_sha2_setkey(struct crypto_shash *tfm,
    const u8 *key, unsigned int keylen)
    {
    pub crypto_shash_ctx(tfm): *mut *mut s390_hmac_ctx tfm_ctx =,
    pub crypto_shash_digestsize(tfm): unsigned int ds =,
    pub crypto_shash_blocksize(tfm): unsigned int bs =,
    pub sizeof(*tfm_ctx)): *mut memset(tfm_ctx, 0,,
    if (keylen > bs)
    pub ds): return hash_key(key, keylen, tfm_ctx->key,,
    pub keylen): memcpy(tfm_ctx->key, key,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn s390_hmac_sha2_init(desc: *mut shash_desc) -> c_int {
    static int s390_hmac_sha2_init(struct shash_desc *desc)
    {
    pub crypto_shash_ctx(desc->tfm): *mut *mut s390_hmac_ctx tfm_ctx =,
    pub shash_desc_ctx(desc): *mut *mut s390_kmac_sha2_ctx ctx =,
    pub crypto_shash_blocksize(desc->tfm): unsigned int bs =,
    memcpy(ctx.param + SHA2_KEY_OFFSET(bs),
    pub bs): tfm_ctx->key,,
    pub 0: ctx->buflen[0] =,
    pub 0: ctx->buflen[1] =,
    pub 0: ctx->gr0.reg =,
    switch (crypto_shash_digestsize(desc.tfm)) {
    case SHA224_DIGEST_SIZE:
    pub CPACF_KMAC_HMAC_SHA_224: ctx->gr0.fc =,
    case SHA256_DIGEST_SIZE:
    pub CPACF_KMAC_HMAC_SHA_256: ctx->gr0.fc =,
    case SHA384_DIGEST_SIZE:
    pub CPACF_KMAC_HMAC_SHA_384: ctx->gr0.fc =,
    case SHA512_DIGEST_SIZE:
    pub CPACF_KMAC_HMAC_SHA_512: ctx->gr0.fc =,
    default:
    pub -EINVAL: return,
    }
    pub 0: return,
    }
    static int s390_hmac_sha2_update(struct shash_desc *desc,
    const u8 *data, unsigned int len)
    {
    pub shash_desc_ctx(desc): *mut *mut s390_kmac_sha2_ctx ctx =,
    pub crypto_shash_blocksize(desc->tfm): unsigned int bs =,
    pub bs): unsigned int n = round_down(len,,
    pub n: ctx->buflen[0] +=,
    if (ctx.buflen[0] < n)
// process as many blocks as possible
    pub 1: ctx->gr0.iimp =,
    pub n): _cpacf_kmac(&ctx->gr0.reg, ctx->param, data,,
    pub n: return len -,
    }
    static int s390_hmac_sha2_finup(struct shash_desc *desc, const u8 *src,
    unsigned int len, u8 *out)
    {
    pub shash_desc_ctx(desc): *mut *mut s390_kmac_sha2_ctx ctx =,
    pub crypto_shash_blocksize(desc->tfm): unsigned int bs =,
    pub len: ctx->buflen[0] +=,
    if (ctx.buflen[0] < len)
    pub 0: ctx->gr0.iimp =,
    pub bs): kmac_sha2_set_imbl(ctx->param, ctx->buflen[0], ctx->buflen[1],,
    pub len): _cpacf_kmac(&ctx->gr0.reg, ctx->param, src,,
    pub crypto_shash_digestsize(desc->tfm)): memcpy(out, ctx->param,,
    pub 0: return,
    }
    static int s390_hmac_sha2_digest(struct shash_desc *desc,
    const u8 *data, unsigned int len, u8 *out)
    {
    pub shash_desc_ctx(desc): *mut *mut s390_kmac_sha2_ctx ctx =,
    pub crypto_shash_digestsize(desc->tfm): unsigned int ds =,
    pub rc: c_int,
    pub s390_hmac_sha2_init(desc): rc =,
    if (rc)
    pub rc: return,
    pub 0: ctx->gr0.iimp =,
    kmac_sha2_set_imbl(ctx.param, len, 0,
    pub len): _cpacf_kmac(&ctx->gr0.reg, ctx->param, data,,
    pub ds): memcpy(out, ctx->param,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn s390_hmac_export_zero(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int s390_hmac_export_zero(struct shash_desc *desc, void *out)
    {
    pub desc->tfm: *mut *mut crypto_shash tfm =,
    pub ipad: [u8; SHA512_BLOCK_SIZE],
    pub ctx: *mut s390_hmac_ctx,
    pub bs: c_uint,
    pub i: int err,,
    pub crypto_shash_ctx(tfm): ctx =,
    pub crypto_shash_blocksize(tfm): bs =,
    pub i++): for (i = 0; i < bs;,
    pub HMAC_IPAD_VALUE: ipad[i] = ctx->key[i] ^,
    pub false): err = hash_data(ipad, bs, out, crypto_shash_digestsize(tfm),,
    pub sizeof(ipad)): memzero_explicit(ipad,,
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn s390_hmac_export(desc: *mut shash_desc, out: *mut c_void) -> c_int {
    static int s390_hmac_export(struct shash_desc *desc, void *out)
    {
    pub shash_desc_ctx(desc): *mut *mut s390_kmac_sha2_ctx ctx =,
    pub crypto_shash_blocksize(desc->tfm): unsigned int bs =,
    pub 2: unsigned int ds = bs /,
    pub ctx->buflen[0]: u64 lo =,
    union {
    pub u8: *mut u8,
    pub u64: *mut u64,
    pub }: } p = { .u8 = out,
    pub 0: int err =,
    if (!ctx.gr0.ikp)
    pub out): err = s390_hmac_export_zero(desc,,
    else
    pub ds): memcpy(p.u8, ctx->param,,
    pub ds: p.u8 +=,
    pub bs: lo +=,
    pub p.u64++): put_unaligned(lo,,
    if (ds == SHA512_DIGEST_SIZE)
    pub p.u64): put_unaligned(ctx->buflen[1] + (lo < bs),,
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn s390_hmac_import(desc: *mut shash_desc, in: *const c_void) -> c_int {
    static int s390_hmac_import(struct shash_desc *desc, const void *in)
    {
    pub shash_desc_ctx(desc): *mut *mut s390_kmac_sha2_ctx ctx =,
    pub crypto_shash_blocksize(desc->tfm): unsigned int bs =,
    pub 2: unsigned int ds = bs /,
    union {
    pub u8: *const u8,
    pub u64: *const u64,
    pub }: } p = { .u8 = in,
    pub lo: u64,
    pub err: c_int,
    pub s390_hmac_sha2_init(desc): err =,
    pub ds): memcpy(ctx->param, p.u8,,
    pub ds: p.u8 +=,
    pub get_unaligned(p.u64++): lo =,
    pub bs: ctx->buflen[0] = lo -,
    if (ds == SHA512_DIGEST_SIZE)
    pub bs): ctx->buflen[1] = get_unaligned(p.u64) - (lo <,
    if (ctx.buflen[0] | ctx.buflen[1])
    pub 1: ctx->gr0.ikp =,
    pub err: return,
    }

    .fc = CPACF_KMAC_HMAC_SHA_##x,					\
    .alg = {							\
    .init = s390_hmac_sha2_init,				\
    .update = s390_hmac_sha2_update,			\
    .finup = s390_hmac_sha2_finup,				\
    .digest = s390_hmac_sha2_digest,			\
    .setkey = s390_hmac_sha2_setkey,			\
    .export = s390_hmac_export,				\
    .import = s390_hmac_import,				\
    .descsize = sizeof(struct s390_kmac_sha2_ctx),		\
    .halg = {						\
    .statesize = ss,				\
    .digestsize = SHA##x##_DIGEST_SIZE,		\
    .base = {					\
    .cra_name = "hmac(sha" #x ")",		\
    .cra_driver_name = "hmac_s390_sha" #x,	\
    .cra_blocksize = SHA##x##_BLOCK_SIZE,	\
    .cra_priority = 400,			\
    .cra_flags = CRYPTO_AHASH_ALG_BLOCK_ONLY | \
    CRYPTO_AHASH_ALG_FINUP_MAX, \
    .cra_ctxsize = sizeof(struct s390_hmac_ctx), \
    .cra_module = THIS_MODULE,		\
    },						\
    },							\
    },								\
    }
    static struct s390_hmac_alg {
    pub registered: bool,
    pub fc: c_uint,
    pub alg: shash_alg,
    } s390_hmac_algs[] = {
    S390_HMAC_SHA2_ALG(224, sizeof(struct crypto_sha256_state)),
    S390_HMAC_SHA2_ALG(256, sizeof(struct crypto_sha256_state)),
    S390_HMAC_SHA2_ALG(384, SHA512_STATE_SIZE),
    S390_HMAC_SHA2_ALG(512, SHA512_STATE_SIZE),
}

#[no_mangle]
unsafe extern "C" fn _s390_hmac_algs_unregister() -> __always_inline void {
    static __always_inline void _s390_hmac_algs_unregister(void)
    {
    struct s390_hmac_alg *hmac;
    int i;
    for (i = ARRAY_SIZE(s390_hmac_algs) - 1; i >= 0; i--) {
    hmac = &s390_hmac_algs[i];
    if (!hmac.registered)
    continue;
    crypto_unregister_shash(&hmac.alg);
    }
    }
#[no_mangle]
unsafe extern "C" fn hmac_s390_init() -> int __init {
    static int __init hmac_s390_init(void)
    {
    struct s390_hmac_alg *hmac;
    int i, rc = -ENODEV;
    if (!cpacf_query_func(CPACF_KLMD, CPACF_KLMD_SHA_256))
    return -ENODEV;
    if (!cpacf_query_func(CPACF_KLMD, CPACF_KLMD_SHA_512))
    return -ENODEV;
    for (i = 0; i < ARRAY_SIZE(s390_hmac_algs); i++) {
    hmac = &s390_hmac_algs[i];
    if (!cpacf_query_func(CPACF_KMAC, hmac.fc))
    continue;
    rc = crypto_register_shash(&hmac.alg);
    if (rc) {
    pr_err("unable to register %s\n",
    hmac.alg.halg.base.cra_name);
    goto out;
    }
    hmac.registered = true;
    pr_debug("registered %s\n", hmac.alg.halg.base.cra_name);
    }
    return rc;
    out:
    _s390_hmac_algs_unregister();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn hmac_s390_exit() -> void __exit {
    static void __exit hmac_s390_exit(void)
    {
    _s390_hmac_algs_unregister();
    }
    module_cpu_feature_match(S390_CPU_FEATURE_MSA, hmac_s390_init);
    module_exit(hmac_s390_exit);
    MODULE_DESCRIPTION("S390 HMAC driver");
    MODULE_LICENSE("GPL");
