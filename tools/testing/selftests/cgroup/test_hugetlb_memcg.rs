//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cgroup/test_hugetlb_memcg.c
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
// Macro flag: #define _GNU_SOURCE

// mapping 8 MBs == 4 hugepages

// borrowed from mm/hmm-tests.c
#[no_mangle]
unsafe extern "C" fn get_hugepage_size() -> c_long {
    static long get_hugepage_size(void)
    {
    int fd;
    char buf[2048];
    int len;
    char *p, *q, *path = "/proc/meminfo", *tag = "Hugepagesize:";
    long val;
    fd = open(path, O_RDONLY);
    if (fd < 0) {
// Error opening the file
    return -1;
    }
    len = read(fd, buf, sizeof(buf));
    close(fd);
    if (len < 0) {
// Error in reading the file
    return -1;
    }
    if (len == sizeof(buf)) {
// Error file is too large
    return -1;
    }
    buf[len] = '\0';
// Search for a tag if provided
    if (tag) {
    p = strstr(buf, tag);
    if (!p)
    return -1; /* looks like the line we want isn't there */
    p += strlen(tag);
    } else
    p = buf;
    val = strtol(p, &q, 0);
    if (*q != ' ') {
// Error parsing the file
    return -1;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn set_file(path: *const c_char, value: c_long) -> c_int {
    static int set_file(const char *path, long value)
    {
    FILE *file;
    int ret;
    file = fopen(path, "w");
    if (!file)
    return -1;
    ret = fprintf(file, "%ld\n", value);
    fclose(file);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_nr_hugepages(value: c_long) -> c_int {
    static int set_nr_hugepages(long value)
    {
    return set_file("/proc/sys/vm/nr_hugepages", value);
    }
#[no_mangle]
unsafe extern "C" fn check_first(addr: *mut c_char) -> c_uint {
    static unsigned int check_first(char *addr)
    {
    return *(unsigned int *)addr;
    }
#[no_mangle]
unsafe extern "C" fn write_data(addr: *mut c_char) {
    static void write_data(char *addr)
    {
    unsigned long i;
    for (i = 0; i < LENGTH; i++)
// (addr + i) = (char)i;
    }
#[no_mangle]
unsafe extern "C" fn hugetlb_test_program(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int hugetlb_test_program(const char *cgroup, void *arg)
    {
    char *test_group = (char *)arg;
    void *addr;
    long old_current, expected_current, current;
    let mut ret: c_int = EXIT_FAILURE;
    old_current = cg_read_long(test_group, "memory.current");
    set_nr_hugepages(20);
    current = cg_read_long(test_group, "memory.current");
    if (current - old_current >= MB(2)) {
    ksft_print_msg(
    "setting nr_hugepages should not increase hugepage usage.\n");
    ksft_print_msg("before: %ld, after: %ld\n", old_current, current);
    return EXIT_FAILURE;
    }
    addr = mmap(ADDR, LENGTH, PROTECTION, FLAGS, 0, 0);
    if (addr == MAP_FAILED) {
    ksft_print_msg("fail to mmap.\n");
    return EXIT_FAILURE;
    }
    current = cg_read_long(test_group, "memory.current");
    if (current - old_current >= MB(2)) {
    ksft_print_msg("mmap should not increase hugepage usage.\n");
    ksft_print_msg("before: %ld, after: %ld\n", old_current, current);
    goto out_failed_munmap;
    }
    old_current = current;
// read the first page
    check_first(addr);
    expected_current = old_current + MB(2);
    current = cg_read_long(test_group, "memory.current");
    if (!values_close(expected_current, current, 5)) {
    ksft_print_msg("memory usage should increase by around 2MB.\n");
    ksft_print_msg(
    "expected memory: %ld, actual memory: %ld\n",
    expected_current, current);
    goto out_failed_munmap;
    }
// write to the whole range
    write_data(addr);
    current = cg_read_long(test_group, "memory.current");
    expected_current = old_current + MB(8);
    if (!values_close(expected_current, current, 5)) {
    ksft_print_msg("memory usage should increase by around 8MB.\n");
    ksft_print_msg(
    "expected memory: %ld, actual memory: %ld\n",
    expected_current, current);
    goto out_failed_munmap;
    }
// unmap the whole range
    munmap(addr, LENGTH);
    current = cg_read_long(test_group, "memory.current");
    expected_current = old_current;
    if (!values_close(expected_current, current, 5)) {
    ksft_print_msg("memory usage should go back down.\n");
    ksft_print_msg(
    "expected memory: %ld, actual memory: %ld\n",
    expected_current, current);
    return ret;
    }
    ret = EXIT_SUCCESS;
    return ret;
    out_failed_munmap:
    munmap(addr, LENGTH);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_hugetlb_memcg(root: *mut c_char) -> c_int {
    static int test_hugetlb_memcg(char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *test_group;
    test_group = cg_name(root, "hugetlb_memcg_test");
    if (!test_group || cg_create(test_group)) {
    ksft_print_msg("fail to create cgroup.\n");
    goto out;
    }
    if (cg_write(test_group, "memory.max", "100M")) {
    ksft_print_msg("fail to set cgroup memory limit.\n");
    goto out;
    }
// disable swap
    if (cg_write(test_group, "memory.swap.max", "0")) {
    ksft_print_msg("fail to disable swap.\n");
    goto out;
    }
    if (!cg_run(test_group, hugetlb_test_program, (void *)test_group))
    ret = KSFT_PASS;
    out:
    cg_destroy(test_group);
    free(test_group);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char root[PATH_MAX];
    int has_memory_hugetlb_acc;
    ksft_print_header();
    ksft_set_plan(1);
    has_memory_hugetlb_acc = proc_mount_contains("memory_hugetlb_accounting");
    if (has_memory_hugetlb_acc < 0)
    ksft_exit_skip("Failed to query cgroup mount option\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !has_memory_hugetlb_acc) -> else {
    else if (!has_memory_hugetlb_acc)
    ksft_exit_skip("memory hugetlb accounting is disabled\n");
// Unit is kB!
    if (get_hugepage_size() != 2048) {
    ksft_print_msg("test_hugetlb_memcg requires 2MB hugepages\n");
    ksft_test_result_skip("test_hugetlb_memcg\n");
    ksft_finished();
    }
    if (cg_find_unified_root(root, sizeof(root), core::ptr::null_mut()))
    ksft_exit_skip("cgroup v2 isn't mounted\n");
    if (cg_read_strstr(root, "cgroup.controllers", "memory"))
    ksft_exit_skip("memory controller isn't available\n");
    if (cg_read_strstr(root, "cgroup.subtree_control", "memory")) {
    if (cg_write(root, "cgroup.subtree_control", "+memory"))
    ksft_exit_skip("Failed to set memory controller\n");
    }
    switch (test_hugetlb_memcg(root)) {
    case KSFT_PASS:
    ksft_test_result_pass("test_hugetlb_memcg\n");
    break;
    case KSFT_SKIP:
    ksft_test_result_skip("test_hugetlb_memcg\n");
    break;
    default:
    ksft_test_result_fail("test_hugetlb_memcg\n");
    break;
    }
    ksft_finished();
    }
