//! Automatically rewritten from C to Rust
//! Source: crypto/dh_helper.c
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

    static inline u8 *dh_pack_data(u8 *dst, u8 *end, const void *src, size_t size)
    {
    if (!dst || size > end - dst)
    return core::ptr::null_mut();
    memcpy(dst, src, size);
    return dst + size;
    }
    static inline const u8 *dh_unpack_data(void *dst, const void *src, size_t size)
    {
    memcpy(dst, src, size);
    return src + size;
    }
#[no_mangle]
pub unsafe extern "C" fn dh_data_size(p: *const dh) -> c_uint {
    static inline unsigned int dh_data_size(const struct dh *p)
    {
    return p.key_size + p.p_size + p.g_size;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_dh_key_len(p: *const dh) -> c_uint {
    unsigned int crypto_dh_key_len(const struct dh *p)
    {
    return DH_KPP_SECRET_MIN_SIZE + dh_data_size(p);
    }
    EXPORT_SYMBOL_GPL(crypto_dh_key_len);
#[no_mangle]
pub unsafe extern "C" fn crypto_dh_encode_key(buf: *mut c_char, len: c_uint, params: *const dh) -> c_int {
    int crypto_dh_encode_key(char *buf, unsigned int len, const struct dh *params)
    {
    u8 *ptr = buf;
    let mut end: *mut u8  const = ptr + len;
    struct kpp_secret secret = {
    .type = CRYPTO_KPP_SECRET_TYPE_DH,
    .len = len
    };
    if (unlikely(!len))
    return -EINVAL;
    ptr = dh_pack_data(ptr, end, &secret, sizeof(secret));
    ptr = dh_pack_data(ptr, end, &params.key_size,
    sizeof(params.key_size));
    ptr = dh_pack_data(ptr, end, &params.p_size, sizeof(params.p_size));
    ptr = dh_pack_data(ptr, end, &params.g_size, sizeof(params.g_size));
    ptr = dh_pack_data(ptr, end, params.key, params.key_size);
    ptr = dh_pack_data(ptr, end, params.p, params.p_size);
    ptr = dh_pack_data(ptr, end, params.g, params.g_size);
    if (ptr != end)
    return -EINVAL;
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_dh_encode_key);
#[no_mangle]
pub unsafe extern "C" fn __crypto_dh_decode_key(buf: *const c_char, len: c_uint, params: *mut dh) -> c_int {
    int __crypto_dh_decode_key(const char *buf, unsigned int len, struct dh *params)
    {
    const u8 *ptr = buf;
    struct kpp_secret secret;
    if (unlikely(!buf || len < DH_KPP_SECRET_MIN_SIZE))
    return -EINVAL;
    ptr = dh_unpack_data(&secret, ptr, sizeof(secret));
    if (secret.type != CRYPTO_KPP_SECRET_TYPE_DH)
    return -EINVAL;
    ptr = dh_unpack_data(&params.key_size, ptr, sizeof(params.key_size));
    ptr = dh_unpack_data(&params.p_size, ptr, sizeof(params.p_size));
    ptr = dh_unpack_data(&params.g_size, ptr, sizeof(params.g_size));
    if (secret.len != crypto_dh_key_len(params))
    return -EINVAL;
// Don't allocate memory. Set pointers to data within
// the given buffer
//
    params.key = (void *)ptr;
    params.p = (void *)(ptr + params.key_size);
    params.g = (void *)(ptr + params.key_size + params.p_size);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crypto_dh_decode_key(buf: *const c_char, len: c_uint, params: *mut dh) -> c_int {
    int crypto_dh_decode_key(const char *buf, unsigned int len, struct dh *params)
    {
    int err;
    err = __crypto_dh_decode_key(buf, len, params);
    if (err)
    return err;
//
// Don't permit the buffer for 'key' or 'g' to be larger than 'p', since
// some drivers assume otherwise.
//
    if (params.key_size > params.p_size ||
    params.g_size > params.p_size)
    return -EINVAL;
//
// Don't permit 'p' to be 0.  It's not a prime number, and it's subject
// to corner cases such as 'mod 0' being undefined or
// crypto_kpp_maxsize() returning 0.
//
    if (memchr_inv(params.p, 0, params.p_size) == core::ptr::null_mut())
    return -EINVAL;
    return 0;
    }
    EXPORT_SYMBOL_GPL(crypto_dh_decode_key);
