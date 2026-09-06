//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/kvm-stat-arch/riscv_trap_types.h
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
// Exception cause high bit - is an interrupt if set

// Interrupt causes (minus the high bit)
pub const IRQ_S_SOFT: c_int = 1;
pub const IRQ_VS_SOFT: c_int = 2;
pub const IRQ_M_SOFT: c_int = 3;
pub const IRQ_S_TIMER: c_int = 5;
pub const IRQ_VS_TIMER: c_int = 6;
pub const IRQ_M_TIMER: c_int = 7;
pub const IRQ_S_EXT: c_int = 9;
pub const IRQ_VS_EXT: c_int = 10;
pub const IRQ_M_EXT: c_int = 11;
pub const IRQ_S_GEXT: c_int = 12;
pub const IRQ_PMU_OVF: c_int = 13;
// Exception causes
pub const EXC_INST_MISALIGNED: c_int = 0;
pub const EXC_INST_ACCESS: c_int = 1;
pub const EXC_INST_ILLEGAL: c_int = 2;
pub const EXC_BREAKPOINT: c_int = 3;
pub const EXC_LOAD_MISALIGNED: c_int = 4;
pub const EXC_LOAD_ACCESS: c_int = 5;
pub const EXC_STORE_MISALIGNED: c_int = 6;
pub const EXC_STORE_ACCESS: c_int = 7;
pub const EXC_SYSCALL: c_int = 8;
pub const EXC_HYPERVISOR_SYSCALL: c_int = 9;
pub const EXC_SUPERVISOR_SYSCALL: c_int = 10;
pub const EXC_INST_PAGE_FAULT: c_int = 12;
pub const EXC_LOAD_PAGE_FAULT: c_int = 13;
pub const EXC_STORE_PAGE_FAULT: c_int = 15;
pub const EXC_INST_GUEST_PAGE_FAULT: c_int = 20;
pub const EXC_LOAD_GUEST_PAGE_FAULT: c_int = 21;
pub const EXC_VIRTUAL_INST_FAULT: c_int = 22;
pub const EXC_STORE_GUEST_PAGE_FAULT: c_int = 23;

