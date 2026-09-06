//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/percpu.h
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
// s390 uses its own implementation for per cpu data, the offset of
// the cpu local data area is cached in the cpu's lowcore memory.
//

//
// We use a compare-and-swap loop since that uses less cpu cycles than
// disabling and enabling interrupts like the generic variant would do.
//

//
// Macros to be used for percpu code section based on atomic instructions.
//
// Avoid the need to use preempt_disable() / preempt_disable() pairs and the
// conditional preempt_schedule_notrace() function calls which come with
// this. The idea is that this_cpu operations based on atomic instructions are
// guarded with mviy instructions:
//
// - The first mviy instruction writes the register number, which contains the
// percpu address variable to lowcore. This also indicates that a percpu
// code section is executed.
//
// - The first mviy instruction following the mviy instruction must be the ag
// instruction which adds the percpu offset to the percpu address register.
//
// - Afterwards the atomic percpu operation follows.
//
// - Then a second mviy instruction writes a zero to lowcore, which indicates
// the end of the percpu code section.
//
// - In case of an interrupt/exception/nmi the register number which was
// written to lowcore is copied to the exception frame (pt_regs), and a zero
// is written to lowcore.
//
// - On return to the previous context it is checked if a percpu code section
// was executed (saved register number not zero), and if the process was
// migrated to a different cpu. If the percpu offset was already added to
// the percpu address register (instruction address does _not_ point to the
// ag instruction) the content of the percpu address register is adjusted so
// it points to percpu variable of the new cpu.
//
// Inline assemblies making use of this typically have a code sequence like:
//
// MVIY_PERCPU(...) <- start of percpu code section
// AG_ALT(...)      <- add percpu offset; must be the second instruction
// atomic_op	      <- atomic op
// MVIY_ALT(...)    <- end of percpu code section
//

