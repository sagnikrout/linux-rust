//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77620.h
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
// Defining registers address and its bit definitions of MAX77620 and MAX20024
//
// Copyright (C) 2016 NVIDIA CORPORATION. All rights reserved.
//

// GLOBAL, PMIC, GPIO, FPS, ONOFFC, CID Registers
pub const MAX77620_REG_CNFGGLBL1: c_uint = 0x00;
pub const MAX77620_REG_CNFGGLBL2: c_uint = 0x01;
pub const MAX77620_REG_CNFGGLBL3: c_uint = 0x02;
pub const MAX77620_REG_CNFG1_32K: c_uint = 0x03;
pub const MAX77620_REG_CNFGBBC: c_uint = 0x04;
pub const MAX77620_REG_IRQTOP: c_uint = 0x05;
pub const MAX77620_REG_INTLBT: c_uint = 0x06;
pub const MAX77620_REG_IRQSD: c_uint = 0x07;
pub const MAX77620_REG_IRQ_LVL2_L0_7: c_uint = 0x08;
pub const MAX77620_REG_IRQ_LVL2_L8: c_uint = 0x09;
pub const MAX77620_REG_IRQ_LVL2_GPIO: c_uint = 0x0A;
pub const MAX77620_REG_ONOFFIRQ: c_uint = 0x0B;
pub const MAX77620_REG_NVERC: c_uint = 0x0C;
pub const MAX77620_REG_IRQTOPM: c_uint = 0x0D;
pub const MAX77620_REG_INTENLBT: c_uint = 0x0E;
pub const MAX77620_REG_IRQMASKSD: c_uint = 0x0F;
pub const MAX77620_REG_IRQ_MSK_L0_7: c_uint = 0x10;
pub const MAX77620_REG_IRQ_MSK_L8: c_uint = 0x11;
pub const MAX77620_REG_ONOFFIRQM: c_uint = 0x12;
pub const MAX77620_REG_STATLBT: c_uint = 0x13;
pub const MAX77620_REG_STATSD: c_uint = 0x14;
pub const MAX77620_REG_ONOFFSTAT: c_uint = 0x15;
// SD and LDO Registers
pub const MAX77620_REG_SD0: c_uint = 0x16;
pub const MAX77620_REG_SD1: c_uint = 0x17;
pub const MAX77620_REG_SD2: c_uint = 0x18;
pub const MAX77620_REG_SD3: c_uint = 0x19;
pub const MAX77620_REG_SD4: c_uint = 0x1A;
pub const MAX77620_REG_DVSSD0: c_uint = 0x1B;
pub const MAX77620_REG_DVSSD1: c_uint = 0x1C;
pub const MAX77620_REG_SD0_CFG: c_uint = 0x1D;
pub const MAX77620_REG_SD1_CFG: c_uint = 0x1E;
pub const MAX77620_REG_SD2_CFG: c_uint = 0x1F;
pub const MAX77620_REG_SD3_CFG: c_uint = 0x20;
pub const MAX77620_REG_SD4_CFG: c_uint = 0x21;
pub const MAX77620_REG_SD_CFG2: c_uint = 0x22;
pub const MAX77620_REG_LDO0_CFG: c_uint = 0x23;
pub const MAX77620_REG_LDO0_CFG2: c_uint = 0x24;
pub const MAX77620_REG_LDO1_CFG: c_uint = 0x25;
pub const MAX77620_REG_LDO1_CFG2: c_uint = 0x26;
pub const MAX77620_REG_LDO2_CFG: c_uint = 0x27;
pub const MAX77620_REG_LDO2_CFG2: c_uint = 0x28;
pub const MAX77620_REG_LDO3_CFG: c_uint = 0x29;
pub const MAX77620_REG_LDO3_CFG2: c_uint = 0x2A;
pub const MAX77620_REG_LDO4_CFG: c_uint = 0x2B;
pub const MAX77620_REG_LDO4_CFG2: c_uint = 0x2C;
pub const MAX77620_REG_LDO5_CFG: c_uint = 0x2D;
pub const MAX77620_REG_LDO5_CFG2: c_uint = 0x2E;
pub const MAX77620_REG_LDO6_CFG: c_uint = 0x2F;
pub const MAX77620_REG_LDO6_CFG2: c_uint = 0x30;
pub const MAX77620_REG_LDO7_CFG: c_uint = 0x31;
pub const MAX77620_REG_LDO7_CFG2: c_uint = 0x32;
pub const MAX77620_REG_LDO8_CFG: c_uint = 0x33;
pub const MAX77620_REG_LDO8_CFG2: c_uint = 0x34;
pub const MAX77620_REG_LDO_CFG3: c_uint = 0x35;
pub const MAX77620_LDO_SLEW_RATE_MASK: c_uint = 0x1;
// LDO Configuration 3

