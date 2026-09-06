//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/stacktrace.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_frame_user {
    pub back_chain: c_ulong,
    pub empty1: [c_ulong; 5],
    pub gprs: [c_ulong; 10],
    pub empty2: [c_ulong; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_frame_vdso_wrapper {
    pub sf: stack_frame_user,
    pub return_address: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stack_type {
    STACK_TYPE_UNKNOWN,
    STACK_TYPE_TASK,
    STACK_TYPE_IRQ,
    STACK_TYPE_NODAT,
    STACK_TYPE_RESTART,
    STACK_TYPE_MCCK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_info {
    pub type: stack_type,
    pub end: unsigned long begin,,
}

//
// Stack layout of a C stack frame.
// Kernel uses the packed stack layout (-mpacked-stack).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_frame {
    pub empty: [c_ulong; 9],
    pub sie_control_block: c_ulong,
    pub sie_savearea: c_ulong,
    pub sie_return: c_ulong,
    pub sie_flags: c_ulong,
    pub sie_control_block_phys: c_ulong,
    pub sie_guest_asce: c_ulong,
    pub sie_irq: c_ulong,
}

//
// Unlike current_stack_pointer which simply contains the current value of %r15
// current_frame_address() returns function stack frame address, which matches
// %r15 upon function invocation. It may differ from %r15 later if function
// allocates stack for local variables or new stack frame to call other
// functions.
//

extern "C" {
    pub fn current_frame_address() -> return;
}
//
// To keep this simple mark register 2-6 as being changed (volatile)
// by the called function, even though register 6 is saved/nonvolatile.
//

// Macro flag: #define CALL_TYPECHECK_0(...)

//
// Use call_on_stack() to call a function switching to a specified
// stack. Proper sign and zero extension of function arguments is
// done. Usage:
//
// rc = call_on_stack(nr, stack, rettype, fn, t1, a1, t2, a2, ...)
//
// - nr specifies the number of function arguments of fn.
// - stack specifies the stack to be used.
// - fn is the function to be called.
// - rettype is the return type of fn.
// - t1, a1, ... are pairs, where t1 must match the type of the first
// argument of fn, t2 the second, etc. a1 is the corresponding
// first function argument (not name), etc.
//

//
// Use call_nodat() to call a function with DAT disabled.
// Proper sign and zero extension of function arguments is done.
// Usage:
//
// rc = call_nodat(nr, rettype, fn, t1, a1, t2, a2, ...)
//
// - nr specifies the number of function arguments of fn.
// - fn is the function to be called, where fn is a physical address.
// - rettype is the return type of fn.
// - t1, a1, ... are pairs, where t1 must match the type of the first
// argument of fn, t2 the second, etc. a1 is the corresponding
// first function argument (not name), etc.
//
// fn() is called with standard C function call ABI, with the exception
// that no useful stackframe or stackpointer is passed via register 15.
// Therefore the called function must not use r15 to access the stack.
//

// aligned since psw_leave must not cross page boundary */	\
