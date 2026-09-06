//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/probe_read_user_str.c
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

    static const char str1[] = "mestring";
    static const char str2[] = "mestringalittlebigger";
    static const char str3[] = "mestringblubblubblubblubblub";
    static int test_one_str(struct test_probe_read_user_str *skel, const char *str,
    size_t len)
    {
    int err, duration = 0;
    char buf[256];
// Ensure bytes after string are ones
    memset(buf, 1, sizeof(buf));
    memcpy(buf, str, len);
// Give prog our userspace pointer
    skel.bss.user_ptr = buf;
// Trigger tracepoint
    usleep(1);
// Did helper fail?
    if (CHECK(skel.bss.ret < 0, "prog_ret", "prog returned: %ld\n",
    skel.bss.ret))
    return 1;
// Check that string was copied correctly
    err = memcmp(skel.bss.buf, str, len);
    if (CHECK(err, "memcmp", "prog copied wrong string"))
    return 1;
// Now check that no extra trailing bytes were copied
    memset(buf, 0, sizeof(buf));
    err = memcmp(skel.bss.buf + len, buf, sizeof(buf) - len);
    if (CHECK(err, "memcmp", "trailing bytes were not stripped"))
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_probe_read_user_str() {
    void test_probe_read_user_str(void)
    {
    struct test_probe_read_user_str *skel;
    int err, duration = 0;
    skel = test_probe_read_user_str__open_and_load();
    if (CHECK(!skel, "test_probe_read_user_str__open_and_load",
    "skeleton open and load failed\n"))
    return;
// Give pid to bpf prog so it doesn't read from anyone else
    skel.bss.pid = getpid();
    err = test_probe_read_user_str__attach(skel);
    if (CHECK(err, "test_probe_read_user_str__attach",
    "skeleton attach failed: %d\n", err))
    goto out;
    if (test_one_str(skel, str1, sizeof(str1)))
    goto out;
    if (test_one_str(skel, str2, sizeof(str2)))
    goto out;
    if (test_one_str(skel, str3, sizeof(str3)))
    goto out;
    out:
    test_probe_read_user_str__destroy(skel);
    }
