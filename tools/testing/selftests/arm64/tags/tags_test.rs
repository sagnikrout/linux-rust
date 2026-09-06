//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/tags/tags_test.c
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

    SHIFT_TAG(tag))
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut tbi_enabled: static int = 0;
    let mut tag: c_ulong = 0;
    struct utsname *ptr;
    ksft_print_header();
    ksft_set_plan(1);
    if (prctl(PR_SET_TAGGED_ADDR_CTRL, PR_TAGGED_ADDR_ENABLE, 0, 0, 0) == 0)
    tbi_enabled = 1;
    ptr = (struct utsname *)malloc(sizeof(*ptr));
    if (!ptr)
    ksft_exit_fail_perror("Failed to allocate utsname buffer");
    if (tbi_enabled)
    tag = 0x42;
    ptr = (struct utsname *)SET_TAG(ptr, tag);
    ksft_test_result(!uname(ptr), "Syscall successful with tagged address\n");
    free(ptr);
    ksft_finished();
    }
