//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-platform-isys-csi2-reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2023--2024 Intel Corporation

pub const CSI_REG_BASE: c_uint = 0x220000;

// CSI Port Genral Purpose Registers
pub const CSI_REG_PORT_GPREG_SRST: c_uint = 0x0;
pub const CSI_REG_PORT_GPREG_CSI2_SLV_REG_SRST: c_uint = 0x4;
pub const CSI_REG_PORT_GPREG_CSI2_PORT_CONTROL: c_uint = 0x8;
//
// Port IRQs mapping events:
// IRQ0 - CSI_FE event
// IRQ1 - CSI_SYNC
// IRQ2 - S2M_SIDS0TO7
// IRQ3 - S2M_SIDS8TO15
//
pub const CSI_PORT_REG_BASE_IRQ_CSI: c_uint = 0x80;
pub const CSI_PORT_REG_BASE_IRQ_CSI_SYNC: c_uint = 0xA0;
pub const CSI_PORT_REG_BASE_IRQ_S2M_SIDS0TOS7: c_uint = 0xC0;
pub const CSI_PORT_REG_BASE_IRQ_S2M_SIDS8TOS15: c_uint = 0xE0;
pub const CSI_PORT_REG_BASE_IRQ_EDGE_OFFSET: c_uint = 0x0;
pub const CSI_PORT_REG_BASE_IRQ_MASK_OFFSET: c_uint = 0x4;
pub const CSI_PORT_REG_BASE_IRQ_STATUS_OFFSET: c_uint = 0x8;
pub const CSI_PORT_REG_BASE_IRQ_CLEAR_OFFSET: c_uint = 0xc;
pub const CSI_PORT_REG_BASE_IRQ_ENABLE_OFFSET: c_uint = 0x10;
pub const CSI_PORT_REG_BASE_IRQ_LEVEL_NOT_PULSE_OFFSET: c_uint = 0x14;

pub const CSI_RX_NUM_ERRORS_IN_IRQ: c_int = 20;
pub const CSI_RX_NUM_IRQ: c_int = 32;

// PPI2CSI
pub const CSI_REG_PPI2CSI_ENABLE: c_uint = 0x200;
pub const CSI_REG_PPI2CSI_CONFIG_PPI_INTF: c_uint = 0x204;

pub const CSI_REG_PPI2CSI_CONFIG_CSI_FEATURE: c_uint = 0x208;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSI_PPI2CSI_CTRL {
    CSI_PPI2CSI_DISABLE = 0,
    CSI_PPI2CSI_ENABLE = 1,
}

