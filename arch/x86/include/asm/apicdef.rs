//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/apicdef.h
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
// Constants for various Intel APICs. (local APIC, IOAPIC, etc.)
//
// Alan Cox <Alan.Cox@linux.org>, 1995.
// Ingo Molnar <mingo@redhat.com>, 1999, 2000
//
pub const IO_APIC_DEFAULT_PHYS_BASE: c_uint = 0xfec00000;
pub const APIC_DEFAULT_PHYS_BASE: c_uint = 0xfee00000;
//
// This is the IO-APIC register space as specified
// by Intel docs:
//
pub const IO_APIC_SLOT_SIZE: c_int = 1024;
pub const APIC_DELIVERY_MODE_FIXED: c_int = 0;
pub const APIC_DELIVERY_MODE_LOWESTPRIO: c_int = 1;
pub const APIC_DELIVERY_MODE_SMI: c_int = 2;
pub const APIC_DELIVERY_MODE_NMI: c_int = 4;
pub const APIC_DELIVERY_MODE_INIT: c_int = 5;
pub const APIC_DELIVERY_MODE_EXTINT: c_int = 7;
pub const APIC_ID: c_uint = 0x20;
pub const APIC_LVR: c_uint = 0x30;
pub const APIC_LVR_MASK: c_uint = 0xFF00FF;

pub const APIC_TASKPRI: c_uint = 0x80;
pub const APIC_TPRI_MASK: c_uint = 0xFFu;
pub const APIC_ARBPRI: c_uint = 0x90;
pub const APIC_ARBPRI_MASK: c_uint = 0xFFu;
pub const APIC_PROCPRI: c_uint = 0xA0;
pub const APIC_EOI: c_uint = 0xB0;
pub const APIC_EOI_ACK: c_uint = 0x0 /* Docs say 0 for future compat. */;
pub const APIC_RRR: c_uint = 0xC0;
pub const APIC_LDR: c_uint = 0xD0;

pub const APIC_ALL_CPUS: c_uint = 0xFFu;
pub const APIC_DFR: c_uint = 0xE0;
pub const APIC_DFR_CLUSTER: c_uint = 0x0FFFFFFFul;
pub const APIC_DFR_FLAT: c_uint = 0xFFFFFFFFul;
pub const APIC_SPIV: c_uint = 0xF0;

pub const APIC_ISR: c_uint = 0x100;
pub const APIC_ISR_NR: c_uint = 0x8     /* Number of 32 bit ISR registers. */;
pub const APIC_TMR: c_uint = 0x180;
pub const APIC_IRR: c_uint = 0x200;
pub const APIC_ESR: c_uint = 0x280;
pub const APIC_ESR_SEND_CS: c_uint = 0x00001;
pub const APIC_ESR_RECV_CS: c_uint = 0x00002;
pub const APIC_ESR_SEND_ACC: c_uint = 0x00004;
pub const APIC_ESR_RECV_ACC: c_uint = 0x00008;
pub const APIC_ESR_SENDILL: c_uint = 0x00020;
pub const APIC_ESR_RECVILL: c_uint = 0x00040;
pub const APIC_ESR_ILLREGA: c_uint = 0x00080;
pub const APIC_LVTCMCI: c_uint = 0x2f0;
pub const APIC_ICR: c_uint = 0x300;
pub const APIC_DEST_SELF: c_uint = 0x40000;
pub const APIC_DEST_ALLINC: c_uint = 0x80000;
pub const APIC_DEST_ALLBUT: c_uint = 0xC0000;
pub const APIC_ICR_RR_MASK: c_uint = 0x30000;
pub const APIC_ICR_RR_INVALID: c_uint = 0x00000;
pub const APIC_ICR_RR_INPROG: c_uint = 0x10000;
pub const APIC_ICR_RR_VALID: c_uint = 0x20000;
pub const APIC_INT_LEVELTRIG: c_uint = 0x08000;
pub const APIC_INT_ASSERT: c_uint = 0x04000;
pub const APIC_ICR_BUSY: c_uint = 0x01000;
pub const APIC_DEST_LOGICAL: c_uint = 0x00800;
pub const APIC_DEST_PHYSICAL: c_uint = 0x00000;
pub const APIC_DM_FIXED: c_uint = 0x00000;
pub const APIC_DM_FIXED_MASK: c_uint = 0x00700;
pub const APIC_DM_LOWEST: c_uint = 0x00100;
pub const APIC_DM_SMI: c_uint = 0x00200;
pub const APIC_DM_REMRD: c_uint = 0x00300;
pub const APIC_DM_NMI: c_uint = 0x00400;
pub const APIC_DM_INIT: c_uint = 0x00500;
pub const APIC_DM_STARTUP: c_uint = 0x00600;
pub const APIC_DM_EXTINT: c_uint = 0x00700;
pub const APIC_VECTOR_MASK: c_uint = 0x000FF;
pub const APIC_ICR2: c_uint = 0x310;

pub const APIC_LVTT: c_uint = 0x320;
pub const APIC_LVTTHMR: c_uint = 0x330;
pub const APIC_LVTPC: c_uint = 0x340;
pub const APIC_LVT0: c_uint = 0x350;

pub const APIC_MODE_MASK: c_uint = 0x700;

pub const APIC_MODE_FIXED: c_uint = 0x0;
pub const APIC_MODE_NMI: c_uint = 0x4;
pub const APIC_MODE_EXTINT: c_uint = 0x7;
pub const APIC_LVT1: c_uint = 0x360;
pub const APIC_LVTERR: c_uint = 0x370;
pub const APIC_TMICT: c_uint = 0x380;
pub const APIC_TMCCT: c_uint = 0x390;
pub const APIC_TDCR: c_uint = 0x3E0;
pub const APIC_SELF_IPI: c_uint = 0x3F0;

pub const APIC_TDR_DIV_1: c_uint = 0xB;
pub const APIC_TDR_DIV_2: c_uint = 0x0;
pub const APIC_TDR_DIV_4: c_uint = 0x1;
pub const APIC_TDR_DIV_8: c_uint = 0x2;
pub const APIC_TDR_DIV_16: c_uint = 0x3;
pub const APIC_TDR_DIV_32: c_uint = 0x8;
pub const APIC_TDR_DIV_64: c_uint = 0x9;
pub const APIC_TDR_DIV_128: c_uint = 0xA;
pub const APIC_EFEAT: c_uint = 0x400;
pub const APIC_ECTRL: c_uint = 0x410;
pub const APIC_SEOI: c_uint = 0x420;
pub const APIC_IER: c_uint = 0x480;

pub const APIC_EILVT_NR_AMD_10H: c_int = 4;

pub const APIC_BASE_MSR: c_uint = 0x800;
pub const APIC_X2APIC_ID_MSR: c_uint = 0x802;

//
// All x86-64 systems are xAPIC compatible.
// In the following, "apicid" is a physical APIC ID.
//
pub const XAPIC_DEST_CPUS_SHIFT: c_int = 4;

pub const BAD_APICID: c_uint = 0xFFu;

pub const BAD_APICID: c_uint = 0xFFFFu;

