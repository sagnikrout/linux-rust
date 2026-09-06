//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/endian.c
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
// Copyright (c) 2020 Facebook

    static int duration;
pub const IN16: c_uint = 0x1234;
pub const IN32: c_uint = 0x12345678U;
pub const IN64: c_uint = 0x123456789abcdef0ULL;
pub const OUT16: c_uint = 0x3412;
pub const OUT32: c_uint = 0x78563412U;
pub const OUT64: c_uint = 0xf0debc9a78563412ULL;
#[no_mangle]
pub unsafe extern "C" fn test_endian() {
    void test_endian(void)
    {
    struct test_endian* skel;
    struct test_endian__bss *bss;
    int err;
    skel = test_endian__open_and_load();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    return;
    bss = skel.bss;
    bss.in16 = IN16;
    bss.in32 = IN32;
    bss.in64 = IN64;
    err = test_endian__attach(skel);
    if (CHECK(err, "skel_attach", "skeleton attach failed: %d\n", err))
    goto cleanup;
    usleep(1);
    CHECK(bss.out16 != OUT16, "out16", "got 0x%llx != exp 0x%llx\n",
    (__u64)bss.out16, (__u64)OUT16);
    CHECK(bss.out32 != OUT32, "out32", "got 0x%llx != exp 0x%llx\n",
    (__u64)bss.out32, (__u64)OUT32);
    CHECK(bss.out64 != OUT64, "out16", "got 0x%llx != exp 0x%llx\n",
    (__u64)bss.out64, (__u64)OUT64);
    CHECK(bss.const16 != OUT16, "const16", "got 0x%llx != exp 0x%llx\n",
    (__u64)bss.const16, (__u64)OUT16);
    CHECK(bss.const32 != OUT32, "const32", "got 0x%llx != exp 0x%llx\n",
    (__u64)bss.const32, (__u64)OUT32);
    CHECK(bss.const64 != OUT64, "const64", "got 0x%llx != exp 0x%llx\n",
    (__u64)bss.const64, (__u64)OUT64);
    cleanup:
    test_endian__destroy(skel);
    }
