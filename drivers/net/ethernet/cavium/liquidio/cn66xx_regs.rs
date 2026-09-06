//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/cn66xx_regs.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// ! \file cn66xx_regs.h
// \brief Host Driver: Register Address and Register Mask values for
// Octeon CN66XX devices.
//
pub const CN6XXX_XPANSION_BAR: c_uint = 0x30;
pub const CN6XXX_MSI_CAP: c_uint = 0x50;
pub const CN6XXX_MSI_ADDR_LO: c_uint = 0x54;
pub const CN6XXX_MSI_ADDR_HI: c_uint = 0x58;
pub const CN6XXX_MSI_DATA: c_uint = 0x5C;
pub const CN6XXX_PCIE_CAP: c_uint = 0x70;
pub const CN6XXX_PCIE_DEVCAP: c_uint = 0x74;
pub const CN6XXX_PCIE_DEVCTL: c_uint = 0x78;
pub const CN6XXX_PCIE_LINKCAP: c_uint = 0x7C;
pub const CN6XXX_PCIE_LINKCTL: c_uint = 0x80;
pub const CN6XXX_PCIE_SLOTCAP: c_uint = 0x84;
pub const CN6XXX_PCIE_SLOTCTL: c_uint = 0x88;
pub const CN6XXX_PCIE_ENH_CAP: c_uint = 0x100;
pub const CN6XXX_PCIE_UNCORR_ERR_STATUS: c_uint = 0x104;
pub const CN6XXX_PCIE_UNCORR_ERR_MASK: c_uint = 0x108;
pub const CN6XXX_PCIE_UNCORR_ERR: c_uint = 0x10C;
pub const CN6XXX_PCIE_CORR_ERR_STATUS: c_uint = 0x110;
pub const CN6XXX_PCIE_CORR_ERR_MASK: c_uint = 0x114;
pub const CN6XXX_PCIE_ADV_ERR_CAP: c_uint = 0x118;
pub const CN6XXX_PCIE_ACK_REPLAY_TIMER: c_uint = 0x700;
pub const CN6XXX_PCIE_OTHER_MSG: c_uint = 0x704;
pub const CN6XXX_PCIE_PORT_FORCE_LINK: c_uint = 0x708;
pub const CN6XXX_PCIE_ACK_FREQ: c_uint = 0x70C;
pub const CN6XXX_PCIE_PORT_LINK_CTL: c_uint = 0x710;
pub const CN6XXX_PCIE_LANE_SKEW: c_uint = 0x714;
pub const CN6XXX_PCIE_SYM_NUM: c_uint = 0x718;
pub const CN6XXX_PCIE_FLTMSK: c_uint = 0x720;
// ##############  BAR0 Registers ################
pub const CN6XXX_SLI_CTL_PORT0: c_uint = 0x0050;
pub const CN6XXX_SLI_CTL_PORT1: c_uint = 0x0060;
pub const CN6XXX_SLI_WINDOW_CTL: c_uint = 0x02E0;
pub const CN6XXX_SLI_DBG_DATA: c_uint = 0x0310;
pub const CN6XXX_SLI_SCRATCH1: c_uint = 0x03C0;
pub const CN6XXX_SLI_SCRATCH2: c_uint = 0x03D0;
pub const CN6XXX_SLI_CTL_STATUS: c_uint = 0x0570;
pub const CN6XXX_WIN_WR_ADDR_LO: c_uint = 0x0000;
pub const CN6XXX_WIN_WR_ADDR_HI: c_uint = 0x0004;

pub const CN6XXX_WIN_RD_ADDR_LO: c_uint = 0x0010;
pub const CN6XXX_WIN_RD_ADDR_HI: c_uint = 0x0014;

pub const CN6XXX_WIN_WR_DATA_LO: c_uint = 0x0020;
pub const CN6XXX_WIN_WR_DATA_HI: c_uint = 0x0024;

pub const CN6XXX_WIN_RD_DATA_LO: c_uint = 0x0040;
pub const CN6XXX_WIN_RD_DATA_HI: c_uint = 0x0044;

pub const CN6XXX_WIN_WR_MASK_LO: c_uint = 0x0030;
pub const CN6XXX_WIN_WR_MASK_HI: c_uint = 0x0034;

