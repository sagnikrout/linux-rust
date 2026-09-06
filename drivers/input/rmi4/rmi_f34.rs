//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/rmi4/rmi_f34.h
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
// Copyright (c) 2007-2016, Synaptics Incorporated
// Copyright (C) 2016 Zodiac Inflight Innovations
//
// F34 image file offsets.
pub const F34_FW_IMAGE_OFFSET: c_uint = 0x100;
// F34 register offsets.
pub const F34_BLOCK_DATA_OFFSET: c_int = 2;
// F34 commands
pub const F34_WRITE_FW_BLOCK: c_uint = 0x2;
pub const F34_ERASE_ALL: c_uint = 0x3;
pub const F34_READ_CONFIG_BLOCK: c_uint = 0x5;
pub const F34_WRITE_CONFIG_BLOCK: c_uint = 0x6;
pub const F34_ERASE_CONFIG: c_uint = 0x7;
pub const F34_ENABLE_FLASH_PROG: c_uint = 0xf;
pub const F34_STATUS_IN_PROGRESS: c_uint = 0xff;
pub const F34_STATUS_IDLE: c_uint = 0x80;
pub const F34_IDLE_WAIT_MS: c_int = 500;
pub const F34_ENABLE_WAIT_MS: c_int = 300;
pub const F34_ERASE_WAIT_MS: c_int = 5000;
pub const F34_WRITE_WAIT_MS: c_int = 3000;
pub const F34_BOOTLOADER_ID_LEN: c_int = 2;
// F34 V7 defines
pub const V7_FLASH_STATUS_OFFSET: c_int = 0;
pub const V7_PARTITION_ID_OFFSET: c_int = 1;
pub const V7_BLOCK_NUMBER_OFFSET: c_int = 2;
pub const V7_TRANSFER_LENGTH_OFFSET: c_int = 3;
pub const V7_COMMAND_OFFSET: c_int = 4;
pub const V7_PAYLOAD_OFFSET: c_int = 5;
pub const V7_BOOTLOADER_ID_OFFSET: c_int = 1;
pub const IMAGE_HEADER_VERSION_10: c_uint = 0x10;
pub const CONFIG_ID_SIZE: c_int = 32;
pub const PRODUCT_ID_SIZE: c_int = 10;

