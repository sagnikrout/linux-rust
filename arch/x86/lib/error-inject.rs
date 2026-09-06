//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/error-inject.c
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

    asmlinkage void just_return_func(void);
    asm(
    ".text\n"
    ".type just_return_func, @function\n"
    ".globl just_return_func\n"
    ASM_FUNC_ALIGN
    "just_return_func:\n"
    ANNOTATE_NOENDBR "\n"
    ASM_RET
    ".size just_return_func, .-just_return_func\n"
    );
#[no_mangle]
pub unsafe extern "C" fn override_function_with_return(regs: *mut pt_regs) {
    void override_function_with_return(struct pt_regs *regs)
    {
    regs.ip = (unsigned long)&just_return_func;
    }
    NOKPROBE_SYMBOL(override_function_with_return);
