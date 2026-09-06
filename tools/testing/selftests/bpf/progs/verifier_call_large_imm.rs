//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_call_large_imm.c
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

    let mut call_happened: c_int = 0;
//
// 32765 is the exact minimum number of padding instructions needed to
// trigger the verifier failure, because:
// 1. Counting the wrapper instructions around the padding block (one
// "r0=0" and two "exit" instructions), the actual jump distance
// evaluates to N + 3.
// 2. To overflow the s16 max bound (32767), we need N + 3 > 32767.
// Thus, N = 32765 is the exact minimum padding size required.
//
#[no_mangle]
pub unsafe extern "C" fn __attribute__(padding_subprog(void: (noinline)) void) -> static {
    static __attribute__((noinline)) void padding_subprog(void)
    {
    asm volatile (
    "r0 = 0;"
    ".rept 32765;"
    "r0 += 0;"
    ".endr;"
    ::: __clobber_all);
    }
#[no_mangle]
pub unsafe extern "C" fn __attribute__(target_subprog(void: (noinline)) int) -> static {
    static __attribute__((noinline)) int target_subprog(void)
    {
// Use volatile variable here to prevent optimization.
    let mut magic_ret: volatile int = 3;
    return magic_ret;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 3) -> __success {
    __success __retval(3)
#[no_mangle]
pub unsafe extern "C" fn call_large_imm_test(ctx: *mut c_void) -> c_int {
    int call_large_imm_test(void *ctx)
    {
//
// Landing pad to handle call error on kernel without the fix,
// preventing kernel panic.
//
    asm volatile (
    "r0 = 0;"
    ".rept 32768;"
    "r0 += 0;"
    ".endr;"
    ::: __clobber_all);
//
// The call_happened variable is 1 only when the call insn wrongly
// go back to the landing pad above.
//
    if (call_happened == 1) {
// Use volatile variable here to prevent optimization.
    let mut flag: volatile int = -1;
    return flag;
    }
    call_happened = 1;
    padding_subprog();
    return target_subprog();
    }
    char LICENSE[] SEC("license") = "GPL";
