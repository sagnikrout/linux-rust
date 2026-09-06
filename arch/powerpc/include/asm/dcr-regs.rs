//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/dcr-regs.h
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
// Common DCR / SDR / CPR register definitions used on various IBM/AMCC
// 4xx processors
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp
// <benh@kernel.crashing.org>
//
// Mostly lifted from asm-ppc/ibm4xx.h by
//
// Copyright (c) 1999 Grant Erickson <grant@lcse.umn.edu>
//
// Most DCRs used for controlling devices such as the MAL, DMA engine,
// etc... are obtained for the device tree.
//
// The definitions in this files are fixed DCRs and indirect DCRs that
// are commonly used outside of specific drivers or refer to core
// common registers that may occasionally have to be tweaked outside
// of the driver main register set
//
// CPRs (440GX and 440SP/440SPe)
pub const DCRN_CPR0_CONFIG_ADDR: c_uint = 0xc;
pub const DCRN_CPR0_CONFIG_DATA: c_uint = 0xd;
// SDRs (440GX and 440SP/440SPe)
pub const DCRN_SDR0_CONFIG_ADDR: c_uint = 0xe;
pub const DCRN_SDR0_CONFIG_DATA: c_uint = 0xf;
pub const SDR0_PFC0: c_uint = 0x4100;
pub const SDR0_PFC1: c_uint = 0x4101;
pub const SDR0_PFC1_EPS: c_uint = 0x1c00000;
pub const SDR0_PFC1_EPS_SHIFT: c_int = 22;
pub const SDR0_PFC1_RMII: c_uint = 0x02000000;
pub const SDR0_MFR: c_uint = 0x4300;
pub const SDR0_MFR_TAH0: c_uint = 0x80000000  	/* TAHOE0 Enable */;
pub const SDR0_MFR_TAH1: c_uint = 0x40000000  	/* TAHOE1 Enable */;
pub const SDR0_MFR_PCM: c_uint = 0x10000000  	/* PPC440GP irq compat mode */;
pub const SDR0_MFR_ECS: c_uint = 0x08000000  	/* EMAC int clk */;
pub const SDR0_MFR_T0TXFL: c_uint = 0x00080000;
pub const SDR0_MFR_T0TXFH: c_uint = 0x00040000;
pub const SDR0_MFR_T1TXFL: c_uint = 0x00020000;
pub const SDR0_MFR_T1TXFH: c_uint = 0x00010000;
pub const SDR0_MFR_E0TXFL: c_uint = 0x00008000;
pub const SDR0_MFR_E0TXFH: c_uint = 0x00004000;
pub const SDR0_MFR_E0RXFL: c_uint = 0x00002000;
pub const SDR0_MFR_E0RXFH: c_uint = 0x00001000;
pub const SDR0_MFR_E1TXFL: c_uint = 0x00000800;
pub const SDR0_MFR_E1TXFH: c_uint = 0x00000400;
pub const SDR0_MFR_E1RXFL: c_uint = 0x00000200;
pub const SDR0_MFR_E1RXFH: c_uint = 0x00000100;
pub const SDR0_MFR_E2TXFL: c_uint = 0x00000080;
pub const SDR0_MFR_E2TXFH: c_uint = 0x00000040;
pub const SDR0_MFR_E2RXFL: c_uint = 0x00000020;
pub const SDR0_MFR_E2RXFH: c_uint = 0x00000010;
pub const SDR0_MFR_E3TXFL: c_uint = 0x00000008;
pub const SDR0_MFR_E3TXFH: c_uint = 0x00000004;
pub const SDR0_MFR_E3RXFL: c_uint = 0x00000002;
pub const SDR0_MFR_E3RXFH: c_uint = 0x00000001;
pub const SDR0_UART0: c_uint = 0x0120;
pub const SDR0_UART1: c_uint = 0x0121;
pub const SDR0_UART2: c_uint = 0x0122;
pub const SDR0_UART3: c_uint = 0x0123;
pub const SDR0_CUST0: c_uint = 0x4000;
// SDR for 405EZ
pub const DCRN_SDR_ICINTSTAT: c_uint = 0x4510;
pub const ICINTSTAT_ICRX: c_uint = 0x80000000;
pub const ICINTSTAT_ICTX0: c_uint = 0x40000000;
pub const ICINTSTAT_ICTX1: c_uint = 0x20000000;
pub const ICINTSTAT_ICTX: c_uint = 0x60000000;
// SDRs (460EX/460GT)
pub const SDR0_ETH_CFG: c_uint = 0x4103;
pub const SDR0_ETH_CFG_ECS: c_uint = 0x00000100	/* EMAC int clk source */;
//
// All those DCR register addresses are offsets from the base address
// for the SRAM0 controller (e.g. 0x20 on 440GX). The base address is
// excluded here and configured in the device tree.
//
pub const DCRN_SRAM0_SB0CR: c_uint = 0x00;
pub const DCRN_SRAM0_SB1CR: c_uint = 0x01;
pub const DCRN_SRAM0_SB2CR: c_uint = 0x02;
pub const DCRN_SRAM0_SB3CR: c_uint = 0x03;
pub const SRAM_SBCR_BU_MASK: c_uint = 0x00000180;
pub const SRAM_SBCR_BS_64KB: c_uint = 0x00000800;
pub const SRAM_SBCR_BU_RO: c_uint = 0x00000080;
pub const SRAM_SBCR_BU_RW: c_uint = 0x00000180;
pub const DCRN_SRAM0_BEAR: c_uint = 0x04;
pub const DCRN_SRAM0_BESR0: c_uint = 0x05;
pub const DCRN_SRAM0_BESR1: c_uint = 0x06;
pub const DCRN_SRAM0_PMEG: c_uint = 0x07;
pub const DCRN_SRAM0_CID: c_uint = 0x08;
pub const DCRN_SRAM0_REVID: c_uint = 0x09;
pub const DCRN_SRAM0_DPC: c_uint = 0x0a;
pub const SRAM_DPC_ENABLE: c_uint = 0x80000000;
//
// All those DCR register addresses are offsets from the base address
// for the SRAM0 controller (e.g. 0x30 on 440GX). The base address is
// excluded here and configured in the device tree.
//
pub const DCRN_L2C0_CFG: c_uint = 0x00;
pub const L2C_CFG_L2M: c_uint = 0x80000000;
pub const L2C_CFG_ICU: c_uint = 0x40000000;
pub const L2C_CFG_DCU: c_uint = 0x20000000;
pub const L2C_CFG_DCW_MASK: c_uint = 0x1e000000;
pub const L2C_CFG_TPC: c_uint = 0x01000000;
pub const L2C_CFG_CPC: c_uint = 0x00800000;
pub const L2C_CFG_FRAN: c_uint = 0x00200000;
pub const L2C_CFG_SS_MASK: c_uint = 0x00180000;
pub const L2C_CFG_SS_256: c_uint = 0x00000000;
pub const L2C_CFG_CPIM: c_uint = 0x00040000;
pub const L2C_CFG_TPIM: c_uint = 0x00020000;
pub const L2C_CFG_LIM: c_uint = 0x00010000;
pub const L2C_CFG_PMUX_MASK: c_uint = 0x00007000;
pub const L2C_CFG_PMUX_SNP: c_uint = 0x00000000;
pub const L2C_CFG_PMUX_IF: c_uint = 0x00001000;
pub const L2C_CFG_PMUX_DF: c_uint = 0x00002000;
pub const L2C_CFG_PMUX_DS: c_uint = 0x00003000;
pub const L2C_CFG_PMIM: c_uint = 0x00000800;
pub const L2C_CFG_TPEI: c_uint = 0x00000400;
pub const L2C_CFG_CPEI: c_uint = 0x00000200;
pub const L2C_CFG_NAM: c_uint = 0x00000100;
pub const L2C_CFG_SMCM: c_uint = 0x00000080;
pub const L2C_CFG_NBRM: c_uint = 0x00000040;
pub const L2C_CFG_RDBW: c_uint = 0x00000008	/* only 460EX/GT */;
pub const DCRN_L2C0_CMD: c_uint = 0x01;
pub const L2C_CMD_CLR: c_uint = 0x80000000;
pub const L2C_CMD_DIAG: c_uint = 0x40000000;
pub const L2C_CMD_INV: c_uint = 0x20000000;
pub const L2C_CMD_CCP: c_uint = 0x10000000;
pub const L2C_CMD_CTE: c_uint = 0x08000000;
pub const L2C_CMD_STRC: c_uint = 0x04000000;
pub const L2C_CMD_STPC: c_uint = 0x02000000;
pub const L2C_CMD_RPMC: c_uint = 0x01000000;
pub const L2C_CMD_HCC: c_uint = 0x00800000;
pub const DCRN_L2C0_ADDR: c_uint = 0x02;
pub const DCRN_L2C0_DATA: c_uint = 0x03;
pub const DCRN_L2C0_SR: c_uint = 0x04;
pub const L2C_SR_CC: c_uint = 0x80000000;
pub const L2C_SR_CPE: c_uint = 0x40000000;
pub const L2C_SR_TPE: c_uint = 0x20000000;
pub const L2C_SR_LRU: c_uint = 0x10000000;
pub const L2C_SR_PCS: c_uint = 0x08000000;
pub const DCRN_L2C0_REVID: c_uint = 0x05;
pub const DCRN_L2C0_SNP0: c_uint = 0x06;
pub const DCRN_L2C0_SNP1: c_uint = 0x07;
pub const L2C_SNP_BA_MASK: c_uint = 0xffff0000;
pub const L2C_SNP_SSR_MASK: c_uint = 0x0000f000;
pub const L2C_SNP_SSR_32G: c_uint = 0x0000f000;
pub const L2C_SNP_ESR: c_uint = 0x00000800;
//
// DCR register offsets for 440SP/440SPe I2O/DMA controller.
// The base address is configured in the device tree.
//
pub const DCRN_I2O0_IBAL: c_uint = 0x006;
pub const DCRN_I2O0_IBAH: c_uint = 0x007;
pub const I2O_REG_ENABLE: c_uint = 0x00000001	/* Enable I2O/DMA access */;
// 440SP/440SPe Software Reset DCR
pub const DCRN_SDR0_SRST: c_uint = 0x0200;

// 440SP/440SPe Memory Queue DCR offsets
pub const DCRN_MQ0_XORBA: c_uint = 0x04;
pub const DCRN_MQ0_CF2H: c_uint = 0x06;
pub const DCRN_MQ0_CFBHL: c_uint = 0x0f;
pub const DCRN_MQ0_BAUH: c_uint = 0x10;
// HB/LL Paths Configuration Register
pub const MQ0_CFBHL_TPLM: c_int = 28;
pub const MQ0_CFBHL_HBCL: c_int = 23;
pub const MQ0_CFBHL_POLY: c_int = 15;
