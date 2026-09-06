//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/net/bpf_jit.h
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
// BPF JIT compiler for ARM64
//
// Copyright (C) 2014-2016 Zi Shen Lim <zlim.lnx@gmail.com>
//

// 5-bit Register Operand

// Compare & branch (immediate)

// Conditional branch (immediate)

// Unconditional branch (immediate)

// Unconditional branch (register)

// Load/store register (register offset)

// Load/store register (immediate offset)

// LDR (literal)

// Load/store register pair

// Rn -= 16; Rn[0] = Rt; Rn[8] = Rt2;

// Rt = Rn[0]; Rt2 = Rn[8]; Rn += 16;

// Load/store exclusive

// Rt = [Rn]; (atomic)

// [Rn] = Rt; (atomic) Rs = [state]

// [Rn] = Rt (store release); (atomic) Rs = [state]

// Load-acquire & store-release

// Rt = [Rn] (load acquire)

// [Rn] = Rt (store release)

//
// LSE atomics
//
// ST{ADD,CLR,SET,EOR} is simply encoded as an alias for
// LDD{ADD,CLR,SET,EOR} with XZR as the destination register.
//

// [Rn] <op>= Rs

// Rt = [Rn] (load acquire); [Rn] <op>= Rs (store release)

// Rt = [Rn] (load acquire); [Rn] = Rs (store release)

// Rs = CAS(Rn, Rs, Rt) (load acquire & store release)

// Add/subtract (immediate)

// Rd = Rn OP imm12

// Rn + imm12; set condition flags

// Rn - imm12; set condition flags

// Rd = Rn

// Bitfield move

// Signed, with sign replication to left and zeros to right

// Unsigned, with zeros to left and right

// Rd = Rn << shift

// Rd = Rn >> shift

// Rd = Rn >> shift; signed

// Zero extend

// Sign extend

// Move wide (immediate)

// Rd = Zeros (for MOVZ);
// Rd |= imm16 << shift (where shift is {0, 16, 32, 48});
// Rd = ~Rd; (for MOVN);

// Add/subtract (shifted register)

// Rd = Rn OP Rm

// Rd = -Rm

// Rn - Rm; set condition flags

// Add/subtract (extended register)

// Rd = Rn + (EXT(Rm) << shift)

// Rd = Rn + (u32)Rm

// Data-processing (1 source)

// Rd = BSWAPx(Rn)

// Data-processing (2 source)
// Rd = Rn OP Rm

// Data-processing (3 source)
// Rd = Ra + Rn * Rm

// Rd = Ra - Rn * Rm

// Rd = Rn * Rm

// Logical (shifted register)

// Rd = Rn OP Rm

// Rn & Rm; set condition flags

// Rd = ~Rm (alias of ORN with A64_ZR as Rn)

// Logical (immediate)

// Rd = Rn OP imm

// Rn & imm; set condition flags

// HINTs

// BTI

// DMB

// ADR

// MRS

// Barriers

