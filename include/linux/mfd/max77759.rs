//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/max77759.h
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
// Copyright 2020 Google Inc.
// Copyright 2025 Linaro Ltd.
//
// Maxim MAX77759 core driver
//

pub const MAX77759_PMIC_REG_PMIC_ID: c_uint = 0x00;
pub const MAX77759_PMIC_REG_PMIC_REVISION: c_uint = 0x01;
pub const MAX77759_PMIC_REG_OTP_REVISION: c_uint = 0x02;
pub const MAX77759_PMIC_REG_INTSRC: c_uint = 0x22;
pub const MAX77759_PMIC_REG_INTSRCMASK: c_uint = 0x23;

pub const MAX77759_PMIC_REG_TOPSYS_INT: c_uint = 0x24;
pub const MAX77759_PMIC_REG_TOPSYS_INT_MASK: c_uint = 0x26;

pub const MAX77759_PMIC_REG_I2C_CNFG: c_uint = 0x40;
pub const MAX77759_PMIC_REG_SWRESET: c_uint = 0x50;
pub const MAX77759_PMIC_REG_CONTROL_FG: c_uint = 0x51;
pub const MAX77759_MAXQ_REG_UIC_INT1: c_uint = 0x64;

pub const MAX77759_MAXQ_REG_UIC_INT2: c_uint = 0x65;
pub const MAX77759_MAXQ_REG_UIC_INT3: c_uint = 0x66;
pub const MAX77759_MAXQ_REG_UIC_INT4: c_uint = 0x67;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS1: c_uint = 0x68;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS2: c_uint = 0x69;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS3: c_uint = 0x6a;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS4: c_uint = 0x6b;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS5: c_uint = 0x6c;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS6: c_uint = 0x6d;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS7: c_uint = 0x6f;
pub const MAX77759_MAXQ_REG_UIC_UIC_STATUS8: c_uint = 0x6f;
pub const MAX77759_MAXQ_REG_UIC_INT1_M: c_uint = 0x70;
pub const MAX77759_MAXQ_REG_UIC_INT2_M: c_uint = 0x71;
pub const MAX77759_MAXQ_REG_UIC_INT3_M: c_uint = 0x72;
pub const MAX77759_MAXQ_REG_UIC_INT4_M: c_uint = 0x73;
pub const MAX77759_MAXQ_REG_AP_DATAOUT0: c_uint = 0x81;
pub const MAX77759_MAXQ_REG_AP_DATAOUT32: c_uint = 0xa1;
pub const MAX77759_MAXQ_REG_AP_DATAIN0: c_uint = 0xb1;
pub const MAX77759_MAXQ_REG_UIC_SWRST: c_uint = 0xe0;
pub const MAX77759_CHGR_REG_CHG_INT: c_uint = 0xb0;

pub const MAX77759_CHGR_REG_CHG_INT2: c_uint = 0xb1;

pub const MAX77759_CHGR_REG_CHG_INT_MASK: c_uint = 0xb2;
pub const MAX77759_CHGR_REG_CHG_INT2_MASK: c_uint = 0xb3;
pub const MAX77759_CHGR_REG_CHG_INT_OK: c_uint = 0xb4;
pub const MAX77759_CHGR_REG_CHG_DETAILS_00: c_uint = 0xb5;

pub const MAX77759_CHGR_REG_CHG_DETAILS_01: c_uint = 0xb6;

pub const MAX77759_CHGR_REG_CHG_DETAILS_02: c_uint = 0xb7;

pub const MAX77759_CHGR_REG_CHG_DETAILS_03: c_uint = 0xb8;
pub const MAX77759_CHGR_REG_CHG_CNFG_00: c_uint = 0xb9;

pub const MAX77759_CHGR_REG_CHG_CNFG_01: c_uint = 0xba;
pub const MAX77759_CHGR_REG_CHG_CNFG_02: c_uint = 0xbb;

pub const MAX77759_CHGR_REG_CHG_CNFG_03: c_uint = 0xbc;
pub const MAX77759_CHGR_REG_CHG_CNFG_04: c_uint = 0xbd;

pub const MAX77759_CHGR_REG_CHG_CNFG_05: c_uint = 0xbe;
pub const MAX77759_CHGR_REG_CHG_CNFG_06: c_uint = 0xbf;

pub const MAX77759_CHGR_REG_CHG_CNFG_07: c_uint = 0xc0;
pub const MAX77759_CHGR_REG_CHG_CNFG_08: c_uint = 0xc1;
pub const MAX77759_CHGR_REG_CHG_CNFG_09: c_uint = 0xc2;

