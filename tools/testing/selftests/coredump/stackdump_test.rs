//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/coredump/stackdump_test.c
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

pub const PAGE_SIZE: c_int = 4096;

    FIXTURE_SETUP(coredump)
    {
    FILE *file;
    int ret;
    self.pid_coredump_server = -ESRCH;
    self.fd_tmpfs_detached = -1;
    file = fopen("/proc/sys/kernel/core_pattern", "r");
    ASSERT_NE(core::ptr::null_mut(), file);
    ret = fread(self.original_core_pattern, 1, sizeof(self.original_core_pattern), file);
    ASSERT_TRUE(ret || feof(file));
    ASSERT_LT(ret, sizeof(self.original_core_pattern));
    self.original_core_pattern[ret] = '\0';
    self.fd_tmpfs_detached = create_detached_tmpfs();
    ASSERT_GE(self.fd_tmpfs_detached, 0);
    ret = fclose(file);
    ASSERT_EQ(0, ret);
    }
    FIXTURE_TEARDOWN(coredump)
    {
    const char *reason;
    FILE *file;
    int ret, status;
    unlink(STACKDUMP_FILE);
    if (self.pid_coredump_server > 0) {
    kill(self.pid_coredump_server, SIGTERM);
    waitpid(self.pid_coredump_server, &status, 0);
    }
    unlink("/tmp/coredump.file");
    unlink("/tmp/coredump.socket");
    file = fopen("/proc/sys/kernel/core_pattern", "w");
    if (!file) {
    reason = "Unable to open core_pattern";
    goto fail;
    }
    ret = fprintf(file, "%s", self.original_core_pattern);
    if (ret < 0) {
    reason = "Unable to write to core_pattern";
    goto fail;
    }
    ret = fclose(file);
    if (ret) {
    reason = "Unable to close core_pattern";
    goto fail;
    }
    if (self.fd_tmpfs_detached >= 0) {
    ret = close(self.fd_tmpfs_detached);
    if (ret < 0) {
    reason = "Unable to close detached tmpfs";
    goto fail;
    }
    self.fd_tmpfs_detached = -1;
    }
    return;
    fail:
// This should never happen
    fprintf(stderr, "Failed to cleanup stackdump test: %s\n", reason);
    }
    TEST_F_TIMEOUT(coredump, stackdump, 120)
    {
    unsigned long long stack;
    char *test_dir, *line;
    size_t line_length;
    char buf[PAGE_SIZE];
    int ret, i, status;
    FILE *file;
    pid_t pid;
//
// Step 1: Setup core_pattern so that the stackdump script is executed when the child
// process crashes
//
    ret = readlink("/proc/self/exe", buf, sizeof(buf));
    ASSERT_NE(-1, ret);
    ASSERT_LT(ret, sizeof(buf));
    buf[ret] = '\0';
    test_dir = dirname(buf);
    file = fopen("/proc/sys/kernel/core_pattern", "w");
    ASSERT_NE(core::ptr::null_mut(), file);
    ret = fprintf(file, "|%1$s/%2$s %%P %1$s/%3$s", test_dir, STACKDUMP_SCRIPT, STACKDUMP_FILE);
    ASSERT_LT(0, ret);
    ret = fclose(file);
    ASSERT_EQ(0, ret);
// Step 2: Create a process who spawns some threads then crashes
    pid = fork();
    ASSERT_TRUE(pid >= 0);
    if (pid == 0)
    crashing_child();
//
// Step 3: Wait for the stackdump script to write the stack pointers to the stackdump file
//
    waitpid(pid, &status, 0);
    ASSERT_TRUE(WIFSIGNALED(status));
    ASSERT_TRUE(WCOREDUMP(status));
    for (i = 0; i < 10; ++i) {
    file = fopen(STACKDUMP_FILE, "r");
    if (file)
    break;
    sleep(1);
    }
    ASSERT_NE(file, core::ptr::null_mut());
// Step 4: Make sure all stack pointer values are non-zero
    line = core::ptr::null_mut();
    for (i = 0; -1 != getline(&line, &line_length, file); ++i) {
    stack = strtoull(line, core::ptr::null_mut(), 10);
    ASSERT_NE(stack, 0);
    }
    free(line);
    ASSERT_EQ(i, 1 + NUM_THREAD_SPAWN);
    fclose(file);
    }
    TEST_HARNESS_MAIN
