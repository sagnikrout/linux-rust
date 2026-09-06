//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/kmod-path.c
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

    static int test(const char *path, bool alloc_name, bool kmod,
    int comp, const char *name)
    {
    struct kmod_path m;
    memset(&m, 0x0, sizeof(m));
    TEST_ASSERT_VAL("kmod_path__parse",
    !__kmod_path__parse(&m, path, alloc_name));
    pr_debug("%s - alloc name %d, kmod %d, comp %d, name '%s'\n",
    path, alloc_name, m.kmod, m.comp, m.name);
    TEST_ASSERT_VAL("wrong kmod", m.kmod == kmod);
    TEST_ASSERT_VAL("wrong comp", m.comp == comp);
    if (name)
    TEST_ASSERT_VAL("wrong name", m.name && !strcmp(name, m.name));
    else
    TEST_ASSERT_VAL("wrong name", !m.name);
    free(m.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_is_kernel_module(path: *const c_char, cpumode: c_int, expect: bool) -> c_int {
    static int test_is_kernel_module(const char *path, int cpumode, bool expect)
    {
    TEST_ASSERT_VAL("is_kernel_module",
    (!!is_kernel_module(path, cpumode)) == (!!expect));
    pr_debug("%s (cpumode: %d) - is_kernel_module: %s\n",
    path, cpumode, expect ? "true" : "false");
    return 0;
    }

    TEST_ASSERT_VAL("failed", !test(path, an, k, c, n))

    TEST_ASSERT_VAL("failed", !test_is_kernel_module(path, c, e))
#[no_mangle]
unsafe extern "C" fn test__kmod_path__parse(__maybe_unused: *mut *mut test_suite t, __maybe_unused: int subtest) -> c_int {
    static int test__kmod_path__parse(struct test_suite *t __maybe_unused, int subtest __maybe_unused)
    {
// path                alloc_name  kmod  comp   name
    T("/xxxx/xxxx/x-x.ko", true      , true, 0    , "[x_x]");
    T("/xxxx/xxxx/x-x.ko", false     , true, 0    , core::ptr::null_mut()   );
    T("/xxxx/xxxx/x-x.ko", true      , true, 0    , "[x_x]");
    T("/xxxx/xxxx/x-x.ko", false     , true, 0    , core::ptr::null_mut()   );
    M("/xxxx/xxxx/x-x.ko", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
    M("/xxxx/xxxx/x-x.ko", PERF_RECORD_MISC_KERNEL, true);
    M("/xxxx/xxxx/x-x.ko", PERF_RECORD_MISC_USER, false);

// path                alloc_name   kmod  comp  name
    T("/xxxx/xxxx/x.ko.gz", true     , true, 1   , "[x]");
    T("/xxxx/xxxx/x.ko.gz", false    , true, 1   , core::ptr::null_mut() );
    T("/xxxx/xxxx/x.ko.gz", true     , true, 1   , "[x]");
    T("/xxxx/xxxx/x.ko.gz", false    , true, 1   , core::ptr::null_mut() );
    M("/xxxx/xxxx/x.ko.gz", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
    M("/xxxx/xxxx/x.ko.gz", PERF_RECORD_MISC_KERNEL, true);
    M("/xxxx/xxxx/x.ko.gz", PERF_RECORD_MISC_USER, false);
// path              alloc_name  kmod   comp  name
    T("/xxxx/xxxx/x.gz", true      , false, 1   , "x.gz");
    T("/xxxx/xxxx/x.gz", false     , false, 1   , core::ptr::null_mut()  );
    T("/xxxx/xxxx/x.gz", true      , false, 1   , "x.gz");
    T("/xxxx/xxxx/x.gz", false     , false, 1   , core::ptr::null_mut()  );
    M("/xxxx/xxxx/x.gz", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
    M("/xxxx/xxxx/x.gz", PERF_RECORD_MISC_KERNEL, false);
    M("/xxxx/xxxx/x.gz", PERF_RECORD_MISC_USER, false);
// path   alloc_name  kmod   comp  name
    T("x.gz", true      , false, 1   , "x.gz");
    T("x.gz", false     , false, 1   , core::ptr::null_mut()  );
    T("x.gz", true      , false, 1   , "x.gz");
    T("x.gz", false     , false, 1   , core::ptr::null_mut()  );
    M("x.gz", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
    M("x.gz", PERF_RECORD_MISC_KERNEL, false);
    M("x.gz", PERF_RECORD_MISC_USER, false);
// path      alloc_name  kmod  comp  name
    T("x.ko.gz", true      , true, 1   , "[x]");
    T("x.ko.gz", false     , true, 1   , core::ptr::null_mut() );
    T("x.ko.gz", true      , true, 1   , "[x]");
    T("x.ko.gz", false     , true, 1   , core::ptr::null_mut() );
    M("x.ko.gz", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
    M("x.ko.gz", PERF_RECORD_MISC_KERNEL, true);
    M("x.ko.gz", PERF_RECORD_MISC_USER, false);

// path            alloc_name  kmod  comp   name
    T("[test_module]", true      , true, false, "[test_module]");
    T("[test_module]", false     , true, false, core::ptr::null_mut()           );
    T("[test_module]", true      , true, false, "[test_module]");
    T("[test_module]", false     , true, false, core::ptr::null_mut()           );
    M("[test_module]", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
    M("[test_module]", PERF_RECORD_MISC_KERNEL, true);
    M("[test_module]", PERF_RECORD_MISC_USER, false);
// path            alloc_name  kmod  comp   name
    T("[test.module]", true      , true, false, "[test.module]");
    T("[test.module]", false     , true, false, core::ptr::null_mut()           );
    T("[test.module]", true      , true, false, "[test.module]");
    T("[test.module]", false     , true, false, core::ptr::null_mut()           );
    M("[test.module]", PERF_RECORD_MISC_CPUMODE_UNKNOWN, true);
    M("[test.module]", PERF_RECORD_MISC_KERNEL, true);
    M("[test.module]", PERF_RECORD_MISC_USER, false);
// path     alloc_name  kmod   comp   name
    T("[vdso]", true      , false, false, "[vdso]");
    T("[vdso]", false     , false, false, core::ptr::null_mut()    );
    T("[vdso]", true      , false, false, "[vdso]");
    T("[vdso]", false     , false, false, core::ptr::null_mut()    );
    M("[vdso]", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
    M("[vdso]", PERF_RECORD_MISC_KERNEL, false);
    M("[vdso]", PERF_RECORD_MISC_USER, false);
    T("[vdso32]", true      , false, false, "[vdso32]");
    T("[vdso32]", false     , false, false, core::ptr::null_mut()      );
    T("[vdso32]", true      , false, false, "[vdso32]");
    T("[vdso32]", false     , false, false, core::ptr::null_mut()      );
    M("[vdso32]", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
    M("[vdso32]", PERF_RECORD_MISC_KERNEL, false);
    M("[vdso32]", PERF_RECORD_MISC_USER, false);
    T("[vdsox32]", true      , false, false, "[vdsox32]");
    T("[vdsox32]", false     , false, false, core::ptr::null_mut()       );
    T("[vdsox32]", true      , false, false, "[vdsox32]");
    T("[vdsox32]", false     , false, false, core::ptr::null_mut()       );
    M("[vdsox32]", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
    M("[vdsox32]", PERF_RECORD_MISC_KERNEL, false);
    M("[vdsox32]", PERF_RECORD_MISC_USER, false);
// path         alloc_name  kmod   comp   name
    T("[vsyscall]", true      , false, false, "[vsyscall]");
    T("[vsyscall]", false     , false, false, core::ptr::null_mut()        );
    T("[vsyscall]", true      , false, false, "[vsyscall]");
    T("[vsyscall]", false     , false, false, core::ptr::null_mut()        );
    M("[vsyscall]", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
    M("[vsyscall]", PERF_RECORD_MISC_KERNEL, false);
    M("[vsyscall]", PERF_RECORD_MISC_USER, false);
// path                alloc_name  kmod   comp   name
    T("[kernel.kallsyms]", true      , false, false, "[kernel.kallsyms]");
    T("[kernel.kallsyms]", false     , false, false, core::ptr::null_mut()               );
    T("[kernel.kallsyms]", true      , false, false, "[kernel.kallsyms]");
    T("[kernel.kallsyms]", false     , false, false, core::ptr::null_mut()               );
    M("[kernel.kallsyms]", PERF_RECORD_MISC_CPUMODE_UNKNOWN, false);
    M("[kernel.kallsyms]", PERF_RECORD_MISC_KERNEL, false);
    M("[kernel.kallsyms]", PERF_RECORD_MISC_USER, false);
    return 0;
    }
    DEFINE_SUITE("kmod_path__parse", kmod_path__parse);
