//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/extcon/extcon-sm5502.h
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
// sm5502.h
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd
//
// SM5502 registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sm5502_reg {
    SM5502_REG_DEVICE_ID = 0x01,
    SM5502_REG_CONTROL,
    SM5502_REG_INT1,
    SM5502_REG_INT2,
    SM5502_REG_INTMASK1,
    SM5502_REG_INTMASK2,
    SM5502_REG_ADC,
    SM5502_REG_TIMING_SET1,
    SM5502_REG_TIMING_SET2,
    SM5502_REG_DEV_TYPE1,
    SM5502_REG_DEV_TYPE2,
    SM5502_REG_BUTTON1,
    SM5502_REG_BUTTON2,
    SM5502_REG_CAR_KIT_STATUS,
    SM5502_REG_RSVD1,
    SM5502_REG_RSVD2,
    SM5502_REG_RSVD3,
    SM5502_REG_RSVD4,
    SM5502_REG_MANUAL_SW1,
    SM5502_REG_MANUAL_SW2,
    SM5502_REG_DEV_TYPE3,
    SM5502_REG_RSVD5,
    SM5502_REG_RSVD6,
    SM5502_REG_RSVD7,
    SM5502_REG_RSVD8,
    SM5502_REG_RSVD9,
    SM5502_REG_RESET,
    SM5502_REG_RSVD10,
    SM5502_REG_RESERVED_ID1,
    SM5502_REG_RSVD11,
    SM5502_REG_RSVD12,
    SM5502_REG_RESERVED_ID2,
    SM5502_REG_RSVD13,
    SM5502_REG_OCP,
    SM5502_REG_RSVD14,
    SM5502_REG_RSVD15,
    SM5502_REG_RSVD16,
    SM5502_REG_RSVD17,
    SM5502_REG_RSVD18,
    SM5502_REG_RSVD19,
    SM5502_REG_RSVD20,
    SM5502_REG_RSVD21,
    SM5502_REG_RSVD22,
    SM5502_REG_RSVD23,
    SM5502_REG_RSVD24,
    SM5502_REG_RSVD25,
    SM5502_REG_RSVD26,
    SM5502_REG_RSVD27,
    SM5502_REG_RSVD28,
    SM5502_REG_RSVD29,
    SM5502_REG_RSVD30,
    SM5502_REG_RSVD31,
    SM5502_REG_RSVD32,
    SM5502_REG_RSVD33,
    SM5502_REG_RSVD34,
    SM5502_REG_RSVD35,
    SM5502_REG_RSVD36,
    SM5502_REG_RESERVED_ID3,

    SM5502_REG_END,
}

// Define SM5502 MASK/SHIFT constant
pub const SM5502_REG_DEVICE_ID_VENDOR_SHIFT: c_int = 0;
pub const SM5502_REG_DEVICE_ID_VERSION_SHIFT: c_int = 3;

pub const SM5502_REG_CONTROL_MASK_INT_SHIFT: c_int = 0;
pub const SM5502_REG_CONTROL_WAIT_SHIFT: c_int = 1;
pub const SM5502_REG_CONTROL_MANUAL_SW_SHIFT: c_int = 2;
pub const SM5502_REG_CONTROL_RAW_DATA_SHIFT: c_int = 3;
pub const SM5502_REG_CONTROL_SW_OPEN_SHIFT: c_int = 4;

pub const SM5504_REG_CONTROL_CHGTYP_SHIFT: c_int = 5;
pub const SM5504_REG_CONTROL_USBCHDEN_SHIFT: c_int = 6;
pub const SM5504_REG_CONTROL_ADC_EN_SHIFT: c_int = 7;

pub const SM5502_REG_INTM1_ATTACH_SHIFT: c_int = 0;
pub const SM5502_REG_INTM1_DETACH_SHIFT: c_int = 1;
pub const SM5502_REG_INTM1_KP_SHIFT: c_int = 2;
pub const SM5502_REG_INTM1_LKP_SHIFT: c_int = 3;
pub const SM5502_REG_INTM1_LKR_SHIFT: c_int = 4;
pub const SM5502_REG_INTM1_OVP_EVENT_SHIFT: c_int = 5;
pub const SM5502_REG_INTM1_OCP_EVENT_SHIFT: c_int = 6;
pub const SM5502_REG_INTM1_OVP_OCP_DIS_SHIFT: c_int = 7;

pub const SM5502_REG_INTM2_VBUS_DET_SHIFT: c_int = 0;
pub const SM5502_REG_INTM2_REV_ACCE_SHIFT: c_int = 1;
pub const SM5502_REG_INTM2_ADC_CHG_SHIFT: c_int = 2;
pub const SM5502_REG_INTM2_STUCK_KEY_SHIFT: c_int = 3;
pub const SM5502_REG_INTM2_STUCK_KEY_RCV_SHIFT: c_int = 4;
pub const SM5502_REG_INTM2_MHL_SHIFT: c_int = 5;

