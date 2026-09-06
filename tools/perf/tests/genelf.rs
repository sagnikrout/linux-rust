//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/genelf.c
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

    static int test__jit_write_elf(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {

    static unsigned char x86_code[] = {
    0xBB, 0x2A, 0x00, 0x00, 0x00, /* movl $42, %ebx */
    0xB8, 0x01, 0x00, 0x00, 0x00, /* movl $1, %eax */
    0xCD, 0x80            /* int $0x80 */
    };
    char path[PATH_MAX];
    int fd, ret;
    strcpy(path, TEMPL);
    fd = mkstemp(path);
    if (fd < 0) {
    perror("mkstemp failed");
    return TEST_FAIL;
    }
    pr_info("Writing jit code to: %s\n", path);
    ret = jit_write_elf(fd, 0, "main", x86_code, sizeof(x86_code),
    core::ptr::null_mut(), 0, core::ptr::null_mut(), 0, 0);
    close(fd);
    unlink(path);
    return ret ? TEST_FAIL : 0;

    return TEST_SKIP;

    }
    DEFINE_SUITE("Test jit_write_elf", jit_write_elf);
