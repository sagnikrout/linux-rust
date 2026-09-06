//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/interrupt.h
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
// BookE/4xx
pub const INTERRUPT_CRITICAL_INPUT: c_uint = 0x100;
// BookE
pub const INTERRUPT_DEBUG: c_uint = 0xd00;

pub const INTERRUPT_PERFMON: c_uint = 0x260;
pub const INTERRUPT_DOORBELL: c_uint = 0x280;

// BookS/4xx/8xx
pub const INTERRUPT_MACHINE_CHECK: c_uint = 0x200;
// BookS/8xx
pub const INTERRUPT_SYSTEM_RESET: c_uint = 0x100;
// BookS
pub const INTERRUPT_DATA_SEGMENT: c_uint = 0x380;
pub const INTERRUPT_INST_SEGMENT: c_uint = 0x480;
pub const INTERRUPT_TRACE: c_uint = 0xd00;
pub const INTERRUPT_H_DATA_STORAGE: c_uint = 0xe00;
pub const INTERRUPT_HMI: c_uint = 0xe60;
pub const INTERRUPT_H_FAC_UNAVAIL: c_uint = 0xf80;

pub const INTERRUPT_DOORBELL: c_uint = 0xa00;
pub const INTERRUPT_PERFMON: c_uint = 0xf00;
pub const INTERRUPT_ALTIVEC_UNAVAIL: c_uint = 0xf20;

// BookE/BookS/4xx/8xx
pub const INTERRUPT_DATA_STORAGE: c_uint = 0x300;
pub const INTERRUPT_INST_STORAGE: c_uint = 0x400;
pub const INTERRUPT_EXTERNAL: c_uint = 0x500;
pub const INTERRUPT_ALIGNMENT: c_uint = 0x600;
pub const INTERRUPT_PROGRAM: c_uint = 0x700;
pub const INTERRUPT_SYSCALL: c_uint = 0xc00;
pub const INTERRUPT_TRACE: c_uint = 0xd00;
// BookE/BookS/44x
pub const INTERRUPT_FP_UNAVAIL: c_uint = 0x800;
// BookE/BookS/44x/8xx
pub const INTERRUPT_DECREMENTER: c_uint = 0x900;

pub const INTERRUPT_PERFMON: c_uint = 0x0;

// 8xx
pub const INTERRUPT_SOFT_EMU_8xx: c_uint = 0x1000;
pub const INTERRUPT_INST_TLB_MISS_8xx: c_uint = 0x1100;
pub const INTERRUPT_DATA_TLB_MISS_8xx: c_uint = 0x1200;
pub const INTERRUPT_INST_TLB_ERROR_8xx: c_uint = 0x1300;
pub const INTERRUPT_DATA_TLB_ERROR_8xx: c_uint = 0x1400;
pub const INTERRUPT_DATA_BREAKPOINT_8xx: c_uint = 0x1c00;
pub const INTERRUPT_INST_BREAKPOINT_8xx: c_uint = 0x1d00;
// 603
pub const INTERRUPT_INST_TLB_MISS_603: c_uint = 0x1000;
pub const INTERRUPT_DATA_LOAD_TLB_MISS_603: c_uint = 0x1100;
pub const INTERRUPT_DATA_STORE_TLB_MISS_603: c_uint = 0x1200;

//
// WARN/BUG is handled with a program interrupt so minimise checks here to
// avoid recursion and maximise the chance of getting the first oops handled.
//

//
// Don't use noinstr here like x86, but rather add NOKPROBE_SYMBOL to each
// function definition. The reason for this is the noinstr section is placed
// after the main text section, i.e., very far away from the interrupt entry
// asm. That creates problems with fitting linker stubs when building large
// kernels.
//

//
// DECLARE_INTERRUPT_HANDLER_RAW - Declare raw interrupt handler function
// @func:	Function name of the entry point
// @returns:	Returns a value back to asm caller
//

