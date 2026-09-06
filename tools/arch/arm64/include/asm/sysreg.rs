//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/arm64/include/asm/sysreg.h
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
// Macros for accessing system registers with older binutils.
//
// Copyright (C) 2014 ARM Ltd.
// Author: Catalin Marinas <catalin.marinas@arm.com>
//

//
// ARMv8 ARM reserves the following encoding for system registers:
// (Ref: ARMv8 ARM, Section: "System instruction class encoding overview",
// C5.2, version:ARM DDI 0487A.f)
// [20-19] : Op0
// [18-16] : Op1
// [15-12] : CRn
// [11-8]  : CRm
// [7-5]   : Op2
//
pub const Op0_shift: c_int = 19;
pub const Op0_mask: c_uint = 0x3;
pub const Op1_shift: c_int = 16;
pub const Op1_mask: c_uint = 0x7;
pub const CRn_shift: c_int = 12;
pub const CRn_mask: c_uint = 0xf;
pub const CRm_shift: c_int = 8;
pub const CRm_mask: c_uint = 0xf;
pub const Op2_shift: c_int = 5;
pub const Op2_mask: c_uint = 0x7;

// The space separator is omitted so that __emit_inst(x) can be parsed as
// either an assembler directive or an assembler macro argument.

//
// Instructions for modifying PSTATE fields.
// As per Arm ARM for v8-A, Section "C.5.1.3 op0 == 0b00, architectural hints,
// barriers and CLREX, and PSTATE access", ARM DDI 0487 C.a, system instructions
// for accessing PSTATE fields have the following encoding:
// Op0 = 0, CRn = 4
// Op1, Op2 encodes the PSTATE field modified and defines the constraints.
// CRm = Imm4 for the instruction.
// Rt = 0x1f
//

// Register-based PAN access, for save/restore purposes

// Data cache zero operations

//
// Automatically generated definitions for system registers, the
// manual encodings below are in the process of being converted to
// come from here. The header relies on the definition of sys_reg()
// earlier in this file.
//

//
// System registers, organised loosely by encoding but grouped together
// where the architected name contains an index. e.g. ID_MMFR<n>_EL1.
//

pub const OSLSR_EL1_OSLM_NI: c_int = 0;

// ETM

// When PAR_EL1.F == 1

// When PAR_EL1.F == 0

// Statistical Profiling Extension

// Buffer error reporting

pub const PMBSR_EL1_BUF_BSC_FULL: c_uint = 0x1UL;
// End of Statistical Profiling Extension

pub const TRBSR_EL1_BSC_SHIFT: c_int = 0;

// Definitions for system register interface to AMU for ARMv8.4 onwards

//
// Group 0 of activity monitors (architected):
// op0  op1  CRn   CRm       op2
// Counter:       11   011  1101  010:n<3>  n<2:0>
// Type:          11   011  1101  011:n<3>  n<2:0>
// n: 0-15
//
// Group 1 of activity monitors (auxiliary):
// op0  op1  CRn   CRm       op2
// Counter:       11   011  1101  110:n<3>  n<2:0>
// Type:          11   011  1101  111:n<3>  n<2:0>
// n: 0-15
//

// AMU v1: Fixed (architecturally defined) activity monitors

// VHE encodings for architectural EL0/1 system registers

// AT instructions
pub const AT_Op0: c_int = 1;
pub const AT_CRn: c_int = 7;

// TLBI instructions
pub const TLBI_Op0: c_int = 1;

// Misc instructions

// Common SCTLR_ELx flags.

pub const SCTLR_ELx_EE_SHIFT: c_int = 25;
pub const SCTLR_ELx_ENIA_SHIFT: c_int = 31;

pub const ENDIAN_SET_EL2: c_int = 0;

// SCTLR_EL1 specific flags.

pub const ENDIAN_SET_EL1: c_int = 0;

// MAIR_ELx memory attributes (used by Linux)

// Position the attr at the correct index

// id_aa64mmfr0
pub const ID_AA64MMFR0_EL1_TGRAN4_SUPPORTED_MIN: c_uint = 0x0;