pub const MAX77759_CHGR_REG_CHG_CNFG_10: c_uint = 0xc3;
pub const MAX77759_CHGR_REG_CHG_CNFG_11: c_uint = 0xc4;
pub const MAX77759_CHGR_REG_CHG_CNFG_12: c_uint = 0xc5;
// Setting this enables the Wireless Charging input channel.

// Setting this enables the CHGIN/USB input channel.

pub const MAX77759_CHGR_REG_CHG_CNFG_13: c_uint = 0xc6;
pub const MAX77759_CHGR_REG_CHG_CNFG_14: c_uint = 0xc7;
pub const MAX77759_CHGR_REG_CHG_CNFG_15: c_uint = 0xc8;
pub const MAX77759_CHGR_REG_CHG_CNFG_16: c_uint = 0xc9;
pub const MAX77759_CHGR_REG_CHG_CNFG_17: c_uint = 0xca;
pub const MAX77759_CHGR_REG_CHG_CNFG_18: c_uint = 0xcb;

pub const MAX77759_CHGR_REG_CHG_CNFG_19: c_uint = 0xcc;
// MaxQ opcodes for max77759_maxq_command()

pub const MAX77759_MAXQ_OPCODE_GPIO_TRIGGER_READ: c_uint = 0x21;
pub const MAX77759_MAXQ_OPCODE_GPIO_TRIGGER_WRITE: c_uint = 0x22;
pub const MAX77759_MAXQ_OPCODE_GPIO_CONTROL_READ: c_uint = 0x23;
pub const MAX77759_MAXQ_OPCODE_GPIO_CONTROL_WRITE: c_uint = 0x24;
pub const MAX77759_MAXQ_OPCODE_USER_SPACE_READ: c_uint = 0x81;
pub const MAX77759_MAXQ_OPCODE_USER_SPACE_WRITE: c_uint = 0x82;
//
// enum max77759_chgr_chgin_dtls_status - Charger Input Status
// @MAX77759_CHGR_CHGIN_DTLS_VBUS_UNDERVOLTAGE:
// Charger input voltage (Vchgin) < Under Voltage Threshold (Vuvlo)
// @MAX77759_CHGR_CHGIN_DTLS_VBUS_MARGINAL_VOLTAGE:
// Vchgin > Vuvlo and Vchgin < (Battery Voltage (Vbatt) + system voltage (Vsys))
// @MAX77759_CHGR_CHGIN_DTLS_VBUS_OVERVOLTAGE:
// Vchgin > Over Voltage threshold (Vovlo)
// @MAX77759_CHGR_CHGIN_DTLS_VBUS_VALID:
// Vchgin > Vuvlo, Vchgin < Vovlo and Vchgin > (Vsys + Vbatt)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77759_chgr_chgin_dtls_status {
    MAX77759_CHGR_CHGIN_DTLS_VBUS_UNDERVOLTAGE,
    MAX77759_CHGR_CHGIN_DTLS_VBUS_MARGINAL_VOLTAGE,
    MAX77759_CHGR_CHGIN_DTLS_VBUS_OVERVOLTAGE,
    MAX77759_CHGR_CHGIN_DTLS_VBUS_VALID,
}

//
// enum max77759_chgr_bat_dtls_states - Battery Details
// @MAX77759_CHGR_BAT_DTLS_NO_BATT_CHG_SUSP:	No battery and the charger suspended
// @MAX77759_CHGR_BAT_DTLS_DEAD_BATTERY:	Vbatt < Vtrickle
// @MAX77759_CHGR_BAT_DTLS_BAT_CHG_TIMER_FAULT:	Charging suspended due to timer fault
// @MAX77759_CHGR_BAT_DTLS_BAT_OKAY:		Battery okay and Vbatt > Min Sys Voltage (Vsysmin)
// @MAX77759_CHGR_BAT_DTLS_BAT_UNDERVOLTAGE:	Battery is okay. Vtrickle < Vbatt < Vsysmin
// @MAX77759_CHGR_BAT_DTLS_BAT_OVERVOLTAGE:	Battery voltage > Overvoltage threshold
// @MAX77759_CHGR_BAT_DTLS_BAT_OVERCURRENT:	Battery current exceeds overcurrent threshold
// @MAX77759_CHGR_BAT_DTLS_BAT_ONLY_MODE:	Battery only mode and battery level not available
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77759_chgr_bat_dtls_states {
    MAX77759_CHGR_BAT_DTLS_NO_BATT_CHG_SUSP,
    MAX77759_CHGR_BAT_DTLS_DEAD_BATTERY,
    MAX77759_CHGR_BAT_DTLS_BAT_CHG_TIMER_FAULT,
    MAX77759_CHGR_BAT_DTLS_BAT_OKAY,
    MAX77759_CHGR_BAT_DTLS_BAT_UNDERVOLTAGE,
    MAX77759_CHGR_BAT_DTLS_BAT_OVERVOLTAGE,
    MAX77759_CHGR_BAT_DTLS_BAT_OVERCURRENT,
    MAX77759_CHGR_BAT_DTLS_BAT_ONLY_MODE,
}