// 1 register (32-bit) to enable Input queues
pub const CN6XXX_SLI_PKT_INSTR_ENB: c_uint = 0x1000;
// 1 register (32-bit) to enable Output queues
pub const CN6XXX_SLI_PKT_OUT_ENB: c_uint = 0x1010;
// 1 register (32-bit) to determine whether Output queues are in reset.
pub const CN6XXX_SLI_PORT_IN_RST_OQ: c_uint = 0x11F0;
// 1 register (32-bit) to determine whether Input queues are in reset.
pub const CN6XXX_SLI_PORT_IN_RST_IQ: c_uint = 0x11F4;
// ###################### REQUEST QUEUE #########################
// 1 register (32-bit) - instr. size of each input queue.
pub const CN6XXX_SLI_PKT_INSTR_SIZE: c_uint = 0x1020;
// 32 registers for Input Queue Instr Count - SLI_PKT_IN_DONE0_CNTS
pub const CN6XXX_SLI_IQ_INSTR_COUNT_START: c_uint = 0x2000;
// 32 registers for Input Queue Start Addr - SLI_PKT0_INSTR_BADDR
pub const CN6XXX_SLI_IQ_BASE_ADDR_START64: c_uint = 0x2800;
// 32 registers for Input Doorbell - SLI_PKT0_INSTR_BAOFF_DBELL
pub const CN6XXX_SLI_IQ_DOORBELL_START: c_uint = 0x2C00;
// 32 registers for Input Queue size - SLI_PKT0_INSTR_FIFO_RSIZE
pub const CN6XXX_SLI_IQ_SIZE_START: c_uint = 0x3000;
// 32 registers for Instruction Header Options - SLI_PKT0_INSTR_HEADER
pub const CN6XXX_SLI_IQ_PKT_INSTR_HDR_START64: c_uint = 0x3400;
// 1 register (64-bit) - Back Pressure for each input queue - SLI_PKT0_IN_BP
pub const CN66XX_SLI_INPUT_BP_START64: c_uint = 0x3800;
// Each Input Queue register is at a 16-byte Offset in BAR0
pub const CN6XXX_IQ_OFFSET: c_uint = 0x10;
// 1 register (32-bit) - ES, RO, NS, Arbitration for Input Queue Data &
// gather list fetches. SLI_PKT_INPUT_CONTROL.
//
pub const CN6XXX_SLI_PKT_INPUT_CONTROL: c_uint = 0x1170;
// 1 register (64-bit) - Number of instructions to read at one time
// - 2 bits for each input ring. SLI_PKT_INSTR_RD_SIZE.
//
pub const CN6XXX_SLI_PKT_INSTR_RD_SIZE: c_uint = 0x11A0;
// 1 register (64-bit) - Assign Input ring to MAC port
// - 2 bits for each input ring. SLI_PKT_IN_PCIE_PORT.
//
pub const CN6XXX_SLI_IN_PCIE_PORT: c_uint = 0x11B0;
// ------- Request Queue Macros ---------

// ------------------ Masks ----------------

