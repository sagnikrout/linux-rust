//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/core.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// include/linux/mfd/wm831x/core.h -- Core interface for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

//
// Register values.
//
pub const WM831X_RESET_ID: c_uint = 0x00;
pub const WM831X_REVISION: c_uint = 0x01;
pub const WM831X_PARENT_ID: c_uint = 0x4000;
pub const WM831X_SYSVDD_CONTROL: c_uint = 0x4001;
pub const WM831X_THERMAL_MONITORING: c_uint = 0x4002;
pub const WM831X_POWER_STATE: c_uint = 0x4003;
pub const WM831X_WATCHDOG: c_uint = 0x4004;
pub const WM831X_ON_PIN_CONTROL: c_uint = 0x4005;
pub const WM831X_RESET_CONTROL: c_uint = 0x4006;
pub const WM831X_CONTROL_INTERFACE: c_uint = 0x4007;
pub const WM831X_SECURITY_KEY: c_uint = 0x4008;
pub const WM831X_SOFTWARE_SCRATCH: c_uint = 0x4009;
pub const WM831X_OTP_CONTROL: c_uint = 0x400A;
pub const WM831X_GPIO_LEVEL: c_uint = 0x400C;
pub const WM831X_SYSTEM_STATUS: c_uint = 0x400D;
pub const WM831X_ON_SOURCE: c_uint = 0x400E;
pub const WM831X_OFF_SOURCE: c_uint = 0x400F;
pub const WM831X_SYSTEM_INTERRUPTS: c_uint = 0x4010;
pub const WM831X_INTERRUPT_STATUS_1: c_uint = 0x4011;
pub const WM831X_INTERRUPT_STATUS_2: c_uint = 0x4012;
pub const WM831X_INTERRUPT_STATUS_3: c_uint = 0x4013;
pub const WM831X_INTERRUPT_STATUS_4: c_uint = 0x4014;
pub const WM831X_INTERRUPT_STATUS_5: c_uint = 0x4015;
pub const WM831X_IRQ_CONFIG: c_uint = 0x4017;
pub const WM831X_SYSTEM_INTERRUPTS_MASK: c_uint = 0x4018;
pub const WM831X_INTERRUPT_STATUS_1_MASK: c_uint = 0x4019;
pub const WM831X_INTERRUPT_STATUS_2_MASK: c_uint = 0x401A;
pub const WM831X_INTERRUPT_STATUS_3_MASK: c_uint = 0x401B;
pub const WM831X_INTERRUPT_STATUS_4_MASK: c_uint = 0x401C;
pub const WM831X_INTERRUPT_STATUS_5_MASK: c_uint = 0x401D;
pub const WM831X_RTC_WRITE_COUNTER: c_uint = 0x4020;
pub const WM831X_RTC_TIME_1: c_uint = 0x4021;
pub const WM831X_RTC_TIME_2: c_uint = 0x4022;
pub const WM831X_RTC_ALARM_1: c_uint = 0x4023;
pub const WM831X_RTC_ALARM_2: c_uint = 0x4024;
pub const WM831X_RTC_CONTROL: c_uint = 0x4025;
pub const WM831X_RTC_TRIM: c_uint = 0x4026;
pub const WM831X_TOUCH_CONTROL_1: c_uint = 0x4028;
pub const WM831X_TOUCH_CONTROL_2: c_uint = 0x4029;
pub const WM831X_TOUCH_DATA_X: c_uint = 0x402A;
pub const WM831X_TOUCH_DATA_Y: c_uint = 0x402B;
pub const WM831X_TOUCH_DATA_Z: c_uint = 0x402C;
pub const WM831X_AUXADC_DATA: c_uint = 0x402D;
pub const WM831X_AUXADC_CONTROL: c_uint = 0x402E;
pub const WM831X_AUXADC_SOURCE: c_uint = 0x402F;
pub const WM831X_COMPARATOR_CONTROL: c_uint = 0x4030;
pub const WM831X_COMPARATOR_1: c_uint = 0x4031;
pub const WM831X_COMPARATOR_2: c_uint = 0x4032;
pub const WM831X_COMPARATOR_3: c_uint = 0x4033;
pub const WM831X_COMPARATOR_4: c_uint = 0x4034;
pub const WM831X_GPIO1_CONTROL: c_uint = 0x4038;
pub const WM831X_GPIO2_CONTROL: c_uint = 0x4039;
pub const WM831X_GPIO3_CONTROL: c_uint = 0x403A;
pub const WM831X_GPIO4_CONTROL: c_uint = 0x403B;
pub const WM831X_GPIO5_CONTROL: c_uint = 0x403C;
pub const WM831X_GPIO6_CONTROL: c_uint = 0x403D;
pub const WM831X_GPIO7_CONTROL: c_uint = 0x403E;
pub const WM831X_GPIO8_CONTROL: c_uint = 0x403F;
pub const WM831X_GPIO9_CONTROL: c_uint = 0x4040;
pub const WM831X_GPIO10_CONTROL: c_uint = 0x4041;
pub const WM831X_GPIO11_CONTROL: c_uint = 0x4042;
pub const WM831X_GPIO12_CONTROL: c_uint = 0x4043;
pub const WM831X_GPIO13_CONTROL: c_uint = 0x4044;
pub const WM831X_GPIO14_CONTROL: c_uint = 0x4045;
pub const WM831X_GPIO15_CONTROL: c_uint = 0x4046;
pub const WM831X_GPIO16_CONTROL: c_uint = 0x4047;
pub const WM831X_CHARGER_CONTROL_1: c_uint = 0x4048;
pub const WM831X_CHARGER_CONTROL_2: c_uint = 0x4049;
pub const WM831X_CHARGER_STATUS: c_uint = 0x404A;
pub const WM831X_BACKUP_CHARGER_CONTROL: c_uint = 0x404B;
pub const WM831X_STATUS_LED_1: c_uint = 0x404C;
pub const WM831X_STATUS_LED_2: c_uint = 0x404D;
pub const WM831X_CURRENT_SINK_1: c_uint = 0x404E;
pub const WM831X_CURRENT_SINK_2: c_uint = 0x404F;
pub const WM831X_DCDC_ENABLE: c_uint = 0x4050;
pub const WM831X_LDO_ENABLE: c_uint = 0x4051;
pub const WM831X_DCDC_STATUS: c_uint = 0x4052;
pub const WM831X_LDO_STATUS: c_uint = 0x4053;
pub const WM831X_DCDC_UV_STATUS: c_uint = 0x4054;
pub const WM831X_LDO_UV_STATUS: c_uint = 0x4055;
pub const WM831X_DC1_CONTROL_1: c_uint = 0x4056;
pub const WM831X_DC1_CONTROL_2: c_uint = 0x4057;
pub const WM831X_DC1_ON_CONFIG: c_uint = 0x4058;
pub const WM831X_DC1_SLEEP_CONTROL: c_uint = 0x4059;
pub const WM831X_DC1_DVS_CONTROL: c_uint = 0x405A;
pub const WM831X_DC2_CONTROL_1: c_uint = 0x405B;
pub const WM831X_DC2_CONTROL_2: c_uint = 0x405C;
pub const WM831X_DC2_ON_CONFIG: c_uint = 0x405D;
pub const WM831X_DC2_SLEEP_CONTROL: c_uint = 0x405E;
pub const WM831X_DC2_DVS_CONTROL: c_uint = 0x405F;
pub const WM831X_DC3_CONTROL_1: c_uint = 0x4060;
pub const WM831X_DC3_CONTROL_2: c_uint = 0x4061;
pub const WM831X_DC3_ON_CONFIG: c_uint = 0x4062;
pub const WM831X_DC3_SLEEP_CONTROL: c_uint = 0x4063;
pub const WM831X_DC4_CONTROL: c_uint = 0x4064;
pub const WM831X_DC4_SLEEP_CONTROL: c_uint = 0x4065;
pub const WM832X_DC4_SLEEP_CONTROL: c_uint = 0x4067;
pub const WM831X_EPE1_CONTROL: c_uint = 0x4066;
pub const WM831X_EPE2_CONTROL: c_uint = 0x4067;
pub const WM831X_LDO1_CONTROL: c_uint = 0x4068;
pub const WM831X_LDO1_ON_CONTROL: c_uint = 0x4069;
pub const WM831X_LDO1_SLEEP_CONTROL: c_uint = 0x406A;
pub const WM831X_LDO2_CONTROL: c_uint = 0x406B;
pub const WM831X_LDO2_ON_CONTROL: c_uint = 0x406C;
pub const WM831X_LDO2_SLEEP_CONTROL: c_uint = 0x406D;
pub const WM831X_LDO3_CONTROL: c_uint = 0x406E;
pub const WM831X_LDO3_ON_CONTROL: c_uint = 0x406F;
pub const WM831X_LDO3_SLEEP_CONTROL: c_uint = 0x4070;
pub const WM831X_LDO4_CONTROL: c_uint = 0x4071;
pub const WM831X_LDO4_ON_CONTROL: c_uint = 0x4072;
pub const WM831X_LDO4_SLEEP_CONTROL: c_uint = 0x4073;
pub const WM831X_LDO5_CONTROL: c_uint = 0x4074;
pub const WM831X_LDO5_ON_CONTROL: c_uint = 0x4075;
pub const WM831X_LDO5_SLEEP_CONTROL: c_uint = 0x4076;
pub const WM831X_LDO6_CONTROL: c_uint = 0x4077;
pub const WM831X_LDO6_ON_CONTROL: c_uint = 0x4078;
pub const WM831X_LDO6_SLEEP_CONTROL: c_uint = 0x4079;
pub const WM831X_LDO7_CONTROL: c_uint = 0x407A;
pub const WM831X_LDO7_ON_CONTROL: c_uint = 0x407B;
pub const WM831X_LDO7_SLEEP_CONTROL: c_uint = 0x407C;
pub const WM831X_LDO8_CONTROL: c_uint = 0x407D;
pub const WM831X_LDO8_ON_CONTROL: c_uint = 0x407E;
pub const WM831X_LDO8_SLEEP_CONTROL: c_uint = 0x407F;
pub const WM831X_LDO9_CONTROL: c_uint = 0x4080;
pub const WM831X_LDO9_ON_CONTROL: c_uint = 0x4081;
pub const WM831X_LDO9_SLEEP_CONTROL: c_uint = 0x4082;
pub const WM831X_LDO10_CONTROL: c_uint = 0x4083;
pub const WM831X_LDO10_ON_CONTROL: c_uint = 0x4084;
pub const WM831X_LDO10_SLEEP_CONTROL: c_uint = 0x4085;
pub const WM831X_LDO11_ON_CONTROL: c_uint = 0x4087;
pub const WM831X_LDO11_SLEEP_CONTROL: c_uint = 0x4088;
pub const WM831X_POWER_GOOD_SOURCE_1: c_uint = 0x408E;
pub const WM831X_POWER_GOOD_SOURCE_2: c_uint = 0x408F;
pub const WM831X_CLOCK_CONTROL_1: c_uint = 0x4090;
pub const WM831X_CLOCK_CONTROL_2: c_uint = 0x4091;
pub const WM831X_FLL_CONTROL_1: c_uint = 0x4092;
pub const WM831X_FLL_CONTROL_2: c_uint = 0x4093;
pub const WM831X_FLL_CONTROL_3: c_uint = 0x4094;
pub const WM831X_FLL_CONTROL_4: c_uint = 0x4095;
pub const WM831X_FLL_CONTROL_5: c_uint = 0x4096;
pub const WM831X_UNIQUE_ID_1: c_uint = 0x7800;
pub const WM831X_UNIQUE_ID_2: c_uint = 0x7801;
pub const WM831X_UNIQUE_ID_3: c_uint = 0x7802;
pub const WM831X_UNIQUE_ID_4: c_uint = 0x7803;
pub const WM831X_UNIQUE_ID_5: c_uint = 0x7804;
pub const WM831X_UNIQUE_ID_6: c_uint = 0x7805;
pub const WM831X_UNIQUE_ID_7: c_uint = 0x7806;
pub const WM831X_UNIQUE_ID_8: c_uint = 0x7807;
pub const WM831X_FACTORY_OTP_ID: c_uint = 0x7808;
pub const WM831X_FACTORY_OTP_1: c_uint = 0x7809;
pub const WM831X_FACTORY_OTP_2: c_uint = 0x780A;
pub const WM831X_FACTORY_OTP_3: c_uint = 0x780B;
pub const WM831X_FACTORY_OTP_4: c_uint = 0x780C;
pub const WM831X_FACTORY_OTP_5: c_uint = 0x780D;
pub const WM831X_CUSTOMER_OTP_ID: c_uint = 0x7810;
pub const WM831X_DC1_OTP_CONTROL: c_uint = 0x7811;
pub const WM831X_DC2_OTP_CONTROL: c_uint = 0x7812;
pub const WM831X_DC3_OTP_CONTROL: c_uint = 0x7813;
pub const WM831X_LDO1_2_OTP_CONTROL: c_uint = 0x7814;
pub const WM831X_LDO3_4_OTP_CONTROL: c_uint = 0x7815;
pub const WM831X_LDO5_6_OTP_CONTROL: c_uint = 0x7816;
pub const WM831X_LDO7_8_OTP_CONTROL: c_uint = 0x7817;
pub const WM831X_LDO9_10_OTP_CONTROL: c_uint = 0x7818;
pub const WM831X_LDO11_EPE_CONTROL: c_uint = 0x7819;
pub const WM831X_GPIO1_OTP_CONTROL: c_uint = 0x781A;
pub const WM831X_GPIO2_OTP_CONTROL: c_uint = 0x781B;
pub const WM831X_GPIO3_OTP_CONTROL: c_uint = 0x781C;
pub const WM831X_GPIO4_OTP_CONTROL: c_uint = 0x781D;
pub const WM831X_GPIO5_OTP_CONTROL: c_uint = 0x781E;
pub const WM831X_GPIO6_OTP_CONTROL: c_uint = 0x781F;
pub const WM831X_DBE_CHECK_DATA: c_uint = 0x7827;
//
// R0 (0x00) - Reset ID
//
pub const WM831X_CHIP_ID_MASK: c_uint = 0xFFFF  /* CHIP_ID - [15:0] */;

