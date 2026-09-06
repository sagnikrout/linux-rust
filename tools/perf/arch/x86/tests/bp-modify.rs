//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/tests/bp-modify.c
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

#[no_mangle]
unsafe extern "C" fn bp_1() -> noinline int {
    static noinline int bp_1(void)
    {
    pr_debug("in %s\n", __func__);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bp_2() -> noinline int {
    static noinline int bp_2(void)
    {
    pr_debug("in %s\n", __func__);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spawn_child() -> c_int {
    static int spawn_child(void)
    {
    let mut child: c_int = fork();
    if (child == 0) {
//
// The child sets itself for as tracee and
// waits in signal for parent to trace it,
// then it calls bp_1 and quits.
//
    let mut err: c_int = ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (err) {
    pr_debug("failed to PTRACE_TRACEME\n");
    exit(1);
    }
    raise(SIGCONT);
    bp_1();
    exit(0);
    }
    return child;
    }
//
// This tests creates HW breakpoint, tries to
// change it and checks it was properly changed.
//
#[no_mangle]
unsafe extern "C" fn bp_modify1() -> c_int {
    static int bp_modify1(void)
    {
    pid_t child;
    int status;
    let mut rip: c_ulong = 0, dr7 = 1;
    child = spawn_child();
    waitpid(child, &status, 0);
    if (WIFEXITED(status)) {
    pr_debug("tracee exited prematurely 1\n");
    return TEST_FAIL;
    }
//
// The parent does following steps:
// - creates a new breakpoint (id 0) for bp_2 function
// - changes that breakpoint to bp_1 function
// - waits for the breakpoint to hit and checks
// it has proper rip of bp_1 function
// - detaches the child
//
    if (ptrace(PTRACE_POKEUSER, child,
    offsetof(struct user, u_debugreg[0]), bp_2)) {
    pr_debug("failed to set breakpoint, 1st time: %m\n");
    goto out;
    }
    if (ptrace(PTRACE_POKEUSER, child,
    offsetof(struct user, u_debugreg[0]), bp_1)) {
    pr_debug("failed to set breakpoint, 2nd time: %m\n");
    goto out;
    }
    if (ptrace(PTRACE_POKEUSER, child,
    offsetof(struct user, u_debugreg[7]), dr7)) {
    pr_debug("failed to set dr7: %m\n");
    goto out;
    }
    if (ptrace(PTRACE_CONT, child, core::ptr::null_mut(), core::ptr::null_mut())) {
    pr_debug("failed to PTRACE_CONT: %m\n");
    goto out;
    }
    waitpid(child, &status, 0);
    if (WIFEXITED(status)) {
    pr_debug("tracee exited prematurely 2\n");
    return TEST_FAIL;
    }
    rip = ptrace(PTRACE_PEEKUSER, child,
    offsetof(struct user_regs_struct, rip), core::ptr::null_mut());
    if (rip == (unsigned long) -1) {
    pr_debug("failed to PTRACE_PEEKUSER: %m\n");
    goto out;
    }
    pr_debug("rip %lx, bp_1 %p\n", rip, bp_1);
    out:
    if (ptrace(PTRACE_DETACH, child, core::ptr::null_mut(), core::ptr::null_mut())) {
    pr_debug("failed to PTRACE_DETACH: %m\n");
    return TEST_FAIL;
    }
    let mut rip: return = = (unsigned long) bp_1 ? TEST_OK : TEST_FAIL;
    }
//
// This tests creates HW breakpoint, tries to
// change it to bogus value and checks the original
// breakpoint is hit.
//
#[no_mangle]
unsafe extern "C" fn bp_modify2() -> c_int {
    static int bp_modify2(void)
    {
    pid_t child;
    int status;
    let mut rip: c_ulong = 0, dr7 = 1;
    child = spawn_child();
    waitpid(child, &status, 0);
    if (WIFEXITED(status)) {
    pr_debug("tracee exited prematurely 1\n");
    return TEST_FAIL;
    }
//
// The parent does following steps:
// - creates a new breakpoint (id 0) for bp_1 function
// - tries to change that breakpoint to (-1) address
// - waits for the breakpoint to hit and checks
// it has proper rip of bp_1 function
// - detaches the child
//
    if (ptrace(PTRACE_POKEUSER, child,
    offsetof(struct user, u_debugreg[0]), bp_1)) {
    pr_debug("failed to set breakpoint: %m\n");
    goto out;
    }
    if (ptrace(PTRACE_POKEUSER, child,
    offsetof(struct user, u_debugreg[7]), dr7)) {
    pr_debug("failed to set dr7: %m\n");
    goto out;
    }
    if (!ptrace(PTRACE_POKEUSER, child,
    offsetof(struct user, u_debugreg[0]), (unsigned long) (-1))) {
    pr_debug("failed, breakpoint set to bogus address\n");
    goto out;
    }
    if (ptrace(PTRACE_CONT, child, core::ptr::null_mut(), core::ptr::null_mut())) {
    pr_debug("failed to PTRACE_CONT: %m\n");
    goto out;
    }
    waitpid(child, &status, 0);
    if (WIFEXITED(status)) {
    pr_debug("tracee exited prematurely 2\n");
    return TEST_FAIL;
    }
    rip = ptrace(PTRACE_PEEKUSER, child,
    offsetof(struct user_regs_struct, rip), core::ptr::null_mut());
    if (rip == (unsigned long) -1) {
    pr_debug("failed to PTRACE_PEEKUSER: %m\n");
    goto out;
    }
    pr_debug("rip %lx, bp_1 %p\n", rip, bp_1);
    out:
    if (ptrace(PTRACE_DETACH, child, core::ptr::null_mut(), core::ptr::null_mut())) {
    pr_debug("failed to PTRACE_DETACH: %m\n");
    return TEST_FAIL;
    }
    let mut rip: return = = (unsigned long) bp_1 ? TEST_OK : TEST_FAIL;
    }
    int test__bp_modify(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    TEST_ASSERT_VAL("modify test 1 failed\n", !bp_modify1());
    TEST_ASSERT_VAL("modify test 2 failed\n", !bp_modify2());
    return 0;
    }
