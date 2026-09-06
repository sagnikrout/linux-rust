//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/fuse/fusectl_test.c
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
// Copyright (c) 2025 Chen Linxuan <chenlinxuan@uniontech.com>
// Macro flag: #define _GNU_SOURCE

    static void write_file(struct __test_metadata *const _metadata,
    const char *path, const char *val)
    {
    let mut fd: c_int = open(path, O_WRONLY);
    let mut len: usize = strlen(val);
    ASSERT_GE(fd, 0);
    ASSERT_EQ(write(fd, val, len), len);
    ASSERT_EQ(close(fd), 0);
    }
    FIXTURE(fusectl){
    char fuse_mountpoint[sizeof(FUSE_MOUNTPOINT)];
    int connection;
    };
    FIXTURE_SETUP(fusectl)
    {
    const char *fuse_mnt_prog = "./fuse_mnt";
    int status, pid;
    struct stat statbuf;
    let mut uid: uid_t = getuid();
    let mut gid: gid_t = getgid();
    char buf[32];
// Setup userns
    ASSERT_EQ(unshare(CLONE_NEWNS|CLONE_NEWUSER), 0);
    sprintf(buf, "0 %d 1", uid);
    write_file(_metadata, "/proc/self/uid_map", buf);
    write_file(_metadata, "/proc/self/setgroups", "deny");
    sprintf(buf, "0 %d 1", gid);
    write_file(_metadata, "/proc/self/gid_map", buf);
    ASSERT_EQ(mount("", "/", core::ptr::null_mut(), MS_REC|MS_PRIVATE, core::ptr::null_mut()), 0);
    strcpy(self.fuse_mountpoint, FUSE_MOUNTPOINT);
    if (!mkdtemp(self.fuse_mountpoint))
    SKIP(return,
    "Failed to create FUSE mountpoint %s",
    strerror(errno));
    if (access(FUSECTL_MOUNTPOINT, F_OK))
    SKIP(return,
    "FUSE control filesystem not mounted");
    pid = fork();
    if (pid < 0)
    SKIP(return,
    "Failed to fork FUSE daemon process: %s",
    strerror(errno));
    if (pid == 0) {
    execlp(fuse_mnt_prog, fuse_mnt_prog, self.fuse_mountpoint, core::ptr::null_mut());
    exit(errno);
    }
    waitpid(pid, &status, 0);
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
    SKIP(return,
    "Failed to start FUSE daemon %s",
    strerror(WEXITSTATUS(status)));
    }
    if (stat(self.fuse_mountpoint, &statbuf))
    SKIP(return,
    "Failed to stat FUSE mountpoint %s",
    strerror(errno));
    self.connection = statbuf.st_dev;
    }
    FIXTURE_TEARDOWN(fusectl)
    {
    umount2(self.fuse_mountpoint, MNT_DETACH);
    rmdir(self.fuse_mountpoint);
    }
    TEST_F(fusectl, abort)
    {
    char path_buf[PATH_MAX];
    int abort_fd, test_fd, ret;
    sprintf(path_buf, "/sys/fs/fuse/connections/%d/abort", self.connection);
    ASSERT_EQ(0, access(path_buf, F_OK));
    abort_fd = open(path_buf, O_WRONLY);
    ASSERT_GE(abort_fd, 0);
    sprintf(path_buf, "%s/test", self.fuse_mountpoint);
    test_fd = open(path_buf, O_RDWR);
    ASSERT_GE(test_fd, 0);
    ret = read(test_fd, path_buf, sizeof(path_buf));
    ASSERT_EQ(ret, 0);
    ret = write(test_fd, "test", sizeof("test"));
    ASSERT_EQ(ret, sizeof("test"));
    ret = lseek(test_fd, 0, SEEK_SET);
    ASSERT_GE(ret, 0);
    ret = write(abort_fd, FUSECTL_TEST_VALUE, sizeof(FUSECTL_TEST_VALUE));
    ASSERT_GT(ret, 0);
    close(abort_fd);
    ret = read(test_fd, path_buf, sizeof(path_buf));
    ASSERT_EQ(ret, -1);
    ASSERT_EQ(errno, ENOTCONN);
    }
    TEST_HARNESS_MAIN
