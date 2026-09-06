//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/memstick.h
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
// Sony MemoryStick support
//
// Copyright (C) 2007 Alex Dubov <oakad@yahoo.com>
//

// Hardware based structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_status_register {
    pub reserved: c_uchar,
    pub interrupt: c_uchar,
pub const MEMSTICK_INT_CMDNAK: c_uint = 0x01;
pub const MEMSTICK_INT_IOREQ: c_uint = 0x08;
pub const MEMSTICK_INT_IOBREQ: c_uint = 0x10;
pub const MEMSTICK_INT_BREQ: c_uint = 0x20;
pub const MEMSTICK_INT_ERR: c_uint = 0x40;
pub const MEMSTICK_INT_CED: c_uint = 0x80;
    pub status0: c_uchar,
pub const MEMSTICK_STATUS0_WP: c_uint = 0x01;
pub const MEMSTICK_STATUS0_SL: c_uint = 0x02;
pub const MEMSTICK_STATUS0_BF: c_uint = 0x10;
pub const MEMSTICK_STATUS0_BE: c_uint = 0x20;
pub const MEMSTICK_STATUS0_FB0: c_uint = 0x40;
pub const MEMSTICK_STATUS0_MB: c_uint = 0x80;
    pub status1: c_uchar,
pub const MEMSTICK_STATUS1_UCFG: c_uint = 0x01;
pub const MEMSTICK_STATUS1_FGER: c_uint = 0x02;
pub const MEMSTICK_STATUS1_UCEX: c_uint = 0x04;
pub const MEMSTICK_STATUS1_EXER: c_uint = 0x08;
pub const MEMSTICK_STATUS1_UCDT: c_uint = 0x10;
pub const MEMSTICK_STATUS1_DTER: c_uint = 0x20;
pub const MEMSTICK_STATUS1_FB1: c_uint = 0x40;
pub const MEMSTICK_STATUS1_MB: c_uint = 0x80;
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_id_register {
    pub type: c_uchar,
    pub if_mode: c_uchar,
    pub category: c_uchar,
    pub class: c_uchar,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_param_register {
    pub system: c_uchar,
pub const MEMSTICK_SYS_PAM: c_uint = 0x08;
pub const MEMSTICK_SYS_BAMD: c_uint = 0x80;
    pub block_address_msb: c_uchar,
    pub block_address: c_ushort,
    pub cp: c_uchar,
pub const MEMSTICK_CP_BLOCK: c_uint = 0x00;
pub const MEMSTICK_CP_PAGE: c_uint = 0x20;
pub const MEMSTICK_CP_EXTRA: c_uint = 0x40;
pub const MEMSTICK_CP_OVERWRITE: c_uint = 0x80;
    pub page_address: c_uchar,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_extra_data_register {
    pub overwrite_flag: c_uchar,
pub const MEMSTICK_OVERWRITE_UDST: c_uint = 0x10;
pub const MEMSTICK_OVERWRITE_PGST1: c_uint = 0x20;
pub const MEMSTICK_OVERWRITE_PGST0: c_uint = 0x40;
pub const MEMSTICK_OVERWRITE_BKST: c_uint = 0x80;
    pub management_flag: c_uchar,
pub const MEMSTICK_MANAGEMENT_SYSFLG: c_uint = 0x04;
pub const MEMSTICK_MANAGEMENT_ATFLG: c_uint = 0x08;
pub const MEMSTICK_MANAGEMENT_SCMS1: c_uint = 0x10;
pub const MEMSTICK_MANAGEMENT_SCMS0: c_uint = 0x20;
    pub logical_address: c_ushort,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_register {
    pub status: ms_status_register,
    pub id: ms_id_register,
    pub reserved: [c_uchar; 8],
    pub param: ms_param_register,
    pub extra_data: ms_extra_data_register,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mspro_param_register {
    pub system: c_uchar,
pub const MEMSTICK_SYS_PAR4: c_uint = 0x00;
pub const MEMSTICK_SYS_PAR8: c_uint = 0x40;
pub const MEMSTICK_SYS_SERIAL: c_uint = 0x80;
    pub data_count: __be16,
    pub data_address: __be32,
    pub tpc_param: c_uchar,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mspro_io_info_register {
    pub version: c_uchar,
    pub io_category: c_uchar,
    pub current_req: c_uchar,
    pub card_opt_info: c_uchar,
    pub rdy_wait_time: c_uchar,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mspro_io_func_register {
    pub func_enable: c_uchar,
    pub func_select: c_uchar,
    pub func_intmask: c_uchar,
    pub transfer_mode: c_uchar,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mspro_io_cmd_register {
    pub tpc_param: c_ushort,
    pub data_count: c_ushort,
    pub data_address: c_uint,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mspro_register {
    pub status: ms_status_register,
    pub id: ms_id_register,
    pub reserved0: [c_uchar; 8],
    pub param: mspro_param_register,
    pub reserved1: [c_uchar; 8],
    pub io_info: mspro_io_info_register,
    pub io_func: mspro_io_func_register,
    pub reserved2: [c_uchar; 7],
    pub io_cmd: mspro_io_cmd_register,
    pub io_int: c_uchar,
    pub io_int_func: c_uchar,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_register_addr {
    pub r_offset: c_uchar,
    pub r_length: c_uchar,
    pub w_offset: c_uchar,
    pub w_length: c_uchar,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memstick_tpc {
    MS_TPC_READ_MG_STATUS   = 0x01,
    MS_TPC_READ_LONG_DATA   = 0x02,
    MS_TPC_READ_SHORT_DATA  = 0x03,
    MS_TPC_READ_MG_DATA     = 0x03,
    MS_TPC_READ_REG         = 0x04,
    MS_TPC_READ_QUAD_DATA   = 0x05,
    MS_TPC_READ_IO_DATA     = 0x05,
    MS_TPC_GET_INT          = 0x07,
    MS_TPC_SET_RW_REG_ADRS  = 0x08,
    MS_TPC_EX_SET_CMD       = 0x09,
    MS_TPC_WRITE_QUAD_DATA  = 0x0a,
    MS_TPC_WRITE_IO_DATA    = 0x0a,
    MS_TPC_WRITE_REG        = 0x0b,
    MS_TPC_WRITE_SHORT_DATA = 0x0c,
    MS_TPC_WRITE_MG_DATA    = 0x0c,
    MS_TPC_WRITE_LONG_DATA  = 0x0d,
    MS_TPC_SET_CMD          = 0x0e
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memstick_command {
    MS_CMD_BLOCK_END       = 0x33,
    MS_CMD_RESET           = 0x3c,
    MS_CMD_BLOCK_WRITE     = 0x55,
    MS_CMD_SLEEP           = 0x5a,
    MS_CMD_BLOCK_ERASE     = 0x99,
    MS_CMD_BLOCK_READ      = 0xaa,
    MS_CMD_CLEAR_BUF       = 0xc3,
    MS_CMD_FLASH_STOP      = 0xcc,
    MS_CMD_LOAD_ID         = 0x60,
    MS_CMD_CMP_ICV         = 0x7f,
    MSPRO_CMD_FORMAT       = 0x10,
    MSPRO_CMD_SLEEP        = 0x11,
    MSPRO_CMD_WAKEUP       = 0x12,
    MSPRO_CMD_READ_DATA    = 0x20,
    MSPRO_CMD_WRITE_DATA   = 0x21,
    MSPRO_CMD_READ_ATRB    = 0x24,
    MSPRO_CMD_STOP         = 0x25,
    MSPRO_CMD_ERASE        = 0x26,
    MSPRO_CMD_READ_QUAD    = 0x27,
    MSPRO_CMD_WRITE_QUAD   = 0x28,
    MSPRO_CMD_SET_IBD      = 0x46,
    MSPRO_CMD_GET_IBD      = 0x47,
    MSPRO_CMD_IN_IO_DATA   = 0xb0,
    MSPRO_CMD_OUT_IO_DATA  = 0xb1,
    MSPRO_CMD_READ_IO_ATRB = 0xb2,
    MSPRO_CMD_IN_IO_FIFO   = 0xb3,
    MSPRO_CMD_OUT_IO_FIFO  = 0xb4,
    MSPRO_CMD_IN_IOM       = 0xb5,
    MSPRO_CMD_OUT_IOM      = 0xb6,
}

// Driver structures and functions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memstick_param {

pub const MEMSTICK_POWER_OFF: c_int = 0;
pub const MEMSTICK_POWER_ON: c_int = 1;

pub const MEMSTICK_SERIAL: c_int = 0;
pub const MEMSTICK_PAR4: c_int = 1;
pub const MEMSTICK_PAR8: c_int = 2;

    struct memstick_host;
    struct memstick_driver;

    struct memstick_device_id {
    unsigned char match_flags;
pub const MEMSTICK_MATCH_ALL: c_uint = 0x01;

    unsigned char type;
pub const MEMSTICK_TYPE_LEGACY: c_uint = 0xff;
pub const MEMSTICK_TYPE_DUO: c_uint = 0x00;
pub const MEMSTICK_TYPE_PRO: c_uint = 0x01;

    unsigned char category;
pub const MEMSTICK_CATEGORY_STORAGE: c_uint = 0xff;
pub const MEMSTICK_CATEGORY_STORAGE_DUO: c_uint = 0x00;
pub const MEMSTICK_CATEGORY_IO: c_uint = 0x01;
pub const MEMSTICK_CATEGORY_IO_PRO: c_uint = 0x10;

    unsigned char class;
pub const MEMSTICK_CLASS_FLASH: c_uint = 0xff;
pub const MEMSTICK_CLASS_DUO: c_uint = 0x00;
pub const MEMSTICK_CLASS_ROM: c_uint = 0x01;
pub const MEMSTICK_CLASS_RO: c_uint = 0x02;
pub const MEMSTICK_CLASS_WP: c_uint = 0x03;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memstick_request {
    pub tpc: c_uchar,
    pub int_reg: c_uchar,
    pub error: c_int,
    pub sg: scatterlist,
    pub data_len: c_uchar,
    pub data: [c_uchar; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memstick_dev {
    pub id: memstick_device_id,
    pub host: *mut memstick_host,
    pub reg_addr: ms_register_addr,
    pub mrq_complete: completion,
    pub current_mrq: memstick_request,
// Check that media driver is still willing to operate the device.
    pub card): *mut *mut int (check)(struct memstick_dev,
// Get next request from the media driver.
    pub mrq): *mut memstick_request,
// Tell the media driver to stop doing things
    pub card): *mut *mut void (stop)(struct memstick_dev,
// Allow the media driver to continue
    pub card): *mut *mut void (start)(struct memstick_dev,
    pub dev: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memstick_host {
    pub lock: mutex,
    pub id: c_uint,
    pub caps: c_uint,
pub const MEMSTICK_CAP_AUTO_GET_INT: c_int = 1;
pub const MEMSTICK_CAP_PAR4: c_int = 2;
pub const MEMSTICK_CAP_PAR8: c_int = 4;
    pub media_checker: work_struct,
    pub dev: device,
    pub card: *mut memstick_dev,
    pub retries: c_uint,
    pub removing: bool,
// Notify the host that some requests are pending.
    pub host): *mut *mut void (request)(struct memstick_host,
// Set host IO parameters (power, clock, etc).
    pub value): c_int,
    pub ____cacheline_aligned: unsigned long private[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memstick_driver {
    pub id_table: *const memstick_device_id,
    pub card): *mut *mut int (probe)(struct memstick_dev,
    pub card): *mut *mut void (remove)(struct memstick_dev,
    pub state): pm_message_t,
    pub card): *mut *mut int (resume)(struct memstick_dev,
    pub driver: device_driver,
}

extern "C" {
    pub fn memstick_register_driver(drv: *mut memstick_driver) -> c_int;
}
extern "C" {
    pub fn memstick_unregister_driver(drv: *mut memstick_driver);
}
extern "C" {
    pub fn memstick_add_host(host: *mut memstick_host) -> c_int;
}
extern "C" {
    pub fn memstick_remove_host(host: *mut memstick_host);
}
extern "C" {
    pub fn memstick_free_host(host: *mut memstick_host);
}
extern "C" {
    pub fn memstick_detect_change(host: *mut memstick_host);
}
extern "C" {
    pub fn memstick_suspend_host(host: *mut memstick_host);
}
extern "C" {
    pub fn memstick_resume_host(host: *mut memstick_host);
}
extern "C" {
    pub fn memstick_new_req(host: *mut memstick_host);
}
extern "C" {
    pub fn memstick_set_rw_addr(card: *mut memstick_dev) -> c_int;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &card->dev) -> return;
}
