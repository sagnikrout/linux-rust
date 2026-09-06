//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/cn23xx_vf_regs.h
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
// ! \file cn23xx_vf_regs.h
// \brief Host Driver: Register Address and Register Mask values for
// Octeon CN23XX vf functions.
//
pub const CN23XX_CONFIG_XPANSION_BAR: c_uint = 0x38;
pub const CN23XX_CONFIG_PCIE_CAP: c_uint = 0x70;
pub const CN23XX_CONFIG_PCIE_DEVCAP: c_uint = 0x74;
pub const CN23XX_CONFIG_PCIE_DEVCTL: c_uint = 0x78;
pub const CN23XX_CONFIG_PCIE_LINKCAP: c_uint = 0x7C;
pub const CN23XX_CONFIG_PCIE_LINKCTL: c_uint = 0x80;
pub const CN23XX_CONFIG_PCIE_SLOTCAP: c_uint = 0x84;
pub const CN23XX_CONFIG_PCIE_SLOTCTL: c_uint = 0x88;
pub const CN23XX_CONFIG_PCIE_FLTMSK: c_uint = 0x720;
// The input jabber is used to determine the TSO max size.
// Due to H/W limitation, this needs to be reduced to 60000
// in order to use H/W TSO and avoid the WQE malformation
// PKO_BUG_24989_WQE_LEN
//
pub const CN23XX_DEFAULT_INPUT_JABBER: c_uint = 0xEA60 /*60000*/;
// ##############  BAR0 Registers ################
// Each Input Queue register is at a 16-byte Offset in BAR0
pub const CN23XX_VF_IQ_OFFSET: c_uint = 0x20000;
// ###################### REQUEST QUEUE #########################
// 64 registers for Input Queue Instr Count - SLI_PKT_IN_DONE0_CNTS
pub const CN23XX_VF_SLI_IQ_INSTR_COUNT_START64: c_uint = 0x10040;
// 64 registers for Input Queues Start Addr - SLI_PKT0_INSTR_BADDR
pub const CN23XX_VF_SLI_IQ_BASE_ADDR_START64: c_uint = 0x10010;
// 64 registers for Input Doorbell - SLI_PKT0_INSTR_BAOFF_DBELL
pub const CN23XX_VF_SLI_IQ_DOORBELL_START: c_uint = 0x10020;
// 64 registers for Input Queue size - SLI_PKT0_INSTR_FIFO_RSIZE
pub const CN23XX_VF_SLI_IQ_SIZE_START: c_uint = 0x10030;
// 64 registers (64-bit) - ES, RO, NS, Arbitration for Input Queue Data &
// gather list fetches. SLI_PKT(0..63)_INPUT_CONTROL.
//
pub const CN23XX_VF_SLI_IQ_PKT_CONTROL_START64: c_uint = 0x10000;
// ------- Request Queue Macros ---------

// ------------------ Masks ----------------

// Number of instructions to be read in one MAC read request.
// setting to Max value(4)
//

// Rings per Virtual Function [RO]

// These bits[47:44][RO] give the Physical function number info within the MAC

// These bits[43:32][RO] give the virtual function number info within the PF

// Masks for SLI_PKT_IN_DONE(0..63)_CNTS Register

