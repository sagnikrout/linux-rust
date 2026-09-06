//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/watchdog/sp5100_tco.h
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
// sp5100_tco:	TCO timer driver for sp5100 chipsets.
//
// (c) Copyright 2009 Google Inc., All Rights Reserved.
//
// TCO timer driver for sp5100 chipsets
//

//
// Some address definitions for the Watchdog
//
pub const SP5100_WDT_MEM_MAP_SIZE: c_uint = 0x08;

pub const SP5100_PM_IOPORTS_SIZE: c_uint = 0x02;
//
// These two IO registers are hardcoded and there doesn't seem to be a way to
// read them from a register.
//
// For SP5100/SB7x0/SB8x0 chipset
pub const SP5100_IO_PM_INDEX_REG: c_uint = 0xCD6;
pub const SP5100_IO_PM_DATA_REG: c_uint = 0xCD7;
// For SP5100/SB7x0 chipset
pub const SP5100_SB_RESOURCE_MMIO_BASE: c_uint = 0x9C;
pub const SP5100_PM_WATCHDOG_CONTROL: c_uint = 0x69;
pub const SP5100_PM_WATCHDOG_BASE: c_uint = 0x6C;
pub const SP5100_PCI_WATCHDOG_MISC_REG: c_uint = 0x41;

// For SB8x0(or later) chipset
pub const SB800_PM_ACPI_MMIO_EN: c_uint = 0x24;
pub const SB800_PM_WATCHDOG_CONTROL: c_uint = 0x48;
pub const SB800_PM_WATCHDOG_BASE: c_uint = 0x48;
pub const SB800_PM_WATCHDOG_CONFIG: c_uint = 0x4C;

pub const SB800_PM_WDT_MMIO_OFFSET: c_uint = 0xB00;

// For recent chips with embedded FCH (rev 40+)
pub const EFCH_PM_DECODEEN: c_uint = 0x00;

pub const EFCH_PM_DECODEEN3: c_uint = 0x03;

// WDT MMIO if enabled with PM00_DECODEEN_WDT_TMREN
pub const EFCH_PM_WDT_ADDR: c_uint = 0xfeb00000;
pub const EFCH_PM_ISACONTROL: c_uint = 0x04;

pub const EFCH_PM_ACPI_MMIO_ADDR: c_uint = 0xfed80000;
pub const EFCH_PM_ACPI_MMIO_PM_OFFSET: c_uint = 0x00000300;
pub const EFCH_PM_ACPI_MMIO_WDT_OFFSET: c_uint = 0x00000b00;

pub const EFCH_PM_ACPI_MMIO_PM_SIZE: c_int = 8;
pub const AMD_ZEN_SMBUS_PCI_REV: c_uint = 0x51;
