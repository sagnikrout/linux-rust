//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/tcpci.h
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
// Copyright 2015-2017 Google, Inc
//
// USB Type-C Port Controller Interface.
//

pub const TCPC_VENDOR_ID: c_uint = 0x0;
pub const TCPC_PRODUCT_ID: c_uint = 0x2;
pub const TCPC_BCD_DEV: c_uint = 0x4;
pub const TCPC_TC_REV: c_uint = 0x6;
pub const TCPC_PD_REV: c_uint = 0x8;
pub const TCPC_PD_INT_REV: c_uint = 0xa;
pub const TCPC_ALERT: c_uint = 0x10;

pub const TCPC_ALERT_MASK: c_uint = 0x12;
pub const TCPC_POWER_STATUS_MASK: c_uint = 0x14;
pub const TCPC_FAULT_STATUS_MASK: c_uint = 0x15;

pub const TCPC_EXTENDED_STATUS_MASK: c_uint = 0x16;

pub const TCPC_ALERT_EXTENDED_MASK: c_uint = 0x17;

pub const TCPC_CONFIG_STD_OUTPUT: c_uint = 0x18;

pub const TCPC_CONFIG_STD_OUTPUT_ORIENTATION_NORMAL: c_int = 0;
pub const TCPC_CONFIG_STD_OUTPUT_ORIENTATION_FLIPPED: c_int = 1;
pub const TCPC_TCPC_CTRL: c_uint = 0x19;

pub const PLUG_ORNT_CC1: c_int = 0;
pub const PLUG_ORNT_CC2: c_int = 1;

pub const TCPC_EXTENDED_STATUS: c_uint = 0x20;

pub const TCPC_ROLE_CTRL: c_uint = 0x1a;

pub const TCPC_ROLE_CTRL_RP_VAL_DEF: c_uint = 0x0;
pub const TCPC_ROLE_CTRL_RP_VAL_1_5: c_uint = 0x1;
pub const TCPC_ROLE_CTRL_RP_VAL_3_0: c_uint = 0x2;

pub const TCPC_ROLE_CTRL_CC_RA: c_uint = 0x0;
pub const TCPC_ROLE_CTRL_CC_RP: c_uint = 0x1;
pub const TCPC_ROLE_CTRL_CC_RD: c_uint = 0x2;
pub const TCPC_ROLE_CTRL_CC_OPEN: c_uint = 0x3;
pub const TCPC_FAULT_CTRL: c_uint = 0x1b;
pub const TCPC_POWER_CTRL: c_uint = 0x1c;

pub const TCPC_CC_STATUS: c_uint = 0x1d;

pub const TCPC_CC_STATUS_TERM_RP: c_int = 0;
pub const TCPC_CC_STATUS_TERM_RD: c_int = 1;

pub const TCPC_CC_STATE_SRC_OPEN: c_int = 0;
pub const TCPC_POWER_STATUS: c_uint = 0x1e;

pub const TCPC_FAULT_STATUS: c_uint = 0x1f;

pub const TCPC_ALERT_EXTENDED: c_uint = 0x21;
pub const TCPC_COMMAND: c_uint = 0x23;
pub const TCPC_CMD_WAKE_I2C: c_uint = 0x11;
pub const TCPC_CMD_DISABLE_VBUS_DETECT: c_uint = 0x22;
pub const TCPC_CMD_ENABLE_VBUS_DETECT: c_uint = 0x33;
pub const TCPC_CMD_DISABLE_SINK_VBUS: c_uint = 0x44;
pub const TCPC_CMD_SINK_VBUS: c_uint = 0x55;
pub const TCPC_CMD_DISABLE_SRC_VBUS: c_uint = 0x66;
pub const TCPC_CMD_SRC_VBUS_DEFAULT: c_uint = 0x77;
pub const TCPC_CMD_SRC_VBUS_HIGH: c_uint = 0x88;
pub const TCPC_CMD_LOOK4CONNECTION: c_uint = 0x99;
pub const TCPC_CMD_RXONEMORE: c_uint = 0xAA;
pub const TCPC_CMD_I2C_IDLE: c_uint = 0xFF;
pub const TCPC_DEV_CAP_1: c_uint = 0x24;
pub const TCPC_DEV_CAP_2: c_uint = 0x26;
pub const TCPC_STD_INPUT_CAP: c_uint = 0x28;
pub const TCPC_STD_OUTPUT_CAP: c_uint = 0x29;

pub const TCPC_MSG_HDR_INFO: c_uint = 0x2e;

pub const TCPC_RX_DETECT: c_uint = 0x2f;

