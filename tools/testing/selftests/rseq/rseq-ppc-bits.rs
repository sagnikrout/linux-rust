//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/rseq/rseq-ppc-bits.h
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


// SPDX-License-Identifier: LGPL-2.1 OR MIT
//
// rseq-ppc-bits.h
//
// (C) Copyright 2016-2018 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
// (C) Copyright 2016-2018 - Boqun Feng <boqun.feng@gmail.com>
//

// Start rseq by storing table entry pointer into rseq_cs.
// cmp cpuid
// cmp @v equal to @expect

// cmp cpuid
// cmp @v equal to @expect

// final store

// Start rseq by storing table entry pointer into rseq_cs.
// cmp cpuid
// cmp @v not equal to @expectnot

// cmp cpuid
// cmp @v not equal to @expectnot

// load the value of @v
// store it in @load
// dereference voffp(v)
// final store the value at voffp(v)
// final store input

// Start rseq by storing table entry pointer into rseq_cs.
// cmp cpuid

// cmp cpuid

// load the value of @v
// add @count to it
// final store
// final store input

// Start rseq by storing table entry pointer into rseq_cs.
// cmp cpuid
// cmp @v equal to @expect
// cmp @v2 equal to @expct2

// cmp cpuid
// cmp @v equal to @expect
// cmp @v2 equal to @expct2

// final store
// cmp2 input
// final store input

// Start rseq by storing table entry pointer into rseq_cs.
// cmp cpuid
// cmp @v equal to @expect

// cmp cpuid
// cmp @v equal to @expect

// try store

// for 'release'

// final store
// try store input
// final store input

// setup for mempcy
// Start rseq by storing table entry pointer into rseq_cs.
// cmp cpuid
// cmp @v equal to @expect

// cmp cpuid
// cmp @v equal to @expect

// try memcpy

// for 'release'

// final store
// teardown
// final store input
// try memcpy input

