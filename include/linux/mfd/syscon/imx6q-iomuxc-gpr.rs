//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/syscon/imx6q-iomuxc-gpr.h
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
//
// Copyright (C) 2012 Freescale Semiconductor, Inc.
//

pub const IOMUXC_GPR0: c_uint = 0x00;
pub const IOMUXC_GPR1: c_uint = 0x04;
pub const IOMUXC_GPR2: c_uint = 0x08;
pub const IOMUXC_GPR3: c_uint = 0x0c;
pub const IOMUXC_GPR4: c_uint = 0x10;
pub const IOMUXC_GPR5: c_uint = 0x14;
pub const IOMUXC_GPR6: c_uint = 0x18;
pub const IOMUXC_GPR7: c_uint = 0x1c;
pub const IOMUXC_GPR8: c_uint = 0x20;
pub const IOMUXC_GPR9: c_uint = 0x24;
pub const IOMUXC_GPR10: c_uint = 0x28;
pub const IOMUXC_GPR11: c_uint = 0x2c;
pub const IOMUXC_GPR12: c_uint = 0x30;
pub const IOMUXC_GPR13: c_uint = 0x34;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL7_SPDIF: c_uint = 0x0;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL6_ESAI: c_uint = 0x0;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL5_ECSPI4: c_uint = 0x0;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL4_ECSPI4: c_uint = 0x0;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL3_ECSPI2: c_uint = 0x0;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL2_ECSPI1: c_uint = 0x0;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL1_ECSPI1: c_uint = 0x0;

pub const IMX6Q_GPR0_DMAREQ_MUX_SEL0_IPU1: c_uint = 0x0;

pub const IMX6Q_GPR1_EXC_MON_OKAY: c_uint = 0x0;

pub const IMX6Q_GPR1_ENET_CLK_SEL_PAD: c_int = 0;

pub const IMX6Q_GPR1_MIPI_IPU2_MUX_GASKET: c_uint = 0x0;

pub const IMX6Q_GPR1_MIPI_IPU1_MUX_GASKET: c_uint = 0x0;

pub const IMX6Q_GPR1_IPU_VPU_MUX_IPU1: c_uint = 0x0;

pub const IMX6Q_GPR1_USB_OTG_ID_SEL_ENET_RX_ER: c_uint = 0x0;

pub const IMX6Q_GPR2_BGREF_RRMODE_EXT_RESISTOR: c_uint = 0x0;

pub const IMX6Q_GPR2_DI1_VS_POLARITY_ACTIVE_H: c_uint = 0x0;

pub const IMX6Q_GPR2_DI0_VS_POLARITY_ACTIVE_H: c_uint = 0x0;

pub const IMX6Q_GPR2_BIT_MAPPING_CH1_SPWG: c_uint = 0x0;

pub const IMX6Q_GPR2_DATA_WIDTH_CH1_18BIT: c_uint = 0x0;

pub const IMX6Q_GPR2_BIT_MAPPING_CH0_SPWG: c_uint = 0x0;

pub const IMX6Q_GPR2_DATA_WIDTH_CH0_18BIT: c_uint = 0x0;

pub const IMX6Q_GPR3_MIPI_MUX_CTL_SHIFT: c_int = 4;

pub const IMX6Q_GPR3_HDMI_MUX_CTL_SHIFT: c_int = 2;

pub const IMX6Q_GPR4_SOC_VERSION_OFF: c_uint = 0x8;

pub const IMX6Q_GPR10_SEC_ERR_RESP_OKEY: c_uint = 0x0;

pub const IMX6Q_GPR13_SATA_SPD_MODE_1P5G: c_uint = 0x0;

// For imx6sl iomux gpr register field define

// For imx6sx iomux gpr register field define

// For imx6ul iomux gpr register field define

// For imx6sll iomux gpr register field define

