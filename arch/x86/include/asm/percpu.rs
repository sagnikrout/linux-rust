//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/percpu.h
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

// Macro flag: #define __percpu_prefix

//
// Compared to the generic __my_cpu_offset version, the following
// saves one instruction and avoids clobbering a temp register.
//

//
// arch_raw_cpu_ptr should not be used in 32-bit VDSO for a 64-bit
// kernel, because games are played with CONFIG_X86_64 there and
// sizeof(this_cpu_off) becames 4.
//

// Macro flag: #define _percpu_prefix
// Macro flag: #define __percpu_prefix

//
// For arch-specific code, we can use direct single-insn ops (they
// don't give an lvalue though).
//

// (qual __my_cpu_type(pcp) * )__my_cpu_ptr(&(pcp));	\

// (qual __my_cpu_type(pcp) * )__my_cpu_ptr(&(pcp)) = (val);	\

//
// The generic per-CPU infrastrucutre is not suitable for
// reading const-qualified variables.
//

//
// Generate a per-CPU add to memory instruction and optimize code
// if one is added or subtracted.
//

//
// Add return operation
//

//
// raw_cpu_xchg() can use a load-store since
// it is not required to be IRQ-safe.
//

//
// this_cpu_xchg() is implemented using CMPXCHG without a LOCK prefix.
// XCHG is expensive due to the implied LOCK prefix. The processor
// cannot prefetch cachelines if XCHG is used.
//

//
// CMPXCHG has no such implied lock semantics as a result it is much
// more efficient for CPU-local operations.
//

// pco_oval__ = pco_old__;				\

// _oval = old__.var;					\

// _oval = old__.var;					\

//
// Per-CPU atomic 64-bit operations are only available under 64-bit kernels.
// 32-bit kernels must fall back to generic operations.
//

// There is no generic 64-bit read stable operation for 32-bit targets.

//
// this_cpu_read() makes the compiler load the per-CPU variable every time
// it is accessed while this_cpu_read_stable() allows the value to be cached.
// this_cpu_read_stable() is more efficient and can be used if its value
// is guaranteed to be valid across CPUs.  The current users include
// current_task and cpu_current_top_of_stack, both of which are
// actually per-thread variables implemented as per-CPU variables and
// thus stable for the duration of the respective task.
//

// We can use this directly for local CPU (faster).

//
// Define the "EARLY_PER_CPU" macros.  These are used for some per_cpu
// variables that are initialized and accessed before there are per_cpu
// areas allocated.
//

// (early_per_cpu_ptr(_name) ?					\

// no early_per_cpu_map()

