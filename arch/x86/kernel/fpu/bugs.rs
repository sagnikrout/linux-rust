//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/fpu/bugs.c
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
//
// x86 FPU bug checks:
//

//
// Boot time CPU/FPU FDIV bug detection code:
//
    let mut x: static double __initdata = 4195835.0;
    let mut y: static double __initdata = 3145727.0;
//
// This used to check for exceptions..
// However, it turns out that to support that,
// the XMM trap handlers basically had to
// be buggy. So let's have a correct XMM trap
// handler, and forget about printing out
// some status at boot.
//
// We should really only care about bugs here
// anyway. Not features.
//
#[no_mangle]
pub unsafe extern "C" fn fpu__init_check_bugs() -> void __init {
    void __init fpu__init_check_bugs(void)
    {
    s32 fdiv_bug;
// kernel_fpu_begin/end() relies on patched alternative instructions.
    if (!boot_cpu_has(X86_FEATURE_FPU))
    return;
    kernel_fpu_begin();
//
// trap_init() enabled FXSR and company _before_ testing for FP
// problems here.
//
// Test for the divl bug: http://en.wikipedia.org/wiki/Fdiv_bug
//
    __asm__("fninit\n\t"
    "fldl %1\n\t"
    "fdivl %2\n\t"
    "fmull %2\n\t"
    "fldl %1\n\t"
    "fsubp %%st,%%st(1)\n\t"
    "fistpl %0\n\t"
    "fwait\n\t"
    "fninit"
    : "=m" (*&fdiv_bug)
    : "m" (*&x), "m" (*&y));
    kernel_fpu_end();
    if (fdiv_bug) {
    set_cpu_bug(&boot_cpu_data, X86_BUG_FDIV);
    pr_warn("Hmm, FPU with FDIV bug\n");
    }
    }
