//! Automatically rewritten from C to Rust
//! Source: crypto/krb5/rfc6803_camellia.c
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
// rfc6803 Camellia Encryption for Kerberos 5
//
// Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Calculate the key derivation function KDF-FEEDBACK_CMAC(key, constant)
//
// n = ceiling(k / 128)
// K(0) = zeros
// K(i) = CMAC(key, K(i-1) | i | constant | 0x00 | k)
// DR(key, constant) = k-truncate(K(1) | K(2) | ... | K(n))
// KDF-FEEDBACK-CMAC(key, constant) = random-to-key(DR(key, constant))
//
// [rfc6803 sec 3]
//
    static int rfc6803_calc_KDF_FEEDBACK_CMAC(const struct krb5_enctype *krb5,
    const struct krb5_buffer *key,
    const struct krb5_buffer *constant,
    struct krb5_buffer *result,
    gfp_t gfp)
    {
    struct crypto_shash *shash;
    struct krb5_buffer K, data;
    struct shash_desc *desc;
    __be32 tmp;
    size_t bsize, offset, seg;
    void *buffer;
    let mut i: u32 = 0, k = result.len * 8;
    u8 *p;
    let mut ret: c_int = -ENOMEM;
    shash = crypto_alloc_shash(krb5.cksum_name, 0, 0);
    if (IS_ERR(shash))
    return (PTR_ERR(shash) == -ENOENT) ? -ENOPKG : PTR_ERR(shash);
    ret = crypto_shash_setkey(shash, key.data, key.len);
    if (ret < 0)
    goto error_shash;
    ret = -ENOMEM;
    K.len = crypto_shash_digestsize(shash);
    data.len = K.len + 4 + constant.len + 1 + 4;
    bsize = krb5_shash_size(shash) +
    krb5_digest_size(shash) +
    crypto_roundup(K.len) +
    crypto_roundup(data.len);
    buffer = kzalloc(bsize, GFP_NOFS);
    if (!buffer)
    goto error_shash;
    desc = buffer;
    desc.tfm = shash;
    K.data = buffer +
    krb5_shash_size(shash) +
    krb5_digest_size(shash);
    data.data = buffer +
    krb5_shash_size(shash) +
    krb5_digest_size(shash) +
    crypto_roundup(K.len);
    p = data.data + K.len + 4;
    memcpy(p, constant.data, constant.len);
    p += constant.len;
// p++ = 0x00;
    tmp = htonl(k);
    memcpy(p, &tmp, 4);
    p += 4;
    ret = -EINVAL;
    if (WARN_ON(p - (u8 *)data.data != data.len))
    goto error;
    offset = 0;
    do {
    i++;
    p = data.data;
    memcpy(p, K.data, K.len);
    p += K.len;
// (__be32 *)p = htonl(i);
    ret = crypto_shash_init(desc);
    if (ret < 0)
    goto error;
    ret = crypto_shash_finup(desc, data.data, data.len, K.data);
    if (ret < 0)
    goto error;
    seg = min_t(size_t, result.len - offset, K.len);
    memcpy(result.data + offset, K.data, seg);
    offset += seg;
    } while (offset < result.len);
    error:
    kfree_sensitive(buffer);
    error_shash:
    crypto_free_shash(shash);
    return ret;
    }
