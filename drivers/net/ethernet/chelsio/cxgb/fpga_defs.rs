//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb/fpga_defs.h
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
// $Date: 2005/03/07 23:59:05 $ $RCSfile: fpga_defs.h,v $ $Revision: 1.4 $
//
// FPGA specific definitions
//
pub const FPGA_PCIX_ADDR_VERSION: c_uint = 0xA08;
pub const FPGA_PCIX_ADDR_STAT: c_uint = 0xA0C;
// FPGA master interrupt Cause/Enable bits
pub const FPGA_PCIX_INTERRUPT_SGE_ERROR: c_uint = 0x1;
pub const FPGA_PCIX_INTERRUPT_SGE_DATA: c_uint = 0x2;
pub const FPGA_PCIX_INTERRUPT_TP: c_uint = 0x4;
pub const FPGA_PCIX_INTERRUPT_MC3: c_uint = 0x8;
pub const FPGA_PCIX_INTERRUPT_GMAC: c_uint = 0x10;
pub const FPGA_PCIX_INTERRUPT_PCIX: c_uint = 0x20;
// TP interrupt register addresses
pub const FPGA_TP_ADDR_INTERRUPT_ENABLE: c_uint = 0xA10;
pub const FPGA_TP_ADDR_INTERRUPT_CAUSE: c_uint = 0xA14;
pub const FPGA_TP_ADDR_VERSION: c_uint = 0xA18;
// TP interrupt Cause/Enable bits
pub const FPGA_TP_INTERRUPT_MC4: c_uint = 0x1;
pub const FPGA_TP_INTERRUPT_MC5: c_uint = 0x2;
//
// PM interrupt register addresses
//
pub const FPGA_MC3_REG_INTRENABLE: c_uint = 0xA20;
pub const FPGA_MC3_REG_INTRCAUSE: c_uint = 0xA24;
pub const FPGA_MC3_REG_VERSION: c_uint = 0xA28;
//
// GMAC interrupt register addresses
//
pub const FPGA_GMAC_ADDR_INTERRUPT_ENABLE: c_uint = 0xA30;
pub const FPGA_GMAC_ADDR_INTERRUPT_CAUSE: c_uint = 0xA34;
pub const FPGA_GMAC_ADDR_VERSION: c_uint = 0xA38;
// GMAC Cause/Enable bits
pub const FPGA_GMAC_INTERRUPT_PORT0: c_uint = 0x1;
pub const FPGA_GMAC_INTERRUPT_PORT1: c_uint = 0x2;
pub const FPGA_GMAC_INTERRUPT_PORT2: c_uint = 0x4;
pub const FPGA_GMAC_INTERRUPT_PORT3: c_uint = 0x8;
// MI0 registers
pub const A_MI0_CLK: c_uint = 0xb00;
pub const S_MI0_CLK_DIV: c_int = 0;
pub const M_MI0_CLK_DIV: c_uint = 0xff;

pub const S_MI0_CLK_CNT: c_int = 8;
pub const M_MI0_CLK_CNT: c_uint = 0xff;

pub const A_MI0_CSR: c_uint = 0xb04;
pub const S_MI0_CSR_POLL: c_int = 0;

pub const S_MI0_PREAMBLE: c_int = 1;

pub const S_MI0_INTR_ENABLE: c_int = 2;

pub const S_MI0_BUSY: c_int = 3;

pub const S_MI0_MDIO: c_int = 4;

pub const A_MI0_ADDR: c_uint = 0xb08;
pub const S_MI0_PHY_REG_ADDR: c_int = 0;
pub const M_MI0_PHY_REG_ADDR: c_uint = 0x1f;

pub const S_MI0_PHY_ADDR: c_int = 5;
pub const M_MI0_PHY_ADDR: c_uint = 0x1f;

pub const A_MI0_DATA_EXT: c_uint = 0xb0c;
pub const A_MI0_DATA_INT: c_uint = 0xb10;
// GMAC registers
pub const A_GMAC_MACID_LO: c_uint = 0x28;
pub const A_GMAC_MACID_HI: c_uint = 0x2c;
pub const A_GMAC_CSR: c_uint = 0x30;
pub const S_INTERFACE: c_int = 0;
pub const M_INTERFACE: c_uint = 0x3;

pub const S_MAC_TX_ENABLE: c_int = 2;

pub const S_MAC_RX_ENABLE: c_int = 3;

pub const S_MAC_LB_ENABLE: c_int = 4;

pub const S_MAC_SPEED: c_int = 5;
pub const M_MAC_SPEED: c_uint = 0x3;

pub const S_MAC_HD_FC_ENABLE: c_int = 7;

pub const S_MAC_HALF_DUPLEX: c_int = 8;

pub const S_MAC_PROMISC: c_int = 9;

pub const S_MAC_MC_ENABLE: c_int = 10;

pub const S_MAC_RESET: c_int = 11;

pub const S_MAC_RX_PAUSE_ENABLE: c_int = 12;

pub const S_MAC_TX_PAUSE_ENABLE: c_int = 13;

pub const S_MAC_LWM_ENABLE: c_int = 14;

pub const S_MAC_MAGIC_PKT_ENABLE: c_int = 15;

pub const S_MAC_ISL_ENABLE: c_int = 16;

pub const S_MAC_JUMBO_ENABLE: c_int = 17;

pub const S_MAC_RX_PAD_ENABLE: c_int = 18;

pub const S_MAC_RX_CRC_ENABLE: c_int = 19;

pub const A_GMAC_IFS: c_uint = 0x34;
pub const S_MAC_IFS2: c_int = 0;
pub const M_MAC_IFS2: c_uint = 0x3f;

pub const S_MAC_IFS1: c_int = 8;
pub const M_MAC_IFS1: c_uint = 0x7f;

pub const A_GMAC_JUMBO_FRAME_LEN: c_uint = 0x38;
pub const A_GMAC_LNK_DLY: c_uint = 0x3c;
pub const A_GMAC_PAUSETIME: c_uint = 0x40;
pub const A_GMAC_MCAST_LO: c_uint = 0x44;
pub const A_GMAC_MCAST_HI: c_uint = 0x48;
pub const A_GMAC_MCAST_MASK_LO: c_uint = 0x4c;
pub const A_GMAC_MCAST_MASK_HI: c_uint = 0x50;
pub const A_GMAC_RMT_CNT: c_uint = 0x54;
pub const A_GMAC_RMT_DATA: c_uint = 0x58;
pub const A_GMAC_BACKOFF_SEED: c_uint = 0x5c;
pub const A_GMAC_TXF_THRES: c_uint = 0x60;
pub const S_TXF_READ_THRESHOLD: c_int = 0;
pub const M_TXF_READ_THRESHOLD: c_uint = 0xff;

pub const S_TXF_WRITE_THRESHOLD: c_int = 16;
pub const M_TXF_WRITE_THRESHOLD: c_uint = 0xff;

pub const MAC_REG_BASE: c_uint = 0x600;

