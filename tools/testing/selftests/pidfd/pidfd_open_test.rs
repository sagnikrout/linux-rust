//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/pidfd/pidfd_open_test.c
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

#[no_mangle]
unsafe extern "C" fn safe_int(numstr: *const c_char, converted: *mut c_int) -> c_int {
    static int safe_int(const char *numstr, int *converted)
    {
    char *err = core::ptr::null_mut();
    long sli;
    errno = 0;
    sli = strtol(numstr, &err, 0);
    if (errno == ERANGE && (sli == LONG_MAX || sli == LONG_MIN))
    return -ERANGE;
    if (errno != 0 && sli == 0)
    return -EINVAL;
    if (err == numstr || *err != '\0')
    return -EINVAL;
    if (sli > INT_MAX || sli < INT_MIN)
    return -ERANGE;
// converted = (int)sli;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn char_left_gc(buffer: *const c_char, len: usize) -> c_int {
    static int char_left_gc(const char *buffer, size_t len)
    {
    size_t i;
    for (i = 0; i < len; i++) {
    if (buffer[i] == ' ' ||
    buffer[i] == '\t')
    continue;
    return i;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn char_right_gc(buffer: *const c_char, len: usize) -> c_int {
    static int char_right_gc(const char *buffer, size_t len)
    {
    int i;
    for (i = len - 1; i >= 0; i--) {
    if (buffer[i] == ' '  ||
    buffer[i] == '\t' ||
    buffer[i] == '\n' ||
    buffer[i] == '\0')
    continue;
    return i + 1;
    }
    return 0;
    }
    static char *trim_whitespace_in_place(char *buffer)
    {
    buffer += char_left_gc(buffer, strlen(buffer));
    buffer[char_right_gc(buffer, strlen(buffer))] = '\0';
    return buffer;
    }
#[no_mangle]
unsafe extern "C" fn get_pid_from_fdinfo_file(pidfd: c_int, key: *const c_char, keylen: usize) -> pid_t {
    static pid_t get_pid_from_fdinfo_file(int pidfd, const char *key, size_t keylen)
    {
    int ret;
    char path[512];
    FILE *f;
    let mut n: usize = 0;
    let mut result: pid_t = -1;
    char *line = core::ptr::null_mut();
    snprintf(path, sizeof(path), "/proc/self/fdinfo/%d", pidfd);
    f = fopen(path, "re");
    if (!f)
    return -1;
    while (getline(&line, &n, f) != -1) {
    char *numstr;
    if (strncmp(line, key, keylen))
    continue;
    numstr = trim_whitespace_in_place(line + 4);
    ret = safe_int(numstr, &result);
    if (ret < 0)
    goto out;
    break;
    }
    out:
    free(line);
    fclose(f);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct pidfd_info info = {
    .mask = PIDFD_INFO_CGROUPID,
    };
    let mut pidfd: c_int = -1, ret = 1;
    pid_t pid;
    ksft_set_plan(4);
    pidfd = sys_pidfd_open(-1, 0);
    if (pidfd >= 0) {
    ksft_print_msg(
    "%s - succeeded to open pidfd for invalid pid -1\n",
    strerror(errno));
    goto on_error;
    }
    ksft_test_result_pass("do not allow invalid pid test: passed\n");
    pidfd = sys_pidfd_open(getpid(), 1);
    if (pidfd >= 0) {
    ksft_print_msg(
    "%s - succeeded to open pidfd with invalid flag value specified\n",
    strerror(errno));
    goto on_error;
    }
    ksft_test_result_pass("do not allow invalid flag test: passed\n");
    pidfd = sys_pidfd_open(getpid(), 0);
    if (pidfd < 0) {
    ksft_print_msg("%s - failed to open pidfd\n", strerror(errno));
    goto on_error;
    }
    ksft_test_result_pass("open a new pidfd test: passed\n");
    pid = get_pid_from_fdinfo_file(pidfd, "Pid:", sizeof("Pid:") - 1);
    ksft_print_msg("pidfd %d refers to process with pid %d\n", pidfd, pid);
    if (ioctl(pidfd, PIDFD_GET_INFO, &info) < 0) {
    ksft_print_msg("%s - failed to get info from pidfd\n", strerror(errno));
    goto on_error;
    }
    if (info.pid != pid) {
    ksft_print_msg("pid from fdinfo file %d does not match pid from ioctl %d\n",
    pid, info.pid);
    goto on_error;
    }
    if (info.ppid != getppid()) {
    ksft_print_msg("ppid %d does not match ppid from ioctl %d\n",
    pid, info.pid);
    goto on_error;
    }
    if (info.ruid != getuid()) {
    ksft_print_msg("uid %d does not match uid from ioctl %d\n",
    getuid(), info.ruid);
    goto on_error;
    }
    if (info.rgid != getgid()) {
    ksft_print_msg("gid %d does not match gid from ioctl %d\n",
    getgid(), info.rgid);
    goto on_error;
    }
    if (info.euid != geteuid()) {
    ksft_print_msg("euid %d does not match euid from ioctl %d\n",
    geteuid(), info.euid);
    goto on_error;
    }
    if (info.egid != getegid()) {
    ksft_print_msg("egid %d does not match egid from ioctl %d\n",
    getegid(), info.egid);
    goto on_error;
    }
    if (info.suid != geteuid()) {
    ksft_print_msg("suid %d does not match suid from ioctl %d\n",
    geteuid(), info.suid);
    goto on_error;
    }
    if (info.sgid != getegid()) {
    ksft_print_msg("sgid %d does not match sgid from ioctl %d\n",
    getegid(), info.sgid);
    goto on_error;
    }
    if ((info.mask & PIDFD_INFO_CGROUPID) && info.cgroupid == 0) {
    ksft_print_msg("cgroupid should not be 0 when PIDFD_INFO_CGROUPID is set\n");
    goto on_error;
    }
    ksft_test_result_pass("get info from pidfd test: passed\n");
    ret = 0;
    on_error:
    if (pidfd >= 0)
    close(pidfd);
    if (ret)
    ksft_exit_fail();
    ksft_exit_pass();
    }
