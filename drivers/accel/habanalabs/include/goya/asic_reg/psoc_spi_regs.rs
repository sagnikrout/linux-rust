//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/psoc_spi_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PSOC_SPI (Prototype: SPI)
//
pub const mmPSOC_SPI_CTRLR0: c_uint = 0xC43000;
pub const mmPSOC_SPI_CTRLR1: c_uint = 0xC43004;
pub const mmPSOC_SPI_SSIENR: c_uint = 0xC43008;
pub const mmPSOC_SPI_MWCR: c_uint = 0xC4300C;
pub const mmPSOC_SPI_SER: c_uint = 0xC43010;
pub const mmPSOC_SPI_BAUDR: c_uint = 0xC43014;
pub const mmPSOC_SPI_TXFTLR: c_uint = 0xC43018;
pub const mmPSOC_SPI_RXFTLR: c_uint = 0xC4301C;
pub const mmPSOC_SPI_TXFLR: c_uint = 0xC43020;
pub const mmPSOC_SPI_RXFLR: c_uint = 0xC43024;
pub const mmPSOC_SPI_SR: c_uint = 0xC43028;
pub const mmPSOC_SPI_IMR: c_uint = 0xC4302C;
pub const mmPSOC_SPI_ISR: c_uint = 0xC43030;
pub const mmPSOC_SPI_RISR: c_uint = 0xC43034;
pub const mmPSOC_SPI_TXOICR: c_uint = 0xC43038;
pub const mmPSOC_SPI_RXOICR: c_uint = 0xC4303C;
pub const mmPSOC_SPI_RXUICR: c_uint = 0xC43040;
pub const mmPSOC_SPI_MSTICR: c_uint = 0xC43044;
pub const mmPSOC_SPI_ICR: c_uint = 0xC43048;
pub const mmPSOC_SPI_IDR: c_uint = 0xC43058;
pub const mmPSOC_SPI_SSI_VERSION_ID: c_uint = 0xC4305C;
pub const mmPSOC_SPI_DR0: c_uint = 0xC43060;
pub const mmPSOC_SPI_DR1: c_uint = 0xC43064;
pub const mmPSOC_SPI_DR2: c_uint = 0xC43068;
pub const mmPSOC_SPI_DR3: c_uint = 0xC4306C;
pub const mmPSOC_SPI_DR4: c_uint = 0xC43070;
pub const mmPSOC_SPI_DR5: c_uint = 0xC43074;
pub const mmPSOC_SPI_DR6: c_uint = 0xC43078;
pub const mmPSOC_SPI_DR7: c_uint = 0xC4307C;
pub const mmPSOC_SPI_DR8: c_uint = 0xC43080;
pub const mmPSOC_SPI_DR9: c_uint = 0xC43084;
pub const mmPSOC_SPI_DR10: c_uint = 0xC43088;
pub const mmPSOC_SPI_DR11: c_uint = 0xC4308C;
pub const mmPSOC_SPI_DR12: c_uint = 0xC43090;
pub const mmPSOC_SPI_DR13: c_uint = 0xC43094;
pub const mmPSOC_SPI_DR14: c_uint = 0xC43098;
pub const mmPSOC_SPI_DR15: c_uint = 0xC4309C;
pub const mmPSOC_SPI_DR16: c_uint = 0xC430A0;
pub const mmPSOC_SPI_DR17: c_uint = 0xC430A4;
pub const mmPSOC_SPI_DR18: c_uint = 0xC430A8;
pub const mmPSOC_SPI_DR19: c_uint = 0xC430AC;
pub const mmPSOC_SPI_DR20: c_uint = 0xC430B0;
pub const mmPSOC_SPI_DR21: c_uint = 0xC430B4;
pub const mmPSOC_SPI_DR22: c_uint = 0xC430B8;
pub const mmPSOC_SPI_DR23: c_uint = 0xC430BC;
pub const mmPSOC_SPI_DR24: c_uint = 0xC430C0;
pub const mmPSOC_SPI_DR25: c_uint = 0xC430C4;
pub const mmPSOC_SPI_DR26: c_uint = 0xC430C8;
pub const mmPSOC_SPI_DR27: c_uint = 0xC430CC;
pub const mmPSOC_SPI_DR28: c_uint = 0xC430D0;
pub const mmPSOC_SPI_DR29: c_uint = 0xC430D4;
pub const mmPSOC_SPI_DR30: c_uint = 0xC430D8;
pub const mmPSOC_SPI_DR31: c_uint = 0xC430DC;
pub const mmPSOC_SPI_DR32: c_uint = 0xC430E0;
pub const mmPSOC_SPI_DR33: c_uint = 0xC430E4;
pub const mmPSOC_SPI_DR34: c_uint = 0xC430E8;
pub const mmPSOC_SPI_DR35: c_uint = 0xC430EC;
pub const mmPSOC_SPI_RX_SAMPLE_DLY: c_uint = 0xC430F0;
pub const mmPSOC_SPI_RSVD_1: c_uint = 0xC430F8;
pub const mmPSOC_SPI_RSVD_2: c_uint = 0xC430FC;
