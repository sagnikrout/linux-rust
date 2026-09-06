//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soundwire/sdw_registers.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2015-17 Intel Corporation.

//
// SDW registers as defined by MIPI 1.2 Spec
//

pub const SDW_REG_NO_PAGE: c_uint = 0x00008000;
pub const SDW_REG_OPTIONAL_PAGE: c_uint = 0x00010000;
pub const SDW_REG_MAX: c_uint = 0x48000000;
pub const SDW_DPN_SIZE: c_uint = 0x100;
pub const SDW_BANK1_OFFSET: c_uint = 0x10;
//
// DP0 Interrupt register & bits
//
// Spec treats Status (RO) and Clear (WC) as separate but they are same
// address, so treat as same register with WC.
//
// both INT and STATUS register are same
pub const SDW_DP0_INT: c_uint = 0x0;
pub const SDW_DP0_INTMASK: c_uint = 0x1;
pub const SDW_DP0_PORTCTRL: c_uint = 0x2;
pub const SDW_DP0_BLOCKCTRL1: c_uint = 0x3;
pub const SDW_DP0_PREPARESTATUS: c_uint = 0x4;
pub const SDW_DP0_PREPARECTRL: c_uint = 0x5;

// BIT(4) not allocated in SoundWire specification 1.2

pub const SDW_DP0_CHANNELEN: c_uint = 0x20;
pub const SDW_DP0_SAMPLECTRL1: c_uint = 0x22;
pub const SDW_DP0_SAMPLECTRL2: c_uint = 0x23;
pub const SDW_DP0_OFFSETCTRL1: c_uint = 0x24;
pub const SDW_DP0_OFFSETCTRL2: c_uint = 0x25;
pub const SDW_DP0_HCTRL: c_uint = 0x26;
pub const SDW_DP0_LANECTRL: c_uint = 0x28;
// Both INT and STATUS register are same
pub const SDW_SCP_INT1: c_uint = 0x40;
pub const SDW_SCP_INTMASK1: c_uint = 0x41;

pub const SDW_SCP_INTSTAT2: c_uint = 0x42;

pub const SDW_SCP_INTSTAT3: c_uint = 0x43;

// Number of interrupt status registers
pub const SDW_NUM_INT_STAT_REGISTERS: c_int = 3;
// Number of interrupt clear registers
pub const SDW_NUM_INT_CLEAR_REGISTERS: c_int = 1;
pub const SDW_SCP_CTRL: c_uint = 0x44;

pub const SDW_SCP_STAT: c_uint = 0x44;

pub const SDW_SCP_SYSTEMCTRL: c_uint = 0x45;

pub const SDW_SCP_SYSTEMCTRL_CLK_STP_MODE0: c_int = 0;

pub const SDW_SCP_DEVNUMBER: c_uint = 0x46;
pub const SDW_SCP_HIGH_PHY_CHECK: c_uint = 0x47;
pub const SDW_SCP_ADDRPAGE1: c_uint = 0x48;
pub const SDW_SCP_ADDRPAGE2: c_uint = 0x49;
pub const SDW_SCP_KEEPEREN: c_uint = 0x4A;
pub const SDW_SCP_BANKDELAY: c_uint = 0x4B;
pub const SDW_SCP_COMMIT: c_uint = 0x4C;
pub const SDW_SCP_BUS_CLOCK_BASE: c_uint = 0x4D;

pub const SDW_SCP_BASE_CLOCK_UNKNOWN: c_uint = 0x0;
pub const SDW_SCP_BASE_CLOCK_19200000_HZ: c_uint = 0x1;
pub const SDW_SCP_BASE_CLOCK_24000000_HZ: c_uint = 0x2;
pub const SDW_SCP_BASE_CLOCK_24576000_HZ: c_uint = 0x3;
pub const SDW_SCP_BASE_CLOCK_22579200_HZ: c_uint = 0x4;
pub const SDW_SCP_BASE_CLOCK_32000000_HZ: c_uint = 0x5;
pub const SDW_SCP_BASE_CLOCK_RESERVED: c_uint = 0x6;
pub const SDW_SCP_BASE_CLOCK_IMP_DEF: c_uint = 0x7;
// 0x4E is not allocated in SoundWire specification 1.2
pub const SDW_SCP_TESTMODE: c_uint = 0x4F;
pub const SDW_SCP_DEVID_0: c_uint = 0x50;
pub const SDW_SCP_DEVID_1: c_uint = 0x51;
pub const SDW_SCP_DEVID_2: c_uint = 0x52;
pub const SDW_SCP_DEVID_3: c_uint = 0x53;
pub const SDW_SCP_DEVID_4: c_uint = 0x54;
pub const SDW_SCP_DEVID_5: c_uint = 0x55;
// Both INT and STATUS register are same
pub const SDW_SCP_SDCA_INT1: c_uint = 0x58;

