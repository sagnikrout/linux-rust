//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/fchmodat2/fchmodat2_test.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct testdir {
    pub dirname: *mut c_char,
    pub dfd: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn sys_fchmodat2(dfd: c_int, filename: *const c_char, mode: mode_t, flags: c_int) -> c_int {
    int sys_fchmodat2(int dfd, const char *filename, mode_t mode, int flags)
    {
    let mut ret: c_int = syscall(__NR_fchmodat2, dfd, filename, mode, flags);
    return ret >= 0 ? ret : -errno;
    }
#[no_mangle]
unsafe extern "C" fn setup_testdir(testdir: *mut testdir) {
    static void setup_testdir(struct testdir *testdir)
    {
    int ret, dfd;
    char dirname[] = "/tmp/ksft-fchmodat2.XXXXXX";
// Make the top-level directory.
    if (!mkdtemp(dirname))
    ksft_exit_fail_msg("%s: failed to create tmpdir\n", __func__);
    dfd = open(dirname, O_PATH | O_DIRECTORY);
    if (dfd < 0) {
    ksft_perror("failed to open tmpdir");
    goto err;
    }
    ret = openat(dfd, "regfile", O_CREAT | O_WRONLY | O_TRUNC, 0644);
    if (ret < 0) {
    ksft_perror("failed to create file in tmpdir");
    goto err;
    }
    close(ret);
    ret = symlinkat("regfile", dfd, "symlink");
    if (ret < 0) {
    ksft_perror("symlinkat() failed");
    goto err_regfile;
    }
    testdir.dirname = strdup(dirname);
    if (!testdir.dirname) {
    ksft_perror("Out of memory");
    goto err_symlink;
    }
    testdir.dfd = dfd;
    return;
    err_symlink:
    unlinkat(testdir.dfd, "symlink", 0);
    err_regfile:
    unlinkat(testdir.dfd, "regfile", 0);
    err:
    unlink(dirname);
    ksft_exit_fail();
    }
#[no_mangle]
unsafe extern "C" fn cleanup_testdir(testdir: *mut testdir) {
    static void cleanup_testdir(struct testdir *testdir)
    {
    unlinkat(testdir.dfd, "regfile", 0);
    unlinkat(testdir.dfd, "symlink", 0);
    rmdir(testdir.dirname);
    free(testdir.dirname);
    }
#[no_mangle]
pub unsafe extern "C" fn expect_mode(dfd: c_int, filename: *const c_char, expect_mode: mode_t) -> c_int {
    int expect_mode(int dfd, const char *filename, mode_t expect_mode)
    {
    struct stat st;
    let mut ret: c_int = fstatat(dfd, filename, &st, AT_SYMLINK_NOFOLLOW);
    if (ret) {
    ksft_perror("fstatat() failed\n");
    return 0;
    }
    return (st.st_mode == expect_mode);
    }
#[no_mangle]
pub unsafe extern "C" fn test_regfile() {
    void test_regfile(void)
    {
    struct testdir testdir;
    int ret;
    setup_testdir(&testdir);
    ret = sys_fchmodat2(testdir.dfd, "regfile", 0640, 0);
    if (ret < 0) {
    ksft_perror("fchmodat2(noflag) failed");
    goto out;
    }
    if (!expect_mode(testdir.dfd, "regfile", 0100640)) {
    ksft_print_msg("%s: wrong file mode bits after fchmodat2\n",
    __func__);
    ret = 1;
    goto out;
    }
    ret = sys_fchmodat2(testdir.dfd, "regfile", 0600, AT_SYMLINK_NOFOLLOW);
    if (ret < 0) {
    ksft_perror("fchmodat2(AT_SYMLINK_NOFOLLOW) failed");
    goto out;
    }
    if (!expect_mode(testdir.dfd, "regfile", 0100600)) {
    ksft_print_msg("%s: wrong file mode bits after fchmodat2 with nofollow\n",
    __func__);
    ret = 1;
    }
    out:
    ksft_test_result(ret == 0, "fchmodat2(regfile)\n");
    cleanup_testdir(&testdir);
    }
#[no_mangle]
pub unsafe extern "C" fn test_symlink() {
    void test_symlink(void)
    {
    struct testdir testdir;
    int ret;
    setup_testdir(&testdir);
    ret = sys_fchmodat2(testdir.dfd, "symlink", 0640, 0);
    if (ret < 0) {
    ksft_perror("fchmodat2(noflag) failed");
    goto err;
    }
    if (!expect_mode(testdir.dfd, "regfile", 0100640)) {
    ksft_print_msg("%s: wrong file mode bits after fchmodat2\n",
    __func__);
    goto err;
    }
    if (!expect_mode(testdir.dfd, "symlink", 0120777)) {
    ksft_print_msg("%s: wrong symlink mode bits after fchmodat2\n",
    __func__);
    goto err;
    }
    ret = sys_fchmodat2(testdir.dfd, "symlink", 0600, AT_SYMLINK_NOFOLLOW);
//
// On certain filesystems (xfs or btrfs), chmod operation fails. So we
// first check the symlink target but if the operation fails we mark the
// test as skipped.
//
// https://sourceware.org/legacy-ml/libc-alpha/2020-02/msg00467.html
//
    if (ret == 0 && !expect_mode(testdir.dfd, "symlink", 0120600)) {
    ksft_print_msg("%s: wrong symlink mode bits after fchmodat2 with nofollow\n",
    __func__);
    ret = 1;
    goto err;
    }
    if (!expect_mode(testdir.dfd, "regfile", 0100640)) {
    ksft_print_msg("%s: wrong file mode bits after fchmodat2 with nofollow\n",
    __func__);
    }
    if (ret != 0)
    ksft_test_result_skip("fchmodat2(symlink)\n");
    else
    ksft_test_result_pass("fchmodat2(symlink)\n");
    cleanup_testdir(&testdir);
    return;
    err:
    ksft_test_result_fail("fchmodat2(symlink)\n");
    cleanup_testdir(&testdir);
    }
pub const NUM_TESTS: c_int = 2;
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    ksft_print_header();
    ksft_set_plan(NUM_TESTS);
    test_regfile();
    test_symlink();
    ksft_finished();
    }
