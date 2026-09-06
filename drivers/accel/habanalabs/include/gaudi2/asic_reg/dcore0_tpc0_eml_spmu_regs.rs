//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_eml_spmu_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// DCORE0_TPC0_EML_SPMU
// (Prototype: SPMU)
//
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTR0_EL0: c_uint = 0x1000;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTR1_EL0: c_uint = 0x1008;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTR2_EL0: c_uint = 0x1010;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTR3_EL0: c_uint = 0x1018;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTR4_EL0: c_uint = 0x1020;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTR5_EL0: c_uint = 0x1028;
pub const mmDCORE0_TPC0_EML_SPMU_PMCCNTR_L_EL0: c_uint = 0x10F8;
pub const mmDCORE0_TPC0_EML_SPMU_PMCCNTR_H_EL0: c_uint = 0x10FC;
pub const mmDCORE0_TPC0_EML_SPMU_PMTRC: c_uint = 0x1200;
pub const mmDCORE0_TPC0_EML_SPMU_TRC_CTRL_HOST: c_uint = 0x1204;
pub const mmDCORE0_TPC0_EML_SPMU_TRC_STAT_HOST: c_uint = 0x1208;
pub const mmDCORE0_TPC0_EML_SPMU_TRC_EN_HOST: c_uint = 0x120C;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVTYPER0_EL0: c_uint = 0x1400;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVTYPER1_EL0: c_uint = 0x1404;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVTYPER2_EL0: c_uint = 0x1408;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVTYPER3_EL0: c_uint = 0x140C;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVTYPER4_EL0: c_uint = 0x1410;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVTYPER5_EL0: c_uint = 0x1414;
pub const mmDCORE0_TPC0_EML_SPMU_PMSSR: c_uint = 0x1610;
pub const mmDCORE0_TPC0_EML_SPMU_PMOVSSR: c_uint = 0x1614;
pub const mmDCORE0_TPC0_EML_SPMU_PMCCNTSR_L: c_uint = 0x1618;
pub const mmDCORE0_TPC0_EML_SPMU_PMCCNTSR_H: c_uint = 0x161C;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTSR0: c_uint = 0x1620;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTSR1: c_uint = 0x1624;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTSR2: c_uint = 0x1628;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTSR3: c_uint = 0x162C;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTSR4: c_uint = 0x1630;
pub const mmDCORE0_TPC0_EML_SPMU_PMEVCNTSR5: c_uint = 0x1634;
pub const mmDCORE0_TPC0_EML_SPMU_PMSCR: c_uint = 0x16F0;
pub const mmDCORE0_TPC0_EML_SPMU_PMSRR: c_uint = 0x16F4;
pub const mmDCORE0_TPC0_EML_SPMU_PMCNTENSET_EL0: c_uint = 0x1C00;
pub const mmDCORE0_TPC0_EML_SPMU_PMCNTENCLR_EL0: c_uint = 0x1C20;
pub const mmDCORE0_TPC0_EML_SPMU_PMINTENSET_EL1: c_uint = 0x1C40;
pub const mmDCORE0_TPC0_EML_SPMU_PMINTENCLR_EL1: c_uint = 0x1C60;
pub const mmDCORE0_TPC0_EML_SPMU_PMOVSCLR_EL0: c_uint = 0x1C80;
pub const mmDCORE0_TPC0_EML_SPMU_PMSWINC_EL0: c_uint = 0x1CA0;
pub const mmDCORE0_TPC0_EML_SPMU_PMOVSSET_EL0: c_uint = 0x1CC0;
pub const mmDCORE0_TPC0_EML_SPMU_PMCFGR: c_uint = 0x1E00;
pub const mmDCORE0_TPC0_EML_SPMU_PMCR_EL0: c_uint = 0x1E04;
pub const mmDCORE0_TPC0_EML_SPMU_PMITCTRL: c_uint = 0x1F00;
pub const mmDCORE0_TPC0_EML_SPMU_PMCLAIMSET: c_uint = 0x1FA0;
pub const mmDCORE0_TPC0_EML_SPMU_PMCLAIMCLR: c_uint = 0x1FA4;
pub const mmDCORE0_TPC0_EML_SPMU_PMDEVAFF0: c_uint = 0x1FA8;
pub const mmDCORE0_TPC0_EML_SPMU_PMDEVAFF1: c_uint = 0x1FAC;
pub const mmDCORE0_TPC0_EML_SPMU_PMLAR: c_uint = 0x1FB0;
pub const mmDCORE0_TPC0_EML_SPMU_PMLSR: c_uint = 0x1FB4;
pub const mmDCORE0_TPC0_EML_SPMU_PMAUTHSTATUS: c_uint = 0x1FB8;
pub const mmDCORE0_TPC0_EML_SPMU_PMDEVARCH: c_uint = 0x1FBC;
pub const mmDCORE0_TPC0_EML_SPMU_PMDEVID2: c_uint = 0x1FC0;
pub const mmDCORE0_TPC0_EML_SPMU_PMDEVID1: c_uint = 0x1FC4;
pub const mmDCORE0_TPC0_EML_SPMU_PMDEVID: c_uint = 0x1FC8;
pub const mmDCORE0_TPC0_EML_SPMU_PMDEVTYPE: c_uint = 0x1FCC;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR4: c_uint = 0x1FD0;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR5: c_uint = 0x1FD4;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR6: c_uint = 0x1FD8;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR7: c_uint = 0x1FDC;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR0: c_uint = 0x1FE0;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR1: c_uint = 0x1FE4;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR2: c_uint = 0x1FE8;
pub const mmDCORE0_TPC0_EML_SPMU_PMPIDR3: c_uint = 0x1FEC;
pub const mmDCORE0_TPC0_EML_SPMU_PMCIDR0: c_uint = 0x1FF0;
pub const mmDCORE0_TPC0_EML_SPMU_PMCIDR1: c_uint = 0x1FF4;
pub const mmDCORE0_TPC0_EML_SPMU_PMCIDR2: c_uint = 0x1FF8;
pub const mmDCORE0_TPC0_EML_SPMU_PMCIDR3: c_uint = 0x1FFC;
