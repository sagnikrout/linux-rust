//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/user.h
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
// Adapted from <asm-alpha/user.h>
//
// Core file format: The core file is written in such a way that gdb
// can understand it and provide useful information to the user (under
// linux we use the `trad-core' bfd, NOT the osf-core).  The file contents
// are as follows:
//
// upage: 1 page consisting of a user struct that tells gdb
// what is present in the file.  Directly after this is a
// copy of the task_struct, which is currently not used by gdb,
// but it may come in handy at some point.  All of the registers
// are stored as part of the upage.  The upage should always be
// only one page long.
// data: The data segment follows next.  We use current->end_text to
// current->brk to pick up all of the user variables, plus any memory
// that may have been sbrk'ed.  No attempt is made to determine if a
// page is demand-zero or if a page is totally unused, we just cover
// the entire range.  All of the addresses are rounded in such a way
// that an integral number of pages is written.
// stack: We need the stack information in order to get a meaningful
// backtrace.  We need to write the data from usp to
// current->start_stack, so we round each of these in order to be able
// to write an integer number of pages.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user {
    pub /: *mut *mut user_pt_regs regs; / entire machine state,
    pub /: *mut *mut size_t u_tsize; / text size (pages),
    pub /: *mut *mut size_t u_dsize; / data size (pages),
    pub /: *mut *mut size_t u_ssize; / stack size (pages),
    pub /: *mut *mut unsigned long start_code; / text starting address,
    pub /: *mut *mut unsigned long start_data; / data starting address,
    pub /: *mut *mut unsigned long start_stack; / stack starting address,
    pub /: *mut *mut long int signal; / signal causing core dump,
    pub /: *mut *mut unsigned long u_ar0; / help gdb find registers,
    pub /: *mut *mut unsigned long magic; / identifies a core file,
    pub /: *mut *mut char u_comm[32]; / user command name,
}
