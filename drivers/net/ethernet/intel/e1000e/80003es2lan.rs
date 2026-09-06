//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/80003es2lan.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
pub const E1000_KMRNCTRLSTA_OFFSET_FIFO_CTRL: c_uint = 0x00;
pub const E1000_KMRNCTRLSTA_OFFSET_INB_CTRL: c_uint = 0x02;
pub const E1000_KMRNCTRLSTA_OFFSET_HD_CTRL: c_uint = 0x10;
pub const E1000_KMRNCTRLSTA_OFFSET_MAC2PHY_OPMODE: c_uint = 0x1F;
pub const E1000_KMRNCTRLSTA_FIFO_CTRL_RX_BYPASS: c_uint = 0x0008;
pub const E1000_KMRNCTRLSTA_FIFO_CTRL_TX_BYPASS: c_uint = 0x0800;
pub const E1000_KMRNCTRLSTA_INB_CTRL_DIS_PADDING: c_uint = 0x0010;
pub const E1000_KMRNCTRLSTA_HD_CTRL_10_100_DEFAULT: c_uint = 0x0004;
pub const E1000_KMRNCTRLSTA_HD_CTRL_1000_DEFAULT: c_uint = 0x0000;
pub const E1000_KMRNCTRLSTA_OPMODE_E_IDLE: c_uint = 0x2000;
pub const E1000_KMRNCTRLSTA_OPMODE_MASK: c_uint = 0x000C;
pub const E1000_KMRNCTRLSTA_OPMODE_INBAND_MDIO: c_uint = 0x0004;
pub const E1000_TCTL_EXT_GCEX_MASK: c_uint = 0x000FFC00	/* Gig Carry Extend Padding */;
pub const DEFAULT_TCTL_EXT_GCEX_80003ES2LAN: c_uint = 0x00010000;
pub const DEFAULT_TIPG_IPGT_1000_80003ES2LAN: c_uint = 0x8;
pub const DEFAULT_TIPG_IPGT_10_100_80003ES2LAN: c_uint = 0x9;
// GG82563 PHY Specific Status Register (Page 0, Register 16
pub const GG82563_PSCR_POLARITY_REVERSAL_DISABLE: c_uint = 0x0002	/* 1=Reversal Dis */;
pub const GG82563_PSCR_CROSSOVER_MODE_MASK: c_uint = 0x0060;
pub const GG82563_PSCR_CROSSOVER_MODE_MDI: c_uint = 0x0000	/* 00=Manual MDI */;
pub const GG82563_PSCR_CROSSOVER_MODE_MDIX: c_uint = 0x0020	/* 01=Manual MDIX */;
pub const GG82563_PSCR_CROSSOVER_MODE_AUTO: c_uint = 0x0060	/* 11=Auto crossover */;
// PHY Specific Control Register 2 (Page 0, Register 26)
pub const GG82563_PSCR2_REVERSE_AUTO_NEG: c_uint = 0x2000	/* 1=Reverse Auto-Neg */;
// MAC Specific Control Register (Page 2, Register 21)
// Tx clock speed for Link Down and 1000BASE-T for the following speeds
pub const GG82563_MSCR_TX_CLK_MASK: c_uint = 0x0007;
pub const GG82563_MSCR_TX_CLK_10MBPS_2_5: c_uint = 0x0004;
pub const GG82563_MSCR_TX_CLK_100MBPS_25: c_uint = 0x0005;
pub const GG82563_MSCR_TX_CLK_1000MBPS_25: c_uint = 0x0007;
pub const GG82563_MSCR_ASSERT_CRS_ON_TX: c_uint = 0x0010	/* 1=Assert */;
// DSP Distance Register (Page 5, Register 26)
// 0 = <50M
// 1 = 50-80M
// 2 = 80-100M
// 3 = 110-140M
// 4 = >140M
//
pub const GG82563_DSPD_CABLE_LENGTH: c_uint = 0x0007;
// Kumeran Mode Control Register (Page 193, Register 16)
pub const GG82563_KMCR_PASS_FALSE_CARRIER: c_uint = 0x0800;
// Max number of times Kumeran read/write should be validated
pub const GG82563_MAX_KMRN_RETRY: c_uint = 0x5;
// Power Management Control Register (Page 193, Register 20)
// 1=Enable SERDES Electrical Idle
pub const GG82563_PMCR_ENABLE_ELECTRICAL_IDLE: c_uint = 0x0001;
// In-Band Control Register (Page 194, Register 18)
pub const GG82563_ICR_DIS_PADDING: c_uint = 0x0010	/* Disable Padding */;
