//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/dcore0_tpc0_eml_busmon_0_regs.h
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
// DCORE0_TPC0_EML_BUSMON_0
// (Prototype: BMON)
//
pub const mmDCORE0_TPC0_EML_BUSMON_0_CR: c_uint = 0x7000;
pub const mmDCORE0_TPC0_EML_BUSMON_0_REG_RESET: c_uint = 0x7004;
pub const mmDCORE0_TPC0_EML_BUSMON_0_INT_CLR: c_uint = 0x7008;
pub const mmDCORE0_TPC0_EML_BUSMON_0_TRIG_TH: c_uint = 0x700C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_S0: c_uint = 0x7020;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_S0: c_uint = 0x7024;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_E0: c_uint = 0x7028;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_E0: c_uint = 0x702C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_S1: c_uint = 0x7030;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_S1: c_uint = 0x7034;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_E1: c_uint = 0x7038;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_E1: c_uint = 0x703C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_S2: c_uint = 0x7040;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_S2: c_uint = 0x7044;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_E2: c_uint = 0x7048;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_E2: c_uint = 0x704C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_S3: c_uint = 0x7050;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_S3: c_uint = 0x7054;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_E3: c_uint = 0x7058;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_E3: c_uint = 0x705C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_REDUCTION: c_uint = 0x7060;
pub const mmDCORE0_TPC0_EML_BUSMON_0_IDL: c_uint = 0x7070;
pub const mmDCORE0_TPC0_EML_BUSMON_0_IDH: c_uint = 0x7074;
pub const mmDCORE0_TPC0_EML_BUSMON_0_IDENL: c_uint = 0x7078;
pub const mmDCORE0_TPC0_EML_BUSMON_0_IDENH: c_uint = 0x707C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_LATENCY_SMP: c_uint = 0x7090;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ATTR: c_uint = 0x7100;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ATTREN: c_uint = 0x7104;
pub const mmDCORE0_TPC0_EML_BUSMON_0_USRENL: c_uint = 0x7108;
pub const mmDCORE0_TPC0_EML_BUSMON_0_USRL: c_uint = 0x710C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_USRENH: c_uint = 0x7120;
pub const mmDCORE0_TPC0_EML_BUSMON_0_USRH: c_uint = 0x7124;
pub const mmDCORE0_TPC0_EML_BUSMON_0_CAPTURE: c_uint = 0x7200;
pub const mmDCORE0_TPC0_EML_BUSMON_0_RELEASE: c_uint = 0x7204;
pub const mmDCORE0_TPC0_EML_BUSMON_0_WIN_CAPTURE: c_uint = 0x7208;
pub const mmDCORE0_TPC0_EML_BUSMON_0_BW_WIN: c_uint = 0x720C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MATCH_CNT_SOD: c_uint = 0x7220;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MATCH_CNT_WIN: c_uint = 0x7224;
pub const mmDCORE0_TPC0_EML_BUSMON_0_CYCCNT_L: c_uint = 0x7228;
pub const mmDCORE0_TPC0_EML_BUSMON_0_CYCCNT_H: c_uint = 0x722C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MAXLAT_SOD: c_uint = 0x7304;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MINLAT_SOD: c_uint = 0x7308;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MAXBW_SOD: c_uint = 0x7310;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MINBW_SOD: c_uint = 0x7314;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MAXOS_SOD: c_uint = 0x7320;
pub const mmDCORE0_TPC0_EML_BUSMON_0_MINOS_SOD: c_uint = 0x7324;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRL_SNAPSHOT: c_uint = 0x7400;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ADDRH_SNAPSHOT: c_uint = 0x7404;
pub const mmDCORE0_TPC0_EML_BUSMON_0_IDL_SNAPSHOT: c_uint = 0x7408;
pub const mmDCORE0_TPC0_EML_BUSMON_0_IDH_SNAPSHOT: c_uint = 0x740C;
pub const mmDCORE0_TPC0_EML_BUSMON_0_ATTR_SNAPSHOT: c_uint = 0x7410;
pub const mmDCORE0_TPC0_EML_BUSMON_0_STM_TRC: c_uint = 0x7420;
pub const mmDCORE0_TPC0_EML_BUSMON_0_STM_TRC_DROP: c_uint = 0x7424;
pub const mmDCORE0_TPC0_EML_BUSMON_0_DEVARCH: c_uint = 0x7FBC;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PMDEVID2: c_uint = 0x7FC0;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PMDEVID1: c_uint = 0x7FC4;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PMDEVID: c_uint = 0x7FC8;
pub const mmDCORE0_TPC0_EML_BUSMON_0_DEVTYPE: c_uint = 0x7FCC;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR4: c_uint = 0x7FD0;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR5: c_uint = 0x7FD4;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR6: c_uint = 0x7FD8;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR7: c_uint = 0x7FDC;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR0: c_uint = 0x7FE0;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR1: c_uint = 0x7FE4;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR2: c_uint = 0x7FE8;
pub const mmDCORE0_TPC0_EML_BUSMON_0_PIDR3: c_uint = 0x7FEC;
pub const mmDCORE0_TPC0_EML_BUSMON_0_CIDR0: c_uint = 0x7FF0;
pub const mmDCORE0_TPC0_EML_BUSMON_0_CIDR1: c_uint = 0x7FF4;
pub const mmDCORE0_TPC0_EML_BUSMON_0_CIDR2: c_uint = 0x7FF8;
pub const mmDCORE0_TPC0_EML_BUSMON_0_CIDR3: c_uint = 0x7FFC;