// ############################ OUTPUT QUEUE #########################
// 32 registers for Output queue buffer and info size - SLI_PKT0_OUT_SIZE
pub const CN6XXX_SLI_OQ0_BUFF_INFO_SIZE: c_uint = 0x0C00;
// 32 registers for Output Queue Start Addr - SLI_PKT0_SLIST_BADDR
pub const CN6XXX_SLI_OQ_BASE_ADDR_START64: c_uint = 0x1400;
// 32 registers for Output Queue Packet Credits - SLI_PKT0_SLIST_BAOFF_DBELL
pub const CN6XXX_SLI_OQ_PKT_CREDITS_START: c_uint = 0x1800;
// 32 registers for Output Queue size - SLI_PKT0_SLIST_FIFO_RSIZE
pub const CN6XXX_SLI_OQ_SIZE_START: c_uint = 0x1C00;
// 32 registers for Output Queue Packet Count - SLI_PKT0_CNTS
pub const CN6XXX_SLI_OQ_PKT_SENT_START: c_uint = 0x2400;
// Each Output Queue register is at a 16-byte Offset in BAR0
pub const CN6XXX_OQ_OFFSET: c_uint = 0x10;
// 1 register (32-bit) - 1 bit for each output queue
// - Relaxed Ordering setting for reading Output Queues descriptors
// - SLI_PKT_SLIST_ROR
//
pub const CN6XXX_SLI_PKT_SLIST_ROR: c_uint = 0x1030;
// 1 register (32-bit) - 1 bit for each output queue
// - No Snoop mode for reading Output Queues descriptors
// - SLI_PKT_SLIST_NS
//
pub const CN6XXX_SLI_PKT_SLIST_NS: c_uint = 0x1040;
// 1 register (64-bit) - 2 bits for each output queue
// - Endian-Swap mode for reading Output Queue descriptors
// - SLI_PKT_SLIST_ES
//
pub const CN6XXX_SLI_PKT_SLIST_ES64: c_uint = 0x1050;
// 1 register (32-bit) - 1 bit for each output queue
// - InfoPtr mode for Output Queues.
// - SLI_PKT_IPTR
//
pub const CN6XXX_SLI_PKT_IPTR: c_uint = 0x1070;
// 1 register (32-bit) - 1 bit for each output queue
// - DPTR format selector for Output queues.
// - SLI_PKT_DPADDR
//
pub const CN6XXX_SLI_PKT_DPADDR: c_uint = 0x1080;
// 1 register (32-bit) - 1 bit for each output queue
// - Relaxed Ordering setting for reading Output Queues data
// - SLI_PKT_DATA_OUT_ROR
//
pub const CN6XXX_SLI_PKT_DATA_OUT_ROR: c_uint = 0x1090;
// 1 register (32-bit) - 1 bit for each output queue
// - No Snoop mode for reading Output Queues data
// - SLI_PKT_DATA_OUT_NS
//
pub const CN6XXX_SLI_PKT_DATA_OUT_NS: c_uint = 0x10A0;
// 1 register (64-bit)  - 2 bits for each output queue
// - Endian-Swap mode for reading Output Queue data
// - SLI_PKT_DATA_OUT_ES
//
pub const CN6XXX_SLI_PKT_DATA_OUT_ES64: c_uint = 0x10B0;
// 1 register (32-bit) - 1 bit for each output queue
// - Controls whether SLI_PKTn_CNTS is incremented for bytes or for packets.
// - SLI_PKT_OUT_BMODE
//
pub const CN6XXX_SLI_PKT_OUT_BMODE: c_uint = 0x10D0;
// 1 register (64-bit) - 2 bits for each output queue
// - Assign PCIE port for Output queues
// - SLI_PKT_PCIE_PORT.
//
pub const CN6XXX_SLI_PKT_PCIE_PORT64: c_uint = 0x10E0;
// 1 (64-bit) register for Output Queue Packet Count Interrupt Threshold
// & Time Threshold. The same setting applies to all 32 queues.
// The register is defined as a 64-bit registers, but we use the
// 32-bit offsets to define distinct addresses.
//
pub const CN6XXX_SLI_OQ_INT_LEVEL_PKTS: c_uint = 0x1120;
pub const CN6XXX_SLI_OQ_INT_LEVEL_TIME: c_uint = 0x1124;
// 1 (64-bit register) for Output Queue backpressure across all rings.
pub const CN6XXX_SLI_OQ_WMARK: c_uint = 0x1180;
// 1 register to control output queue global backpressure & ring enable.
pub const CN6XXX_SLI_PKT_CTL: c_uint = 0x1220;
// ------- Output Queue Macros ---------

// ######################### DMA Counters #########################
// 2 registers (64-bit) - DMA Count - 1 for each DMA counter 0/1.
pub const CN6XXX_DMA_CNT_START: c_uint = 0x0400;
// 2 registers (64-bit) - DMA Timer 0/1, contains DMA timer values
// SLI_DMA_0_TIM
//
pub const CN6XXX_DMA_TIM_START: c_uint = 0x0420;
// 2 registers (64-bit) - DMA count & Time Interrupt threshold -
// SLI_DMA_0_INT_LEVEL
//
pub const CN6XXX_DMA_INT_LEVEL_START: c_uint = 0x03E0;
// Each DMA register is at a 16-byte Offset in BAR0
pub const CN6XXX_DMA_OFFSET: c_uint = 0x10;
// ---------- DMA Counter Macros ---------

// ######################## INTERRUPTS #########################
// 1 register (64-bit) for Interrupt Summary
pub const CN6XXX_SLI_INT_SUM64: c_uint = 0x0330;
// 1 register (64-bit) for Interrupt Enable
pub const CN6XXX_SLI_INT_ENB64_PORT0: c_uint = 0x0340;
pub const CN6XXX_SLI_INT_ENB64_PORT1: c_uint = 0x0350;
// 1 register (32-bit) to enable Output Queue Packet/Byte Count Interrupt
pub const CN6XXX_SLI_PKT_CNT_INT_ENB: c_uint = 0x1150;
// 1 register (32-bit) to enable Output Queue Packet Timer Interrupt
pub const CN6XXX_SLI_PKT_TIME_INT_ENB: c_uint = 0x1160;
// 1 register (32-bit) to indicate which Output Queue reached pkt threshold
pub const CN6XXX_SLI_PKT_CNT_INT: c_uint = 0x1130;
// 1 register (32-bit) to indicate which Output Queue reached time threshold
pub const CN6XXX_SLI_PKT_TIME_INT: c_uint = 0x1140;
// ------------------ Interrupt Masks ----------------

