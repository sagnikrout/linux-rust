//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sha256.c
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
// Copyright 2025 Google LLC

pub const MAX_LEN: c_int = 4096;
// Test libbpf_sha256() for all lengths from 0 to MAX_LEN inclusively.
#[no_mangle]
pub unsafe extern "C" fn test_sha256() {
    void test_sha256(void)
    {
//
// The correctness of this value was verified by running this test with
// libbpf_sha256() replaced by OpenSSL's SHA256().
//
    static const __u8 expected_digest_of_digests[SHA256_DIGEST_LENGTH] = {
    0x62, 0x30, 0x0e, 0x1d, 0xea, 0x7f, 0xc4, 0x74,
    0xfd, 0x8e, 0x64, 0x0b, 0xd8, 0x5f, 0xea, 0x04,
    0xf3, 0xef, 0x77, 0x42, 0xc2, 0x01, 0xb8, 0x90,
    0x6e, 0x19, 0x91, 0x1b, 0xca, 0xb3, 0x28, 0x42,
    };
    let mut seed: __u64 = 0;
    __u8 *data = core::ptr::null_mut(), *digests = core::ptr::null_mut();
    __u8 digest_of_digests[SHA256_DIGEST_LENGTH];
    size_t i;
    data = malloc(MAX_LEN);
    if (!ASSERT_NEQ(data, core::ptr::null_mut(), "malloc"))
    goto out;
    digests = malloc((MAX_LEN + 1) * SHA256_DIGEST_LENGTH);
    if (!ASSERT_NEQ(digests, core::ptr::null_mut(), "malloc"))
    goto out;
// Generate MAX_LEN bytes of "random" data deterministically.
    for (i = 0; i < MAX_LEN; i++) {
    seed = (seed * 25214903917 + 11) & ((1ULL << 48) - 1);
    data[i] = (__u8)(seed >> 16);
    }
// Calculate a digest for each length 0 through MAX_LEN inclusively.
    for (i = 0; i <= MAX_LEN; i++)
    libbpf_sha256(data, i, &digests[i * SHA256_DIGEST_LENGTH]);
// Calculate and verify the digest of all the digests.
    libbpf_sha256(digests, (MAX_LEN + 1) * SHA256_DIGEST_LENGTH,
    digest_of_digests);
    ASSERT_MEMEQ(digest_of_digests, expected_digest_of_digests,
    SHA256_DIGEST_LENGTH, "digest_of_digests");
    out:
    free(data);
    free(digests);
    }
