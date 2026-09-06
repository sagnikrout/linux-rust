//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_flash.h
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
// Copyright (C) 2020-2021 Intel Corporation.
//
// Buffer size used to read the fls image
pub const IOSM_FLS_BUF_SIZE: c_uint = 0x00100000;
// Full erase start address
pub const IOSM_ERASE_START_ADDR: c_uint = 0x00000000;
// Erase length for NAND flash
pub const IOSM_ERASE_LEN: c_uint = 0xFFFFFFFF;
// EBL response Header size
pub const IOSM_EBL_HEAD_SIZE: c_int = 8;
// EBL payload size
pub const IOSM_EBL_W_PAYL_SIZE: c_int = 2048;
// Total EBL pack size

// EBL payload size
pub const IOSM_EBL_DW_PAYL_SIZE: c_int = 16384;
// Total EBL pack size

// EBL name size
pub const IOSM_EBL_NAME: c_int = 32;
// Maximum supported error types
pub const IOSM_MAX_ERRORS: c_int = 8;
// Read size for RPSI/EBL response
pub const IOSM_READ_SIZE: c_int = 2;
// Link establishment response ack size
pub const IOSM_LER_ACK_SIZE: c_int = 2;
// PSI ACK len
pub const IOSM_PSI_ACK: c_int = 8;
// SWID capability for packed swid type
pub const IOSM_EXT_CAP_SWID_OOS_PACK: c_uint = 0x02;
// EBL error response buffer
pub const IOSM_EBL_RSP_BUFF: c_uint = 0x0041;
// SWID string length
pub const IOSM_SWID_STR: c_int = 64;
// Load EBL command size
pub const IOSM_RPSI_LOAD_SIZE: c_int = 0;
// EBL payload checksum
pub const IOSM_EBL_CKSM: c_uint = 0x0000FFFF;
// SWID msg len and argument
pub const IOSM_MSG_LEN_ARG: c_int = 0;
// Data to be sent to modem
pub const IOSM_MDM_SEND_DATA: c_uint = 0x0000;
// Data received from modem as part of erase check
pub const IOSM_MDM_ERASE_RSP: c_uint = 0x0001;
// Bit shift to calculate Checksum
pub const IOSM_EBL_PAYL_SHIFT: c_int = 16;
// Flag To be set
pub const IOSM_SET_FLAG: c_int = 1;
// Set flash erase check timeout to 100 msec
pub const IOSM_FLASH_ERASE_CHECK_TIMEOUT: c_int = 100;
// Set flash erase check interval to 20 msec
pub const IOSM_FLASH_ERASE_CHECK_INTERVAL: c_int = 20;
// Link establishment response ack size
pub const IOSM_LER_RSP_SIZE: c_int = 60;
//
// enum iosm_flash_package_type -	Enum for the flashing operations
// @FLASH_SET_PROT_CONF:	Write EBL capabilities
// @FLASH_SEC_START:		Start writing the secpack
// @FLASH_SEC_END:		Validate secpack end
// @FLASH_SET_ADDRESS:		Set the address for flashing
// @FLASH_ERASE_START:		Start erase before flashing
// @FLASH_ERASE_CHECK:		Validate the erase functionality
// @FLASH_OOS_CONTROL:		Retrieve data based on oos actions
// @FLASH_OOS_DATA_READ:	Read data from EBL
// @FLASH_WRITE_IMAGE_RAW:	Write the raw image to flash
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_flash_package_type {
    FLASH_SET_PROT_CONF = 0x0086,
    FLASH_SEC_START = 0x0204,
    FLASH_SEC_END,
    FLASH_SET_ADDRESS = 0x0802,
    FLASH_ERASE_START = 0x0805,
    FLASH_ERASE_CHECK,
    FLASH_OOS_CONTROL = 0x080C,
    FLASH_OOS_DATA_READ = 0x080E,
    FLASH_WRITE_IMAGE_RAW,
}

//
// enum iosm_out_of_session_action -	Actions possible over the
// OutOfSession command interface
// @FLASH_OOSC_ACTION_READ:		Read data according to its type
// @FLASH_OOSC_ACTION_ERASE:		Erase data according to its type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_out_of_session_action {
    FLASH_OOSC_ACTION_READ = 2,
    FLASH_OOSC_ACTION_ERASE = 3,
}

