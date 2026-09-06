//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/pd.h
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
// Copyright 2015-2017 Google, Inc
//

// USB PD Messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_ctrl_msg_type {
// 0 Reserved
    PD_CTRL_GOOD_CRC = 1,
    PD_CTRL_GOTO_MIN = 2,
    PD_CTRL_ACCEPT = 3,
    PD_CTRL_REJECT = 4,
    PD_CTRL_PING = 5,
    PD_CTRL_PS_RDY = 6,
    PD_CTRL_GET_SOURCE_CAP = 7,
    PD_CTRL_GET_SINK_CAP = 8,
    PD_CTRL_DR_SWAP = 9,
    PD_CTRL_PR_SWAP = 10,
    PD_CTRL_VCONN_SWAP = 11,
    PD_CTRL_WAIT = 12,
    PD_CTRL_SOFT_RESET = 13,
// 14-15 Reserved
    PD_CTRL_NOT_SUPP = 16,
    PD_CTRL_GET_SOURCE_CAP_EXT = 17,
    PD_CTRL_GET_STATUS = 18,
    PD_CTRL_FR_SWAP = 19,
    PD_CTRL_GET_PPS_STATUS = 20,
    PD_CTRL_GET_COUNTRY_CODES = 21,
    PD_CTRL_GET_SINK_CAP_EXT = 22,
// 23 Reserved
    PD_CTRL_GET_REVISION = 24,
// 25-31 Reserved
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_data_msg_type {
// 0 Reserved
    PD_DATA_SOURCE_CAP = 1,
    PD_DATA_REQUEST = 2,
    PD_DATA_BIST = 3,
    PD_DATA_SINK_CAP = 4,
    PD_DATA_BATT_STATUS = 5,
    PD_DATA_ALERT = 6,
    PD_DATA_GET_COUNTRY_INFO = 7,
    PD_DATA_ENTER_USB = 8,
// 9-11 Reserved
    PD_DATA_REVISION = 12,
// 13-14 Reserved
    PD_DATA_VENDOR_DEF = 15,
// 16-31 Reserved
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_ext_msg_type {
// 0 Reserved
    PD_EXT_SOURCE_CAP_EXT = 1,
    PD_EXT_STATUS = 2,
    PD_EXT_GET_BATT_CAP = 3,
    PD_EXT_GET_BATT_STATUS = 4,
    PD_EXT_BATT_CAP = 5,
    PD_EXT_GET_MANUFACTURER_INFO = 6,
    PD_EXT_MANUFACTURER_INFO = 7,
    PD_EXT_SECURITY_REQUEST = 8,
    PD_EXT_SECURITY_RESPONSE = 9,
    PD_EXT_FW_UPDATE_REQUEST = 10,
    PD_EXT_FW_UPDATE_RESPONSE = 11,
    PD_EXT_PPS_STATUS = 12,
    PD_EXT_COUNTRY_INFO = 13,
    PD_EXT_COUNTRY_CODES = 14,
    PD_EXT_SINK_CAP_EXT = 15,
// 16-31 Reserved
}

pub const PD_REV10: c_uint = 0x0;
pub const PD_REV20: c_uint = 0x1;
pub const PD_REV30: c_uint = 0x2;

pub const PD_HEADER_CNT_SHIFT: c_int = 12;
pub const PD_HEADER_CNT_MASK: c_uint = 0x7;
pub const PD_HEADER_ID_SHIFT: c_int = 9;
pub const PD_HEADER_ID_MASK: c_uint = 0x7;

pub const PD_HEADER_REV_SHIFT: c_int = 6;
pub const PD_HEADER_REV_MASK: c_uint = 0x3;

pub const PD_HEADER_TYPE_SHIFT: c_int = 0;
pub const PD_HEADER_TYPE_MASK: c_uint = 0x1f;

extern "C" {
    pub fn pd_header_cnt(_arg: le16_to_cpu(header)) -> return;
}
extern "C" {
    pub fn pd_header_type(_arg: le16_to_cpu(header)) -> return;
}
extern "C" {
    pub fn pd_header_msgid(_arg: le16_to_cpu(header)) -> return;
}
extern "C" {
    pub fn pd_header_rev(_arg: le16_to_cpu(header)) -> return;
}

pub const PD_EXT_HDR_CHUNK_NUM_SHIFT: c_int = 11;
pub const PD_EXT_HDR_CHUNK_NUM_MASK: c_uint = 0xf;

pub const PD_EXT_HDR_DATA_SIZE_SHIFT: c_int = 0;
pub const PD_EXT_HDR_DATA_SIZE_MASK: c_uint = 0x1ff;

extern "C" {
    pub fn pd_ext_header_data_size(_arg: le16_to_cpu(ext_header)) -> return;
}
pub const PD_MAX_PAYLOAD: c_int = 7;
pub const PD_EXT_MAX_CHUNK_DATA: c_int = 26;
//
// struct pd_chunked_ext_message_data - PD chunked extended message data as
// seen on wire
// @header:    PD extended message header
// @data:      PD extended message data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pd_chunked_ext_message_data {
    pub header: __le16,
    pub data: [u8; PD_EXT_MAX_CHUNK_DATA],
    pub __packed: },
