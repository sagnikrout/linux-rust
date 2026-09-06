//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/string_kfuncs_failure2.c
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
// Copyright (C) 2025 Red Hat, Inc.

    char long_str[XATTR_SIZE_MAX + 1];
    SEC("syscall") int test_strcmp_too_long(void *ctx) { return bpf_strcmp(long_str, long_str); }
    SEC("syscall") int test_strcasecmp_too_long(void *ctx) { return bpf_strcasecmp(long_str, long_str); }
    SEC("syscall") int test_strncasecmp_too_long(void *ctx) { return bpf_strncasecmp(long_str, long_str, sizeof(long_str)); }
    SEC("syscall") int test_strchr_too_long(void *ctx) { return bpf_strchr(long_str, 'b'); }
    SEC("syscall") int test_strchrnul_too_long(void *ctx) { return bpf_strchrnul(long_str, 'b'); }
    SEC("syscall") int test_strnchr_too_long(void *ctx) { return bpf_strnchr(long_str, sizeof(long_str), 'b'); }
    SEC("syscall") int test_strrchr_too_long(void *ctx) { return bpf_strrchr(long_str, 'b'); }
    SEC("syscall") int test_strlen_too_long(void *ctx) { return bpf_strlen(long_str); }
    SEC("syscall") int test_strnlen_too_long(void *ctx) { return bpf_strnlen(long_str, sizeof(long_str)); }
    SEC("syscall") int test_strspn_str_too_long(void *ctx) { return bpf_strspn(long_str, "a"); }
    SEC("syscall") int test_strspn_accept_too_long(void *ctx) { return bpf_strspn("b", long_str); }
    SEC("syscall") int test_strcspn_str_too_long(void *ctx) { return bpf_strcspn(long_str, "b"); }
    SEC("syscall") int test_strcspn_reject_too_long(void *ctx) { return bpf_strcspn("b", long_str); }
    SEC("syscall") int test_strstr_too_long(void *ctx) { return bpf_strstr(long_str, "hello"); }
    SEC("syscall") int test_strcasestr_too_long(void *ctx) { return bpf_strcasestr(long_str, "hello"); }
    SEC("syscall") int test_strnstr_too_long(void *ctx) { return bpf_strnstr(long_str, "hello", sizeof(long_str)); }
    SEC("syscall") int test_strncasestr_too_long(void *ctx) { return bpf_strncasestr(long_str, "hello", sizeof(long_str)); }
    char _license[] SEC("license") = "GPL";
