//! Automatically rewritten from C to Rust
//! Source: lib/tests/test_kprobes.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// test_kprobes.c - simple sanity test for k*probes
//
// Copyright IBM Corp. 2008
//

pub const div_factor: c_int = 3;

    do { \
    (_kp).addr = core::ptr::null_mut(); \
    (_kp).flags = 0; \
    } while (0)
    static u32 rand1, preh_val, posth_val;
    static u32 (*target)(u32 value);
    static u32 (*recursed_target)(u32 value);
    static u32 (*target2)(u32 value);
    static struct kunit *current_test;
    static unsigned long (*internal_target)(void);
    static unsigned long (*stacktrace_target)(void);
    static unsigned long (*stacktrace_driver)(void);
    static unsigned long target_return_address[2];
#[no_mangle]
unsafe extern "C" fn kprobe_target(value: u32) -> noinline u32 {
    static noinline u32 kprobe_target(u32 value)
    {
    return (value / div_factor);
    }
#[no_mangle]
unsafe extern "C" fn kprobe_recursed_target(value: u32) -> noinline u32 {
    static noinline u32 kprobe_recursed_target(u32 value)
    {
    return (value / div_factor);
    }
#[no_mangle]
unsafe extern "C" fn kp_pre_handler(p: *mut kprobe, regs: *mut pt_regs) -> c_int {
    static int kp_pre_handler(struct kprobe *p, struct pt_regs *regs)
    {
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    preh_val = recursed_target(rand1);
    return 0;
    }
    static void kp_post_handler(struct kprobe *p, struct pt_regs *regs,
    unsigned long flags)
    {
    let mut expval: u32 = recursed_target(rand1);
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    KUNIT_EXPECT_EQ(current_test, preh_val, expval);
    posth_val = preh_val + div_factor;
    }
    static struct kprobe kp = {
    .symbol_name = "kprobe_target",
    .pre_handler = kp_pre_handler,
    .post_handler = kp_post_handler
    };
#[no_mangle]
unsafe extern "C" fn test_kprobe(test: *mut kunit) {
    static void test_kprobe(struct kunit *test)
    {
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_kprobe(&kp));
    target(rand1);
    unregister_kprobe(&kp);
    KUNIT_EXPECT_NE(test, 0, preh_val);
    KUNIT_EXPECT_NE(test, 0, posth_val);
    }
#[no_mangle]
unsafe extern "C" fn kprobe_target2(value: u32) -> noinline u32 {
    static noinline u32 kprobe_target2(u32 value)
    {
    return (value / div_factor) + 1;
    }
#[no_mangle]
unsafe extern "C" fn kprobe_stacktrace_internal_target() -> noinline unsigned long {
    static noinline unsigned long kprobe_stacktrace_internal_target(void)
    {
    if (!target_return_address[0])
    target_return_address[0] = (unsigned long)__builtin_return_address(0);
    return target_return_address[0];
    }
#[no_mangle]
unsafe extern "C" fn kprobe_stacktrace_target() -> noinline unsigned long {
    static noinline unsigned long kprobe_stacktrace_target(void)
    {
    if (!target_return_address[1])
    target_return_address[1] = (unsigned long)__builtin_return_address(0);
    if (internal_target)
    internal_target();
    return target_return_address[1];
    }
#[no_mangle]
unsafe extern "C" fn kprobe_stacktrace_driver() -> noinline unsigned long {
    static noinline unsigned long kprobe_stacktrace_driver(void)
    {
    if (stacktrace_target)
    stacktrace_target();
// This is for preventing inlining the function
    return (unsigned long)__builtin_return_address(0);
    }
#[no_mangle]
unsafe extern "C" fn kp_pre_handler2(p: *mut kprobe, regs: *mut pt_regs) -> c_int {
    static int kp_pre_handler2(struct kprobe *p, struct pt_regs *regs)
    {
    preh_val = (rand1 / div_factor) + 1;
    return 0;
    }
    static void kp_post_handler2(struct kprobe *p, struct pt_regs *regs,
    unsigned long flags)
    {
    KUNIT_EXPECT_EQ(current_test, preh_val, (rand1 / div_factor) + 1);
    posth_val = preh_val + div_factor;
    }
    static struct kprobe kp2 = {
    .symbol_name = "kprobe_target2",
    .pre_handler = kp_pre_handler2,
    .post_handler = kp_post_handler2
    };
#[no_mangle]
unsafe extern "C" fn test_kprobes(test: *mut kunit) {
    static void test_kprobes(struct kunit *test)
    {
    struct kprobe *kps[2] = {&kp, &kp2};
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_kprobes(kps, 2));
    preh_val = 0;
    posth_val = 0;
    target(rand1);
    KUNIT_EXPECT_NE(test, 0, preh_val);
    KUNIT_EXPECT_NE(test, 0, posth_val);
    preh_val = 0;
    posth_val = 0;
    target2(rand1);
    KUNIT_EXPECT_NE(test, 0, preh_val);
    KUNIT_EXPECT_NE(test, 0, posth_val);
    unregister_kprobes(kps, 2);
    }
    static struct kprobe kp_missed = {
    .symbol_name = "kprobe_recursed_target",
    .pre_handler = kp_pre_handler,
    .post_handler = kp_post_handler,
    };
#[no_mangle]
unsafe extern "C" fn test_kprobe_missed(test: *mut kunit) {
    static void test_kprobe_missed(struct kunit *test)
    {
    current_test = test;
    preh_val = 0;
    posth_val = 0;
    KUNIT_EXPECT_EQ(test, 0, register_kprobe(&kp_missed));
    recursed_target(rand1);
    KUNIT_EXPECT_EQ(test, 2, kp_missed.nmissed);
    KUNIT_EXPECT_NE(test, 0, preh_val);
    KUNIT_EXPECT_NE(test, 0, posth_val);
    unregister_kprobe(&kp_missed);
    }

    static u32 krph_val;
#[no_mangle]
unsafe extern "C" fn entry_handler(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    static int entry_handler(struct kretprobe_instance *ri, struct pt_regs *regs)
    {
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    krph_val = (rand1 / div_factor);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn return_handler(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    static int return_handler(struct kretprobe_instance *ri, struct pt_regs *regs)
    {
    let mut ret: c_ulong = regs_return_value(regs);
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    KUNIT_EXPECT_EQ(current_test, ret, rand1 / div_factor);
    KUNIT_EXPECT_NE(current_test, krph_val, 0);
    krph_val = rand1;
    return 0;
    }
    static struct kretprobe rp = {
    .handler	= return_handler,
    .entry_handler  = entry_handler,
    .kp.symbol_name = "kprobe_target"
    };
#[no_mangle]
unsafe extern "C" fn test_kretprobe(test: *mut kunit) {
    static void test_kretprobe(struct kunit *test)
    {
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_kretprobe(&rp));
    target(rand1);
    unregister_kretprobe(&rp);
    KUNIT_EXPECT_EQ(test, krph_val, rand1);
    }
#[no_mangle]
unsafe extern "C" fn return_handler2(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    static int return_handler2(struct kretprobe_instance *ri, struct pt_regs *regs)
    {
    let mut ret: c_ulong = regs_return_value(regs);
    KUNIT_EXPECT_EQ(current_test, ret, (rand1 / div_factor) + 1);
    KUNIT_EXPECT_NE(current_test, krph_val, 0);
    krph_val = rand1;
    return 0;
    }
    static struct kretprobe rp2 = {
    .handler	= return_handler2,
    .entry_handler  = entry_handler,
    .kp.symbol_name = "kprobe_target2"
    };
#[no_mangle]
unsafe extern "C" fn test_kretprobes(test: *mut kunit) {
    static void test_kretprobes(struct kunit *test)
    {
    struct kretprobe *rps[2] = {&rp, &rp2};
    current_test = test;
    KUNIT_EXPECT_EQ(test, 0, register_kretprobes(rps, 2));
    krph_val = 0;
    target(rand1);
    KUNIT_EXPECT_EQ(test, krph_val, rand1);
    krph_val = 0;
    target2(rand1);
    KUNIT_EXPECT_EQ(test, krph_val, rand1);
    unregister_kretprobes(rps, 2);
    }

pub const STACK_BUF_SIZE: c_int = 16;
    static unsigned long stack_buf[STACK_BUF_SIZE];
#[no_mangle]
unsafe extern "C" fn stacktrace_return_handler(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    static int stacktrace_return_handler(struct kretprobe_instance *ri, struct pt_regs *regs)
    {
    let mut retval: c_ulong = regs_return_value(regs);
    int i, ret;
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    KUNIT_EXPECT_EQ(current_test, retval, target_return_address[1]);
//
// Test stacktrace inside the kretprobe handler, this will involves
// kretprobe trampoline, but must include correct return address
// of the target function.
//
    ret = stack_trace_save(stack_buf, STACK_BUF_SIZE, 0);
    KUNIT_EXPECT_NE(current_test, ret, 0);
    for (i = 0; i < ret; i++) {
    if (stack_buf[i] == target_return_address[1])
    break;
    }
    KUNIT_EXPECT_NE(current_test, i, ret);

//
// Test stacktrace from pt_regs at the return address. Thus the stack
// trace must start from the target return address.
//
    ret = stack_trace_save_regs(regs, stack_buf, STACK_BUF_SIZE, 0);
    KUNIT_EXPECT_NE(current_test, ret, 0);
    KUNIT_EXPECT_EQ(current_test, stack_buf[0], target_return_address[1]);

    return 0;
    }
    static struct kretprobe rp3 = {
    .handler	= stacktrace_return_handler,
    .kp.symbol_name = "kprobe_stacktrace_target"
    };
#[no_mangle]
unsafe extern "C" fn test_stacktrace_on_kretprobe(test: *mut kunit) {
    static void test_stacktrace_on_kretprobe(struct kunit *test)
    {
    let mut myretaddr: c_ulong = (unsigned long)__builtin_return_address(0);
    current_test = test;
//
// Run the stacktrace_driver() to record correct return address in
// stacktrace_target() and ensure stacktrace_driver() call is not
// inlined by checking the return address of stacktrace_driver()
// and the return address of this function is different.
//
    KUNIT_ASSERT_NE(test, myretaddr, stacktrace_driver());
    KUNIT_ASSERT_EQ(test, 0, register_kretprobe(&rp3));
    KUNIT_ASSERT_NE(test, myretaddr, stacktrace_driver());
    unregister_kretprobe(&rp3);
    }
#[no_mangle]
unsafe extern "C" fn stacktrace_internal_return_handler(ri: *mut kretprobe_instance, regs: *mut pt_regs) -> c_int {
    static int stacktrace_internal_return_handler(struct kretprobe_instance *ri, struct pt_regs *regs)
    {
    let mut retval: c_ulong = regs_return_value(regs);
    int i, ret;
    KUNIT_EXPECT_FALSE(current_test, preemptible());
    KUNIT_EXPECT_EQ(current_test, retval, target_return_address[0]);
//
// Test stacktrace inside the kretprobe handler for nested case.
// The unwinder will find the kretprobe_trampoline address on the
// return address, and kretprobe must solve that.
//
    ret = stack_trace_save(stack_buf, STACK_BUF_SIZE, 0);
    KUNIT_EXPECT_NE(current_test, ret, 0);
    for (i = 0; i < ret - 1; i++) {
    if (stack_buf[i] == target_return_address[0]) {
    KUNIT_EXPECT_EQ(current_test, stack_buf[i + 1], target_return_address[1]);
    break;
    }
    }
    KUNIT_EXPECT_NE(current_test, i, ret);

// Ditto for the regs version.
    ret = stack_trace_save_regs(regs, stack_buf, STACK_BUF_SIZE, 0);
    KUNIT_EXPECT_NE(current_test, ret, 0);
    KUNIT_EXPECT_EQ(current_test, stack_buf[0], target_return_address[0]);
    KUNIT_EXPECT_EQ(current_test, stack_buf[1], target_return_address[1]);

    return 0;
    }
    static struct kretprobe rp4 = {
    .handler	= stacktrace_internal_return_handler,
    .kp.symbol_name = "kprobe_stacktrace_internal_target"
    };
#[no_mangle]
unsafe extern "C" fn test_stacktrace_on_nested_kretprobe(test: *mut kunit) {
    static void test_stacktrace_on_nested_kretprobe(struct kunit *test)
    {
    let mut myretaddr: c_ulong = (unsigned long)__builtin_return_address(0);
    struct kretprobe *rps[2] = {&rp3, &rp4};
    current_test = test;
// KUNIT_ASSERT_NE(test, myretaddr, stacktrace_driver());
    KUNIT_ASSERT_EQ(test, 0, register_kretprobes(rps, 2));
    KUNIT_ASSERT_NE(test, myretaddr, stacktrace_driver());
    unregister_kretprobes(rps, 2);
    }

#[no_mangle]
unsafe extern "C" fn kprobes_test_init(test: *mut kunit) -> c_int {
    static int kprobes_test_init(struct kunit *test)
    {
    KP_CLEAR(kp);
    KP_CLEAR(kp2);
    KP_CLEAR(kp_missed);

    KP_CLEAR(rp.kp);
    KP_CLEAR(rp2.kp);

    KP_CLEAR(rp3.kp);
    KP_CLEAR(rp4.kp);

    target = kprobe_target;
    target2 = kprobe_target2;
    recursed_target = kprobe_recursed_target;
    stacktrace_target = kprobe_stacktrace_target;
    internal_target = kprobe_stacktrace_internal_target;
    stacktrace_driver = kprobe_stacktrace_driver;
    rand1 = get_random_u32_above(div_factor);
    return 0;
    }
    static struct kunit_case kprobes_testcases[] = {
    KUNIT_CASE(test_kprobe),
    KUNIT_CASE(test_kprobes),
    KUNIT_CASE(test_kprobe_missed),

    KUNIT_CASE(test_kretprobe),
    KUNIT_CASE(test_kretprobes),

    KUNIT_CASE(test_stacktrace_on_kretprobe),
    KUNIT_CASE(test_stacktrace_on_nested_kretprobe),

    {}
    };
    static struct kunit_suite kprobes_test_suite = {
    .name = "kprobes_test",
    .init = kprobes_test_init,
    .test_cases = kprobes_testcases,
    };
    kunit_test_suites(&kprobes_test_suite);
    MODULE_DESCRIPTION("simple sanity test for k*probes");
    MODULE_LICENSE("GPL");
