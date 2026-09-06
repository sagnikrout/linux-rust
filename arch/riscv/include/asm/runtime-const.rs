//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/runtime-const.h
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
// Loading 64-bit constants into a register from immediates is a non-trivial
// task on riscv64. To get it somewhat performant, load 32 bits into two
// different registers and then combine the results.
//
// If the processor supports the Zbkb extension, we can combine the final
// "slli,slli,srli,add" into the single "pack" instruction. If the processor
// doesn't support Zbkb but does support the Zbb extension, we can
// combine the final "slli,srli,add" into one instruction "add.uw".
//

// On riscv there are currently only cache-wide flushes so va is ignored.
//
// The 32-bit immediate is stored in a lui+addi pairing.
// lui holds the upper 20 bits of the immediate in the first 20 bits of the instruction.
// addi holds the lower 12 bits of the immediate in the first 12 bits of the instruction.
//
// Mask out upper 12 bit of addi
// replace upper 20 bits of lui with upper immediate
// replace lui with nop if immediate is small enough to fit in addi
//
// lui is being skipped, so do a load instead of an add. A load
// is performed by adding with the x0 register. Setting rs to
// zero with the following mask will accomplish this goal.
//
// replace upper 12 bits of addi with lower 12 bits of val
// replace addi with nop if lower_immediate is empty

//
// Replace the least significant 5 bits of the srli/srliw immediate that is
// located at bits 20-24
//
// XXX: Current implementation only supports patching masks of
// form GENMASK(width, 0) (width >= 0) using a SRLI + SLLI
// sequence instead of LUI + ADDI + AND sequence to improve
// performance, density, and covers all the current use-cases.
//
// When the need arises to support any generic mask, and this
// BUG_ON() is tripped, consider using a:
//
// lui  %[__ret], #imm16
// addi %[__ret], #imm16
//
// sequence to load the 32bit const mask, and perform a logical
// and outside the asm block before returning the result. Fixup
// can simply reuse the existing __runtime_fixup_32() to patch
// the LUI + ADDI sequence.
//
