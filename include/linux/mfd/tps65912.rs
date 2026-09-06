//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tps65912.h
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
// Copyright (C) 2015 Texas Instruments Incorporated - https://www.ti.com
// Andrew F. Davis <afd@ti.com>
//
// Based on the TPS65218 driver and the previous TPS65912 driver by
// Margarita Olaya Cabrera <magi@slimlogic.co.uk>
//

// List of registers for TPS65912
pub const TPS65912_DCDC1_CTRL: c_uint = 0x00;
pub const TPS65912_DCDC2_CTRL: c_uint = 0x01;
pub const TPS65912_DCDC3_CTRL: c_uint = 0x02;
pub const TPS65912_DCDC4_CTRL: c_uint = 0x03;
pub const TPS65912_DCDC1_OP: c_uint = 0x04;
pub const TPS65912_DCDC1_AVS: c_uint = 0x05;
pub const TPS65912_DCDC1_LIMIT: c_uint = 0x06;
pub const TPS65912_DCDC2_OP: c_uint = 0x07;
pub const TPS65912_DCDC2_AVS: c_uint = 0x08;
pub const TPS65912_DCDC2_LIMIT: c_uint = 0x09;
pub const TPS65912_DCDC3_OP: c_uint = 0x0A;
pub const TPS65912_DCDC3_AVS: c_uint = 0x0B;
pub const TPS65912_DCDC3_LIMIT: c_uint = 0x0C;
pub const TPS65912_DCDC4_OP: c_uint = 0x0D;
pub const TPS65912_DCDC4_AVS: c_uint = 0x0E;
pub const TPS65912_DCDC4_LIMIT: c_uint = 0x0F;
pub const TPS65912_LDO1_OP: c_uint = 0x10;
pub const TPS65912_LDO1_AVS: c_uint = 0x11;
pub const TPS65912_LDO1_LIMIT: c_uint = 0x12;
pub const TPS65912_LDO2_OP: c_uint = 0x13;
pub const TPS65912_LDO2_AVS: c_uint = 0x14;
pub const TPS65912_LDO2_LIMIT: c_uint = 0x15;
pub const TPS65912_LDO3_OP: c_uint = 0x16;
pub const TPS65912_LDO3_AVS: c_uint = 0x17;
pub const TPS65912_LDO3_LIMIT: c_uint = 0x18;
pub const TPS65912_LDO4_OP: c_uint = 0x19;
pub const TPS65912_LDO4_AVS: c_uint = 0x1A;
pub const TPS65912_LDO4_LIMIT: c_uint = 0x1B;
pub const TPS65912_LDO5: c_uint = 0x1C;
pub const TPS65912_LDO6: c_uint = 0x1D;
pub const TPS65912_LDO7: c_uint = 0x1E;
pub const TPS65912_LDO8: c_uint = 0x1F;
pub const TPS65912_LDO9: c_uint = 0x20;
pub const TPS65912_LDO10: c_uint = 0x21;
pub const TPS65912_THRM: c_uint = 0x22;
pub const TPS65912_CLK32OUT: c_uint = 0x23;
pub const TPS65912_DEVCTRL: c_uint = 0x24;
pub const TPS65912_DEVCTRL2: c_uint = 0x25;
pub const TPS65912_I2C_SPI_CFG: c_uint = 0x26;
pub const TPS65912_KEEP_ON: c_uint = 0x27;
pub const TPS65912_KEEP_ON2: c_uint = 0x28;
pub const TPS65912_SET_OFF1: c_uint = 0x29;
pub const TPS65912_SET_OFF2: c_uint = 0x2A;
pub const TPS65912_DEF_VOLT: c_uint = 0x2B;
pub const TPS65912_DEF_VOLT_MAPPING: c_uint = 0x2C;
pub const TPS65912_DISCHARGE: c_uint = 0x2D;
pub const TPS65912_DISCHARGE2: c_uint = 0x2E;
pub const TPS65912_EN1_SET1: c_uint = 0x2F;
pub const TPS65912_EN1_SET2: c_uint = 0x30;
pub const TPS65912_EN2_SET1: c_uint = 0x31;
pub const TPS65912_EN2_SET2: c_uint = 0x32;
pub const TPS65912_EN3_SET1: c_uint = 0x33;
pub const TPS65912_EN3_SET2: c_uint = 0x34;
pub const TPS65912_EN4_SET1: c_uint = 0x35;
pub const TPS65912_EN4_SET2: c_uint = 0x36;
pub const TPS65912_PGOOD: c_uint = 0x37;
pub const TPS65912_PGOOD2: c_uint = 0x38;
pub const TPS65912_INT_STS: c_uint = 0x39;
pub const TPS65912_INT_MSK: c_uint = 0x3A;
pub const TPS65912_INT_STS2: c_uint = 0x3B;
pub const TPS65912_INT_MSK2: c_uint = 0x3C;
pub const TPS65912_INT_STS3: c_uint = 0x3D;
pub const TPS65912_INT_MSK3: c_uint = 0x3E;
pub const TPS65912_INT_STS4: c_uint = 0x3F;
pub const TPS65912_INT_MSK4: c_uint = 0x40;
pub const TPS65912_GPIO1: c_uint = 0x41;
pub const TPS65912_GPIO2: c_uint = 0x42;
pub const TPS65912_GPIO3: c_uint = 0x43;
pub const TPS65912_GPIO4: c_uint = 0x44;
pub const TPS65912_GPIO5: c_uint = 0x45;
pub const TPS65912_VMON: c_uint = 0x46;
pub const TPS65912_LEDA_CTRL1: c_uint = 0x47;
pub const TPS65912_LEDA_CTRL2: c_uint = 0x48;
pub const TPS65912_LEDA_CTRL3: c_uint = 0x49;
pub const TPS65912_LEDA_CTRL4: c_uint = 0x4A;
pub const TPS65912_LEDA_CTRL5: c_uint = 0x4B;
pub const TPS65912_LEDA_CTRL6: c_uint = 0x4C;
pub const TPS65912_LEDA_CTRL7: c_uint = 0x4D;
pub const TPS65912_LEDA_CTRL8: c_uint = 0x4E;
pub const TPS65912_LEDB_CTRL1: c_uint = 0x4F;
pub const TPS65912_LEDB_CTRL2: c_uint = 0x50;
pub const TPS65912_LEDB_CTRL3: c_uint = 0x51;
pub const TPS65912_LEDB_CTRL4: c_uint = 0x52;
pub const TPS65912_LEDB_CTRL5: c_uint = 0x53;
pub const TPS65912_LEDB_CTRL6: c_uint = 0x54;
pub const TPS65912_LEDB_CTRL7: c_uint = 0x55;
pub const TPS65912_LEDB_CTRL8: c_uint = 0x56;
pub const TPS65912_LEDC_CTRL1: c_uint = 0x57;
pub const TPS65912_LEDC_CTRL2: c_uint = 0x58;
pub const TPS65912_LEDC_CTRL3: c_uint = 0x59;
pub const TPS65912_LEDC_CTRL4: c_uint = 0x5A;
pub const TPS65912_LEDC_CTRL5: c_uint = 0x5B;
pub const TPS65912_LEDC_CTRL6: c_uint = 0x5C;
pub const TPS65912_LEDC_CTRL7: c_uint = 0x5D;
pub const TPS65912_LEDC_CTRL8: c_uint = 0x5E;
pub const TPS65912_LED_RAMP_UP_TIME: c_uint = 0x5F;
pub const TPS65912_LED_RAMP_DOWN_TIME: c_uint = 0x60;
pub const TPS65912_LED_SEQ_EN: c_uint = 0x61;
pub const TPS65912_LOADSWITCH: c_uint = 0x62;
pub const TPS65912_SPARE: c_uint = 0x63;
pub const TPS65912_VERNUM: c_uint = 0x64;
pub const TPS6591X_MAX_REGISTER: c_uint = 0x64;
// INT_STS Register field definitions

