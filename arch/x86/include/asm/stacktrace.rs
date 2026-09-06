//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/stacktrace.h
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
// Copyright (C) 1991, 1992  Linus Torvalds
// Copyright (C) 2000, 2001, 2002 Andi Kleen, SuSE Labs
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stack_type {
    STACK_TYPE_UNKNOWN,
    STACK_TYPE_TASK,
    STACK_TYPE_IRQ,
    STACK_TYPE_SOFTIRQ,
    STACK_TYPE_ENTRY,
    STACK_TYPE_EXCEPTION,
    STACK_TYPE_EXCEPTION_LAST = STACK_TYPE_EXCEPTION + N_EXCEPTION_STACKS-1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_info {
    pub type: stack_type,
    pub next_sp: *mut *mut *mut unsigned long begin, end,,
}

extern "C" {
    pub fn in_entry_stack(stack: *mut c_ulong, info: *mut stack_info) -> bool;
}
// make sure it's not in the stack proper
// but if it is in the page below it, we hit a guard
extern "C" {
    pub fn get_stack_info_noinstr(PAGE_SIZE: *mut *mut (void )stack +, _arg: current, _arg: info) -> return;
}

pub const STACKSLOTS_PER_LINE: c_int = 8;

pub const STACKSLOTS_PER_LINE: c_int = 4;

extern "C" {
    pub fn __builtin_frame_address(_arg: 0) -> return;
}

extern "C" {
    pub fn __builtin_frame_address(_arg: 0) -> return;
}
// The form of the top of the frame on the stack
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_frame {
    pub next_frame: *mut stack_frame,
    pub return_address: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_frame_ia32 {
    pub next_frame: u32,
    pub return_address: u32,
}

extern "C" {
    pub fn show_opcodes(regs: *mut pt_regs, loglvl: *const c_char);
}
extern "C" {
    pub fn show_ip(regs: *mut pt_regs, loglvl: *const c_char);
}