//
// R1 (0x01) - Revision
//
pub const WM831X_PARENT_REV_MASK: c_uint = 0xFF00  /* PARENT_REV - [15:8] */;

pub const WM831X_CHILD_REV_MASK: c_uint = 0x00FF  /* CHILD_REV - [7:0] */;

//
// R16384 (0x4000) - Parent ID
//
pub const WM831X_PARENT_ID_MASK: c_uint = 0xFFFF  /* PARENT_ID - [15:0] */;

//
// R16389 (0x4005) - ON Pin Control
//
pub const WM831X_ON_PIN_SECACT_MASK: c_uint = 0x0300  /* ON_PIN_SECACT - [9:8] */;

pub const WM831X_ON_PIN_PRIMACT_MASK: c_uint = 0x0030  /* ON_PIN_PRIMACT - [5:4] */;

pub const WM831X_ON_PIN_STS: c_uint = 0x0008  /* ON_PIN_STS */;
pub const WM831X_ON_PIN_STS_MASK: c_uint = 0x0008  /* ON_PIN_STS */;

pub const WM831X_ON_PIN_TO_MASK: c_uint = 0x0003  /* ON_PIN_TO - [1:0] */;

//
// R16528 (0x4090) - Clock Control 1
//
pub const WM831X_CLKOUT_ENA: c_uint = 0x8000  /* CLKOUT_ENA */;
pub const WM831X_CLKOUT_ENA_MASK: c_uint = 0x8000  /* CLKOUT_ENA */;