//
// enum iosm_out_of_session_type -	Data types that can be handled over the
// Out Of Session command Interface
// @FLASH_OOSC_TYPE_ALL_FLASH:		The whole flash area
// @FLASH_OOSC_TYPE_SWID_TABLE:		Read the swid table from the target
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_out_of_session_type {
    FLASH_OOSC_TYPE_ALL_FLASH = 8,
    FLASH_OOSC_TYPE_SWID_TABLE = 16,
}

//
// enum iosm_ebl_caps -	EBL capability settings
// @IOSM_CAP_NOT_ENHANCED:	If capability not supported
// @IOSM_CAP_USE_EXT_CAP:	To be set if extended capability is set
// @IOSM_EXT_CAP_ERASE_ALL:	Set Erase all capability
// @IOSM_EXT_CAP_COMMIT_ALL:	Set the commit all capability
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_ebl_caps {
    IOSM_CAP_NOT_ENHANCED = 0x00,
    IOSM_CAP_USE_EXT_CAP = 0x01,
    IOSM_EXT_CAP_ERASE_ALL = 0x08,
    IOSM_EXT_CAP_COMMIT_ALL = 0x20,
}

//
// enum iosm_ebl_rsp -  EBL response field
// @EBL_CAPS_FLAG:	EBL capability flag
// @EBL_SKIP_ERASE:	EBL skip erase flag
// @EBL_SKIP_CRC:	EBL skip wr_pack crc
// @EBL_EXT_CAPS_HANDLED:	EBL extended capability handled flag
// @EBL_OOS_CONFIG:	EBL oos configuration
// @EBL_RSP_SW_INFO_VER: EBL SW info version
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_ebl_rsp {
    EBL_CAPS_FLAG = 50,
    EBL_SKIP_ERASE = 54,
    EBL_SKIP_CRC = 55,
    EBL_EXT_CAPS_HANDLED = 57,
    EBL_OOS_CONFIG = 64,
    EBL_RSP_SW_INFO_VER = 70,
}

//
// enum iosm_mdm_send_recv_data - Data to send to modem
// @IOSM_MDM_SEND_2:	Send 2 bytes of payload
// @IOSM_MDM_SEND_4:	Send 4 bytes of payload
// @IOSM_MDM_SEND_8:	Send 8 bytes of payload
// @IOSM_MDM_SEND_16:	Send 16 bytes of payload
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_mdm_send_recv_data {
    IOSM_MDM_SEND_2 = 2,
    IOSM_MDM_SEND_4 = 4,
    IOSM_MDM_SEND_8 = 8,
    IOSM_MDM_SEND_16 = 16,
}

//
// struct iosm_ebl_one_error -	Structure containing error details
// @error_class:		Error type- standard, security and text error
// @error_code:			Specific error from error type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_ebl_one_error {
    pub error_class: u16,
    pub error_code: u16,
}

//
// struct iosm_ebl_error- Structure with max error type supported
// @error:		Array of one_error structure with max errors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_ebl_error {
    pub error: [iosm_ebl_one_error; IOSM_MAX_ERRORS],
}

//
// struct iosm_swid_table - SWID table data for modem
// @number_of_data_sets:	Number of swid types
// @sw_id_type:			SWID type - SWID
// @sw_id_val:			SWID value
// @rf_engine_id_type:		RF engine ID type - RF_ENGINE_ID
// @rf_engine_id_val:		RF engine ID value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_swid_table {
    pub number_of_data_sets: u32,
    pub sw_id_type: [c_char; IOSM_EBL_NAME],
    pub sw_id_val: u32,
    pub rf_engine_id_type: [c_char; IOSM_EBL_NAME],
    pub rf_engine_id_val: u32,
}

//
// struct iosm_flash_msg_control - Data sent to modem
// @action:	Action to be performed
// @type:	Type of action
// @length:	Length of the action
// @arguments:	Argument value sent to modem
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_flash_msg_control {
    pub action: __le32,
    pub type: __le32,
    pub length: __le32,
    pub arguments: __le32,
}

//
// struct iosm_flash_data -  Header Data to be sent to modem
// @checksum:	Checksum value calculated for the payload data
// @pack_id:	Flash Action type
// @msg_length:	Payload length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_flash_data {
    pub checksum: __le16,
    pub pack_id: __le16,
    pub msg_length: __le32,
}

extern "C" {
    pub fn ipc_flash_link_establish(ipc_imem: *mut iosm_imem) -> c_int;
}
extern "C" {
    pub fn ipc_flash_read_swid(ipc_devlink: *mut iosm_devlink, mdm_rsp: *mut u8) -> c_int;
}
