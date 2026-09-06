//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/cn23xx_pf_regs.h
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
// ! \file cn23xx_regs.h
// \brief Host Driver: Register Address and Register Mask values for
// Octeon CN23XX devices.
//
pub const CN23XX_CONFIG_VENDOR_ID: c_uint = 0x00;
pub const CN23XX_CONFIG_DEVICE_ID: c_uint = 0x02;
pub const CN23XX_CONFIG_XPANSION_BAR: c_uint = 0x38;
pub const CN23XX_CONFIG_MSIX_CAP: c_uint = 0x50;
pub const CN23XX_CONFIG_MSIX_LMSI: c_uint = 0x54;
pub const CN23XX_CONFIG_MSIX_UMSI: c_uint = 0x58;
pub const CN23XX_CONFIG_MSIX_MSIMD: c_uint = 0x5C;
pub const CN23XX_CONFIG_MSIX_MSIMM: c_uint = 0x60;
pub const CN23XX_CONFIG_MSIX_MSIMP: c_uint = 0x64;
pub const CN23XX_CONFIG_PCIE_CAP: c_uint = 0x70;
pub const CN23XX_CONFIG_PCIE_DEVCAP: c_uint = 0x74;
pub const CN23XX_CONFIG_PCIE_DEVCTL: c_uint = 0x78;
pub const CN23XX_CONFIG_PCIE_LINKCAP: c_uint = 0x7C;
pub const CN23XX_CONFIG_PCIE_LINKCTL: c_uint = 0x80;
pub const CN23XX_CONFIG_PCIE_SLOTCAP: c_uint = 0x84;
pub const CN23XX_CONFIG_PCIE_SLOTCTL: c_uint = 0x88;
pub const CN23XX_CONFIG_PCIE_DEVCTL2: c_uint = 0x98;
pub const CN23XX_CONFIG_PCIE_LINKCTL2: c_uint = 0xA0;
pub const CN23XX_CONFIG_PCIE_UNCORRECT_ERR_MASK: c_uint = 0x108;
pub const CN23XX_CONFIG_PCIE_CORRECT_ERR_STATUS: c_uint = 0x110;
pub const CN23XX_CONFIG_PCIE_DEVCTL_MASK: c_uint = 0x00040000;
pub const CN23XX_PCIE_SRIOV_FDL: c_uint = 0x188;
pub const CN23XX_PCIE_SRIOV_FDL_BIT_POS: c_uint = 0x10;
pub const CN23XX_PCIE_SRIOV_FDL_MASK: c_uint = 0xFF;
pub const CN23XX_CONFIG_PCIE_FLTMSK: c_uint = 0x720;
pub const CN23XX_CONFIG_SRIOV_VFDEVID: c_uint = 0x190;
pub const CN23XX_CONFIG_SRIOV_BAR_START: c_uint = 0x19C;

pub const CN23XX_CONFIG_SRIOV_BAR_PF: c_uint = 0x08;
pub const CN23XX_CONFIG_SRIOV_BAR_64BIT: c_uint = 0x04;
pub const CN23XX_CONFIG_SRIOV_BAR_IO: c_uint = 0x01;
// ##############  BAR0 Registers ################
pub const CN23XX_SLI_CTL_PORT_START: c_uint = 0x286E0;
pub const CN23XX_PORT_OFFSET: c_uint = 0x10;

// 2 scatch registers (64-bit)
pub const CN23XX_SLI_WINDOW_CTL: c_uint = 0x282E0;
pub const CN23XX_SLI_SCRATCH1: c_uint = 0x283C0;
pub const CN23XX_SLI_SCRATCH2: c_uint = 0x283D0;
pub const CN23XX_SLI_WINDOW_CTL_DEFAULT: c_uint = 0x200000ULL;
// 1 registers (64-bit)  - SLI_CTL_STATUS
pub const CN23XX_SLI_CTL_STATUS: c_uint = 0x28570;
// SLI Packet Input Jabber Register (64 bit register)
// <31:0> for Byte count for limiting sizes of packet sizes
// that are allowed for sli packet inbound packets.
// the default value is 0xFA00(=64000).
//
pub const CN23XX_SLI_PKT_IN_JABBER: c_uint = 0x29170;
// The input jabber is used to determine the TSO max size.
// Due to H/W limitation, this needs to be reduced to 60000
// in order to use H/W TSO and avoid the WQE malformation
// PKO_BUG_24989_WQE_LEN
//
pub const CN23XX_DEFAULT_INPUT_JABBER: c_uint = 0xEA60 /*60000*/;
pub const CN23XX_WIN_WR_ADDR_LO: c_uint = 0x20000;
pub const CN23XX_WIN_WR_ADDR_HI: c_uint = 0x20004;

