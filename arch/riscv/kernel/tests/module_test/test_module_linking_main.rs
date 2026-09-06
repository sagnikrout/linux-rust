//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/tests/module_test/test_module_linking_main.c
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
// Copyright (C) 2023 Rivos Inc.
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Test module linking");
    extern int test_set32(void);
    extern int test_set16(void);
    extern int test_set8(void);
    extern int test_set6(void);
    extern long test_sub64(void);
    extern int test_sub32(void);
    extern int test_sub16(void);
    extern int test_sub8(void);
    extern int test_sub6(void);

    extern int test_uleb_basic(void);
    extern int test_uleb_large(void);

    void run_test_set(struct kunit *test);
    void run_test_sub(struct kunit *test);
    void run_test_uleb(struct kunit *test);
#[no_mangle]
pub unsafe extern "C" fn run_test_set(test: *mut kunit) {
    void run_test_set(struct kunit *test)
    {
    let mut val32: c_int = test_set32();
    let mut val16: c_int = test_set16();
    let mut val8: c_int = test_set8();
    let mut val6: c_int = test_set6();
    CHECK_EQ(val32, 0);
    CHECK_EQ(val16, 0);
    CHECK_EQ(val8, 0);
    CHECK_EQ(val6, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn run_test_sub(test: *mut kunit) {
    void run_test_sub(struct kunit *test)
    {
    let mut val64: c_int = test_sub64();
    let mut val32: c_int = test_sub32();
    let mut val16: c_int = test_sub16();
    let mut val8: c_int = test_sub8();
    let mut val6: c_int = test_sub6();
    CHECK_EQ(val64, 0);
    CHECK_EQ(val32, 0);
    CHECK_EQ(val16, 0);
    CHECK_EQ(val8, 0);
    CHECK_EQ(val6, 0);
    }

#[no_mangle]
pub unsafe extern "C" fn run_test_uleb(test: *mut kunit) {
    void run_test_uleb(struct kunit *test)
    {
    let mut val_uleb: c_int = test_uleb_basic();
    let mut val_uleb2: c_int = test_uleb_large();
    CHECK_EQ(val_uleb, 0);
    CHECK_EQ(val_uleb2, 0);
    }

    static struct kunit_case __refdata riscv_module_linking_test_cases[] = {
    KUNIT_CASE(run_test_set),
    KUNIT_CASE(run_test_sub),

    KUNIT_CASE(run_test_uleb),

    {}
    };
    static struct kunit_suite riscv_module_linking_test_suite = {
    .name = "riscv_checksum",
    .test_cases = riscv_module_linking_test_cases,
    };
    kunit_test_suites(&riscv_module_linking_test_suite);