//
// DEFINE_INTERRUPT_HANDLER_RAW - Define raw interrupt handler function
// @func:	Function name of the entry point
// @returns:	Returns a value back to asm caller
//
// @func is called from ASM entry code.
//
// This is a plain function which does no tracing, reconciling, etc.
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//
// raw interrupt handlers must not enable or disable interrupts, or
// schedule, tracing and instrumentation (ftrace, lockdep, etc) would
// not be advisable either, although may be possible in a pinch, the
// trace will look odd at least.
//
// A raw handler may call one of the other interrupt handler functions
// to be converted into that interrupt context without these restrictions.
//
// On PPC64, _RAW handlers may return with fast_interrupt_return.
//
// Specific handlers may have additional restrictions.
//

//
// DECLARE_INTERRUPT_HANDLER - Declare synchronous interrupt handler function
// @func:	Function name of the entry point
//

//
// DEFINE_INTERRUPT_HANDLER - Define synchronous interrupt handler function
// @func:	Function name of the entry point
//
// @func is called from ASM entry code.
//
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//

//
// DECLARE_INTERRUPT_HANDLER_RET - Declare synchronous interrupt handler function
// @func:	Function name of the entry point
// @returns:	Returns a value back to asm caller
//

//
// DEFINE_INTERRUPT_HANDLER_RET - Define synchronous interrupt handler function
// @func:	Function name of the entry point
// @returns:	Returns a value back to asm caller
//
// @func is called from ASM entry code.
//
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//

//
// DECLARE_INTERRUPT_HANDLER_ASYNC - Declare asynchronous interrupt handler function
// @func:	Function name of the entry point
//

//
// DEFINE_INTERRUPT_HANDLER_ASYNC - Define asynchronous interrupt handler function
// @func:	Function name of the entry point
//
// @func is called from ASM entry code.
//
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//

//
// DECLARE_INTERRUPT_HANDLER_NMI - Declare NMI interrupt handler function
// @func:	Function name of the entry point
// @returns:	Returns a value back to asm caller
//

//
// DEFINE_INTERRUPT_HANDLER_NMI - Define NMI interrupt handler function
// @func:	Function name of the entry point
// @returns:	Returns a value back to asm caller
//
// @func is called from ASM entry code.
//
// The macro is written so it acts as function definition. Append the
// body with a pair of curly brackets.
//

// nmi_entry if relocations are on */			\
// no nmi_entry for a pseries hash guest		\
// taking a real mode exception */			\
// no nmi_entry for KASAN in real mode */		\
// no nmi_entry if percpu first chunk is not embedded */\
// nmi_exit if relocations are on */			\
// no nmi_exit for a pseries hash guest			\
// taking a real mode exception */			\
// no nmi_exit for KASAN in real mode */		\
// no nmi_exit if percpu first chunk is not embedded */	\
// Interrupt handlers
// kernel/traps.c

// slb.c
// hash_utils.c
// fault.c
// process.c
// time.c
// mce.c
// irq.c
extern "C" {
    pub fn unrecoverable_exception(regs: *mut pt_regs) -> void __noreturn;
}
extern "C" {
    pub fn replay_system_reset();
}
extern "C" {
    pub fn replay_soft_interrupts();
}
extern "C" {
    pub fn system_call_exception(regs: *mut pt_regs, r0: c_ulong) -> c_long;
}
extern "C" {
    pub fn syscall_exit_prepare(r3: c_ulong, regs: *mut pt_regs, scv: c_long) -> notrace unsigned long;
}
extern "C" {
    pub fn interrupt_exit_user_prepare(regs: *mut pt_regs) -> notrace unsigned long;
}
extern "C" {
    pub fn interrupt_exit_kernel_prepare(regs: *mut pt_regs) -> notrace unsigned long;
}

extern "C" {
    pub fn syscall_exit_restart(r3: c_ulong, regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn interrupt_exit_user_restart(regs: *mut pt_regs) -> c_ulong;
}
extern "C" {
    pub fn interrupt_exit_kernel_restart(regs: *mut pt_regs) -> c_ulong;
}