pub const WM831X_CLKOUT_OD: c_uint = 0x2000  /* CLKOUT_OD */;
pub const WM831X_CLKOUT_OD_MASK: c_uint = 0x2000  /* CLKOUT_OD */;

pub const WM831X_CLKOUT_SLOT_MASK: c_uint = 0x0700  /* CLKOUT_SLOT - [10:8] */;

pub const WM831X_CLKOUT_SLPSLOT_MASK: c_uint = 0x0070  /* CLKOUT_SLPSLOT - [6:4] */;

pub const WM831X_CLKOUT_SRC: c_uint = 0x0001  /* CLKOUT_SRC */;
pub const WM831X_CLKOUT_SRC_MASK: c_uint = 0x0001  /* CLKOUT_SRC */;

//
// R16529 (0x4091) - Clock Control 2
//
pub const WM831X_XTAL_INH: c_uint = 0x8000  /* XTAL_INH */;
pub const WM831X_XTAL_INH_MASK: c_uint = 0x8000  /* XTAL_INH */;

pub const WM831X_XTAL_ENA: c_uint = 0x2000  /* XTAL_ENA */;
pub const WM831X_XTAL_ENA_MASK: c_uint = 0x2000  /* XTAL_ENA */;

pub const WM831X_XTAL_BKUPENA: c_uint = 0x1000  /* XTAL_BKUPENA */;
pub const WM831X_XTAL_BKUPENA_MASK: c_uint = 0x1000  /* XTAL_BKUPENA */;

