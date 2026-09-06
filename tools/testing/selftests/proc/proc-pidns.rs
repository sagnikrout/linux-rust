//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/proc/proc-pidns.c
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
#[no_mangle]
unsafe extern "C" fn touch(path: *mut c_char) -> c_int {
    static int touch(char *path)
    {
    let mut fd: c_int = open(path, O_WRONLY|O_CREAT|O_CLOEXEC, 0644);
    if (fd < 0)
    return -1;
    return close(fd);
    }
    FIXTURE(ns)
    {
    int host_mntns, host_pidns;
    int dummy_pidns;
    };
    FIXTURE_SETUP(ns)
    {
// Stash the old mntns.
    self.host_mntns = open("/proc/self/ns/mnt", O_RDONLY|O_CLOEXEC);
    ASSERT_SUCCESS(self.host_mntns);
// Create a new mount namespace and make it private.
    ASSERT_SUCCESS(unshare(CLONE_NEWNS));
    ASSERT_SUCCESS(mount(core::ptr::null_mut(), "/", core::ptr::null_mut(), MS_PRIVATE|MS_REC, core::ptr::null_mut()));
//
// Create a proper tmpfs that we can use and will disappear once we
// leave this mntns.
//
    ASSERT_SUCCESS(mount("tmpfs", "/tmp", "tmpfs", 0, core::ptr::null_mut()));
//
// Create a pidns we can use for later tests. We need to fork off a
// child so that we get a usable nsfd that we can bind-mount and open.
//
    ASSERT_SUCCESS(mkdir("/tmp/dummy", 0755));
    ASSERT_SUCCESS(touch("/tmp/dummy/pidns"));
    ASSERT_SUCCESS(mkdir("/tmp/dummy/proc", 0755));
    self.host_pidns = open("/proc/self/ns/pid", O_RDONLY|O_CLOEXEC);
    ASSERT_SUCCESS(self.host_pidns);
    ASSERT_SUCCESS(unshare(CLONE_NEWPID));
    let mut pid: pid_t = fork();
    ASSERT_SUCCESS(pid);
    if (!pid) {
    prctl(PR_SET_PDEATHSIG, SIGKILL);
    ASSERT_SUCCESS(mount("/proc/self/ns/pid", "/tmp/dummy/pidns", core::ptr::null_mut(), MS_BIND, core::ptr::null_mut()));
    ASSERT_SUCCESS(mount("proc", "/tmp/dummy/proc", "proc", 0, core::ptr::null_mut()));
    exit(0);
    }
    int wstatus;
    ASSERT_EQ(waitpid(pid, &wstatus, 0), pid);
    ASSERT_TRUE(WIFEXITED(wstatus));
    ASSERT_EQ(WEXITSTATUS(wstatus), 0);
    ASSERT_SUCCESS(setns(self.host_pidns, CLONE_NEWPID));
    self.dummy_pidns = open("/tmp/dummy/pidns", O_RDONLY|O_CLOEXEC);
    ASSERT_SUCCESS(self.dummy_pidns);
    }
    FIXTURE_TEARDOWN(ns)
    {
    ASSERT_SUCCESS(setns(self.host_mntns, CLONE_NEWNS));
    ASSERT_SUCCESS(close(self.host_mntns));
    ASSERT_SUCCESS(close(self.host_pidns));
    ASSERT_SUCCESS(close(self.dummy_pidns));
    }
    TEST_F(ns, pidns_mount_string_path)
    {
    ASSERT_SUCCESS(mkdir("/tmp/proc-host", 0755));
    ASSERT_SUCCESS(mount("proc", "/tmp/proc-host", "proc", 0, "pidns=/proc/self/ns/pid"));
    ASSERT_SUCCESS(access("/tmp/proc-host/self/", X_OK));
    ASSERT_SUCCESS(mkdir("/tmp/proc-dummy", 0755));
    ASSERT_SUCCESS(mount("proc", "/tmp/proc-dummy", "proc", 0, "pidns=/tmp/dummy/pidns"));
    ASSERT_ERRNO_EQ(-ENOENT, access("/tmp/proc-dummy/1/", X_OK));
    ASSERT_ERRNO_EQ(-ENOENT, access("/tmp/proc-dummy/self/", X_OK));
    }
    TEST_F(ns, pidns_fsconfig_string_path)
    {
    let mut fsfd: c_int = fsopen("proc", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_SET_STRING, "pidns", "/tmp/dummy/pidns", 0));
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0));
    let mut mountfd: c_int = fsmount(fsfd, FSMOUNT_CLOEXEC, 0);
    ASSERT_SUCCESS(mountfd);
    ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, "1/", X_OK, 0));
    ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, "self/", X_OK, 0));
    ASSERT_SUCCESS(close(fsfd));
    ASSERT_SUCCESS(close(mountfd));
    }
    TEST_F(ns, pidns_fsconfig_fd)
    {
    let mut fsfd: c_int = fsopen("proc", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_SET_FD, "pidns", core::ptr::null_mut(), self.dummy_pidns));
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0));
    let mut mountfd: c_int = fsmount(fsfd, FSMOUNT_CLOEXEC, 0);
    ASSERT_SUCCESS(mountfd);
    ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, "1/", X_OK, 0));
    ASSERT_ERRNO_EQ(-ENOENT, faccessat(mountfd, "self/", X_OK, 0));
    ASSERT_SUCCESS(close(fsfd));
    ASSERT_SUCCESS(close(mountfd));
    }
    TEST_F(ns, pidns_reconfigure_remount)
    {
    ASSERT_SUCCESS(mkdir("/tmp/proc", 0755));
    ASSERT_SUCCESS(mount("proc", "/tmp/proc", "proc", 0, ""));
    ASSERT_SUCCESS(access("/tmp/proc/1/", X_OK));
    ASSERT_SUCCESS(access("/tmp/proc/self/", X_OK));
    ASSERT_ERRNO_EQ(-EBUSY, mount(core::ptr::null_mut(), "/tmp/proc", core::ptr::null_mut(), MS_REMOUNT, "pidns=/tmp/dummy/pidns"));
    ASSERT_SUCCESS(access("/tmp/proc/1/", X_OK));
    ASSERT_SUCCESS(access("/tmp/proc/self/", X_OK));
    }
    TEST_F(ns, pidns_reconfigure_fsconfig_string_path)
    {
    let mut fsfd: c_int = fsopen("proc", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0));
    let mut mountfd: c_int = fsmount(fsfd, FSMOUNT_CLOEXEC, 0);
    ASSERT_SUCCESS(mountfd);
    ASSERT_SUCCESS(faccessat(mountfd, "1/", X_OK, 0));
    ASSERT_SUCCESS(faccessat(mountfd, "self/", X_OK, 0));
    ASSERT_ERRNO_EQ(-EBUSY, fsconfig(fsfd, FSCONFIG_SET_STRING, "pidns", "/tmp/dummy/pidns", 0));
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_CMD_RECONFIGURE, core::ptr::null_mut(), core::ptr::null_mut(), 0)); /* noop */
    ASSERT_SUCCESS(faccessat(mountfd, "1/", X_OK, 0));
    ASSERT_SUCCESS(faccessat(mountfd, "self/", X_OK, 0));
    ASSERT_SUCCESS(close(fsfd));
    ASSERT_SUCCESS(close(mountfd));
    }
    TEST_F(ns, pidns_reconfigure_fsconfig_fd)
    {
    let mut fsfd: c_int = fsopen("proc", FSOPEN_CLOEXEC);
    ASSERT_SUCCESS(fsfd);
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_CMD_CREATE, core::ptr::null_mut(), core::ptr::null_mut(), 0));
    let mut mountfd: c_int = fsmount(fsfd, FSMOUNT_CLOEXEC, 0);
    ASSERT_SUCCESS(mountfd);
    ASSERT_SUCCESS(faccessat(mountfd, "1/", X_OK, 0));
    ASSERT_SUCCESS(faccessat(mountfd, "self/", X_OK, 0));
    ASSERT_ERRNO_EQ(-EBUSY, fsconfig(fsfd, FSCONFIG_SET_FD, "pidns", core::ptr::null_mut(), self.dummy_pidns));
    ASSERT_SUCCESS(fsconfig(fsfd, FSCONFIG_CMD_RECONFIGURE, core::ptr::null_mut(), core::ptr::null_mut(), 0)); /* noop */
    ASSERT_SUCCESS(faccessat(mountfd, "1/", X_OK, 0));
    ASSERT_SUCCESS(faccessat(mountfd, "self/", X_OK, 0));
    ASSERT_SUCCESS(close(fsfd));
    ASSERT_SUCCESS(close(mountfd));
    }
    TEST_HARNESS_MAIN