// CSI_FE
pub const CSI_REG_CSI_FE_ENABLE: c_uint = 0x280;
pub const CSI_REG_CSI_FE_MODE: c_uint = 0x284;
pub const CSI_REG_CSI_FE_MUX_CTRL: c_uint = 0x288;
pub const CSI_REG_CSI_FE_SYNC_CNTR_SEL: c_uint = 0x290;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSI_FE_ENABLE_TYPE {
    CSI_FE_DISABLE = 0,
    CSI_FE_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSI_FE_MODE_TYPE {
    CSI_FE_DPHY_MODE = 0,
    CSI_FE_CPHY_MODE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSI_FE_INPUT_SELECTOR {
    CSI_SENSOR_INPUT = 0,
    CSI_MIPIGEN_INPUT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSI_FE_SYNC_CNTR_SEL_TYPE {
    CSI_CNTR_SENSOR_LINE_ID = BIT(0),
    CSI_CNTR_INT_LINE_PKT_ID = ~CSI_CNTR_SENSOR_LINE_ID,
    CSI_CNTR_SENSOR_FRAME_ID = BIT(1),
    CSI_CNTR_INT_FRAME_PKT_ID = ~CSI_CNTR_SENSOR_FRAME_ID,
}

// CSI HUB General Purpose Registers

pub const CSI_REG_HUB_FW_ACCESS_PORT_OFS: c_uint = 0x17000;
pub const CSI_REG_HUB_FW_ACCESS_PORT_V6OFS: c_uint = 0x16000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CSI_PORT_CLK_GATING_SWITCH {
    CSI_PORT_CLK_GATING_OFF = 0,
    CSI_PORT_CLK_GATING_ON = 1,
}

pub const CSI_REG_BASE_HUB_IRQ: c_uint = 0x18200;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL0_IRQ_EDGE: c_uint = 0x238200;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL0_IRQ_MASK: c_uint = 0x238204;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL0_IRQ_STATUS: c_uint = 0x238208;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL0_IRQ_CLEAR: c_uint = 0x23820c;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL0_IRQ_ENABLE: c_uint = 0x238210;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL0_IRQ_LEVEL_NOT_PULSE: c_uint = 0x238214;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL1_IRQ_EDGE: c_uint = 0x238220;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL1_IRQ_MASK: c_uint = 0x238224;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL1_IRQ_STATUS: c_uint = 0x238228;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL1_IRQ_CLEAR: c_uint = 0x23822c;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL1_IRQ_ENABLE: c_uint = 0x238230;
pub const IPU6_REG_ISYS_CSI_TOP_CTRL1_IRQ_LEVEL_NOT_PULSE: c_uint = 0x238234;
// MTL IPU6V6 irq ctrl0 & ctrl1
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL0_IRQ_EDGE: c_uint = 0x238700;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL0_IRQ_MASK: c_uint = 0x238704;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL0_IRQ_STATUS: c_uint = 0x238708;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL0_IRQ_CLEAR: c_uint = 0x23870c;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL0_IRQ_ENABLE: c_uint = 0x238710;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL0_IRQ_LEVEL_NOT_PULSE: c_uint = 0x238714;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL1_IRQ_EDGE: c_uint = 0x238720;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL1_IRQ_MASK: c_uint = 0x238724;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL1_IRQ_STATUS: c_uint = 0x238728;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL1_IRQ_CLEAR: c_uint = 0x23872c;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL1_IRQ_ENABLE: c_uint = 0x238730;
pub const IPU6V6_REG_ISYS_CSI_TOP_CTRL1_IRQ_LEVEL_NOT_PULSE: c_uint = 0x238734;
//
// 3:0 CSI_PORT.irq_out[3:0] CSI_PORT_CTRL0 IRQ outputs (4bits)
// [0] CSI_PORT.IRQ_CTRL0_csi
// [1] CSI_PORT.IRQ_CTRL1_csi_sync
// [2] CSI_PORT.IRQ_CTRL2_s2m_sids0to7
// [3] CSI_PORT.IRQ_CTRL3_s2m_sids8to15
//

//
// ipu6se support 2 front ends, 2 port per front end, 4 ports 0..3
// sip0 - 0, 1
// sip1 - 2, 3
// 0 and 2 support 4 data lanes, 1 and 3 support 2 data lanes
// all offset are base from isys base address
//

pub const CSI2_HUB_GPREG_DPHY_TIMER_INCR: c_uint = 0x238040;
pub const CSI2_HUB_GPREG_HPLL_FREQ: c_uint = 0x238044;
pub const CSI2_HUB_GPREG_IS_CLK_RATIO: c_uint = 0x238048;
pub const CSI2_HUB_GPREG_HPLL_FREQ_ISCLK_RATE_OVERRIDE: c_uint = 0x23804c;
pub const CSI2_HUB_GPREG_PORT_CLKGATING_DISABLE: c_uint = 0x238058;
pub const CSI2_HUB_GPREG_SIP0_CSI_RX_A_CONTROL: c_uint = 0x23805c;
pub const CSI2_HUB_GPREG_SIP0_CSI_RX_B_CONTROL: c_uint = 0x238088;
pub const CSI2_HUB_GPREG_SIP1_CSI_RX_A_CONTROL: c_uint = 0x2380a4;
pub const CSI2_HUB_GPREG_SIP1_CSI_RX_B_CONTROL: c_uint = 0x2380d0;

// offset from port base
pub const CSI2_SIP_TOP_CSI_RX_PORT_CONTROL: c_uint = 0x0;
pub const CSI2_SIP_TOP_CSI_RX_DLY_CNT_TERMEN_CLANE: c_uint = 0x4;
pub const CSI2_SIP_TOP_CSI_RX_DLY_CNT_SETTLE_CLANE: c_uint = 0x8;