pub const WM831X_FLL_AUTO: c_uint = 0x0080  /* FLL_AUTO */;
pub const WM831X_FLL_AUTO_MASK: c_uint = 0x0080  /* FLL_AUTO */;

pub const WM831X_FLL_AUTO_FREQ_MASK: c_uint = 0x0007  /* FLL_AUTO_FREQ - [2:0] */;

//
// R16530 (0x4092) - FLL Control 1
//
pub const WM831X_FLL_FRAC: c_uint = 0x0004  /* FLL_FRAC */;
pub const WM831X_FLL_FRAC_MASK: c_uint = 0x0004  /* FLL_FRAC */;

pub const WM831X_FLL_OSC_ENA: c_uint = 0x0002  /* FLL_OSC_ENA */;
pub const WM831X_FLL_OSC_ENA_MASK: c_uint = 0x0002  /* FLL_OSC_ENA */;

pub const WM831X_FLL_ENA: c_uint = 0x0001  /* FLL_ENA */;
pub const WM831X_FLL_ENA_MASK: c_uint = 0x0001  /* FLL_ENA */;

//
// R16531 (0x4093) - FLL Control 2
//
pub const WM831X_FLL_OUTDIV_MASK: c_uint = 0x3F00  /* FLL_OUTDIV - [13:8] */;

