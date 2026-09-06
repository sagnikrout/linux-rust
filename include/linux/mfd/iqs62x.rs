//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/iqs62x.h
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
// Azoteq IQS620A/621/622/624/625 Multi-Function Sensors
//
// Copyright (C) 2019 Jeff LaBundy <jeff@labundy.com>
//
pub const IQS620_PROD_NUM: c_uint = 0x41;
pub const IQS621_PROD_NUM: c_uint = 0x46;
pub const IQS622_PROD_NUM: c_uint = 0x42;
pub const IQS624_PROD_NUM: c_uint = 0x43;
pub const IQS625_PROD_NUM: c_uint = 0x4E;
pub const IQS620_HW_NUM_V0: c_uint = 0x82;

pub const IQS620_HW_NUM_V3: c_uint = 0x92;
pub const IQS621_ALS_FLAGS: c_uint = 0x16;
pub const IQS622_ALS_FLAGS: c_uint = 0x14;
pub const IQS624_HALL_UI: c_uint = 0x70;

pub const IQS624_INTERVAL_DIV: c_uint = 0x7D;
pub const IQS620_GLBL_EVENT_MASK: c_uint = 0xD7;

pub const IQS62X_NUM_KEYS: c_int = 16;

pub const IQS62X_EVENT_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iqs62x_ui_sel {
    IQS62X_UI_PROX,
    IQS62X_UI_SAR1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iqs62x_event_reg {
    IQS62X_EVENT_NONE,
    IQS62X_EVENT_SYS,
    IQS62X_EVENT_PROX,
    IQS62X_EVENT_HYST,
    IQS62X_EVENT_HALL,
    IQS62X_EVENT_ALS,
    IQS62X_EVENT_IR,
    IQS62X_EVENT_WHEEL,
    IQS62X_EVENT_INTER,
    IQS62X_EVENT_UI_LO,
    IQS62X_EVENT_UI_HI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iqs62x_event_flag {
// keys
    IQS62X_EVENT_PROX_CH0_T,
    IQS62X_EVENT_PROX_CH0_P,
    IQS62X_EVENT_PROX_CH1_T,
    IQS62X_EVENT_PROX_CH1_P,
    IQS62X_EVENT_PROX_CH2_T,
    IQS62X_EVENT_PROX_CH2_P,
    IQS62X_EVENT_HYST_POS_T,
    IQS62X_EVENT_HYST_POS_P,
    IQS62X_EVENT_HYST_NEG_T,
    IQS62X_EVENT_HYST_NEG_P,
    IQS62X_EVENT_SAR1_ACT,
    IQS62X_EVENT_SAR1_QRD,
    IQS62X_EVENT_SAR1_MOVE,
    IQS62X_EVENT_SAR1_HALT,
    IQS62X_EVENT_WHEEL_UP,
    IQS62X_EVENT_WHEEL_DN,

// switches
    IQS62X_EVENT_HALL_N_T,
    IQS62X_EVENT_HALL_N_P,
    IQS62X_EVENT_HALL_S_T,
    IQS62X_EVENT_HALL_S_P,

// everything else
    IQS62X_EVENT_SYS_RESET,
    IQS62X_EVENT_SYS_ATI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs62x_event_data {
    pub ui_data: u16,
    pub als_flags: u8,
    pub ir_flags: u8,
    pub interval: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs62x_event_desc {
    pub reg: iqs62x_event_reg,
    pub mask: u8,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs62x_dev_desc {
    pub dev_name: *const c_char,
    pub sub_devs: *const mfd_cell,
    pub num_sub_devs: c_int,
    pub prod_num: u8,
    pub sw_num: u8,
    pub cal_regs: *const u8,
    pub num_cal_regs: c_int,
    pub prox_mask: u8,
    pub sar_mask: u8,
    pub hall_mask: u8,
    pub hyst_mask: u8,
    pub temp_mask: u8,
    pub als_mask: u8,
    pub ir_mask: u8,
    pub prox_settings: u8,
    pub als_flags: u8,
    pub hall_flags: u8,
    pub hyst_shift: u8,
    pub interval: u8,
    pub interval_div: u8,
    pub fw_name: *const c_char,
    pub (*event_regs)[IQS62X_EVENT_SIZE]: *const iqs62x_event_reg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs62x_core {
    pub dev_desc: *const iqs62x_dev_desc,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub nh: blocking_notifier_head,
    pub fw_blk_head: list_head,
    pub ati_done: completion,
    pub fw_done: completion,
    pub ui_sel: iqs62x_ui_sel,
    pub event_cache: c_ulong,
    pub sw_num: u8,
    pub hw_num: u8,
}