//
// struct pd_message - PD message as seen on wire
// @header:    PD message header
// @payload:   PD message payload
// @ext_msg:   PD message chunked extended message data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pd_message {
    pub header: __le16,
    pub payload: [__le32; PD_MAX_PAYLOAD],
    pub ext_msg: pd_chunked_ext_message_data,
}

//
// count_chunked_data_objs - Helper to calculate number of Data Objects on a 4
// byte boundary.
// @size: Size of data block for extended message. Should *not* include extended
// header size.
//
// batt_cap_ext_msg - Battery capability extended PD message
// @vid: Battery Vendor ID (assigned by USB-IF)
// @pid: Battery Product ID (assigned by battery or device vendor)
// @batt_design_cap: Battery design capacity in 0.1Wh
// @batt_last_chg_cap: Battery last full charge capacity in 0.1Wh
// @batt_type: Battery Type. bit0 when set indicates invalid battery reference.
// Rest of the bits are reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct batt_cap_ext_msg {
    pub vid: __le16,
    pub pid: __le16,
    pub batt_design_cap: __le16,
    pub batt_last_chg_cap: __le16,
    pub batt_type: u8,
    pub __packed: },

// Sink Caps Extended Data Block Version
pub const SKEDB_VER_1_0: c_int = 1;
// Sink Caps Extended Sink Modes

//
// struct sink_caps_ext_msg - Sink extended capability PD message
// @vid: Vendor ID
// @pid: Product ID
// @xid: Value assigned by USB-IF for product
// @fw: Firmware version
// @hw: Hardware version
// @skedb_ver: Sink Caps Extended Data Block (SKEDB) Version
// @load_step: Indicates the load step slew rate.
// @load_char: Sink overload characteristics
// @compliance: Types of sources the sink has been tested & certified on
// @touch_temp: Indicates the IEC standard to which the touch temperature
// conforms to (if applicable).
// @batt_info: Indicates number batteries and hot swappable ports
// @modes: Charging caps & power sources supported
// @spr_min_pdp: Sink Minimum PDP for SPR mode
// @spr_op_pdp: Sink Operational PDP for SPR mode
// @spr_max_pdp: Sink Maximum PDP for SPR mode
// @epr_min_pdp: Sink Minimum PDP for EPR mode
// @epr_op_pdp: Sink Operational PDP for EPR mode
// @epr_max_pdp: Sink Maximum PDP for EPR mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sink_caps_ext_msg {
    pub vid: __le16,
    pub pid: __le16,
    pub xid: __le32,
    pub fw: u8,
    pub hw: u8,
    pub skedb_ver: u8,
    pub load_step: u8,
    pub load_char: __le16,
    pub compliance: u8,
    pub touch_temp: u8,
    pub batt_info: u8,
    pub modes: u8,
    pub spr_min_pdp: u8,
    pub spr_op_pdp: u8,
    pub spr_max_pdp: u8,
    pub epr_min_pdp: u8,
    pub epr_op_pdp: u8,
    pub epr_max_pdp: u8,
    pub __packed: },
