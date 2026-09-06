//! Automatically rewritten from C to Rust
//! Source: arch/s390/purgatory/purgatory.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Purgatory code running between two kernels.
//
// Copyright IBM Corp. 2018
//
// Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
//

#[no_mangle]
pub unsafe extern "C" fn verify_sha256_digest() -> c_int {
    int verify_sha256_digest(void)
    {
    struct kexec_sha_region *ptr, *end;
    u8 digest[SHA256_DIGEST_SIZE];
    struct sha256_ctx sctx;
    sha256_init(&sctx);
    end = purgatory_sha_regions + ARRAY_SIZE(purgatory_sha_regions);
    for (ptr = purgatory_sha_regions; ptr < end; ptr++)
    sha256_update(&sctx, (uint8_t *)(ptr.start), ptr.len);
    sha256_final(&sctx, digest);
    if (memcmp(digest, purgatory_sha256_digest, sizeof(digest)))
    return 1;
    return 0;
    }
