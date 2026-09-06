//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/exec/binfmt_misc_common.h
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
// Helpers shared by the binfmt_misc selftests.

// comm holds 15 usable chars; a read of /proc/self/comm appends a newline.
pub const TASK_COMM_LEN: c_int = 16;
// The canonical payload argv: run_payload() passes it, the payloads assert it.

// Marker the loader tests poke into the payload's e_ident padding.

// Exit status run_payload() reports when the exec was refused as unhandled.
pub const RUN_ENOEXEC: c_int = 42;
// The tests share /tmp, so never write through a name they don't own.
// Write @rule to the register file, preserving the write's errno.
// Write @line to @entry's file, reporting the errno it was refused with.
// Does @entry's file report @line?
// Mount binfmt_misc unless it already is, and report whether it is usable.
// Absolute path of @name in the directory this test was built into.
// Probe kernel support for a registration flag with a throwaway entry.
//
// Run @path with the canonical payload argv and return its exit status, or
// RUN_ENOEXEC when the exec itself was refused as unhandled.
//
extern "C" {
    pub fn WEXITSTATUS(_arg: status) -> return;
}
// Does the exe link name @path?
// Is comm @name truncated to what a comm can hold?
// Opening @path for writing has to fail with ETXTBSY.
// start_code and end_code are the 26th and 27th fields of /proc/pid/stat.
// Skip "pid (comm)", then start_code is the 24th field after it.
// Find the system loader through our own PT_INTERP.