// PDO: Power Data Object
pub const PDO_MAX_OBJECTS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_pdo_type {
    PDO_TYPE_FIXED = 0,
    PDO_TYPE_BATT = 1,
    PDO_TYPE_VAR = 2,
    PDO_TYPE_APDO = 3,
}

pub const PDO_TYPE_SHIFT: c_int = 30;
pub const PDO_TYPE_MASK: c_uint = 0x3;

pub const PDO_VOLT_MASK: c_uint = 0x3ff;
pub const PDO_CURR_MASK: c_uint = 0x3ff;
pub const PDO_PWR_MASK: c_uint = 0x3ff;

pub const PDO_FIXED_FRS_CURR_SHIFT: c_int = 23;
pub const PDO_FIXED_PEAK_CURR_SHIFT: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pd_apdo_type {
    APDO_TYPE_PPS = 0,
    APDO_TYPE_EPR_AVS = 1,
    APDO_TYPE_SPR_AVS = 2,
}

pub const PDO_APDO_TYPE_SHIFT: c_int = 28;
pub const PDO_APDO_TYPE_MASK: c_uint = 0x3;

pub const PDO_PPS_APDO_VOLT_MASK: c_uint = 0xff;
pub const PDO_PPS_APDO_CURR_MASK: c_uint = 0x7f;

//
// Applicable only to EPR AVS APDO source cap as per
// Table 6.15 EPR Adjustable Voltage Supply APDO – Source
//

//
// Applicable to both EPR AVS APDO source and sink cap as per
// Table 6.15 EPR Adjustable Voltage Supply APDO – Source
// Table 6.22 EPR Adjustable Voltage Supply APDO – Sink
//

//
// Applicable only SPR AVS APDO source cap as per
// Table 6.14 SPR Adjustable Voltage Supply APDO – Source
//

//
// Applicable to both SPR AVS APDO source and sink cap as per
// Table 6.14 SPR Adjustable Voltage Supply APDO – Source
// Table 6.21 SPR Adjustable Voltage Supply APDO – Sink
//

