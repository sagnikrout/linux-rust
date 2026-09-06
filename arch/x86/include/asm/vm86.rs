//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/vm86.h
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
// This is the (kernel) stack-layout when we have done a "SAVE_ALL" from vm86
// mode - the main change is that the old segment descriptors aren't
// useful any more and are forced to be zero by the kernel (and the
// hardware when a trap occurs), and the real segment descriptors are
// at the end of the structure. Look at ptrace.h to see the "normal"
// setup. For user space layout see 'struct vm86_regs' above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_vm86_regs {
//
// normal regs, with special meaning for the segment descriptors..
//
    pub pt: pt_regs,
//
// these are specific to v86 mode:
//
    pub __esh: unsigned short es,,
    pub __dsh: unsigned short ds,,
    pub __fsh: unsigned short fs,,
    pub __gsh: unsigned short gs,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm86 {
    pub user_vm86: *mut vm86plus___user,
    pub regs32: pt_regs,
    pub veflags: c_ulong,
    pub veflags_mask: c_ulong,
    pub saved_sp0: c_ulong,
    pub flags: c_ulong,
    pub cpu_type: c_ulong,
    pub int_revectored: revectored_struct,
    pub int21_revectored: revectored_struct,
    pub vm86plus: vm86plus_info_struct,
}

extern "C" {
    pub fn handle_vm86_fault(: *mut kernel_vm86_regs, _arg: c_long);
}
extern "C" {
    pub fn handle_vm86_trap(: *mut kernel_vm86_regs, _arg: c_long, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn save_v86_state(: *mut kernel_vm86_regs, _arg: c_int);
}

//
// Support for VM86 programs to request interrupts for
// real mode hardware drivers:
//
pub const FIRST_VM86_IRQ: c_int = 3;
pub const LAST_VM86_IRQ: c_int = 15;
extern "C" {
    pub fn release_vm86_irqs(: *mut task_struct);
}

// Macro flag: #define release_vm86_irqs(a)