// INT_STS Register field definitions

// INT_STS Register field definitions

// INT_STS Register field definitions

// GPIO 1 and 2 Register field definitions
pub const GPIO_SLEEP_MASK: c_uint = 0x80;
pub const GPIO_SLEEP_SHIFT: c_int = 7;
pub const GPIO_DEB_MASK: c_uint = 0x10;
pub const GPIO_DEB_SHIFT: c_int = 4;
pub const GPIO_CFG_MASK: c_uint = 0x04;
pub const GPIO_CFG_SHIFT: c_int = 2;
pub const GPIO_STS_MASK: c_uint = 0x02;
pub const GPIO_STS_SHIFT: c_int = 1;
pub const GPIO_SET_MASK: c_uint = 0x01;
pub const GPIO_SET_SHIFT: c_int = 0;
// GPIO 3 Register field definitions
pub const GPIO3_SLEEP_MASK: c_uint = 0x80;
pub const GPIO3_SLEEP_SHIFT: c_int = 7;
pub const GPIO3_SEL_MASK: c_uint = 0x40;
pub const GPIO3_SEL_SHIFT: c_int = 6;
pub const GPIO3_ODEN_MASK: c_uint = 0x20;
pub const GPIO3_ODEN_SHIFT: c_int = 5;
pub const GPIO3_DEB_MASK: c_uint = 0x10;
pub const GPIO3_DEB_SHIFT: c_int = 4;
pub const GPIO3_PDEN_MASK: c_uint = 0x08;
pub const GPIO3_PDEN_SHIFT: c_int = 3;
pub const GPIO3_CFG_MASK: c_uint = 0x04;
pub const GPIO3_CFG_SHIFT: c_int = 2;
pub const GPIO3_STS_MASK: c_uint = 0x02;
pub const GPIO3_STS_SHIFT: c_int = 1;
pub const GPIO3_SET_MASK: c_uint = 0x01;
pub const GPIO3_SET_SHIFT: c_int = 0;
// GPIO 4 Register field definitions
pub const GPIO4_SLEEP_MASK: c_uint = 0x80;
pub const GPIO4_SLEEP_SHIFT: c_int = 7;
pub const GPIO4_SEL_MASK: c_uint = 0x40;
pub const GPIO4_SEL_SHIFT: c_int = 6;
pub const GPIO4_ODEN_MASK: c_uint = 0x20;
pub const GPIO4_ODEN_SHIFT: c_int = 5;
pub const GPIO4_DEB_MASK: c_uint = 0x10;
pub const GPIO4_DEB_SHIFT: c_int = 4;
pub const GPIO4_PDEN_MASK: c_uint = 0x08;
pub const GPIO4_PDEN_SHIFT: c_int = 3;
pub const GPIO4_CFG_MASK: c_uint = 0x04;
pub const GPIO4_CFG_SHIFT: c_int = 2;
pub const GPIO4_STS_MASK: c_uint = 0x02;
pub const GPIO4_STS_SHIFT: c_int = 1;
pub const GPIO4_SET_MASK: c_uint = 0x01;
pub const GPIO4_SET_SHIFT: c_int = 0;
// Register THERM  (0x80) register.RegisterDescription
pub const THERM_THERM_HD_MASK: c_uint = 0x20;
pub const THERM_THERM_HD_SHIFT: c_int = 5;
pub const THERM_THERM_TS_MASK: c_uint = 0x10;
pub const THERM_THERM_TS_SHIFT: c_int = 4;
pub const THERM_THERM_HDSEL_MASK: c_uint = 0x0C;
pub const THERM_THERM_HDSEL_SHIFT: c_int = 2;
pub const THERM_RSVD1_MASK: c_uint = 0x02;
pub const THERM_RSVD1_SHIFT: c_int = 1;
pub const THERM_THERM_STATE_MASK: c_uint = 0x01;
pub const THERM_THERM_STATE_SHIFT: c_int = 0;
// Register DCDCCTRL1 register.RegisterDescription
pub const DCDCCTRL_VCON_ENABLE_MASK: c_uint = 0x80;
pub const DCDCCTRL_VCON_ENABLE_SHIFT: c_int = 7;
pub const DCDCCTRL_VCON_RANGE1_MASK: c_uint = 0x40;
pub const DCDCCTRL_VCON_RANGE1_SHIFT: c_int = 6;
pub const DCDCCTRL_VCON_RANGE0_MASK: c_uint = 0x20;
pub const DCDCCTRL_VCON_RANGE0_SHIFT: c_int = 5;
pub const DCDCCTRL_TSTEP2_MASK: c_uint = 0x10;
pub const DCDCCTRL_TSTEP2_SHIFT: c_int = 4;
pub const DCDCCTRL_TSTEP1_MASK: c_uint = 0x08;
pub const DCDCCTRL_TSTEP1_SHIFT: c_int = 3;
pub const DCDCCTRL_TSTEP0_MASK: c_uint = 0x04;
pub const DCDCCTRL_TSTEP0_SHIFT: c_int = 2;
pub const DCDCCTRL_DCDC1_MODE_MASK: c_uint = 0x02;
pub const DCDCCTRL_DCDC1_MODE_SHIFT: c_int = 1;
// Register DCDCCTRL2 and DCDCCTRL3 register.RegisterDescription
pub const DCDCCTRL_TSTEP2_MASK: c_uint = 0x10;
pub const DCDCCTRL_TSTEP2_SHIFT: c_int = 4;
pub const DCDCCTRL_TSTEP1_MASK: c_uint = 0x08;
pub const DCDCCTRL_TSTEP1_SHIFT: c_int = 3;
pub const DCDCCTRL_TSTEP0_MASK: c_uint = 0x04;
pub const DCDCCTRL_TSTEP0_SHIFT: c_int = 2;
pub const DCDCCTRL_DCDC_MODE_MASK: c_uint = 0x02;
pub const DCDCCTRL_DCDC_MODE_SHIFT: c_int = 1;
pub const DCDCCTRL_RSVD0_MASK: c_uint = 0x01;
pub const DCDCCTRL_RSVD0_SHIFT: c_int = 0;
// Register DCDCCTRL4 register.RegisterDescription
pub const DCDCCTRL_RAMP_TIME_MASK: c_uint = 0x01;
pub const DCDCCTRL_RAMP_TIME_SHIFT: c_int = 0;
// Register DCDCx_AVS
pub const DCDC_AVS_ENABLE_MASK: c_uint = 0x80;
pub const DCDC_AVS_ENABLE_SHIFT: c_int = 7;
pub const DCDC_AVS_ECO_MASK: c_uint = 0x40;
pub const DCDC_AVS_ECO_SHIFT: c_int = 6;
// Register DCDCx_LIMIT
pub const DCDC_LIMIT_RANGE_MASK: c_uint = 0xC0;
pub const DCDC_LIMIT_RANGE_SHIFT: c_int = 6;
pub const DCDC_LIMIT_MAX_SEL_MASK: c_uint = 0x3F;
pub const DCDC_LIMIT_MAX_SEL_SHIFT: c_int = 0;
// Define the TPS65912 IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tps65912_irqs {
// INT_STS registers
    TPS65912_IRQ_PWRHOLD_F,
    TPS65912_IRQ_VMON,
    TPS65912_IRQ_PWRON,
    TPS65912_IRQ_PWRON_LP,
    TPS65912_IRQ_PWRHOLD_R,
    TPS65912_IRQ_HOTDIE,
    TPS65912_IRQ_GPIO1_R,
    TPS65912_IRQ_GPIO1_F,
