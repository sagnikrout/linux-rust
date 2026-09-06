//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_tmpl.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2014 QLogic Corporation
//

pub const TEMPLATE_TYPE_FWDUMP: c_int = 99;
pub const ENTRY_TYPE_NOP: c_int = 0;
pub const ENTRY_TYPE_TMP_END: c_int = 255;
pub const ENTRY_TYPE_RD_IOB_T1: c_int = 256;
pub const ENTRY_TYPE_WR_IOB_T1: c_int = 257;
pub const ENTRY_TYPE_RD_IOB_T2: c_int = 258;
pub const ENTRY_TYPE_WR_IOB_T2: c_int = 259;
pub const ENTRY_TYPE_RD_PCI: c_int = 260;
pub const ENTRY_TYPE_WR_PCI: c_int = 261;
pub const ENTRY_TYPE_RD_RAM: c_int = 262;
pub const ENTRY_TYPE_GET_QUEUE: c_int = 263;
pub const ENTRY_TYPE_GET_FCE: c_int = 264;
pub const ENTRY_TYPE_PSE_RISC: c_int = 265;
pub const ENTRY_TYPE_RST_RISC: c_int = 266;
pub const ENTRY_TYPE_DIS_INTR: c_int = 267;
pub const ENTRY_TYPE_GET_HBUF: c_int = 268;
pub const ENTRY_TYPE_SCRATCH: c_int = 269;
pub const ENTRY_TYPE_RDREMREG: c_int = 270;
pub const ENTRY_TYPE_WRREMREG: c_int = 271;
pub const ENTRY_TYPE_RDREMRAM: c_int = 272;
pub const ENTRY_TYPE_PCICFG: c_int = 273;
pub const ENTRY_TYPE_GET_SHADOW: c_int = 274;
pub const ENTRY_TYPE_WRITE_BUF: c_int = 275;
pub const ENTRY_TYPE_CONDITIONAL: c_int = 276;
pub const ENTRY_TYPE_RDPEPREG: c_int = 277;
pub const ENTRY_TYPE_WRPEPREG: c_int = 278;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub type: __le32,
    pub size: __le32,
    pub reserved_1: u32,
    pub capture_flags: u8,
    pub reserved_2: [u8; 2],
    pub driver_flags: u8,
    pub hdr: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union __packed {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub t0: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub t255: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub base_addr: __le32,
    pub reg_width: u8,
    pub reg_count: __le16,
    pub pci_offset: u8,
    pub t256: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub base_addr: __le32,
    pub write_data: __le32,
    pub pci_offset: u8,
    pub reserved: [u8; 3],
    pub t257: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub base_addr: __le32,
    pub reg_width: u8,
    pub reg_count: __le16,
    pub pci_offset: u8,
    pub banksel_offset: u8,
    pub reserved: [u8; 3],
    pub bank: __le32,
    pub t258: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub base_addr: __le32,
    pub write_data: __le32,
    pub reserved: [u8; 2],
    pub pci_offset: u8,
    pub banksel_offset: u8,
    pub bank: __le32,
    pub t259: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub pci_offset: u8,
    pub reserved: [u8; 3],
    pub t260: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub pci_offset: u8,
    pub reserved: [u8; 3],
    pub write_data: __le32,
    pub t261: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub ram_area: u8,
    pub reserved: [u8; 3],
    pub start_addr: __le32,
    pub end_addr: __le32,
    pub t262: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub num_queues: u32,
    pub queue_type: u8,
    pub reserved: [u8; 3],
    pub t263: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub fce_trace_size: u32,
    pub write_pointer: u64,
    pub base_pointer: u64,
    pub fce_enable_mb0: u32,
    pub fce_enable_mb2: u32,
    pub fce_enable_mb3: u32,
    pub fce_enable_mb4: u32,
    pub fce_enable_mb5: u32,
    pub fce_enable_mb6: u32,
    pub t264: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub t265: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub t266: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub pci_offset: u8,
    pub reserved: [u8; 3],
    pub data: __le32,
    pub t267: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub buf_type: u8,
    pub reserved: [u8; 3],
    pub buf_size: u32,
    pub start_addr: u64,
    pub t268: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub scratch_size: u32,
    pub t269: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub addr: __le32,
    pub count: __le32,
    pub t270: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub addr: __le32,
    pub data: __le32,
    pub t271: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub addr: __le32,
    pub count: __le32,
    pub t272: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub addr: __le32,
    pub count: __le32,
    pub t273: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub num_queues: u32,
    pub queue_type: u8,
    pub reserved: [u8; 3],
    pub t274: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub length: __le32,
    pub buffer: [u8; ],
    pub t275: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cond1: __le32,
    pub cond2: __le32,
    pub t276: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd_addr: __le32,
    pub wr_cmd_data: __le32,
    pub data_addr: __le32,
    pub t277: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __packed {
    pub cmd_addr: __le32,
    pub wr_cmd_data: __le32,
    pub data_addr: __le32,
    pub wr_data: __le32,
    pub t278: },
}

pub const T262_RAM_AREA_CRITICAL_RAM: c_int = 1;
pub const T262_RAM_AREA_EXTERNAL_RAM: c_int = 2;
pub const T262_RAM_AREA_SHARED_RAM: c_int = 3;
pub const T262_RAM_AREA_DDR_RAM: c_int = 4;
pub const T262_RAM_AREA_MISC: c_int = 5;
pub const T263_QUEUE_TYPE_REQ: c_int = 1;
pub const T263_QUEUE_TYPE_RSP: c_int = 2;
pub const T263_QUEUE_TYPE_ATIO: c_int = 3;
pub const T268_BUF_TYPE_EXTD_TRACE: c_int = 1;
pub const T268_BUF_TYPE_EXCH_BUFOFF: c_int = 2;
pub const T268_BUF_TYPE_EXTD_LOGIN: c_int = 3;
pub const T268_BUF_TYPE_REQ_MIRROR: c_int = 4;
pub const T268_BUF_TYPE_RSP_MIRROR: c_int = 5;
pub const T274_QUEUE_TYPE_REQ_SHAD: c_int = 1;
pub const T274_QUEUE_TYPE_RSP_SHAD: c_int = 2;
pub const T274_QUEUE_TYPE_ATIO_SHAD: c_int = 3;
