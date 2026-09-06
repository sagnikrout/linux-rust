//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/qos.h
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

// cached value of srmcfg csr for each cpu
// default srmcfg value for each cpu, set via resctrl cpu assignment
//
// RCID and MCID inherit from cpu_srmcfg_default independently.
// RESCTRL_RESERVED_CLOSID and RESCTRL_RESERVED_RMID are both 0, so a
// zero field means "unassigned" and takes the CPU default.
//
// No fence around the csrw. Ssqosid is silent on srmcfg
// ordering versus memory accesses, so a few accesses at the
// switch boundary may carry the previous RCID/MCID. The
// tagging inaccuracy is bounded and acceptable for QoS.
//
extern "C" {
    pub fn riscv_has_extension_unlikely(_arg: RISCV_ISA_EXT_SSQOSID) -> return;
}

