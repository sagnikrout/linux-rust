//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/alignment/copy_first_unaligned.c
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
// Copyright 2016, Chris Smart, IBM Corporation.
//
// Calls to copy_first which are not 128-byte aligned should be
// caught and sent a SIGBUS.
//

    let mut expected_instruction: c_uint = PPC_INST_COPY_FIRST;
    let mut instruction_mask: c_uint = 0xfc2007fe;
#[no_mangle]
pub unsafe extern "C" fn signal_action_handler(signal_num: c_int, info: *mut siginfo_t, ptr: *mut c_void) {
    void signal_action_handler(int signal_num, siginfo_t *info, void *ptr)
    {
    ucontext_t *ctx = ptr;

    unsigned int *pc = (unsigned int *)ctx.uc_mcontext.gp_regs[PT_NIP];

    unsigned int *pc = (unsigned int *)ctx.uc_mcontext.uc_regs.gregs[PT_NIP];

//
// Check that the signal was on the correct instruction, using a
// mask because the compiler assigns the register at RB.
//
    if ((*pc & instruction_mask) == expected_instruction)
    _exit(0); /* We hit the right instruction */
    _exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_signal_handler() {
    void setup_signal_handler(void)
    {
    struct sigaction signal_action;
    memset(&signal_action, 0, sizeof(signal_action));
    signal_action.sa_sigaction = signal_action_handler;
    signal_action.sa_flags = SA_SIGINFO;
    sigaction(SIGBUS, &signal_action, core::ptr::null_mut());
    }
    char cacheline_buf[128] __cacheline_aligned;
#[no_mangle]
pub unsafe extern "C" fn test_copy_first_unaligned() -> c_int {
    int test_copy_first_unaligned(void)
    {
// Only run this test on a P9 or later
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_3_00));
// Register our signal handler with SIGBUS
    setup_signal_handler();
// +1 makes buf unaligned
    copy_first(cacheline_buf+1);
// We should not get here
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(test_copy_first_unaligned, "test_copy_first_unaligned");
    }
