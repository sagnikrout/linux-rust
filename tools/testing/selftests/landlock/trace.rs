//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/landlock/trace.h
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
// Landlock trace test helpers
//
// Copyright © 2026 Cloudflare, Inc.
//

//
// Trace line prefix: matches the ftrace "trace" file format.  Format: "
// <task>-<pid> [<cpu>] <flags> <timestamp>: "
//
// The task parameter must be a string literal truncated to 15 chars
// (TASK_COMM_LEN - 1), matching what the kernel stores in task->comm.  The
// pattern accepts either the expected task name or "<...>" because the ftrace
// comm cache may evict short-lived processes (e.g., forked children that exit
// before the trace buffer is read).
//
// No unescaped '.' in any REGEX macro; literal dots use '\\.'.
//

//
// Task name for events emitted by kworker threads (e.g., free_domain fires from
// a work queue, not from the test process).
//

extern "C" {
    pub fn tracefs_write(_arg: path, _arg: buf) -> return;
}
// Mount tracefs if not already mounted.
// Verify landlock events are available.
//
// Set up PID-based event filtering so only events from the current process and
// its children are recorded.  This is analogous to audit's AUDIT_EXE filter: it
// prevents events from unrelated processes from polluting the trace buffer.
//
// Enable event-fork so children inherit the PID filter.
extern "C" {
    pub fn tracefs_write_int(_arg: TRACEFS_SET_EVENT_PID, _arg: pid) -> return;
}
// Clear the PID filter to stop filtering by PID.
extern "C" {
    pub fn tracefs_write(_arg: TRACEFS_SET_EVENT_PID, _arg: "") -> return;
}
extern "C" {
    pub fn tracefs_write(_arg: enable_path, "0": enable ? "1" :) -> return;
}
extern "C" {
    pub fn tracefs_write(_arg: TRACEFS_TRACE, _arg: "") -> return;
}
//
// Reads the trace buffer content into a newly allocated buffer.  The caller is
// responsible for freeing the returned buffer.  Returns NULL on error.
//
// Counts the number of lines in @buf matching the basic regex @pattern.
// Create a temporary NUL-terminated line.
//
// Extracts the value of a named field from a trace line in @buf.  Searches for
// the first line matching @line_pattern, then extracts the value after
// "@field_name=" into @out.  Stops at space or newline.
//
// Returns 0 on success, -ENOENT if no match.
//
// Find "field_name=" in the line, ensuring a word
// boundary before the field name to avoid substring
// matches (e.g., "port" in "sport").
//
// Common fixture setup for trace tests.  Mounts tracefs if needed and sets a
// PID filter.  The caller must create a mount namespace first
// (unshare(CLONE_NEWNS) + mount(MS_REC | MS_PRIVATE)) to isolate the tracefs
// mount; the trace buffer, per-event enable flags, and PID filter are global
// kernel state, scoped to the test by the PID filter.
//
// Returns 0 on success, -errno on failure (caller should SKIP).
//
extern "C" {
    pub fn tracefs_set_pid_filter(_arg: getpid()) -> return;
}
//
// Temporarily raises CAP_SYS_ADMIN effective capability, calls @func, then
// drops the capability.  Returns the value from @func, or -EPERM if the
// capability manipulation fails.
//
// Read the trace buffer with elevated privileges.  Returns NULL on failure.
// Cannot use tracefs_priv_call() because the return type is char *.
// Clear the trace buffer with elevated privileges.  Returns 0 on success.
extern "C" {
    pub fn tracefs_priv_call(_arg: tracefs_clear) -> return;
}
//
// Forks a child that creates a Landlock sandbox and performs an FS access.  The
// parent waits for the child, then reads the trace buffer.
//
// Requires common.h and wrappers.h to be included before trace.h.
//
// Forks a child that creates a Landlock sandbox allowing execute+read_dir for
// /usr and execute-only for ".", then execs ./true.  The true binary opens "."
// on startup, triggering a read_dir denial with same_exec=0.  The parent waits
// for the child to exit.
//
