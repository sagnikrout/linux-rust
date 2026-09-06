//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/mount/nosymfollow-test.c
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
unsafe extern "C" fn die(fmt: *mut c_char, ...) {
    static void die(char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    exit(EXIT_FAILURE);
    }
    static void vmaybe_write_file(bool enoent_ok, char *filename, char *fmt,
    va_list ap)
    {
    ssize_t written;
    char buf[4096];
    int buf_len;
    int fd;
    buf_len = vsnprintf(buf, sizeof(buf), fmt, ap);
    if (buf_len < 0)
    die("vsnprintf failed: %s\n", strerror(errno));
    if (buf_len >= sizeof(buf))
    die("vsnprintf output truncated\n");
    fd = open(filename, O_WRONLY);
    if (fd < 0) {
    if ((errno == ENOENT) && enoent_ok)
    return;
    die("open of %s failed: %s\n", filename, strerror(errno));
    }
    written = write(fd, buf, buf_len);
    if (written != buf_len) {
    if (written >= 0) {
    die("short write to %s\n", filename);
    } else {
    die("write to %s failed: %s\n",
    filename, strerror(errno));
    }
    }
    if (close(fd) != 0)
    die("close of %s failed: %s\n", filename, strerror(errno));
    }
#[no_mangle]
unsafe extern "C" fn maybe_write_file(filename: *mut c_char, fmt: *mut c_char, ...) {
    static void maybe_write_file(char *filename, char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vmaybe_write_file(true, filename, fmt, ap);
    va_end(ap);
    }
#[no_mangle]
unsafe extern "C" fn write_file(filename: *mut c_char, fmt: *mut c_char, ...) {
    static void write_file(char *filename, char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vmaybe_write_file(false, filename, fmt, ap);
    va_end(ap);
    }
#[no_mangle]
unsafe extern "C" fn create_and_enter_ns() {
    static void create_and_enter_ns(void)
    {
    let mut uid: uid_t = getuid();
    let mut gid: gid_t = getgid();
    if (unshare(CLONE_NEWUSER) != 0)
    die("unshare(CLONE_NEWUSER) failed: %s\n", strerror(errno));
    maybe_write_file("/proc/self/setgroups", "deny");
    write_file("/proc/self/uid_map", "0 %d 1", uid);
    write_file("/proc/self/gid_map", "0 %d 1", gid);
    if (setgid(0) != 0)
    die("setgid(0) failed %s\n", strerror(errno));
    if (setuid(0) != 0)
    die("setuid(0) failed %s\n", strerror(errno));
    if (unshare(CLONE_NEWNS) != 0)
    die("unshare(CLONE_NEWNS) failed: %s\n", strerror(errno));
    }
#[no_mangle]
unsafe extern "C" fn setup_symlink() {
    static void setup_symlink(void)
    {
    int data, err;
    data = creat(DATA, O_RDWR);
    if (data < 0)
    die("creat failed: %s\n", strerror(errno));
    err = symlink(DATA, LINK);
    if (err < 0)
    die("symlink failed: %s\n", strerror(errno));
    if (close(data) != 0)
    die("close of %s failed: %s\n", DATA, strerror(errno));
    }
#[no_mangle]
unsafe extern "C" fn test_link_traversal(nosymfollow: bool) {
    static void test_link_traversal(bool nosymfollow)
    {
    int link;
    link = open(LINK, 0, O_RDWR);
    if (nosymfollow) {
    if ((link != -1 || errno != ELOOP)) {
    die("link traversal unexpected result: %d, %s\n",
    link, strerror(errno));
    }
    } else {
    if (link < 0)
    die("link traversal failed: %s\n", strerror(errno));
    if (close(link) != 0)
    die("close of link failed: %s\n", strerror(errno));
    }
    }
#[no_mangle]
unsafe extern "C" fn test_readlink() {
    static void test_readlink(void)
    {
    char buf[4096];
    ssize_t ret;
    bzero(buf, sizeof(buf));
    ret = readlink(LINK, buf, sizeof(buf));
    if (ret < 0)
    die("readlink failed: %s\n", strerror(errno));
    if (strcmp(buf, DATA) != 0)
    die("readlink strcmp failed: '%s' '%s'\n", buf, DATA);
    }
#[no_mangle]
unsafe extern "C" fn test_realpath() {
    static void test_realpath(void)
    {
    char *path = realpath(LINK, core::ptr::null_mut());
    if (!path)
    die("realpath failed: %s\n", strerror(errno));
    if (strcmp(path, DATA) != 0)
    die("realpath strcmp failed\n");
    free(path);
    }
#[no_mangle]
unsafe extern "C" fn test_statfs(nosymfollow: bool) {
    static void test_statfs(bool nosymfollow)
    {
    struct statfs buf;
    int ret;
    ret = statfs(TMP, &buf);
    if (ret)
    die("statfs failed: %s\n", strerror(errno));
    if (nosymfollow) {
    if ((buf.f_flags & ST_NOSYMFOLLOW) == 0)
    die("ST_NOSYMFOLLOW not set on %s\n", TMP);
    } else {
    if ((buf.f_flags & ST_NOSYMFOLLOW) != 0)
    die("ST_NOSYMFOLLOW set on %s\n", TMP);
    }
    }
#[no_mangle]
unsafe extern "C" fn run_tests(nosymfollow: bool) {
    static void run_tests(bool nosymfollow)
    {
    test_link_traversal(nosymfollow);
    test_readlink();
    test_realpath();
    test_statfs(nosymfollow);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    create_and_enter_ns();
    if (mount("testing", TMP, "ramfs", 0, core::ptr::null_mut()) != 0)
    die("mount failed: %s\n", strerror(errno));
    setup_symlink();
    run_tests(false);
    if (mount("testing", TMP, "ramfs", MS_REMOUNT|MS_NOSYMFOLLOW, core::ptr::null_mut()) != 0)
    die("remount failed: %s\n", strerror(errno));
    run_tests(true);
    return EXIT_SUCCESS;
    }
