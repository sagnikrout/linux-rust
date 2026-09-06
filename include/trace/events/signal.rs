//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/signal.h
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
// signal_generate - called when a signal is generated
// @sig: signal number
// @info: pointer to struct siginfo
// @task: pointer to struct task_struct
// @group: shared or private
// @result: TRACE_SIGNAL_
//
// Current process sends a 'sig' signal to 'task' process with
// 'info' siginfo. If 'info' is SEND_SIG_NOINFO or SEND_SIG_PRIV,
// 'info' is not a pointer and you can't access its field. Instead,
// SEND_SIG_NOINFO means that si_code is SI_USER, and SEND_SIG_PRIV
// means that si_code is SI_KERNEL.
//
// signal_deliver - called when a signal is delivered
// @sig: signal number
// @info: pointer to struct siginfo
// @ka: pointer to struct k_sigaction
//
// A 'sig' signal is delivered to current process with 'info' siginfo,
// and it will be handled by 'ka'. ka->sa.sa_handler can be SIG_IGN or
// SIG_DFL.
// Note that some signals reported by signal_generate tracepoint can be
// lost, ignored or modified (by debugger) before hitting this tracepoint.
// This means, this can show which signals are actually delivered, but
// matching generated signals and delivered signals may not be correct.
//

// This part must be outside protection