pub const SDW_SCP_SDCA_INT2: c_uint = 0x59;

pub const SDW_SCP_SDCA_INT3: c_uint = 0x5A;

pub const SDW_SCP_SDCA_INT4: c_uint = 0x5B;

// BIT(7) not allocated in SoundWire 1.2 specification
pub const SDW_SCP_SDCA_INTMASK1: c_uint = 0x5C;

pub const SDW_SCP_SDCA_INTMASK2: c_uint = 0x5D;

pub const SDW_SCP_SDCA_INTMASK3: c_uint = 0x5E;

pub const SDW_SCP_SDCA_INTMASK4: c_uint = 0x5F;

// BIT(7) not allocated in SoundWire 1.2 specification
// Banked Registers
pub const SDW_SCP_FRAMECTRL_B0: c_uint = 0x60;

pub const SDW_SCP_NEXTFRAME_B0: c_uint = 0x61;

pub const SDW_SCP_BUSCLOCK_SCALE_B0: c_uint = 0x62;

// PHY registers - CTRL and STAT are the same address
pub const SDW_SCP_PHY_OUT_CTRL_0: c_uint = 0x80;
pub const SDW_SCP_PHY_OUT_CTRL_1: c_uint = 0x81;
pub const SDW_SCP_PHY_OUT_CTRL_2: c_uint = 0x82;
pub const SDW_SCP_PHY_OUT_CTRL_3: c_uint = 0x83;
pub const SDW_SCP_PHY_OUT_CTRL_4: c_uint = 0x84;
pub const SDW_SCP_PHY_OUT_CTRL_5: c_uint = 0x85;
pub const SDW_SCP_PHY_OUT_CTRL_6: c_uint = 0x86;
pub const SDW_SCP_PHY_OUT_CTRL_7: c_uint = 0x87;

// Both INT and STATUS register is same

pub const SDW_NUM_CASC_PORT_INTSTAT1: c_int = 4;
pub const SDW_CASC_PORT_START_INTSTAT1: c_int = 0;
pub const SDW_CASC_PORT_MASK_INTSTAT1: c_uint = 0x8;
pub const SDW_CASC_PORT_REG_OFFSET_INTSTAT1: c_uint = 0x0;
pub const SDW_NUM_CASC_PORT_INTSTAT2: c_int = 7;
pub const SDW_CASC_PORT_START_INTSTAT2: c_int = 4;
pub const SDW_CASC_PORT_MASK_INTSTAT2: c_int = 1;
pub const SDW_CASC_PORT_REG_OFFSET_INTSTAT2: c_int = 1;
pub const SDW_NUM_CASC_PORT_INTSTAT3: c_int = 4;
pub const SDW_CASC_PORT_START_INTSTAT3: c_int = 11;
pub const SDW_CASC_PORT_MASK_INTSTAT3: c_int = 1;
pub const SDW_CASC_PORT_REG_OFFSET_INTSTAT3: c_int = 2;
//
// v1.2 device - SDCA address mapping
//
// Spec definition
// Bits		Contents
// 31		0 (required by addressing range)
// 30:26		0b10000 (Control Prefix)
// 25		0 (Reserved)
// 24:22		Function Number [2:0]
// 21		Entity[6]
// 20:19		Control Selector[5:4]
// 18		0 (Reserved)
// 17:15		Control Number[5:3]
// 14		Next
// 13		MBQ
// 12:7		Entity[5:0]
// 6:3		Control Selector[3:0]
// 2:0		Control Number[2:0]
//

// Check the reserved and fixed bits in address

pub const SDW_SDCA_MAX_REGISTER: c_uint = 0x47FFFFFF;
