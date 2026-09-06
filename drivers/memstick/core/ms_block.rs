//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memstick/core/ms_block.h
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
// ms_block.h - Sony MemoryStick (legacy) storage support
// Copyright (C) 2013 Maxim Levitsky <maximlevitsky@gmail.com>
//
// Minor portions of the driver are copied from mspro_block.c which is
// Copyright (C) 2007 Alex Dubov <oakad@yahoo.com>
//
// Also ms structures were copied from old broken driver by same author
// These probably come from MS spec
//
pub const MS_BLOCK_MAX_SEGS: c_int = 32;

pub const MS_BLOCK_MAX_BOOT_ADDR: c_uint = 0x000c;
pub const MS_BLOCK_BOOT_ID: c_uint = 0x0001;
pub const MS_BLOCK_INVALID: c_uint = 0xffff;
pub const MS_MAX_ZONES: c_int = 16;
pub const MS_BLOCKS_IN_ZONE: c_int = 512;
pub const MS_BLOCK_MAP_LINE_SZ: c_int = 16;
pub const MS_BLOCK_PART_SHIFT: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_boot_header {
    pub block_id: c_ushort,
    pub format_reserved: c_ushort,
    pub reserved0: [c_uchar; 184],
    pub data_entry: c_uchar,
    pub reserved1: [c_uchar; 179],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_system_item {
    pub start_addr: c_uint,
    pub data_size: c_uint,
    pub data_type_id: c_uchar,
    pub reserved: [c_uchar; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_system_entry {
    pub disabled_block: ms_system_item,
    pub cis_idi: ms_system_item,
    pub reserved: [c_uchar; 24],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_boot_attr_info {
    pub memorystick_class: c_uchar,
    pub format_unique_value1: c_uchar,
    pub block_size: c_ushort,
    pub number_of_blocks: c_ushort,
    pub number_of_effective_blocks: c_ushort,
    pub page_size: c_ushort,
    pub extra_data_size: c_uchar,
    pub format_unique_value2: c_uchar,
    pub assembly_time: [c_uchar; 8],
    pub format_unique_value3: c_uchar,
    pub serial_number: [c_uchar; 3],
    pub assembly_manufacturer_code: c_uchar,
    pub assembly_model_code: [c_uchar; 3],
    pub memory_manufacturer_code: c_ushort,
    pub memory_device_code: c_ushort,
    pub implemented_capacity: c_ushort,
    pub format_unique_value4: [c_uchar; 2],
    pub vcc: c_uchar,
    pub vpp: c_uchar,
    pub controller_number: c_ushort,
    pub controller_function: c_ushort,
    pub reserved0: [c_uchar; 9],
    pub transfer_supporting: c_uchar,
    pub format_unique_value5: c_ushort,
    pub format_type: c_uchar,
    pub memorystick_application: c_uchar,
    pub device_type: c_uchar,
    pub reserved1: [c_uchar; 22],
    pub format_uniqure_value6: [c_uchar; 2],
    pub reserved2: [c_uchar; 15],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_cis_idi {
    pub general_config: c_ushort,
    pub logical_cylinders: c_ushort,
    pub reserved0: c_ushort,
    pub logical_heads: c_ushort,
    pub track_size: c_ushort,
    pub page_size: c_ushort,
    pub pages_per_track: c_ushort,
    pub msw: c_ushort,
    pub lsw: c_ushort,
    pub reserved1: c_ushort,
    pub serial_number: [c_uchar; 20],
    pub buffer_type: c_ushort,
    pub buffer_size_increments: c_ushort,
    pub long_command_ecc: c_ushort,
    pub firmware_version: [c_uchar; 28],
    pub model_name: [c_uchar; 18],
    pub reserved2: [c_ushort; 5],
    pub pio_mode_number: c_ushort,
    pub dma_mode_number: c_ushort,
    pub field_validity: c_ushort,
    pub current_logical_cylinders: c_ushort,
    pub current_logical_heads: c_ushort,
    pub current_pages_per_track: c_ushort,
    pub current_page_capacity: c_uint,
    pub mutiple_page_setting: c_ushort,
    pub addressable_pages: c_uint,
    pub single_word_dma: c_ushort,
    pub multi_word_dma: c_ushort,
    pub reserved3: [c_uchar; 128],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_boot_page {
    pub header: ms_boot_header,
    pub entry: ms_system_entry,
    pub attr: ms_boot_attr_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msb_data {
    pub card: *mut memstick_dev,
    pub disk: *mut gendisk,
    pub queue: *mut request_queue,
    pub q_lock: spinlock_t,
    pub tag_set: blk_mq_tag_set,
    pub geometry: hd_geometry,
    pub attr_group: attribute_group,
    pub req: *mut request,
    pub caps: c_int,
    pub disk_id: c_int,
// IO
    pub io_queue: *mut workqueue_struct,
    pub io_queue_stopped: bool,
    pub io_work: work_struct,
    pub card_dead: bool,
// Media properties
    pub boot_page: *mut ms_boot_page,
    pub boot_block_locations: [u16; 2],
    pub boot_block_count: c_int,
    pub read_only: bool,
    pub page_size: c_ushort,
    pub block_size: c_int,
    pub pages_in_block: c_int,
    pub zone_count: c_int,
    pub block_count: c_int,
    pub logical_block_count: c_int,
// FTL tables
    pub used_blocks_bitmap: *mut c_ulong,
    pub erased_blocks_bitmap: *mut c_ulong,
    pub lba_to_pba_table: *mut u16,
    pub free_block_count: [c_int; MS_MAX_ZONES],
    pub ftl_initialized: bool,
// Cache
    pub cache: *mut c_uchar,
    pub valid_cache_bitmap: c_ulong,
    pub cache_block_lba: c_int,
    pub need_flush_cache: bool,
    pub cache_flush_timer: timer_list,
// Preallocated buffers
    pub block_buffer: *mut c_uchar,
    pub prealloc_sg: [scatterlist; MS_BLOCK_MAX_SEGS+1],
// handler's local data
    pub reg_addr: ms_register_addr,
    pub addr_valid: bool,
    pub command_value: u8,
    pub command_need_oob: bool,
    pub current_sg: *mut scatterlist,
    pub current_sg_offset: c_int,
    pub regs: ms_register,
    pub current_page: c_int,
    pub state: c_int,
    pub exit_error: c_int,
    pub int_polling: bool,
    pub int_timeout: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msb_readpage_states {
    MSB_RP_SEND_BLOCK_ADDRESS = 0,
    MSB_RP_SEND_READ_COMMAND,

    MSB_RP_SEND_INT_REQ,
    MSB_RP_RECEIVE_INT_REQ_RESULT,

    MSB_RP_SEND_READ_STATUS_REG,
    MSB_RP_RECEIVE_STATUS_REG,

    MSB_RP_SEND_OOB_READ,
    MSB_RP_RECEIVE_OOB_READ,

    MSB_RP_SEND_READ_DATA,
    MSB_RP_RECEIVE_READ_DATA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msb_write_block_states {
    MSB_WB_SEND_WRITE_PARAMS = 0,
    MSB_WB_SEND_WRITE_OOB,
    MSB_WB_SEND_WRITE_COMMAND,

    MSB_WB_SEND_INT_REQ,
    MSB_WB_RECEIVE_INT_REQ,

    MSB_WB_SEND_WRITE_DATA,
    MSB_WB_RECEIVE_WRITE_CONFIRMATION,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msb_send_command_states {
    MSB_SC_SEND_WRITE_PARAMS,
    MSB_SC_SEND_WRITE_OOB,
    MSB_SC_SEND_COMMAND,

    MSB_SC_SEND_INT_REQ,
    MSB_SC_RECEIVE_INT_REQ,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msb_reset_states {
    MSB_RS_SEND,
    MSB_RS_CONFIRM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msb_par_switch_states {
    MSB_PS_SEND_SWITCH_COMMAND,
    MSB_PS_SWICH_HOST,
    MSB_PS_CONFIRM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chs_entry {
    pub size: c_ulong,
    pub sec: c_uchar,
    pub cyl: c_ushort,
    pub head: c_uchar,
}

extern "C" {
    pub fn msb_reset(msb: *mut msb_data, full: bool) -> static int;
}

