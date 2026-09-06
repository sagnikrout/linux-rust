//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/randomize_kstack.h
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
// Do not use this anywhere else in the kernel. This is used here because
// it provides an arch-agnostic way to grow the stack with correct
// alignment. Also, since this use is being explicitly masked to a max of
// 10 bits, stack-clash style attacks are unlikely. For more details see
// "VLAs" in Documentation/process/deprecated.rst
//
// The normal __builtin_alloca() is initialized with INIT_STACK_ALL (currently
// only with Clang and not GCC). Initializing the unused area on each syscall
// entry is expensive, and generating an implicit call to memset() may also be
// problematic (such as in noinstr functions). Therefore, if the compiler
// supports it (which it should if it initializes allocas), always use the
// "uninitialized" variant of the builtin.
//

//
// Use, at most, 6 bits of entropy (on 64-bit; 8 on 32-bit). This cap is
// to keep the "VLA" from being unbounded (see above). Additionally clear
// the bottom 4 bits (on 64-bit systems, 2 for 32-bit), since stack
// alignment will always be at least word size. This makes the compiler
// code gen better when it is applying the actual per-arch alignment to
// the final offset. The resulting randomness is reasonable without overly
// constraining usable stack space.
//

//
// add_random_kstack_offset - Increase stack utilization by a random offset.
//
// This should be used in the syscall entry path after user registers have been
// stored to the stack. Preemption may be enabled. For testing the resulting
// entropy, please see: tools/testing/selftests/lkdtm/stack-entropy.sh
//

// Keep allocation even after "ptr" loses scope. */	\
//
// add_random_kstack_offset_irqsoff - Increase stack utilization by a random offset.
//
// This should be used in the syscall entry path after user registers have been
// stored to the stack. Interrupts must be still disabled.
//

// Keep allocation even after "ptr" loses scope. */		\

