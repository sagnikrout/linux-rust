//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/extcon/extcon-rt8973a.h
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
// rt8973a.h
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt8973a_types {
    TYPE_RT8973A,
}

// RT8973A registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt8973A_reg {
    RT8973A_REG_DEVICE_ID = 0x1,
    RT8973A_REG_CONTROL1,
    RT8973A_REG_INT1,
    RT8973A_REG_INT2,
    RT8973A_REG_INTM1,
    RT8973A_REG_INTM2,
    RT8973A_REG_ADC,
    RT8973A_REG_RSVD_1,
    RT8973A_REG_RSVD_2,
    RT8973A_REG_DEV1,
    RT8973A_REG_DEV2,
    RT8973A_REG_RSVD_3,
    RT8973A_REG_RSVD_4,
    RT8973A_REG_RSVD_5,
    RT8973A_REG_RSVD_6,
    RT8973A_REG_RSVD_7,
    RT8973A_REG_RSVD_8,
    RT8973A_REG_RSVD_9,
    RT8973A_REG_MANUAL_SW1,
    RT8973A_REG_MANUAL_SW2,
    RT8973A_REG_RSVD_10,
    RT8973A_REG_RSVD_11,
    RT8973A_REG_RSVD_12,
    RT8973A_REG_RSVD_13,
    RT8973A_REG_RSVD_14,
    RT8973A_REG_RSVD_15,
    RT8973A_REG_RESET,

    RT8973A_REG_END,
}

// Define RT8973A MASK/SHIFT constant
pub const RT8973A_REG_DEVICE_ID_VENDOR_SHIFT: c_int = 0;
pub const RT8973A_REG_DEVICE_ID_VERSION_SHIFT: c_int = 3;

pub const RT8973A_REG_CONTROL1_INTM_SHIFT: c_int = 0;
pub const RT8973A_REG_CONTROL1_AUTO_CONFIG_SHIFT: c_int = 2;
pub const RT8973A_REG_CONTROL1_I2C_RST_EN_SHIFT: c_int = 3;
pub const RT8973A_REG_CONTROL1_SWITCH_OPEN_SHIFT: c_int = 4;
pub const RT8973A_REG_CONTROL1_CHGTYP_SHIFT: c_int = 5;
pub const RT8973A_REG_CONTROL1_USB_CHD_EN_SHIFT: c_int = 6;
pub const RT8973A_REG_CONTROL1_ADC_EN_SHIFT: c_int = 7;

pub const RT9873A_REG_INTM1_ATTACH_SHIFT: c_int = 0;
pub const RT9873A_REG_INTM1_DETACH_SHIFT: c_int = 1;
pub const RT9873A_REG_INTM1_CHGDET_SHIFT: c_int = 2;
pub const RT9873A_REG_INTM1_DCD_T_SHIFT: c_int = 3;
pub const RT9873A_REG_INTM1_OVP_SHIFT: c_int = 4;
pub const RT9873A_REG_INTM1_CONNECT_SHIFT: c_int = 5;
pub const RT9873A_REG_INTM1_ADC_CHG_SHIFT: c_int = 6;
pub const RT9873A_REG_INTM1_OTP_SHIFT: c_int = 7;

pub const RT9873A_REG_INTM2_UVLO_SHIFT: c_int = 1;
pub const RT9873A_REG_INTM2_POR_SHIFT: c_int = 2;
pub const RT9873A_REG_INTM2_OTP_FET_SHIFT: c_int = 3;
pub const RT9873A_REG_INTM2_OVP_FET_SHIFT: c_int = 4;
pub const RT9873A_REG_INTM2_OCP_LATCH_SHIFT: c_int = 5;
pub const RT9873A_REG_INTM2_OCP_SHIFT: c_int = 6;
pub const RT9873A_REG_INTM2_OVP_OCP_SHIFT: c_int = 7;

pub const RT8973A_REG_ADC_SHIFT: c_int = 0;

pub const RT8973A_REG_DEV1_OTG_SHIFT: c_int = 0;
pub const RT8973A_REG_DEV1_SDP_SHIFT: c_int = 2;
pub const RT8973A_REG_DEV1_UART_SHIFT: c_int = 3;
pub const RT8973A_REG_DEV1_CAR_KIT_TYPE1_SHIFT: c_int = 4;
pub const RT8973A_REG_DEV1_CDPORT_SHIFT: c_int = 5;
pub const RT8973A_REG_DEV1_DCPORT_SHIFT: c_int = 6;

pub const RT8973A_REG_DEV2_JIG_USB_ON_SHIFT: c_int = 0;
pub const RT8973A_REG_DEV2_JIG_USB_OFF_SHIFT: c_int = 1;
pub const RT8973A_REG_DEV2_JIG_UART_ON_SHIFT: c_int = 2;
pub const RT8973A_REG_DEV2_JIG_UART_OFF_SHIFT: c_int = 3;

pub const RT8973A_REG_MANUAL_SW1_DP_SHIFT: c_int = 2;
pub const RT8973A_REG_MANUAL_SW1_DM_SHIFT: c_int = 5;

pub const DM_DP_CON_SWITCH_OPEN: c_uint = 0x0;
pub const DM_DP_CON_SWITCH_USB: c_uint = 0x1;
pub const DM_DP_CON_SWITCH_UART: c_uint = 0x3;

pub const RT8973A_REG_MANUAL_SW2_FET_ON_SHIFT: c_int = 0;
pub const RT8973A_REG_MANUAL_SW2_JIG_ON_SHIFT: c_int = 2;
pub const RT8973A_REG_MANUAL_SW2_BOOT_SW_SHIFT: c_int = 3;

pub const RT8973A_REG_MANUAL_SW2_FET_ON: c_int = 0;
pub const RT8973A_REG_MANUAL_SW2_FET_OFF: c_uint = 0x1;
pub const RT8973A_REG_MANUAL_SW2_JIG_OFF: c_int = 0;
pub const RT8973A_REG_MANUAL_SW2_JIG_ON: c_uint = 0x1;
pub const RT8973A_REG_MANUAL_SW2_BOOT_SW_ON: c_int = 0;
pub const RT8973A_REG_MANUAL_SW2_BOOT_SW_OFF: c_uint = 0x1;
pub const RT8973A_REG_RESET_SHIFT: c_int = 0;

pub const RT8973A_REG_RESET: c_uint = 0x1;
// RT8973A Interrupts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt8973a_irq {
// Interrupt1
    RT8973A_INT1_ATTACH,
    RT8973A_INT1_DETACH,
    RT8973A_INT1_CHGDET,
    RT8973A_INT1_DCD_T,
    RT8973A_INT1_OVP,
    RT8973A_INT1_CONNECT,
    RT8973A_INT1_ADC_CHG,
    RT8973A_INT1_OTP,

// Interrupt2
    RT8973A_INT2_UVLO,
    RT8973A_INT2_POR,
    RT8973A_INT2_OTP_FET,
    RT8973A_INT2_OVP_FET,
    RT8973A_INT2_OCP_LATCH,
    RT8973A_INT2_OCP,
    RT8973A_INT2_OVP_OCP,

    RT8973A_NUM,
}

