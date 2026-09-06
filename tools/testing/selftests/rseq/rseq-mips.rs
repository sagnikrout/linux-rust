//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/rseq/rseq-mips.h
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
// Author: Paul Burton <paul.burton@mips.com>
// (C) Copyright 2018 MIPS Tech LLC
// (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//
// RSEQ_SIG uses the break instruction. The instruction pattern is:
//
// On MIPS:
// 0350000d        break     0x350
//
// On nanoMIPS:
// 00100350        break     0x350
//
// On microMIPS:
// 0000d407        break     0x350
//
// For nanoMIPS32 and microMIPS, the instruction stream is encoded as 16-bit
// halfwords, so the signature halfwords need to be swapped accordingly for
// little-endian.
//

// Unknown MIPS architecture.

//
// Exit points of a rseq critical section consist of all instructions outside
// of the critical section where a critical section can either branch to or
// reach through the normal course of its execution. The abort IP and the
// post-commit IP are already part of the __rseq_cs section and should not be
// explicitly defined as additional exit points. Knowing all exit points is
// useful to assist debuggers stepping over the critical section.
//

// Per-cpu-id indexing.
// Macro flag: #define RSEQ_TEMPLATE_CPU_ID
// Macro flag: #define RSEQ_TEMPLATE_MO_RELAXED

// Macro flag: #define RSEQ_TEMPLATE_MO_RELEASE

// Per-mm-cid indexing.
// Macro flag: #define RSEQ_TEMPLATE_MM_CID
// Macro flag: #define RSEQ_TEMPLATE_MO_RELAXED

// Macro flag: #define RSEQ_TEMPLATE_MO_RELEASE

// APIs which are not based on cpu ids.
// Macro flag: #define RSEQ_TEMPLATE_CPU_ID_NONE
// Macro flag: #define RSEQ_TEMPLATE_MO_RELAXED

