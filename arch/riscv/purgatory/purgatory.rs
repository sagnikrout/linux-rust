//! Automatically rewritten from C to Rust
//! Source: arch/riscv/purgatory/purgatory.c
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
// purgatory: Runs between two kernels
//
// Copyright (C) 2022 Huawei Technologies Co, Ltd.
//
// Author: Li Zhengyu (lizhengyu3@huawei.com)
//

    u8 purgatory_sha256_digest[SHA256_DIGEST_SIZE] __section(".kexec-purgatory");
    struct kexec_sha_region purgatory_sha_regions[KEXEC_SEGMENT_MAX] __section(".kexec-purgatory");
#[no_mangle]
unsafe extern "C" fn verify_sha256_digest() -> bool {
    static bool verify_sha256_digest(void)
    {
    struct kexec_sha_region *ptr, *end;
    struct sha256_ctx sctx;
    u8 digest[SHA256_DIGEST_SIZE];
    sha256_init(&sctx);
    end = purgatory_sha_regions + ARRAY_SIZE(purgatory_sha_regions);
    for (ptr = purgatory_sha_regions; ptr < end; ptr++)
    sha256_update(&sctx, (const u8 *)(ptr.start), ptr.len);
    sha256_final(&sctx, digest);
    return memcmp(digest, purgatory_sha256_digest, sizeof(digest)) == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn purgatory() {
    void purgatory(void)
    {
    if (!verify_sha256_digest())
    for (;;)
// loop forever
    ;
    }
