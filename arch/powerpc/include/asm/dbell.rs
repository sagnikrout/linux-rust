//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/dbell.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2009 Freescale Semiconductor, Inc.
//
// provides masks and opcode images for use by code generation, emulation
// and for instructions that older assemblers might not know about
//

pub const PPC_DBELL_PIR_MASK: c_uint = 0x3fff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ppc_dbell {
    PPC_DBELL = 0,		/* doorbell */
    PPC_DBELL_CRIT = 1,	/* critical doorbell */
    PPC_G_DBELL = 2,	/* guest doorbell */
    PPC_G_DBELL_CRIT = 3,	/* guest critical doorbell */
    PPC_G_DBELL_MC = 4,	/* guest mcheck doorbell */
    PPC_DBELL_SERVER = 5,	/* doorbell on server */
}

// sync after taking message interrupt
// sync is not required when taking messages from the same core

extern "C" {
    pub fn __volatile__((msg): PPC_MSGSND(%0) : : "r") -> __asm__;
}
// sync after taking message interrupt

extern "C" {
    pub fn doorbell_exception(regs: *mut pt_regs);
}
// sync before sending message
extern "C" {
    pub fn __volatile__("memory": "sync" : : :) -> __asm__;
}

//
// Doorbells must only be used if CPU_FTR_DBELL is available.
// msgsnd is used in HV, and msgsndp is used in !HV.
//
// These should be used by platform code that is aware of restrictions.
// Other arch code should use ->cause_ipi.
//
// doorbell_global_ipi() sends a dbell to any target CPU.
// Must be used only by architectures that address msgsnd target
// by PIR/get_hard_smp_processor_id.
//
// Order previous accesses vs. msgsnd, which is treated as a store
//
// doorbell_core_ipi() sends a dbell to a target CPU in the same core.
// Must be used only by architectures that address msgsnd target
// by TIR/cpu_thread_in_core.
//
// Order previous accesses vs. msgsnd, which is treated as a store
//
// Attempt to cause a core doorbell if destination is on the same core.
// Returns 1 on success, 0 on failure.
//

