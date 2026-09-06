//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/barrier.h
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
// Generic barrier definitions.
//
// It should be possible to use these on really simple architectures,
// but it serves more as a starting point for new ports.
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Architectures that want generic instrumentation can define __ prefixed
// variants of all barriers.
//

//
// Force strict CPU ordering. And yes, this is required on UP too when we're
// talking to devices.
//
// Fall back to compiler barriers if nothing better is provided.
//

// Barriers for virtual machine guests when talking to an SMP host

//
// smp_acquire__after_ctrl_dep() - Provide ACQUIRE ordering after a control dependency
//
// A control dependency provides a LOAD->STORE order, the additional RMB
// provides LOAD->LOAD order, together they provide LOAD->{LOAD,STORE} order,
// aka. (load)-ACQUIRE.
//
// Architectures that do not do load speculation can have this be barrier().
//

//
// smp_cond_load_relaxed() - (Spin) wait for cond with no ordering guarantees
// @ptr: pointer to the variable to wait on
// @cond_expr: boolean expression to wait for
//
// Equivalent to using READ_ONCE() on the condition variable.
//
// Due to C lacking lambda expressions we load the value of *ptr into a
// pre-named variable @VAL to be used in @cond.
//

//
// smp_cond_load_acquire() - (Spin) wait for cond with ACQUIRE ordering
// @ptr: pointer to the variable to wait on
// @cond_expr: boolean expression to wait for
//
// Equivalent to using smp_load_acquire() on the condition variable but employs
// the control dependency of the wait to reduce the barrier on many platforms.
//

//
// pmem_wmb() ensures that all stores for which the modification
// are written to persistent storage by preceding instructions have
// updated persistent storage before any data  access or data transfer
// caused by subsequent instructions is initiated.
//

//
// ioremap_wc() maps I/O memory as memory with write-combining attributes. For
// this kind of memory accesses, the CPU may wait for prior accesses to be
// merged with subsequent ones. In some situation, such wait is bad for the
// performance. io_stop_wc() can be used to prevent the merging of
// write-combining memory accesses before this macro with those after it.
//

//
// Architectures that guarantee an implicit smp_mb() in switch_mm()
// can override smp_mb__after_switch_mm.
//

