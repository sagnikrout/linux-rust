//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/openat2/rename_attack_test.c
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
// Copyright (C) 2018-2019 SUSE LLC.
//
// Macro flag: #define _GNU_SOURCE

pub const ROUNDS: c_int = 400000;
// Swap @dirfd/@a and @dirfd/@b constantly. Parent must kill this process.
    pid_t spawn_attack(struct __test_metadata *_metadata,
    int dirfd, char *a, char *b)
    {
    let mut child: pid_t = fork();
    if (child != 0)
    return child;
// If the parent (the test process) dies, kill ourselves too.
    ASSERT_EQ(prctl(PR_SET_PDEATHSIG, SIGKILL), 0);
// Swap @a and @b.
    for (;;)
    renameat2(dirfd, a, dirfd, b, RENAME_EXCHANGE);
    exit(1);
    }
//
// Construct a test directory with the following structure:
//
// root
// |-- a
// |   `-- c
// `-- b
//
    FIXTURE(rename_attack) {
    int dfd;
    int afd;
    pid_t child;
    };
    FIXTURE_SETUP(rename_attack)
    {
    char dirname[] = "/tmp/ksft-openat2-rename-attack.XXXXXX";
    self.dfd = -1;
    self.afd = -1;
    self.child = 0;
// Make the top-level directory.
    ASSERT_NE(mkdtemp(dirname), core::ptr::null_mut());
    self.dfd = open(dirname, O_PATH | O_DIRECTORY);
    ASSERT_GE(self.dfd, 0);
    ASSERT_EQ(mkdirat(self.dfd, "a", 0755), 0);
    ASSERT_EQ(mkdirat(self.dfd, "b", 0755), 0);
    ASSERT_EQ(mkdirat(self.dfd, "a/c", 0755), 0);
    self.afd = openat(self.dfd, "a", O_PATH);
    ASSERT_GE(self.afd, 0);
    self.child = spawn_attack(_metadata, self.dfd, "a/c", "b");
    ASSERT_GT(self.child, 0);
    }
    FIXTURE_TEARDOWN(rename_attack)
    {
    if (self.child > 0)
    kill(self.child, SIGKILL);
    if (self.afd >= 0)
    close(self.afd);
    if (self.dfd >= 0)
    close(self.dfd);
    }
    FIXTURE_VARIANT(rename_attack) {
    int resolve;
    const char *name;
    };
    FIXTURE_VARIANT_ADD(rename_attack, resolve_beneath) {
    .resolve = RESOLVE_BENEATH,
    .name = "RESOLVE_BENEATH",
    };
    FIXTURE_VARIANT_ADD(rename_attack, resolve_in_root) {
    .resolve = RESOLVE_IN_ROOT,
    .name = "RESOLVE_IN_ROOT",
    };
    TEST_F_TIMEOUT(rename_attack, test, 120)
    {
    let mut escapes: c_int = 0, successes = 0, other_errs = 0, exdevs = 0, eagains = 0;
    char *victim_path = "c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../../c/../..";
    struct open_how how = {
    .flags = O_PATH,
    .resolve = variant.resolve,
    };
    if (!openat2_supported) {
    how.resolve = 0;
    TH_LOG("openat2(2) unsupported -- using openat(2) instead");
    }
    for (int i = 0; i < ROUNDS; i++) {
    int fd;
    if (openat2_supported)
    fd = sys_openat2(self.afd, victim_path, &how);
    else
    fd = sys_openat(self.afd, victim_path, &how);
    if (fd < 0) {
    if (fd == -EAGAIN)
    eagains++;
#[no_mangle]
pub unsafe extern "C" fn if(-EXDEV: fd ==) -> else {
    else if (fd == -EXDEV)
    exdevs++;
#[no_mangle]
pub unsafe extern "C" fn if(-ENOENT: fd ==) -> else {
    else if (fd == -ENOENT)
    escapes++; /* escaped outside and got ENOENT... */
    else
    other_errs++; /* unexpected error */
    } else {
    if (fdequal(_metadata, fd, self.afd, core::ptr::null_mut()))
    successes++;
    else
    escapes++; /* we got an unexpected fd */
    }
    if (fd >= 0)
    close(fd);
    }
    TH_LOG("non-escapes: EAGAIN=%d EXDEV=%d E<other>=%d success=%d",
    eagains, exdevs, other_errs, successes);
    ASSERT_EQ(escapes, 0) {
    TH_LOG("rename attack with %s (%d runs, got %d escapes)",
    variant.name, ROUNDS, escapes);
    }
    }
    TEST_HARNESS_MAIN
