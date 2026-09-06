//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/adp5585.h
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
// Analog Devices ADP5585 I/O expander, PWM controller and keypad controller
//
// Copyright 2022 NXP
// Copyright 2024 Ideas on Board Oy
//

pub const ADP5585_ID: c_uint = 0x00;
pub const ADP5585_MAN_ID_VALUE: c_uint = 0x20;

pub const ADP5585_INT_STATUS: c_uint = 0x01;

pub const ADP5585_STATUS: c_uint = 0x02;

pub const ADP5585_FIFO_1: c_uint = 0x03;

pub const ADP5585_FIFO_2: c_uint = 0x04;
pub const ADP5585_FIFO_3: c_uint = 0x05;
pub const ADP5585_FIFO_4: c_uint = 0x06;
pub const ADP5585_FIFO_5: c_uint = 0x07;
pub const ADP5585_FIFO_6: c_uint = 0x08;
pub const ADP5585_FIFO_7: c_uint = 0x09;
pub const ADP5585_FIFO_8: c_uint = 0x0a;
pub const ADP5585_FIFO_9: c_uint = 0x0b;
pub const ADP5585_FIFO_10: c_uint = 0x0c;
pub const ADP5585_FIFO_11: c_uint = 0x0d;
pub const ADP5585_FIFO_12: c_uint = 0x0e;
pub const ADP5585_FIFO_13: c_uint = 0x0f;
pub const ADP5585_FIFO_14: c_uint = 0x10;
pub const ADP5585_FIFO_15: c_uint = 0x11;
pub const ADP5585_FIFO_16: c_uint = 0x12;

pub const ADP5585_GPI_INT_STAT_A: c_uint = 0x13;
pub const ADP5585_GPI_INT_STAT_B: c_uint = 0x14;
pub const ADP5585_GPI_STATUS_A: c_uint = 0x15;
pub const ADP5585_GPI_STATUS_B: c_uint = 0x16;
pub const ADP5585_RPULL_CONFIG_A: c_uint = 0x17;
pub const ADP5585_RPULL_CONFIG_B: c_uint = 0x18;
pub const ADP5585_RPULL_CONFIG_C: c_uint = 0x19;
pub const ADP5585_RPULL_CONFIG_D: c_uint = 0x1a;
pub const ADP5585_Rx_PULL_CFG_PU_300K: c_int = 0;
pub const ADP5585_Rx_PULL_CFG_PD_300K: c_int = 1;
pub const ADP5585_Rx_PULL_CFG_PU_100K: c_int = 2;
pub const ADP5585_Rx_PULL_CFG_DISABLE: c_int = 3;
pub const ADP5585_Rx_PULL_CFG_MASK: c_int = 3;
pub const ADP5585_GPI_INT_LEVEL_A: c_uint = 0x1b;
pub const ADP5585_GPI_INT_LEVEL_B: c_uint = 0x1c;
pub const ADP5585_GPI_EVENT_EN_A: c_uint = 0x1d;
pub const ADP5585_GPI_EVENT_EN_B: c_uint = 0x1e;
pub const ADP5585_GPI_INTERRUPT_EN_A: c_uint = 0x1f;
pub const ADP5585_GPI_INTERRUPT_EN_B: c_uint = 0x20;
pub const ADP5585_DEBOUNCE_DIS_A: c_uint = 0x21;
pub const ADP5585_DEBOUNCE_DIS_B: c_uint = 0x22;
pub const ADP5585_GPO_DATA_OUT_A: c_uint = 0x23;
pub const ADP5585_GPO_DATA_OUT_B: c_uint = 0x24;
pub const ADP5585_GPO_OUT_MODE_A: c_uint = 0x25;
pub const ADP5585_GPO_OUT_MODE_B: c_uint = 0x26;
pub const ADP5585_GPIO_DIRECTION_A: c_uint = 0x27;
pub const ADP5585_GPIO_DIRECTION_B: c_uint = 0x28;
pub const ADP5585_RESET1_EVENT_A: c_uint = 0x29;

pub const ADP5585_RESET1_EVENT_B: c_uint = 0x2a;
pub const ADP5585_RESET1_EVENT_C: c_uint = 0x2b;
pub const ADP5585_RESET2_EVENT_A: c_uint = 0x2c;
pub const ADP5585_RESET2_EVENT_B: c_uint = 0x2d;
pub const ADP5585_RESET_CFG: c_uint = 0x2e;
pub const ADP5585_PWM_OFFT_LOW: c_uint = 0x2f;
pub const ADP5585_PWM_OFFT_HIGH: c_uint = 0x30;
pub const ADP5585_PWM_ONT_LOW: c_uint = 0x31;
pub const ADP5585_PWM_ONT_HIGH: c_uint = 0x32;
pub const ADP5585_PWM_CFG: c_uint = 0x33;

