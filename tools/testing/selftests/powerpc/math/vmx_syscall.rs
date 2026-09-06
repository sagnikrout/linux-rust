//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/math/vmx_syscall.c
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
// Copyright 2015, Cyril Bur, IBM Corp.
//
// This test attempts to see if the VMX registers change across a syscall (fork).
//

    vector int varray[] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10,11,12},
    {13,14,15,16},{17,18,19,20},{21,22,23,24},
    {25,26,27,28},{29,30,31,32},{33,34,35,36},
    {37,38,39,40},{41,42,43,44},{45,46,47,48}};
    extern int test_vmx(vector int *varray, pid_t *pid);
#[no_mangle]
pub unsafe extern "C" fn vmx_syscall() -> c_int {
    int vmx_syscall(void)
    {
    pid_t fork_pid;
    int i;
    int ret;
    int child_ret;
    for (i = 0; i < 1000; i++) {
// test_vmx will fork()
    ret = test_vmx(varray, &fork_pid);
    if (fork_pid == -1)
    return -1;
    if (fork_pid == 0)
    exit(ret);
    waitpid(fork_pid, &child_ret, 0);
    if (ret || child_ret)
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_vmx_syscall() -> c_int {
    int test_vmx_syscall(void)
    {
//
// Setup an environment with much context switching
//
    pid_t pid2;
    pid_t pid;
    int ret;
    int child_ret;
// vcmpequd used in vmx_asm.S is v2.07
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
    pid = fork();
    FAIL_IF(pid == -1);
    pid2 = fork();
    ret = vmx_syscall();
// Can't FAIL_IF(pid2 == -1); because we've already forked
    if (pid2 == -1) {
//
// Couldn't fork, ensure child_ret is set and is a fail
//
    ret = child_ret = 1;
    } else {
    if (pid2)
    waitpid(pid2, &child_ret, 0);
    else
    exit(ret);
    }
    ret |= child_ret;
    if (pid)
    waitpid(pid, &child_ret, 0);
    else
    exit(ret);
    FAIL_IF(ret || child_ret);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(test_vmx_syscall, "vmx_syscall");
    }