//
// Calculate the pseudo-random function, PRF().
//
// Kp = KDF-FEEDBACK-CMAC(protocol-key, "prf")
// PRF = CMAC(Kp, octet-string)
// [rfc6803 sec 6]
//
    static int rfc6803_calc_PRF(const struct krb5_enctype *krb5,
    const struct krb5_buffer *protocol_key,
    const struct krb5_buffer *octet_string,
    struct krb5_buffer *result,
    gfp_t gfp)
    {
    let mut prfconstant: static struct krb5_buffer = { 3, "prf" };
    struct crypto_shash *shash;
    struct krb5_buffer Kp;
    struct shash_desc *desc;
    size_t bsize;
    void *buffer;
    int ret;
    Kp.len = krb5.prf_len;
    shash = crypto_alloc_shash(krb5.cksum_name, 0, 0);
    if (IS_ERR(shash))
    return (PTR_ERR(shash) == -ENOENT) ? -ENOPKG : PTR_ERR(shash);
    ret = -EINVAL;
    if (result.len != crypto_shash_digestsize(shash))
    goto out_shash;
    ret = -ENOMEM;
    bsize = krb5_shash_size(shash) +
    krb5_digest_size(shash) +
    crypto_roundup(Kp.len);
    buffer = kzalloc(bsize, GFP_NOFS);
    if (!buffer)
    goto out_shash;
    Kp.data = buffer +
    krb5_shash_size(shash) +
    krb5_digest_size(shash);
    ret = rfc6803_calc_KDF_FEEDBACK_CMAC(krb5, protocol_key, &prfconstant,
    &Kp, gfp);
    if (ret < 0)
    goto out;
    ret = crypto_shash_setkey(shash, Kp.data, Kp.len);
    if (ret < 0)
    goto out;
    desc = buffer;
    desc.tfm = shash;
    ret = crypto_shash_init(desc);
    if (ret < 0)
    goto out;
    ret = crypto_shash_finup(desc, octet_string.data, octet_string.len, result.data);
    if (ret < 0)
    goto out;
    out:
    kfree_sensitive(buffer);
    out_shash:
    crypto_free_shash(shash);
    return ret;
    }
    static const struct krb5_crypto_profile rfc6803_crypto_profile = {
    .calc_PRF		= rfc6803_calc_PRF,
    .calc_Kc		= rfc6803_calc_KDF_FEEDBACK_CMAC,
    .calc_Ke		= rfc6803_calc_KDF_FEEDBACK_CMAC,
    .calc_Ki		= rfc6803_calc_KDF_FEEDBACK_CMAC,
    .derive_encrypt_keys	= authenc_derive_encrypt_keys,
    .load_encrypt_keys	= authenc_load_encrypt_keys,
    .derive_checksum_key	= rfc3961_derive_checksum_key,
    .load_checksum_key	= rfc3961_load_checksum_key,
    .encrypt		= krb5_aead_encrypt,
    .decrypt		= krb5_aead_decrypt,
    .get_mic		= rfc3961_get_mic,
    .verify_mic		= rfc3961_verify_mic,
    };
    const struct krb5_enctype krb5_camellia128_cts_cmac = {
    .etype		= KRB5_ENCTYPE_CAMELLIA128_CTS_CMAC,
    .ctype		= KRB5_CKSUMTYPE_CMAC_CAMELLIA128,
    .name		= "camellia128-cts-cmac",
    .encrypt_name	= "krb5enc(cmac(camellia),cts(cbc(camellia)))",
    .cksum_name	= "cmac(camellia)",
    .hash_name	= core::ptr::null_mut(),
    .derivation_enc	= "cts(cbc(camellia))",
    .key_bytes	= 16,
    .key_len	= 16,
    .Kc_len		= 16,
    .Ke_len		= 16,
    .Ki_len		= 16,
    .block_len	= 16,
    .conf_len	= 16,
    .cksum_len	= 16,
    .hash_len	= 16,
    .prf_len	= 16,
    .keyed_cksum	= true,
    .random_to_key	= core::ptr::null_mut(), /* Identity */
    .profile	= &rfc6803_crypto_profile,
    };
    const struct krb5_enctype krb5_camellia256_cts_cmac = {
    .etype		= KRB5_ENCTYPE_CAMELLIA256_CTS_CMAC,
    .ctype		= KRB5_CKSUMTYPE_CMAC_CAMELLIA256,
    .name		= "camellia256-cts-cmac",
    .encrypt_name	= "krb5enc(cmac(camellia),cts(cbc(camellia)))",
    .cksum_name	= "cmac(camellia)",
    .hash_name	= core::ptr::null_mut(),
    .derivation_enc	= "cts(cbc(camellia))",
    .key_bytes	= 32,
    .key_len	= 32,
    .Kc_len		= 32,
    .Ke_len		= 32,
    .Ki_len		= 32,
    .block_len	= 16,
    .conf_len	= 16,
    .cksum_len	= 16,
    .hash_len	= 16,
    .prf_len	= 16,
    .keyed_cksum	= true,
    .random_to_key	= core::ptr::null_mut(), /* Identity */
    .profile	= &rfc6803_crypto_profile,
    };
