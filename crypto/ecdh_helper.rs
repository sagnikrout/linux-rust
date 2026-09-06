//! Automatically rewritten from C to Rust
//! Source: crypto/ecdh_helper.c
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
// Copyright (c) 2016, Intel Corporation
// Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
//

    static inline u8 *ecdh_pack_data(void *dst, const void *src, size_t sz)
    {
    memcpy(dst, src, sz);
    return dst + sz;
    }
    static inline const u8 *ecdh_unpack_data(void *dst, const void *src, size_t sz)
    {
    memcpy(dst, src, sz);
    return src + sz;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_ecdh_key_len(params: *const ecdh) -> c_uint {
    unsigned int crypto_ecdh_key_len(const struct ecdh *params)
    {
    return ECDH_KPP_SECRET_MIN_SIZE + params.key_size;
    }
    EXPORT_SYMBOL_GPL(crypto_ecdh_key_len);
    int crypto_ecdh_encode_key(char *buf, unsigned int len,
    const struct ecdh *params)
    {
    u8 *ptr = buf;
    struct kpp_secret secret = {
    .type = CRYPTO_KPP_SECRET_TYPE_ECDH,
    .len = len
    };
    if (unlikely(!buf))
    return -EINVAL;
    if (len != crypto_ecdh_key_len(params))
    return -EINVAL;
    ptr = ecdh_pack_data(ptr, &secret, sizeof(secret));
    ptr = ecdh_pack_data(ptr, &params.key_size, sizeof(params.key_size));
    ecdh_pack_data(ptr, params.key, params.key_size);
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_ecdh_encode_key);
    int crypto_ecdh_decode_key(const char *buf, unsigned int len,
    struct ecdh *params)
    {
    const u8 *ptr = buf;
    struct kpp_secret secret;
    if (unlikely(!buf || len < ECDH_KPP_SECRET_MIN_SIZE))
    return -EINVAL;
    ptr = ecdh_unpack_data(&secret, ptr, sizeof(secret));
    if (secret.type != CRYPTO_KPP_SECRET_TYPE_ECDH)
    return -EINVAL;
    if (unlikely(len < secret.len))
    return -EINVAL;
    ptr = ecdh_unpack_data(&params.key_size, ptr, sizeof(params.key_size));
    if (secret.len != crypto_ecdh_key_len(params))
    return -EINVAL;
// Don't allocate memory. Set pointer to data
// within the given buffer
//
    params.key = (void *)ptr;
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_ecdh_decode_key);
