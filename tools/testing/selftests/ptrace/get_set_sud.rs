//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ptrace/get_set_sud.c
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
unsafe extern "C" fn sys_ptrace(request: c_int, pid: pid_t, addr: *mut c_void, data: *mut c_void) -> c_int {
    static int sys_ptrace(int request, pid_t pid, void *addr, void *data)
    {
    return syscall(SYS_ptrace, request, pid, addr, data);
    }
    TEST(get_set_sud)
    {
    struct ptrace_sud_config config;
    pid_t child;
    let mut ret: c_int = 0;
    int status;
    child = fork();
    ASSERT_GE(child, 0);
    if (child == 0) {
    ASSERT_EQ(0, sys_ptrace(PTRACE_TRACEME, 0, 0, 0)) {
    TH_LOG("PTRACE_TRACEME: %m");
    }
    kill(getpid(), SIGSTOP);
    _exit(1);
    }
    waitpid(child, &status, 0);
    memset(&config, 0xff, sizeof(config));
    config.mode = PR_SYS_DISPATCH_ON;
    ret = sys_ptrace(PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG, child,
    (void *)sizeof(config), &config);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(config.mode, PR_SYS_DISPATCH_OFF);
    ASSERT_EQ(config.selector, 0);
    ASSERT_EQ(config.offset, 0);
    ASSERT_EQ(config.len, 0);
    config.mode = PR_SYS_DISPATCH_ON;
    config.selector = 0;
    config.offset = 0x400000;
    config.len = 0x1000;
    ret = sys_ptrace(PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG, child,
    (void *)sizeof(config), &config);
    ASSERT_EQ(ret, 0);
    memset(&config, 1, sizeof(config));
    ret = sys_ptrace(PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG, child,
    (void *)sizeof(config), &config);
    ASSERT_EQ(ret, 0);
    ASSERT_EQ(config.mode, PR_SYS_DISPATCH_ON);
    ASSERT_EQ(config.selector, 0);
    ASSERT_EQ(config.offset, 0x400000);
    ASSERT_EQ(config.len, 0x1000);
    kill(child, SIGKILL);
    }
    TEST_HARNESS_MAIN
