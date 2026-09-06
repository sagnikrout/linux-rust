//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ptrace.h
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
// Copyright (C) 2001 PPC64 Team, IBM Corp
//
// This struct defines the way the registers are stored on the
// kernel stack during a system call or other kernel entry.
//
// this should only contain volatile regs
// since we can keep non-volatile in the thread_struct
// should set this up when only volatiles are saved
// by intr code.
//
// Since this is going on the stack, *CARE MUST BE TAKEN* to insure
// that the overall structure is a multiple of 16 bytes in length.
//
// Note that the offsets of the fields in this struct correspond with
// the PT_* values below.  This simplifies arch/powerpc/kernel/ptrace.c.
//

// Always displays as "REGS" in memory dumps

//
// Size of redzone that userspace is allowed to use below the stack
// pointer.  This is 288 in the 64-bit big-endian ELF ABI, and 512 in
// the new ELFv2 little-endian ABI, so we allow the larger amount.
//
// For kernel code we allow a 288-byte redzone, in order to conserve
// kernel stack space; gcc currently only uses 288 bytes, and will
// hopefully allow explicit control of the redzone size in future.
//
pub const USER_REDZONE_SIZE: c_int = 512;
pub const KERNEL_REDZONE_SIZE: c_int = 288;

pub const STACK_FRAME_MIN_SIZE: c_int = 32;

//
// The ELFv1 ABI specifies 48 bytes plus a minimum 64 byte parameter save
// area. This parameter area is not used by calls to C from interrupt entry,
// so the second from last one of those is used for the frame marker.
//
pub const STACK_FRAME_MIN_SIZE: c_int = 112;

// Size of dummy stack frame allocated when calling signal handler.
pub const __SIGNAL_FRAMESIZE: c_int = 128;
pub const __SIGNAL_FRAMESIZE32: c_int = 64;

pub const USER_REDZONE_SIZE: c_int = 0;
pub const KERNEL_REDZONE_SIZE: c_int = 0;
pub const STACK_FRAME_MIN_SIZE: c_int = 16;

// Size of stack frame allocated when calling signal handler.
pub const __SIGNAL_FRAMESIZE: c_int = 64;

extern "C" {
    pub fn profile_pc(regs: *mut pt_regs) -> c_ulong;
}

//
// The 4 low bits (0xf) are available as flags to overload the trap word,
// because interrupt vectors have minimum alignment of 0x10. TRAP_FLAGS_MASK
// must cover the bits used as flags, including bit 0 which is used as the
// "norestart" bit.
//

pub const TRAP_FLAGS_MASK: c_uint = 0x1;

//
// On 4xx we use bit 1 in the trap word to indicate whether the exception
// is a critical exception (1 means it is).
//
pub const TRAP_FLAGS_MASK: c_uint = 0xf;

extern "C" {
    pub fn unlikely(MSR_RI): cpu_has_msr_ri() && !(regs->msr &) -> return;
}

// Macro flag: #define ARCH_HAS_USER_SINGLE_STEP_REPORT
//
// kprobe-based event tracer support
//

extern "C" {
    pub fn regs_query_register_offset(name: *const c_char) -> c_int;
}

//
// regs_get_register() - get register value from its offset
// @regs:	   pt_regs from which register value is gotten
// @offset:    offset number of the register.
//
// regs_get_register returns the value of a register whose offset from @regs.
// The @offset is the offset of the register in struct pt_regs.
// If @offset is bigger than MAX_REG_OFFSET, this returns 0.
//
// regs_within_kernel_stack() - check the address in the stack
// @regs:      pt_regs which contains kernel stack pointer.
// @addr:      address which is checked.
//
// regs_within_kernel_stack() checks @addr is within the kernel stack page(s).
// If @addr is within the kernel stack, it returns true. If not, returns false.
//
// regs_get_kernel_stack_nth() - get Nth entry of the stack
// @regs:	pt_regs which contains kernel stack pointer.
// @n:		stack entry number.
//
// regs_get_kernel_stack_nth() returns @n th entry of the kernel stack which
// is specified by @regs. If the @n th entry is NOT in the kernel stack,
// this returns 0.
//
// regs_get_kernel_argument() - get Nth function argument in kernel
// @regs:	pt_regs of that context
// @n:		function argument number (start from 0)
//
// We support up to 8 arguments and assume they are sent in through the GPRs.
// This will fail for fp/vector arguments, but those aren't usually found in
// kernel code. This is expected to be called from kprobes or ftrace with regs.
//
pub const NR_REG_ARGUMENTS: c_int = 8;
extern "C" {
    pub fn regs_get_register(_arg: regs, pt_regs: offsetof(struct, n]): gpr[3 +) -> return;
}

// We need PT_SOFTE defined at all time to avoid #ifdefs

