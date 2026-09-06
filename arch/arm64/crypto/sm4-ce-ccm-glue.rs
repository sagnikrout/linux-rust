//! Automatically rewritten from C to Rust
//! Source: arch/arm64/crypto/sm4-ce-ccm-glue.c
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
// SM4-CCM AEAD Algorithm using ARMv8 Crypto Extensions
// as specified in rfc8998
// https://datatracker.ietf.org/doc/html/rfc8998
//
// Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
//

    asmlinkage void sm4_ce_cbcmac_update(const u32 *rkey_enc, u8 *mac,
    const u8 *src, unsigned int nblocks);
    asmlinkage void sm4_ce_ccm_enc(const u32 *rkey_enc, u8 *dst, const u8 *src,
    u8 *iv, unsigned int nbytes, u8 *mac);
    asmlinkage void sm4_ce_ccm_dec(const u32 *rkey_enc, u8 *dst, const u8 *src,
    u8 *iv, unsigned int nbytes, u8 *mac);
    asmlinkage void sm4_ce_ccm_final(const u32 *rkey_enc, u8 *iv, u8 *mac);
    static int ccm_setkey(struct crypto_aead *tfm, const u8 *key,
    unsigned int key_len)
    {
    struct sm4_ctx *ctx = crypto_aead_ctx(tfm);
    if (key_len != SM4_KEY_SIZE)
    return -EINVAL;
    scoped_ksimd()
    sm4_ce_expand_key(key, ctx.rkey_enc, ctx.rkey_dec,
    crypto_sm4_fk, crypto_sm4_ck);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccm_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int ccm_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    if ((authsize & 1) || authsize < 4)
    return -EINVAL;
    return 0;
    }
    static int ccm_format_input(u8 info[], struct aead_request *req,
    unsigned int msglen)
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    let mut l: c_uint = req.iv[0] + 1;
    unsigned int m;
    __be32 len;
// verify that CCM dimension 'L': 2 <= L <= 8
    if (l < 2 || l > 8)
    return -EINVAL;
    if (l < 4 && msglen >> (8 * l))
    return -EOVERFLOW;
    memset(&req.iv[SM4_BLOCK_SIZE - l], 0, l);
    memcpy(info, req.iv, SM4_BLOCK_SIZE);
    m = crypto_aead_authsize(aead);
// format flags field per RFC 3610/NIST 800-38C
// info |= ((m - 2) / 2) << 3;
    if (req.assoclen)
