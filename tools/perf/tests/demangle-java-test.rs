//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/demangle-java-test.c
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

#[no_mangle]
unsafe extern "C" fn test__demangle_java(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__demangle_java(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut ret: c_int = TEST_OK;
    char *buf = core::ptr::null_mut();
    size_t i;
    struct {
    const char *mangled, *demangled;
    } test_cases[] = {
    { "Ljava/lang/StringLatin1;equals([B[B)Z",
    "java.lang.StringLatin1.equals(byte[], byte[])" },
    { "Ljava/util/zip/ZipUtils;CENSIZ([BI)J",
    "java.util.zip.ZipUtils.CENSIZ(byte[], int)" },
    { "Ljava/util/regex/Pattern$BmpCharProperty;match(Ljava/util/regex/Matcher;ILjava/lang/CharSequence;)Z",
    "java.util.regex.Pattern$BmpCharProperty.match(java.util.regex.Matcher, int, java.lang.CharSequence)" },
    { "Ljava/lang/AbstractStringBuilder;appendChars(Ljava/lang/String;II)V",
    "java.lang.AbstractStringBuilder.appendChars(java.lang.String, int, int)" },
    { "Ljava/lang/Object;<init>()V",
    "java.lang.Object<init>()" },
    };
    for (i = 0; i < ARRAY_SIZE(test_cases); i++) {
    buf = dso__demangle_sym(/*dso=*/core::ptr::null_mut(), /*kmodule=*/0, test_cases[i].mangled);
    if (!buf) {
    pr_debug("FAILED to demangle: \"%s\"\n \"%s\"\n", test_cases[i].mangled,
    test_cases[i].demangled);
    continue;
    }
    if (strcmp(buf, test_cases[i].demangled)) {
    pr_debug("FAILED: %s: %s != %s\n", test_cases[i].mangled,
    buf, test_cases[i].demangled);
    ret = TEST_FAIL;
    }
    free(buf);
    }
    return ret;
    }
    DEFINE_SUITE("Demangle Java", demangle_java);
