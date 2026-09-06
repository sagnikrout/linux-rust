//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iopoll.h
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
// Copyright (c) 2012-2014 The Linux Foundation. All rights reserved.
//

//
// poll_timeout_us - Periodically poll and perform an operation until
// a condition is met or a timeout occurs
//
// @op: Operation
// @cond: Break condition
// @sleep_us: Maximum time to sleep between operations in us (0 tight-loops).
// Please read usleep_range() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
// @sleep_before_op: if it is true, sleep @sleep_us before operation.
//
// When available, you'll probably want to use one of the specialized
// macros defined below rather than this macro directly.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout. Must not
// be called from atomic context if sleep_us or timeout_us are used.
//

// guarantee 'op' and 'cond' are evaluated after timeout expired */ \
//
// poll_timeout_us_atomic - Periodically poll and perform an operation until
// a condition is met or a timeout occurs
//
// @op: Operation
// @cond: Break condition
// @delay_us: Time to udelay between operations in us (0 tight-loops).
// Please read udelay() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
// @delay_before_op: if it is true, delay @delay_us before operation.
//
// This macro does not rely on timekeeping.  Hence it is safe to call even when
// timekeeping is suspended, at the expense of an underestimation of wall clock
// time, which is rather minimal with a non-zero delay_us.
//
// When available, you'll probably want to use one of the specialized
// macros defined below rather than this macro directly.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout.
//

// guarantee 'op' and 'cond' are evaluated after timeout expired */ \
//
// read_poll_timeout - Periodically poll an address until a condition is
// met or a timeout occurs
// @op: accessor function (takes @args as its arguments)
// @val: Variable to read the value into
// @cond: Break condition (usually involving @val)
// @sleep_us: Maximum time to sleep between reads in us (0 tight-loops). Please
// read usleep_range() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
// @sleep_before_read: if it is true, sleep @sleep_us before read.
// @args: arguments for @op poll
//
// When available, you'll probably want to use one of the specialized
// macros defined below rather than this macro directly.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout. In either
// case, the last read value at @args is stored in @val. Must not
// be called from atomic context if sleep_us or timeout_us are used.
//

//
// read_poll_timeout_atomic - Periodically poll an address until a condition is
// met or a timeout occurs
// @op: accessor function (takes @args as its arguments)
// @val: Variable to read the value into
// @cond: Break condition (usually involving @val)
// @delay_us: Time to udelay between reads in us (0 tight-loops). Please
// read udelay() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
// @delay_before_read: if it is true, delay @delay_us before read.
// @args: arguments for @op poll
//
// This macro does not rely on timekeeping.  Hence it is safe to call even when
// timekeeping is suspended, at the expense of an underestimation of wall clock
// time, which is rather minimal with a non-zero @delay_us.
//
// When available, you'll probably want to use one of the specialized
// macros defined below rather than this macro directly.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout. In either
// case, the last read value at @args is stored in @val.
//

//
// readx_poll_timeout - Periodically poll an address until a condition is met or a timeout occurs
// @op: accessor function (takes @addr as its only argument)
// @addr: Address to poll
// @val: Variable to read the value into
// @cond: Break condition (usually involving @val)
// @sleep_us: Maximum time to sleep between reads in us (0 tight-loops). Please
// read usleep_range() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
//
// When available, you'll probably want to use one of the specialized
// macros defined below rather than this macro directly.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout. In either
// case, the last read value at @addr is stored in @val. Must not
// be called from atomic context if sleep_us or timeout_us are used.
//

//
// readx_poll_timeout_atomic - Periodically poll an address until a condition is met or a timeout occurs
// @op: accessor function (takes @addr as its only argument)
// @addr: Address to poll
// @val: Variable to read the value into
// @cond: Break condition (usually involving @val)
// @delay_us: Time to udelay between reads in us (0 tight-loops). Please
// read udelay() function description for details and
// limitations.
// @timeout_us: Timeout in us, 0 means never timeout
//
// When available, you'll probably want to use one of the specialized
// macros defined below rather than this macro directly.
//
// Returns: 0 on success and -ETIMEDOUT upon a timeout. In either
// case, the last read value at @addr is stored in @val.
//