pub const CN23XX_WIN_RD_ADDR_LO: c_uint = 0x20010;
pub const CN23XX_WIN_RD_ADDR_HI: c_uint = 0x20014;

pub const CN23XX_WIN_WR_DATA_LO: c_uint = 0x20020;
pub const CN23XX_WIN_WR_DATA_HI: c_uint = 0x20024;

pub const CN23XX_WIN_RD_DATA_LO: c_uint = 0x20040;
pub const CN23XX_WIN_RD_DATA_HI: c_uint = 0x20044;

pub const CN23XX_WIN_WR_MASK_LO: c_uint = 0x20030;
pub const CN23XX_WIN_WR_MASK_HI: c_uint = 0x20034;

pub const CN23XX_SLI_MAC_CREDIT_CNT: c_uint = 0x23D70;
// 4 registers (64-bit) for mapping IOQs to MACs(PEMs)-
// SLI_PKT_MAC(0..3)_PF(0..1)_RINFO
//
pub const CN23XX_SLI_PKT_MAC_RINFO_START64: c_uint = 0x29030;
// 1 register (64-bit) to determine whether IOQs are in reset.
pub const CN23XX_SLI_PKT_IOQ_RING_RST: c_uint = 0x291E0;
// Each Input Queue register is at a 16-byte Offset in BAR0
pub const CN23XX_IQ_OFFSET: c_uint = 0x20000;
pub const CN23XX_MAC_RINFO_OFFSET: c_uint = 0x20;
pub const CN23XX_PF_RINFO_OFFSET: c_uint = 0x10;

// mask for total rings, setting TRS to base

// mask for starting ring number: setting SRN <6:0> = 0x7F

// Starting bit of the TRS field in CN23XX_SLI_PKT_MAC_RINFO64 register
pub const CN23XX_PKT_MAC_CTL_RINFO_TRS_BIT_POS: c_int = 16;
// Starting bit of SRN field in CN23XX_SLI_PKT_MAC_RINFO64 register
pub const CN23XX_PKT_MAC_CTL_RINFO_SRN_BIT_POS: c_int = 0;
// Starting bit of RPVF field in CN23XX_SLI_PKT_MAC_RINFO64 register
pub const CN23XX_PKT_MAC_CTL_RINFO_RPVF_BIT_POS: c_int = 32;
// Starting bit of NVFS field in CN23XX_SLI_PKT_MAC_RINFO64 register
pub const CN23XX_PKT_MAC_CTL_RINFO_NVFS_BIT_POS: c_int = 48;
// ###################### REQUEST QUEUE #########################
// 64 registers for Input Queue Instr Count - SLI_PKT_IN_DONE0_CNTS
pub const CN23XX_SLI_IQ_INSTR_COUNT_START64: c_uint = 0x10040;
// 64 registers for Input Queues Start Addr - SLI_PKT0_INSTR_BADDR
pub const CN23XX_SLI_IQ_BASE_ADDR_START64: c_uint = 0x10010;
// 64 registers for Input Doorbell - SLI_PKT0_INSTR_BAOFF_DBELL
pub const CN23XX_SLI_IQ_DOORBELL_START: c_uint = 0x10020;
// 64 registers for Input Queue size - SLI_PKT0_INSTR_FIFO_RSIZE
pub const CN23XX_SLI_IQ_SIZE_START: c_uint = 0x10030;
// 64 registers (64-bit) - ES, RO, NS, Arbitration for Input Queue Data &
// gather list fetches. SLI_PKT(0..63)_INPUT_CONTROL.
//
pub const CN23XX_SLI_IQ_PKT_CONTROL_START64: c_uint = 0x10000;
// ------- Request Queue Macros ---------

// ------------------ Masks ----------------

// Number of instructions to be read in one MAC read request.
// setting to Max value(4)
//

// Rings per Virtual Function

// These bits[47:44] select the Physical function number within the MAC

// These bits[43:32] select the function number within the PF

// Masks for SLI_PKT_IN_DONE(0..63)_CNTS Register

