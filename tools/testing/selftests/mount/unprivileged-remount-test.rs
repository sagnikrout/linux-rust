//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/mount/unprivileged-remount-test.c
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
#[no_mangle]
unsafe extern "C" fn vmaybe_write_file(enoent_ok: bool, filename: *mut c_char, fmt: *mut c_char, ap: va_list) {
    static void vmaybe_write_file(bool enoent_ok, char *filename, char *fmt, va_list ap)
    {
    char buf[4096];
    int fd;
    ssize_t written;
    int buf_len;
    buf_len = vsnprintf(buf, sizeof(buf), fmt, ap);
    if (buf_len < 0) {
    die("vsnprintf failed: %s\n",
    strerror(errno));
    }
    if (buf_len >= sizeof(buf)) {
    die("vsnprintf output truncated\n");
    }
    fd = open(filename, O_WRONLY);
    if (fd < 0) {
    if ((errno == ENOENT) && enoent_ok)
    return;
    die("open of %s failed: %s\n",
    filename, strerror(errno));
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
    if (close(fd) != 0) {
    die("close of %s failed: %s\n",
    filename, strerror(errno));
    }
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
unsafe extern "C" fn read_mnt_flags(path: *const c_char) -> c_int {
    static int read_mnt_flags(const char *path)
    {
    int ret;
    struct statvfs stat;
    int mnt_flags;
    ret = statvfs(path, &stat);
    if (ret != 0) {
    die("statvfs of %s failed: %s\n",
    path, strerror(errno));
    }
    if (stat.f_flag & ~(ST_RDONLY | ST_NOSUID | ST_NODEV | \
    ST_NOEXEC | ST_NOATIME | ST_NODIRATIME | ST_RELATIME | \
    ST_SYNCHRONOUS | ST_MANDLOCK)) {
    die("Unrecognized mount flags\n");
    }
    mnt_flags = 0;
    if (stat.f_flag & ST_RDONLY)
    mnt_flags |= MS_RDONLY;
    if (stat.f_flag & ST_NOSUID)
    mnt_flags |= MS_NOSUID;
    if (stat.f_flag & ST_NODEV)
    mnt_flags |= MS_NODEV;
    if (stat.f_flag & ST_NOEXEC)
    mnt_flags |= MS_NOEXEC;
    if (stat.f_flag & ST_NOATIME)
    mnt_flags |= MS_NOATIME;
    if (stat.f_flag & ST_NODIRATIME)
    mnt_flags |= MS_NODIRATIME;
    if (stat.f_flag & ST_RELATIME)
    mnt_flags |= MS_RELATIME;
    if (stat.f_flag & ST_SYNCHRONOUS)
    mnt_flags |= MS_SYNCHRONOUS;
    if (stat.f_flag & ST_MANDLOCK)
    mnt_flags |= ST_MANDLOCK;
    return mnt_flags;
    }
#[no_mangle]
unsafe extern "C" fn create_and_enter_userns() {
    static void create_and_enter_userns(void)
    {
    uid_t uid;
    gid_t gid;
    uid = getuid();
    gid = getgid();
    if (unshare(CLONE_NEWUSER) !=0) {
    die("unshare(CLONE_NEWUSER) failed: %s\n",
    strerror(errno));
    }
    maybe_write_file("/proc/self/setgroups", "deny");
    write_file("/proc/self/uid_map", "0 %d 1", uid);
    write_file("/proc/self/gid_map", "0 %d 1", gid);
    if (setgid(0) != 0) {
    die ("setgid(0) failed %s\n",
    strerror(errno));
    }
    if (setuid(0) != 0) {
    die("setuid(0) failed %s\n",
    strerror(errno));
    }
    }
    static
    bool test_unpriv_remount(const char *fstype, const char *mount_options,
    int mount_flags, int remount_flags, int invalid_flags)
    {
    pid_t child;
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
    return WEXITSTATUS(status) == EXIT_SUCCESS;
    }
    create_and_enter_userns();
    if (unshare(CLONE_NEWNS) != 0) {
    die("unshare(CLONE_NEWNS) failed: %s\n",
    strerror(errno));
    }
    if (mount("testing", "/tmp", fstype, mount_flags, mount_options) != 0) {
    die("mount of %s with options '%s' on /tmp failed: %s\n",
    fstype,
    mount_options? mount_options : "",
    strerror(errno));
    }
    create_and_enter_userns();
    if (unshare(CLONE_NEWNS) != 0) {
    die("unshare(CLONE_NEWNS) failed: %s\n",
    strerror(errno));
    }
    if (mount("/tmp", "/tmp", "none",
    MS_REMOUNT | MS_BIND | remount_flags, core::ptr::null_mut()) != 0) {
// system("cat /proc/self/mounts");
    die("remount of /tmp failed: %s\n",
    strerror(errno));
    }
    if (mount("/tmp", "/tmp", "none",
    MS_REMOUNT | MS_BIND | invalid_flags, core::ptr::null_mut()) == 0) {
// system("cat /proc/self/mounts");
    die("remount of /tmp with invalid flags "
    "succeeded unexpectedly\n");
    }
    exit(EXIT_SUCCESS);
    }
#[no_mangle]
unsafe extern "C" fn test_unpriv_remount_simple(mount_flags: c_int) -> bool {
    static bool test_unpriv_remount_simple(int mount_flags)
    {
    return test_unpriv_remount("ramfs", core::ptr::null_mut(), mount_flags, mount_flags, 0);
    }
#[no_mangle]
unsafe extern "C" fn test_unpriv_remount_atime(mount_flags: c_int, invalid_flags: c_int) -> bool {
    static bool test_unpriv_remount_atime(int mount_flags, int invalid_flags)
    {
    return test_unpriv_remount("ramfs", core::ptr::null_mut(), mount_flags, mount_flags,
    invalid_flags);
    }
#[no_mangle]
unsafe extern "C" fn test_priv_mount_unpriv_remount() -> bool {
    static bool test_priv_mount_unpriv_remount(void)
    {
    pid_t child;
    int ret;
    const char *orig_path = "/dev";
    const char *dest_path = "/tmp";
    int orig_mnt_flags, remount_mnt_flags;
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
    return WEXITSTATUS(status) == EXIT_SUCCESS;
    }
    orig_mnt_flags = read_mnt_flags(orig_path);
    create_and_enter_userns();
    ret = unshare(CLONE_NEWNS);
    if (ret != 0) {
    die("unshare(CLONE_NEWNS) failed: %s\n",
    strerror(errno));
    }
    ret = mount(orig_path, dest_path, "bind", MS_BIND | MS_REC, core::ptr::null_mut());
    if (ret != 0) {
    die("recursive bind mount of %s onto %s failed: %s\n",
    orig_path, dest_path, strerror(errno));
    }
    ret = mount(dest_path, dest_path, "none",
    MS_REMOUNT | MS_BIND | orig_mnt_flags , core::ptr::null_mut());
    if (ret != 0) {
// system("cat /proc/self/mounts");
    die("remount of /tmp failed: %s\n",
    strerror(errno));
    }
    remount_mnt_flags = read_mnt_flags(dest_path);
    if (orig_mnt_flags != remount_mnt_flags) {
    die("Mount flags unexpectedly changed during remount of %s originally mounted on %s\n",
    dest_path, orig_path);
    }
    exit(EXIT_SUCCESS);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    if (!test_unpriv_remount_simple(MS_RDONLY)) {
    die("MS_RDONLY malfunctions\n");
    }
    if (!test_unpriv_remount("devpts", "newinstance", MS_NODEV, MS_NODEV, 0)) {
    die("MS_NODEV malfunctions\n");
    }
    if (!test_unpriv_remount_simple(MS_NOSUID)) {
    die("MS_NOSUID malfunctions\n");
    }
    if (!test_unpriv_remount_simple(MS_NOEXEC)) {
    die("MS_NOEXEC malfunctions\n");
    }
    if (!test_unpriv_remount_atime(MS_RELATIME,
    MS_NOATIME))
    {
    die("MS_RELATIME malfunctions\n");
    }
    if (!test_unpriv_remount_atime(MS_STRICTATIME,
    MS_NOATIME))
    {
    die("MS_STRICTATIME malfunctions\n");
    }
    if (!test_unpriv_remount_atime(MS_NOATIME,
    MS_STRICTATIME))
    {
    die("MS_NOATIME malfunctions\n");
    }
    if (!test_unpriv_remount_atime(MS_RELATIME|MS_NODIRATIME,
    MS_NOATIME))
    {
    die("MS_RELATIME|MS_NODIRATIME malfunctions\n");
    }
    if (!test_unpriv_remount_atime(MS_STRICTATIME|MS_NODIRATIME,
    MS_NOATIME))
    {
    die("MS_STRICTATIME|MS_NODIRATIME malfunctions\n");
    }
    if (!test_unpriv_remount_atime(MS_NOATIME|MS_NODIRATIME,
    MS_STRICTATIME))
    {
    die("MS_NOATIME|MS_DIRATIME malfunctions\n");
    }
    if (!test_unpriv_remount("ramfs", core::ptr::null_mut(), MS_STRICTATIME, 0, MS_NOATIME))
    {
    die("Default atime malfunctions\n");
    }
    if (!test_priv_mount_unpriv_remount()) {
    die("Mount flags unexpectedly changed after remount\n");
    }
    return EXIT_SUCCESS;
    }