// Sum of interrupts for all PCI-Express Data Interrupts

// Sum of interrupts for error events

// Programmed Mask for Interrupt Sum

pub const CN6XXX_SLI_S2M_PORT0_CTL: c_uint = 0x3D80;
pub const CN6XXX_SLI_S2M_PORT1_CTL: c_uint = 0x3D90;

pub const CN6XXX_SLI_MAC_NUMBER: c_uint = 0x3E00;
// CN6XXX BAR1 Index registers.
pub const CN6XXX_PEM_BAR1_INDEX000: c_uint = 0x00011800C00000A8ULL;
pub const CN6XXX_PEM_OFFSET: c_uint = 0x0000000001000000ULL;

pub const CN6XXX_PCI_BAR1_OFFSET: c_uint = 0x8;

// ############################ DPI #########################
pub const CN6XXX_DPI_CTL: c_uint = 0x0001df0000000040ULL;
pub const CN6XXX_DPI_DMA_CONTROL: c_uint = 0x0001df0000000048ULL;
pub const CN6XXX_DPI_REQ_GBL_ENB: c_uint = 0x0001df0000000050ULL;
pub const CN6XXX_DPI_REQ_ERR_RSP: c_uint = 0x0001df0000000058ULL;
pub const CN6XXX_DPI_REQ_ERR_RST: c_uint = 0x0001df0000000060ULL;
pub const CN6XXX_DPI_DMA_ENG0_ENB: c_uint = 0x0001df0000000080ULL;

pub const CN6XXX_DPI_DMA_ENG0_BUF: c_uint = 0x0001df0000000880ULL;

pub const CN6XXX_DPI_SLI_PRT0_CFG: c_uint = 0x0001df0000000900ULL;
pub const CN6XXX_DPI_SLI_PRT1_CFG: c_uint = 0x0001df0000000908ULL;

// ############################ CIU #########################
pub const CN6XXX_CIU_SOFT_BIST: c_uint = 0x0001070000000738ULL;
pub const CN6XXX_CIU_SOFT_RST: c_uint = 0x0001070000000740ULL;
// ############################ MIO #########################
pub const CN6XXX_MIO_PTP_CLOCK_CFG: c_uint = 0x0001070000000f00ULL;
pub const CN6XXX_MIO_PTP_CLOCK_LO: c_uint = 0x0001070000000f08ULL;
pub const CN6XXX_MIO_PTP_CLOCK_HI: c_uint = 0x0001070000000f10ULL;
pub const CN6XXX_MIO_PTP_CLOCK_COMP: c_uint = 0x0001070000000f18ULL;
pub const CN6XXX_MIO_PTP_TIMESTAMP: c_uint = 0x0001070000000f20ULL;
pub const CN6XXX_MIO_PTP_EVT_CNT: c_uint = 0x0001070000000f28ULL;
pub const CN6XXX_MIO_PTP_CKOUT_THRESH_LO: c_uint = 0x0001070000000f30ULL;
pub const CN6XXX_MIO_PTP_CKOUT_THRESH_HI: c_uint = 0x0001070000000f38ULL;
pub const CN6XXX_MIO_PTP_CKOUT_HI_INCR: c_uint = 0x0001070000000f40ULL;
pub const CN6XXX_MIO_PTP_CKOUT_LO_INCR: c_uint = 0x0001070000000f48ULL;
pub const CN6XXX_MIO_PTP_PPS_THRESH_LO: c_uint = 0x0001070000000f50ULL;
pub const CN6XXX_MIO_PTP_PPS_THRESH_HI: c_uint = 0x0001070000000f58ULL;
pub const CN6XXX_MIO_PTP_PPS_HI_INCR: c_uint = 0x0001070000000f60ULL;
pub const CN6XXX_MIO_PTP_PPS_LO_INCR: c_uint = 0x0001070000000f68ULL;
pub const CN6XXX_MIO_QLM4_CFG: c_uint = 0x00011800000015B0ULL;
pub const CN6XXX_MIO_RST_BOOT: c_uint = 0x0001180000001600ULL;
pub const CN6XXX_MIO_QLM_CFG_MASK: c_uint = 0x7;
// ############################ LMC #########################
pub const CN6XXX_LMC0_RESET_CTL: c_uint = 0x0001180088000180ULL;
pub const CN6XXX_LMC0_RESET_CTL_DDR3RST_MASK: c_uint = 0x0000000000000001ULL;