// ############################ OUTPUT QUEUE #########################
// 64 registers for Output queue control - SLI_PKT(0..63)_OUTPUT_CONTROL
pub const CN23XX_SLI_OQ_PKT_CONTROL_START: c_uint = 0x10050;
// 64 registers for Output queue buffer and info size - SLI_PKT0_OUT_SIZE
pub const CN23XX_SLI_OQ0_BUFF_INFO_SIZE: c_uint = 0x10060;
// 64 registers for Output Queue Start Addr - SLI_PKT0_SLIST_BADDR
pub const CN23XX_SLI_OQ_BASE_ADDR_START64: c_uint = 0x10070;
// 64 registers for Output Queue Packet Credits - SLI_PKT0_SLIST_BAOFF_DBELL
pub const CN23XX_SLI_OQ_PKT_CREDITS_START: c_uint = 0x10080;
// 64 registers for Output Queue size - SLI_PKT0_SLIST_FIFO_RSIZE
pub const CN23XX_SLI_OQ_SIZE_START: c_uint = 0x10090;
// 64 registers for Output Queue Packet Count - SLI_PKT0_CNTS
pub const CN23XX_SLI_OQ_PKT_SENT_START: c_uint = 0x100B0;
// 64 registers for Output Queue INT Levels - SLI_PKT0_INT_LEVELS
pub const CN23XX_SLI_OQ_PKT_INT_LEVELS_START64: c_uint = 0x100A0;
// Each Output Queue register is at a 16-byte Offset in BAR0
pub const CN23XX_OQ_OFFSET: c_uint = 0x20000;
// 1 (64-bit register) for Output Queue backpressure across all rings.
pub const CN23XX_SLI_OQ_WMARK: c_uint = 0x29180;
// Global pkt control register
pub const CN23XX_SLI_GBL_CONTROL: c_uint = 0x29210;
// Backpressure enable register for PF0
pub const CN23XX_SLI_OUT_BP_EN_W1S: c_uint = 0x29260;
// Backpressure enable register for PF1
pub const CN23XX_SLI_OUT_BP_EN2_W1S: c_uint = 0x29270;
// Backpressure disable register for PF0
pub const CN23XX_SLI_OUT_BP_EN_W1C: c_uint = 0x29280;
// Backpressure disable register for PF1
pub const CN23XX_SLI_OUT_BP_EN2_W1C: c_uint = 0x29290;
// ------- Output Queue Macros ---------

// Macro's for accessing CNT and TIME separately from INT_LEVELS

// ------------------ Masks ----------------

// ######################### Mailbox Reg Macros ########################
pub const CN23XX_SLI_PKT_MBOX_INT_START: c_uint = 0x10210;
pub const CN23XX_SLI_PKT_PF_VF_MBOX_SIG_START: c_uint = 0x10200;
pub const CN23XX_SLI_MAC_PF_MBOX_INT_START: c_uint = 0x27380;
pub const CN23XX_SLI_MBOX_OFFSET: c_uint = 0x20000;
pub const CN23XX_SLI_MBOX_SIG_IDX_OFFSET: c_uint = 0x8;

// ######################### DMA Counters #########################
// 2 registers (64-bit) - DMA Count - 1 for each DMA counter 0/1.
pub const CN23XX_DMA_CNT_START: c_uint = 0x28400;
// 2 registers (64-bit) - DMA Timer 0/1, contains DMA timer values
// SLI_DMA_0_TIM
pub const CN23XX_DMA_TIM_START: c_uint = 0x28420;
// 2 registers (64-bit) - DMA count & Time Interrupt threshold -
// SLI_DMA_0_INT_LEVEL
//
pub const CN23XX_DMA_INT_LEVEL_START: c_uint = 0x283E0;
// Each DMA register is at a 16-byte Offset in BAR0
pub const CN23XX_DMA_OFFSET: c_uint = 0x10;
// ---------- DMA Counter Macros ---------

// ######################## MSIX TABLE #########################
pub const CN23XX_MSIX_TABLE_ADDR_START: c_uint = 0x0;
pub const CN23XX_MSIX_TABLE_DATA_START: c_uint = 0x8;
pub const CN23XX_MSIX_TABLE_SIZE: c_uint = 0x10;
pub const CN23XX_MSIX_TABLE_ENTRIES: c_uint = 0x41;

