//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/gcs/gcs-locking.c
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
// Copyright (C) 2023 ARM Limited.
//
// Tests for GCS mode locking.  These tests rely on both having GCS
// unconfigured on entry and on the kselftest harness running each
// test in a fork()ed process which will have it's own mode.
//

    ({                                                                            \
    register long _num  __asm__ ("x8") = (num);                           \
    register long _arg1 __asm__ ("x0") = (long)(arg1);                    \
    register long _arg2 __asm__ ("x1") = (long)(arg2);                    \
    register long _arg3 __asm__ ("x2") = 0;                               \
    register long _arg4 __asm__ ("x3") = 0;                               \
    register long _arg5 __asm__ ("x4") = 0;                               \
    \
    __asm__  volatile (                                                   \
    "svc #0\n"                                                    \
    : "=r"(_arg1)                                                 \
    : "r"(_arg1), "r"(_arg2),                                     \
    "r"(_arg3), "r"(_arg4),                                     \
    "r"(_arg5), "r"(_num)					      \
    : "memory", "cc"                                              \
    );                                                                    \
    _arg1;                                                                \
    })
// No mode bits are rejected for locking
    TEST(lock_all_modes)
    {
    int ret;
    ret = prctl(PR_LOCK_SHADOW_STACK_STATUS, ULONG_MAX, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    }
    FIXTURE(valid_modes)
    {
    };
    FIXTURE_VARIANT(valid_modes)
    {
    unsigned long mode;
    };
    FIXTURE_VARIANT_ADD(valid_modes, enable)
    {
    .mode = PR_SHADOW_STACK_ENABLE,
    };
    FIXTURE_VARIANT_ADD(valid_modes, enable_write)
    {
    .mode = PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE,
    };
    FIXTURE_VARIANT_ADD(valid_modes, enable_push)
    {
    .mode = PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_PUSH,
    };
    FIXTURE_VARIANT_ADD(valid_modes, enable_write_push)
    {
    .mode = PR_SHADOW_STACK_ENABLE | PR_SHADOW_STACK_WRITE |
    PR_SHADOW_STACK_PUSH,
    };
    FIXTURE_SETUP(valid_modes)
    {
    }
    FIXTURE_TEARDOWN(valid_modes)
    {
    }
// We can set the mode at all
    TEST_F(valid_modes, set)
    {
    int ret;
    ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS,
    variant.mode);
    ASSERT_EQ(ret, 0);
    _exit(0);
    }
// Enabling, locking then disabling is rejected
    TEST_F(valid_modes, enable_lock_disable)
    {
    unsigned long mode;
    int ret;
    ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS,
    variant.mode);
    ASSERT_EQ(ret, 0);
    ret = prctl(PR_GET_SHADOW_STACK_STATUS, &mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(mode, variant.mode);
    ret = prctl(PR_LOCK_SHADOW_STACK_STATUS, variant.mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS, 0);
    ASSERT_EQ(ret, -EBUSY);
    _exit(0);
    }
// Locking then enabling is rejected
    TEST_F(valid_modes, lock_enable)
    {
    unsigned long mode;
    int ret;
    ret = prctl(PR_LOCK_SHADOW_STACK_STATUS, variant.mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS,
    variant.mode);
    ASSERT_EQ(ret, -EBUSY);
    ret = prctl(PR_GET_SHADOW_STACK_STATUS, &mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(mode, 0);
    _exit(0);
    }
// Locking then changing other modes is fine
    TEST_F(valid_modes, lock_enable_disable_others)
    {
    unsigned long mode;
    int ret;
    ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS,
    variant.mode);
    ASSERT_EQ(ret, 0);
    ret = prctl(PR_GET_SHADOW_STACK_STATUS, &mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(mode, variant.mode);
    ret = prctl(PR_LOCK_SHADOW_STACK_STATUS, variant.mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS,
    PR_SHADOW_STACK_ALL_MODES);
    ASSERT_EQ(ret, 0);
    ret = prctl(PR_GET_SHADOW_STACK_STATUS, &mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(mode, PR_SHADOW_STACK_ALL_MODES);
    ret = my_syscall2(__NR_prctl, PR_SET_SHADOW_STACK_STATUS,
    variant.mode);
    ASSERT_EQ(ret, 0);
    ret = prctl(PR_GET_SHADOW_STACK_STATUS, &mode, 0, 0, 0);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(mode, variant.mode);
    _exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    unsigned long mode;
    int ret;
    if (!(getauxval(AT_HWCAP) & HWCAP_GCS))
    ksft_exit_skip("SKIP GCS not supported\n");
    ret = prctl(PR_GET_SHADOW_STACK_STATUS, &mode, 0, 0, 0);
    if (ret) {
    ksft_print_msg("Failed to read GCS state: %d\n", ret);
    return EXIT_FAILURE;
    }
    if (mode & PR_SHADOW_STACK_ENABLE) {
    ksft_print_msg("GCS was enabled, test unsupported\n");
    return KSFT_SKIP;
    }
    return test_harness_run(argc, argv);
    }
