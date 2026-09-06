//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/bpftool_metadata.c
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

pub const MAX_TOKENS_TO_CHECK: c_int = 3;
    static char output[MAX_BPFTOOL_OUTPUT_LEN];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_desc {
    pub name: *mut c_char,
    pub bpf_prog: *mut c_char,
    pub bpffs_path: *mut c_char,
    pub expected_output: [*mut c_char; MAX_TOKENS_TO_CHECK],
    pub expected_output_json: [*mut c_char; MAX_TOKENS_TO_CHECK],
    pub metadata_map_name: *mut c_char,
}

#[no_mangle]
unsafe extern "C" fn setup(test: *mut test_desc) -> c_int {
    static int setup(struct test_desc *test)
    {
    return mkdir(BPFFS_DIR, 0700);
    }
#[no_mangle]
unsafe extern "C" fn cleanup(test: *mut test_desc) {
    static void cleanup(struct test_desc *test)
    {
    unlink(test.bpffs_path);
    rmdir(BPFFS_DIR);
    }
#[no_mangle]
unsafe extern "C" fn check_metadata(buf: *mut c_char, tokens: *const *const c_char, count: c_int) -> c_int {
    static int check_metadata(char *buf, char * const *tokens, int count)
    {
    int i;
    for (i = 0; i < count && tokens[i]; i++)
    if (!strstr(buf, tokens[i]))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_test(test: *mut test_desc) {
    static void run_test(struct test_desc *test)
    {
    int ret;
    char cmd[MAX_BPFTOOL_CMD_LEN];
    ret = snprintf(cmd, MAX_BPFTOOL_CMD_LEN, "prog load %s %s",
    test.bpf_prog, test.bpffs_path);
    if (!ASSERT_GT(ret, 0, "format prog insert command"))
    return;
    ret = run_bpftool_command(cmd);
    if (!ASSERT_OK(ret, "load program"))
    return;
// Check output with default format
    ret = snprintf(cmd, MAX_BPFTOOL_CMD_LEN, "prog show pinned %s",
    test.bpffs_path);
    if (!ASSERT_GT(ret, 0, "format pinned prog check command"))
    return;
    ret = get_bpftool_command_output(cmd, output,
    MAX_BPFTOOL_OUTPUT_LEN);
    if (ASSERT_OK(ret, "get program info")) {
    ret = check_metadata(output, test.expected_output,
    ARRAY_SIZE(test.expected_output));
    ASSERT_OK(ret, "find metadata");
    }
// Check output with json format
    ret = snprintf(cmd, MAX_BPFTOOL_CMD_LEN, "prog -j show pinned %s",
    test.bpffs_path);
    if (!ASSERT_GT(ret, 0, "format pinned prog check command in json"))
    return;
    ret = get_bpftool_command_output(cmd, output,
    MAX_BPFTOOL_OUTPUT_LEN);
    if (ASSERT_OK(ret, "get program info in json")) {
    ret = check_metadata(output, test.expected_output_json,
    ARRAY_SIZE(test.expected_output_json));
    ASSERT_OK(ret, "find metadata in json");
    }
// Check that the corresponding map can be found and accessed
    ret = snprintf(cmd, MAX_BPFTOOL_CMD_LEN, "map show name %s",
    test.metadata_map_name);
    if (!ASSERT_GT(ret, 0, "format map check command"))
    return;
    ASSERT_OK(run_bpftool_command(cmd), "access metadata map");
    }
    static struct test_desc tests[] = {
    {
    .name = "metadata_unused",
    .bpf_prog = BPF_FILE_UNUSED,
    .bpffs_path = BPFFS_UNUSED,
    .expected_output = {
    "a = \"foo\"",
    "b = 1"
    },
    .expected_output_json = {
    "\"metadata\":{\"a\":\"foo\",\"b\":1}"
    },
    .metadata_map_name = METADATA_MAP_NAME
    },
    {
    .name = "metadata_used",
    .bpf_prog = BPF_FILE_USED,
    .bpffs_path = BPFFS_USED,
    .expected_output = {
    "a = \"bar\"",
    "b = 2"
    },
    .expected_output_json = {
    "\"metadata\":{\"a\":\"bar\",\"b\":2}"
    },
    .metadata_map_name = METADATA_MAP_NAME
    }
    };
    let mut tests_count: static int = ARRAY_SIZE(tests);
#[no_mangle]
pub unsafe extern "C" fn test_bpftool_metadata() {
    void test_bpftool_metadata(void)
    {
    int i;
    for (i = 0; i < tests_count; i++) {
    if (!test__start_subtest(tests[i].name))
    continue;
    if (ASSERT_OK(setup(&tests[i]), "setup bpffs pin dir")) {
    run_test(&tests[i]);
    cleanup(&tests[i]);
    }
    }
    }
