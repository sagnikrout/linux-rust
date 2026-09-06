//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/regulator/da9211-regulator.h
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
// da9211-regulator.h - Regulator definitions for DA9211/DA9212
// /DA9213/DA9223/DA9214/DA9224/DA9215/DA9225
// Copyright (C) 2015  Dialog Semiconductor Ltd.
//
// Page selection
pub const DA9211_REG_PAGE_CON: c_uint = 0x00;
// System Control and Event Registers
pub const DA9211_REG_STATUS_A: c_uint = 0x50;
pub const DA9211_REG_STATUS_B: c_uint = 0x51;
pub const DA9211_REG_EVENT_A: c_uint = 0x52;
pub const DA9211_REG_EVENT_B: c_uint = 0x53;
pub const DA9211_REG_MASK_A: c_uint = 0x54;
pub const DA9211_REG_MASK_B: c_uint = 0x55;
pub const DA9211_REG_CONTROL_A: c_uint = 0x56;
// GPIO Control Registers
pub const DA9211_REG_GPIO_0_1: c_uint = 0x58;
pub const DA9211_REG_GPIO_2_3: c_uint = 0x59;
pub const DA9211_REG_GPIO_4: c_uint = 0x5A;
// Regulator Registers
pub const DA9211_REG_BUCKA_CONT: c_uint = 0x5D;
pub const DA9211_REG_BUCKB_CONT: c_uint = 0x5E;
pub const DA9211_REG_BUCK_ILIM: c_uint = 0xD0;
pub const DA9211_REG_BUCKA_CONF: c_uint = 0xD1;
pub const DA9211_REG_BUCKB_CONF: c_uint = 0xD2;
pub const DA9211_REG_BUCK_CONF: c_uint = 0xD3;
pub const DA9211_REG_VBACKA_MAX: c_uint = 0xD5;
pub const DA9211_REG_VBACKB_MAX: c_uint = 0xD6;
pub const DA9211_REG_VBUCKA_A: c_uint = 0xD7;
pub const DA9211_REG_VBUCKA_B: c_uint = 0xD8;
pub const DA9211_REG_VBUCKB_A: c_uint = 0xD9;
pub const DA9211_REG_VBUCKB_B: c_uint = 0xDA;
// I2C Interface Settings
pub const DA9211_REG_INTERFACE: c_uint = 0x105;
// BUCK Phase Selection
pub const DA9211_REG_CONFIG_E: c_uint = 0x147;
// Device ID
pub const DA9211_REG_DEVICE_ID: c_uint = 0x201;
//
// Registers bits
//
// DA9211_REG_PAGE_CON (addr=0x00)
pub const DA9211_REG_PAGE_SHIFT: c_int = 1;
pub const DA9211_REG_PAGE_MASK: c_uint = 0x06;
// On I2C registers 0x00 - 0xFF
pub const DA9211_REG_PAGE0: c_int = 0;
// On I2C registers 0x100 - 0x1FF
pub const DA9211_REG_PAGE2: c_int = 2;
pub const DA9211_PAGE_WRITE_MODE: c_uint = 0x00;
pub const DA9211_REPEAT_WRITE_MODE: c_uint = 0x40;
pub const DA9211_PAGE_REVERT: c_uint = 0x80;
// DA9211_REG_STATUS_A (addr=0x50)
pub const DA9211_GPI0: c_uint = 0x01;
pub const DA9211_GPI1: c_uint = 0x02;
pub const DA9211_GPI2: c_uint = 0x04;
pub const DA9211_GPI3: c_uint = 0x08;
pub const DA9211_GPI4: c_uint = 0x10;
// DA9211_REG_EVENT_A (addr=0x52)
pub const DA9211_E_GPI0: c_uint = 0x01;
pub const DA9211_E_GPI1: c_uint = 0x02;
pub const DA9211_E_GPI2: c_uint = 0x04;
pub const DA9211_E_GPI3: c_uint = 0x08;
pub const DA9211_E_GPI4: c_uint = 0x10;
pub const DA9211_E_UVLO_IO: c_uint = 0x40;
// DA9211_REG_EVENT_B (addr=0x53)
pub const DA9211_E_PWRGOOD_A: c_uint = 0x01;
pub const DA9211_E_PWRGOOD_B: c_uint = 0x02;
pub const DA9211_E_TEMP_WARN: c_uint = 0x04;
pub const DA9211_E_TEMP_CRIT: c_uint = 0x08;
pub const DA9211_E_OV_CURR_A: c_uint = 0x10;
pub const DA9211_E_OV_CURR_B: c_uint = 0x20;
// DA9211_REG_MASK_A (addr=0x54)
pub const DA9211_M_GPI0: c_uint = 0x01;
pub const DA9211_M_GPI1: c_uint = 0x02;
pub const DA9211_M_GPI2: c_uint = 0x04;
pub const DA9211_M_GPI3: c_uint = 0x08;
pub const DA9211_M_GPI4: c_uint = 0x10;
pub const DA9211_M_UVLO_IO: c_uint = 0x40;
// DA9211_REG_MASK_B (addr=0x55)
pub const DA9211_M_PWRGOOD_A: c_uint = 0x01;
pub const DA9211_M_PWRGOOD_B: c_uint = 0x02;
pub const DA9211_M_TEMP_WARN: c_uint = 0x04;
pub const DA9211_M_TEMP_CRIT: c_uint = 0x08;
pub const DA9211_M_OV_CURR_A: c_uint = 0x10;
pub const DA9211_M_OV_CURR_B: c_uint = 0x20;
// DA9211_REG_CONTROL_A (addr=0x56)
pub const DA9211_DEBOUNCING_SHIFT: c_int = 0;
pub const DA9211_DEBOUNCING_MASK: c_uint = 0x07;
pub const DA9211_SLEW_RATE_SHIFT: c_int = 3;
pub const DA9211_SLEW_RATE_A_MASK: c_uint = 0x18;
pub const DA9211_SLEW_RATE_B_SHIFT: c_int = 5;
pub const DA9211_SLEW_RATE_B_MASK: c_uint = 0x60;
pub const DA9211_V_LOCK: c_uint = 0x80;
// DA9211_REG_GPIO_0_1 (addr=0x58)
pub const DA9211_GPIO0_PIN_SHIFT: c_int = 0;
pub const DA9211_GPIO0_PIN_MASK: c_uint = 0x03;
pub const DA9211_GPIO0_PIN_GPI: c_uint = 0x00;
pub const DA9211_GPIO0_PIN_GPO_OD: c_uint = 0x02;
pub const DA9211_GPIO0_PIN_GPO: c_uint = 0x03;
pub const DA9211_GPIO0_TYPE: c_uint = 0x04;
pub const DA9211_GPIO0_TYPE_GPI: c_uint = 0x00;
pub const DA9211_GPIO0_TYPE_GPO: c_uint = 0x04;
pub const DA9211_GPIO0_MODE: c_uint = 0x08;
pub const DA9211_GPIO1_PIN_SHIFT: c_int = 4;
pub const DA9211_GPIO1_PIN_MASK: c_uint = 0x30;
pub const DA9211_GPIO1_PIN_GPI: c_uint = 0x00;
pub const DA9211_GPIO1_PIN_VERROR: c_uint = 0x10;
pub const DA9211_GPIO1_PIN_GPO_OD: c_uint = 0x20;
pub const DA9211_GPIO1_PIN_GPO: c_uint = 0x30;
pub const DA9211_GPIO1_TYPE_SHIFT: c_uint = 0x40;
pub const DA9211_GPIO1_TYPE_GPI: c_uint = 0x00;
pub const DA9211_GPIO1_TYPE_GPO: c_uint = 0x40;
pub const DA9211_GPIO1_MODE: c_uint = 0x80;
// DA9211_REG_GPIO_2_3 (addr=0x59)
pub const DA9211_GPIO2_PIN_SHIFT: c_int = 0;
pub const DA9211_GPIO2_PIN_MASK: c_uint = 0x03;
pub const DA9211_GPIO2_PIN_GPI: c_uint = 0x00;
pub const DA9211_GPIO5_PIN_BUCK_CLK: c_uint = 0x10;
pub const DA9211_GPIO2_PIN_GPO_OD: c_uint = 0x02;
pub const DA9211_GPIO2_PIN_GPO: c_uint = 0x03;
pub const DA9211_GPIO2_TYPE: c_uint = 0x04;
pub const DA9211_GPIO2_TYPE_GPI: c_uint = 0x00;
pub const DA9211_GPIO2_TYPE_GPO: c_uint = 0x04;
pub const DA9211_GPIO2_MODE: c_uint = 0x08;
pub const DA9211_GPIO3_PIN_SHIFT: c_int = 4;
pub const DA9211_GPIO3_PIN_MASK: c_uint = 0x30;
pub const DA9211_GPIO3_PIN_GPI: c_uint = 0x00;
pub const DA9211_GPIO3_PIN_IERROR: c_uint = 0x10;
pub const DA9211_GPIO3_PIN_GPO_OD: c_uint = 0x20;
pub const DA9211_GPIO3_PIN_GPO: c_uint = 0x30;
pub const DA9211_GPIO3_TYPE_SHIFT: c_uint = 0x40;
pub const DA9211_GPIO3_TYPE_GPI: c_uint = 0x00;
pub const DA9211_GPIO3_TYPE_GPO: c_uint = 0x40;
pub const DA9211_GPIO3_MODE: c_uint = 0x80;
// DA9211_REG_GPIO_4 (addr=0x5A)
pub const DA9211_GPIO4_PIN_SHIFT: c_int = 0;
pub const DA9211_GPIO4_PIN_MASK: c_uint = 0x03;
pub const DA9211_GPIO4_PIN_GPI: c_uint = 0x00;
pub const DA9211_GPIO4_PIN_GPO_OD: c_uint = 0x02;
pub const DA9211_GPIO4_PIN_GPO: c_uint = 0x03;
pub const DA9211_GPIO4_TYPE: c_uint = 0x04;
pub const DA9211_GPIO4_TYPE_GPI: c_uint = 0x00;
pub const DA9211_GPIO4_TYPE_GPO: c_uint = 0x04;
pub const DA9211_GPIO4_MODE: c_uint = 0x08;
// DA9211_REG_BUCKA_CONT (addr=0x5D)
pub const DA9211_BUCKA_EN: c_uint = 0x01;
pub const DA9211_BUCKA_GPI_SHIFT: c_int = 1;
pub const DA9211_BUCKA_GPI_MASK: c_uint = 0x06;
pub const DA9211_BUCKA_GPI_OFF: c_uint = 0x00;
pub const DA9211_BUCKA_GPI_GPIO0: c_uint = 0x02;
pub const DA9211_BUCKA_GPI_GPIO1: c_uint = 0x04;
pub const DA9211_BUCKA_GPI_GPIO3: c_uint = 0x06;
pub const DA9211_BUCKA_PD_DIS: c_uint = 0x08;
pub const DA9211_VBUCKA_SEL: c_uint = 0x10;
pub const DA9211_VBUCKA_SEL_A: c_uint = 0x00;
pub const DA9211_VBUCKA_SEL_B: c_uint = 0x10;
pub const DA9211_VBUCKA_GPI_SHIFT: c_int = 5;
pub const DA9211_VBUCKA_GPI_MASK: c_uint = 0x60;
pub const DA9211_VBUCKA_GPI_OFF: c_uint = 0x00;
pub const DA9211_VBUCKA_GPI_GPIO1: c_uint = 0x20;
pub const DA9211_VBUCKA_GPI_GPIO2: c_uint = 0x40;
pub const DA9211_VBUCKA_GPI_GPIO4: c_uint = 0x60;
// DA9211_REG_BUCKB_CONT (addr=0x5E)
pub const DA9211_BUCKB_EN: c_uint = 0x01;
pub const DA9211_BUCKB_GPI_SHIFT: c_int = 1;
pub const DA9211_BUCKB_GPI_MASK: c_uint = 0x06;
pub const DA9211_BUCKB_GPI_OFF: c_uint = 0x00;
pub const DA9211_BUCKB_GPI_GPIO0: c_uint = 0x02;
pub const DA9211_BUCKB_GPI_GPIO1: c_uint = 0x04;
pub const DA9211_BUCKB_GPI_GPIO3: c_uint = 0x06;
pub const DA9211_BUCKB_PD_DIS: c_uint = 0x08;
pub const DA9211_VBUCKB_SEL: c_uint = 0x10;
pub const DA9211_VBUCKB_SEL_A: c_uint = 0x00;
pub const DA9211_VBUCKB_SEL_B: c_uint = 0x10;
pub const DA9211_VBUCKB_GPI_SHIFT: c_int = 5;
pub const DA9211_VBUCKB_GPI_MASK: c_uint = 0x60;
pub const DA9211_VBUCKB_GPI_OFF: c_uint = 0x00;
pub const DA9211_VBUCKB_GPI_GPIO1: c_uint = 0x20;
pub const DA9211_VBUCKB_GPI_GPIO2: c_uint = 0x40;
pub const DA9211_VBUCKB_GPI_GPIO4: c_uint = 0x60;
// DA9211_REG_BUCK_ILIM (addr=0xD0)
pub const DA9211_BUCKA_ILIM_SHIFT: c_int = 0;
pub const DA9211_BUCKA_ILIM_MASK: c_uint = 0x0F;
pub const DA9211_BUCKB_ILIM_SHIFT: c_int = 4;
pub const DA9211_BUCKB_ILIM_MASK: c_uint = 0xF0;
// DA9211_REG_BUCKA_CONF (addr=0xD1)
pub const DA9211_BUCKA_MODE_SHIFT: c_int = 0;
pub const DA9211_BUCKA_MODE_MASK: c_uint = 0x03;
pub const DA9211_BUCKA_MODE_MANUAL: c_uint = 0x00;
pub const DA9211_BUCKA_MODE_SLEEP: c_uint = 0x01;
pub const DA9211_BUCKA_MODE_SYNC: c_uint = 0x02;
pub const DA9211_BUCKA_MODE_AUTO: c_uint = 0x03;
pub const DA9211_BUCKA_UP_CTRL_SHIFT: c_int = 2;
pub const DA9211_BUCKA_UP_CTRL_MASK: c_uint = 0x1C;
pub const DA9211_BUCKA_DOWN_CTRL_SHIFT: c_int = 5;
pub const DA9211_BUCKA_DOWN_CTRL_MASK: c_uint = 0xE0;
// DA9211_REG_BUCKB_CONF (addr=0xD2)
pub const DA9211_BUCKB_MODE_SHIFT: c_int = 0;
pub const DA9211_BUCKB_MODE_MASK: c_uint = 0x03;
pub const DA9211_BUCKB_MODE_MANUAL: c_uint = 0x00;
pub const DA9211_BUCKB_MODE_SLEEP: c_uint = 0x01;
pub const DA9211_BUCKB_MODE_SYNC: c_uint = 0x02;
pub const DA9211_BUCKB_MODE_AUTO: c_uint = 0x03;
pub const DA9211_BUCKB_UP_CTRL_SHIFT: c_int = 2;
pub const DA9211_BUCKB_UP_CTRL_MASK: c_uint = 0x1C;
pub const DA9211_BUCKB_DOWN_CTRL_SHIFT: c_int = 5;
pub const DA9211_BUCKB_DOWN_CTRL_MASK: c_uint = 0xE0;
// DA9211_REG_BUCK_CONF (addr=0xD3)
pub const DA9211_PHASE_SEL_A_SHIFT: c_int = 0;
pub const DA9211_PHASE_SEL_A_MASK: c_uint = 0x03;
pub const DA9211_PHASE_SEL_B_SHIFT: c_int = 2;
pub const DA9211_PHASE_SEL_B_MASK: c_uint = 0x04;
pub const DA9211_PH_SH_EN_A_SHIFT: c_int = 3;
pub const DA9211_PH_SH_EN_A_MASK: c_uint = 0x08;
pub const DA9211_PH_SH_EN_B_SHIFT: c_int = 4;
pub const DA9211_PH_SH_EN_B_MASK: c_uint = 0x10;
// DA9211_REG_VBUCKA_MAX (addr=0xD5)
pub const DA9211_VBUCKA_BASE_SHIFT: c_int = 0;
pub const DA9211_VBUCKA_BASE_MASK: c_uint = 0x7F;
// DA9211_REG_VBUCKB_MAX (addr=0xD6)
pub const DA9211_VBUCKB_BASE_SHIFT: c_int = 0;
pub const DA9211_VBUCKB_BASE_MASK: c_uint = 0x7F;
// DA9211_REG_VBUCKA/B_A/B (addr=0xD7/0xD8/0xD9/0xDA)
pub const DA9211_VBUCK_SHIFT: c_int = 0;
pub const DA9211_VBUCK_MASK: c_uint = 0x7F;
pub const DA9211_VBUCK_BIAS: c_int = 0;
pub const DA9211_BUCK_SL: c_uint = 0x80;
// DA9211_REG_INTERFACE (addr=0x105)
pub const DA9211_IF_BASE_ADDR_SHIFT: c_int = 4;
pub const DA9211_IF_BASE_ADDR_MASK: c_uint = 0xF0;
// DA9211_REG_CONFIG_E (addr=0x147)
pub const DA9211_SLAVE_SEL: c_uint = 0x40;
