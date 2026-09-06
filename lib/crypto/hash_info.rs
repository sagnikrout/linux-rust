//! Automatically rewritten from C to Rust
//! Source: lib/crypto/hash_info.c
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
// Hash Info: Hash algorithms information
//
// Copyright (c) 2013 Dmitry Kasatkin <d.kasatkin@samsung.com>
//

    const char *const hash_algo_name[HASH_ALGO__LAST] = {
    [HASH_ALGO_MD4]		= "md4",
    [HASH_ALGO_MD5]		= "md5",
    [HASH_ALGO_SHA1]	= "sha1",
    [HASH_ALGO_RIPE_MD_160]	= "rmd160",
    [HASH_ALGO_SHA256]	= "sha256",
    [HASH_ALGO_SHA384]	= "sha384",
    [HASH_ALGO_SHA512]	= "sha512",
    [HASH_ALGO_SHA224]	= "sha224",
    [HASH_ALGO_RIPE_MD_128]	= "rmd128",
    [HASH_ALGO_RIPE_MD_256]	= "rmd256",
    [HASH_ALGO_RIPE_MD_320]	= "rmd320",
    [HASH_ALGO_WP_256]	= "wp256",
    [HASH_ALGO_WP_384]	= "wp384",
    [HASH_ALGO_WP_512]	= "wp512",
    [HASH_ALGO_TGR_128]	= "tgr128",
    [HASH_ALGO_TGR_160]	= "tgr160",
    [HASH_ALGO_TGR_192]	= "tgr192",
    [HASH_ALGO_SM3_256]	= "sm3",
    [HASH_ALGO_STREEBOG_256] = "streebog256",
    [HASH_ALGO_STREEBOG_512] = "streebog512",
    [HASH_ALGO_SHA3_256]    = "sha3-256",
    [HASH_ALGO_SHA3_384]    = "sha3-384",
    [HASH_ALGO_SHA3_512]    = "sha3-512",
    };
    EXPORT_SYMBOL_GPL(hash_algo_name);
    const int hash_digest_size[HASH_ALGO__LAST] = {
    [HASH_ALGO_MD4]		= MD5_DIGEST_SIZE,
    [HASH_ALGO_MD5]		= MD5_DIGEST_SIZE,
    [HASH_ALGO_SHA1]	= SHA1_DIGEST_SIZE,
    [HASH_ALGO_RIPE_MD_160]	= RMD160_DIGEST_SIZE,
    [HASH_ALGO_SHA256]	= SHA256_DIGEST_SIZE,
    [HASH_ALGO_SHA384]	= SHA384_DIGEST_SIZE,
    [HASH_ALGO_SHA512]	= SHA512_DIGEST_SIZE,
    [HASH_ALGO_SHA224]	= SHA224_DIGEST_SIZE,
    [HASH_ALGO_RIPE_MD_128]	= RMD128_DIGEST_SIZE,
    [HASH_ALGO_RIPE_MD_256]	= RMD256_DIGEST_SIZE,
    [HASH_ALGO_RIPE_MD_320]	= RMD320_DIGEST_SIZE,
    [HASH_ALGO_WP_256]	= WP256_DIGEST_SIZE,
    [HASH_ALGO_WP_384]	= WP384_DIGEST_SIZE,
    [HASH_ALGO_WP_512]	= WP512_DIGEST_SIZE,
    [HASH_ALGO_TGR_128]	= TGR128_DIGEST_SIZE,
    [HASH_ALGO_TGR_160]	= TGR160_DIGEST_SIZE,
    [HASH_ALGO_TGR_192]	= TGR192_DIGEST_SIZE,
    [HASH_ALGO_SM3_256]	= SM3256_DIGEST_SIZE,
    [HASH_ALGO_STREEBOG_256] = STREEBOG256_DIGEST_SIZE,
    [HASH_ALGO_STREEBOG_512] = STREEBOG512_DIGEST_SIZE,
    [HASH_ALGO_SHA3_256]    = SHA3_256_DIGEST_SIZE,
    [HASH_ALGO_SHA3_384]    = SHA3_384_DIGEST_SIZE,
    [HASH_ALGO_SHA3_512]    = SHA3_512_DIGEST_SIZE,
    };
    EXPORT_SYMBOL_GPL(hash_digest_size);
