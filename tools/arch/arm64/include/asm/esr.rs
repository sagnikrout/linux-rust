//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/arm64/include/asm/esr.h
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
// Copyright (C) 2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

// Unallocated EC: 0x02

// Unallocated EC: 0x0A - 0x0B

// Unallocated EC: 0x0F - 0x10

// Unallocated EC: 0x14

// Unallocated EC: 0x1B

// Unallocated EC: 0x1E

// Unallocated EC: 0x23

// Unallocated EC: 0x29 - 0x2B

// Unallocated EC: 0x2D - 0x2E

// Unallocated EC: 0x36 - 0x37

// Unallocated EC: 0x39

// Unallocated EC: 0x3B

// Unallocated EC: 0x3D - 0x3F

// ISS field definitions shared by different classes

// Asynchronous Error Type

// Shared ISS field definitions for Data/Instruction aborts

// Shared ISS fault status code(IFSC/DFSC) for Data/Instruction aborts

// Status codes for individual page table levels

// ISS field definitions for Data Aborts

// ISS2 field definitions for Data Aborts

// ISS field definitions for exceptions taken in to Hyp

//
// DISR_EL1 and ESR_ELx share the bottom 13 bits, but the RES0 bits may mean
// different things in the future...
//

// ESR value templates for specific events

// BRK instruction trap from AArch64 state
pub const ESR_ELx_BRK64_ISS_COMMENT_MASK: c_uint = 0xffff;
// ISS field definitions for System instruction traps
pub const ESR_ELx_SYS64_ISS_RES0_SHIFT: c_int = 22;

pub const ESR_ELx_SYS64_ISS_DIR_MASK: c_uint = 0x1;
pub const ESR_ELx_SYS64_ISS_DIR_READ: c_uint = 0x1;
pub const ESR_ELx_SYS64_ISS_DIR_WRITE: c_uint = 0x0;
pub const ESR_ELx_SYS64_ISS_RT_SHIFT: c_int = 5;

pub const ESR_ELx_SYS64_ISS_CRM_SHIFT: c_int = 1;

pub const ESR_ELx_SYS64_ISS_CRN_SHIFT: c_int = 10;

pub const ESR_ELx_SYS64_ISS_OP1_SHIFT: c_int = 14;

pub const ESR_ELx_SYS64_ISS_OP2_SHIFT: c_int = 17;

pub const ESR_ELx_SYS64_ISS_OP0_SHIFT: c_int = 20;

//
// User space cache operations have the following sysreg encoding
// in System instructions.
// op0=1, op1=3, op2=1, crn=7, crm={ 5, 10, 11, 12, 13, 14 }, WRITE (L=0)
//
pub const ESR_ELx_SYS64_ISS_CRM_DC_CIVAC: c_int = 14;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVADP: c_int = 13;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAP: c_int = 12;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAU: c_int = 11;
pub const ESR_ELx_SYS64_ISS_CRM_DC_CVAC: c_int = 10;
pub const ESR_ELx_SYS64_ISS_CRM_IC_IVAU: c_int = 5;

//
// User space MRS operations which are supported for emulation
// have the following sysreg encoding in System instructions.
// op0 = 3, op1= 0, crn = 0, {crm = 0, 4-7}, READ (L = 1)
//

// ISS field definitions for ERET/ERETAA/ERETAB trapping
pub const ESR_ELx_ERET_ISS_ERET: c_uint = 0x2;
pub const ESR_ELx_ERET_ISS_ERETA: c_uint = 0x1;
//
// ISS field definitions for floating-point exception traps
// (FP_EXC_32/FP_EXC_64).
//
// (The FPEXC_* constants are used instead for common bits.)
//

//
// ISS field definitions for CP15 accesses
//
pub const ESR_ELx_CP15_32_ISS_DIR_MASK: c_uint = 0x1;
pub const ESR_ELx_CP15_32_ISS_DIR_READ: c_uint = 0x1;
pub const ESR_ELx_CP15_32_ISS_DIR_WRITE: c_uint = 0x0;
pub const ESR_ELx_CP15_32_ISS_RT_SHIFT: c_int = 5;

pub const ESR_ELx_CP15_32_ISS_CRM_SHIFT: c_int = 1;

pub const ESR_ELx_CP15_32_ISS_CRN_SHIFT: c_int = 10;

pub const ESR_ELx_CP15_32_ISS_OP1_SHIFT: c_int = 14;

pub const ESR_ELx_CP15_32_ISS_OP2_SHIFT: c_int = 17;

pub const ESR_ELx_CP15_64_ISS_DIR_MASK: c_uint = 0x1;
pub const ESR_ELx_CP15_64_ISS_DIR_READ: c_uint = 0x1;
pub const ESR_ELx_CP15_64_ISS_DIR_WRITE: c_uint = 0x0;
pub const ESR_ELx_CP15_64_ISS_RT_SHIFT: c_int = 5;

pub const ESR_ELx_CP15_64_ISS_RT2_SHIFT: c_int = 10;

pub const ESR_ELx_CP15_64_ISS_OP1_SHIFT: c_int = 16;

pub const ESR_ELx_CP15_64_ISS_CRM_SHIFT: c_int = 1;

//
// ISS values for SME traps
//
pub const ESR_ELx_SME_ISS_SME_DISABLED: c_int = 0;
pub const ESR_ELx_SME_ISS_ILL: c_int = 1;
pub const ESR_ELx_SME_ISS_SM_DISABLED: c_int = 2;
pub const ESR_ELx_SME_ISS_ZA_DISABLED: c_int = 3;
pub const ESR_ELx_SME_ISS_ZT_DISABLED: c_int = 4;
// ISS field definitions for MOPS exceptions

// Indicate whether ESR.EC==0x1A is for an ERETAx instruction
// Indicate which key is used for ERETAx (false: A-Key, true: B-Key)

