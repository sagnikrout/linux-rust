//! Automatically rewritten from C to Rust
//! Source: arch/s390/crypto/aes_s390.c
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
// Cryptographic API.
//
// s390 implementation of the AES Cipher Algorithm.
//
// s390 Version:
// Copyright IBM Corp. 2005, 2017
// Author(s): Jan Glauber (jang@de.ibm.com)
// Sebastian Siewior (sebastian@breakpoint.cc> SW-Fallback
// Patrick Steuer <patrick.steuer@de.ibm.com>
// Harald Freudenberger <freude@de.ibm.com>
//
// Derived from "crypto/aes_generic.c"
//

    static u8 *ctrblk;
    static DEFINE_MUTEX(ctrblk_lock);
    static cpacf_mask_t km_functions, kmc_functions, kmctr_functions,
    kma_functions;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_aes_ctx {
    pub key: [u8; AES_MAX_KEY_SIZE],
    pub key_len: c_int,
    pub fc: c_ulong,
    union {
    pub skcipher: *mut crypto_skcipher,
    pub fallback: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_xts_ctx {
    union {
    pub keys: [u8; 64],
    struct {
    pub key: [u8; 32],
    pub pcc_key: [u8; 32],
}

    };
    int key_len;
    unsigned long fc;
    struct crypto_skcipher *fallback;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcm_sg_walk {
    pub walk: scatter_walk,
    pub walk_bytes: c_uint,
    pub walk_bytes_remain: c_uint,
    pub buf: [u8; AES_BLOCK_SIZE],
    pub buf_bytes: c_uint,
    pub ptr: *mut u8,
    pub nbytes: c_uint,
}

    static int setkey_fallback_skcipher(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int len)
    {
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    crypto_skcipher_clear_flags(sctx.fallback.skcipher,
    CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(sctx.fallback.skcipher,
    crypto_skcipher_get_flags(tfm) &
    CRYPTO_TFM_REQ_MASK);
    return crypto_skcipher_setkey(sctx.fallback.skcipher, key, len);
    }
    static int fallback_skcipher_crypt(struct s390_aes_ctx *sctx,
    struct skcipher_request *req,
    unsigned long modifier)
    {
    struct skcipher_request *subreq = skcipher_request_ctx(req);
// subreq = *req;
    skcipher_request_set_tfm(subreq, sctx.fallback.skcipher);
    return (modifier & CPACF_DECRYPT) ?
    crypto_skcipher_decrypt(subreq) :
    crypto_skcipher_encrypt(subreq);
    }
    static int ecb_aes_set_key(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    unsigned long fc;
// Pick the correct function code based on the key length
    fc = (key_len == 16) ? CPACF_KM_AES_128 :
    (key_len == 24) ? CPACF_KM_AES_192 :
    (key_len == 32) ? CPACF_KM_AES_256 : 0;
// Check if the function code is available
    sctx.fc = (fc && cpacf_test_func(&km_functions, fc)) ? fc : 0;
    if (!sctx.fc)
    return setkey_fallback_skcipher(tfm, in_key, key_len);
    sctx.key_len = key_len;
    memcpy(sctx.key, in_key, key_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ecb_aes_crypt(req: *mut skcipher_request, modifier: c_ulong) -> c_int {
    static int ecb_aes_crypt(struct skcipher_request *req, unsigned long modifier)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes, n;
    int ret;
    if (unlikely(!sctx.fc))
    return fallback_skcipher_crypt(sctx, req, modifier);
    ret = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    cpacf_km(sctx.fc | modifier, sctx.key,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    ret = skcipher_walk_done(&walk, nbytes - n);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ecb_aes_encrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_aes_encrypt(struct skcipher_request *req)
    {
    return ecb_aes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn ecb_aes_decrypt(req: *mut skcipher_request) -> c_int {
    static int ecb_aes_decrypt(struct skcipher_request *req)
    {
    return ecb_aes_crypt(req, CPACF_DECRYPT);
    }
#[no_mangle]
unsafe extern "C" fn fallback_init_skcipher(tfm: *mut crypto_skcipher) -> c_int {
    static int fallback_init_skcipher(struct crypto_skcipher *tfm)
    {
    const char *name = crypto_tfm_alg_name(&tfm.base);
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    sctx.fallback.skcipher = crypto_alloc_skcipher(name, 0,
    CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC);
    if (IS_ERR(sctx.fallback.skcipher)) {
    pr_err("Allocating AES fallback algorithm %s failed\n",
    name);
    return PTR_ERR(sctx.fallback.skcipher);
    }
    crypto_skcipher_set_reqsize(tfm, sizeof(struct skcipher_request) +
    crypto_skcipher_reqsize(sctx.fallback.skcipher));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fallback_exit_skcipher(tfm: *mut crypto_skcipher) {
    static void fallback_exit_skcipher(struct crypto_skcipher *tfm)
    {
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    crypto_free_skcipher(sctx.fallback.skcipher);
    }
    static struct skcipher_alg ecb_aes_alg = {
    .base.cra_name		=	"ecb(aes)",
    .base.cra_driver_name	=	"ecb-aes-s390",
    .base.cra_priority	=	401,	/* combo: aes + ecb + 1 */
    .base.cra_flags		=	CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	=	AES_BLOCK_SIZE,
    .base.cra_ctxsize	=	sizeof(struct s390_aes_ctx),
    .base.cra_module	=	THIS_MODULE,
    .init			=	fallback_init_skcipher,
    .exit			=	fallback_exit_skcipher,
    .min_keysize		=	AES_MIN_KEY_SIZE,
    .max_keysize		=	AES_MAX_KEY_SIZE,
    .setkey			=	ecb_aes_set_key,
    .encrypt		=	ecb_aes_encrypt,
    .decrypt		=	ecb_aes_decrypt,
    };
    static int cbc_aes_set_key(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    unsigned long fc;
// Pick the correct function code based on the key length
    fc = (key_len == 16) ? CPACF_KMC_AES_128 :
    (key_len == 24) ? CPACF_KMC_AES_192 :
    (key_len == 32) ? CPACF_KMC_AES_256 : 0;
// Check if the function code is available
    sctx.fc = (fc && cpacf_test_func(&kmc_functions, fc)) ? fc : 0;
    if (!sctx.fc)
    return setkey_fallback_skcipher(tfm, in_key, key_len);
    sctx.key_len = key_len;
    memcpy(sctx.key, in_key, key_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cbc_aes_crypt(req: *mut skcipher_request, modifier: c_ulong) -> c_int {
    static int cbc_aes_crypt(struct skcipher_request *req, unsigned long modifier)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes, n;
    int ret;
    struct {
    u8 iv[AES_BLOCK_SIZE];
    u8 key[AES_MAX_KEY_SIZE];
    } param;
    if (unlikely(!sctx.fc))
    return fallback_skcipher_crypt(sctx, req, modifier);
    ret = skcipher_walk_virt(&walk, req, false);
    if (ret)
    return ret;
    memcpy(param.iv, walk.iv, AES_BLOCK_SIZE);
    memcpy(param.key, sctx.key, sctx.key_len);
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    cpacf_kmc(sctx.fc | modifier, &param,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    memcpy(walk.iv, param.iv, AES_BLOCK_SIZE);
    ret = skcipher_walk_done(&walk, nbytes - n);
    }
    memzero_explicit(&param, sizeof(param));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cbc_aes_encrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_aes_encrypt(struct skcipher_request *req)
    {
    return cbc_aes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn cbc_aes_decrypt(req: *mut skcipher_request) -> c_int {
    static int cbc_aes_decrypt(struct skcipher_request *req)
    {
    return cbc_aes_crypt(req, CPACF_DECRYPT);
    }
    static struct skcipher_alg cbc_aes_alg = {
    .base.cra_name		=	"cbc(aes)",
    .base.cra_driver_name	=	"cbc-aes-s390",
    .base.cra_priority	=	402,	/* ecb-aes-s390 + 1 */
    .base.cra_flags		=	CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	=	AES_BLOCK_SIZE,
    .base.cra_ctxsize	=	sizeof(struct s390_aes_ctx),
    .base.cra_module	=	THIS_MODULE,
    .init			=	fallback_init_skcipher,
    .exit			=	fallback_exit_skcipher,
    .min_keysize		=	AES_MIN_KEY_SIZE,
    .max_keysize		=	AES_MAX_KEY_SIZE,
    .ivsize			=	AES_BLOCK_SIZE,
    .setkey			=	cbc_aes_set_key,
    .encrypt		=	cbc_aes_encrypt,
    .decrypt		=	cbc_aes_decrypt,
    };
    static int xts_fallback_setkey(struct crypto_skcipher *tfm, const u8 *key,
    unsigned int len)
    {
    struct s390_xts_ctx *xts_ctx = crypto_skcipher_ctx(tfm);
    crypto_skcipher_clear_flags(xts_ctx.fallback, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags(xts_ctx.fallback,
    crypto_skcipher_get_flags(tfm) &
    CRYPTO_TFM_REQ_MASK);
    return crypto_skcipher_setkey(xts_ctx.fallback, key, len);
    }
    static int xts_aes_set_key(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_xts_ctx *xts_ctx = crypto_skcipher_ctx(tfm);
    unsigned long fc;
    int err;
    err = xts_fallback_setkey(tfm, in_key, key_len);
    if (err)
    return err;
// Pick the correct function code based on the key length
    fc = (key_len == 32) ? CPACF_KM_XTS_128 :
    (key_len == 64) ? CPACF_KM_XTS_256 : 0;
// Check if the function code is available
    xts_ctx.fc = (fc && cpacf_test_func(&km_functions, fc)) ? fc : 0;
    if (!xts_ctx.fc)
    return 0;
// Split the XTS key into the two subkeys
    key_len = key_len / 2;
    xts_ctx.key_len = key_len;
    memcpy(xts_ctx.key, in_key, key_len);
    memcpy(xts_ctx.pcc_key, in_key + key_len, key_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xts_aes_crypt(req: *mut skcipher_request, modifier: c_ulong) -> c_int {
    static int xts_aes_crypt(struct skcipher_request *req, unsigned long modifier)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_xts_ctx *xts_ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int offset, nbytes, n;
    int ret;
    struct {
    u8 key[32];
    u8 tweak[16];
    u8 block[16];
    u8 bit[16];
    u8 xts[16];
    } pcc_param;
    struct {
    u8 key[32];
    u8 init[16];
    } xts_param;
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    if (unlikely(!xts_ctx.fc || (req.cryptlen % AES_BLOCK_SIZE) != 0)) {
    struct skcipher_request *subreq = skcipher_request_ctx(req);
// subreq = *req;
    skcipher_request_set_tfm(subreq, xts_ctx.fallback);
    return (modifier & CPACF_DECRYPT) ?
    crypto_skcipher_decrypt(subreq) :
    crypto_skcipher_encrypt(subreq);
    }
    ret = skcipher_walk_virt(&walk, req, false);
    if (ret)
    return ret;
    offset = xts_ctx.key_len & 0x10;
    memset(pcc_param.block, 0, sizeof(pcc_param.block));
    memset(pcc_param.bit, 0, sizeof(pcc_param.bit));
    memset(pcc_param.xts, 0, sizeof(pcc_param.xts));
    memcpy(pcc_param.tweak, walk.iv, sizeof(pcc_param.tweak));
    memcpy(pcc_param.key + offset, xts_ctx.pcc_key, xts_ctx.key_len);
    cpacf_pcc(xts_ctx.fc, pcc_param.key + offset);
    memcpy(xts_param.key + offset, xts_ctx.key, xts_ctx.key_len);
    memcpy(xts_param.init, pcc_param.xts, 16);
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    cpacf_km(xts_ctx.fc | modifier, xts_param.key + offset,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    ret = skcipher_walk_done(&walk, nbytes - n);
    }
    memzero_explicit(&pcc_param, sizeof(pcc_param));
    memzero_explicit(&xts_param, sizeof(xts_param));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn xts_aes_encrypt(req: *mut skcipher_request) -> c_int {
    static int xts_aes_encrypt(struct skcipher_request *req)
    {
    return xts_aes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn xts_aes_decrypt(req: *mut skcipher_request) -> c_int {
    static int xts_aes_decrypt(struct skcipher_request *req)
    {
    return xts_aes_crypt(req, CPACF_DECRYPT);
    }
#[no_mangle]
unsafe extern "C" fn xts_fallback_init(tfm: *mut crypto_skcipher) -> c_int {
    static int xts_fallback_init(struct crypto_skcipher *tfm)
    {
    const char *name = crypto_tfm_alg_name(&tfm.base);
    struct s390_xts_ctx *xts_ctx = crypto_skcipher_ctx(tfm);
    xts_ctx.fallback = crypto_alloc_skcipher(name, 0,
    CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC);
    if (IS_ERR(xts_ctx.fallback)) {
    pr_err("Allocating XTS fallback algorithm %s failed\n",
    name);
    return PTR_ERR(xts_ctx.fallback);
    }
    crypto_skcipher_set_reqsize(tfm, sizeof(struct skcipher_request) +
    crypto_skcipher_reqsize(xts_ctx.fallback));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xts_fallback_exit(tfm: *mut crypto_skcipher) {
    static void xts_fallback_exit(struct crypto_skcipher *tfm)
    {
    struct s390_xts_ctx *xts_ctx = crypto_skcipher_ctx(tfm);
    crypto_free_skcipher(xts_ctx.fallback);
    }
    static struct skcipher_alg xts_aes_alg = {
    .base.cra_name		=	"xts(aes)",
    .base.cra_driver_name	=	"xts-aes-s390",
    .base.cra_priority	=	402,	/* ecb-aes-s390 + 1 */
    .base.cra_flags		=	CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	=	AES_BLOCK_SIZE,
    .base.cra_ctxsize	=	sizeof(struct s390_xts_ctx),
    .base.cra_module	=	THIS_MODULE,
    .init			=	xts_fallback_init,
    .exit			=	xts_fallback_exit,
    .min_keysize		=	2 * AES_MIN_KEY_SIZE,
    .max_keysize		=	2 * AES_MAX_KEY_SIZE,
    .ivsize			=	AES_BLOCK_SIZE,
    .setkey			=	xts_aes_set_key,
    .encrypt		=	xts_aes_encrypt,
    .decrypt		=	xts_aes_decrypt,
    };
    static int fullxts_aes_set_key(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_xts_ctx *xts_ctx = crypto_skcipher_ctx(tfm);
    unsigned long fc;
    int err;
    err = xts_fallback_setkey(tfm, in_key, key_len);
    if (err)
    return err;
// Pick the correct function code based on the key length
    fc = (key_len == 32) ? CPACF_KM_XTS_128_FULL :
    (key_len == 64) ? CPACF_KM_XTS_256_FULL : 0;
// Check if the function code is available
    xts_ctx.fc = (fc && cpacf_test_func(&km_functions, fc)) ? fc : 0;
    if (!xts_ctx.fc)
    return 0;
// Store double-key
    memcpy(xts_ctx.keys, in_key, key_len);
    xts_ctx.key_len = key_len;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fullxts_aes_crypt(req: *mut skcipher_request, modifier: c_ulong) -> c_int {
    static int fullxts_aes_crypt(struct skcipher_request *req,  unsigned long modifier)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_xts_ctx *xts_ctx = crypto_skcipher_ctx(tfm);
    unsigned int offset, nbytes, n;
    struct skcipher_walk walk;
    int ret;
    struct {
    __u8 key[64];
    __u8 tweak[16];
    __u8 nap[16];
    } fxts_param = {
    .nap = {0},
    };
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    if (unlikely(!xts_ctx.fc || (req.cryptlen % AES_BLOCK_SIZE) != 0)) {
    struct skcipher_request *subreq = skcipher_request_ctx(req);
// subreq = *req;
    skcipher_request_set_tfm(subreq, xts_ctx.fallback);
    return (modifier & CPACF_DECRYPT) ?
    crypto_skcipher_decrypt(subreq) :
    crypto_skcipher_encrypt(subreq);
    }
    ret = skcipher_walk_virt(&walk, req, false);
    if (ret)
    return ret;
    offset = xts_ctx.key_len & 0x20;
    memcpy(fxts_param.key + offset, xts_ctx.keys, xts_ctx.key_len);
    memcpy(fxts_param.tweak, req.iv, AES_BLOCK_SIZE);
    fxts_param.nap[0] = 0x01; /* initial alpha power (1, little-endian) */
    while ((nbytes = walk.nbytes) != 0) {
// only use complete blocks
    n = nbytes & ~(AES_BLOCK_SIZE - 1);
    cpacf_km(xts_ctx.fc | modifier, fxts_param.key + offset,
    walk.dst.virt.addr, walk.src.virt.addr, n);
    ret = skcipher_walk_done(&walk, nbytes - n);
    }
    memzero_explicit(&fxts_param, sizeof(fxts_param));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fullxts_aes_encrypt(req: *mut skcipher_request) -> c_int {
    static int fullxts_aes_encrypt(struct skcipher_request *req)
    {
    return fullxts_aes_crypt(req, 0);
    }
#[no_mangle]
unsafe extern "C" fn fullxts_aes_decrypt(req: *mut skcipher_request) -> c_int {
    static int fullxts_aes_decrypt(struct skcipher_request *req)
    {
    return fullxts_aes_crypt(req, CPACF_DECRYPT);
    }
    static struct skcipher_alg fullxts_aes_alg = {
    .base.cra_name		=	"xts(aes)",
    .base.cra_driver_name	=	"full-xts-aes-s390",
    .base.cra_priority	=	403,	/* aes-xts-s390 + 1 */
    .base.cra_flags		=	CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	=	AES_BLOCK_SIZE,
    .base.cra_ctxsize	=	sizeof(struct s390_xts_ctx),
    .base.cra_module	=	THIS_MODULE,
    .init			=	xts_fallback_init,
    .exit			=	xts_fallback_exit,
    .min_keysize		=	2 * AES_MIN_KEY_SIZE,
    .max_keysize		=	2 * AES_MAX_KEY_SIZE,
    .ivsize			=	AES_BLOCK_SIZE,
    .setkey			=	fullxts_aes_set_key,
    .encrypt		=	fullxts_aes_encrypt,
    .decrypt		=	fullxts_aes_decrypt,
    };
    static int ctr_aes_set_key(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    unsigned long fc;
// Pick the correct function code based on the key length
    fc = (key_len == 16) ? CPACF_KMCTR_AES_128 :
    (key_len == 24) ? CPACF_KMCTR_AES_192 :
    (key_len == 32) ? CPACF_KMCTR_AES_256 : 0;
// Check if the function code is available
    sctx.fc = (fc && cpacf_test_func(&kmctr_functions, fc)) ? fc : 0;
    if (!sctx.fc)
    return setkey_fallback_skcipher(tfm, in_key, key_len);
    sctx.key_len = key_len;
    memcpy(sctx.key, in_key, key_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __ctrblk_init(ctrptr: *mut u8, iv: *mut u8, nbytes: c_uint) -> c_uint {
    static unsigned int __ctrblk_init(u8 *ctrptr, u8 *iv, unsigned int nbytes)
    {
    unsigned int i, n;
// only use complete blocks, max. PAGE_SIZE
    memcpy(ctrptr, iv, AES_BLOCK_SIZE);
    n = (nbytes > PAGE_SIZE) ? PAGE_SIZE : nbytes & ~(AES_BLOCK_SIZE - 1);
    for (i = (n / AES_BLOCK_SIZE) - 1; i > 0; i--) {
    memcpy(ctrptr + AES_BLOCK_SIZE, ctrptr, AES_BLOCK_SIZE);
    crypto_inc(ctrptr + AES_BLOCK_SIZE, AES_BLOCK_SIZE);
    ctrptr += AES_BLOCK_SIZE;
    }
    return n;
    }
#[no_mangle]
unsafe extern "C" fn ctr_aes_crypt(req: *mut skcipher_request) -> c_int {
    static int ctr_aes_crypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct s390_aes_ctx *sctx = crypto_skcipher_ctx(tfm);
    u8 buf[AES_BLOCK_SIZE], *ctrptr;
    struct skcipher_walk walk;
    unsigned int n, nbytes;
    int ret, locked;
    if (unlikely(!sctx.fc))
    return fallback_skcipher_crypt(sctx, req, 0);
    locked = mutex_trylock(&ctrblk_lock);
    ret = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) >= AES_BLOCK_SIZE) {
    n = AES_BLOCK_SIZE;
    if (nbytes >= 2*AES_BLOCK_SIZE && locked)
    n = __ctrblk_init(ctrblk, walk.iv, nbytes);
    ctrptr = (n > AES_BLOCK_SIZE) ? ctrblk : walk.iv;
    cpacf_kmctr(sctx.fc, sctx.key, walk.dst.virt.addr,
    walk.src.virt.addr, n, ctrptr);
    if (ctrptr == ctrblk)
    memcpy(walk.iv, ctrptr + n - AES_BLOCK_SIZE,
    AES_BLOCK_SIZE);
    crypto_inc(walk.iv, AES_BLOCK_SIZE);
    ret = skcipher_walk_done(&walk, nbytes - n);
    }
    if (locked)
    mutex_unlock(&ctrblk_lock);
//
// final block may be < AES_BLOCK_SIZE, copy only nbytes
//
    if (nbytes) {
    memset(buf, 0, AES_BLOCK_SIZE);
    memcpy(buf, walk.src.virt.addr, nbytes);
    cpacf_kmctr(sctx.fc, sctx.key, buf, buf,
    AES_BLOCK_SIZE, walk.iv);
    memcpy(walk.dst.virt.addr, buf, nbytes);
    crypto_inc(walk.iv, AES_BLOCK_SIZE);
    ret = skcipher_walk_done(&walk, 0);
    }
    return ret;
    }
    static struct skcipher_alg ctr_aes_alg = {
    .base.cra_name		=	"ctr(aes)",
    .base.cra_driver_name	=	"ctr-aes-s390",
    .base.cra_priority	=	402,	/* ecb-aes-s390 + 1 */
    .base.cra_flags		=	CRYPTO_ALG_NEED_FALLBACK,
    .base.cra_blocksize	=	1,
    .base.cra_ctxsize	=	sizeof(struct s390_aes_ctx),
    .base.cra_module	=	THIS_MODULE,
    .init			=	fallback_init_skcipher,
    .exit			=	fallback_exit_skcipher,
    .min_keysize		=	AES_MIN_KEY_SIZE,
    .max_keysize		=	AES_MAX_KEY_SIZE,
    .ivsize			=	AES_BLOCK_SIZE,
    .setkey			=	ctr_aes_set_key,
    .encrypt		=	ctr_aes_crypt,
    .decrypt		=	ctr_aes_crypt,
    .chunksize		=	AES_BLOCK_SIZE,
    };
    static int gcm_aes_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct s390_aes_ctx *ctx = crypto_aead_ctx(tfm);
    switch (keylen) {
    case AES_KEYSIZE_128:
    ctx.fc = CPACF_KMA_GCM_AES_128;
    break;
    case AES_KEYSIZE_192:
    ctx.fc = CPACF_KMA_GCM_AES_192;
    break;
    case AES_KEYSIZE_256:
    ctx.fc = CPACF_KMA_GCM_AES_256;
    break;
    default:
    return -EINVAL;
    }
    memcpy(ctx.key, key, keylen);
    ctx.key_len = keylen;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gcm_aes_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int gcm_aes_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    switch (authsize) {
    case 4:
    case 8:
    case 12:
    case 13:
    case 14:
    case 15:
    case 16:
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static void gcm_walk_start(struct gcm_sg_walk *gw, struct scatterlist *sg,
    unsigned int len)
    {
    memset(gw, 0, sizeof(*gw));
    gw.walk_bytes_remain = len;
    scatterwalk_start(&gw.walk, sg);
    }
#[no_mangle]
pub unsafe extern "C" fn _gcm_sg_clamp_and_map(gw: *mut gcm_sg_walk) -> c_uint {
    static inline unsigned int _gcm_sg_clamp_and_map(struct gcm_sg_walk *gw)
    {
    if (gw.walk_bytes_remain == 0)
    return 0;
    gw.walk_bytes = scatterwalk_next(&gw.walk, gw.walk_bytes_remain);
    return gw.walk_bytes;
    }
    static inline void _gcm_sg_unmap_and_advance(struct gcm_sg_walk *gw,
    unsigned int nbytes, bool out)
    {
    gw.walk_bytes_remain -= nbytes;
    if (out)
    scatterwalk_done_dst(&gw.walk, nbytes);
    else
    scatterwalk_done_src(&gw.walk, nbytes);
    }
#[no_mangle]
unsafe extern "C" fn gcm_in_walk_go(gw: *mut gcm_sg_walk, minbytesneeded: c_uint) -> c_int {
    static int gcm_in_walk_go(struct gcm_sg_walk *gw, unsigned int minbytesneeded)
    {
    int n;
    if (gw.buf_bytes && gw.buf_bytes >= minbytesneeded) {
    gw.ptr = gw.buf;
    gw.nbytes = gw.buf_bytes;
    goto out;
    }
    if (gw.walk_bytes_remain == 0) {
    gw.ptr = core::ptr::null_mut();
    gw.nbytes = 0;
    goto out;
    }
    if (!_gcm_sg_clamp_and_map(gw)) {
    gw.ptr = core::ptr::null_mut();
    gw.nbytes = 0;
    goto out;
    }
    if (!gw.buf_bytes && gw.walk_bytes >= minbytesneeded) {
    gw.ptr = gw.walk.addr;
    gw.nbytes = gw.walk_bytes;
    goto out;
    }
    while (1) {
    n = min(gw.walk_bytes, AES_BLOCK_SIZE - gw.buf_bytes);
    memcpy(gw.buf + gw.buf_bytes, gw.walk.addr, n);
    gw.buf_bytes += n;
    _gcm_sg_unmap_and_advance(gw, n, false);
    if (gw.buf_bytes >= minbytesneeded) {
    gw.ptr = gw.buf;
    gw.nbytes = gw.buf_bytes;
    goto out;
    }
    if (!_gcm_sg_clamp_and_map(gw)) {
    gw.ptr = core::ptr::null_mut();
    gw.nbytes = 0;
    goto out;
    }
    }
    out:
    return gw.nbytes;
    }
#[no_mangle]
unsafe extern "C" fn gcm_out_walk_go(gw: *mut gcm_sg_walk, minbytesneeded: c_uint) -> c_int {
    static int gcm_out_walk_go(struct gcm_sg_walk *gw, unsigned int minbytesneeded)
    {
    if (gw.walk_bytes_remain == 0) {
    gw.ptr = core::ptr::null_mut();
    gw.nbytes = 0;
    goto out;
    }
    if (!_gcm_sg_clamp_and_map(gw)) {
    gw.ptr = core::ptr::null_mut();
    gw.nbytes = 0;
    goto out;
    }
    if (gw.walk_bytes >= minbytesneeded) {
    gw.ptr = gw.walk.addr;
    gw.nbytes = gw.walk_bytes;
    goto out;
    }
    scatterwalk_unmap(&gw.walk);
    gw.ptr = gw.buf;
    gw.nbytes = sizeof(gw.buf);
    out:
    return gw.nbytes;
    }
#[no_mangle]
unsafe extern "C" fn gcm_in_walk_done(gw: *mut gcm_sg_walk, bytesdone: c_uint) -> c_int {
    static int gcm_in_walk_done(struct gcm_sg_walk *gw, unsigned int bytesdone)
    {
    if (gw.ptr == core::ptr::null_mut())
    return 0;
    if (gw.ptr == gw.buf) {
    let mut n: c_int = gw.buf_bytes - bytesdone;
    if (n > 0) {
    memmove(gw.buf, gw.buf + bytesdone, n);
    gw.buf_bytes = n;
    } else
    gw.buf_bytes = 0;
    } else
    _gcm_sg_unmap_and_advance(gw, bytesdone, false);
    return bytesdone;
    }
#[no_mangle]
unsafe extern "C" fn gcm_out_walk_done(gw: *mut gcm_sg_walk, bytesdone: c_uint) -> c_int {
    static int gcm_out_walk_done(struct gcm_sg_walk *gw, unsigned int bytesdone)
    {
    int i, n;
    if (gw.ptr == core::ptr::null_mut())
    return 0;
    if (gw.ptr == gw.buf) {
    for (i = 0; i < bytesdone; i += n) {
    if (!_gcm_sg_clamp_and_map(gw))
    return i;
    n = min(gw.walk_bytes, bytesdone - i);
    memcpy(gw.walk.addr, gw.buf + i, n);
    _gcm_sg_unmap_and_advance(gw, n, true);
    }
    } else
    _gcm_sg_unmap_and_advance(gw, bytesdone, true);
    return bytesdone;
    }
#[no_mangle]
unsafe extern "C" fn gcm_aes_crypt(req: *mut aead_request, flags: c_uint) -> c_int {
    static int gcm_aes_crypt(struct aead_request *req, unsigned int flags)
    {
    struct crypto_aead *tfm = crypto_aead_reqtfm(req);
    struct s390_aes_ctx *ctx = crypto_aead_ctx(tfm);
    let mut ivsize: c_uint = crypto_aead_ivsize(tfm);
    let mut taglen: c_uint = crypto_aead_authsize(tfm);
    let mut aadlen: c_uint = req.assoclen;
    let mut pclen: c_uint = req.cryptlen;
    let mut ret: c_int = 0;
    unsigned int n, len, in_bytes, out_bytes,
    min_bytes, bytes, aad_bytes, pc_bytes;
    struct gcm_sg_walk gw_in, gw_out;
    u8 tag[GHASH_DIGEST_SIZE];
    struct {
    u32 _[3];		/* reserved */
    u32 cv;			/* Counter Value */
    u8 t[GHASH_DIGEST_SIZE];/* Tag */
    u8 h[AES_BLOCK_SIZE];	/* Hash-subkey */
    u64 taadl;		/* Total AAD Length */
    u64 tpcl;		/* Total Plain-/Cipher-text Length */
    u8 j0[GHASH_BLOCK_SIZE];/* initial counter value */
    u8 k[AES_MAX_KEY_SIZE];	/* Key */
    } param;
//
// encrypt
// req->src: aad||plaintext
// req->dst: aad||ciphertext||tag
// decrypt
// req->src: aad||ciphertext||tag
// req->dst: aad||plaintext, return 0 or -EBADMSG
// aad, plaintext and ciphertext may be empty.
//
    if (flags & CPACF_DECRYPT)
    pclen -= taglen;
    len = aadlen + pclen;
    memset(&param, 0, sizeof(param));
    param.cv = 1;
    param.taadl = aadlen * 8;
    param.tpcl = pclen * 8;
    memcpy(param.j0, req.iv, ivsize);
// (u32 *)(param.j0 + ivsize) = 1;
    memcpy(param.k, ctx.key, ctx.key_len);
    gcm_walk_start(&gw_in, req.src, len);
    gcm_walk_start(&gw_out, req.dst, len);
    do {
    min_bytes = min_t(unsigned int,
    aadlen > 0 ? aadlen : pclen, AES_BLOCK_SIZE);
    in_bytes = gcm_in_walk_go(&gw_in, min_bytes);
    out_bytes = gcm_out_walk_go(&gw_out, min_bytes);
    bytes = min(in_bytes, out_bytes);
    if (aadlen + pclen <= bytes) {
    aad_bytes = aadlen;
    pc_bytes = pclen;
    flags |= CPACF_KMA_LAAD | CPACF_KMA_LPC;
    } else {
    if (aadlen <= bytes) {
    aad_bytes = aadlen;
    pc_bytes = (bytes - aadlen) &
    ~(AES_BLOCK_SIZE - 1);
    flags |= CPACF_KMA_LAAD;
    } else {
    aad_bytes = bytes & ~(AES_BLOCK_SIZE - 1);
    pc_bytes = 0;
    }
    }
    if (aad_bytes > 0)
    memcpy(gw_out.ptr, gw_in.ptr, aad_bytes);
    cpacf_kma(ctx.fc | flags, &param,
    gw_out.ptr + aad_bytes,
    gw_in.ptr + aad_bytes, pc_bytes,
    gw_in.ptr, aad_bytes);
    n = aad_bytes + pc_bytes;
    if (gcm_in_walk_done(&gw_in, n) != n)
    return -ENOMEM;
    if (gcm_out_walk_done(&gw_out, n) != n)
    return -ENOMEM;
    aadlen -= aad_bytes;
    pclen -= pc_bytes;
    } while (aadlen + pclen > 0);
    if (flags & CPACF_DECRYPT) {
    scatterwalk_map_and_copy(tag, req.src, len, taglen, 0);
    if (crypto_memneq(tag, param.t, taglen))
    ret = -EBADMSG;
    } else
    scatterwalk_map_and_copy(param.t, req.dst, len, taglen, 1);
    memzero_explicit(&param, sizeof(param));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gcm_aes_encrypt(req: *mut aead_request) -> c_int {
    static int gcm_aes_encrypt(struct aead_request *req)
    {
    return gcm_aes_crypt(req, CPACF_ENCRYPT);
    }
#[no_mangle]
unsafe extern "C" fn gcm_aes_decrypt(req: *mut aead_request) -> c_int {
    static int gcm_aes_decrypt(struct aead_request *req)
    {
    return gcm_aes_crypt(req, CPACF_DECRYPT);
    }
    static struct aead_alg gcm_aes_aead = {
    .setkey			= gcm_aes_setkey,
    .setauthsize		= gcm_aes_setauthsize,
    .encrypt		= gcm_aes_encrypt,
    .decrypt		= gcm_aes_decrypt,
    .ivsize			= GHASH_BLOCK_SIZE - sizeof(u32),
    .maxauthsize		= GHASH_DIGEST_SIZE,
    .chunksize		= AES_BLOCK_SIZE,
    .base			= {
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct s390_aes_ctx),
    .cra_priority		= 900,
    .cra_name		= "gcm(aes)",
    .cra_driver_name	= "gcm-aes-s390",
    .cra_module		= THIS_MODULE,
    },
    };
    static struct skcipher_alg *aes_s390_skcipher_algs[5];
    static int aes_s390_skciphers_num;
    static struct aead_alg *aes_s390_aead_alg;
#[no_mangle]
unsafe extern "C" fn aes_s390_register_skcipher(alg: *mut skcipher_alg) -> c_int {
    static int aes_s390_register_skcipher(struct skcipher_alg *alg)
    {
    int ret;
    ret = crypto_register_skcipher(alg);
    if (!ret)
    aes_s390_skcipher_algs[aes_s390_skciphers_num++] = alg;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aes_s390_fini() {
    static void aes_s390_fini(void)
    {
    while (aes_s390_skciphers_num--)
    crypto_unregister_skcipher(aes_s390_skcipher_algs[aes_s390_skciphers_num]);
    if (ctrblk)
    free_page((unsigned long) ctrblk);
    if (aes_s390_aead_alg)
    crypto_unregister_aead(aes_s390_aead_alg);
    }
#[no_mangle]
unsafe extern "C" fn aes_s390_init() -> int __init {
    static int __init aes_s390_init(void)
    {
    int ret;
// Query available functions for KM, KMC, KMCTR and KMA
    cpacf_query(CPACF_KM, &km_functions);
    cpacf_query(CPACF_KMC, &kmc_functions);
    cpacf_query(CPACF_KMCTR, &kmctr_functions);
    cpacf_query(CPACF_KMA, &kma_functions);
    if (cpacf_test_func(&km_functions, CPACF_KM_AES_128) ||
    cpacf_test_func(&km_functions, CPACF_KM_AES_192) ||
    cpacf_test_func(&km_functions, CPACF_KM_AES_256)) {
    ret = aes_s390_register_skcipher(&ecb_aes_alg);
    if (ret)
    goto out_err;
    }
    if (cpacf_test_func(&kmc_functions, CPACF_KMC_AES_128) ||
    cpacf_test_func(&kmc_functions, CPACF_KMC_AES_192) ||
    cpacf_test_func(&kmc_functions, CPACF_KMC_AES_256)) {
    ret = aes_s390_register_skcipher(&cbc_aes_alg);
    if (ret)
    goto out_err;
    }
    if (cpacf_test_func(&km_functions, CPACF_KM_XTS_128_FULL) ||
    cpacf_test_func(&km_functions, CPACF_KM_XTS_256_FULL)) {
    ret = aes_s390_register_skcipher(&fullxts_aes_alg);
    if (ret)
    goto out_err;
    }
    if (cpacf_test_func(&km_functions, CPACF_KM_XTS_128) ||
    cpacf_test_func(&km_functions, CPACF_KM_XTS_256)) {
    ret = aes_s390_register_skcipher(&xts_aes_alg);
    if (ret)
    goto out_err;
    }
    if (cpacf_test_func(&kmctr_functions, CPACF_KMCTR_AES_128) ||
    cpacf_test_func(&kmctr_functions, CPACF_KMCTR_AES_192) ||
    cpacf_test_func(&kmctr_functions, CPACF_KMCTR_AES_256)) {
    ctrblk = (u8 *) __get_free_page(GFP_KERNEL);
    if (!ctrblk) {
    ret = -ENOMEM;
    goto out_err;
    }
    ret = aes_s390_register_skcipher(&ctr_aes_alg);
    if (ret)
    goto out_err;
    }
    if (cpacf_test_func(&kma_functions, CPACF_KMA_GCM_AES_128) ||
    cpacf_test_func(&kma_functions, CPACF_KMA_GCM_AES_192) ||
    cpacf_test_func(&kma_functions, CPACF_KMA_GCM_AES_256)) {
    ret = crypto_register_aead(&gcm_aes_aead);
    if (ret)
    goto out_err;
    aes_s390_aead_alg = &gcm_aes_aead;
    }
    return 0;
    out_err:
    aes_s390_fini();
    return ret;
    }
    module_cpu_feature_match(S390_CPU_FEATURE_MSA, aes_s390_init);
    module_exit(aes_s390_fini);
    MODULE_ALIAS_CRYPTO("aes-all");
    MODULE_DESCRIPTION("Rijndael (AES) Cipher Algorithm");
    MODULE_LICENSE("GPL");
