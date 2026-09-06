//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/stacktrace/common.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Common arm64 stack unwinder code.
//
// See: arch/arm64/kernel/stacktrace.c for the reference implementation.
//
// Copyright (C) 2012 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_info {
    pub low: c_ulong,
    pub high: c_ulong,
}

//
// struct unwind_state - state used for robust unwinding.
//
// @fp:          The fp value in the frame record (or the real fp)
// @pc:          The lr value in the frame record (or the real lr)
//
// @stack:       The stack currently being unwound.
// @stacks:      An array of stacks which can be unwound.
// @nr_stacks:   The number of stacks in @stacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_state {
    pub fp: c_ulong,
    pub pc: c_ulong,
    pub stack: stack_info,
    pub stacks: *mut stack_info,
    pub nr_stacks: c_int,
}

//
// unwind_find_stack() - Find the accessible stack which entirely contains an
// object.
//
// @state: the current unwind state.
// @sp:    the base address of the object.
// @size:  the size of the object.
//
// Return: a pointer to the relevant stack_info if found; NULL otherwise.
//
// unwind_consume_stack() - Update stack boundaries so that future unwind steps
// cannot consume this object again.
//
// @state: the current unwind state.
// @info:  the stack_info of the stack containing the object.
// @sp:    the base address of the object.
// @size:  the size of the object.
//
// Return: 0 upon success, an error code otherwise.
//
// Stack transitions are strictly one-way, and once we've
// transitioned from one stack to another, it's never valid to
// unwind back to the old stack.
//
// Destroy the old stack info so that it cannot be found upon a
// subsequent transition. If the stack has not changed, we'll
// immediately restore the current stack info.
//
// Note that stacks can nest in several valid orders, e.g.
//
// TASK -> IRQ -> OVERFLOW -> SDEI_NORMAL
// TASK -> SDEI_NORMAL -> SDEI_CRITICAL -> OVERFLOW
// HYP -> OVERFLOW
//
// ... so we do not check the specific order of stack
// transitions.
//
// info = stackinfo_get_unknown();
//
// Future unwind steps can only consume stack above this frame record.
// Update the current stack to start immediately above it.
//
// unwind_next_frame_record() - Unwind to the next frame record.
//
// @state:        the current unwind state.
//
// Return: 0 upon success, an error code otherwise.
//
// Record this frame record's values.
//
