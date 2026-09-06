//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/delay.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Copyright 1996, Paul Mackerras.
// Copyright (C) 2009 Freescale Semiconductor, Inc. All rights reserved.
//
// PPC64 Support added by Dave Engebretsen, Todd Inglett, Mike Corrigan,
// Anton Blanchard.
//
extern "C" {
    pub fn __delay(loops: c_ulong);
}
extern "C" {
    pub fn udelay(usecs: c_ulong);
}
//
// On shared processor machines the generic implementation of mdelay can
// result in large errors. While each iteration of the loop inside mdelay
// is supposed to take 1ms, the hypervisor could sleep our partition for
// longer (eg 10ms). With the right timing these errors can add up.
//
// Since there is no 32bit overflow issue on 64bit kernels, just call
// udelay directly.
//

//
// spin_event_timeout - spin until a condition gets true or a timeout elapses
// @condition: a C expression to evalate
// @timeout: timeout, in microseconds
// @delay: the number of microseconds to delay between each evaluation of
// @condition
//
// The process spins until the condition evaluates to true (non-zero) or the
// timeout elapses.  The return value of this macro is the value of
// @condition when the loop terminates. This allows you to determine the cause
// of the loop terminates.  If the return value is zero, then you know a
// timeout has occurred.
//
// This primary purpose of this macro is to poll on a hardware register
// until a status bit changes.  The timeout ensures that the loop still
// terminates even if the bit never changes.  The delay is for devices that
// need a delay in between successive reads.
//
// gcc will optimize out the if-statement if @delay is a constant.
//