// info |= (1 << 6);
//
// format message length field,
// Linux uses a u32 type to represent msglen
//
    if (l >= 4)
    l = 4;
    len = cpu_to_be32(msglen);
    memcpy(&info[SM4_BLOCK_SIZE - l], (u8 *)&len + 4 - l, l);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccm_calculate_auth_mac(req: *mut aead_request, mac[]: u8) {
    static void ccm_calculate_auth_mac(struct aead_request *req, u8 mac[])
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    struct sm4_ctx *ctx = crypto_aead_ctx(aead);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub req->assoclen: u32 assoclen =,
    pub walk: scatter_walk,
    pub len: c_uint,
    if (assoclen < 0xff00) {
    pub cpu_to_be16(assoclen): aadlen.l =,
    pub 2: len =,
    } else {
    pub cpu_to_be16(0xfffe): aadlen.l =,
    pub &aadlen.h): put_unaligned_be32(assoclen,,
    pub 6: len =,
    }
    pub mac): sm4_ce_crypt_block(ctx->rkey_enc, mac,,
    pub len): *const *const crypto_xor(mac, (u8 )&aadlen,,
    pub req->src): scatterwalk_start(&walk,,
    do {
    pub orig_n: unsigned int n,,
    pub p: *const u8,
    pub assoclen): orig_n = scatterwalk_next(&walk,,
    pub walk.addr: p =,
    pub orig_n: n =,
    while (n > 0) {
    pub nblocks: unsigned int l,,
    if (len == SM4_BLOCK_SIZE) {
    if (n < SM4_BLOCK_SIZE) {
    sm4_ce_crypt_block(ctx.rkey_enc,
    pub mac): mac,,
    pub 0: len =,
    } else {
    pub SM4_BLOCK_SIZE: nblocks = n /,
    sm4_ce_cbcmac_update(ctx.rkey_enc,
    pub nblocks): mac, p,,
    pub SM4_BLOCK_SIZE: *mut *mut p += nblocks,
    pub SM4_BLOCK_SIZE: n %=,
    }
    }
    pub len): l = min(n, SM4_BLOCK_SIZE -,
    if (l) {
    pub l): crypto_xor(mac + len, p,,
    pub l: len +=,
    pub l: p +=,
    pub l: n -=,
    }
    }
    pub orig_n): scatterwalk_done_src(&walk,,
    pub orig_n: assoclen -=,
    pub (assoclen): } while,
    }
    static int ccm_crypt(struct aead_request *req, struct skcipher_walk *walk,
    u32 *rkey_enc, u8 mac[],
    void (*sm4_ce_ccm_crypt)(const u32 *rkey_enc, u8 *dst,
    const u8 *src, u8 *iv,
    unsigned int nbytes, u8 *mac))
    {
    pub ctr0: [u8 __aligned(8); SM4_BLOCK_SIZE],
    pub 0: int err =,
// preserve the initial ctr0 for the TAG
    pub SM4_BLOCK_SIZE): memcpy(ctr0, walk->iv,,
    pub SM4_BLOCK_SIZE): crypto_inc(walk->iv,,
    scoped_ksimd() {
    if (req.assoclen)
    pub mac): ccm_calculate_auth_mac(req,,
    while (walk.nbytes) {
    pub SM4_BLOCK_SIZE: unsigned int tail = walk->nbytes %,
    if (walk.nbytes == walk.total)
    pub 0: tail =,
    sm4_ce_ccm_crypt(rkey_enc, walk.dst.virt.addr,
    walk.src.virt.addr, walk.iv,
    pub mac): walk->nbytes - tail,,
    pub tail): err = skcipher_walk_done(walk,,
    }
    pub mac): sm4_ce_ccm_final(rkey_enc, ctr0,,
    }
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn ccm_encrypt(req: *mut aead_request) -> c_int {
    static int ccm_encrypt(struct aead_request *req)
    {
    pub crypto_aead_reqtfm(req): *mut *mut crypto_aead aead =,
    pub crypto_aead_ctx(aead): *mut *mut sm4_ctx ctx =,
    pub mac: [u8 __aligned(8); SM4_BLOCK_SIZE],
    pub walk: skcipher_walk,
    pub err: c_int,
    pub req->cryptlen): err = ccm_format_input(mac, req,,
    if (err)
    pub err: return,
    pub false): err = skcipher_walk_aead_encrypt(&walk, req,,
    if (err)
    pub err: return,
    pub sm4_ce_ccm_enc): err = ccm_crypt(req, &walk, ctx->rkey_enc, mac,,
    if (err)
    pub err: return,
// copy authtag to end of dst
    scatterwalk_map_and_copy(mac, req.dst, req.assoclen + req.cryptlen,
    pub 1): crypto_aead_authsize(aead),,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ccm_decrypt(req: *mut aead_request) -> c_int {
    static int ccm_decrypt(struct aead_request *req)
    {
    pub crypto_aead_reqtfm(req): *mut *mut crypto_aead aead =,
    pub crypto_aead_authsize(aead): unsigned int authsize =,
    pub crypto_aead_ctx(aead): *mut *mut sm4_ctx ctx =,
    pub mac: [u8 __aligned(8); SM4_BLOCK_SIZE],
    pub authtag: [u8; SM4_BLOCK_SIZE],
    pub walk: skcipher_walk,
    pub err: c_int,
    pub authsize): err = ccm_format_input(mac, req, req->cryptlen -,
    if (err)
    pub err: return,
    pub false): err = skcipher_walk_aead_decrypt(&walk, req,,
    if (err)
    pub err: return,
    pub sm4_ce_ccm_dec): err = ccm_crypt(req, &walk, ctx->rkey_enc, mac,,
    if (err)
    pub err: return,
// compare calculated auth tag with the stored one
    scatterwalk_map_and_copy(authtag, req.src,
    req.assoclen + req.cryptlen - authsize,
    pub 0): authsize,,
    if (crypto_memneq(authtag, mac, authsize))
    pub -EBADMSG: return,
    pub 0: return,
    }
    static struct aead_alg sm4_ccm_alg = {
    .base = {
    .cra_name		= "ccm(sm4)",
    .cra_driver_name	= "ccm-sm4-ce",
    .cra_priority		= 400,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct sm4_ctx),
    .cra_module		= THIS_MODULE,
    },
    .ivsize		= SM4_BLOCK_SIZE,
    .chunksize	= SM4_BLOCK_SIZE,
    .maxauthsize	= SM4_BLOCK_SIZE,
    .setkey		= ccm_setkey,
    .setauthsize	= ccm_setauthsize,
    .encrypt	= ccm_encrypt,
    .decrypt	= ccm_decrypt,
}

#[no_mangle]
unsafe extern "C" fn sm4_ce_ccm_init() -> int __init {
    static int __init sm4_ce_ccm_init(void)
    {
    return crypto_register_aead(&sm4_ccm_alg);
    }
#[no_mangle]
unsafe extern "C" fn sm4_ce_ccm_exit() -> void __exit {
    static void __exit sm4_ce_ccm_exit(void)
    {
    crypto_unregister_aead(&sm4_ccm_alg);
    }
    module_cpu_feature_match(SM4, sm4_ce_ccm_init);
    module_exit(sm4_ce_ccm_exit);
    MODULE_DESCRIPTION("Synchronous SM4 in CCM mode using ARMv8 Crypto Extensions");
    MODULE_ALIAS_CRYPTO("ccm(sm4)");
    MODULE_AUTHOR("Tianjia Zhang <tianjia.zhang@linux.alibaba.com>");
    MODULE_LICENSE("GPL v2");
