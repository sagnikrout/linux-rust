//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/vncr_mapping.h
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
// System register offsets in the VNCR page
// All offsets are *byte* displacements!
//
pub const VNCR_VTTBR_EL2: c_uint = 0x020;
pub const VNCR_VTCR_EL2: c_uint = 0x040;
pub const VNCR_VMPIDR_EL2: c_uint = 0x050;
pub const VNCR_CNTVOFF_EL2: c_uint = 0x060;
pub const VNCR_NVHCR_EL2: c_uint = 0x078;
pub const VNCR_HSTR_EL2: c_uint = 0x080;
pub const VNCR_VPIDR_EL2: c_uint = 0x088;
pub const VNCR_TPIDR_EL2: c_uint = 0x090;
pub const VNCR_HCRX_EL2: c_uint = 0x0A0;
pub const VNCR_VNCR_EL2: c_uint = 0x0B0;
pub const VNCR_CPACR_EL1: c_uint = 0x100;
pub const VNCR_CONTEXTIDR_EL1: c_uint = 0x108;
pub const VNCR_SCTLR_EL1: c_uint = 0x110;
pub const VNCR_ACTLR_EL1: c_uint = 0x118;
pub const VNCR_TCR_EL1: c_uint = 0x120;
pub const VNCR_AFSR0_EL1: c_uint = 0x128;
pub const VNCR_AFSR1_EL1: c_uint = 0x130;
pub const VNCR_ESR_EL1: c_uint = 0x138;
pub const VNCR_MAIR_EL1: c_uint = 0x140;
pub const VNCR_AMAIR_EL1: c_uint = 0x148;
pub const VNCR_MDSCR_EL1: c_uint = 0x158;
pub const VNCR_SPSR_EL1: c_uint = 0x160;
pub const VNCR_CNTV_CVAL_EL0: c_uint = 0x168;
pub const VNCR_CNTV_CTL_EL0: c_uint = 0x170;
pub const VNCR_CNTP_CVAL_EL0: c_uint = 0x178;
pub const VNCR_CNTP_CTL_EL0: c_uint = 0x180;
pub const VNCR_SCXTNUM_EL1: c_uint = 0x188;
pub const VNCR_TFSR_EL1: c_uint = 0x190;
pub const VNCR_HDFGRTR2_EL2: c_uint = 0x1A0;
pub const VNCR_HDFGWTR2_EL2: c_uint = 0x1B0;
pub const VNCR_HFGRTR_EL2: c_uint = 0x1B8;
pub const VNCR_HFGWTR_EL2: c_uint = 0x1C0;
pub const VNCR_HFGITR_EL2: c_uint = 0x1C8;
pub const VNCR_HDFGRTR_EL2: c_uint = 0x1D0;
pub const VNCR_HDFGWTR_EL2: c_uint = 0x1D8;
pub const VNCR_ZCR_EL1: c_uint = 0x1E0;
pub const VNCR_HAFGRTR_EL2: c_uint = 0x1E8;
pub const VNCR_TTBR0_EL1: c_uint = 0x200;
pub const VNCR_TTBR1_EL1: c_uint = 0x210;
pub const VNCR_FAR_EL1: c_uint = 0x220;
pub const VNCR_ELR_EL1: c_uint = 0x230;
pub const VNCR_SP_EL1: c_uint = 0x240;
pub const VNCR_VBAR_EL1: c_uint = 0x250;
pub const VNCR_TCR2_EL1: c_uint = 0x270;
pub const VNCR_SCTLR2_EL1: c_uint = 0x278;
pub const VNCR_PIRE0_EL1: c_uint = 0x290;
pub const VNCR_PIR_EL1: c_uint = 0x2A0;
pub const VNCR_POR_EL1: c_uint = 0x2A8;
pub const VNCR_HFGRTR2_EL2: c_uint = 0x2C0;
pub const VNCR_HFGWTR2_EL2: c_uint = 0x2C8;
pub const VNCR_HFGITR2_EL2: c_uint = 0x310;
pub const VNCR_ICH_LR0_EL2: c_uint = 0x400;
pub const VNCR_ICH_LR1_EL2: c_uint = 0x408;
pub const VNCR_ICH_LR2_EL2: c_uint = 0x410;
pub const VNCR_ICH_LR3_EL2: c_uint = 0x418;
pub const VNCR_ICH_LR4_EL2: c_uint = 0x420;
pub const VNCR_ICH_LR5_EL2: c_uint = 0x428;
pub const VNCR_ICH_LR6_EL2: c_uint = 0x430;
pub const VNCR_ICH_LR7_EL2: c_uint = 0x438;
pub const VNCR_ICH_LR8_EL2: c_uint = 0x440;
pub const VNCR_ICH_LR9_EL2: c_uint = 0x448;
pub const VNCR_ICH_LR10_EL2: c_uint = 0x450;
pub const VNCR_ICH_LR11_EL2: c_uint = 0x458;
pub const VNCR_ICH_LR12_EL2: c_uint = 0x460;
pub const VNCR_ICH_LR13_EL2: c_uint = 0x468;
pub const VNCR_ICH_LR14_EL2: c_uint = 0x470;
pub const VNCR_ICH_LR15_EL2: c_uint = 0x478;
pub const VNCR_ICH_AP0R0_EL2: c_uint = 0x480;
pub const VNCR_ICH_AP0R1_EL2: c_uint = 0x488;
pub const VNCR_ICH_AP0R2_EL2: c_uint = 0x490;
pub const VNCR_ICH_AP0R3_EL2: c_uint = 0x498;
pub const VNCR_ICH_AP1R0_EL2: c_uint = 0x4A0;
pub const VNCR_ICH_AP1R1_EL2: c_uint = 0x4A8;
pub const VNCR_ICH_AP1R2_EL2: c_uint = 0x4B0;
pub const VNCR_ICH_AP1R3_EL2: c_uint = 0x4B8;
pub const VNCR_ICH_HCR_EL2: c_uint = 0x4C0;
pub const VNCR_ICH_VMCR_EL2: c_uint = 0x4C8;
pub const VNCR_VDISR_EL2: c_uint = 0x500;
pub const VNCR_VSESR_EL2: c_uint = 0x508;
pub const VNCR_PMBLIMITR_EL1: c_uint = 0x800;
pub const VNCR_PMBPTR_EL1: c_uint = 0x810;
pub const VNCR_PMBSR_EL1: c_uint = 0x820;
pub const VNCR_PMSCR_EL1: c_uint = 0x828;
pub const VNCR_PMSEVFR_EL1: c_uint = 0x830;
pub const VNCR_PMSICR_EL1: c_uint = 0x838;
pub const VNCR_PMSIRR_EL1: c_uint = 0x840;
pub const VNCR_PMSLATFR_EL1: c_uint = 0x848;
pub const VNCR_PMSNEVFR_EL1: c_uint = 0x850;
pub const VNCR_PMSDSFR_EL1: c_uint = 0x858;
pub const VNCR_TRFCR_EL1: c_uint = 0x880;
pub const VNCR_MPAM1_EL1: c_uint = 0x900;
pub const VNCR_MPAMHCR_EL2: c_uint = 0x930;
pub const VNCR_MPAMVPMV_EL2: c_uint = 0x938;
pub const VNCR_MPAMVPM0_EL2: c_uint = 0x940;
pub const VNCR_MPAMVPM1_EL2: c_uint = 0x948;
pub const VNCR_MPAMVPM2_EL2: c_uint = 0x950;
pub const VNCR_MPAMVPM3_EL2: c_uint = 0x958;
pub const VNCR_MPAMVPM4_EL2: c_uint = 0x960;
pub const VNCR_MPAMVPM5_EL2: c_uint = 0x968;
pub const VNCR_MPAMVPM6_EL2: c_uint = 0x970;
pub const VNCR_MPAMVPM7_EL2: c_uint = 0x978;
pub const VNCR_ICH_HFGITR_EL2: c_uint = 0xB10;
pub const VNCR_ICH_HFGRTR_EL2: c_uint = 0xB18;
pub const VNCR_ICH_HFGWTR_EL2: c_uint = 0xB20;