pub const ID_AA64MMFR0_EL1_TGRAN4_SUPPORTED_MAX: c_uint = 0x7;
pub const ID_AA64MMFR0_EL1_TGRAN64_SUPPORTED_MIN: c_uint = 0x0;
pub const ID_AA64MMFR0_EL1_TGRAN64_SUPPORTED_MAX: c_uint = 0x7;
pub const ID_AA64MMFR0_EL1_TGRAN16_SUPPORTED_MIN: c_uint = 0x1;

pub const ID_AA64MMFR0_EL1_TGRAN16_SUPPORTED_MAX: c_uint = 0xf;
pub const ARM64_MIN_PARANGE_BITS: c_int = 32;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_DEFAULT: c_uint = 0x0;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_NONE: c_uint = 0x1;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_MIN: c_uint = 0x2;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_LPA2: c_uint = 0x3;
pub const ID_AA64MMFR0_EL1_TGRAN_2_SUPPORTED_MAX: c_uint = 0x7;

// GCR_EL1 Definitions

pub const SYS_GCR_EL1_EXCL_MASK: c_uint = 0xffffUL;

//
// KASAN always uses a whole byte for its tags. With CONFIG_KASAN_HW_TAGS it
// only uses tags in the range 0xF0-0xFF, which we map to MTE tags 0x0-0xF.
//

// RGSR_EL1 Definitions
pub const SYS_RGSR_EL1_TAG_MASK: c_uint = 0xfUL;
pub const SYS_RGSR_EL1_SEED_SHIFT: c_int = 8;
pub const SYS_RGSR_EL1_SEED_MASK: c_uint = 0xffffUL;
// TFSR{,E0}_EL1 bit definitions
pub const SYS_TFSR_EL1_TF0_SHIFT: c_int = 0;
pub const SYS_TFSR_EL1_TF1_SHIFT: c_int = 1;

// Safe value for MPIDR_EL1: Bit31:RES1, Bit30:U:0, Bit24:MT:0

// GIC Hypervisor interface registers
// ICH_LR*_EL2 bit definitions

pub const ICH_LR_PHYS_ID_SHIFT: c_int = 32;

pub const ICH_LR_PRIORITY_SHIFT: c_int = 48;

// ICH_VMCR_EL2 bit definitions
pub const ICH_VMCR_ACK_CTL_SHIFT: c_int = 2;

pub const ICH_VMCR_FIQ_EN_SHIFT: c_int = 3;

pub const ICH_VMCR_CBPR_SHIFT: c_int = 4;

pub const ICH_VMCR_EOIM_SHIFT: c_int = 9;

pub const ICH_VMCR_BPR1_SHIFT: c_int = 18;

pub const ICH_VMCR_BPR0_SHIFT: c_int = 21;

pub const ICH_VMCR_PMR_SHIFT: c_int = 24;

pub const ICH_VMCR_ENG0_SHIFT: c_int = 0;

pub const ICH_VMCR_ENG1_SHIFT: c_int = 1;

//
// Permission Indirection Extension (PIE) permission encodings.
// Encodings with the _O suffix, have overlays applied (Permission Overlay Extension).
//

pub const PIRx_ELx_BITS_PER_IDX: c_int = 4;

//
// Permission Overlay Extension (POE) permission encodings.
//

pub const POR_ELx_BITS_PER_IDX: c_int = 4;

//
// Definitions for Guarded Control Stack
//

pub const GCS_CAP_ADDR_SHIFT: c_int = 12;
pub const GCS_CAP_ADDR_WIDTH: c_int = 52;

pub const GCS_CAP_TOKEN_SHIFT: c_int = 0;
pub const GCS_CAP_TOKEN_WIDTH: c_int = 12;

pub const GCS_CAP_VALID_TOKEN: c_uint = 0x1;
pub const GCS_CAP_IN_PROGRESS_TOKEN: c_uint = 0x5;

//
// Unlike read_cpuid, calls to read_sysreg are never expected to be
// optimized away or replaced with synthetic values.
//

//
// The "Z" constraint normally means a zero immediate, but when combined with
// the "%x0" template means XZR.
//

//
// For registers without architectural names, or simply unsupported by
// GAS.
//
// __check_r forces warnings to be generated by the compiler when
// evaluating r which wouldn't normally happen due to being passed to
// the assembler via __stringify(r).
//

//
// Modify bits in a sysreg. Bits in the clear mask are zeroed, then bits in the
// set mask are set. Other bits are left as-is.
//

