//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/da9210-regulator.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// da9210-regulator.h - Regulator definitions for DA9210
// Copyright (C) 2013  Dialog Semiconductor Ltd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9210_pdata {
    pub da9210_constraints: regulator_init_data,
}

// Page selection
pub const DA9210_REG_PAGE_CON: c_uint = 0x00;
// System Control and Event Registers
pub const DA9210_REG_STATUS_A: c_uint = 0x50;
pub const DA9210_REG_STATUS_B: c_uint = 0x51;
pub const DA9210_REG_EVENT_A: c_uint = 0x52;
pub const DA9210_REG_EVENT_B: c_uint = 0x53;
pub const DA9210_REG_MASK_A: c_uint = 0x54;
pub const DA9210_REG_MASK_B: c_uint = 0x55;
pub const DA9210_REG_CONTROL_A: c_uint = 0x56;
// GPIO Control Registers
pub const DA9210_REG_GPIO_0_1: c_uint = 0x58;
pub const DA9210_REG_GPIO_2_3: c_uint = 0x59;
pub const DA9210_REG_GPIO_4_5: c_uint = 0x5A;
pub const DA9210_REG_GPIO_6: c_uint = 0x5B;
// Regulator Registers
pub const DA9210_REG_BUCK_CONT: c_uint = 0x5D;
pub const DA9210_REG_BUCK_ILIM: c_uint = 0xD0;
pub const DA9210_REG_BUCK_CONF1: c_uint = 0xD1;
pub const DA9210_REG_BUCK_CONF2: c_uint = 0xD2;
pub const DA9210_REG_VBACK_AUTO: c_uint = 0xD4;
pub const DA9210_REG_VBACK_BASE: c_uint = 0xD5;
pub const DA9210_REG_VBACK_MAX_DVC_IF: c_uint = 0xD6;
pub const DA9210_REG_VBACK_DVC: c_uint = 0xD7;
pub const DA9210_REG_VBUCK_A: c_uint = 0xD8;
pub const DA9210_REG_VBUCK_B: c_uint = 0xD9;
// I2C Interface Settings
pub const DA9210_REG_INTERFACE: c_uint = 0x105;
// OTP
pub const DA9210_REG_OPT_COUNT: c_uint = 0x140;
pub const DA9210_REG_OPT_ADDR: c_uint = 0x141;
pub const DA9210_REG_OPT_DATA: c_uint = 0x142;
// Customer Trim and Configuration
pub const DA9210_REG_CONFIG_A: c_uint = 0x143;
pub const DA9210_REG_CONFIG_B: c_uint = 0x144;
pub const DA9210_REG_CONFIG_C: c_uint = 0x145;
pub const DA9210_REG_CONFIG_D: c_uint = 0x146;
pub const DA9210_REG_CONFIG_E: c_uint = 0x147;
//
// Registers bits
//
// DA9210_REG_PAGE_CON (addr=0x00)
pub const DA9210_PEG_PAGE_SHIFT: c_int = 0;
pub const DA9210_REG_PAGE_MASK: c_uint = 0x0F;
// On I2C registers 0x00 - 0xFF
pub const DA9210_REG_PAGE0: c_int = 0;
// On I2C registers 0x100 - 0x1FF
pub const DA9210_REG_PAGE2: c_int = 2;
pub const DA9210_PAGE_WRITE_MODE: c_uint = 0x00;
pub const DA9210_REPEAT_WRITE_MODE: c_uint = 0x40;
pub const DA9210_PAGE_REVERT: c_uint = 0x80;
// DA9210_REG_STATUS_A (addr=0x50)
pub const DA9210_GPI0: c_uint = 0x01;
pub const DA9210_GPI1: c_uint = 0x02;
pub const DA9210_GPI2: c_uint = 0x04;
pub const DA9210_GPI3: c_uint = 0x08;
pub const DA9210_GPI4: c_uint = 0x10;
pub const DA9210_GPI5: c_uint = 0x20;
pub const DA9210_GPI6: c_uint = 0x40;
// DA9210_REG_EVENT_A (addr=0x52)
pub const DA9210_E_GPI0: c_uint = 0x01;
pub const DA9210_E_GPI1: c_uint = 0x02;
pub const DA9210_E_GPI2: c_uint = 0x04;
pub const DA9210_E_GPI3: c_uint = 0x08;
pub const DA9210_E_GPI4: c_uint = 0x10;
pub const DA9210_E_GPI5: c_uint = 0x20;
pub const DA9210_E_GPI6: c_uint = 0x40;
// DA9210_REG_EVENT_B (addr=0x53)
pub const DA9210_E_OVCURR: c_uint = 0x01;
pub const DA9210_E_NPWRGOOD: c_uint = 0x02;
pub const DA9210_E_TEMP_WARN: c_uint = 0x04;
pub const DA9210_E_TEMP_CRIT: c_uint = 0x08;
pub const DA9210_E_VMAX: c_uint = 0x10;
// DA9210_REG_MASK_A (addr=0x54)
pub const DA9210_M_GPI0: c_uint = 0x01;
pub const DA9210_M_GPI1: c_uint = 0x02;
pub const DA9210_M_GPI2: c_uint = 0x04;
pub const DA9210_M_GPI3: c_uint = 0x08;
pub const DA9210_M_GPI4: c_uint = 0x10;
pub const DA9210_M_GPI5: c_uint = 0x20;
pub const DA9210_M_GPI6: c_uint = 0x40;
// DA9210_REG_MASK_B (addr=0x55)
pub const DA9210_M_OVCURR: c_uint = 0x01;
pub const DA9210_M_NPWRGOOD: c_uint = 0x02;
pub const DA9210_M_TEMP_WARN: c_uint = 0x04;
pub const DA9210_M_TEMP_CRIT: c_uint = 0x08;
pub const DA9210_M_VMAX: c_uint = 0x10;
// DA9210_REG_CONTROL_A (addr=0x56)
pub const DA9210_DEBOUNCING_SHIFT: c_int = 0;
pub const DA9210_DEBOUNCING_MASK: c_uint = 0x07;
pub const DA9210_SLEW_RATE_SHIFT: c_int = 3;
pub const DA9210_SLEW_RATE_MASK: c_uint = 0x18;
pub const DA9210_V_LOCK: c_uint = 0x20;
// DA9210_REG_GPIO_0_1 (addr=0x58)
pub const DA9210_GPIO0_PIN_SHIFT: c_int = 0;
pub const DA9210_GPIO0_PIN_MASK: c_uint = 0x03;
pub const DA9210_GPIO0_PIN_GPI: c_uint = 0x00;
pub const DA9210_GPIO0_PIN_GPO_OD: c_uint = 0x02;
pub const DA9210_GPIO0_PIN_GPO: c_uint = 0x03;
pub const DA9210_GPIO0_TYPE: c_uint = 0x04;
pub const DA9210_GPIO0_TYPE_GPI: c_uint = 0x00;
pub const DA9210_GPIO0_TYPE_GPO: c_uint = 0x04;
pub const DA9210_GPIO0_MODE: c_uint = 0x08;
pub const DA9210_GPIO1_PIN_SHIFT: c_int = 4;
pub const DA9210_GPIO1_PIN_MASK: c_uint = 0x30;
pub const DA9210_GPIO1_PIN_GPI: c_uint = 0x00;
pub const DA9210_GPIO1_PIN_VERROR: c_uint = 0x10;
pub const DA9210_GPIO1_PIN_GPO_OD: c_uint = 0x20;
pub const DA9210_GPIO1_PIN_GPO: c_uint = 0x30;
pub const DA9210_GPIO1_TYPE_SHIFT: c_uint = 0x40;
pub const DA9210_GPIO1_TYPE_GPI: c_uint = 0x00;
pub const DA9210_GPIO1_TYPE_GPO: c_uint = 0x40;
pub const DA9210_GPIO1_MODE: c_uint = 0x80;
// DA9210_REG_GPIO_2_3 (addr=0x59)
pub const DA9210_GPIO2_PIN_SHIFT: c_int = 0;
pub const DA9210_GPIO2_PIN_MASK: c_uint = 0x03;
pub const DA9210_GPIO2_PIN_GPI: c_uint = 0x00;
pub const DA9210_GPIO5_PIN_BUCK_CLK: c_uint = 0x10;
pub const DA9210_GPIO2_PIN_GPO_OD: c_uint = 0x02;
pub const DA9210_GPIO2_PIN_GPO: c_uint = 0x03;
pub const DA9210_GPIO2_TYPE: c_uint = 0x04;
pub const DA9210_GPIO2_TYPE_GPI: c_uint = 0x00;
pub const DA9210_GPIO2_TYPE_GPO: c_uint = 0x04;
pub const DA9210_GPIO2_MODE: c_uint = 0x08;
pub const DA9210_GPIO3_PIN_SHIFT: c_int = 4;
pub const DA9210_GPIO3_PIN_MASK: c_uint = 0x30;
pub const DA9210_GPIO3_PIN_GPI: c_uint = 0x00;
pub const DA9210_GPIO3_PIN_IERROR: c_uint = 0x10;
pub const DA9210_GPIO3_PIN_GPO_OD: c_uint = 0x20;
pub const DA9210_GPIO3_PIN_GPO: c_uint = 0x30;
pub const DA9210_GPIO3_TYPE_SHIFT: c_uint = 0x40;
pub const DA9210_GPIO3_TYPE_GPI: c_uint = 0x00;
pub const DA9210_GPIO3_TYPE_GPO: c_uint = 0x40;
pub const DA9210_GPIO3_MODE: c_uint = 0x80;
// DA9210_REG_GPIO_4_5 (addr=0x5A)
pub const DA9210_GPIO4_PIN_SHIFT: c_int = 0;
pub const DA9210_GPIO4_PIN_MASK: c_uint = 0x03;
pub const DA9210_GPIO4_PIN_GPI: c_uint = 0x00;
pub const DA9210_GPIO4_PIN_GPO_OD: c_uint = 0x02;
pub const DA9210_GPIO4_PIN_GPO: c_uint = 0x03;
pub const DA9210_GPIO4_TYPE: c_uint = 0x04;
pub const DA9210_GPIO4_TYPE_GPI: c_uint = 0x00;
pub const DA9210_GPIO4_TYPE_GPO: c_uint = 0x04;
pub const DA9210_GPIO4_MODE: c_uint = 0x08;
pub const DA9210_GPIO5_PIN_SHIFT: c_int = 4;
pub const DA9210_GPIO5_PIN_MASK: c_uint = 0x30;
pub const DA9210_GPIO5_PIN_GPI: c_uint = 0x00;
pub const DA9210_GPIO5_PIN_INTERFACE: c_uint = 0x01;
pub const DA9210_GPIO5_PIN_GPO_OD: c_uint = 0x20;
pub const DA9210_GPIO5_PIN_GPO: c_uint = 0x30;
pub const DA9210_GPIO5_TYPE_SHIFT: c_uint = 0x40;
pub const DA9210_GPIO5_TYPE_GPI: c_uint = 0x00;
pub const DA9210_GPIO5_TYPE_GPO: c_uint = 0x40;
pub const DA9210_GPIO5_MODE: c_uint = 0x80;
// DA9210_REG_GPIO_6 (addr=0x5B)
pub const DA9210_GPIO6_PIN_SHIFT: c_int = 0;
pub const DA9210_GPIO6_PIN_MASK: c_uint = 0x03;
pub const DA9210_GPIO6_PIN_GPI: c_uint = 0x00;
pub const DA9210_GPIO6_PIN_INTERFACE: c_uint = 0x01;
pub const DA9210_GPIO6_PIN_GPO_OD: c_uint = 0x02;
pub const DA9210_GPIO6_PIN_GPO: c_uint = 0x03;
pub const DA9210_GPIO6_TYPE: c_uint = 0x04;
pub const DA9210_GPIO6_TYPE_GPI: c_uint = 0x00;
pub const DA9210_GPIO6_TYPE_GPO: c_uint = 0x04;
pub const DA9210_GPIO6_MODE: c_uint = 0x08;
// DA9210_REG_BUCK_CONT (addr=0x5D)
pub const DA9210_BUCK_EN: c_uint = 0x01;
pub const DA9210_BUCK_GPI_SHIFT: c_int = 1;
pub const DA9210_BUCK_GPI_MASK: c_uint = 0x06;
pub const DA9210_BUCK_GPI_OFF: c_uint = 0x00;
pub const DA9210_BUCK_GPI_GPIO0: c_uint = 0x02;
pub const DA9210_BUCK_GPI_GPIO3: c_uint = 0x04;
pub const DA9210_BUCK_GPI_GPIO4: c_uint = 0x06;
pub const DA9210_BUCK_PD_DIS: c_uint = 0x08;
pub const DA9210_VBUCK_SEL: c_uint = 0x10;
pub const DA9210_VBUCK_SEL_A: c_uint = 0x00;
pub const DA9210_VBUCK_SEL_B: c_uint = 0x10;
pub const DA9210_VBUCK_GPI_SHIFT: c_int = 5;
pub const DA9210_VBUCK_GPI_MASK: c_uint = 0x60;
pub const DA9210_VBUCK_GPI_OFF: c_uint = 0x00;
pub const DA9210_VBUCK_GPI_GPIO0: c_uint = 0x20;
pub const DA9210_VBUCK_GPI_GPIO3: c_uint = 0x40;
pub const DA9210_VBUCK_GPI_GPIO4: c_uint = 0x60;
pub const DA9210_DVC_CTRL_EN: c_uint = 0x80;
// DA9210_REG_BUCK_ILIM (addr=0xD0)
pub const DA9210_BUCK_ILIM_SHIFT: c_int = 0;
pub const DA9210_BUCK_ILIM_MASK: c_uint = 0x0F;
pub const DA9210_BUCK_IALARM: c_uint = 0x10;
// DA9210_REG_BUCK_CONF1 (addr=0xD1)
pub const DA9210_BUCK_MODE_SHIFT: c_int = 0;
pub const DA9210_BUCK_MODE_MASK: c_uint = 0x03;
pub const DA9210_BUCK_MODE_MANUAL: c_uint = 0x00;
pub const DA9210_BUCK_MODE_SLEEP: c_uint = 0x01;
pub const DA9210_BUCK_MODE_SYNC: c_uint = 0x02;
pub const DA9210_BUCK_MODE_AUTO: c_uint = 0x03;
pub const DA9210_STARTUP_CTRL_SHIFT: c_int = 2;
pub const DA9210_STARTUP_CTRL_MASK: c_uint = 0x1C;
pub const DA9210_PWR_DOWN_CTRL_SHIFT: c_int = 5;
pub const DA9210_PWR_DOWN_CTRL_MASK: c_uint = 0xE0;
// DA9210_REG_BUCK_CONF2 (addr=0xD2)
pub const DA9210_PHASE_SEL_SHIFT: c_int = 0;
pub const DA9210_PHASE_SEL_MASK: c_uint = 0x03;
pub const DA9210_FREQ_SEL: c_uint = 0x40;
// DA9210_REG_BUCK_AUTO (addr=0xD4)
pub const DA9210_VBUCK_AUTO_SHIFT: c_int = 0;
pub const DA9210_VBUCK_AUTO_MASK: c_uint = 0x7F;
// DA9210_REG_BUCK_BASE (addr=0xD5)
pub const DA9210_VBUCK_BASE_SHIFT: c_int = 0;
pub const DA9210_VBUCK_BASE_MASK: c_uint = 0x7F;
// DA9210_REG_VBUCK_MAX_DVC_IF (addr=0xD6)
pub const DA9210_VBUCK_MAX_SHIFT: c_int = 0;
pub const DA9210_VBUCK_MAX_MASK: c_uint = 0x7F;
pub const DA9210_DVC_STEP_SIZE: c_uint = 0x80;
pub const DA9210_DVC_STEP_SIZE_10MV: c_uint = 0x00;
pub const DA9210_DVC_STEP_SIZE_20MV: c_uint = 0x80;
// DA9210_REG_VBUCK_DVC (addr=0xD7)
pub const DA9210_VBUCK_DVC_SHIFT: c_int = 0;
pub const DA9210_VBUCK_DVC_MASK: c_uint = 0x7F;
// DA9210_REG_VBUCK_A/B (addr=0xD8/0xD9)
pub const DA9210_VBUCK_SHIFT: c_int = 0;
pub const DA9210_VBUCK_MASK: c_uint = 0x7F;
pub const DA9210_VBUCK_BIAS: c_int = 0;
pub const DA9210_BUCK_SL: c_uint = 0x80;
// DA9210_REG_INTERFACE (addr=0x105)
pub const DA9210_IF_BASE_ADDR_SHIFT: c_int = 4;
pub const DA9210_IF_BASE_ADDR_MASK: c_uint = 0xF0;
// DA9210_REG_CONFIG_E (addr=0x147)
pub const DA9210_STAND_ALONE: c_uint = 0x01;
