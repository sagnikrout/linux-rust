//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_devlink.h
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

// Image ext max len
pub const IOSM_DEVLINK_MAX_IMG_LEN: c_int = 3;
// Magic Header

// Magic Header len
pub const IOSM_DEVLINK_MAGIC_HEADER_LEN: c_int = 20;
// Devlink image type
pub const IOSM_DEVLINK_IMG_TYPE: c_int = 4;
// Reserve header size
pub const IOSM_DEVLINK_RESERVED: c_int = 34;
// Devlink Image Header size

// MAX file name length
pub const IOSM_MAX_FILENAME_LEN: c_int = 32;
// EBL response size
pub const IOSM_EBL_RSP_SIZE: c_int = 76;
// MAX number of regions supported
pub const IOSM_NOF_CD_REGION: c_int = 6;
// MAX number of SNAPSHOTS supported
pub const MAX_SNAPSHOTS: c_int = 1;
// Default Coredump file size
pub const REPORT_JSON_SIZE: c_uint = 0x800;
pub const COREDUMP_FCD_SIZE: c_uint = 0x10E00000;
pub const CDD_LOG_SIZE: c_uint = 0x30000;
pub const EEPROM_BIN_SIZE: c_uint = 0x10000;
pub const BOOTCORE_TRC_BIN_SIZE: c_uint = 0x8000;
pub const BOOTCORE_PREV_TRC_BIN_SIZE: c_uint = 0x20000;
//
// enum iosm_devlink_param_id - Enum type to different devlink params
// @IOSM_DEVLINK_PARAM_ID_BASE:			Devlink param base ID
// @IOSM_DEVLINK_PARAM_ID_ERASE_FULL_FLASH:     Set if full erase required
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_devlink_param_id {
    IOSM_DEVLINK_PARAM_ID_BASE = DEVLINK_PARAM_GENERIC_ID_MAX,
    IOSM_DEVLINK_PARAM_ID_ERASE_FULL_FLASH,
}

//
// enum iosm_rpsi_cmd_code - Enum type for RPSI command list
// @rpsi_cmd_code_ebl:		Command to load ebl
// @rpsi_cmd_coredump_start:    Command to get list of files and
// file size info from PSI
// @rpsi_cmd_coredump_get:      Command to get the coredump data
// @rpsi_cmd_coredump_end:      Command to stop receiving the coredump
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_rpsi_cmd_code {
    rpsi_cmd_code_ebl = 0x02,
    rpsi_cmd_coredump_start = 0x10,
    rpsi_cmd_coredump_get   = 0x11,
    rpsi_cmd_coredump_end   = 0x12,
}

//
// enum iosm_flash_comp_type - Enum for different flash component types
// @FLASH_COMP_TYPE_PSI:	PSI flash comp type
// @FLASH_COMP_TYPE_EBL:	EBL flash comp type
// @FLASH_COMP_TYPE_FLS:	FLS flash comp type
// @FLASH_COMP_TYPE_INVAL:	Invalid flash comp type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iosm_flash_comp_type {
    FLASH_COMP_TYPE_PSI,
    FLASH_COMP_TYPE_EBL,
    FLASH_COMP_TYPE_FLS,
    FLASH_COMP_TYPE_INVAL,
}

//
// struct iosm_devlink_sio - SIO instance
// @rx_list:	Downlink skbuf list received from CP
// @read_sem:	Needed for the blocking read or downlink transfer
// @channel_id: Reserved channel id for flashing/CD collection to RAM
// @channel:	Channel instance for flashing and coredump
// @devlink_read_pend: Check if read is pending
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_devlink_sio {
    pub rx_list: sk_buff_head,
    pub read_sem: completion,
    pub channel_id: c_int,
    pub channel: *mut ipc_mem_channel,
    pub devlink_read_pend: u32,
}

//
// struct iosm_flash_params - List of flash params required for flashing
// @erase_full_flash:   To set the flashing mode
// erase_full_flash = 1; full erase
// erase_full_flash = 0; no erase
// @erase_full_flash_done: Flag to check if it is a full erase
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_flash_params {
    pub erase_full_flash: u8,
    pub erase_full_flash_done: u8,
}

//
// struct iosm_devlink_image - Structure with Fls file header info
// @magic_header:	Header of the firmware image
// @image_type:		Firmware image type
// @region_address:	Address of the region to be flashed
// @download_region:	Field to identify if it is a region
// @last_region:	Field to identify if it is last region
// @reserved:		Reserved field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_devlink_image {
    pub magic_header: [c_char; IOSM_DEVLINK_MAGIC_HEADER_LEN],
    pub image_type: [c_char; IOSM_DEVLINK_IMG_TYPE],
    pub region_address: __le32,
    pub download_region: u8,
    pub last_region: u8,
    pub reserved: [u8; IOSM_DEVLINK_RESERVED],
    pub __packed: },
//
// struct iosm_ebl_ctx_data -  EBL ctx data used during flashing
// @ebl_sw_info_version: SWID version info obtained from EBL
// @m_ebl_resp:         Buffer used to read and write the ebl data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_ebl_ctx_data {
    pub ebl_sw_info_version: u8,
    pub m_ebl_resp: [u8; IOSM_EBL_RSP_SIZE],
}

//
// struct iosm_coredump_file_info -  Coredump file info
// @filename:		Name of coredump file
// @default_size:	Default size of coredump file
// @actual_size:	Actual size of coredump file
// @entry:		Index of the coredump file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_coredump_file_info {
    pub filename: [c_char; IOSM_MAX_FILENAME_LEN],
    pub default_size: u32,
    pub actual_size: u32,
    pub entry: u32,
}

//
// struct iosm_devlink - IOSM Devlink structure
// @devlink_sio:        SIO instance for read/write functionality
// @pcie:               Pointer to PCIe component
// @dev:                Pointer to device struct
// @devlink_ctx:	Pointer to devlink context
// @param:		Params required for flashing
// @ebl_ctx:		Data to be read and written to Modem
// @cd_file_info:	coredump file info
// @iosm_devlink_mdm_coredump:	region ops for coredump collection
// @cd_regions:		coredump regions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_devlink {
    pub devlink_sio: iosm_devlink_sio,
    pub pcie: *mut iosm_pcie,
    pub dev: *mut device,
    pub devlink_ctx: *mut devlink,
    pub param: iosm_flash_params,
    pub ebl_ctx: iosm_ebl_ctx_data,
    pub cd_file_info: *mut iosm_coredump_file_info,
    pub iosm_devlink_mdm_coredump: [devlink_region_ops; IOSM_NOF_CD_REGION],
    pub cd_regions: [*mut devlink_region; IOSM_NOF_CD_REGION],
}

//
// union iosm_rpsi_param_u - RPSI cmd param for CRC calculation
// @word:	Words member used in CRC calculation
// @dword:	Actual data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iosm_rpsi_param_u {
    pub word: [__le16; 2],
    pub dword: __le32,
}

//
// struct iosm_rpsi_cmd - Structure for RPSI Command
// @param:      Used to calculate CRC
// @cmd:        Stores the RPSI command
// @crc:        Stores the CRC value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_rpsi_cmd {
    pub param: iosm_rpsi_param_u,
    pub cmd: __le16,
    pub crc: __le16,
}

extern "C" {
    pub fn ipc_devlink_deinit(ipc_devlink: *mut iosm_devlink);
}
extern "C" {
    pub fn ipc_devlink_send_cmd(ipc_devlink: *mut iosm_devlink, cmd: u16, entry: u32) -> c_int;
}
