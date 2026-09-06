//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/crypto/aes-spe-glue.c
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
// Glue code for AES implementation for SPE instructions (PPC)
//
// Based on generic implementation. The assembler module takes care
// about the SPE registers so it can run from interrupt context.
//
// Copyright (c) 2015 Markus Stockhausen <stockhausen@collogia.de>
//

//
// MAX_BYTES defines the number of bytes that are allowed to be processed
// between preempt_disable() and preempt_enable(). e500 cores can issue two
// instructions per clock cycle using one 32/64 bit unit (SU1) and one 32
// bit unit (SU2). One of these can be a memory access that is executed via
// a single load and store unit (LSU). XTS-AES-256 takes ~780 operations per
// 16 byte block or 25 cycles per byte. Thus 768 bytes of input data
// will need an estimated maximum of 20,000 cycles. Headroom for cache misses
// included. Even with the low end model clocked at 667 MHz this equals to a
// critical time window of less than 30us. The value has been chosen to
// process a 512 byte disk block in one or a large 1400 bytes IPsec network
// packet in two runs.
//
pub const MAX_BYTES: c_int = 768;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_aes_ctx {
    pub key_enc: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_dec: [u32; AES_MAX_KEYLENGTH_U32],
    pub rounds: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_xts_ctx {
    pub key_enc: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_dec: [u32; AES_MAX_KEYLENGTH_U32],
    pub key_twk: [u32; AES_MAX_KEYLENGTH_U32],
    pub rounds: u32,
}

#[no_mangle]
unsafe extern "C" fn spe_begin() {
    static void spe_begin(void)
    {
// disable preemption and save users SPE registers if required
    preempt_disable();
    enable_kernel_spe();
    }
#[no_mangle]
unsafe extern "C" fn spe_end() {
    static void spe_end(void)
    {
    disable_kernel_spe();
// reenable preemption
    preempt_enable();
    }
    static int ppc_aes_setkey_skcipher(struct crypto_skcipher *tfm,
    const u8 *in_key, unsigned int key_len)
    {
    struct ppc_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    switch (key_len) {
    case AES_KEYSIZE_128:
    ctx.rounds = 4;
    ppc_expand_key_128(ctx.key_enc, in_key);
    break;
    case AES_KEYSIZE_192:
    ctx.rounds = 5;
    ppc_expand_key_192(ctx.key_enc, in_key);
    break;
    case AES_KEYSIZE_256:
    ctx.rounds = 6;
    ppc_expand_key_256(ctx.key_enc, in_key);
    break;
    default:
    return -EINVAL;
    }
    ppc_generate_decrypt_key(ctx.key_dec, ctx.key_enc, key_len);
    return 0;
    }
    static int ppc_xts_setkey(struct crypto_skcipher *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct ppc_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    int err;
    err = xts_verify_key(tfm, in_key, key_len);
    if (err)
    return err;
    key_len >>= 1;
    switch (key_len) {
    case AES_KEYSIZE_128:
    ctx.rounds = 4;
    ppc_expand_key_128(ctx.key_enc, in_key);
    ppc_expand_key_128(ctx.key_twk, in_key + AES_KEYSIZE_128);
    break;
    case AES_KEYSIZE_192:
    ctx.rounds = 5;
    ppc_expand_key_192(ctx.key_enc, in_key);
    ppc_expand_key_192(ctx.key_twk, in_key + AES_KEYSIZE_192);
    break;
    case AES_KEYSIZE_256:
    ctx.rounds = 6;
    ppc_expand_key_256(ctx.key_enc, in_key);
    ppc_expand_key_256(ctx.key_twk, in_key + AES_KEYSIZE_256);
    break;
    default:
    return -EINVAL;
    }
    ppc_generate_decrypt_key(ctx.key_dec, ctx.key_enc, key_len);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ppc_ecb_crypt(req: *mut skcipher_request, enc: bool) -> c_int {
    static int ppc_ecb_crypt(struct skcipher_request *req, bool enc)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ppc_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) != 0) {
    nbytes = min(nbytes, MAX_BYTES);
    nbytes = round_down(nbytes, AES_BLOCK_SIZE);
    spe_begin();
    if (enc)
    ppc_encrypt_ecb(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_enc, ctx.rounds, nbytes);
    else
    ppc_decrypt_ecb(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_dec, ctx.rounds, nbytes);
    spe_end();
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ppc_ecb_encrypt(req: *mut skcipher_request) -> c_int {
    static int ppc_ecb_encrypt(struct skcipher_request *req)
    {
    return ppc_ecb_crypt(req, true);
    }
#[no_mangle]
unsafe extern "C" fn ppc_ecb_decrypt(req: *mut skcipher_request) -> c_int {
    static int ppc_ecb_decrypt(struct skcipher_request *req)
    {
    return ppc_ecb_crypt(req, false);
    }
#[no_mangle]
unsafe extern "C" fn ppc_cbc_crypt(req: *mut skcipher_request, enc: bool) -> c_int {
    static int ppc_cbc_crypt(struct skcipher_request *req, bool enc)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ppc_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) != 0) {
    nbytes = min(nbytes, MAX_BYTES);
    nbytes = round_down(nbytes, AES_BLOCK_SIZE);
    spe_begin();
    if (enc)
    ppc_encrypt_cbc(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_enc, ctx.rounds, nbytes,
    walk.iv);
    else
    ppc_decrypt_cbc(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_dec, ctx.rounds, nbytes,
    walk.iv);
    spe_end();
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ppc_cbc_encrypt(req: *mut skcipher_request) -> c_int {
    static int ppc_cbc_encrypt(struct skcipher_request *req)
    {
    return ppc_cbc_crypt(req, true);
    }
#[no_mangle]
unsafe extern "C" fn ppc_cbc_decrypt(req: *mut skcipher_request) -> c_int {
    static int ppc_cbc_decrypt(struct skcipher_request *req)
    {
    return ppc_cbc_crypt(req, false);
    }
#[no_mangle]
unsafe extern "C" fn ppc_ctr_crypt(req: *mut skcipher_request) -> c_int {
    static int ppc_ctr_crypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ppc_aes_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    err = skcipher_walk_virt(&walk, req, false);
    while ((nbytes = walk.nbytes) != 0) {
    nbytes = min(nbytes, MAX_BYTES);
    if (nbytes < walk.total)
    nbytes = round_down(nbytes, AES_BLOCK_SIZE);
    spe_begin();
    ppc_crypt_ctr(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_enc, ctx.rounds, nbytes, walk.iv);
    spe_end();
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ppc_xts_crypt(req: *mut skcipher_request, enc: bool) -> c_int {
    static int ppc_xts_crypt(struct skcipher_request *req, bool enc)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ppc_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    struct skcipher_walk walk;
    unsigned int nbytes;
    int err;
    u32 *twk;
    err = skcipher_walk_virt(&walk, req, false);
    twk = ctx.key_twk;
    while ((nbytes = walk.nbytes) != 0) {
    nbytes = min(nbytes, MAX_BYTES);
    nbytes = round_down(nbytes, AES_BLOCK_SIZE);
    spe_begin();
    if (enc)
    ppc_encrypt_xts(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_enc, ctx.rounds, nbytes,
    walk.iv, twk);
    else
    ppc_decrypt_xts(walk.dst.virt.addr, walk.src.virt.addr,
    ctx.key_dec, ctx.rounds, nbytes,
    walk.iv, twk);
    spe_end();
    twk = core::ptr::null_mut();
    err = skcipher_walk_done(&walk, walk.nbytes - nbytes);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ppc_xts_encrypt(req: *mut skcipher_request) -> c_int {
    static int ppc_xts_encrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ppc_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut tail: c_int = req.cryptlen % AES_BLOCK_SIZE;
    let mut offset: c_int = req.cryptlen - tail - AES_BLOCK_SIZE;
    struct skcipher_request subreq;
    u8 b[2][AES_BLOCK_SIZE];
    int err;
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    if (tail) {
    subreq = *req;
    skcipher_request_set_crypt(&subreq, req.src, req.dst,
    req.cryptlen - tail, req.iv);
    req = &subreq;
    }
    err = ppc_xts_crypt(req, true);
    if (err || !tail)
    return err;
    scatterwalk_map_and_copy(b[0], req.dst, offset, AES_BLOCK_SIZE, 0);
    memcpy(b[1], b[0], tail);
    scatterwalk_map_and_copy(b[0], req.src, offset + AES_BLOCK_SIZE, tail, 0);
    spe_begin();
    ppc_encrypt_xts(b[0], b[0], ctx.key_enc, ctx.rounds, AES_BLOCK_SIZE,
    req.iv, core::ptr::null_mut());
    spe_end();
    scatterwalk_map_and_copy(b[0], req.dst, offset, AES_BLOCK_SIZE + tail, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ppc_xts_decrypt(req: *mut skcipher_request) -> c_int {
    static int ppc_xts_decrypt(struct skcipher_request *req)
    {
    struct crypto_skcipher *tfm = crypto_skcipher_reqtfm(req);
    struct ppc_xts_ctx *ctx = crypto_skcipher_ctx(tfm);
    let mut tail: c_int = req.cryptlen % AES_BLOCK_SIZE;
    let mut offset: c_int = req.cryptlen - tail - AES_BLOCK_SIZE;
    struct skcipher_request subreq;
    u8 b[3][AES_BLOCK_SIZE];
    le128 twk;
    int err;
    if (req.cryptlen < AES_BLOCK_SIZE)
    return -EINVAL;
    if (tail) {
    subreq = *req;
    skcipher_request_set_crypt(&subreq, req.src, req.dst,
    offset, req.iv);
    req = &subreq;
    }
    err = ppc_xts_crypt(req, false);
    if (err || !tail)
    return err;
    scatterwalk_map_and_copy(b[1], req.src, offset, AES_BLOCK_SIZE + tail, 0);
    spe_begin();
    if (!offset)
    ppc_encrypt_ecb(req.iv, req.iv, ctx.key_twk, ctx.rounds,
    AES_BLOCK_SIZE);
    gf128mul_x_ble(&twk, (le128 *)req.iv);
    ppc_decrypt_xts(b[1], b[1], ctx.key_dec, ctx.rounds, AES_BLOCK_SIZE,
    (u8 *)&twk, core::ptr::null_mut());
    memcpy(b[0], b[2], tail);
    memcpy(b[0] + tail, b[1] + tail, AES_BLOCK_SIZE - tail);
    ppc_decrypt_xts(b[0], b[0], ctx.key_dec, ctx.rounds, AES_BLOCK_SIZE,
    req.iv, core::ptr::null_mut());
    spe_end();
    scatterwalk_map_and_copy(b[0], req.dst, offset, AES_BLOCK_SIZE + tail, 1);
    return 0;
    }
//
// Algorithm definitions. Disabling alignment (cra_alignmask=0) was chosen
// because the e500 platform can handle unaligned reads/writes very efficiently.
// This improves IPsec thoughput by another few percent. Additionally we assume
// that AES context is always aligned to at least 8 bytes because it is created
// with kmalloc() in the crypto infrastructure
//
    static struct skcipher_alg aes_skcipher_algs[] = {
    {
    .base.cra_name		=	"ecb(aes)",
    .base.cra_driver_name	=	"ecb-ppc-spe",
    .base.cra_priority	=	300,
    .base.cra_blocksize	=	AES_BLOCK_SIZE,
    .base.cra_ctxsize	=	sizeof(struct ppc_aes_ctx),
    .base.cra_module	=	THIS_MODULE,
    .min_keysize		=	AES_MIN_KEY_SIZE,
    .max_keysize		=	AES_MAX_KEY_SIZE,
    .setkey			=	ppc_aes_setkey_skcipher,
    .encrypt		=	ppc_ecb_encrypt,
    .decrypt		=	ppc_ecb_decrypt,
    }, {
    .base.cra_name		=	"cbc(aes)",
    .base.cra_driver_name	=	"cbc-ppc-spe",
    .base.cra_priority	=	300,
    .base.cra_blocksize	=	AES_BLOCK_SIZE,
    .base.cra_ctxsize	=	sizeof(struct ppc_aes_ctx),
    .base.cra_module	=	THIS_MODULE,
    .min_keysize		=	AES_MIN_KEY_SIZE,
    .max_keysize		=	AES_MAX_KEY_SIZE,
    .ivsize			=	AES_BLOCK_SIZE,
    .setkey			=	ppc_aes_setkey_skcipher,
    .encrypt		=	ppc_cbc_encrypt,
    .decrypt		=	ppc_cbc_decrypt,
    }, {
    .base.cra_name		=	"ctr(aes)",
    .base.cra_driver_name	=	"ctr-ppc-spe",
    .base.cra_priority	=	300,
    .base.cra_blocksize	=	1,
    .base.cra_ctxsize	=	sizeof(struct ppc_aes_ctx),
    .base.cra_module	=	THIS_MODULE,
    .min_keysize		=	AES_MIN_KEY_SIZE,
    .max_keysize		=	AES_MAX_KEY_SIZE,
    .ivsize			=	AES_BLOCK_SIZE,
    .setkey			=	ppc_aes_setkey_skcipher,
    .encrypt		=	ppc_ctr_crypt,
    .decrypt		=	ppc_ctr_crypt,
    .chunksize		=	AES_BLOCK_SIZE,
    }, {
    .base.cra_name		=	"xts(aes)",
    .base.cra_driver_name	=	"xts-ppc-spe",
    .base.cra_priority	=	300,
    .base.cra_blocksize	=	AES_BLOCK_SIZE,
    .base.cra_ctxsize	=	sizeof(struct ppc_xts_ctx),
    .base.cra_module	=	THIS_MODULE,
    .min_keysize		=	AES_MIN_KEY_SIZE * 2,
    .max_keysize		=	AES_MAX_KEY_SIZE * 2,
    .ivsize			=	AES_BLOCK_SIZE,
    .setkey			=	ppc_xts_setkey,
    .encrypt		=	ppc_xts_encrypt,
    .decrypt		=	ppc_xts_decrypt,
    }
    };
#[no_mangle]
unsafe extern "C" fn ppc_aes_mod_init() -> int __init {
    static int __init ppc_aes_mod_init(void)
    {
    return crypto_register_skciphers(aes_skcipher_algs,
    ARRAY_SIZE(aes_skcipher_algs));
    }
#[no_mangle]
unsafe extern "C" fn ppc_aes_mod_fini() -> void __exit {
    static void __exit ppc_aes_mod_fini(void)
    {
    crypto_unregister_skciphers(aes_skcipher_algs,
    ARRAY_SIZE(aes_skcipher_algs));
    }
    module_init(ppc_aes_mod_init);
    module_exit(ppc_aes_mod_fini);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("AES-ECB/CBC/CTR/XTS, SPE optimized");
    MODULE_ALIAS_CRYPTO("aes");
    MODULE_ALIAS_CRYPTO("ecb(aes)");
    MODULE_ALIAS_CRYPTO("cbc(aes)");
    MODULE_ALIAS_CRYPTO("ctr(aes)");
    MODULE_ALIAS_CRYPTO("xts(aes)");
    MODULE_ALIAS_CRYPTO("aes-ppc-spe");