pub const SM5504_REG_INTM1_ATTACH_SHIFT: c_int = 0;
pub const SM5504_REG_INTM1_DETACH_SHIFT: c_int = 1;
pub const SM5504_REG_INTM1_CHG_DET_SHIFT: c_int = 2;
pub const SM5504_REG_INTM1_DCD_OUT_SHIFT: c_int = 3;
pub const SM5504_REG_INTM1_OVP_EVENT_SHIFT: c_int = 4;
pub const SM5504_REG_INTM1_CONNECT_SHIFT: c_int = 5;
pub const SM5504_REG_INTM1_ADC_CHG_SHIFT: c_int = 6;

pub const SM5504_REG_INTM2_RID_CHG_SHIFT: c_int = 0;
pub const SM5504_REG_INTM2_UVLO_SHIFT: c_int = 1;
pub const SM5504_REG_INTM2_POR_SHIFT: c_int = 2;
pub const SM5504_REG_INTM2_OVP_FET_SHIFT: c_int = 4;
pub const SM5504_REG_INTM2_OCP_LATCH_SHIFT: c_int = 5;
pub const SM5504_REG_INTM2_OCP_EVENT_SHIFT: c_int = 6;
pub const SM5504_REG_INTM2_OVP_OCP_EVENT_SHIFT: c_int = 7;

pub const SM5502_REG_ADC_SHIFT: c_int = 0;

pub const SM5502_REG_TIMING_SET1_KEY_PRESS_SHIFT: c_int = 4;

pub const TIMING_KEY_PRESS_100MS: c_uint = 0x0;
pub const TIMING_KEY_PRESS_200MS: c_uint = 0x1;
pub const TIMING_KEY_PRESS_300MS: c_uint = 0x2;
pub const TIMING_KEY_PRESS_400MS: c_uint = 0x3;
pub const TIMING_KEY_PRESS_500MS: c_uint = 0x4;
pub const TIMING_KEY_PRESS_600MS: c_uint = 0x5;
pub const TIMING_KEY_PRESS_700MS: c_uint = 0x6;
pub const TIMING_KEY_PRESS_800MS: c_uint = 0x7;
pub const TIMING_KEY_PRESS_900MS: c_uint = 0x8;
pub const TIMING_KEY_PRESS_1000MS: c_uint = 0x9;
pub const SM5502_REG_TIMING_SET1_ADC_DET_SHIFT: c_int = 0;

pub const TIMING_ADC_DET_50MS: c_uint = 0x0;
pub const TIMING_ADC_DET_100MS: c_uint = 0x1;
pub const TIMING_ADC_DET_150MS: c_uint = 0x2;
pub const TIMING_ADC_DET_200MS: c_uint = 0x3;
pub const TIMING_ADC_DET_300MS: c_uint = 0x4;
pub const TIMING_ADC_DET_400MS: c_uint = 0x5;
pub const TIMING_ADC_DET_500MS: c_uint = 0x6;
pub const TIMING_ADC_DET_600MS: c_uint = 0x7;
pub const TIMING_ADC_DET_700MS: c_uint = 0x8;
pub const TIMING_ADC_DET_800MS: c_uint = 0x9;
pub const TIMING_ADC_DET_900MS: c_uint = 0xA;
pub const TIMING_ADC_DET_1000MS: c_uint = 0xB;
pub const SM5502_REG_TIMING_SET2_SW_WAIT_SHIFT: c_int = 4;

pub const TIMING_SW_WAIT_10MS: c_uint = 0x0;
pub const TIMING_SW_WAIT_30MS: c_uint = 0x1;
pub const TIMING_SW_WAIT_50MS: c_uint = 0x2;
pub const TIMING_SW_WAIT_70MS: c_uint = 0x3;
pub const TIMING_SW_WAIT_90MS: c_uint = 0x4;
pub const TIMING_SW_WAIT_110MS: c_uint = 0x5;
pub const TIMING_SW_WAIT_130MS: c_uint = 0x6;
pub const TIMING_SW_WAIT_150MS: c_uint = 0x7;
pub const TIMING_SW_WAIT_170MS: c_uint = 0x8;
pub const TIMING_SW_WAIT_190MS: c_uint = 0x9;
pub const TIMING_SW_WAIT_210MS: c_uint = 0xA;
pub const SM5502_REG_TIMING_SET2_LONG_KEY_SHIFT: c_int = 0;

