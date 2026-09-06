//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/boot/dcr.h
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

// 440GP/440GX SDRAM controller DCRs
pub const DCRN_SDRAM0_CFGADDR: c_uint = 0x010;
pub const DCRN_SDRAM0_CFGDATA: c_uint = 0x011;

pub const SDRAM0_B0CR: c_uint = 0x40;
pub const SDRAM0_B1CR: c_uint = 0x44;
pub const SDRAM0_B2CR: c_uint = 0x48;
pub const SDRAM0_B3CR: c_uint = 0x4c;
pub const SDRAM_CONFIG_BANK_ENABLE: c_uint = 0x00000001;
pub const SDRAM_CONFIG_SIZE_MASK: c_uint = 0x000e0000;

// 440GP External Bus Controller (EBC)
pub const DCRN_EBC0_CFGADDR: c_uint = 0x012;
pub const DCRN_EBC0_CFGDATA: c_uint = 0x013;
pub const EBC_NUM_BANKS: c_int = 8;
pub const EBC_B0CR: c_uint = 0x00;
pub const EBC_B1CR: c_uint = 0x01;
pub const EBC_B2CR: c_uint = 0x02;
pub const EBC_B3CR: c_uint = 0x03;
pub const EBC_B4CR: c_uint = 0x04;
pub const EBC_B5CR: c_uint = 0x05;
pub const EBC_B6CR: c_uint = 0x06;
pub const EBC_B7CR: c_uint = 0x07;

pub const EBC_BXCR_BAS: c_uint = 0xfff00000;
pub const EBC_BXCR_BS: c_uint = 0x000e0000;

pub const EBC_BXCR_BU: c_uint = 0x00018000;
pub const EBC_BXCR_BU_OFF: c_uint = 0x00000000;
pub const EBC_BXCR_BU_RO: c_uint = 0x00008000;
pub const EBC_BXCR_BU_WO: c_uint = 0x00010000;
pub const EBC_BXCR_BU_RW: c_uint = 0x00018000;
pub const EBC_BXCR_BW: c_uint = 0x00006000;
pub const EBC_B0AP: c_uint = 0x10;
pub const EBC_B1AP: c_uint = 0x11;
pub const EBC_B2AP: c_uint = 0x12;
pub const EBC_B3AP: c_uint = 0x13;
pub const EBC_B4AP: c_uint = 0x14;
pub const EBC_B5AP: c_uint = 0x15;
pub const EBC_B6AP: c_uint = 0x16;
pub const EBC_B7AP: c_uint = 0x17;

pub const EBC_BEAR: c_uint = 0x20;
pub const EBC_BESR: c_uint = 0x21;
pub const EBC_CFG: c_uint = 0x23;
pub const EBC_CID: c_uint = 0x24;
// 440GP Clock, PM, chip control
pub const DCRN_CPC0_SR: c_uint = 0x0b0;
pub const DCRN_CPC0_ER: c_uint = 0x0b1;
pub const DCRN_CPC0_FR: c_uint = 0x0b2;
pub const DCRN_CPC0_SYS0: c_uint = 0x0e0;
pub const CPC0_SYS0_TUNE: c_uint = 0xffc00000;
pub const CPC0_SYS0_FBDV_MASK: c_uint = 0x003c0000;
pub const CPC0_SYS0_FWDVA_MASK: c_uint = 0x00038000;
pub const CPC0_SYS0_FWDVB_MASK: c_uint = 0x00007000;
pub const CPC0_SYS0_OPDV_MASK: c_uint = 0x00000c00;
pub const CPC0_SYS0_EPDV_MASK: c_uint = 0x00000300;
// Helper macros to compute the actual clock divider values from the
// encodings in the CPC0 register