pub const MAX77620_TRACK4_SHIFT: c_int = 5;
// Voltage
pub const MAX77620_SDX_VOLT_MASK: c_uint = 0xFF;
pub const MAX77620_SD0_VOLT_MASK: c_uint = 0x3F;
pub const MAX77620_SD1_VOLT_MASK: c_uint = 0x7F;
pub const MAX77620_LDO_VOLT_MASK: c_uint = 0x3F;
pub const MAX77620_REG_GPIO0: c_uint = 0x36;
pub const MAX77620_REG_GPIO1: c_uint = 0x37;
pub const MAX77620_REG_GPIO2: c_uint = 0x38;
pub const MAX77620_REG_GPIO3: c_uint = 0x39;
pub const MAX77620_REG_GPIO4: c_uint = 0x3A;
pub const MAX77620_REG_GPIO5: c_uint = 0x3B;
pub const MAX77620_REG_GPIO6: c_uint = 0x3C;
pub const MAX77620_REG_GPIO7: c_uint = 0x3D;
pub const MAX77620_REG_PUE_GPIO: c_uint = 0x3E;
pub const MAX77620_REG_PDE_GPIO: c_uint = 0x3F;
pub const MAX77620_REG_AME_GPIO: c_uint = 0x40;
pub const MAX77620_REG_ONOFFCNFG1: c_uint = 0x41;
pub const MAX77620_REG_ONOFFCNFG2: c_uint = 0x42;
// FPS Registers
pub const MAX77620_REG_FPS_CFG0: c_uint = 0x43;
pub const MAX77620_REG_FPS_CFG1: c_uint = 0x44;
pub const MAX77620_REG_FPS_CFG2: c_uint = 0x45;
pub const MAX77620_REG_FPS_LDO0: c_uint = 0x46;
pub const MAX77620_REG_FPS_LDO1: c_uint = 0x47;
pub const MAX77620_REG_FPS_LDO2: c_uint = 0x48;
pub const MAX77620_REG_FPS_LDO3: c_uint = 0x49;
pub const MAX77620_REG_FPS_LDO4: c_uint = 0x4A;
pub const MAX77620_REG_FPS_LDO5: c_uint = 0x4B;
pub const MAX77620_REG_FPS_LDO6: c_uint = 0x4C;
pub const MAX77620_REG_FPS_LDO7: c_uint = 0x4D;
pub const MAX77620_REG_FPS_LDO8: c_uint = 0x4E;
pub const MAX77620_REG_FPS_SD0: c_uint = 0x4F;
pub const MAX77620_REG_FPS_SD1: c_uint = 0x50;
pub const MAX77620_REG_FPS_SD2: c_uint = 0x51;
pub const MAX77620_REG_FPS_SD3: c_uint = 0x52;
pub const MAX77620_REG_FPS_SD4: c_uint = 0x53;
pub const MAX77620_REG_FPS_NONE: c_int = 0;
pub const MAX77620_FPS_SRC_MASK: c_uint = 0xC0;
pub const MAX77620_FPS_SRC_SHIFT: c_int = 6;
pub const MAX77620_FPS_PU_PERIOD_MASK: c_uint = 0x38;
pub const MAX77620_FPS_PU_PERIOD_SHIFT: c_int = 3;
pub const MAX77620_FPS_PD_PERIOD_MASK: c_uint = 0x07;
pub const MAX77620_FPS_PD_PERIOD_SHIFT: c_int = 0;
pub const MAX77620_FPS_TIME_PERIOD_MASK: c_uint = 0x38;
pub const MAX77620_FPS_TIME_PERIOD_SHIFT: c_int = 3;
pub const MAX77620_FPS_EN_SRC_MASK: c_uint = 0x06;
pub const MAX77620_FPS_EN_SRC_SHIFT: c_int = 1;
pub const MAX77620_FPS_ENFPS_SW_MASK: c_uint = 0x01;
pub const MAX77620_FPS_ENFPS_SW: c_uint = 0x01;
// Minimum and maximum FPS period time (in microseconds) are
// different for MAX77620 and Max20024.
//
pub const MAX77620_FPS_PERIOD_MIN_US: c_int = 40;
pub const MAX20024_FPS_PERIOD_MIN_US: c_int = 20;
pub const MAX20024_FPS_PERIOD_MAX_US: c_int = 2560;
pub const MAX77620_FPS_PERIOD_MAX_US: c_int = 5120;
pub const MAX77620_REG_FPS_GPIO1: c_uint = 0x54;
pub const MAX77620_REG_FPS_GPIO2: c_uint = 0x55;
pub const MAX77620_REG_FPS_GPIO3: c_uint = 0x56;
pub const MAX77620_REG_FPS_RSO: c_uint = 0x57;
pub const MAX77620_REG_CID0: c_uint = 0x58;
pub const MAX77620_REG_CID1: c_uint = 0x59;
pub const MAX77620_REG_CID2: c_uint = 0x5A;
pub const MAX77620_REG_CID3: c_uint = 0x5B;
pub const MAX77620_REG_CID4: c_uint = 0x5C;
pub const MAX77620_REG_CID5: c_uint = 0x5D;
pub const MAX77620_REG_DVSSD4: c_uint = 0x5E;
pub const MAX20024_REG_MAX_ADD: c_uint = 0x70;
pub const MAX77620_CID_DIDM_MASK: c_uint = 0xF0;
pub const MAX77620_CID_DIDM_SHIFT: c_int = 4;
// CNCG2SD