// SPR AVS has two different current ranges 9V - 15V, 15V - 20V
pub const SPR_AVS_TIER1_MIN_VOLT_MV: c_int = 9000;
pub const SPR_AVS_TIER1_MAX_VOLT_MV: c_int = 15000;
pub const SPR_AVS_TIER2_MAX_VOLT_MV: c_int = 20000;
pub const SPR_AVS_AVS_SMALL_STEP_V: c_int = 1;
// vAvsStep - 100mv
pub const SPR_AVS_VOLT_MV_STEP: c_int = 100;
// SPR AVS RDO Operating Current is in 50mA step
pub const RDO_SPR_AVS_CURR_MA_STEP: c_int = 50;
// SPR AVS RDO Output voltage is in 25mV step
pub const RDO_SPR_AVS_OUT_VOLT_MV_STEP: c_int = 25;

    pub PDO_TYPE_MASK: return (pdo >> PDO_TYPE_SHIFT) &,
    pub 50: *mut *mut return ((pdo >> PDO_FIXED_VOLT_SHIFT) & PDO_VOLT_MASK),
    pub 10: *mut *mut return ((pdo >> PDO_FIXED_CURR_SHIFT) & PDO_CURR_MASK),
    pub 50: *mut *mut return ((pdo >> PDO_VAR_MIN_VOLT_SHIFT) & PDO_VOLT_MASK),
    pub 50: *mut *mut return ((pdo >> PDO_VAR_MAX_VOLT_SHIFT) & PDO_VOLT_MASK),
    pub 10: *mut *mut return ((pdo >> PDO_VAR_MAX_CURR_SHIFT) & PDO_CURR_MASK),
    pub 250: *mut *mut return ((pdo >> PDO_BATT_MAX_PWR_SHIFT) & PDO_PWR_MASK),
    pub PDO_APDO_TYPE_MASK: return (pdo >> PDO_APDO_TYPE_SHIFT) &,
    pub 100: *mut *mut PDO_PPS_APDO_VOLT_MASK),
    pub 100: *mut *mut PDO_PPS_APDO_VOLT_MASK),
    pub 50: *mut *mut PDO_PPS_APDO_CURR_MASK),
    pub pdo): return FIELD_GET(PDO_EPR_AVS_APDO_PEAK_CURRENT,,
    pub 100: *mut *mut return FIELD_GET(PDO_EPR_AVS_APDO_MIN_VOLT, pdo),
    pub 100: *mut *mut return FIELD_GET(PDO_EPR_AVS_APDO_MAX_VOLT, pdo),
    pub pdo): return FIELD_GET(PDO_EPR_AVS_APDO_PDP,,
    pub pdo): return FIELD_GET(PDO_SPR_AVS_APDO_PEAK_CURRENT,,
    pub 10: *mut *mut return FIELD_GET(PDO_SPR_AVS_APDO_9V_TO_15V_MAX_CURR, pdo),
    pub 10: *mut *mut return FIELD_GET(PDO_SPR_AVS_APDO_15V_TO_20V_MAX_CURR, pdo),
// RDO: Request Data Object
pub const RDO_OBJ_POS_SHIFT: c_int = 28;
pub const RDO_OBJ_POS_MASK: c_uint = 0x7;

pub const RDO_PWR_MASK: c_uint = 0x3ff;
pub const RDO_CURR_MASK: c_uint = 0x3ff;
pub const RDO_FIXED_OP_CURR_SHIFT: c_int = 10;
pub const RDO_FIXED_MAX_CURR_SHIFT: c_int = 0;

pub const RDO_PROG_VOLT_MASK: c_uint = 0x7ff;
pub const RDO_PROG_CURR_MASK: c_uint = 0x7f;
pub const RDO_PROG_VOLT_SHIFT: c_int = 9;
pub const RDO_PROG_CURR_SHIFT: c_int = 0;
pub const RDO_PROG_VOLT_MV_STEP: c_int = 20;
pub const RDO_PROG_CURR_MA_STEP: c_int = 50;

    pub RDO_OBJ_POS_MASK: return (rdo >> RDO_OBJ_POS_SHIFT) &,
    pub 10: *mut *mut return ((rdo >> RDO_FIXED_OP_CURR_SHIFT) & RDO_CURR_MASK),
    pub 10: *mut *mut RDO_CURR_MASK),
    pub 250: *mut *mut return ((rdo >> RDO_BATT_OP_PWR_SHIFT) & RDO_PWR_MASK),
    pub 250: *mut *mut return ((rdo >> RDO_BATT_MAX_PWR_SHIFT) & RDO_PWR_MASK),
// Enter_USB Data Object

pub const EUDO_USB_MODE_SHIFT: c_int = 28;
pub const EUDO_USB_MODE_USB2: c_int = 0;
pub const EUDO_USB_MODE_USB3: c_int = 1;
pub const EUDO_USB_MODE_USB4: c_int = 2;

pub const EUDO_CABLE_SPEED_SHIFT: c_int = 21;
pub const EUDO_CABLE_SPEED_USB2: c_int = 0;
pub const EUDO_CABLE_SPEED_USB3_GEN1: c_int = 1;
pub const EUDO_CABLE_SPEED_USB4_GEN2: c_int = 2;
pub const EUDO_CABLE_SPEED_USB4_GEN3: c_int = 3;