pub const CPC0_SYS0_EXTSL: c_uint = 0x00000080;
pub const CPC0_SYS0_RW_MASK: c_uint = 0x00000060;
pub const CPC0_SYS0_RL: c_uint = 0x00000010;
pub const CPC0_SYS0_ZMIISL_MASK: c_uint = 0x0000000c;
pub const CPC0_SYS0_BYPASS: c_uint = 0x00000002;
pub const CPC0_SYS0_NTO1: c_uint = 0x00000001;
pub const DCRN_CPC0_SYS1: c_uint = 0x0e1;
pub const DCRN_CPC0_CUST0: c_uint = 0x0e2;
pub const DCRN_CPC0_CUST1: c_uint = 0x0e3;
pub const DCRN_CPC0_STRP0: c_uint = 0x0e4;
pub const DCRN_CPC0_STRP1: c_uint = 0x0e5;
pub const DCRN_CPC0_STRP2: c_uint = 0x0e6;
pub const DCRN_CPC0_STRP3: c_uint = 0x0e7;
pub const DCRN_CPC0_GPIO: c_uint = 0x0e8;
pub const DCRN_CPC0_PLB: c_uint = 0x0e9;
pub const DCRN_CPC0_CR1: c_uint = 0x0ea;
pub const DCRN_CPC0_CR0: c_uint = 0x0eb;
pub const CPC0_CR0_SWE: c_uint = 0x80000000;
pub const CPC0_CR0_CETE: c_uint = 0x40000000;
pub const CPC0_CR0_U1FCS: c_uint = 0x20000000;
pub const CPC0_CR0_U0DTE: c_uint = 0x10000000;
pub const CPC0_CR0_U0DRE: c_uint = 0x08000000;
pub const CPC0_CR0_U0DC: c_uint = 0x04000000;
pub const CPC0_CR0_U1DTE: c_uint = 0x02000000;
pub const CPC0_CR0_U1DRE: c_uint = 0x01000000;
pub const CPC0_CR0_U1DC: c_uint = 0x00800000;
pub const CPC0_CR0_U0EC: c_uint = 0x00400000;
pub const CPC0_CR0_U1EC: c_uint = 0x00200000;
pub const CPC0_CR0_UDIV_MASK: c_uint = 0x001f0000;

pub const DCRN_CPC0_MIRQ0: c_uint = 0x0ec;
pub const DCRN_CPC0_MIRQ1: c_uint = 0x0ed;
pub const DCRN_CPC0_JTAGID: c_uint = 0x0ef;
pub const DCRN_MAL0_CFG: c_uint = 0x180;
pub const MAL_RESET: c_uint = 0x80000000;
// 440EP Clock/Power-on Reset regs
pub const DCRN_CPR0_ADDR: c_uint = 0xc;
pub const DCRN_CPR0_DATA: c_uint = 0xd;
pub const CPR0_PLLD0: c_uint = 0x60;
pub const CPR0_OPBD0: c_uint = 0xc0;
pub const CPR0_PERD0: c_uint = 0xe0;
pub const CPR0_PRIMBD0: c_uint = 0xa0;
pub const CPR0_SCPID: c_uint = 0x120;
pub const CPR0_PLLC0: c_uint = 0x40;
// 440GX/405EX Clock Control reg
pub const DCRN_CPR0_CLKUPD: c_uint = 0x020;
pub const DCRN_CPR0_PLLC: c_uint = 0x040;
pub const DCRN_CPR0_PLLD: c_uint = 0x060;
pub const DCRN_CPR0_PRIMAD: c_uint = 0x080;
pub const DCRN_CPR0_PRIMBD: c_uint = 0x0a0;
pub const DCRN_CPR0_OPBD: c_uint = 0x0c0;
pub const DCRN_CPR0_PERD: c_uint = 0x0e0;
pub const DCRN_CPR0_MALD: c_uint = 0x100;
pub const DCRN_SDR0_CONFIG_ADDR: c_uint = 0xe;
pub const DCRN_SDR0_CONFIG_DATA: c_uint = 0xf;
// SDR read/write helper macros

pub const DCRN_SDR0_UART0: c_uint = 0x0120;
pub const DCRN_SDR0_UART1: c_uint = 0x0121;
pub const DCRN_SDR0_UART2: c_uint = 0x0122;
pub const DCRN_SDR0_UART3: c_uint = 0x0123;
// CPRs read/write helper macros - based off include/asm-ppc/ibm44x.h
pub const DCRN_CPR0_CFGADDR: c_uint = 0xc;
pub const DCRN_CPR0_CFGDATA: c_uint = 0xd;

