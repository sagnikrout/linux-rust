//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/firmware/fw_namespace.c
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
// Test triggering of loading of firmware from different mount
// namespaces. Expect firmware to be always loaded from the mount
// namespace of PID 1.
// Macro flag: #define _GNU_SOURCE

    static char *fw_path = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn die(fmt: *mut c_char, ...) {
    static void die(char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    if (fw_path)
    unlink(fw_path);
    umount("/lib/firmware");
    exit(EXIT_FAILURE);
    }
#[no_mangle]
unsafe extern "C" fn trigger_fw(fw_name: *const c_char, sys_path: *const c_char) {
    static void trigger_fw(const char *fw_name, const char *sys_path)
    {
    int fd;
    fd = open(sys_path, O_WRONLY);
    if (fd < 0)
    die("open failed: %s\n",
    strerror(errno));
    if (write(fd, fw_name, strlen(fw_name)) != strlen(fw_name))
    exit(EXIT_FAILURE);
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn setup_fw(fw_path: *const c_char) {
    static void setup_fw(const char *fw_path)
    {
    int fd;
    const char fw[] = "ABCD0123";
    fd = open(fw_path, O_WRONLY | O_CREAT, 0600);
    if (fd < 0)
    die("open failed: %s\n",
    strerror(errno));
    if (write(fd, fw, sizeof(fw) -1) != sizeof(fw) -1)
    die("write failed: %s\n",
    strerror(errno));
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn test_fw_in_ns(fw_name: *const c_char, sys_path: *const c_char, block_fw_in_parent_ns: bool) -> bool {
    static bool test_fw_in_ns(const char *fw_name, const char *sys_path, bool block_fw_in_parent_ns)
    {
    pid_t child;
    if (block_fw_in_parent_ns)
    if (mount("test", "/lib/firmware", "tmpfs", MS_RDONLY, core::ptr::null_mut()) == -1)
    die("blocking firmware in parent ns failed\n");
    child = fork();
    if (child == -1) {
    die("fork failed: %s\n",
    strerror(errno));
    }
    if (child != 0) { /* parent */
    pid_t pid;
    int status;
    pid = waitpid(child, &status, 0);
    if (pid == -1) {
    die("waitpid failed: %s\n",
    strerror(errno));
    }
    if (pid != child) {
    die("waited for %d got %d\n",
    child, pid);
    }
    if (!WIFEXITED(status)) {
    die("child did not terminate cleanly\n");
    }
    if (block_fw_in_parent_ns)
    umount("/lib/firmware");
    return WEXITSTATUS(status) == EXIT_SUCCESS;
    }
    if (unshare(CLONE_NEWNS) != 0) {
    die("unshare(CLONE_NEWNS) failed: %s\n",
    strerror(errno));
    }
    if (mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_SLAVE|MS_REC, core::ptr::null_mut()) == -1)
    die("remount root in child ns failed\n");
    if (!block_fw_in_parent_ns) {
    if (mount("test", "/lib/firmware", "tmpfs", MS_RDONLY, core::ptr::null_mut()) == -1)
    die("blocking firmware in child ns failed\n");
    } else
    umount("/lib/firmware");
    trigger_fw(fw_name, sys_path);
    exit(EXIT_SUCCESS);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *fw_name = "test-firmware.bin";
    char *sys_path;
    if (argc != 2)
    die("usage: %s sys_path\n", argv[0]);
// Mount tmpfs to /lib/firmware so we don't have to assume
    that it is writable for us.*/
    if (mount("test", "/lib/firmware", "tmpfs", 0, core::ptr::null_mut()) == -1)
    die("mounting tmpfs to /lib/firmware failed\n");
    sys_path = argv[1];
    if (asprintf(&fw_path, "/lib/firmware/%s", fw_name) < 0)
    die("error: failed to build full fw_path\n");
    setup_fw(fw_path);
    setvbuf(stdout, core::ptr::null_mut(), _IONBF, 0);
// Positive case: firmware in PID1 mount namespace
    printf("Testing with firmware in parent namespace (assumed to be same file system as PID1)\n");
    if (!test_fw_in_ns(fw_name, sys_path, false))
    die("error: failed to access firmware\n");
// Negative case: firmware in child mount namespace, expected to fail
    printf("Testing with firmware in child namespace\n");
    if (test_fw_in_ns(fw_name, sys_path, true))
    die("error: firmware access did not fail\n");
    unlink(fw_path);
    free(fw_path);
    umount("/lib/firmware");
    exit(EXIT_SUCCESS);
    }