pub const TIMING_LONG_KEY_300MS: c_uint = 0x0;
pub const TIMING_LONG_KEY_400MS: c_uint = 0x1;
pub const TIMING_LONG_KEY_500MS: c_uint = 0x2;
pub const TIMING_LONG_KEY_600MS: c_uint = 0x3;
pub const TIMING_LONG_KEY_700MS: c_uint = 0x4;
pub const TIMING_LONG_KEY_800MS: c_uint = 0x5;
pub const TIMING_LONG_KEY_900MS: c_uint = 0x6;
pub const TIMING_LONG_KEY_1000MS: c_uint = 0x7;
pub const TIMING_LONG_KEY_1100MS: c_uint = 0x8;
pub const TIMING_LONG_KEY_1200MS: c_uint = 0x9;
pub const TIMING_LONG_KEY_1300MS: c_uint = 0xA;
pub const TIMING_LONG_KEY_1400MS: c_uint = 0xB;
pub const TIMING_LONG_KEY_1500MS: c_uint = 0xC;
pub const SM5502_REG_DEV_TYPE1_AUDIO_TYPE1_SHIFT: c_int = 0;
pub const SM5502_REG_DEV_TYPE1_AUDIO_TYPE2_SHIFT: c_int = 1;
pub const SM5502_REG_DEV_TYPE1_USB_SDP_SHIFT: c_int = 2;
pub const SM5502_REG_DEV_TYPE1_UART_SHIFT: c_int = 3;
pub const SM5502_REG_DEV_TYPE1_CAR_KIT_CHARGER_SHIFT: c_int = 4;
pub const SM5502_REG_DEV_TYPE1_USB_CHG_SHIFT: c_int = 5;
pub const SM5502_REG_DEV_TYPE1_DEDICATED_CHG_SHIFT: c_int = 6;
pub const SM5502_REG_DEV_TYPE1_USB_OTG_SHIFT: c_int = 7;

pub const SM5504_REG_DEV_TYPE1_USB_OTG_SHIFT: c_int = 0;

pub const SM5502_REG_DEV_TYPE2_JIG_USB_ON_SHIFT: c_int = 0;
pub const SM5502_REG_DEV_TYPE2_JIG_USB_OFF_SHIFT: c_int = 1;
pub const SM5502_REG_DEV_TYPE2_JIG_UART_ON_SHIFT: c_int = 2;
pub const SM5502_REG_DEV_TYPE2_JIG_UART_OFF_SHIFT: c_int = 3;
pub const SM5502_REG_DEV_TYPE2_PPD_SHIFT: c_int = 4;
pub const SM5502_REG_DEV_TYPE2_TTY_SHIFT: c_int = 5;
pub const SM5502_REG_DEV_TYPE2_AV_CABLE_SHIFT: c_int = 6;

pub const SM5502_REG_MANUAL_SW1_VBUSIN_SHIFT: c_int = 0;
pub const SM5502_REG_MANUAL_SW1_DP_SHIFT: c_int = 2;
pub const SM5502_REG_MANUAL_SW1_DM_SHIFT: c_int = 5;

pub const VBUSIN_SWITCH_OPEN: c_uint = 0x0;
pub const VBUSIN_SWITCH_VBUSOUT: c_uint = 0x1;
pub const VBUSIN_SWITCH_MIC: c_uint = 0x2;
pub const VBUSIN_SWITCH_VBUSOUT_WITH_USB: c_uint = 0x3;
pub const DM_DP_CON_SWITCH_OPEN: c_uint = 0x0;
pub const DM_DP_CON_SWITCH_USB: c_uint = 0x1;
pub const DM_DP_CON_SWITCH_AUDIO: c_uint = 0x2;
pub const DM_DP_CON_SWITCH_UART: c_uint = 0x3;

// SM5502 Interrupts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sm5502_irq {
// INT1
    SM5502_IRQ_INT1_ATTACH,
    SM5502_IRQ_INT1_DETACH,
    SM5502_IRQ_INT1_KP,
    SM5502_IRQ_INT1_LKP,
    SM5502_IRQ_INT1_LKR,
    SM5502_IRQ_INT1_OVP_EVENT,
    SM5502_IRQ_INT1_OCP_EVENT,
    SM5502_IRQ_INT1_OVP_OCP_DIS,

// INT2
    SM5502_IRQ_INT2_VBUS_DET,
    SM5502_IRQ_INT2_REV_ACCE,
    SM5502_IRQ_INT2_ADC_CHG,
    SM5502_IRQ_INT2_STUCK_KEY,
    SM5502_IRQ_INT2_STUCK_KEY_RCV,
    SM5502_IRQ_INT2_MHL,

    SM5502_IRQ_NUM,
}

// SM5504 Interrupts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sm5504_irq {
// INT1
    SM5504_IRQ_INT1_ATTACH,
    SM5504_IRQ_INT1_DETACH,
    SM5504_IRQ_INT1_CHG_DET,
    SM5504_IRQ_INT1_DCD_OUT,
    SM5504_IRQ_INT1_OVP_EVENT,
    SM5504_IRQ_INT1_CONNECT,
    SM5504_IRQ_INT1_ADC_CHG,

// INT2
    SM5504_IRQ_INT2_RID_CHG,
    SM5504_IRQ_INT2_UVLO,
    SM5504_IRQ_INT2_POR,
    SM5504_IRQ_INT2_OVP_FET,
    SM5504_IRQ_INT2_OCP_LATCH,
    SM5504_IRQ_INT2_OCP_EVENT,
    SM5504_IRQ_INT2_OVP_OCP_EVENT,

    SM5504_IRQ_NUM,
}

