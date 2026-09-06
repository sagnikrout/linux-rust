//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/rseq/rseq-arm.h
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
// rseq-arm.h
//
// (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
//
// - ARM little endian
//
// RSEQ_SIG uses the udf A32 instruction with an uncommon immediate operand
// value 0x5de3. This traps if user-space reaches this instruction by mistake,
// and the uncommon operand ensures the kernel does not move the instruction
// pointer to attacker-controlled code on rseq abort.
//
// The instruction pattern in the A32 instruction set is:
//
// e7f5def3    udf    #24035    ; 0x5de3
//
// This translates to the following instruction pattern in the T16 instruction
// set:
//
// little endian:
// def3        udf    #243      ; 0xf3
// e7f5        b.n    <7f5>
//
// - ARMv6+ big endian (BE8):
//
// ARMv6+ -mbig-endian generates mixed endianness code vs data: little-endian
// code and big-endian data. The data value of the signature needs to have its
// byte order reversed to generate the trap instruction:
//
// Data: 0xf3def5e7
//
// Translates to this A32 instruction pattern:
//
// e7f5def3    udf    #24035    ; 0x5de3
//
// Translates to this T16 instruction pattern:
//
// def3        udf    #243      ; 0xf3
// e7f5        b.n    <7f5>
//
// - Prior to ARMv6 big endian (BE32):
//
// Prior to ARMv6, -mbig-endian generates big-endian code and data
// (which match), so the endianness of the data representation of the
// signature should not be reversed. However, the choice between BE32
// and BE8 is done by the linker, so we cannot know whether code and
// data endianness will be mixed before the linker is invoked. So rather
// than try to play tricks with the linker, the rseq signature is simply
// data (not a trap instruction) prior to ARMv6 on big endian. This is
// why the signature is expressed as data (.word) rather than as
// instruction (.inst) in assembler.
//

pub const RSEQ_SIG: c_uint = 0xf3def5e7      /* udf    #24035    ; 0x5de3 (ARMv6+) */;

pub const RSEQ_SIG: c_uint = 0xe7f5def3      /* udf    #24035    ; 0x5de3 */;

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

