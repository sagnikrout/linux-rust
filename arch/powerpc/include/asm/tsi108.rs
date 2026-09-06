//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/tsi108.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// common routine and memory layout for Tundra TSI108(Grendel) host bridge
// memory controller.
//
// Author: Jacob Pan (jacob.pan@freescale.com)
// Alex Bounine (alexandreb@tundra.com)
//
// Copyright 2004-2006 Freescale Semiconductor, Inc.
//

// Size of entire register space

// Sizes of register spaces for individual blocks
pub const TSI108_HLP_SIZE: c_uint = 0x1000;
pub const TSI108_PCI_SIZE: c_uint = 0x1000;
pub const TSI108_CLK_SIZE: c_uint = 0x1000;
pub const TSI108_PB_SIZE: c_uint = 0x1000;
pub const TSI108_SD_SIZE: c_uint = 0x1000;
pub const TSI108_DMA_SIZE: c_uint = 0x1000;
pub const TSI108_ETH_SIZE: c_uint = 0x1000;
pub const TSI108_I2C_SIZE: c_uint = 0x400;
pub const TSI108_MPIC_SIZE: c_uint = 0x400;
pub const TSI108_UART0_SIZE: c_uint = 0x200;
pub const TSI108_GPIO_SIZE: c_uint = 0x200;
pub const TSI108_UART1_SIZE: c_uint = 0x200;
// Offsets within Tsi108(A) CSR space for individual blocks
pub const TSI108_HLP_OFFSET: c_uint = 0x0000;
pub const TSI108_PCI_OFFSET: c_uint = 0x1000;
pub const TSI108_CLK_OFFSET: c_uint = 0x2000;
pub const TSI108_PB_OFFSET: c_uint = 0x3000;
pub const TSI108_SD_OFFSET: c_uint = 0x4000;
pub const TSI108_DMA_OFFSET: c_uint = 0x5000;
pub const TSI108_ETH_OFFSET: c_uint = 0x6000;
pub const TSI108_I2C_OFFSET: c_uint = 0x7000;
pub const TSI108_MPIC_OFFSET: c_uint = 0x7400;
pub const TSI108_UART0_OFFSET: c_uint = 0x7800;
pub const TSI108_GPIO_OFFSET: c_uint = 0x7A00;
pub const TSI108_UART1_OFFSET: c_uint = 0x7C00;
// Tsi108 registers used by common code components

//
// PHY Configuration Options
//
// Specify "bcm54xx" in the compatible property of your device tree phy
// nodes if your board uses the Broadcom PHYs
//

// Global variables
// Exported functions
extern "C" {
    pub fn tsi108_clear_pci_error(pci_cfg_base: u32);
}
extern "C" {
    pub fn get_csrbase() -> phys_addr_t;
}
extern "C" {
    pub fn get_vir_csrbase() -> u32;
}
extern "C" {
    pub fn in_be32(reg_offset): *mut *mut (volatile u32 )(tsi108_csr_vir_base +) -> return;
}