pub const TCPC_RX_BYTE_CNT: c_uint = 0x30;
pub const TCPC_RX_BUF_FRAME_TYPE: c_uint = 0x31;
pub const TCPC_RX_BUF_FRAME_TYPE_SOP: c_int = 0;
pub const TCPC_RX_BUF_FRAME_TYPE_SOP1: c_int = 1;

pub const TCPC_RX_HDR: c_uint = 0x32;
pub const TCPC_RX_DATA: c_uint = 0x34 /* through 0x4f */;
pub const TCPC_TRANSMIT: c_uint = 0x50;

pub const TCPC_TX_BYTE_CNT: c_uint = 0x51;
pub const TCPC_TX_HDR: c_uint = 0x52;
pub const TCPC_TX_DATA: c_uint = 0x54 /* through 0x6f */;
pub const TCPC_VBUS_VOLTAGE: c_uint = 0x70;
pub const TCPC_VBUS_VOLTAGE_MASK: c_uint = 0x3ff;
pub const TCPC_VBUS_VOLTAGE_LSB_MV: c_int = 25;
pub const TCPC_VBUS_SINK_DISCONNECT_THRESH: c_uint = 0x72;
pub const TCPC_VBUS_SINK_DISCONNECT_THRESH_LSB_MV: c_int = 25;
pub const TCPC_VBUS_SINK_DISCONNECT_THRESH_MAX: c_uint = 0x3ff;
pub const TCPC_VBUS_STOP_DISCHARGE_THRESH: c_uint = 0x74;
pub const TCPC_VBUS_VOLTAGE_ALARM_HI_CFG: c_uint = 0x76;
pub const TCPC_VBUS_VOLTAGE_ALARM_LO_CFG: c_uint = 0x78;
// I2C_WRITE_BYTE_COUNT + 1 when TX_BUF_BYTE_x is only accessible I2C_WRITE_BYTE_COUNT
pub const TCPC_TRANSMIT_BUFFER_MAX_LEN: c_int = 31;

//
// @TX_BUF_BYTE_x_hidden:
// optional; Set when TX_BUF_BYTE_x can only be accessed through I2C_WRITE_BYTE_COUNT.
// @frs_sourcing_vbus:
// Optional; Callback to perform chip specific operations when FRS
// is sourcing vbus.
// @auto_discharge_disconnect:
// Optional; Enables TCPC to autonomously discharge vbus on disconnect.
// @vbus_vsafe0v:
// optional; Set when TCPC can detect whether vbus is at VSAFE0V.
// @set_partner_usb_comm_capable:
// Optional; The USB Communications Capable bit indicates if port
// partner is capable of communication over the USB data lines
// (e.g. D+/- or SS Tx/Rx). Called to notify the status of the bit.
// @check_contaminant:
// Optional; The callback is invoked when chiplevel drivers indicated
// that the USB port needs to be checked for contaminant presence.
// Chip level drivers are expected to check for contaminant and call
// tcpm_clean_port when the port is clean to put the port back into
// toggling state.
// @cable_comm_capable
// optional; Set when TCPC can communicate with cable plugs over SOP'
// @attempt_vconn_swap_discovery:
// Optional; The callback is called by the TCPM when the result of
// a Discover Identity request indicates that the port partner is
// a receptacle capable of modal operation. Chip level TCPCI drivers
// can implement their own policy to determine if and when a Vconn
// swap following Discover Identity on SOP' occurs.
// Return true when the TCPM is allowed to request a Vconn swap
// after Discovery Identity on SOP.
// @set_orientation:
// Optional; Enable setting the connector orientation
// CONFIG_STANDARD_OUTPUT (0x18) bit0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcpci_data {
    pub regmap: *mut regmap,
    pub TX_BUF_BYTE_x_hidden:1: c_uchar,
    pub auto_discharge_disconnect:1: c_uchar,
    pub vbus_vsafe0v:1: c_uchar,
    pub cable_comm_capable:1: c_uchar,
    pub set_orientation:1: c_uchar,
    pub data): *mut *mut *mut int (init)(struct tcpci tcpci, struct tcpci_data,
    pub enable): bool,
    pub cc): typec_cc_status,
    pub sink): *mut *mut *mut *mut int (set_vbus)(struct tcpci tcpci, struct tcpci_data data, bool source, bool,
    pub data): *mut *mut *mut void (frs_sourcing_vbus)(struct tcpci tcpci, struct tcpci_data,
    pub capable): bool,
    pub data): *mut *mut *mut void (check_contaminant)(struct tcpci tcpci, struct tcpci_data,
    pub data): *mut *mut *mut bool (attempt_vconn_swap_discovery)(struct tcpci tcpci, struct tcpci_data,
}

extern "C" {
    pub fn tcpci_unregister_port(tcpci: *mut tcpci);
}
extern "C" {
    pub fn tcpci_irq(tcpci: *mut tcpci) -> irqreturn_t;
}