// ############################ OUTPUT QUEUE #########################
// 64 registers for Output queue control - SLI_PKT(0..63)_OUTPUT_CONTROL
pub const CN23XX_VF_SLI_OQ_PKT_CONTROL_START: c_uint = 0x10050;
// 64 registers for Output queue buffer and info size - SLI_PKT0_OUT_SIZE
pub const CN23XX_VF_SLI_OQ0_BUFF_INFO_SIZE: c_uint = 0x10060;
// 64 registers for Output Queue Start Addr - SLI_PKT0_SLIST_BADDR
pub const CN23XX_VF_SLI_OQ_BASE_ADDR_START64: c_uint = 0x10070;
// 64 registers for Output Queue Packet Credits - SLI_PKT0_SLIST_BAOFF_DBELL
pub const CN23XX_VF_SLI_OQ_PKT_CREDITS_START: c_uint = 0x10080;
// 64 registers for Output Queue size - SLI_PKT0_SLIST_FIFO_RSIZE
pub const CN23XX_VF_SLI_OQ_SIZE_START: c_uint = 0x10090;
// 64 registers for Output Queue Packet Count - SLI_PKT0_CNTS
pub const CN23XX_VF_SLI_OQ_PKT_SENT_START: c_uint = 0x100B0;
// 64 registers for Output Queue INT Levels - SLI_PKT0_INT_LEVELS
pub const CN23XX_VF_SLI_OQ_PKT_INT_LEVELS_START64: c_uint = 0x100A0;
// Each Output Queue register is at a 16-byte Offset in BAR0
pub const CN23XX_VF_OQ_OFFSET: c_uint = 0x20000;
// ------- Output Queue Macros ---------

// Macro's for accessing CNT and TIME separately from INT_LEVELS

// ------------------ Masks ----------------

// ######################### Mailbox Reg Macros ########################
pub const CN23XX_VF_SLI_PKT_MBOX_INT_START: c_uint = 0x10210;
pub const CN23XX_SLI_PKT_PF_VF_MBOX_SIG_START: c_uint = 0x10200;
pub const CN23XX_SLI_MBOX_OFFSET: c_uint = 0x20000;
pub const CN23XX_SLI_MBOX_SIG_IDX_OFFSET: c_uint = 0x8;

// ######################## INTERRUPTS #########################
pub const CN23XX_VF_SLI_INT_SUM_START: c_uint = 0x100D0;

// ------------------ Interrupt Masks ----------------

// ############################ MIO #########################
pub const CN23XX_MIO_PTP_CLOCK_CFG: c_uint = 0x0001070000000f00ULL;
pub const CN23XX_MIO_PTP_CLOCK_LO: c_uint = 0x0001070000000f08ULL;
pub const CN23XX_MIO_PTP_CLOCK_HI: c_uint = 0x0001070000000f10ULL;
pub const CN23XX_MIO_PTP_CLOCK_COMP: c_uint = 0x0001070000000f18ULL;
pub const CN23XX_MIO_PTP_TIMESTAMP: c_uint = 0x0001070000000f20ULL;
pub const CN23XX_MIO_PTP_EVT_CNT: c_uint = 0x0001070000000f28ULL;
pub const CN23XX_MIO_PTP_CKOUT_THRESH_LO: c_uint = 0x0001070000000f30ULL;
pub const CN23XX_MIO_PTP_CKOUT_THRESH_HI: c_uint = 0x0001070000000f38ULL;
pub const CN23XX_MIO_PTP_CKOUT_HI_INCR: c_uint = 0x0001070000000f40ULL;
pub const CN23XX_MIO_PTP_CKOUT_LO_INCR: c_uint = 0x0001070000000f48ULL;
pub const CN23XX_MIO_PTP_PPS_THRESH_LO: c_uint = 0x0001070000000f50ULL;
pub const CN23XX_MIO_PTP_PPS_THRESH_HI: c_uint = 0x0001070000000f58ULL;
pub const CN23XX_MIO_PTP_PPS_HI_INCR: c_uint = 0x0001070000000f60ULL;
pub const CN23XX_MIO_PTP_PPS_LO_INCR: c_uint = 0x0001070000000f68ULL;
// ############################ RST #########################
pub const CN23XX_RST_BOOT: c_uint = 0x0001180006001600ULL;
// ######################## MSIX TABLE #########################
pub const CN23XX_MSIX_TABLE_ADDR_START: c_uint = 0x0;
pub const CN23XX_MSIX_TABLE_DATA_START: c_uint = 0x8;
pub const CN23XX_MSIX_TABLE_SIZE: c_uint = 0x10;
pub const CN23XX_MSIX_TABLE_ENTRIES: c_uint = 0x41;

