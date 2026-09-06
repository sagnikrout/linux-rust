//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/fclog.c
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
//
// Author: Aleksa Sarai <cyphar@cyphar.com>
// Copyright (C) 2025 SUSE LLC.
//

    __EXPECT(expected, #expected,					\
    ({__typeof__(seen) _tmp_seen = (seen);			\
    _tmp_seen >= 0 ? _tmp_seen : -errno; }), #seen, _t, 1)

    ASSERT_ERRNO(expected, ==, seen)

    ASSERT_ERRNO(0, <=, seen)
    FIXTURE(ns)
    {
    int host_mntns;
    };
    FIXTURE_SETUP(ns)
    {
// Stash the old mntns.
    self.host_mntns = open("/proc/self/ns/mnt", O_RDONLY|O_CLOEXEC);
    ASSERT_SUCCESS(self.host_mntns);
// Create a new mount namespace and make it private.
    ASSERT_SUCCESS(unshare(CLONE_NEWNS));
    ASSERT_SUCCESS(mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_PRIVATE|MS_REC, core::ptr::null_mut()));
    }
    FIXTURE_TEARDOWN(ns)
    {
    ASSERT_SUCCESS(setns(self.host_mntns, CLONE_NEWNS));
    ASSERT_SUCCESS(close(self.host_mntns));
    }
    TEST_F(ns, fscontext_log_enodata)
    {
    let mut fsfd: c_int = fsopen("tmpfs", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
// A brand new fscontext has no log entries.
    char buf[128] = {};
    for (int i = 0; i < 16; i++)
    ASSERT_ERRNO_EQ(-ENODATA, read(fsfd, buf, sizeof(buf)));
    ASSERT_SUCCESS(close(fsfd));
    }
    TEST_F(ns, fscontext_log_errorfc)
    {
    let mut fsfd: c_int = fsopen("tmpfs", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
    ASSERT_ERRNO_EQ(-EINVAL, fsconfig(fsfd, FSCONFIG_SET_STRING, "invalid-arg", "123", 0));
    char buf[128] = {};
    ASSERT_SUCCESS(read(fsfd, buf, sizeof(buf)));
    EXPECT_STREQ("e tmpfs: Unknown parameter 'invalid-arg'\n", buf);
// The message has been consumed.
    ASSERT_ERRNO_EQ(-ENODATA, read(fsfd, buf, sizeof(buf)));
    ASSERT_SUCCESS(close(fsfd));
    }
    TEST_F(ns, fscontext_log_errorfc_after_fsmount)
    {
    let mut fsfd: c_int = fsopen("tmpfs", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
    ASSERT_ERRNO_EQ(-EINVAL, fsconfig(fsfd, FSCONFIG_SET_STRING, "invalid-arg", "123", 0));
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0));
    let mut mfd: c_int = fsmount(fsfd, FSMOUNT_CLOEXEC, MOUNT_ATTR_NOEXEC | MOUNT_ATTR_NOSUID);
    ASSERT_SUCCESS(mfd);
    ASSERT_SUCCESS(move_mount(mfd, "", AT_FDCWD, "/tmp", MOVE_MOUNT_F_EMPTY_PATH));
//
// The fscontext log should still contain data even after
// FSCONFIG_CMD_CREATE and fsmount().
//
    char buf[128] = {};
    ASSERT_SUCCESS(read(fsfd, buf, sizeof(buf)));
    EXPECT_STREQ("e tmpfs: Unknown parameter 'invalid-arg'\n", buf);
// The message has been consumed.
    ASSERT_ERRNO_EQ(-ENODATA, read(fsfd, buf, sizeof(buf)));
    ASSERT_SUCCESS(close(fsfd));
    }
    TEST_F(ns, fscontext_log_emsgsize)
    {
    let mut fsfd: c_int = fsopen("tmpfs", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
    ASSERT_ERRNO_EQ(-EINVAL, fsconfig(fsfd, FSCONFIG_SET_STRING, "invalid-arg", "123", 0));
    char buf[128] = {};
//
// Attempting to read a message with too small a buffer should not
// result in the message getting consumed.
//
    ASSERT_ERRNO_EQ(-EMSGSIZE, read(fsfd, buf, 0));
    ASSERT_ERRNO_EQ(-EMSGSIZE, read(fsfd, buf, 1));
    for (int i = 0; i < 16; i++)
    ASSERT_ERRNO_EQ(-EMSGSIZE, read(fsfd, buf, 16));
    ASSERT_SUCCESS(read(fsfd, buf, sizeof(buf)));
    EXPECT_STREQ("e tmpfs: Unknown parameter 'invalid-arg'\n", buf);
// The message has been consumed.
    ASSERT_ERRNO_EQ(-ENODATA, read(fsfd, buf, sizeof(buf)));
    ASSERT_SUCCESS(close(fsfd));
    }
    TEST_HARNESS_MAIN
