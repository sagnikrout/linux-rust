//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/ustat_test.c
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
//
// Test ustat(2): looking up superblocks by device number.
//
// ustat() resolves a device number to a mounted superblock via
// user_get_super(). Check that the device number of a mounted tmpfs (an
// anonymous device) resolves, that it stops resolving once the filesystem
// is unmounted and that bogus device numbers report EINVAL.
//
// Macro flag: #define _GNU_SOURCE

// struct ustat is not exported through UAPI, mirror include/linux/types.h.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ustat_buf {
    pub f_tfree: c_int,
    pub f_tinode: c_ulong,
    pub f_fname: [c_char; 6],
    pub f_fpack: [c_char; 6],
// slack in case an architecture lays the struct out differently
    pub pad: [c_char; 64],
}

//
// The kernel decodes @dev with new_decode_dev(), which matches the low 32
// bits of the st_dev encoding stat(2) returns for any major below 4096.
//
#[no_mangle]
unsafe extern "C" fn sys_ustat(dev: c_uint, buf: *mut ustat_buf) -> c_int {
    static int sys_ustat(unsigned int dev, struct ustat_buf *buf)
    {
    return syscall(__NR_ustat, dev, buf);
    }
#[no_mangle]
unsafe extern "C" fn write_string(path: *const c_char, string: *const c_char) -> c_int {
    static int write_string(const char *path, const char *string)
    {
    let mut len: isize = strlen(string);
    int fd;
    fd = open(path, O_WRONLY);
    if (fd < 0)
    return -1;
    if (write(fd, string, len) != len) {
    close(fd);
    return -1;
    }
    return close(fd);
    }
// Enter namespaces in which mounting a tmpfs instance is allowed.
#[no_mangle]
unsafe extern "C" fn setup_namespaces() -> c_int {
    static int setup_namespaces(void)
    {
    let mut uid: uid_t = getuid();
    let mut gid: gid_t = getgid();
    char map[64];
    if (unshare(CLONE_NEWNS | (uid ? CLONE_NEWUSER : 0)))
    return -1;
    if (uid) {
    if (write_string("/proc/self/setgroups", "deny"))
    return -1;
    snprintf(map, sizeof(map), "0 %d 1", uid);
    if (write_string("/proc/self/uid_map", map))
    return -1;
    snprintf(map, sizeof(map), "0 %d 1", gid);
    if (write_string("/proc/self/gid_map", map))
    return -1;
    }
    return mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_REC | MS_PRIVATE, core::ptr::null_mut());
    }
    TEST(resolves_mounted_superblock)
    {
    char dir[] = "/tmp/ustat_test.XXXXXX";
    struct ustat_buf ub;
    struct stat st;
    ASSERT_NE(core::ptr::null_mut(), mkdtemp(dir));
    if (setup_namespaces()) {
    rmdir(dir);
    SKIP(return, "cannot set up namespaces: %s", strerror(errno));
    }
    ASSERT_EQ(0, mount("ustat_test", dir, "tmpfs", 0, core::ptr::null_mut()));
    ASSERT_EQ(0, stat(dir, &st));
    memset(&ub, 0xff, sizeof(ub));
    ASSERT_EQ(0, sys_ustat(st.st_dev, &ub))
    TH_LOG("ustat(%u): %s", (unsigned int)st.st_dev,
    strerror(errno));
    ASSERT_EQ(0, umount(dir));
// The unmount removed the superblock, the device is gone.
    ASSERT_EQ(-1, sys_ustat(st.st_dev, &ub));
    ASSERT_EQ(EINVAL, errno);
    rmdir(dir);
    }
    TEST(bogus_device_numbers)
    {
    struct ustat_buf ub;
    ASSERT_EQ(-1, sys_ustat(0, &ub));
    ASSERT_EQ(EINVAL, errno);
// major 4095, minor 1048575: nothing plausible lives there
    ASSERT_EQ(-1, sys_ustat((0xfffu << 8) | 0xffu | (0xfff00u << 12), &ub));
    ASSERT_EQ(EINVAL, errno);
    }

    TEST(unsupported)
    {
    SKIP(return, "ustat(2) is not available on this architecture");
    }

    TEST_HARNESS_MAIN
