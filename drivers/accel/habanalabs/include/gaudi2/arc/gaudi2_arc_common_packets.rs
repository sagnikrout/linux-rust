//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/arc/gaudi2_arc_common_packets.h
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
// Copyright (C) 2020 HabanaLabs Ltd.
// All Rights Reserved.
//
// Dcore1 MME Engine ARC instance used as scheduler
// Dcore3 MME Engine ARC instance used as scheduler
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arc_regions_t {
    ARC_REGION0_UNSED  = 0,
//
// Extension registers
// None
//
    ARC_REGION1_SRAM = 1,
//
// Extension registers
// AUX_SRAM_LSB_ADDR
// AUX_SRAM_MSB_ADDR
// ARC Address: 0x1000_0000
//
    ARC_REGION2_CFG = 2,
//
// Extension registers
// AUX_CFG_LSB_ADDR
// AUX_CFG_MSB_ADDR
// ARC Address: 0x2000_0000
//
    ARC_REGION3_GENERAL = 3,
//
// Extension registers
// AUX_GENERAL_PURPOSE_LSB_ADDR_0
// AUX_GENERAL_PURPOSE_MSB_ADDR_0
// ARC Address: 0x3000_0000
//
    ARC_REGION4_HBM0_FW = 4,
//
// Extension registers
// AUX_HBM0_LSB_ADDR
// AUX_HBM0_MSB_ADDR
// AUX_HBM0_OFFSET
// ARC Address: 0x4000_0000
//
    ARC_REGION5_HBM1_GC_DATA = 5,
//
// Extension registers
// AUX_HBM1_LSB_ADDR
// AUX_HBM1_MSB_ADDR
// AUX_HBM1_OFFSET
// ARC Address: 0x5000_0000
//
    ARC_REGION6_HBM2_GC_DATA = 6,
//
// Extension registers
// AUX_HBM2_LSB_ADDR
// AUX_HBM2_MSB_ADDR
// AUX_HBM2_OFFSET
// ARC Address: 0x6000_0000
//
    ARC_REGION7_HBM3_GC_DATA = 7,
//
// Extension registers
// AUX_HBM3_LSB_ADDR
// AUX_HBM3_MSB_ADDR
// AUX_HBM3_OFFSET
// ARC Address: 0x7000_0000
//
    ARC_REGION8_DCCM = 8,
//
// Extension registers
// None
// ARC Address: 0x8000_0000
//
    ARC_REGION9_PCIE = 9,
//
// Extension registers
// AUX_PCIE_LSB_ADDR
// AUX_PCIE_MSB_ADDR
// ARC Address: 0x9000_0000
//
    ARC_REGION10_GENERAL = 10,
//
// Extension registers
// AUX_GENERAL_PURPOSE_LSB_ADDR_1
// AUX_GENERAL_PURPOSE_MSB_ADDR_1
// ARC Address: 0xA000_0000
//
    ARC_REGION11_GENERAL = 11,
//
// Extension registers
// AUX_GENERAL_PURPOSE_LSB_ADDR_2
// AUX_GENERAL_PURPOSE_MSB_ADDR_2
// ARC Address: 0xB000_0000
//
    ARC_REGION12_GENERAL = 12,
//
// Extension registers
// AUX_GENERAL_PURPOSE_LSB_ADDR_3
// AUX_GENERAL_PURPOSE_MSB_ADDR_3
// ARC Address: 0xC000_0000
//
    ARC_REGION13_GENERAL = 13,
//
// Extension registers
// AUX_GENERAL_PURPOSE_LSB_ADDR_4
// AUX_GENERAL_PURPOSE_MSB_ADDR_4
// ARC Address: 0xD000_0000
//
    ARC_REGION14_GENERAL = 14,
//
// Extension registers
// AUX_GENERAL_PURPOSE_LSB_ADDR_5
// AUX_GENERAL_PURPOSE_MSB_ADDR_5
// ARC Address: 0xE000_0000
//
    ARC_REGION15_LBU = 15
//
// Extension registers
// None
// ARC Address: 0xF000_0000
//
}
