//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/pmc_atom.h
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
// Intel Atom SoC Power Management Controller Header File
// Copyright (c) 2014-2015,2022 Intel Corporation.
//

// ValleyView Power Control Unit PCI Device ID
pub const PCI_DEVICE_ID_VLV_PMC: c_uint = 0x0F1C;
// CherryTrail Power Control Unit PCI Device ID
pub const PCI_DEVICE_ID_CHT_PMC: c_uint = 0x229C;
// PMC Memory mapped IO registers
pub const PMC_BASE_ADDR_OFFSET: c_uint = 0x44;
pub const PMC_BASE_ADDR_MASK: c_uint = 0xFFFFFE00;
pub const PMC_MMIO_REG_LEN: c_uint = 0x100;
pub const PMC_REG_BIT_WIDTH: c_int = 32;
// BIOS uses FUNC_DIS to disable specific function
pub const PMC_FUNC_DIS: c_uint = 0x34;
pub const PMC_FUNC_DIS_2: c_uint = 0x38;
// CHT specific bits in FUNC_DIS2 register

// S0ix wake event control
pub const PMC_S0IX_WAKE_EN: c_uint = 0x3C;

// External clk generator settings
pub const PMC_CLK_CTL_OFFSET: c_uint = 0x60;
pub const PMC_CLK_CTL_SIZE: c_int = 4;
pub const PMC_CLK_NUM: c_int = 6;
pub const PMC_CLK_CTL_GATED_ON_D3: c_uint = 0x0;
pub const PMC_CLK_CTL_FORCE_ON: c_uint = 0x1;
pub const PMC_CLK_CTL_FORCE_OFF: c_uint = 0x2;
pub const PMC_CLK_CTL_RESERVED: c_uint = 0x3;

// The timers accumulate time spent in sleep state
pub const PMC_S0IR_TMR: c_uint = 0x80;
pub const PMC_S0I1_TMR: c_uint = 0x84;
pub const PMC_S0I2_TMR: c_uint = 0x88;
pub const PMC_S0I3_TMR: c_uint = 0x8C;
pub const PMC_S0_TMR: c_uint = 0x90;
// Sleep state counter is in units of 32us
pub const PMC_TMR_SHIFT: c_int = 5;
// Power status of power islands
pub const PMC_PSS: c_uint = 0x98;

// CHT specific bits in PSS register

// These registers reflect D3 status of functions
pub const PMC_D3_STS_0: c_uint = 0xA0;

pub const PMC_D3_STS_1: c_uint = 0xA4;

// CHT specific bits in PMC_D3_STS_1 register

// PMC I/O Registers
pub const ACPI_BASE_ADDR_OFFSET: c_uint = 0x40;
pub const ACPI_BASE_ADDR_MASK: c_uint = 0xFFFFFE00;
pub const ACPI_MMIO_REG_LEN: c_uint = 0x100;
pub const PM1_CNT: c_uint = 0x4;

pub const SLEEP_TYPE_S5: c_uint = 0x1C00;

extern "C" {
    pub fn pmc_atom_read(offset: c_int, value: *mut u32) -> c_int;
}
