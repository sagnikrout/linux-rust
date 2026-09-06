//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/demangle-ocaml-test.c
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
unsafe extern "C" fn test__demangle_ocaml(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__demangle_ocaml(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut ret: c_int = TEST_OK;
    char *buf = core::ptr::null_mut();
    size_t i;
    struct {
    const char *mangled, *demangled;
    } test_cases[] = {
    { "main",
    core::ptr::null_mut() },
    { "camlStdlib__array__map_154",
    "Stdlib.array.map_154" },
    { "camlStdlib__anon_fn$5bstdlib$2eml$3a334$2c0$2d$2d54$5d_1453",
    "Stdlib.anon_fn[stdlib.ml:334,0--54]_1453" },
    { "camlStdlib__bytes__$2b$2b_2205",
    "Stdlib.bytes.++_2205" },
    };
    for (i = 0; i < ARRAY_SIZE(test_cases); i++) {
    buf = dso__demangle_sym(/*dso=*/core::ptr::null_mut(), /*kmodule=*/0, test_cases[i].mangled);
    if ((buf == core::ptr::null_mut() && test_cases[i].demangled != core::ptr::null_mut())
    || (buf != core::ptr::null_mut() && test_cases[i].demangled == core::ptr::null_mut())
    || (buf != core::ptr::null_mut() && strcmp(buf, test_cases[i].demangled))) {
    pr_debug("FAILED: %s: %s != %s\n", test_cases[i].mangled,
    buf == core::ptr::null_mut() ? "(null)" : buf,
    test_cases[i].demangled == core::ptr::null_mut() ? "(null)" : test_cases[i].demangled);
    ret = TEST_FAIL;
    }
    free(buf);
    }
    return ret;
    }
    DEFINE_SUITE("Demangle OCaml", demangle_ocaml);
