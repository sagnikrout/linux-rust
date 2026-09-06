//! Automatically rewritten from C to Rust
//! Source: mm/tests/lazy_mmu_mode_kunit.c
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

    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
#[no_mangle]
unsafe extern "C" fn expect_not_active(test: *mut kunit) {
    static void expect_not_active(struct kunit *test)
    {
    KUNIT_EXPECT_FALSE(test, is_lazy_mmu_mode_active());
    }
#[no_mangle]
unsafe extern "C" fn expect_active(test: *mut kunit) {
    static void expect_active(struct kunit *test)
    {
    KUNIT_EXPECT_TRUE(test, is_lazy_mmu_mode_active());
    }
#[no_mangle]
unsafe extern "C" fn lazy_mmu_mode_active(test: *mut kunit) {
    static void lazy_mmu_mode_active(struct kunit *test)
    {
    expect_not_active(test);
    lazy_mmu_mode_enable();
    expect_active(test);
    {
// Nested section
    lazy_mmu_mode_enable();
    expect_active(test);
    lazy_mmu_mode_disable();
    expect_active(test);
    }
    {
// Paused section
    lazy_mmu_mode_pause();
    expect_not_active(test);
    {
// No effect (paused)
    lazy_mmu_mode_enable();
    expect_not_active(test);
    lazy_mmu_mode_disable();
    expect_not_active(test);
    lazy_mmu_mode_pause();
    expect_not_active(test);
    lazy_mmu_mode_resume();
    expect_not_active(test);
    }
    lazy_mmu_mode_resume();
    expect_active(test);
    }
    lazy_mmu_mode_disable();
    expect_not_active(test);
    }
    static struct kunit_case lazy_mmu_mode_test_cases[] = {
    KUNIT_CASE(lazy_mmu_mode_active),
    {}
    };
    static struct kunit_suite lazy_mmu_mode_test_suite = {
    .name = "lazy_mmu_mode",
    .test_cases = lazy_mmu_mode_test_cases,
    };
    kunit_test_suite(lazy_mmu_mode_test_suite);
    MODULE_DESCRIPTION("Tests for the lazy MMU mode");
    MODULE_LICENSE("GPL");