// ######################## INTERRUPTS #########################
pub const CN23XX_MAC_INT_OFFSET: c_uint = 0x20;
pub const CN23XX_PF_INT_OFFSET: c_uint = 0x10;
// 1 register (64-bit) for Interrupt Summary
pub const CN23XX_SLI_INT_SUM64: c_uint = 0x27000;
// 4 registers (64-bit) for Interrupt Enable for each Port
pub const CN23XX_SLI_INT_ENB64: c_uint = 0x27080;

// 1 register (64-bit) to indicate which Output Queue reached pkt threshold
pub const CN23XX_SLI_PKT_CNT_INT: c_uint = 0x29130;
// 1 register (64-bit) to indicate which Output Queue reached time threshold
pub const CN23XX_SLI_PKT_TIME_INT: c_uint = 0x29140;
// ------------------ Interrupt Masks ----------------

// By fault only TIME based

// For both COUNT and TIME based
// #define    CN23XX_INTR_PKT_DATA                  \
// (CN23XX_INTR_PKT_COUNT | CN23XX_INTR_PKT_TIME)
//
// Sum of interrupts for all PCI-Express Data Interrupts

// Sum of interrupts for error events

// Programmed Mask for Interrupt Sum

// 4 Registers (64 - bit)
pub const CN23XX_SLI_S2M_PORT_CTL_START: c_uint = 0x23D80;

pub const CN23XX_SLI_MAC_NUMBER: c_uint = 0x20050;
// PEM(0..3)_BAR1_INDEX(0..15)address is defined as
// addr = (0x00011800C0000100  |port <<24 |idx <<3 )
// Here, port is PEM(0..3) & idx is INDEX(0..15)
//
pub const CN23XX_PEM_BAR1_INDEX_START: c_uint = 0x00011800C0000100ULL;
pub const CN23XX_PEM_OFFSET: c_int = 24;
pub const CN23XX_BAR1_INDEX_OFFSET: c_int = 3;

// ############################ DPI #########################
// 1 register (64-bit) - provides DMA Enable
pub const CN23XX_DPI_CTL: c_uint = 0x0001df0000000040ULL;
// 1 register (64-bit) - Controls the DMA IO Operation
pub const CN23XX_DPI_DMA_CONTROL: c_uint = 0x0001df0000000048ULL;
// 1 register (64-bit) - Provides DMA Instr'n Queue Enable
pub const CN23XX_DPI_REQ_GBL_ENB: c_uint = 0x0001df0000000050ULL;
// 1 register (64-bit) - DPI_REQ_ERR_RSP
// Indicates which Instr'n Queue received error response from the IO sub-system
//
pub const CN23XX_DPI_REQ_ERR_RSP: c_uint = 0x0001df0000000058ULL;
// 1 register (64-bit) - DPI_REQ_ERR_RST
// Indicates which Instr'n Queue dropped an Instr'n
//
pub const CN23XX_DPI_REQ_ERR_RST: c_uint = 0x0001df0000000060ULL;
// 6 register (64-bit) - DPI_DMA_ENG(0..5)_EN
// Provides DMA Engine Queue Enable
//
pub const CN23XX_DPI_DMA_ENG0_ENB: c_uint = 0x0001df0000000080ULL;

// 8 register (64-bit) - DPI_DMA(0..7)_REQQ_CTL
// Provides control bits for transaction on 8 Queues
//
pub const CN23XX_DPI_DMA_REQQ0_CTL: c_uint = 0x0001df0000000180ULL;

// 6 register (64-bit) - DPI_ENG(0..5)_BUF
// Provides DMA Engine FIFO (Queue) Size
//
pub const CN23XX_DPI_DMA_ENG0_BUF: c_uint = 0x0001df0000000880ULL;

// 4 Registers (64-bit)
pub const CN23XX_DPI_SLI_PRT_CFG_START: c_uint = 0x0001df0000000900ULL;

// Masks for DPI_DMA_CONTROL Register

// Set the DMA Control, to update packet count not byte count sent by DMA,
// when we use Interrupt Coalescing (CA mode)
//

// selecting 64-bit Byte Swap Mode

// ############################ RST #########################
pub const CN23XX_RST_BOOT: c_uint = 0x0001180006001600ULL;
pub const CN23XX_RST_SOFT_RST: c_uint = 0x0001180006001680ULL;
pub const CN23XX_LMC0_RESET_CTL: c_uint = 0x0001180088000180ULL;
pub const CN23XX_LMC0_RESET_CTL_DDR3RST_MASK: c_uint = 0x0000000000000001ULL;