// INT_STS2 registers
    TPS65912_IRQ_GPIO2_R,
    TPS65912_IRQ_GPIO2_F,
    TPS65912_IRQ_GPIO3_R,
    TPS65912_IRQ_GPIO3_F,
    TPS65912_IRQ_GPIO4_R,
    TPS65912_IRQ_GPIO4_F,
    TPS65912_IRQ_GPIO5_R,
    TPS65912_IRQ_GPIO5_F,
// INT_STS3 registers
    TPS65912_IRQ_PGOOD_DCDC1,
    TPS65912_IRQ_PGOOD_DCDC2,
    TPS65912_IRQ_PGOOD_DCDC3,
    TPS65912_IRQ_PGOOD_DCDC4,
    TPS65912_IRQ_PGOOD_LDO1,
    TPS65912_IRQ_PGOOD_LDO2,
    TPS65912_IRQ_PGOOD_LDO3,
    TPS65912_IRQ_PGOOD_LDO4,
// INT_STS4 registers
    TPS65912_IRQ_PGOOD_LDO5,
    TPS65912_IRQ_PGOOD_LDO6,
    TPS65912_IRQ_PGOOD_LDO7,
    TPS65912_IRQ_PGOOD_LDO8,
    TPS65912_IRQ_PGOOD_LDO9,
    TPS65912_IRQ_PGOOD_LDO10,
}

//
// struct tps65912 - state holder for the tps65912 driver
//
// Device data may be used to access the TPS65912 chip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tps65912 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
// IRQ Data
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
}

extern "C" {
    pub fn tps65912_device_init(tps: *mut tps65912) -> c_int;
}
