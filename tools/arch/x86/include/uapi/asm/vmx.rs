//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/x86/include/uapi/asm/vmx.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// vmx.h: VMX Architecture related definitions
// Copyright (c) 2004, Intel Corporation.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// This program is distributed in the hope it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc., 59 Temple
// Place - Suite 330, Boston, MA 02111-1307 USA.
//
// A few random additions are:
// Copyright (C) 2006 Qumranet
// Avi Kivity <avi@qumranet.com>
// Yaniv Kamay <yaniv@qumranet.com>
//
pub const VMX_EXIT_REASONS_FAILED_VMENTRY: c_uint = 0x80000000;
pub const VMX_EXIT_REASONS_SGX_ENCLAVE_MODE: c_uint = 0x08000000;
pub const EXIT_REASON_EXCEPTION_NMI: c_int = 0;
pub const EXIT_REASON_EXTERNAL_INTERRUPT: c_int = 1;
pub const EXIT_REASON_TRIPLE_FAULT: c_int = 2;
pub const EXIT_REASON_INIT_SIGNAL: c_int = 3;
pub const EXIT_REASON_SIPI_SIGNAL: c_int = 4;
pub const EXIT_REASON_OTHER_SMI: c_int = 6;
pub const EXIT_REASON_INTERRUPT_WINDOW: c_int = 7;
pub const EXIT_REASON_NMI_WINDOW: c_int = 8;
pub const EXIT_REASON_TASK_SWITCH: c_int = 9;
pub const EXIT_REASON_CPUID: c_int = 10;
pub const EXIT_REASON_HLT: c_int = 12;
pub const EXIT_REASON_INVD: c_int = 13;
pub const EXIT_REASON_INVLPG: c_int = 14;
pub const EXIT_REASON_RDPMC: c_int = 15;
pub const EXIT_REASON_RDTSC: c_int = 16;
pub const EXIT_REASON_VMCALL: c_int = 18;
pub const EXIT_REASON_VMCLEAR: c_int = 19;
pub const EXIT_REASON_VMLAUNCH: c_int = 20;
pub const EXIT_REASON_VMPTRLD: c_int = 21;
pub const EXIT_REASON_VMPTRST: c_int = 22;
pub const EXIT_REASON_VMREAD: c_int = 23;
pub const EXIT_REASON_VMRESUME: c_int = 24;
pub const EXIT_REASON_VMWRITE: c_int = 25;
pub const EXIT_REASON_VMOFF: c_int = 26;
pub const EXIT_REASON_VMON: c_int = 27;
pub const EXIT_REASON_CR_ACCESS: c_int = 28;
pub const EXIT_REASON_DR_ACCESS: c_int = 29;
pub const EXIT_REASON_IO_INSTRUCTION: c_int = 30;
pub const EXIT_REASON_MSR_READ: c_int = 31;
pub const EXIT_REASON_MSR_WRITE: c_int = 32;
pub const EXIT_REASON_INVALID_STATE: c_int = 33;
pub const EXIT_REASON_MSR_LOAD_FAIL: c_int = 34;
pub const EXIT_REASON_MWAIT_INSTRUCTION: c_int = 36;
pub const EXIT_REASON_MONITOR_TRAP_FLAG: c_int = 37;
pub const EXIT_REASON_MONITOR_INSTRUCTION: c_int = 39;
pub const EXIT_REASON_PAUSE_INSTRUCTION: c_int = 40;
pub const EXIT_REASON_MCE_DURING_VMENTRY: c_int = 41;
pub const EXIT_REASON_TPR_BELOW_THRESHOLD: c_int = 43;
pub const EXIT_REASON_APIC_ACCESS: c_int = 44;
pub const EXIT_REASON_EOI_INDUCED: c_int = 45;
pub const EXIT_REASON_GDTR_IDTR: c_int = 46;
pub const EXIT_REASON_LDTR_TR: c_int = 47;
pub const EXIT_REASON_EPT_VIOLATION: c_int = 48;
pub const EXIT_REASON_EPT_MISCONFIG: c_int = 49;
pub const EXIT_REASON_INVEPT: c_int = 50;
pub const EXIT_REASON_RDTSCP: c_int = 51;
pub const EXIT_REASON_PREEMPTION_TIMER: c_int = 52;
pub const EXIT_REASON_INVVPID: c_int = 53;
pub const EXIT_REASON_WBINVD: c_int = 54;
pub const EXIT_REASON_XSETBV: c_int = 55;
pub const EXIT_REASON_APIC_WRITE: c_int = 56;
pub const EXIT_REASON_RDRAND: c_int = 57;
pub const EXIT_REASON_INVPCID: c_int = 58;
pub const EXIT_REASON_VMFUNC: c_int = 59;
pub const EXIT_REASON_ENCLS: c_int = 60;
pub const EXIT_REASON_RDSEED: c_int = 61;
pub const EXIT_REASON_PML_FULL: c_int = 62;
pub const EXIT_REASON_XSAVES: c_int = 63;
pub const EXIT_REASON_XRSTORS: c_int = 64;
pub const EXIT_REASON_UMWAIT: c_int = 67;
pub const EXIT_REASON_TPAUSE: c_int = 68;
pub const EXIT_REASON_BUS_LOCK: c_int = 74;
pub const EXIT_REASON_NOTIFY: c_int = 75;
pub const EXIT_REASON_SEAMCALL: c_int = 76;
pub const EXIT_REASON_TDCALL: c_int = 77;
pub const EXIT_REASON_MSR_READ_IMM: c_int = 84;
pub const EXIT_REASON_MSR_WRITE_IMM: c_int = 85;

pub const VMX_ABORT_SAVE_GUEST_MSR_FAIL: c_int = 1;
pub const VMX_ABORT_LOAD_HOST_PDPTE_FAIL: c_int = 2;
pub const VMX_ABORT_LOAD_HOST_MSR_FAIL: c_int = 4;
