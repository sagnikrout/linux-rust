//! Automatically rewritten from C to Rust
//! Source: lib/tests/longest_symbol_kunit.c
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
// Test the longest symbol length. Execute with:
// ./tools/testing/kunit/kunit.py run longest-symbol
// --arch=x86_64 --kconfig_add CONFIG_KPROBES=y --kconfig_add CONFIG_MODULES=y
// --kconfig_add CONFIG_CPU_MITIGATIONS=n --kconfig_add CONFIG_GCOV_KERNEL=n
//

// Generate a symbol whose name length is 511

pub const RETURN_LONGEST_SYM: c_uint = 0xAAAAA;
    noinline int LONGEST_SYM_NAME(void);
#[no_mangle]
pub unsafe extern "C" fn LONGEST_SYM_NAME() -> noinline int {
    noinline int LONGEST_SYM_NAME(void)
    {
    return RETURN_LONGEST_SYM;
    }
    _Static_assert(sizeof(__stringify(LONGEST_SYM_NAME)) == KSYM_NAME_LEN,
    "Incorrect symbol length found. Expected KSYM_NAME_LEN: "
    __stringify(KSYM_NAME_LEN) ", but found: "
    __stringify(sizeof(LONGEST_SYM_NAME)));
#[no_mangle]
unsafe extern "C" fn test_longest_symbol(test: *mut kunit) {
    static void test_longest_symbol(struct kunit *test)
    {
    KUNIT_EXPECT_EQ(test, RETURN_LONGEST_SYM, LONGEST_SYM_NAME());
    };
#[no_mangle]
unsafe extern "C" fn test_longest_symbol_kallsyms(test: *mut kunit) {
    static void test_longest_symbol_kallsyms(struct kunit *test)
    {
    unsigned long (*kallsyms_lookup_name)(const char *name);
    static int (*longest_sym)(void);
    struct kprobe kp = {
    .symbol_name = "kallsyms_lookup_name",
    };
    if (register_kprobe(&kp) < 0) {
    pr_info("%s: kprobe not registered", __func__);
    KUNIT_FAIL(test, "test_longest_symbol kallsyms: kprobe not registered\n");
    return;
    }
    kunit_warn(test, "test_longest_symbol kallsyms: kprobe registered\n");
    kallsyms_lookup_name = (unsigned long (*)(const char *name))kp.addr;
    unregister_kprobe(&kp);
    longest_sym =
    (void *) kallsyms_lookup_name(__stringify(LONGEST_SYM_NAME));
    KUNIT_EXPECT_EQ(test, RETURN_LONGEST_SYM, longest_sym());
    };
    static struct kunit_case longest_symbol_test_cases[] = {
    KUNIT_CASE(test_longest_symbol),
    KUNIT_CASE(test_longest_symbol_kallsyms),
    {}
    };
    static struct kunit_suite longest_symbol_test_suite = {
    .name = "longest-symbol",
    .test_cases = longest_symbol_test_cases,
    };
    kunit_test_suite(longest_symbol_test_suite);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Test the longest symbol length");
    MODULE_AUTHOR("Sergio González Collado");
