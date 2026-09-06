//! Automatically rewritten from C to Rust
//! Source: kernel/events/hw_breakpoint_test.c
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
//
// KUnit test for hw_breakpoint constraints accounting logic.
//
// Copyright (C) 2022, Google LLC.
//

    do {										\
    if ((slots) > get_test_bp_slots()) {					\
    kunit_skip((test), "Requires breakpoint slots: %d > %d", slots,	\
    get_test_bp_slots());				\
    }									\
    } while (0)

pub const MAX_TEST_BREAKPOINTS: c_int = 512;
    static char break_vars[MAX_TEST_BREAKPOINTS];
    static struct perf_event *test_bps[MAX_TEST_BREAKPOINTS];
    static struct task_struct *__other_task;
    static struct perf_event *register_test_bp(int cpu, struct task_struct *tsk, int idx)
    {
    let mut attr: perf_event_attr = {};
    if (WARN_ON(idx < 0 || idx >= MAX_TEST_BREAKPOINTS))
    return core::ptr::null_mut();
    hw_breakpoint_init(&attr);
    attr.bp_addr = (unsigned long)&break_vars[idx];
    attr.bp_len = HW_BREAKPOINT_LEN_1;
    attr.bp_type = HW_BREAKPOINT_RW;
    return perf_event_create_kernel_counter(&attr, cpu, tsk, core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn unregister_test_bp(bp: *mut perf_event) {
    static void unregister_test_bp(struct perf_event **bp)
    {
    if (WARN_ON(IS_ERR(*bp)))
    return;
    if (WARN_ON(!*bp))
    return;
    unregister_hw_breakpoint(*bp);
// bp = NULL;
    }
#[no_mangle]
unsafe extern "C" fn get_test_bp_slots() -> c_int {
    static int get_test_bp_slots(void)
    {
    static int slots;
    if (!slots)
    slots = hw_breakpoint_slots(TYPE_DATA);
    return slots;
    }
#[no_mangle]
unsafe extern "C" fn fill_one_bp_slot(test: *mut kunit, id: *mut c_int, cpu: c_int, tsk: *mut task_struct) {
    static void fill_one_bp_slot(struct kunit *test, int *id, int cpu, struct task_struct *tsk)
    {
    struct perf_event *bp = register_test_bp(cpu, tsk, *id);
    KUNIT_ASSERT_NOT_NULL(test, bp);
    KUNIT_ASSERT_FALSE(test, IS_ERR(bp));
    KUNIT_ASSERT_NULL(test, test_bps[*id]);
    test_bps[(*id)++] = bp;
    }
//
// Fills up the given @cpu/@tsk with breakpoints, only leaving @skip slots free.
//
// Returns true if this can be called again, continuing at @id.
//
#[no_mangle]
unsafe extern "C" fn fill_bp_slots(test: *mut kunit, id: *mut c_int, cpu: c_int, tsk: *mut task_struct, skip: c_int) -> bool {
    static bool fill_bp_slots(struct kunit *test, int *id, int cpu, struct task_struct *tsk, int skip)
    {
    for (int i = 0; i < get_test_bp_slots() - skip; ++i)
    fill_one_bp_slot(test, id, cpu, tsk);
    return *id + get_test_bp_slots() <= MAX_TEST_BREAKPOINTS;
    }
#[no_mangle]
unsafe extern "C" fn dummy_kthread(arg: *mut c_void) -> c_int {
    static int dummy_kthread(void *arg)
    {
    return 0;
    }
    static struct task_struct *get_other_task(struct kunit *test)
    {
    struct task_struct *tsk;
    if (__other_task)
    return __other_task;
    tsk = kthread_create(dummy_kthread, core::ptr::null_mut(), "hw_breakpoint_dummy_task");
    KUNIT_ASSERT_FALSE(test, IS_ERR(tsk));
    __other_task = tsk;
    return __other_task;
    }
#[no_mangle]
unsafe extern "C" fn get_test_cpu(num: c_int) -> c_int {
    static int get_test_cpu(int num)
    {
    int cpu;
    WARN_ON(num < 0);
    for_each_online_cpu(cpu) {
    if (num-- <= 0)
    break;
    }
    return cpu;
    }
// ===== Test cases =====
#[no_mangle]
unsafe extern "C" fn test_one_cpu(test: *mut kunit) {
    static void test_one_cpu(struct kunit *test)
    {
    let mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), core::ptr::null_mut(), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_many_cpus(test: *mut kunit) {
    static void test_many_cpus(struct kunit *test)
    {
    let mut idx: c_int = 0;
    int cpu;
// Test that CPUs are independent.
    for_each_online_cpu(cpu) {
    let mut do_continue: bool = fill_bp_slots(test, &idx, cpu, core::ptr::null_mut(), 0);
    TEST_EXPECT_NOSPC(register_test_bp(cpu, core::ptr::null_mut(), idx));
    if (!do_continue)
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn test_one_task_on_all_cpus(test: *mut kunit) {
    static void test_one_task_on_all_cpus(struct kunit *test)
    {
    let mut idx: c_int = 0;
    fill_bp_slots(test, &idx, -1, current, 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Remove one and adding back CPU-target should work.
    unregister_test_bp(&test_bps[0]);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_two_tasks_on_all_cpus(test: *mut kunit) {
    static void test_two_tasks_on_all_cpus(struct kunit *test)
    {
    let mut idx: c_int = 0;
// Test that tasks are independent.
    fill_bp_slots(test, &idx, -1, current, 0);
    fill_bp_slots(test, &idx, -1, get_other_task(test), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(-1, get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Remove one from first task and adding back CPU-target should not work.
    unregister_test_bp(&test_bps[0]);
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_one_task_on_one_cpu(test: *mut kunit) {
    static void test_one_task_on_one_cpu(struct kunit *test)
    {
    let mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), current, 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
//
// Remove one and adding back CPU-target should work; this case is
// special vs. above because the task's constraints are CPU-dependent.
//
    unregister_test_bp(&test_bps[0]);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_one_task_mixed(test: *mut kunit) {
    static void test_one_task_mixed(struct kunit *test)
    {
    let mut idx: c_int = 0;
    TEST_REQUIRES_BP_SLOTS(test, 3);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), current);
    fill_bp_slots(test, &idx, -1, current, 1);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Transition from CPU-dependent pinned count to CPU-independent.
    unregister_test_bp(&test_bps[0]);
    unregister_test_bp(&test_bps[1]);
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    fill_one_bp_slot(test, &idx, get_test_cpu(0), core::ptr::null_mut());
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_two_tasks_on_one_cpu(test: *mut kunit) {
    static void test_two_tasks_on_one_cpu(struct kunit *test)
    {
    let mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), current, 0);
    fill_bp_slots(test, &idx, get_test_cpu(0), get_other_task(test), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(-1, get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Can still create breakpoints on some other CPU.
    fill_bp_slots(test, &idx, get_test_cpu(1), core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn test_two_tasks_on_one_all_cpus(test: *mut kunit) {
    static void test_two_tasks_on_one_all_cpus(struct kunit *test)
    {
    let mut idx: c_int = 0;
    fill_bp_slots(test, &idx, get_test_cpu(0), current, 0);
    fill_bp_slots(test, &idx, -1, get_other_task(test), 0);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(-1, get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), get_other_task(test), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// Cannot create breakpoints on some other CPU either.
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx));
    }
#[no_mangle]
unsafe extern "C" fn test_task_on_all_and_one_cpu(test: *mut kunit) {
    static void test_task_on_all_and_one_cpu(struct kunit *test)
    {
    int tsk_on_cpu_idx, cpu_idx;
    let mut idx: c_int = 0;
    TEST_REQUIRES_BP_SLOTS(test, 3);
    fill_bp_slots(test, &idx, -1, current, 2);
// Transitioning from only all CPU breakpoints to mixed.
    tsk_on_cpu_idx = idx;
    fill_one_bp_slot(test, &idx, get_test_cpu(0), current);
    fill_one_bp_slot(test, &idx, -1, current);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
// We should still be able to use up another CPU's slots.
    cpu_idx = idx;
    fill_one_bp_slot(test, &idx, get_test_cpu(1), core::ptr::null_mut());
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx));
// Transitioning back to task target on all CPUs.
    unregister_test_bp(&test_bps[tsk_on_cpu_idx]);
// Still have a CPU target breakpoint in get_test_cpu(1).
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
// Remove it and try again.
    unregister_test_bp(&test_bps[cpu_idx]);
    fill_one_bp_slot(test, &idx, -1, current);
    TEST_EXPECT_NOSPC(register_test_bp(-1, current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), current, idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx));
    TEST_EXPECT_NOSPC(register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx));
    }
    static struct kunit_case hw_breakpoint_test_cases[] = {
    KUNIT_CASE(test_one_cpu),
    KUNIT_CASE(test_many_cpus),
    KUNIT_CASE(test_one_task_on_all_cpus),
    KUNIT_CASE(test_two_tasks_on_all_cpus),
    KUNIT_CASE(test_one_task_on_one_cpu),
    KUNIT_CASE(test_one_task_mixed),
    KUNIT_CASE(test_two_tasks_on_one_cpu),
    KUNIT_CASE(test_two_tasks_on_one_all_cpus),
    KUNIT_CASE(test_task_on_all_and_one_cpu),
    {},
    };
#[no_mangle]
unsafe extern "C" fn test_init(test: *mut kunit) -> c_int {
    static int test_init(struct kunit *test)
    {
// Most test cases want 2 distinct CPUs.
    if (num_online_cpus() < 2)
    kunit_skip(test, "not enough cpus");
// Want the system to not use breakpoints elsewhere.
    if (hw_breakpoint_is_used())
    kunit_skip(test, "hw breakpoint already in use");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_exit(test: *mut kunit) {
    static void test_exit(struct kunit *test)
    {
    for (int i = 0; i < MAX_TEST_BREAKPOINTS; ++i) {
    if (test_bps[i])
    unregister_test_bp(&test_bps[i]);
    }
    if (__other_task) {
    kthread_stop(__other_task);
    __other_task = core::ptr::null_mut();
    }
// Verify that internal state agrees that no breakpoints are in use.
    KUNIT_EXPECT_FALSE(test, hw_breakpoint_is_used());
    }
    static struct kunit_suite hw_breakpoint_test_suite = {
    .name = "hw_breakpoint",
    .test_cases = hw_breakpoint_test_cases,
    .init = test_init,
    .exit = test_exit,
    };
    kunit_test_suites(&hw_breakpoint_test_suite);
    MODULE_AUTHOR("Marco Elver <elver@google.com>");