//
// enum max77759_chgr_chg_dtls_states - Charger Details
// @MAX77759_CHGR_CHG_DTLS_PREQUAL:		Charger in prequalification mode
// @MAX77759_CHGR_CHG_DTLS_CC:			Charger in fast charge const curr mode
// @MAX77759_CHGR_CHG_DTLS_CV:			Charger in fast charge const voltage mode
// @MAX77759_CHGR_CHG_DTLS_TO:			Charger is in top off mode
// @MAX77759_CHGR_CHG_DTLS_DONE:		Charger is done
// @MAX77759_CHGR_CHG_DTLS_RSVD_1:		Reserved
// @MAX77759_CHGR_CHG_DTLS_TIMER_FAULT:		Charger is in timer fault mode
// @MAX77759_CHGR_CHG_DTLS_SUSP_BATT_THM:	Charger is suspended as battery removal detected
// @MAX77759_CHGR_CHG_DTLS_OFF:			Charger is off. Input invalid or charger disabled
// @MAX77759_CHGR_CHG_DTLS_RSVD_2:		Reserved
// @MAX77759_CHGR_CHG_DTLS_RSVD_3:		Reserved
// @MAX77759_CHGR_CHG_DTLS_OFF_WDOG_TIMER:	Charger is off as watchdog timer expired
// @MAX77759_CHGR_CHG_DTLS_SUSP_JEITA:		Charger is in JEITA control mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77759_chgr_chg_dtls_states {
    MAX77759_CHGR_CHG_DTLS_PREQUAL,
    MAX77759_CHGR_CHG_DTLS_CC,
    MAX77759_CHGR_CHG_DTLS_CV,
    MAX77759_CHGR_CHG_DTLS_TO,
    MAX77759_CHGR_CHG_DTLS_DONE,
    MAX77759_CHGR_CHG_DTLS_RSVD_1,
    MAX77759_CHGR_CHG_DTLS_TIMER_FAULT,
    MAX77759_CHGR_CHG_DTLS_SUSP_BATT_THM,
    MAX77759_CHGR_CHG_DTLS_OFF,
    MAX77759_CHGR_CHG_DTLS_RSVD_2,
    MAX77759_CHGR_CHG_DTLS_RSVD_3,
    MAX77759_CHGR_CHG_DTLS_OFF_WDOG_TIMER,
    MAX77759_CHGR_CHG_DTLS_SUSP_JEITA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77759_chgr_mode {
    MAX77759_CHGR_MODE_OFF = 0x0,
    MAX77759_CHGR_MODE_CHG_BUCK_ON = 0x5,
    MAX77759_CHGR_MODE_OTG_BOOST_ON = 0xA,
}

//
// struct max77759 - core max77759 internal data structure
//
// @regmap_top: Regmap for accessing TOP registers
// @maxq_lock: Lock for serializing access to MaxQ
// @regmap_maxq: Regmap for accessing MaxQ registers
// @cmd_done: Used to signal completion of a MaxQ command
// @regmap_charger: Regmap for accessing charger registers
//
// The MAX77759 comprises several sub-blocks, namely TOP, MaxQ, Charger,
// Fuel Gauge, and TCPCI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77759 {
    pub regmap_top: *mut regmap,
// This protects MaxQ commands - only one can be active
    pub maxq_lock: mutex,
    pub regmap_maxq: *mut regmap,
    pub cmd_done: completion,
    pub regmap_charger: *mut regmap,
}

//
// struct max77759_maxq_command - structure containing the MaxQ command to
// send
//
// @length: The number of bytes to send.
// @cmd: The data to send.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77759_maxq_command {
    pub length: u8,
    pub __counted_by(length): u8 cmd[],
}

//
// struct max77759_maxq_response - structure containing the MaxQ response
//
// @length: The number of bytes to receive.
// @rsp: The data received. Must have at least @length bytes space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77759_maxq_response {
    pub length: u8,
    pub __counted_by(length): u8 rsp[],
}

//
// max77759_maxq_command() - issue a MaxQ command and wait for the response
// and associated data
//
// @max77759: The core max77759 device handle.
// @cmd: The command to be sent.
// @rsp: Any response data associated with the command will be copied here;
// can be %NULL if the command has no response (other than ACK).
//
// Return: 0 on success, a negative error number otherwise.
//
