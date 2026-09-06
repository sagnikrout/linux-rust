//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regset.h
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
// User-mode machine state access
//
// Copyright (C) 2007 Red Hat, Inc.  All rights reserved.
//
// Red Hat Author: Roland McGrath.
//
pub const _LINUX_REGSET_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct membuf {
    pub p: *mut c_void,
    pub left: usize,
}

// current s->p must be aligned for v; v must be a scalar

// (typeof(__v + 0) *)__s->p = __v;	\
//
// user_regset_active_fn - type of @active function in &struct user_regset
// @target:	thread being examined
// @regset:	regset being examined
//
// Return -%ENODEV if not available on the hardware found.
// Return %0 if no interesting state in this thread.
// Return >%0 number of @size units of interesting state.
// Any get call fetching state beyond that number will
// see the default initialization state for this data,
// so a caller that knows what the default state is need
// not copy it all out.
// This call is optional; the pointer is %NULL if there
// is no inexpensive check to yield a value < @n.
//
// user_regset_set_fn - type of @set function in &struct user_regset
// @target:	thread being examined
// @regset:	regset being examined
// @pos:	offset into the regset data to access, in bytes
// @count:	amount of data to copy, in bytes
// @kbuf:	if not %NULL, a kernel-space pointer to copy from
// @ubuf:	if @kbuf is %NULL, a user-space pointer to copy from
//
// Store register values.  Return %0 on success; -%EIO or -%ENODEV
// are usual failure returns.  The @pos and @count values are in
// bytes, but must be properly aligned.  If @kbuf is non-null, that
// buffer is used and @ubuf is ignored.  If @kbuf is %NULL, then
// ubuf gives a userland pointer to access directly, and an -%EFAULT
// return value is possible.
//
// user_regset_writeback_fn - type of @writeback function in &struct user_regset
// @target:	thread being examined
// @regset:	regset being examined
// @immediate:	zero if writeback at completion of next context switch is OK
//
// This call is optional; usually the pointer is %NULL.  When
// provided, there is some user memory associated with this regset's
// hardware, such as memory backing cached register data on register
// window machines; the regset's data controls what user memory is
// used (e.g. via the stack pointer value).
//
// Write register data back to user memory.  If the @immediate flag
// is nonzero, it must be written to the user memory so uaccess or
// access_process_vm() can see it when this call returns; if zero,
// then it must be written back by the time the task completes a
// context switch (as synchronized with wait_task_inactive()).
// Return %0 on success or if there was nothing to do, -%EFAULT for
// a memory problem (bad stack pointer or whatever), or -%EIO for a
// hardware problem.
//
// struct user_regset - accessible thread CPU state
// @n:			Number of slots (registers).
// @size:		Size in bytes of a slot (register).
// @align:		Required alignment, in bytes.
// @bias:		Bias from natural indexing.
// @core_note_type:	ELF note @n_type value used in core dumps.
// @core_note_name:	ELF note name to qualify the note type.
// @regset_get:		Function to fetch values.
// @set:		Function to store values.
// @active:		Function to report if regset is active, or %NULL.
// @writeback:		Function to write data back to user memory, or %NULL.
//
// This data structure describes a machine resource we call a register set.
// This is part of the state of an individual thread, not necessarily
// actual CPU registers per se.  A register set consists of a number of
// similar slots, given by @n.  Each slot is @size bytes, and aligned to
// @align bytes (which is at least @size).  For dynamically-sized
// regsets, @n must contain the maximum possible number of slots for the
// regset.
//
// For backward compatibility, the @get and @set methods must pad to, or
// accept, @n * @size bytes, even if the current regset size is smaller.
// The precise semantics of these operations depend on the regset being
// accessed.
//
// The functions to which &struct user_regset members point must be
// called only on the current thread or on a thread that is in
// %TASK_STOPPED or %TASK_TRACED state, that we are guaranteed will not
// be woken up and return to user mode, and that we have called
// wait_task_inactive() on.  (The target thread always might wake up for
// SIGKILL while these functions are working, in which case that
// thread's user_regset state might be scrambled.)
//
// The @pos argument must be aligned according to @align; the @count
// argument must be a multiple of @size.  These functions are not
// responsible for checking for invalid arguments.
//
// When there is a natural value to use as an index, @bias gives the
// difference between the natural index and the slot index for the
// register set.  For example, x86 GDT segment descriptors form a regset;
// the segment selector produces a natural index, but only a subset of
// that index space is available as a regset (the TLS slots); subtracting
// @bias from a segment selector index value computes the regset slot.
//
// If nonzero, @core_note_type gives the n_type field (NT_* value)
// of the core file note in which this regset's data appears.
// @core_note_name specifies the note name.  The preferred way to
// specify these two fields is to use the @USER_REGSET_NOTE_TYPE()
// macro.
//
// NT_PRSTATUS is a special case in that the regset data starts at
// offsetof(struct elf_prstatus, pr_reg) into the note data; that is
// part of the per-machine ELF formats userland knows about.  In
// other cases, the core file note contains exactly the whole regset
// (@n * @size) and nothing else.  The core file note is normally
// omitted when there is an @active function and it returns zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_regset {
    pub regset_get: *mut user_regset_get2_fn,
    pub set: *mut user_regset_set_fn,
    pub active: *mut user_regset_active_fn,
    pub writeback: *mut user_regset_writeback_fn,
    pub n: c_uint,
    pub size: c_uint,
    pub align: c_uint,
    pub bias: c_uint,
    pub core_note_type: c_uint,
    pub core_note_name: *const c_char,
}

//
// struct user_regset_view - available regsets
// @name:	Identifier, e.g. UTS_MACHINE string.
// @regsets:	Array of @n regsets available in this view.
// @n:		Number of elements in @regsets.
// @e_machine:	ELF header @e_machine %EM_* value written in core dumps.
// @e_flags:	ELF header @e_flags value written in core dumps.
// @ei_osabi:	ELF header @e_ident[%EI_OSABI] value written in core dumps.
//
// A regset view is a collection of regsets (&struct user_regset,
// above).  This describes all the state of a thread that can be seen
// from a given architecture/ABI environment.  More than one view might
// refer to the same &struct user_regset, or more than one regset
// might refer to the same machine-specific state in the thread.  For
// example, a 32-bit thread's state could be examined from the 32-bit
// view or from the 64-bit view.  Either method reaches the same thread
// register state, doing appropriate widening or truncation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_regset_view {
    pub name: *const c_char,
    pub regsets: *const user_regset,
    pub n: c_uint,
    pub e_flags: u32,
    pub e_machine: u16,
    pub ei_osabi: u8,
}

//
// This is documented here rather than at the definition sites because its
// implementation is machine-dependent but its interface is universal.
//
// task_user_regset_view - Return the process's native regset view.
// @tsk: a thread of the process in question
//
// Return the &struct user_regset_view that is native for the given process.
// For example, what it would access when it called ptrace().
// Throughout the life of the process, this only changes at exec.
//
// kbuf += copy;
// ubuf += copy;
// pos += copy;
// count -= copy;
// kbuf += copy;
// ubuf += copy;
// pos += copy;
// count -= copy;
//
// copy_regset_from_user - store into thread's user_regset data from user memory
// @target:	thread to be examined
// @view:	&struct user_regset_view describing user thread machine state
// @setno:	index in @view->regsets
// @offset:	offset into the regset data, in bytes
// @size:	amount of data to copy, in bytes
// @data:	user-mode pointer to copy from
//
