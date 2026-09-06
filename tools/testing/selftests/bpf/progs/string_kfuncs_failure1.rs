//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/string_kfuncs_failure1.c
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

    char *user_ptr = (char *)1;
    char *invalid_kern_ptr = (char *)-1;
//
// When passing userspace pointers, the error code differs based on arch:
// -ERANGE on arches with non-overlapping address spaces
// -EFAULT on other arches
//

    defined(__TARGET_ARCH_powerpc) || defined(__TARGET_ARCH_x86)

//
// On s390, __get_kernel_nofault (used in string kfuncs) returns 0 for NULL and
// user_ptr (instead of causing an exception) so the below two groups of tests
// are not applicable.
//

// Passing NULL to string kfuncs (treated as a userspace ptr)
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcmp_null1(void *ctx) { return bpf_strcmp(core::ptr::null_mut(), "hello"); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strcmp_null2(void *ctx) { return bpf_strcmp("hello", core::ptr::null_mut()); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcasecmp_null1(void *ctx) { return bpf_strcasecmp(core::ptr::null_mut(), "HELLO"); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strcasecmp_null2(void *ctx) { return bpf_strcasecmp("HELLO", core::ptr::null_mut()); }
    SEC("syscall") __retval(USER_PTR_ERR)int test_strncasecmp_null1(void *ctx) { return bpf_strncasecmp(core::ptr::null_mut(), "HELLO", 5); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strncasecmp_null2(void *ctx) { return bpf_strncasecmp("HELLO", core::ptr::null_mut(), 5);	 }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strchr_null(void *ctx) { return bpf_strchr(core::ptr::null_mut(), 'a'); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strchrnul_null(void *ctx) { return bpf_strchrnul(core::ptr::null_mut(), 'a'); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strnchr_null(void *ctx) { return bpf_strnchr(core::ptr::null_mut(), 1, 'a'); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strrchr_null(void *ctx) { return bpf_strrchr(core::ptr::null_mut(), 'a'); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strlen_null(void *ctx) { return bpf_strlen(core::ptr::null_mut()); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strnlen_null(void *ctx) { return bpf_strnlen(core::ptr::null_mut(), 1); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strspn_null1(void *ctx) { return bpf_strspn(core::ptr::null_mut(), "hello"); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strspn_null2(void *ctx) { return bpf_strspn("hello", core::ptr::null_mut()); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strcspn_null1(void *ctx) { return bpf_strcspn(core::ptr::null_mut(), "hello"); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strcspn_null2(void *ctx) { return bpf_strcspn("hello", core::ptr::null_mut()); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strstr_null1(void *ctx) { return bpf_strstr(core::ptr::null_mut(), "hello"); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strstr_null2(void *ctx) { return bpf_strstr("hello", core::ptr::null_mut()); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strcasestr_null1(void *ctx) { return bpf_strcasestr(core::ptr::null_mut(), "hello"); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strcasestr_null2(void *ctx) { return bpf_strcasestr("hello", core::ptr::null_mut()); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strnstr_null1(void *ctx) { return bpf_strnstr(core::ptr::null_mut(), "hello", 1); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strnstr_null2(void *ctx) { return bpf_strnstr("hello", core::ptr::null_mut(), 1); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strncasestr_null1(void *ctx) { return bpf_strncasestr(core::ptr::null_mut(), "hello", 1); }
    SEC("syscall")  __retval(USER_PTR_ERR)int test_strncasestr_null2(void *ctx) { return bpf_strncasestr("hello", core::ptr::null_mut(), 1); }
// Passing userspace ptr to string kfuncs
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcmp_user_ptr1(void *ctx) { return bpf_strcmp(user_ptr, "hello"); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcmp_user_ptr2(void *ctx) { return bpf_strcmp("hello", user_ptr); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcasecmp_user_ptr1(void *ctx) { return bpf_strcasecmp(user_ptr, "HELLO"); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcasecmp_user_ptr2(void *ctx) { return bpf_strcasecmp("HELLO", user_ptr); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strncasecmp_user_ptr1(void *ctx) { return bpf_strncasecmp(user_ptr, "HELLO", 5); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strncasecmp_user_ptr2(void *ctx) { return bpf_strncasecmp("HELLO", user_ptr, 5);	 }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strchr_user_ptr(void *ctx) { return bpf_strchr(user_ptr, 'a'); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strchrnul_user_ptr(void *ctx) { return bpf_strchrnul(user_ptr, 'a'); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strnchr_user_ptr(void *ctx) { return bpf_strnchr(user_ptr, 1, 'a'); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strrchr_user_ptr(void *ctx) { return bpf_strrchr(user_ptr, 'a'); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strlen_user_ptr(void *ctx) { return bpf_strlen(user_ptr); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strnlen_user_ptr(void *ctx) { return bpf_strnlen(user_ptr, 1); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strspn_user_ptr1(void *ctx) { return bpf_strspn(user_ptr, "hello"); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strspn_user_ptr2(void *ctx) { return bpf_strspn("hello", user_ptr); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcspn_user_ptr1(void *ctx) { return bpf_strcspn(user_ptr, "hello"); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcspn_user_ptr2(void *ctx) { return bpf_strcspn("hello", user_ptr); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strstr_user_ptr1(void *ctx) { return bpf_strstr(user_ptr, "hello"); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strstr_user_ptr2(void *ctx) { return bpf_strstr("hello", user_ptr); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcasestr_user_ptr1(void *ctx) { return bpf_strcasestr(user_ptr, "hello"); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strcasestr_user_ptr2(void *ctx) { return bpf_strcasestr("hello", user_ptr); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strnstr_user_ptr1(void *ctx) { return bpf_strnstr(user_ptr, "hello", 1); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strnstr_user_ptr2(void *ctx) { return bpf_strnstr("hello", user_ptr, 1); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strncasestr_user_ptr1(void *ctx) { return bpf_strncasestr(user_ptr, "hello", 1); }
    SEC("syscall") __retval(USER_PTR_ERR) int test_strncasestr_user_ptr2(void *ctx) { return bpf_strncasestr("hello", user_ptr, 1); }

// Passing invalid kernel ptr to string kfuncs should always return -EFAULT
    SEC("syscall") __retval(-EFAULT) int test_strcmp_pagefault1(void *ctx) { return bpf_strcmp(invalid_kern_ptr, "hello"); }
    SEC("syscall") __retval(-EFAULT) int test_strcmp_pagefault2(void *ctx) { return bpf_strcmp("hello", invalid_kern_ptr); }
    SEC("syscall") __retval(-EFAULT) int test_strcasecmp_pagefault1(void *ctx) { return bpf_strcasecmp(invalid_kern_ptr, "HELLO"); }
    SEC("syscall") __retval(-EFAULT) int test_strcasecmp_pagefault2(void *ctx) { return bpf_strcasecmp("HELLO", invalid_kern_ptr); }
    SEC("syscall") __retval(-EFAULT) int test_strncasecmp_pagefault1(void *ctx) { return bpf_strncasecmp(invalid_kern_ptr, "HELLO", 5); }
    SEC("syscall") __retval(-EFAULT) int test_strncasecmp_pagefault2(void *ctx) { return bpf_strncasecmp("HELLO", invalid_kern_ptr, 5);	 }
    SEC("syscall") __retval(-EFAULT) int test_strchr_pagefault(void *ctx) { return bpf_strchr(invalid_kern_ptr, 'a'); }
    SEC("syscall") __retval(-EFAULT) int test_strchrnul_pagefault(void *ctx) { return bpf_strchrnul(invalid_kern_ptr, 'a'); }
    SEC("syscall") __retval(-EFAULT) int test_strnchr_pagefault(void *ctx) { return bpf_strnchr(invalid_kern_ptr, 1, 'a'); }
    SEC("syscall") __retval(-EFAULT) int test_strrchr_pagefault(void *ctx) { return bpf_strrchr(invalid_kern_ptr, 'a'); }
    SEC("syscall") __retval(-EFAULT) int test_strlen_pagefault(void *ctx) { return bpf_strlen(invalid_kern_ptr); }
    SEC("syscall") __retval(-EFAULT) int test_strnlen_pagefault(void *ctx) { return bpf_strnlen(invalid_kern_ptr, 1); }
    SEC("syscall") __retval(-EFAULT) int test_strspn_pagefault1(void *ctx) { return bpf_strspn(invalid_kern_ptr, "hello"); }
    SEC("syscall") __retval(-EFAULT) int test_strspn_pagefault2(void *ctx) { return bpf_strspn("hello", invalid_kern_ptr); }
    SEC("syscall") __retval(-EFAULT) int test_strcspn_pagefault1(void *ctx) { return bpf_strcspn(invalid_kern_ptr, "hello"); }
    SEC("syscall") __retval(-EFAULT) int test_strcspn_pagefault2(void *ctx) { return bpf_strcspn("hello", invalid_kern_ptr); }
    SEC("syscall") __retval(-EFAULT) int test_strstr_pagefault1(void *ctx) { return bpf_strstr(invalid_kern_ptr, "hello"); }
    SEC("syscall") __retval(-EFAULT) int test_strstr_pagefault2(void *ctx) { return bpf_strstr("hello", invalid_kern_ptr); }
    SEC("syscall") __retval(-EFAULT) int test_strcasestr_pagefault1(void *ctx) { return bpf_strcasestr(invalid_kern_ptr, "hello"); }
    SEC("syscall") __retval(-EFAULT) int test_strcasestr_pagefault2(void *ctx) { return bpf_strcasestr("hello", invalid_kern_ptr); }
    SEC("syscall") __retval(-EFAULT) int test_strnstr_pagefault1(void *ctx) { return bpf_strnstr(invalid_kern_ptr, "hello", 1); }
    SEC("syscall") __retval(-EFAULT) int test_strnstr_pagefault2(void *ctx) { return bpf_strnstr("hello", invalid_kern_ptr, 1); }
    SEC("syscall") __retval(-EFAULT) int test_strncasestr_pagefault1(void *ctx) { return bpf_strncasestr(invalid_kern_ptr, "hello", 1); }
    SEC("syscall") __retval(-EFAULT) int test_strncasestr_pagefault2(void *ctx) { return bpf_strncasestr("hello", invalid_kern_ptr, 1); }
    char _license[] SEC("license") = "GPL";
