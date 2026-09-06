//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_bpffs.c
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
// Copyright (c) 2020 Facebook
// Macro flag: #define _GNU_SOURCE

// TDIR must be in a location we can create a directory in.

#[no_mangle]
unsafe extern "C" fn read_iter(file: *mut c_char) -> c_int {
    static int read_iter(char *file)
    {
// 1024 should be enough to get contiguous 4 "iter" letters at some point
    char buf[1024];
    int fd, len;
    fd = open(file, 0);
    if (fd < 0)
    return -1;
    while ((len = read(fd, buf, sizeof(buf))) > 0) {
    buf[sizeof(buf) - 1] = '\0';
    if (strstr(buf, "iter")) {
    close(fd);
    return 0;
    }
    }
    close(fd);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn fn() -> c_int {
    static int fn(void)
    {
    struct stat a, b, c;
    int err, map;
    err = unshare(CLONE_NEWNS);
    if (!ASSERT_OK(err, "unshare"))
    goto out;
    err = mount("", "/", "", MS_REC | MS_PRIVATE, core::ptr::null_mut());
    if (!ASSERT_OK(err, "mount /"))
    goto out;
    err =  mkdir(TDIR, 0777);
// If the directory already exists we can carry on. It may be left over
// from a previous run.
//
    if ((err && errno != EEXIST) && !ASSERT_OK(err, "mkdir " TDIR))
    goto out;
    err = mount("none", TDIR, "tmpfs", 0, core::ptr::null_mut());
    if (!ASSERT_OK(err, "mount tmpfs"))
    goto out;
    err = mkdir(TDIR "/fs1", 0777);
    if (!ASSERT_OK(err, "mkdir " TDIR "/fs1"))
    goto out;
    err = mkdir(TDIR "/fs2", 0777);
    if (!ASSERT_OK(err, "mkdir " TDIR "/fs2"))
    goto out;
    err = mount("bpf", TDIR "/fs1", "bpf", 0, core::ptr::null_mut());
    if (!ASSERT_OK(err, "mount bpffs " TDIR "/fs1"))
    goto out;
    err = mount("bpf", TDIR "/fs2", "bpf", 0, core::ptr::null_mut());
    if (!ASSERT_OK(err, "mount bpffs " TDIR "/fs2"))
    goto out;
    err = read_iter(TDIR "/fs1/maps.debug");
    if (!ASSERT_OK(err, "reading " TDIR "/fs1/maps.debug"))
    goto out;
    err = read_iter(TDIR "/fs2/progs.debug");
    if (!ASSERT_OK(err, "reading " TDIR "/fs2/progs.debug"))
    goto out;
    err = mkdir(TDIR "/fs1/a", 0777);
    if (!ASSERT_OK(err, "creating " TDIR "/fs1/a"))
    goto out;
    err = mkdir(TDIR "/fs1/a/1", 0777);
    if (!ASSERT_OK(err, "creating " TDIR "/fs1/a/1"))
    goto out;
    err = mkdir(TDIR "/fs1/b", 0777);
    if (!ASSERT_OK(err, "creating " TDIR "/fs1/b"))
    goto out;
    map = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null_mut(), 4, 4, 1, core::ptr::null_mut());
    if (!ASSERT_GT(map, 0, "create_map(ARRAY)"))
    goto out;
    err = bpf_obj_pin(map, TDIR "/fs1/c");
    if (!ASSERT_OK(err, "pin map"))
    goto out;
    close(map);
// Check that RENAME_EXCHANGE works for directories.
    err = stat(TDIR "/fs1/a", &a);
    if (!ASSERT_OK(err, "stat(" TDIR "/fs1/a)"))
    goto out;
    err = renameat2(0, TDIR "/fs1/a", 0, TDIR "/fs1/b", RENAME_EXCHANGE);
    if (!ASSERT_OK(err, "renameat2(/fs1/a, /fs1/b, RENAME_EXCHANGE)"))
    goto out;
    err = stat(TDIR "/fs1/b", &b);
    if (!ASSERT_OK(err, "stat(" TDIR "/fs1/b)"))
    goto out;
    if (!ASSERT_EQ(a.st_ino, b.st_ino, "b should have a's inode"))
    goto out;
    err = access(TDIR "/fs1/b/1", F_OK);
    if (!ASSERT_OK(err, "access(" TDIR "/fs1/b/1)"))
    goto out;
// Check that RENAME_EXCHANGE works for mixed file types.
    err = stat(TDIR "/fs1/c", &c);
    if (!ASSERT_OK(err, "stat(" TDIR "/fs1/map)"))
    goto out;
    err = renameat2(0, TDIR "/fs1/c", 0, TDIR "/fs1/b", RENAME_EXCHANGE);
    if (!ASSERT_OK(err, "renameat2(/fs1/c, /fs1/b, RENAME_EXCHANGE)"))
    goto out;
    err = stat(TDIR "/fs1/b", &b);
    if (!ASSERT_OK(err, "stat(" TDIR "/fs1/b)"))
    goto out;
    if (!ASSERT_EQ(c.st_ino, b.st_ino, "b should have c's inode"))
    goto out;
    err = access(TDIR "/fs1/c/1", F_OK);
    if (!ASSERT_OK(err, "access(" TDIR "/fs1/c/1)"))
    goto out;
// Check that RENAME_NOREPLACE works.
    err = renameat2(0, TDIR "/fs1/b", 0, TDIR "/fs1/a", RENAME_NOREPLACE);
    if (!ASSERT_ERR(err, "renameat2(RENAME_NOREPLACE)")) {
    err = -EINVAL;
    goto out;
    }
    err = access(TDIR "/fs1/b", F_OK);
    if (!ASSERT_OK(err, "access(" TDIR "/fs1/b)"))
    goto out;
    out:
    umount(TDIR "/fs1");
    umount(TDIR "/fs2");
    rmdir(TDIR "/fs1");
    rmdir(TDIR "/fs2");
    umount(TDIR);
    rmdir(TDIR);
    exit(err);
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_bpffs() {
    void test_test_bpffs(void)
    {
    int err, duration = 0, status = 0;
    pid_t pid;
    pid = fork();
    if (CHECK(pid == -1, "clone", "clone failed %d", errno))
    return;
    if (pid == 0)
    fn();
    err = waitpid(pid, &status, 0);
    if (CHECK(err == -1 && errno != ECHILD, "waitpid", "failed %d", errno))
    return;
    if (CHECK(WEXITSTATUS(status), "bpffs test ", "failed %d", WEXITSTATUS(status)))
    return;
    }
