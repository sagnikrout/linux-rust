//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/loongson.h
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
// Author: Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

pub const LOONGSON_LIO_BASE: c_uint = 0x18000000;
pub const LOONGSON_LIO_SIZE: c_uint = 0x00100000	/* 1M */;

pub const LOONGSON_BOOT_BASE: c_uint = 0x1c000000;
pub const LOONGSON_BOOT_SIZE: c_uint = 0x02000000	/* 32M */;

pub const LOONGSON_REG_BASE: c_uint = 0x1fe00000;
pub const LOONGSON_REG_SIZE: c_uint = 0x00100000	/* 1M */;

// GPIO Regs - r/w

pub const MAX_PACKAGES: c_int = 16;

// ============== LS7A registers ===============
pub const LS7A_PCH_REG_BASE: c_uint = 0x10000000UL;
// LPC regs

// CHIPCFG regs

// MISC reg base

// ACPI regs

// RTC regs

pub const LS7A_DMA_NODE_SHF: c_int = 8;
pub const LS7A_DMA_NODE_MASK: c_uint = 0x1F00;

pub const HT1LO_OFFSET: c_uint = 0xe0000000000UL;
// PCI Configuration Space Base
pub const MCFG_EXT_PCICFG_BASE: c_uint = 0xefe00000000UL;
// REG ACCESS

extern "C" {
    pub fn enable_gpe_wakeup();
}
extern "C" {
    pub fn enable_pci_wakeup();
}
