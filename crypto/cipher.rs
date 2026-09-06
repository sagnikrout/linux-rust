//! Automatically rewritten from C to Rust
//! Source: crypto/cipher.c
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
// Single-block cipher operations.
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
//

    static int setkey_unaligned(struct crypto_cipher *tfm, const u8 *key,
    unsigned int keylen)
    {
    struct cipher_alg *cia = crypto_cipher_alg(tfm);
    let mut alignmask: c_ulong = crypto_cipher_alignmask(tfm);
    int ret;
    u8 *buffer, *alignbuffer;
    unsigned long absize;
    absize = keylen + alignmask;
    buffer = kmalloc(absize, GFP_ATOMIC);
    if (!buffer)
    return -ENOMEM;
    alignbuffer = (u8 *)ALIGN((unsigned long)buffer, alignmask + 1);
    memcpy(alignbuffer, key, keylen);
    ret = cia.cia_setkey(crypto_cipher_tfm(tfm), alignbuffer, keylen);
    kfree_sensitive(buffer);
    return ret;
    }
    int crypto_cipher_setkey(struct crypto_cipher *tfm,
    const u8 *key, unsigned int keylen)
    {
    struct cipher_alg *cia = crypto_cipher_alg(tfm);
    let mut alignmask: c_ulong = crypto_cipher_alignmask(tfm);
    if (keylen < cia.cia_min_keysize || keylen > cia.cia_max_keysize)
    return -EINVAL;
    if ((unsigned long)key & alignmask)
    return setkey_unaligned(tfm, key, keylen);
    return cia.cia_setkey(crypto_cipher_tfm(tfm), key, keylen);
    }
    EXPORT_SYMBOL_NS_GPL(crypto_cipher_setkey, "CRYPTO_INTERNAL");
    static inline void cipher_crypt_one(struct crypto_cipher *tfm,
    u8 *dst, const u8 *src, bool enc)
    {
    let mut alignmask: c_ulong = crypto_cipher_alignmask(tfm);
    struct cipher_alg *cia = crypto_cipher_alg(tfm);
    void (*fn)(struct crypto_tfm *, u8 *, const u8 *) =
    enc ? cia.cia_encrypt : cia.cia_decrypt;
    if (unlikely(((unsigned long)dst | (unsigned long)src) & alignmask)) {
    let mut bs: c_uint = crypto_cipher_blocksize(tfm);
    u8 buffer[MAX_CIPHER_BLOCKSIZE + MAX_CIPHER_ALIGNMASK];
    u8 *tmp = (u8 *)ALIGN((unsigned long)buffer, alignmask + 1);
    memcpy(tmp, src, bs);
    fn(crypto_cipher_tfm(tfm), tmp, tmp);
    memcpy(dst, tmp, bs);
    } else {
    fn(crypto_cipher_tfm(tfm), dst, src);
    }
    }
    void crypto_cipher_encrypt_one(struct crypto_cipher *tfm,
    u8 *dst, const u8 *src)
    {
    cipher_crypt_one(tfm, dst, src, true);
    }
    EXPORT_SYMBOL_NS_GPL(crypto_cipher_encrypt_one, "CRYPTO_INTERNAL");
    void crypto_cipher_decrypt_one(struct crypto_cipher *tfm,
    u8 *dst, const u8 *src)
    {
    cipher_crypt_one(tfm, dst, src, false);
    }
    EXPORT_SYMBOL_NS_GPL(crypto_cipher_decrypt_one, "CRYPTO_INTERNAL");
