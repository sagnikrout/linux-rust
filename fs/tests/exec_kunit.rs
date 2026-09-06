//! Automatically rewritten from C to Rust
//! Source: fs/tests/exec_kunit.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bprm_stack_limits_result {
    pub bprm: linux_binprm,
    pub expected_rc: c_int,
    pub expected_argmin: c_ulong,
}

    static const struct bprm_stack_limits_result bprm_stack_limits_results[] = {
// Negative argc/envc counts produce -E2BIG
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = INT_MIN, .envc = INT_MIN }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = 5, .envc = -1 }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = -1, .envc = 10 }, .expected_rc = -E2BIG },
// The max value of argc or envc is MAX_ARG_STRINGS.
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = INT_MAX, .envc = INT_MAX }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = MAX_ARG_STRINGS, .envc = MAX_ARG_STRINGS }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = 0, .envc = MAX_ARG_STRINGS }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = MAX_ARG_STRINGS, .envc = 0 }, .expected_rc = -E2BIG },
//
// On 32-bit system these argc and envc counts, while likely impossible
// to represent within the associated TASK_SIZE, could overflow the
// limit calculation, and bypass the ptr_size <= limit check.
//
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = 0x20000001, .envc = 0x20000001 }, .expected_rc = -E2BIG },

// Make sure a pathological bprm->p doesn't cause an overflow.
    { { .p = sizeof(void *), .rlim_stack.rlim_cur = ULONG_MAX,
    .argc = 10, .envc = 10 }, .expected_rc = -E2BIG },

//
// 0 rlim_stack will get raised to ARG_MAX. With 1 string pointer,
// we should see p - ARG_MAX + sizeof(void *).
//
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = 1, .envc = 0 }, .expected_argmin = ULONG_MAX - ARG_MAX + sizeof(void *)},
// Validate that argc is always raised to a minimum of 1.
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = 0, .envc = 0 }, .expected_argmin = ULONG_MAX - ARG_MAX + sizeof(void *)},
//
// 0 rlim_stack will get raised to ARG_MAX. With pointers filling ARG_MAX,
// we should see -E2BIG. (Note argc is always raised to at least 1.)
//
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = ARG_MAX / sizeof(void *), .envc = 0 }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = 0, .envc = ARG_MAX / sizeof(void *) - 1 }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = ARG_MAX / sizeof(void *) + 1, .envc = 0 }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = 0, .envc = ARG_MAX / sizeof(void *) }, .expected_rc = -E2BIG },
// And with one less, we see space for exactly 1 pointer.
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = (ARG_MAX / sizeof(void *)) - 1, .envc = 0 },
    .expected_argmin = ULONG_MAX - sizeof(void *) },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 0,
    .argc = 0, .envc = (ARG_MAX / sizeof(void *)) - 2, },
    .expected_argmin = ULONG_MAX - sizeof(void *) },
// If we raise rlim_stack / 4 to exactly ARG_MAX, nothing changes.
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ARG_MAX * 4,
    .argc = ARG_MAX / sizeof(void *), .envc = 0 }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ARG_MAX * 4,
    .argc = 0, .envc = ARG_MAX / sizeof(void *) - 1 }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ARG_MAX * 4,
    .argc = ARG_MAX / sizeof(void *) + 1, .envc = 0 }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ARG_MAX * 4,
    .argc = 0, .envc = ARG_MAX / sizeof(void *) }, .expected_rc = -E2BIG },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ARG_MAX * 4,
    .argc = (ARG_MAX / sizeof(void *)) - 1, .envc = 0 },
    .expected_argmin = ULONG_MAX - sizeof(void *) },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = ARG_MAX * 4,
    .argc = 0, .envc = (ARG_MAX / sizeof(void *)) - 2, },
    .expected_argmin = ULONG_MAX - sizeof(void *) },
// But raising it another pointer * 4 will provide space for 1 more pointer.
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = (ARG_MAX + sizeof(void *)) * 4,
    .argc = ARG_MAX / sizeof(void *), .envc = 0 },
    .expected_argmin = ULONG_MAX - sizeof(void *) },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = (ARG_MAX + sizeof(void *)) * 4,
    .argc = 0, .envc = ARG_MAX / sizeof(void *) - 1 },
    .expected_argmin = ULONG_MAX - sizeof(void *) },
// Raising rlim_stack / 4 to _STK_LIM / 4 * 3 will see more space.
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 4 * (_STK_LIM / 4 * 3),
    .argc = 0, .envc = 0 },
    .expected_argmin = ULONG_MAX - (_STK_LIM / 4 * 3) + sizeof(void *) },
// But raising it any further will see no increase.
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 4 * (_STK_LIM / 4 * 3 + sizeof(void *)),
    .argc = 0, .envc = 0 },
    .expected_argmin = ULONG_MAX - (_STK_LIM / 4 * 3) + sizeof(void *) },
    { { .p = ULONG_MAX, .rlim_stack.rlim_cur = 4 * _STK_LIM,
    .argc = 0, .envc = 0 },
    .expected_argmin = ULONG_MAX - (_STK_LIM / 4 * 3) + sizeof(void *) },
    };
#[no_mangle]
unsafe extern "C" fn exec_test_bprm_stack_limits(test: *mut kunit) {
    static void exec_test_bprm_stack_limits(struct kunit *test)
    {
// Double-check the constants.
    KUNIT_EXPECT_EQ(test, _STK_LIM, SZ_8M);
    KUNIT_EXPECT_EQ(test, ARG_MAX, 32 * SZ_4K);
    KUNIT_EXPECT_EQ(test, MAX_ARG_STRINGS, 0x7FFFFFFF);
    for (int i = 0; i < ARRAY_SIZE(bprm_stack_limits_results); i++) {
    const struct bprm_stack_limits_result *result = &bprm_stack_limits_results[i];
    let mut bprm: linux_binprm = result.bprm;
    int rc;
    rc = bprm_stack_limits(&bprm);
    KUNIT_EXPECT_EQ_MSG(test, rc, result.expected_rc, "on loop %d", i);

    KUNIT_EXPECT_EQ_MSG(test, bprm.argmin, result.expected_argmin, "on loop %d", i);

    }
    }
    static struct kunit_case exec_test_cases[] = {
    KUNIT_CASE(exec_test_bprm_stack_limits),
    {},
    };
    static struct kunit_suite exec_test_suite = {
    .name = "exec",
    .test_cases = exec_test_cases,
    };
    kunit_test_suite(exec_test_suite);
