//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cryptouser.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Crypto user configuration API.
//
// Copyright (C) 2011 secunet Security Networks AG
// Copyright (C) 2011 Steffen Klassert <steffen.klassert@secunet.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// This program is distributed in the hope it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin St - Fifth Floor, Boston, MA 02110-1301 USA.
//

// Netlink configuration messages.

pub const CRYPTO_MAX_NAME: c_int = 64;
// Netlink message attributes.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum crypto_attr_type_t {
    CRYPTOCFGA_UNSPEC,
    CRYPTOCFGA_PRIORITY_VAL,	/* __u32 */
    CRYPTOCFGA_REPORT_LARVAL,	/* struct crypto_report_larval */
    CRYPTOCFGA_REPORT_HASH,		/* struct crypto_report_hash */
    CRYPTOCFGA_REPORT_BLKCIPHER,	/* struct crypto_report_blkcipher */
    CRYPTOCFGA_REPORT_AEAD,		/* struct crypto_report_aead */
    CRYPTOCFGA_REPORT_COMPRESS,	/* struct crypto_report_comp */
    CRYPTOCFGA_REPORT_RNG,		/* struct crypto_report_rng */
    CRYPTOCFGA_REPORT_CIPHER,	/* struct crypto_report_cipher */
    CRYPTOCFGA_REPORT_AKCIPHER,	/* struct crypto_report_akcipher */
    CRYPTOCFGA_REPORT_KPP,		/* struct crypto_report_kpp */
    CRYPTOCFGA_REPORT_ACOMP,	/* struct crypto_report_acomp */
    CRYPTOCFGA_STAT_LARVAL,		/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_HASH,		/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_BLKCIPHER,	/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_AEAD,		/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_COMPRESS,	/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_RNG,		/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_CIPHER,		/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_AKCIPHER,	/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_KPP,		/* No longer supported, do not use. */
    CRYPTOCFGA_STAT_ACOMP,		/* No longer supported, do not use. */
    CRYPTOCFGA_REPORT_SIG,		/* struct crypto_report_sig */
    __CRYPTOCFGA_MAX

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_user_alg {
    pub cru_name: [c_char; CRYPTO_MAX_NAME],
    pub cru_driver_name: [c_char; CRYPTO_MAX_NAME],
    pub cru_module_name: [c_char; CRYPTO_MAX_NAME],
    pub cru_type: __u32,
    pub cru_mask: __u32,
    pub cru_refcnt: __u32,
    pub cru_flags: __u32,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_aead {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub stat_encrypt_cnt: __u64,
    pub stat_encrypt_tlen: __u64,
    pub stat_decrypt_cnt: __u64,
    pub stat_decrypt_tlen: __u64,
    pub stat_err_cnt: __u64,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_akcipher {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub stat_encrypt_cnt: __u64,
    pub stat_encrypt_tlen: __u64,
    pub stat_decrypt_cnt: __u64,
    pub stat_decrypt_tlen: __u64,
    pub stat_verify_cnt: __u64,
    pub stat_sign_cnt: __u64,
    pub stat_err_cnt: __u64,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_cipher {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub stat_encrypt_cnt: __u64,
    pub stat_encrypt_tlen: __u64,
    pub stat_decrypt_cnt: __u64,
    pub stat_decrypt_tlen: __u64,
    pub stat_err_cnt: __u64,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_compress {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub stat_compress_cnt: __u64,
    pub stat_compress_tlen: __u64,
    pub stat_decompress_cnt: __u64,
    pub stat_decompress_tlen: __u64,
    pub stat_err_cnt: __u64,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_hash {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub stat_hash_cnt: __u64,
    pub stat_hash_tlen: __u64,
    pub stat_err_cnt: __u64,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_kpp {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub stat_setsecret_cnt: __u64,
    pub stat_generate_public_key_cnt: __u64,
    pub stat_compute_shared_secret_cnt: __u64,
    pub stat_err_cnt: __u64,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_rng {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub stat_generate_cnt: __u64,
    pub stat_generate_tlen: __u64,
    pub stat_seed_cnt: __u64,
    pub stat_err_cnt: __u64,
}

// No longer supported, do not use.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_stat_larval {
    pub type: [c_char; CRYPTO_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_larval {
    pub type: [c_char; CRYPTO_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_hash {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub blocksize: c_uint,
    pub digestsize: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_cipher {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub blocksize: c_uint,
    pub min_keysize: c_uint,
    pub max_keysize: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_blkcipher {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub geniv: [c_char; CRYPTO_MAX_NAME],
    pub blocksize: c_uint,
    pub min_keysize: c_uint,
    pub max_keysize: c_uint,
    pub ivsize: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_aead {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub geniv: [c_char; CRYPTO_MAX_NAME],
    pub blocksize: c_uint,
    pub maxauthsize: c_uint,
    pub ivsize: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_comp {
    pub type: [c_char; CRYPTO_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_rng {
    pub type: [c_char; CRYPTO_MAX_NAME],
    pub seedsize: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_akcipher {
    pub type: [c_char; CRYPTO_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_kpp {
    pub type: [c_char; CRYPTO_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_acomp {
    pub type: [c_char; CRYPTO_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_report_sig {
    pub type: [c_char; CRYPTO_MAX_NAME],
}

