//! Automatically rewritten from C to Rust
//! Source: arch/arm64/crypto/aes-ce-ccm-glue.c
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
// aes-ce-ccm-glue.c - AES-CCM transform for ARMv8 with Crypto Extensions
//
// Copyright (C) 2013 - 2017 Linaro Ltd.
// Copyright (C) 2024 Google LLC
//
// Author: Ard Biesheuvel <ardb@kernel.org>
//

    MODULE_IMPORT_NS("CRYPTO_INTERNAL");
#[no_mangle]
unsafe extern "C" fn num_rounds(ctx: *mut crypto_aes_ctx) -> c_int {
    static int num_rounds(struct crypto_aes_ctx *ctx)
    {
//
// # of rounds specified by AES:
// 128 bit key		10 rounds
// 192 bit key		12 rounds
// 256 bit key		14 rounds
// => n byte key	=> 6 + (n/4) rounds
//
    return 6 + ctx.key_length / 4;
    }
    asmlinkage void ce_aes_ccm_encrypt(u8 out[], u8 const in[], u32 cbytes,
    u32 const rk[], u32 rounds, u8 mac[],
    u8 ctr[], u8 const final_iv[]);
    asmlinkage void ce_aes_ccm_decrypt(u8 out[], u8 const in[], u32 cbytes,
    u32 const rk[], u32 rounds, u8 mac[],
    u8 ctr[], u8 const final_iv[]);
    static int ccm_setkey(struct crypto_aead *tfm, const u8 *in_key,
    unsigned int key_len)
    {
    struct crypto_aes_ctx *ctx = crypto_aead_ctx(tfm);
    return ce_aes_expandkey(ctx, in_key, key_len);
    }
#[no_mangle]
unsafe extern "C" fn ccm_setauthsize(tfm: *mut crypto_aead, authsize: c_uint) -> c_int {
    static int ccm_setauthsize(struct crypto_aead *tfm, unsigned int authsize)
    {
    if ((authsize & 1) || authsize < 4)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ccm_init_mac(req: *mut aead_request, maciv[]: u8, msglen: u32) -> c_int {
    static int ccm_init_mac(struct aead_request *req, u8 maciv[], u32 msglen)
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    __be32 *n = (__be32 *)&maciv[AES_BLOCK_SIZE - 8];
    let mut l: u32 = req.iv[0] + 1;
// verify that CCM dimension 'L' is set correctly in the IV
    if (l < 2 || l > 8)
    return -EINVAL;
// verify that msglen can in fact be represented in L bytes
    if (l < 4 && msglen >> (8 * l))
    return -EOVERFLOW;
//
// Even if the CCM spec allows L values of up to 8, the Linux cryptoapi
// uses a u32 type to represent msglen so the top 4 bytes are always 0.
//
    n[0] = 0;
    n[1] = cpu_to_be32(msglen);
    memcpy(maciv, req.iv, AES_BLOCK_SIZE - l);
//
// Meaning of byte 0 according to CCM spec (RFC 3610/NIST 800-38C)
// - bits 0..2	: max # of bytes required to represent msglen, minus 1
// (already set by caller)
// - bits 3..5	: size of auth tag (1 => 4 bytes, 2 => 6 bytes, etc)
// - bit 6	: indicates presence of authenticate-only data
//
    maciv[0] |= (crypto_aead_authsize(aead) - 2) << 2;
    if (req.assoclen)
    maciv[0] |= 0x40;
    memset(&req.iv[AES_BLOCK_SIZE - l], 0, l);
    return 0;
    }
    static u32 ce_aes_ccm_auth_data(u8 mac[], u8 const in[], u32 abytes,
    u32 macp, u32 const rk[], u32 rounds)
    {
    let mut enc_after: c_int = (macp + abytes) % AES_BLOCK_SIZE;
    do {
    let mut blocks: u32 = abytes / AES_BLOCK_SIZE;
    if (macp == AES_BLOCK_SIZE || (!macp && blocks > 0)) {
    ce_aes_mac_update(in, rk, rounds, blocks, mac, macp,
    enc_after);
    macp = enc_after ? 0 : AES_BLOCK_SIZE;
    in += blocks * AES_BLOCK_SIZE;
    abytes -= blocks * AES_BLOCK_SIZE;
    } else {
    let mut l: u32 = min(AES_BLOCK_SIZE - macp, abytes);
    crypto_xor(&mac[macp], in, l);
    in += l;
    macp += l;
    abytes -= l;
    }
    } while (abytes > 0);
    return macp;
    }
#[no_mangle]
unsafe extern "C" fn ccm_calculate_auth_mac(req: *mut aead_request, mac[]: u8) {
    static void ccm_calculate_auth_mac(struct aead_request *req, u8 mac[])
    {
    struct crypto_aead *aead = crypto_aead_reqtfm(req);
    struct crypto_aes_ctx *ctx = crypto_aead_ctx(aead);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub walk: scatter_walk,
    pub req->assoclen: u32 len =,
    pub AES_BLOCK_SIZE: u32 macp =,
// prepend the AAD with a length tag
    if (len < 0xff00) {
    pub cpu_to_be16(len): ltag.l =,
    pub 2: ltag.len =,
    } else  {
    pub cpu_to_be16(0xfffe): ltag.l =,
    pub &ltag.h): put_unaligned_be32(len,,
    pub 6: ltag.len =,
    }
    macp = ce_aes_ccm_auth_data(mac, (u8 *)&ltag, ltag.len, macp,
    pub num_rounds(ctx)): ctx->key_enc,,
    pub req->src): scatterwalk_start(&walk,,
    do {
    pub n: c_uint,
    pub len): n = scatterwalk_next(&walk,,
    macp = ce_aes_ccm_auth_data(mac, walk.addr, n, macp,
    pub num_rounds(ctx)): ctx->key_enc,,
    pub n): scatterwalk_done_src(&walk,,
    pub n: len -=,
    pub (len): } while,
    }
#[no_mangle]
unsafe extern "C" fn ccm_encrypt(req: *mut aead_request) -> c_int {
    static int ccm_encrypt(struct aead_request *req)
    {
    pub crypto_aead_reqtfm(req): *mut *mut crypto_aead aead =,
    pub crypto_aead_ctx(aead): *mut *mut crypto_aes_ctx ctx =,
    pub walk: skcipher_walk,
    pub mac: [u8 __aligned(8); AES_BLOCK_SIZE],
    pub orig_iv: [u8; AES_BLOCK_SIZE],
    pub req->cryptlen: u32 len =,
    pub err: c_int,
    pub len): err = ccm_init_mac(req, mac,,
    if (err)
    pub err: return,
// preserve the original iv for the final round
    pub AES_BLOCK_SIZE): memcpy(orig_iv, req->iv,,
    pub false): err = skcipher_walk_aead_encrypt(&walk, req,,
    if (unlikely(err))
    pub err: return,
    scoped_ksimd() {
    if (req.assoclen)
    pub mac): ccm_calculate_auth_mac(req,,
    do {
    pub AES_BLOCK_SIZE: u32 tail = walk.nbytes %,
    pub walk.src.virt.addr: *const *const u8 src =,
    pub walk.dst.virt.addr: *mut *mut u8 dst =,
    pub buf: [u8; AES_BLOCK_SIZE],
    pub NULL: *mut *mut u8 final_iv =,
    if (walk.nbytes == walk.total) {
    pub 0: tail =,
    pub orig_iv: final_iv =,
    }
    if (unlikely(walk.nbytes < AES_BLOCK_SIZE))
    src = dst = memcpy(&buf[sizeof(buf) - walk.nbytes],
    pub walk.nbytes): src,,
    ce_aes_ccm_encrypt(dst, src, walk.nbytes - tail,
    ctx.key_enc, num_rounds(ctx),
    pub final_iv): mac, walk.iv,,
    if (unlikely(walk.nbytes < AES_BLOCK_SIZE))
    pub walk.nbytes): memcpy(walk.dst.virt.addr, dst,,
    if (walk.nbytes) {
    pub tail): err = skcipher_walk_done(&walk,,
    }
    pub (walk.nbytes): } while,
    }
    if (unlikely(err))
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
    pub crypto_aead_ctx(aead): *mut *mut crypto_aes_ctx ctx =,
    pub crypto_aead_authsize(aead): unsigned int authsize =,
    pub walk: skcipher_walk,
    pub mac: [u8 __aligned(8); AES_BLOCK_SIZE],
    pub orig_iv: [u8; AES_BLOCK_SIZE],
    pub authsize: u32 len = req->cryptlen -,
    pub err: c_int,
    pub len): err = ccm_init_mac(req, mac,,
    if (err)
    pub err: return,
// preserve the original iv for the final round
    pub AES_BLOCK_SIZE): memcpy(orig_iv, req->iv,,
    pub false): err = skcipher_walk_aead_decrypt(&walk, req,,
    if (unlikely(err))
    pub err: return,
    scoped_ksimd() {
    if (req.assoclen)
    pub mac): ccm_calculate_auth_mac(req,,
    do {
    pub AES_BLOCK_SIZE: u32 tail = walk.nbytes %,
    pub walk.src.virt.addr: *const *const u8 src =,
    pub walk.dst.virt.addr: *mut *mut u8 dst =,
    pub buf: [u8; AES_BLOCK_SIZE],
    pub NULL: *mut *mut u8 final_iv =,
    if (walk.nbytes == walk.total) {
    pub 0: tail =,
    pub orig_iv: final_iv =,
    }
    if (unlikely(walk.nbytes < AES_BLOCK_SIZE))
    src = dst = memcpy(&buf[sizeof(buf) - walk.nbytes],
    pub walk.nbytes): src,,
    ce_aes_ccm_decrypt(dst, src, walk.nbytes - tail,
    ctx.key_enc, num_rounds(ctx),
    pub final_iv): mac, walk.iv,,
    if (unlikely(walk.nbytes < AES_BLOCK_SIZE))
    pub walk.nbytes): memcpy(walk.dst.virt.addr, dst,,
    if (walk.nbytes) {
    pub tail): err = skcipher_walk_done(&walk,,
    }
    pub (walk.nbytes): } while,
    }
    if (unlikely(err))
    pub err: return,
// compare calculated auth tag with the stored one
    scatterwalk_map_and_copy(orig_iv, req.src,
    req.assoclen + req.cryptlen - authsize,
    pub 0): authsize,,
    if (crypto_memneq(mac, orig_iv, authsize))
    pub -EBADMSG: return,
    pub 0: return,
    }
    static struct aead_alg ccm_aes_alg = {
    .base = {
    .cra_name		= "ccm(aes)",
    .cra_driver_name	= "ccm-aes-ce",
    .cra_priority		= 300,
    .cra_blocksize		= 1,
    .cra_ctxsize		= sizeof(struct crypto_aes_ctx),
    .cra_module		= THIS_MODULE,
    },
    .ivsize		= AES_BLOCK_SIZE,
    .chunksize	= AES_BLOCK_SIZE,
    .maxauthsize	= AES_BLOCK_SIZE,
    .setkey		= ccm_setkey,
    .setauthsize	= ccm_setauthsize,
    .encrypt	= ccm_encrypt,
    .decrypt	= ccm_decrypt,
}

#[no_mangle]
unsafe extern "C" fn aes_mod_init() -> int __init {
    static int __init aes_mod_init(void)
    {
    if (!cpu_have_named_feature(AES))
    return -ENODEV;
    return crypto_register_aead(&ccm_aes_alg);
    }
#[no_mangle]
unsafe extern "C" fn aes_mod_exit() -> void __exit {
    static void __exit aes_mod_exit(void)
    {
    crypto_unregister_aead(&ccm_aes_alg);
    }
    module_init(aes_mod_init);
    module_exit(aes_mod_exit);
    MODULE_DESCRIPTION("Synchronous AES in CCM mode using ARMv8 Crypto Extensions");
    MODULE_AUTHOR("Ard Biesheuvel <ardb@kernel.org>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS_CRYPTO("ccm(aes)");
