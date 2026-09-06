//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/tty/tty_tstamp_update.c
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

pub const MIN_TTY_PATH_LEN: c_int = 8;
#[no_mangle]
unsafe extern "C" fn tty_valid(tty: *mut c_char) -> bool {
    static bool tty_valid(char *tty)
    {
    if (strlen(tty) < MIN_TTY_PATH_LEN)
    return false;
    if (strncmp(tty, "/dev/tty", MIN_TTY_PATH_LEN) == 0 ||
    strncmp(tty, "/dev/pts", MIN_TTY_PATH_LEN) == 0)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn write_dev_tty() -> c_int {
    static int write_dev_tty(void)
    {
    FILE *f;
    let mut r: c_int = 0;
    f = fopen("/dev/tty", "r+");
    if (!f)
    return -errno;
    r = fprintf(f, "hello, world!\n");
    if (r != strlen("hello, world!\n"))
    r = -EIO;
    fclose(f);
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int r;
    char tty[PATH_MAX] = {};
    struct stat st1, st2;
    let mut result: c_int = KSFT_FAIL;
    ksft_print_header();
    ksft_set_plan(1);
    r = readlink("/proc/self/fd/0", tty, PATH_MAX);
    if (r < 0) {
    ksft_print_msg("readlink on /proc/self/fd/0 failed: %m\n");
    goto out;
    }
    if (!tty_valid(tty)) {
    ksft_print_msg("invalid tty path '%s'\n", tty);
    result = KSFT_SKIP;
    goto out;
    }
    r = stat(tty, &st1);
    if (r < 0) {
    ksft_print_msg("stat failed on tty path '%s': %m\n", tty);
    goto out;
    }
// We need to wait at least 8 seconds in order to observe timestamp change
// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=fbf47635315ab308c9b58a1ea0906e711a9228de
    sleep(10);
    r = write_dev_tty();
    if (r < 0) {
    ksft_print_msg("failed to write to /dev/tty: %s\n",
    strerror(-r));
    goto out;
    }
    r = stat(tty, &st2);
    if (r < 0) {
    ksft_print_msg("stat failed on tty path '%s': %m\n", tty);
    goto out;
    }
// We wrote to the terminal so timestamps should have been updated
    if (st1.st_atim.tv_sec == st2.st_atim.tv_sec &&
    st1.st_mtim.tv_sec == st2.st_mtim.tv_sec) {
    ksft_print_msg("tty timestamps not updated\n");
    goto out;
    }
    ksft_print_msg(
    "timestamps of terminal '%s' updated after write to /dev/tty\n", tty);
    result = KSFT_PASS;
    out:
    ksft_test_result_report(result, "tty_tstamp_update\n");
    ksft_finished();
    }