// F34 V7 commands
pub const CMD_V7_IDLE: c_int = 0;
pub const CMD_V7_ENTER_BL: c_int = 1;
pub const CMD_V7_READ: c_int = 2;
pub const CMD_V7_WRITE: c_int = 3;
pub const CMD_V7_ERASE: c_int = 4;
pub const CMD_V7_ERASE_AP: c_int = 5;
pub const CMD_V7_SENSOR_ID: c_int = 6;
pub const v7_CMD_IDLE: c_int = 0;
pub const v7_CMD_WRITE_FW: c_int = 1;
pub const v7_CMD_WRITE_CONFIG: c_int = 2;
pub const v7_CMD_WRITE_LOCKDOWN: c_int = 3;
pub const v7_CMD_WRITE_GUEST_CODE: c_int = 4;
pub const v7_CMD_READ_CONFIG: c_int = 5;
pub const v7_CMD_ERASE_ALL: c_int = 6;
pub const v7_CMD_ERASE_UI_FIRMWARE: c_int = 7;
pub const v7_CMD_ERASE_UI_CONFIG: c_int = 8;
pub const v7_CMD_ERASE_BL_CONFIG: c_int = 9;
pub const v7_CMD_ERASE_DISP_CONFIG: c_int = 10;
pub const v7_CMD_ERASE_FLASH_CONFIG: c_int = 11;
pub const v7_CMD_ERASE_GUEST_CODE: c_int = 12;
pub const v7_CMD_ENABLE_FLASH_PROG: c_int = 13;
pub const v7_UI_CONFIG_AREA: c_int = 0;
pub const v7_PM_CONFIG_AREA: c_int = 1;
pub const v7_BL_CONFIG_AREA: c_int = 2;
pub const v7_DP_CONFIG_AREA: c_int = 3;
pub const v7_FLASH_CONFIG_AREA: c_int = 4;
// F34 V7 partition IDs
pub const BOOTLOADER_PARTITION: c_int = 1;
pub const DEVICE_CONFIG_PARTITION: c_int = 2;
pub const FLASH_CONFIG_PARTITION: c_int = 3;
pub const MANUFACTURING_BLOCK_PARTITION: c_int = 4;
pub const GUEST_SERIALIZATION_PARTITION: c_int = 5;
pub const GLOBAL_PARAMETERS_PARTITION: c_int = 6;
pub const CORE_CODE_PARTITION: c_int = 7;
pub const CORE_CONFIG_PARTITION: c_int = 8;
pub const GUEST_CODE_PARTITION: c_int = 9;
pub const DISPLAY_CONFIG_PARTITION: c_int = 10;
// F34 V7 container IDs
pub const TOP_LEVEL_CONTAINER: c_int = 0;
pub const UI_CONTAINER: c_int = 1;
pub const UI_CONFIG_CONTAINER: c_int = 2;
pub const BL_CONTAINER: c_int = 3;
pub const BL_IMAGE_CONTAINER: c_int = 4;
pub const BL_CONFIG_CONTAINER: c_int = 5;
pub const BL_LOCKDOWN_INFO_CONTAINER: c_int = 6;
pub const PERMANENT_CONFIG_CONTAINER: c_int = 7;
pub const GUEST_CODE_CONTAINER: c_int = 8;
pub const BL_PROTOCOL_DESCRIPTOR_CONTAINER: c_int = 9;
pub const UI_PROTOCOL_DESCRIPTOR_CONTAINER: c_int = 10;
pub const RMI_SELF_DISCOVERY_CONTAINER: c_int = 11;
pub const RMI_PAGE_CONTENT_CONTAINER: c_int = 12;
pub const GENERAL_INFORMATION_CONTAINER: c_int = 13;
pub const DEVICE_CONFIG_CONTAINER: c_int = 14;
pub const FLASH_CONFIG_CONTAINER: c_int = 15;
pub const GUEST_SERIALIZATION_CONTAINER: c_int = 16;
pub const GLOBAL_PARAMETERS_CONTAINER: c_int = 17;
pub const CORE_CODE_CONTAINER: c_int = 18;
pub const CORE_CONFIG_CONTAINER: c_int = 19;
pub const DISPLAY_CONFIG_CONTAINER: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f34v7_query_1_7 {
    pub /: *mut *mut u8 bl_minor_revision; / query 1,
    pub bl_major_revision: u8,
    pub /: *mut *mut __le32 bl_fw_id; / query 2,
    pub /: *mut *mut u8 minimum_write_size; / query 3,
    pub block_size: __le16,
    pub flash_page_size: __le16,
    pub /: *mut *mut __le16 adjustable_partition_area_size; / query 4,
    pub /: *mut *mut __le16 flash_config_length; / query 5,
    pub /: *mut *mut __le16 payload_length; / query 6,
    pub /: *mut *mut u8 partition_support[4]; / query 7,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f34v7_data_1_5 {
    pub partition_id: u8,
    pub block_offset: __le16,
    pub transfer_length: __le16,
    pub command: u8,
    pub payload: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_data {
    pub data: *const c_void,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct partition_table {
    pub partition_id: u8,
    pub byte_1_reserved: u8,
    pub partition_length: __le16,
    pub start_physical_address: __le16,
    pub partition_properties: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physical_address {
    pub ui_firmware: u16,
    pub ui_config: u16,
    pub dp_config: u16,
    pub guest_code: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct container_descriptor {
    pub content_checksum: __le32,
    pub container_id: __le16,
    pub minor_version: u8,
    pub major_version: u8,
    pub reserved_08: u8,
    pub reserved_09: u8,
    pub reserved_0a: u8,
    pub reserved_0b: u8,
    pub container_option_flags: [u8; 4],
    pub content_options_length: __le32,
    pub content_options_address: __le32,
    pub content_length: __le32,
    pub content_address: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_count {
    pub ui_firmware: u16,
    pub ui_config: u16,
    pub dp_config: u16,
    pub fl_config: u16,
    pub pm_config: u16,
    pub bl_config: u16,
    pub lockdown: u16,
    pub guest_code: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_header_10 {
    pub checksum: __le32,
    pub reserved_04: u8,
    pub reserved_05: u8,
    pub minor_header_version: u8,
    pub major_header_version: u8,
    pub reserved_08: u8,
    pub reserved_09: u8,
    pub reserved_0a: u8,
    pub reserved_0b: u8,
    pub top_level_container_start_addr: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct image_metadata {
    pub contains_firmware_id: bool,
    pub contains_bootloader: bool,
    pub contains_display_cfg: bool,
    pub contains_guest_code: bool,
    pub contains_flash_config: bool,
    pub firmware_id: c_uint,
    pub checksum: c_uint,
    pub bootloader_size: c_uint,
    pub display_cfg_offset: c_uint,
    pub bl_version: c_uchar,
    pub 1]: unsigned char product_id[PRODUCT_ID_SIZE +,
    pub 1]: unsigned char cstmr_product_id[PRODUCT_ID_SIZE +,
    pub bootloader: block_data,
    pub ui_firmware: block_data,
    pub ui_config: block_data,
    pub dp_config: block_data,
    pub fl_config: block_data,
    pub bl_config: block_data,
    pub guest_code: block_data,
    pub lockdown: block_data,
    pub blkcount: block_count,
    pub phyaddr: physical_address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_f34_firmware {
    pub checksum: __le32,
    pub pad1: [u8; 3],
    pub bootloader_version: u8,
    pub image_size: __le32,
    pub config_size: __le32,
    pub product_id: [u8; 10],
    pub product_info: [u8; 2],
    pub pad2: [u8; 228],
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f34v5_data {
    pub block_size: u16,
    pub fw_blocks: u16,
    pub config_blocks: u16,
    pub ctrl_address: u16,
    pub status: u8,
    pub cmd_done: completion,
    pub flash_mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f34v7_data {
    pub has_display_cfg: bool,
    pub has_guest_code: bool,
    pub in_bl_mode: bool,
    pub read_config_buf: *mut u8,
    pub read_config_buf_size: usize,
    pub command: u8,
    pub flash_status: u8,
    pub block_size: u16,
    pub config_block_count: u16,
    pub config_size: u16,
    pub config_area: u16,
    pub flash_config_length: u16,
    pub payload_length: u16,
    pub partitions: u8,
    pub partition_table_bytes: u16,
    pub blkcount: block_count,
    pub phyaddr: physical_address,
    pub img: image_metadata,
    pub config_data: *const c_void,
    pub image: *const c_void,
    pub cmd_done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f34_data {
    pub fn: *mut rmi_function,
    pub bl_version: u8,
    pub bootloader_id: [c_uchar; 5],
    pub 1]: *mut *mut unsigned char configuration_id[CONFIG_ID_SIZE2 +,
    pub update_status: c_int,
    pub update_progress: c_int,
    pub update_size: c_int,
    pub v5: f34v5_data,
    pub v7: f34v7_data,
}

extern "C" {
    pub fn rmi_f34v7_start_reflash(f34: *mut f34_data, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn rmi_f34v7_do_reflash(f34: *mut f34_data, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn rmi_f34v7_probe(f34: *mut f34_data) -> c_int;
}
