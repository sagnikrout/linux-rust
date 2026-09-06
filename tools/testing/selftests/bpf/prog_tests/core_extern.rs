//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/core_extern.c
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
// Copyright (c) 2019 Facebook

#[no_mangle]
unsafe extern "C" fn get_kernel_version() -> u32 {
    static uint32_t get_kernel_version(void)
    {
    uint32_t major, minor, patch;
    struct utsname info;
    uname(&info);
    if (sscanf(info.release, "%u.%u.%u", &major, &minor, &patch) != 3)
    return 0;
    return KERNEL_VERSION(major, minor, patch);
    }

    static struct test_case {
    const char *name;
    const char *cfg;
    bool fails;
    struct test_core_extern__data data;
    } test_cases[] = {
    { .name = "default search path", .data = { .bpf_syscall = true } },
    {
    .name = "custom values",
    .cfg = "CONFIG_BPF_SYSCALL=n\n"
    "CONFIG_TRISTATE=m\n"
    "CONFIG_BOOL=y\n"
    "CONFIG_CHAR=100\n"
    "CONFIG_USHORT=30000\n"
    "CONFIG_INT=123456\n"
    "CONFIG_ULONG=0xDEADBEEFC0DE\n"
    "CONFIG_STR=\"abracad\"\n"
    "CONFIG_MISSING=0",
    .data = {
    .unkn_virt_val = 0,
    .bpf_syscall = false,
    .tristate_val = TRI_MODULE,
    .bool_val = true,
    .char_val = 100,
    .ushort_val = 30000,
    .int_val = 123456,
    .ulong_val = 0xDEADBEEFC0DE,
    .str_val = "abracad",
    },
    },
// TRISTATE
    { .name = "tristate (y)", .cfg = CFG"CONFIG_TRISTATE=y\n",
    .data = { .tristate_val = TRI_YES } },
    { .name = "tristate (n)", .cfg = CFG"CONFIG_TRISTATE=n\n",
    .data = { .tristate_val = TRI_NO } },
    { .name = "tristate (m)", .cfg = CFG"CONFIG_TRISTATE=m\n",
    .data = { .tristate_val = TRI_MODULE } },
    { .name = "tristate (int)", .fails = 1, .cfg = CFG"CONFIG_TRISTATE=1" },
    { .name = "tristate (bad)", .fails = 1, .cfg = CFG"CONFIG_TRISTATE=M" },
// BOOL
    { .name = "bool (y)", .cfg = CFG"CONFIG_BOOL=y\n",
    .data = { .bool_val = true } },
    { .name = "bool (n)", .cfg = CFG"CONFIG_BOOL=n\n",
    .data = { .bool_val = false } },
    { .name = "bool (tristate)", .fails = 1, .cfg = CFG"CONFIG_BOOL=m" },
    { .name = "bool (int)", .fails = 1, .cfg = CFG"CONFIG_BOOL=1" },
// CHAR
    { .name = "char (tristate)", .cfg = CFG"CONFIG_CHAR=m\n",
    .data = { .char_val = 'm' } },
    { .name = "char (bad)", .fails = 1, .cfg = CFG"CONFIG_CHAR=q\n" },
    { .name = "char (empty)", .fails = 1, .cfg = CFG"CONFIG_CHAR=\n" },
    { .name = "char (str)", .fails = 1, .cfg = CFG"CONFIG_CHAR=\"y\"\n" },
// STRING
    { .name = "str (empty)", .cfg = CFG"CONFIG_STR=\"\"\n",
    .data = { .str_val = "\0\0\0\0\0\0\0" } },
    { .name = "str (padded)", .cfg = CFG"CONFIG_STR=\"abra\"\n",
    .data = { .str_val = "abra\0\0\0" } },
    { .name = "str (too long)", .cfg = CFG"CONFIG_STR=\"abracada\"\n",
    .data = { .str_val = "abracad" } },
    { .name = "str (no value)", .fails = 1, .cfg = CFG"CONFIG_STR=\n" },
    { .name = "str (bad value)", .fails = 1, .cfg = CFG"CONFIG_STR=bla\n" },
// INTEGERS
    {
    .name = "integer forms",
    .cfg = CFG
    "CONFIG_CHAR=0xA\n"
    "CONFIG_USHORT=0462\n"
    "CONFIG_INT=-100\n"
    "CONFIG_ULONG=+1000000000000",
    .data = {
    .char_val = 0xA,
    .ushort_val = 0462,
    .int_val = -100,
    .ulong_val = 1000000000000,
    },
    },
    { .name = "int (bad)", .fails = 1, .cfg = CFG"CONFIG_INT=abc" },
    { .name = "int (str)", .fails = 1, .cfg = CFG"CONFIG_INT=\"abc\"" },
    { .name = "int (empty)", .fails = 1, .cfg = CFG"CONFIG_INT=" },
    { .name = "int (mixed)", .fails = 1, .cfg = CFG"CONFIG_INT=123abc" },
    { .name = "int (max)", .cfg = CFG"CONFIG_INT=2147483647",
    .data = { .int_val = 2147483647 } },
    { .name = "int (min)", .cfg = CFG"CONFIG_INT=-2147483648",
    .data = { .int_val = -2147483648 } },
    { .name = "int (max+1)", .fails = 1, .cfg = CFG"CONFIG_INT=2147483648" },
    { .name = "int (min-1)", .fails = 1, .cfg = CFG"CONFIG_INT=-2147483649" },
    { .name = "ushort (max)", .cfg = CFG"CONFIG_USHORT=65535",
    .data = { .ushort_val = 65535 } },
    { .name = "ushort (min)", .cfg = CFG"CONFIG_USHORT=0",
    .data = { .ushort_val = 0 } },
    { .name = "ushort (max+1)", .fails = 1, .cfg = CFG"CONFIG_USHORT=65536" },
    { .name = "ushort (min-1)", .fails = 1, .cfg = CFG"CONFIG_USHORT=-1" },
    { .name = "u64 (max)", .cfg = CFG"CONFIG_ULONG=0xffffffffffffffff",
    .data = { .ulong_val = 0xffffffffffffffff } },
    { .name = "u64 (min)", .cfg = CFG"CONFIG_ULONG=0",
    .data = { .ulong_val = 0 } },
    { .name = "u64 (max+1)", .fails = 1, .cfg = CFG"CONFIG_ULONG=0x10000000000000000" },
    };
#[no_mangle]
pub unsafe extern "C" fn test_core_extern() {
    void test_core_extern(void)
    {
    let mut kern_ver: u32 = get_kernel_version();
    int err, i, j;
    struct test_core_extern *skel = core::ptr::null_mut();
    uint64_t *got, *exp;
    let mut n: c_int = sizeof(*skel.data) / sizeof(uint64_t);
    for (i = 0; i < ARRAY_SIZE(test_cases); i++) {
    struct test_case *t = &test_cases[i];
    DECLARE_LIBBPF_OPTS(bpf_object_open_opts, opts,
    .kconfig = t.cfg,
    );
    if (!test__start_subtest(t.name))
    continue;
    skel = test_core_extern__open_opts(&opts);
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    goto cleanup;
    err = test_core_extern__load(skel);
    if (t.fails) {
    ASSERT_ERR(err, "skel_load_should_fail");
    goto cleanup;
    } else if (!ASSERT_OK(err, "skel_load")) {
    goto cleanup;
    }
    err = test_core_extern__attach(skel);
    if (!ASSERT_OK(err, "attach_raw_tp"))
    goto cleanup;
    usleep(1);
    t.data.kern_ver = kern_ver;
    t.data.missing_val = 0xDEADC0DE;
    got = (uint64_t *)skel.data;
    exp = (uint64_t *)&t.data;
    for (j = 0; j < n; j++) {
    ASSERT_EQ(got[j], exp[j], "result");
    }
    cleanup:
    test_core_extern__destroy(skel);
    skel = core::ptr::null_mut();
    }
    }
