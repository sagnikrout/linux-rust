//! Automatically rewritten from C to Rust
//! Source: crypto/cbc.c
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
// CBC: Cipher Block Chaining mode
//
// Copyright (c) 2006-2016 Herbert Xu <herbert@gondor.apana.org.au>
//

    static int crypto_cbc_encrypt_segment(struct crypto_lskcipher *tfm,
    const u8 *src, u8 *dst, unsigned nbytes,
    u8 *iv)
    {
    let mut bsize: c_uint = crypto_lskcipher_blocksize(tfm);
    for (; nbytes >= bsize; src += bsize, dst += bsize, nbytes -= bsize) {
    crypto_xor(iv, src, bsize);
    crypto_lskcipher_encrypt(tfm, iv, dst, bsize, core::ptr::null_mut());
    memcpy(iv, dst, bsize);
    }
    return nbytes;
    }
    static int crypto_cbc_encrypt_inplace(struct crypto_lskcipher *tfm,
    u8 *src, unsigned nbytes, u8 *oiv)
    {
    let mut bsize: c_uint = crypto_lskcipher_blocksize(tfm);
    u8 *iv = oiv;
    if (nbytes < bsize)
    goto out;
    do {
    crypto_xor(src, iv, bsize);
    crypto_lskcipher_encrypt(tfm, src, src, bsize, core::ptr::null_mut());
    iv = src;
    src += bsize;
    } while ((nbytes -= bsize) >= bsize);
    memcpy(oiv, iv, bsize);
    out:
    return nbytes;
    }
    static int crypto_cbc_encrypt(struct crypto_lskcipher *tfm, const u8 *src,
    u8 *dst, unsigned len, u8 *iv, u32 flags)
    {
    struct crypto_lskcipher **ctx = crypto_lskcipher_ctx(tfm);
    let mut final: bool = flags & CRYPTO_LSKCIPHER_FLAG_FINAL;
    struct crypto_lskcipher *cipher = *ctx;
    int rem;
    if (src == dst)
    rem = crypto_cbc_encrypt_inplace(cipher, dst, len, iv);
    else
    rem = crypto_cbc_encrypt_segment(cipher, src, dst, len, iv);
    return rem && final ? -EINVAL : rem;
    }
    static int crypto_cbc_decrypt_segment(struct crypto_lskcipher *tfm,
    const u8 *src, u8 *dst, unsigned nbytes,
    u8 *oiv)
    {
    let mut bsize: c_uint = crypto_lskcipher_blocksize(tfm);
    const u8 *iv = oiv;
    if (nbytes < bsize)
    goto out;
    do {
    crypto_lskcipher_decrypt(tfm, src, dst, bsize, core::ptr::null_mut());
    crypto_xor(dst, iv, bsize);
    iv = src;
    src += bsize;
    dst += bsize;
    } while ((nbytes -= bsize) >= bsize);
    memcpy(oiv, iv, bsize);
    out:
    return nbytes;
    }
    static int crypto_cbc_decrypt_inplace(struct crypto_lskcipher *tfm,
    u8 *src, unsigned nbytes, u8 *iv)
    {
    let mut bsize: c_uint = crypto_lskcipher_blocksize(tfm);
    u8 last_iv[MAX_CIPHER_BLOCKSIZE];
    if (nbytes < bsize)
    goto out;
// Start of the last block.
    src += nbytes - (nbytes & (bsize - 1)) - bsize;
    memcpy(last_iv, src, bsize);
    for (;;) {
    crypto_lskcipher_decrypt(tfm, src, src, bsize, core::ptr::null_mut());
    if ((nbytes -= bsize) < bsize)
    break;
    crypto_xor(src, src - bsize, bsize);
    src -= bsize;
    }
    crypto_xor(src, iv, bsize);
    memcpy(iv, last_iv, bsize);
    out:
    return nbytes;
    }
    static int crypto_cbc_decrypt(struct crypto_lskcipher *tfm, const u8 *src,
    u8 *dst, unsigned len, u8 *iv, u32 flags)
    {
    struct crypto_lskcipher **ctx = crypto_lskcipher_ctx(tfm);
    let mut final: bool = flags & CRYPTO_LSKCIPHER_FLAG_FINAL;
    struct crypto_lskcipher *cipher = *ctx;
    int rem;
    if (src == dst)
    rem = crypto_cbc_decrypt_inplace(cipher, dst, len, iv);
    else
    rem = crypto_cbc_decrypt_segment(cipher, src, dst, len, iv);
    return rem && final ? -EINVAL : rem;
    }
#[no_mangle]
unsafe extern "C" fn crypto_cbc_create(tmpl: *mut crypto_template, tb: *mut rtattr) -> c_int {
    static int crypto_cbc_create(struct crypto_template *tmpl, struct rtattr **tb)
    {
    struct lskcipher_instance *inst;
    int err;
    inst = lskcipher_alloc_instance_simple(tmpl, tb);
    if (IS_ERR(inst))
    return PTR_ERR(inst);
    err = -EINVAL;
    if (!is_power_of_2(inst.alg.co.base.cra_blocksize))
    goto out_free_inst;
    if (inst.alg.co.statesize)
    goto out_free_inst;
    inst.alg.encrypt = crypto_cbc_encrypt;
    inst.alg.decrypt = crypto_cbc_decrypt;
    err = lskcipher_register_instance(tmpl, inst);
    if (err) {
    out_free_inst:
    inst.free(inst);
    }
    return err;
    }
    static struct crypto_template crypto_cbc_tmpl = {
    .name = "cbc",
    .create = crypto_cbc_create,
    .module = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn crypto_cbc_module_init() -> int __init {
    static int __init crypto_cbc_module_init(void)
    {
    return crypto_register_template(&crypto_cbc_tmpl);
    }
#[no_mangle]
unsafe extern "C" fn crypto_cbc_module_exit() -> void __exit {
    static void __exit crypto_cbc_module_exit(void)
    {
    crypto_unregister_template(&crypto_cbc_tmpl);
    }
    module_init(crypto_cbc_module_init);
    module_exit(crypto_cbc_module_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("CBC block cipher mode of operation");
    MODULE_ALIAS_CRYPTO("cbc");
