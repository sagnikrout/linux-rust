//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/cmdhdr.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2025 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_cmdhdr_h__
//
// DOC: Host command section
//
// A host command is a command issued by the upper layer to the fw. There are
// several versions of fw that have several APIs. The transport layer is
// completely agnostic to these differences.
// The transport does provide helper functionality (i.e. SYNC / ASYNC mode),
//

//
// those functions retrieve specific information from
// the id field in the iwl_host_cmd struct which contains
// the command id, the group id and the version of the command
// and vice versa
//
// make u16 wide id out of u8 group and opcode

// due to the conversion, this group is special; new groups
// should be defined in the appropriate fw-api header files
//
pub const IWL_ALWAYS_LONG_GROUP: c_int = 1;
//
// struct iwl_cmd_header - (short) command header format
//
// This header format appears in the beginning of each command sent from the
// driver, and each response/notification received from uCode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_cmd_header {
//
// @cmd: Command ID: REPLY_RXON, etc.
//
    pub cmd: u8,
//
// @group_id: group ID, for commands with groups
//
    pub group_id: u8,
//
// @sequence:
// Sequence number for the command.
//
// The driver sets up the sequence number to values of its choosing.
// uCode does not use this value, but passes it back to the driver
// when sending the response to each driver-originated command, so
// the driver can match the response to the command.  Since the values
// don't get used by uCode, the driver may set up an arbitrary format.
//
// There is one exception:  uCode sets bit 15 when it originates
// the response/notification, i.e. when the response/notification
// is not a direct response to a command sent by the driver.  For
// example, uCode issues REPLY_RX when it sends a received frame
// to the driver; it is not a direct response to any driver command.
//
// The Linux driver uses the following format:
//
// 0:7		tfd index - position within TX queue
// 8:12	TX queue id
// 13:14	reserved
// 15		unsolicited RX or uCode-originated notification
//
    pub sequence: __le16,
    pub __packed: },
//
// struct iwl_cmd_header_wide - wide command header
//
// This header format appears in the beginning of each command sent from the
// driver, and each response/notification received from uCode.
// this is the wide version that contains more information about the command
// like length, version and command type
//
// @cmd: command ID, like in &struct iwl_cmd_header
// @group_id: group ID, like in &struct iwl_cmd_header
// @sequence: sequence, like in &struct iwl_cmd_header
// @length: length of the command
// @reserved: reserved
// @version: command version
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_cmd_header_wide {
    pub cmd: u8,
    pub group_id: u8,
    pub sequence: __le16,
    pub length: __le16,
    pub reserved: u8,
    pub version: u8,
    pub __packed: },
//
// struct iwl_calib_res_notif_phy_db - Receive phy db chunk after calibrations
// @type: type of the result - mostly ignored
// @length: length of the data
// @data: data, length in @length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_res_notif_phy_db {
    pub type: __le16,
    pub length: __le16,
    pub data: [u8; ],
    pub __packed: },
//
// struct iwl_phy_db_cmd - configure operational ucode
// @type: type of the data
// @length: length of the data
// @data: data, length in @length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_phy_db_cmd {
    pub type: __le16,
    pub length: __le16,
    pub data: [u8; ],
    pub __packed: },
//
// struct iwl_cmd_response - generic response struct for most commands
// @status: status of the command asked, changes for each one
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_cmd_response {
    pub status: __le32,
}