pub const ADP5585_LOGIC_CFG: c_uint = 0x34;
pub const ADP5585_LOGIC_FF_CFG: c_uint = 0x35;
pub const ADP5585_LOGIC_INT_EVENT_EN: c_uint = 0x36;
pub const ADP5585_POLL_PTIME_CFG: c_uint = 0x37;
pub const ADP5585_PIN_CONFIG_A: c_uint = 0x38;
pub const ADP5585_PIN_CONFIG_B: c_uint = 0x39;
pub const ADP5585_PIN_CONFIG_C: c_uint = 0x3a;

pub const ADP5585_GENERAL_CFG: c_uint = 0x3b;

pub const ADP5585_INT_EN: c_uint = 0x3c;

pub const ADP5585_PIN_MAX: c_int = 11;
pub const ADP5585_MAX_UNLOCK_TIME_SEC: c_int = 7;
pub const ADP5585_KEY_EVENT_START: c_int = 1;
pub const ADP5585_KEY_EVENT_END: c_int = 25;
pub const ADP5585_GPI_EVENT_START: c_int = 37;
pub const ADP5585_GPI_EVENT_END: c_int = 47;
pub const ADP5585_ROW5_KEY_EVENT_START: c_int = 1;
pub const ADP5585_ROW5_KEY_EVENT_END: c_int = 30;
pub const ADP5585_PWM_OUT: c_int = 3;
pub const ADP5585_RESET1_OUT: c_int = 4;
pub const ADP5585_RESET2_OUT: c_int = 9;
pub const ADP5585_ROW5: c_int = 5;
// ADP5589
pub const ADP5589_MAN_ID_VALUE: c_uint = 0x10;
pub const ADP5589_GPI_STATUS_A: c_uint = 0x16;
pub const ADP5589_GPI_STATUS_C: c_uint = 0x18;
pub const ADP5589_RPULL_CONFIG_A: c_uint = 0x19;
pub const ADP5589_GPI_INT_LEVEL_A: c_uint = 0x1e;
pub const ADP5589_GPI_EVENT_EN_A: c_uint = 0x21;
pub const ADP5589_DEBOUNCE_DIS_A: c_uint = 0x27;
pub const ADP5589_GPO_DATA_OUT_A: c_uint = 0x2a;
pub const ADP5589_GPO_OUT_MODE_A: c_uint = 0x2d;
pub const ADP5589_GPIO_DIRECTION_A: c_uint = 0x30;
pub const ADP5589_UNLOCK1: c_uint = 0x33;

pub const ADP5589_UNLOCK_TIMERS: c_uint = 0x36;

pub const ADP5589_LOCK_CFG: c_uint = 0x37;

pub const ADP5589_RESET1_EVENT_A: c_uint = 0x38;
pub const ADP5589_RESET2_EVENT_A: c_uint = 0x3B;
pub const ADP5589_RESET_CFG: c_uint = 0x3D;

pub const ADP5589_PWM_OFFT_LOW: c_uint = 0x3e;
pub const ADP5589_PWM_ONT_LOW: c_uint = 0x40;
pub const ADP5589_PWM_CFG: c_uint = 0x42;
pub const ADP5589_POLL_PTIME_CFG: c_uint = 0x48;
pub const ADP5589_PIN_CONFIG_A: c_uint = 0x49;
pub const ADP5589_PIN_CONFIG_D: c_uint = 0x4C;
pub const ADP5589_GENERAL_CFG: c_uint = 0x4d;
pub const ADP5589_INT_EN: c_uint = 0x4e;

pub const ADP5589_PIN_MAX: c_int = 19;
pub const ADP5589_KEY_EVENT_START: c_int = 1;
pub const ADP5589_KEY_EVENT_END: c_int = 88;
pub const ADP5589_GPI_EVENT_START: c_int = 97;
pub const ADP5589_GPI_EVENT_END: c_int = 115;
pub const ADP5589_UNLOCK_WILDCARD: c_int = 127;
pub const ADP5589_RESET2_OUT: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adp5585_variant {
    ADP5585_00 = 1,
    ADP5585_01,
    ADP5585_02,
    ADP5585_03,
    ADP5585_04,
    ADP5589_00,
    ADP5589_01,
    ADP5589_02,
    ADP5585_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5585_regs {
    pub gen_cfg: c_uint,
    pub ext_cfg: c_uint,
    pub int_en: c_uint,
    pub poll_ptime_cfg: c_uint,
    pub reset_cfg: c_uint,
    pub reset1_event_a: c_uint,
    pub reset2_event_a: c_uint,
    pub pin_cfg_a: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp5585_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regs: *const adp5585_regs,
    pub event_notifier: blocking_notifier_head,
    pub pin_usage: *mut c_ulong,
    pub n_pins: c_uint,
    pub reset2_out: c_uint,
    pub variant: adp5585_variant,
    pub id: c_uint,
    pub has_unlock: bool,
    pub has_pin6: bool,
    pub irq: c_int,
    pub ev_poll_time: c_uint,
    pub unlock_time: c_uint,
    pub unlock_keys: [c_uint; 2],
    pub nkeys_unlock: c_uint,
    pub reset1_keys: [c_uint; 3],
    pub nkeys_reset1: c_uint,
    pub reset2_keys: [c_uint; 2],
    pub nkeys_reset2: c_uint,
    pub reset_cfg: u8,
}
