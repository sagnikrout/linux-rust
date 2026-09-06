//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/nsfs/iterate_mntns.c
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
// Copyright (c) 2024 Christian Brauner <brauner@kernel.org>
// Macro flag: #define _GNU_SOURCE

pub const MNT_NS_COUNT: c_int = 11;
pub const MNT_NS_LAST_INDEX: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mnt_ns_info {
    pub size: __u32,
    pub nr_mounts: __u32,
    pub mnt_ns_id: __u64,
}

// Get information about namespace.

// Get next namespace.

// Get previous namespace.

    FIXTURE(iterate_mount_namespaces) {
    int fd_mnt_ns[MNT_NS_COUNT];
    __u64 mnt_ns_id[MNT_NS_COUNT];
    };
#[no_mangle]
pub unsafe extern "C" fn mntns_in_list(mnt_ns_id: *mut __u64, info: *mut mnt_ns_info) -> bool {
    static inline bool mntns_in_list(__u64 *mnt_ns_id, struct mnt_ns_info *info)
    {
    for (int i = 0; i < MNT_NS_COUNT; i++) {
    if (mnt_ns_id[i] == info.mnt_ns_id)
    return true;
    }
    return false;
    }
    FIXTURE_SETUP(iterate_mount_namespaces)
    {
    for (int i = 0; i < MNT_NS_COUNT; i++)
    self.fd_mnt_ns[i] = -EBADF;
    for (int i = 0; i < MNT_NS_COUNT; i++) {
    let mut info: mnt_ns_info = {};
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    self.fd_mnt_ns[i] = open("/proc/self/ns/mnt", O_RDONLY | O_CLOEXEC);
    ASSERT_GE(self.fd_mnt_ns[i], 0);
    ASSERT_EQ(ioctl(self.fd_mnt_ns[i], NS_MNT_GET_INFO, &info), 0);
    self.mnt_ns_id[i] = info.mnt_ns_id;
    }
    }
    FIXTURE_TEARDOWN(iterate_mount_namespaces)
    {
    for (int i = 0; i < MNT_NS_COUNT; i++) {
    if (self.fd_mnt_ns[i] < 0)
    continue;
    ASSERT_EQ(close(self.fd_mnt_ns[i]), 0);
    }
    }
    TEST_F(iterate_mount_namespaces, iterate_all_forward)
    {
    int fd_mnt_ns_cur, count = 0;
    fd_mnt_ns_cur = fcntl(self.fd_mnt_ns[0], F_DUPFD_CLOEXEC);
    ASSERT_GE(fd_mnt_ns_cur, 0);
    for (;;) {
    let mut info: mnt_ns_info = {};
    int fd_mnt_ns_next;
    fd_mnt_ns_next = ioctl(fd_mnt_ns_cur, NS_MNT_GET_NEXT, &info);
    if (fd_mnt_ns_next < 0 && errno == ENOENT)
    break;
    if (mntns_in_list(self.mnt_ns_id, &info))
    count++;
    ASSERT_GE(fd_mnt_ns_next, 0);
    ASSERT_EQ(close(fd_mnt_ns_cur), 0);
    fd_mnt_ns_cur = fd_mnt_ns_next;
    }
    ASSERT_EQ(count, MNT_NS_LAST_INDEX);
    }
    TEST_F(iterate_mount_namespaces, iterate_all_backwards)
    {
    int fd_mnt_ns_cur, count = 0;
    fd_mnt_ns_cur = fcntl(self.fd_mnt_ns[MNT_NS_LAST_INDEX], F_DUPFD_CLOEXEC);
    ASSERT_GE(fd_mnt_ns_cur, 0);
    for (;;) {
    let mut info: mnt_ns_info = {};
    int fd_mnt_ns_prev;
    fd_mnt_ns_prev = ioctl(fd_mnt_ns_cur, NS_MNT_GET_PREV, &info);
    if (fd_mnt_ns_prev < 0 && errno == ENOENT)
    break;
    if (mntns_in_list(self.mnt_ns_id, &info))
    count++;
    ASSERT_GE(fd_mnt_ns_prev, 0);
    ASSERT_EQ(close(fd_mnt_ns_cur), 0);
    fd_mnt_ns_cur = fd_mnt_ns_prev;
    }
    ASSERT_EQ(count, MNT_NS_LAST_INDEX);
    }
    TEST_F(iterate_mount_namespaces, iterate_forward)
    {
    int fd_mnt_ns_cur;
    ASSERT_EQ(setns(self.fd_mnt_ns[0], CLONE_NEWNS), 0);
    fd_mnt_ns_cur = self.fd_mnt_ns[0];
    for (int i = 1; i < MNT_NS_COUNT; i++) {
    let mut info: mnt_ns_info = {};
    int fd_mnt_ns_next;
    fd_mnt_ns_next = ioctl(fd_mnt_ns_cur, NS_MNT_GET_NEXT, &info);
    ASSERT_GE(fd_mnt_ns_next, 0);
    ASSERT_EQ(close(fd_mnt_ns_cur), 0);
    fd_mnt_ns_cur = fd_mnt_ns_next;
    }
    }
    TEST_F(iterate_mount_namespaces, iterate_backward)
    {
    int fd_mnt_ns_cur;
    ASSERT_EQ(setns(self.fd_mnt_ns[MNT_NS_LAST_INDEX], CLONE_NEWNS), 0);
    fd_mnt_ns_cur = self.fd_mnt_ns[MNT_NS_LAST_INDEX];
    for (int i = MNT_NS_LAST_INDEX - 1; i >= 0; i--) {
    let mut info: mnt_ns_info = {};
    int fd_mnt_ns_prev;
    fd_mnt_ns_prev = ioctl(fd_mnt_ns_cur, NS_MNT_GET_PREV, &info);
    ASSERT_GE(fd_mnt_ns_prev, 0);
    ASSERT_EQ(close(fd_mnt_ns_cur), 0);
    fd_mnt_ns_cur = fd_mnt_ns_prev;
    }
    }
    TEST_F(iterate_mount_namespaces, nfs_valid_ioctl)
    {
    ASSERT_NE(ioctl(self.fd_mnt_ns[0], AUTOFS_DEV_IOCTL_OPENMOUNT, core::ptr::null_mut()), 0);
    ASSERT_EQ(errno, ENOTTY);
    ASSERT_NE(ioctl(self.fd_mnt_ns[0], AUTOFS_DEV_IOCTL_CLOSEMOUNT, core::ptr::null_mut()), 0);
    ASSERT_EQ(errno, ENOTTY);
    ASSERT_NE(ioctl(self.fd_mnt_ns[0], AUTOFS_DEV_IOCTL_READY, core::ptr::null_mut()), 0);
    ASSERT_EQ(errno, ENOTTY);
    }
    TEST_HARNESS_MAIN