pub const WM831X_FLL_CTRL_RATE_MASK: c_uint = 0x0070  /* FLL_CTRL_RATE - [6:4] */;

pub const WM831X_FLL_FRATIO_MASK: c_uint = 0x0007  /* FLL_FRATIO - [2:0] */;

//
// R16532 (0x4094) - FLL Control 3
//
pub const WM831X_FLL_K_MASK: c_uint = 0xFFFF  /* FLL_K - [15:0] */;

//
// R16533 (0x4095) - FLL Control 4
//
pub const WM831X_FLL_N_MASK: c_uint = 0x7FE0  /* FLL_N - [14:5] */;

pub const WM831X_FLL_GAIN_MASK: c_uint = 0x000F  /* FLL_GAIN - [3:0] */;

//
// R16534 (0x4096) - FLL Control 5
//
pub const WM831X_FLL_CLK_REF_DIV_MASK: c_uint = 0x0018  /* FLL_CLK_REF_DIV - [4:3] */;

pub const WM831X_FLL_CLK_SRC_MASK: c_uint = 0x0003  /* FLL_CLK_SRC - [1:0] */;

pub const WM831X_NUM_IRQ_REGS: c_int = 5;
pub const WM831X_NUM_GPIO_REGS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm831x_parent {
    WM8310 = 0x8310,
    WM8311 = 0x8311,
    WM8312 = 0x8312,
    WM8320 = 0x8320,
    WM8321 = 0x8321,
    WM8325 = 0x8325,
    WM8326 = 0x8326,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm831x {
    pub io_lock: mutex,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pdata: wm831x_pdata,
    pub type: wm831x_parent,
    pub /: *mut *mut int irq; / Our chip IRQ,
    pub irq_lock: mutex,
    pub irq_domain: *mut irq_domain,
    pub /: *mut *mut int irq_masks_cur[WM831X_NUM_IRQ_REGS]; / Currently active value,
    pub /: *mut *mut int irq_masks_cache[WM831X_NUM_IRQ_REGS]; / Cached hardware value,
    pub soft_shutdown: bool,
// Chip revision based flags
    pub /: *mut *mut unsigned has_gpio_ena:1; / Has GPIO enable bit,
    pub /: *mut *mut unsigned has_cs_sts:1; / Has current sink status bit,
    pub /: *mut *mut unsigned charger_irq_wake:1; / Are charger IRQs a wake source?,
    pub num_gpio: c_int,
// Used by the interrupt controller code to post writes
    pub gpio_update: [c_int; WM831X_NUM_GPIO_REGS],
    pub gpio_level_high: [bool; WM831X_NUM_GPIO_REGS],
    pub gpio_level_low: [bool; WM831X_NUM_GPIO_REGS],
    pub auxadc_lock: mutex,
    pub auxadc_pending: list_head,
    pub auxadc_active: u16,
    pub auxadc_read: wm831x_auxadc_read_fn,
// The WM831x has a security key blocking access to certain
// registers.  The mutex is taken by the accessors for locking
// and unlocking the security key, locked is used to fail
// writes if the lock is held.
//
    pub key_lock: mutex,
    pub locked:1: c_uint,
}

// Device I/O API
extern "C" {
    pub fn wm831x_reg_read(wm831x: *mut wm831x, reg: c_ushort) -> c_int;
}
extern "C" {
    pub fn wm831x_reg_lock(wm831x: *mut wm831x);
}
extern "C" {
    pub fn wm831x_reg_unlock(wm831x: *mut wm831x) -> c_int;
}
extern "C" {
    pub fn wm831x_device_init(wm831x: *mut wm831x, irq: c_int) -> c_int;
}
extern "C" {
    pub fn wm831x_device_suspend(wm831x: *mut wm831x) -> c_int;
}
extern "C" {
    pub fn wm831x_device_shutdown(wm831x: *mut wm831x);
}
extern "C" {
    pub fn wm831x_irq_init(wm831x: *mut wm831x, irq: c_int) -> c_int;
}
extern "C" {
    pub fn wm831x_irq_exit(wm831x: *mut wm831x);
}
extern "C" {
    pub fn wm831x_auxadc_init(wm831x: *mut wm831x);
}
extern "C" {
    pub fn irq_create_mapping(_arg: wm831x->irq_domain, _arg: irq) -> return;
}