// Device Identification Metal

// Device Indentification OTP

// SD CNFG1
pub const MAX77620_SD_SR_MASK: c_uint = 0xC0;
pub const MAX77620_SD_SR_SHIFT: c_int = 6;
pub const MAX77620_SD_POWER_MODE_MASK: c_uint = 0x30;
pub const MAX77620_SD_POWER_MODE_SHIFT: c_int = 4;

pub const MAX77620_SD_CFG1_ADE_DISABLE: c_int = 0;

pub const MAX77620_SD_FPWM_MASK: c_uint = 0x04;
pub const MAX77620_SD_FPWM_SHIFT: c_int = 2;
pub const MAX77620_SD_FSRADE_MASK: c_uint = 0x01;
pub const MAX77620_SD_FSRADE_SHIFT: c_int = 0;

pub const MAX77620_SD_CFG1_FPWM_SD_SKIP: c_int = 0;

pub const MAX77620_SD_CFG1_FSRADE_SD_DISABLE: c_int = 0;

// LDO_CNFG2
pub const MAX77620_LDO_POWER_MODE_MASK: c_uint = 0xC0;
pub const MAX77620_LDO_POWER_MODE_SHIFT: c_int = 6;

pub const MAX77620_LDO_CFG2_ADE_DISABLE: c_int = 0;

pub const MAX77620_LDO_CFG2_SS_SLOW: c_int = 0;

pub const MAX77620_PWR_I2C_ADDR: c_uint = 0x3c;
pub const MAX77620_RTC_I2C_ADDR: c_uint = 0x68;

pub const MAX77620_CNFG_GPIO_DRV_OPENDRAIN: c_int = 0;

pub const MAX77620_CNFG_GPIO_DIR_OUTPUT: c_int = 0;

pub const MAX77620_CNFG_GPIO_OUTPUT_VAL_LOW: c_int = 0;

pub const MAX77620_ONOFFCNFG1_MRT_MASK: c_uint = 0x38;
pub const MAX77620_ONOFFCNFG1_MRT_SHIFT: c_uint = 0x3;

pub const MAX20024_ONOFFCNFG1_CLRSE: c_uint = 0x18;

pub const MAX77620_WDTC_MASK: c_uint = 0x3;

pub const MAX77620_TWD_MASK: c_uint = 0x3;
pub const MAX77620_TWD_2s: c_uint = 0x0;
pub const MAX77620_TWD_16s: c_uint = 0x1;
pub const MAX77620_TWD_64s: c_uint = 0x2;
pub const MAX77620_TWD_128s: c_uint = 0x3;

pub const MAX77620_CNFGGLBL1_LBDAC: c_uint = 0x0E;

// CNFG BBC registers

pub const MAX77620_CNFGBBC_CURRENT_MASK: c_uint = 0x06;
pub const MAX77620_CNFGBBC_CURRENT_SHIFT: c_int = 1;
pub const MAX77620_CNFGBBC_VOLTAGE_MASK: c_uint = 0x18;
pub const MAX77620_CNFGBBC_VOLTAGE_SHIFT: c_int = 3;

pub const MAX77620_CNFGBBC_RESISTOR_MASK: c_uint = 0xC0;
pub const MAX77620_CNFGBBC_RESISTOR_SHIFT: c_int = 6;
pub const MAX77620_FPS_COUNT: c_int = 3;
// Interrupts
// GPIOs
// FPS Source
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77620_fps_src {
    MAX77620_FPS_SRC_0,
    MAX77620_FPS_SRC_1,
    MAX77620_FPS_SRC_2,
    MAX77620_FPS_SRC_NONE,
    MAX77620_FPS_SRC_DEF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77620_chip_id {
    MAX77620,
    MAX20024,
    MAX77663,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77620_chip {
    pub dev: *mut device,
    pub rmap: *mut regmap,
    pub chip_irq: c_int,
// chip id
    pub chip_id: max77620_chip_id,
    pub sleep_enable: bool,
    pub enable_global_lpm: bool,
    pub shutdown_fps_period: [c_int; MAX77620_FPS_COUNT],
    pub suspend_fps_period: [c_int; MAX77620_FPS_COUNT],
    pub top_irq_data: *mut regmap_irq_chip_data,
    pub gpio_irq_data: *mut regmap_irq_chip_data,
}