pub const EUDO_CABLE_TYPE_SHIFT: c_int = 19;
pub const EUDO_CABLE_TYPE_PASSIVE: c_int = 0;
pub const EUDO_CABLE_TYPE_RE_TIMER: c_int = 1;
pub const EUDO_CABLE_TYPE_RE_DRIVER: c_int = 2;
pub const EUDO_CABLE_TYPE_OPTICAL: c_int = 3;

pub const EUDO_CABLE_CURRENT_SHIFT: c_int = 17;
pub const EUDO_CABLE_CURRENT_NOTSUPP: c_int = 0;
pub const EUDO_CABLE_CURRENT_3A: c_int = 2;
pub const EUDO_CABLE_CURRENT_5A: c_int = 3;

//
// Request Message Data Object (PD Revision 3.1+ only)
// --------
// <31:28> :: Revision Major
// <27:24> :: Revision Minor
// <23:20> :: Version Major
// <19:16> :: Version Minor
// <15:0>  :: Reserved, Shall be set to zero
//

// USB PD timers and counters

pub const PD_T_SOURCE_ACTIVITY: c_int = 45;
pub const PD_T_SINK_ACTIVITY: c_int = 135;

pub const PD_T_PS_TRANSITION: c_int = 500;
pub const PD_T_SRC_TRANSITION: c_int = 35;
pub const PD_T_DRP_SNK: c_int = 40;
pub const PD_T_DRP_SRC: c_int = 30;
pub const PD_T_PS_SOURCE_OFF: c_int = 920;
pub const PD_T_PS_SOURCE_ON: c_int = 480;

pub const PD_T_PS_HARD_RESET: c_int = 30;
pub const PD_T_SRC_RECOVER: c_int = 760;
pub const PD_T_SRC_RECOVER_MAX: c_int = 1000;
pub const PD_T_SRC_TURN_ON: c_int = 275;
pub const PD_T_SAFE_0V: c_int = 650;
pub const PD_T_VCONN_SOURCE_ON: c_int = 100;

pub const PD_T_VCONN_STABLE: c_int = 50;

pub const PD_N_HARD_RESET_COUNT: c_int = 2;

    pub usb_power_delivery: struct,
//
// usb_power_delivery_desc - USB Power Delivery Descriptor
// @revision: USB Power Delivery Specification Revision
// @version: USB Power Delivery Specicication Version - optional
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_power_delivery_desc {
    pub revision: u16,
    pub version: u16,
}

//
// usb_power_delivery_capabilities_desc - Description of USB Power Delivery Capabilities Message
// @pdo: The Power Data Objects in the Capability Message
// @role: Power role of the capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_power_delivery_capabilities_desc {
    pub pdo: [u32; PDO_MAX_OBJECTS],
    pub role: typec_role,
}

extern "C" {
    pub fn usb_power_delivery_unregister_capabilities(cap: *mut usb_power_delivery_capabilities);
}
extern "C" {
    pub fn usb_power_delivery_unregister(pd: *mut usb_power_delivery);
}
extern "C" {
    pub fn usb_power_delivery_link_device(pd: *mut usb_power_delivery, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn usb_power_delivery_unlink_device(pd: *mut usb_power_delivery, dev: *mut device);
}

// Battery Status Data Object

//
// Battery Charge Status: Battery Charging Status Values as defined in
// "USB PD Spec Rev3.1 Ver1.8", "Table 6-46 Battery Status Data Object (BSDO)".
//
pub const BSDO_BATTERY_INFO_CHARGING: c_uint = 0x0;
pub const BSDO_BATTERY_INFO_DISCHARGING: c_uint = 0x1;
pub const BSDO_BATTERY_INFO_IDLE: c_uint = 0x2;
pub const BSDO_BATTERY_INFO_RSVD: c_uint = 0x3;
//
// BSDO() - Pack data into Battery Status Data Object format.
// @batt_charge: Battery's present state of charge in 0.1WH increment.
// @chg_status: Battery charge status.
// @batt_present: Indicates that battery is present/attached when set else absent when unset.
// @invalid_ref: Indicates that an invalid battery reference was made in the Get_Battery_Status
// request.
//

