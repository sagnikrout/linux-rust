//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/pe-file-parsing.c
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
unsafe extern "C" fn run_dir(d: *const c_char) -> c_int {
    static int run_dir(const char *d)
    {
    char filename[PATH_MAX];
    char debugfile[PATH_MAX];
    let mut bid: build_id = { .size = 0, };
    char debuglink[PATH_MAX];
    char expect_build_id[] = {
    0x5a, 0x0f, 0xd8, 0x82, 0xb5, 0x30, 0x84, 0x22,
    0x4b, 0xa4, 0x7b, 0x62, 0x4c, 0x55, 0xa4, 0x69,
    };
    char expect_debuglink[PATH_MAX] = "pe-file.exe.debug";
    struct dso *dso;
    struct symbol *sym;
    int ret;
    size_t idx;
    scnprintf(filename, PATH_MAX, "%s/pe-file.exe", d);
    ret = filename__read_build_id(filename, &bid);
    TEST_ASSERT_VAL("Failed to read build_id",
    ret == sizeof(expect_build_id));
    TEST_ASSERT_VAL("Wrong build_id", !memcmp(bid.data, expect_build_id,
    sizeof(expect_build_id)));
    ret = filename__read_debuglink(filename, debuglink, PATH_MAX);
    TEST_ASSERT_VAL("Failed to read debuglink", ret == 0);
    TEST_ASSERT_VAL("Wrong debuglink",
    !strcmp(debuglink, expect_debuglink));
    scnprintf(debugfile, PATH_MAX, "%s/%s", d, debuglink);
    ret = filename__read_build_id(debugfile, &bid);
    TEST_ASSERT_VAL("Failed to read debug file build_id",
    ret == sizeof(expect_build_id));
    TEST_ASSERT_VAL("Wrong build_id", !memcmp(bid.data, expect_build_id,
    sizeof(expect_build_id)));
    dso = dso__new(filename);
    TEST_ASSERT_VAL("Failed to get dso", dso);
    ret = dso__load_bfd_symbols(dso, debugfile);
    TEST_ASSERT_VAL("Failed to load symbols", ret == 0);
    dso__sort_by_name(dso);
    sym = dso__find_symbol_by_name(dso, "main", &idx);
    TEST_ASSERT_VAL("Failed to find main", sym);
    dso__delete(dso);
    return TEST_OK;
    }
    static int test__pe_file_parsing(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct stat st;
    char path_dir[PATH_MAX];
// First try development tree tests.
    if (!lstat("./tests", &st))
    return run_dir("./tests");
// Then installed path.
    snprintf(path_dir, PATH_MAX, "%s/tests", get_argv_exec_path());
    if (!lstat(path_dir, &st))
    return run_dir(path_dir);
    return TEST_SKIP;
    }

    static int test__pe_file_parsing(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    return TEST_SKIP;
    }

    DEFINE_SUITE("PE file support", pe_file_parsing);
