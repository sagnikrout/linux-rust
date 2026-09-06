//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/debug/kdb/kdb_private.h
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
// Kernel Debugger Architecture Independent Private Headers
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (c) 2000-2004 Silicon Graphics, Inc.  All Rights Reserved.
// Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
//

// Kernel Debugger Command codes.  Must not overlap with error codes.

// Internal debug flags
pub const KDB_DEBUG_FLAG_BP: c_uint = 0x0002	/* Breakpoint subsystem debug */;
pub const KDB_DEBUG_FLAG_BB_SUMM: c_uint = 0x0004	/* Basic block analysis, summary only */;
pub const KDB_DEBUG_FLAG_AR: c_uint = 0x0008	/* Activation record, generic */;
pub const KDB_DEBUG_FLAG_ARA: c_uint = 0x0010	/* Activation record, arch specific */;
pub const KDB_DEBUG_FLAG_BB: c_uint = 0x0020	/* All basic block analysis */;
pub const KDB_DEBUG_FLAG_STATE: c_uint = 0x0040	/* State flags */;
pub const KDB_DEBUG_FLAG_MASK: c_uint = 0xffff	/* All debug flags */;

//
// KDB_MAXBPT describes the total number of breakpoints
// supported by this architecture.
//
pub const KDB_MAXBPT: c_int = 16;
// Symbol table format returned by kallsyms.
// "kernel"
// any version
extern "C" {
    pub fn kallsyms_symbol_next(prefix_name: *mut c_char, flag: c_int, buf_size: c_int) -> c_int;
}
extern "C" {
    pub fn kallsyms_symbol_complete(prefix_name: *mut c_char, max_len: c_int) -> c_int;
}
// Exported Symbols for kernel loadable modules to use.
extern "C" {
    pub fn kdb_getarea_size(: *mut c_void, long: unsigned, _arg: usize) -> c_int;
}
extern "C" {
    pub fn kdb_putarea_size(long: unsigned, : *mut c_void, _arg: usize) -> c_int;
}
//
// Like get_user and put_user, kdb_getarea and kdb_putarea take variable
// names, not pointers.  The underlying *_size functions take pointers.
//

extern "C" {
    pub fn kdb_getword(: *mut c_ulong, long: unsigned, _arg: usize) -> c_int;
}
extern "C" {
    pub fn kdb_putword(long: unsigned, long: unsigned, _arg: usize) -> c_int;
}
extern "C" {
    pub fn kdbgetularg(: *const c_char, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn kdbgetu64arg(: *const c_char, : *mut u64) -> c_int;
}
extern "C" {
    pub fn kdbgetsymval(: *const c_char, : *mut kdb_symtab_t) -> c_int;
}
extern "C" {
    pub fn kdbnearsym(long: unsigned, : *mut kdb_symtab_t) -> c_int;
}
extern "C" {
    pub fn kdb_symbol_print(long: unsigned, : *const kdb_symtab_t, int: unsigned);
}
// Routine for debugging the debugger state.
extern "C" {
    pub fn kdb_print_state(: *const c_char, _arg: c_int);
}
pub const KDB_STATE_KDB: c_uint = 0x00000001	/* Cpu is inside kdb */;
pub const KDB_STATE_LEAVING: c_uint = 0x00000002	/* Cpu is leaving kdb */;
pub const KDB_STATE_CMD: c_uint = 0x00000004	/* Running a kdb command */;
pub const KDB_STATE_KDB_CONTROL: c_uint = 0x00000008	/* This cpu is under;
// kdb control
pub const KDB_STATE_HOLD_CPU: c_uint = 0x00000010	/* Hold this cpu inside kdb */;
pub const KDB_STATE_DOING_SS: c_uint = 0x00000020	/* Doing ss command */;
pub const KDB_STATE_SSBPT: c_uint = 0x00000080	/* Install breakpoint;
// after one ss, independent of
// DOING_SS
pub const KDB_STATE_REENTRY: c_uint = 0x00000100	/* Valid re-entry into kdb */;
pub const KDB_STATE_SUPPRESS: c_uint = 0x00000200	/* Suppress error messages */;
pub const KDB_STATE_PAGER: c_uint = 0x00000400	/* pager is available */;
pub const KDB_STATE_GO_SWITCH: c_uint = 0x00000800	/* go is switching;
// back to initial cpu
pub const KDB_STATE_WAIT_IPI: c_uint = 0x00002000	/* Waiting for kdb_ipi() NMI */;
pub const KDB_STATE_RECURSE: c_uint = 0x00004000	/* Recursive entry to kdb */;
pub const KDB_STATE_IP_ADJUSTED: c_uint = 0x00008000	/* Restart IP has been;
// adjusted
pub const KDB_STATE_GO1: c_uint = 0x00010000	/* go only releases one cpu */;
pub const KDB_STATE_KEYBOARD: c_uint = 0x00020000	/* kdb entered via;
// keyboard on this cpu
pub const KDB_STATE_KEXEC: c_uint = 0x00040000	/* kexec issued */;
pub const KDB_STATE_DOING_KGDB: c_uint = 0x00080000	/* kgdb enter now issued */;
pub const KDB_STATE_KGDB_TRANS: c_uint = 0x00200000	/* Transition to kgdb */;
pub const KDB_STATE_ARCH: c_uint = 0xff000000	/* Reserved for arch;
// specific use

extern "C" {
    pub fn kdb_register_table(kp: *mut kdbtab_t, len: usize);
}
// KDB breakpoint management functions
extern "C" {
    pub fn kdb_initbptab();
}
extern "C" {
    pub fn kdb_bp_install(: *mut pt_regs);
}
extern "C" {
    pub fn kdb_bp_remove();
}
// Miscellaneous functions and data areas
pub const KDB_GREPPING_FLAG_SEARCH: c_uint = 0x8000;
pub const KDB_GREP_STRLEN: c_int = 256;
extern "C" {
    pub fn kdb_task_state_char(: *const task_struct) -> c_char;
}
extern "C" {
    pub fn kdb_task_state(p: *const task_struct, mask: *const c_char) -> bool;
}
extern "C" {
    pub fn kdb_ps_suppressed();
}
extern "C" {
    pub fn kdb_ps1(p: *const task_struct);
}
extern "C" {
    pub fn kdb_getchar() -> c_char;
}
extern "C" {
    pub fn kdb_gdb_state_pass(buf: *mut c_char);
}
// Defines for kdb_symbol_print
pub const KDB_SP_SPACEB: c_uint = 0x0001		/* Space before string */;
pub const KDB_SP_SPACEA: c_uint = 0x0002		/* Space after string */;
pub const KDB_SP_PAREN: c_uint = 0x0004		/* Parenthesis around string */;
pub const KDB_SP_VALUE: c_uint = 0x0008		/* Print the value of the address */;
pub const KDB_SP_SYMSIZE: c_uint = 0x0010		/* Print the size of the symbol */;
pub const KDB_SP_NEWLINE: c_uint = 0x0020		/* Newline after string */;

extern "C" {
    pub fn kdb_kbd_cleanup_state();
}

// Macro flag: #define kdb_kbd_cleanup_state()

