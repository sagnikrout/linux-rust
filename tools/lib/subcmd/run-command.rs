//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/subcmd/run-command.h
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
pub struct child_process {
    pub argv: *const c_char,
    pub pid: pid_t,
//
// Using .in, .out, .err:
// - Specify 0 for no redirections (child inherits stdin, stdout,
// stderr from parent).
// - Specify -1 to have a pipe allocated as follows:
// .in: returns the writable pipe end; parent writes to it,
// the readable pipe end becomes child's stdin
// .out, .err: returns the readable pipe end; parent reads from
// it, the writable pipe end becomes child's stdout/stderr
// The caller of start_command() must close the returned FDs
// after it has completed reading from/writing to it!
// - Specify > 0 to set a channel to a particular FD as follows:
// .in: a readable FD, becomes child's stdin
// .out: a writable FD, becomes child's stdout/stderr
// .err > 0 not supported
// The specified FD is closed by start_command(), even in case
// of errors!
//
    pub in: c_int,
    pub out: c_int,
    pub err: c_int,
    pub dir: *const c_char,
    pub env: *const *const c_char,
    pub finish_result: c_int,
    pub no_stdin:1: unsigned,
    pub no_stdout:1: unsigned,
    pub no_stderr:1: unsigned,
    pub /: *mut *mut unsigned exec_cmd:1; / if this is to be external sub-command,
    pub stdout_to_stderr:1: unsigned,
    pub finished:1: unsigned,
    pub (*preexec_cb)(void): *mut c_void,
// If set, call function in child rather than doing an exec.
    pub process): *mut *mut int (no_exec_cmd)(struct child_process,
}

extern "C" {
    pub fn start_command(: *mut child_process) -> c_int;
}
extern "C" {
    pub fn check_if_command_finished(: *mut child_process) -> c_int;
}
extern "C" {
    pub fn finish_command(: *mut child_process) -> c_int;
}
extern "C" {
    pub fn run_command(: *mut child_process) -> c_int;
}
pub const RUN_COMMAND_NO_STDIN: c_int = 1;

pub const RUN_COMMAND_STDOUT_TO_STDERR: c_int = 4;
extern "C" {
    pub fn run_command_v_opt(argv: *const c_char, opt: c_int) -> c_int;
}
