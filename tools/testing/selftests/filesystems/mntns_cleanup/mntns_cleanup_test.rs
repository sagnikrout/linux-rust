//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/mntns_cleanup/mntns_cleanup_test.c
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

    FIXTURE(mntns_cleanup) {
    };
    FIXTURE_SETUP(mntns_cleanup)
    {
    if (geteuid() != 0)
    SKIP(return, "test requires CAP_SYS_ADMIN");
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ(mount("", "/", core::ptr::null_mut(), MS_REC | MS_PRIVATE, core::ptr::null_mut()), 0);
    rmdir("/mnt_dir");
    ASSERT_EQ(mkdir("/mnt_dir", 0755), 0);
    ASSERT_EQ(mount("tmpfs", "/mnt_dir", "tmpfs", 0, core::ptr::null_mut()), 0);
    ASSERT_EQ(mkdir("/mnt_dir/hidden", 0755), 0);
    ASSERT_EQ(mkdir("/mnt_dir/hidden/secret", 0755), 0);
    ASSERT_EQ(mount("tmpfs", "/mnt_dir/hidden", "tmpfs", 0, core::ptr::null_mut()), 0);
    }
    FIXTURE_TEARDOWN(mntns_cleanup)
    {
    }
// Mounts must stay connected when a mount namespace is cleaned up.
    TEST_F(mntns_cleanup, keeps_mounts_connected)
    {
    int fd, sfd, err;
    fd = open("/mnt_dir", O_PATH | O_DIRECTORY | O_CLOEXEC);
    ASSERT_GE(fd, 0);
// Destroy the namespace; the fd keeps /mnt_dir alive.
    ASSERT_EQ(unshare(CLONE_NEWNS), 0);
    sfd = openat(fd, "hidden/secret", O_RDONLY);
    err = errno;
    if (sfd >= 0)
    close(sfd);
    close(fd);
    ASSERT_LT(sfd, 0)
    TH_LOG("mount namespace teardown revealed what the overmount covered");
    ASSERT_EQ(err, ENOENT);
    }
    TEST_HARNESS_MAIN
