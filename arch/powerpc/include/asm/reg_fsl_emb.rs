//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/reg_fsl_emb.h
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
// Contains register definitions for the Freescale Embedded Performance
// Monitor.
//

// Performance Monitor Registers

// Freescale Book E Performance Monitor APU Registers
pub const PMRN_PMC0: c_uint = 0x010	/* Performance Monitor Counter 0 */;
pub const PMRN_PMC1: c_uint = 0x011	/* Performance Monitor Counter 1 */;
pub const PMRN_PMC2: c_uint = 0x012	/* Performance Monitor Counter 2 */;
pub const PMRN_PMC3: c_uint = 0x013	/* Performance Monitor Counter 3 */;
pub const PMRN_PMC4: c_uint = 0x014	/* Performance Monitor Counter 4 */;
pub const PMRN_PMC5: c_uint = 0x015	/* Performance Monitor Counter 5 */;
pub const PMRN_PMLCA0: c_uint = 0x090	/* PM Local Control A0 */;
pub const PMRN_PMLCA1: c_uint = 0x091	/* PM Local Control A1 */;
pub const PMRN_PMLCA2: c_uint = 0x092	/* PM Local Control A2 */;
pub const PMRN_PMLCA3: c_uint = 0x093	/* PM Local Control A3 */;
pub const PMRN_PMLCA4: c_uint = 0x094	/* PM Local Control A4 */;
pub const PMRN_PMLCA5: c_uint = 0x095	/* PM Local Control A5 */;
pub const PMLCA_FC: c_uint = 0x80000000	/* Freeze Counter */;
pub const PMLCA_FCS: c_uint = 0x40000000	/* Freeze in Supervisor */;
pub const PMLCA_FCU: c_uint = 0x20000000	/* Freeze in User */;
pub const PMLCA_FCM1: c_uint = 0x10000000	/* Freeze when PMM==1 */;
pub const PMLCA_FCM0: c_uint = 0x08000000	/* Freeze when PMM==0 */;
pub const PMLCA_CE: c_uint = 0x04000000	/* Condition Enable */;
pub const PMLCA_FGCS1: c_uint = 0x00000002	/* Freeze in guest state */;
pub const PMLCA_FGCS0: c_uint = 0x00000001	/* Freeze in hypervisor state */;
pub const PMLCA_EVENT_MASK: c_uint = 0x01ff0000	/* Event field */;
pub const PMLCA_EVENT_SHIFT: c_int = 16;
pub const PMRN_PMLCB0: c_uint = 0x110	/* PM Local Control B0 */;
pub const PMRN_PMLCB1: c_uint = 0x111	/* PM Local Control B1 */;
pub const PMRN_PMLCB2: c_uint = 0x112	/* PM Local Control B2 */;
pub const PMRN_PMLCB3: c_uint = 0x113	/* PM Local Control B3 */;
pub const PMRN_PMLCB4: c_uint = 0x114	/* PM Local Control B4 */;
pub const PMRN_PMLCB5: c_uint = 0x115	/* PM Local Control B5 */;
pub const PMLCB_THRESHMUL_MASK: c_uint = 0x0700	/* Threshold Multiple Field */;
pub const PMLCB_THRESHMUL_SHIFT: c_int = 8;
pub const PMLCB_THRESHOLD_MASK: c_uint = 0x003f	/* Threshold Field */;
pub const PMLCB_THRESHOLD_SHIFT: c_int = 0;
pub const PMRN_PMGC0: c_uint = 0x190	/* PM Global Control 0 */;
pub const PMGC0_FAC: c_uint = 0x80000000	/* Freeze all Counters */;
pub const PMGC0_PMIE: c_uint = 0x40000000	/* Interrupt Enable */;
pub const PMGC0_FCECE: c_uint = 0x20000000	/* Freeze countes on;
pub const PMRN_UPMC0: c_uint = 0x000	/* User Performance Monitor Counter 0 */;
pub const PMRN_UPMC1: c_uint = 0x001	/* User Performance Monitor Counter 1 */;
pub const PMRN_UPMC2: c_uint = 0x002	/* User Performance Monitor Counter 2 */;
pub const PMRN_UPMC3: c_uint = 0x003	/* User Performance Monitor Counter 3 */;
pub const PMRN_UPMC4: c_uint = 0x004	/* User Performance Monitor Counter 4 */;
pub const PMRN_UPMC5: c_uint = 0x005	/* User Performance Monitor Counter 5 */;
pub const PMRN_UPMLCA0: c_uint = 0x080	/* User PM Local Control A0 */;
pub const PMRN_UPMLCA1: c_uint = 0x081	/* User PM Local Control A1 */;
pub const PMRN_UPMLCA2: c_uint = 0x082	/* User PM Local Control A2 */;
pub const PMRN_UPMLCA3: c_uint = 0x083	/* User PM Local Control A3 */;
pub const PMRN_UPMLCA4: c_uint = 0x084	/* User PM Local Control A4 */;
pub const PMRN_UPMLCA5: c_uint = 0x085	/* User PM Local Control A5 */;
pub const PMRN_UPMLCB0: c_uint = 0x100	/* User PM Local Control B0 */;
pub const PMRN_UPMLCB1: c_uint = 0x101	/* User PM Local Control B1 */;
pub const PMRN_UPMLCB2: c_uint = 0x102	/* User PM Local Control B2 */;
pub const PMRN_UPMLCB3: c_uint = 0x103	/* User PM Local Control B3 */;
pub const PMRN_UPMLCB4: c_uint = 0x104	/* User PM Local Control B4 */;
pub const PMRN_UPMLCB5: c_uint = 0x105	/* User PM Local Control B5 */;
pub const PMRN_UPMGC0: c_uint = 0x180	/* User PM Global Control 0 */;

