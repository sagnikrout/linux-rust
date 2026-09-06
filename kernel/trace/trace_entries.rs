//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/trace_entries.h
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
// This file defines the trace event structures that go into the ring
// buffer directly. They are created via macros so that changes for them
// appear in the format file. Using macros will automate this process.
//
// The macro used to create a ftrace data structure is:
//
// FTRACE_ENTRY( name, struct_name, id, structure, print )
//
// @name: the name used the event name, as well as the name of
// the directory that holds the format file.
//
// @struct_name: the name of the structure that is created.
//
// @id: The event identifier that is used to detect what event
// this is from the ring buffer.
//
// @structure: the structure layout
//
// - __field(	type,	item	)
// This is equivalent to declaring
// type	item;
// in the structure.
// - __array(	type,	item,	size	)
// This is equivalent to declaring
// type	item[size];
// in the structure.
//
// * for structures within structures, the format of the internal
// structure is laid out. This allows the internal structure
// to be deciphered for the format file. Although these macros
// may become out of sync with the internal structure, they
// will create a compile error if it happens. Since the
// internal structures are just tracing helpers, this is not
// an issue.
//
// When an internal structure is used, it should use:
//
// __field_struct(	type,	item	)
//
// instead of __field. This will prevent it from being shown in
// the output file. The fields in the structure should use.
//
// __field_desc(	type,	container,	item		)
// __array_desc(	type,	container,	item,	len	)
//
// type, item and len are the same as __field and __array, but
// container is added. This is the name of the item in
// __field_struct that this is describing.
//
// @print: the print format shown to users in the format file.
//
// Function trace entry - function address and parent function address:
//
// Function call entry

// Function call entry with a return address

// Function return entry

// Function return entry

//
// Context switch trace entry - which task (and prio) we switched from/to:
//
// This is used for both wakeup and context switches. We only want
// to create one structure, but we need two outputs for it.
//

//
// FTRACE_ENTRY_DUP only creates the format file, it will not
// create another structure.
//
// Stack-trace entry:
//
pub const FTRACE_STACK_ENTRIES: c_int = 8;
//
// trace_printk entry:
//
pub const TRACE_FUNC_SIZE: c_int = 30;
pub const TRACE_FILE_SIZE: c_int = 20;
