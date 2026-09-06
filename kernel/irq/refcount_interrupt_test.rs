//! Automatically rewritten from C to Rust
//! Source: kernel/irq/refcount_interrupt_test.c
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
// KUnit test for refcounted interrupt enable/disables.
//

// ===== Test cases =====
#[no_mangle]
unsafe extern "C" fn test_single_irq_change(test: *mut kunit) {
    static void test_single_irq_change(struct kunit *test)
    {
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    }
#[no_mangle]
unsafe extern "C" fn test_nested_irq_change(test: *mut kunit) {
    static void test_nested_irq_change(struct kunit *test)
    {
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_ON();
    }
#[no_mangle]
unsafe extern "C" fn test_multiple_irq_change(test: *mut kunit) {
    static void test_multiple_irq_change(struct kunit *test)
    {
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_ON();
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_ON();
    }
#[no_mangle]
unsafe extern "C" fn test_irq_save(test: *mut kunit) {
    static void test_irq_save(struct kunit *test)
    {
    unsigned long flags;
    local_irq_save(flags);
    TEST_IRQ_OFF();
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_OFF();
    local_irq_restore(flags);
    TEST_IRQ_ON();
    local_interrupt_disable();
    TEST_IRQ_OFF();
    local_irq_save(flags);
    TEST_IRQ_OFF();
    local_irq_restore(flags);
    TEST_IRQ_OFF();
    local_interrupt_enable();
    TEST_IRQ_ON();
    }
    static struct kunit_case test_cases[] = {
    KUNIT_CASE(test_single_irq_change),
    KUNIT_CASE(test_nested_irq_change),
    KUNIT_CASE(test_multiple_irq_change),
    KUNIT_CASE(test_irq_save),
    {},
    };
// init and exit are the same.
#[no_mangle]
unsafe extern "C" fn test_init(test: *mut kunit) -> c_int {
    static int test_init(struct kunit *test)
    {
    TEST_IRQ_ON();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_exit(test: *mut kunit) {
    static void test_exit(struct kunit *test)
    {
    TEST_IRQ_ON();
    }
    static struct kunit_suite refcount_interrupt_test_suite = {
    .name = "refcount_interrupt",
    .test_cases = test_cases,
    .init = test_init,
    .exit = test_exit,
    };
    kunit_test_suite(refcount_interrupt_test_suite);
    MODULE_AUTHOR("Lyude Paul <lyude@redhat.com>");
    MODULE_DESCRIPTION("Refcounted interrupt unit test suite");
    MODULE_LICENSE("GPL");
