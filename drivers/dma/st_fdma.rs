//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/st_fdma.h
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
// DMA driver header for STMicroelectronics STi FDMA controller
//
// Copyright (C) 2014 STMicroelectronics
//
// Author: Ludovic Barre <Ludovic.barre@st.com>
//

pub const ST_FDMA_NR_DREQS: c_int = 32;
pub const FW_NAME_SIZE: c_int = 30;

//
// struct st_fdma_generic_node - Free running/paced generic node
//
// @length: Length in bytes of a line in a 2D mem to mem
// @sstride: Stride, in bytes, between source lines in a 2D data move
// @dstride: Stride, in bytes, between destination lines in a 2D data move
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_generic_node {
    pub length: u32,
    pub sstride: u32,
    pub dstride: u32,
}

//
// struct st_fdma_hw_node - Node structure used by fdma hw
//
// @next: Pointer to next node
// @control: Transfer Control Parameters
// @nbytes: Number of Bytes to read
// @saddr: Source address
// @daddr: Destination address
//
// @generic: generic node for free running/paced transfert type
// 2 others transfert type are possible, but not yet implemented
//
// The NODE structures must be aligned to a 32 byte boundary
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_hw_node {
    pub next: u32,
    pub control: u32,
    pub nbytes: u32,
    pub saddr: u32,
    pub daddr: u32,
    pub generic: st_fdma_generic_node,
}

//
// node control parameters
//

pub const FDMA_NODE_CTRL_REQ_MAP_FREE_RUN: c_uint = 0x0;

//
// struct st_fdma_sw_node - descriptor structure for link list
//
// @pdesc: Physical address of desc
// @node: link used for putting this into a channel queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_sw_node {
    pub pdesc: dma_addr_t,
    pub desc: *mut st_fdma_hw_node,
}

pub const NAME_SZ: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_driverdata {
    pub id: u32,
    pub name: [c_char; NAME_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_desc {
    pub vdesc: virt_dma_desc,
    pub fchan: *mut st_fdma_chan,
    pub iscyclic: bool,
    pub n_nodes: c_uint,
    pub __counted_by(n_nodes): st_fdma_sw_node node[],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_fdma_type {
    ST_FDMA_TYPE_FREE_RUN,
    ST_FDMA_TYPE_PACED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_cfg {
    pub of_node: *mut device_node,
    pub type: st_fdma_type,
    pub dev_addr: dma_addr_t,
    pub dir: dma_transfer_direction,
    pub /: *mut *mut int req_line; / request line,
    pub /: *mut *mut long req_ctrl; / Request control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_chan {
    pub fdev: *mut st_fdma_dev,
    pub node_pool: *mut dma_pool,
    pub scfg: dma_slave_config,
    pub cfg: st_fdma_cfg,
    pub dreq_line: c_long,
    pub vchan: virt_dma_chan,
    pub fdesc: *mut st_fdma_desc,
    pub status: dma_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_fdma_dev {
    pub dev: *mut device,
    pub drvdata: *const st_fdma_driverdata,
    pub dma_device: dma_device,
    pub slim_rproc: *mut st_slim_rproc,
    pub irq: c_int,
    pub chans: *mut st_fdma_chan,
    pub dreq_lock: spinlock_t,
    pub dreq_mask: c_ulong,
    pub nr_channels: u32,
    pub fw_name: [c_char; FW_NAME_SIZE],
}

// Peripheral Registers
pub const FDMA_CMD_STA_OFST: c_uint = 0xFC0;
pub const FDMA_CMD_SET_OFST: c_uint = 0xFC4;
pub const FDMA_CMD_CLR_OFST: c_uint = 0xFC8;
pub const FDMA_CMD_MASK_OFST: c_uint = 0xFCC;

pub const FDMA_INT_STA_OFST: c_uint = 0xFD0;
pub const FDMA_INT_STA_CH: c_uint = 0x1;
pub const FDMA_INT_STA_ERR: c_uint = 0x2;
pub const FDMA_INT_SET_OFST: c_uint = 0xFD4;
pub const FDMA_INT_CLR_OFST: c_uint = 0xFD8;
pub const FDMA_INT_MASK_OFST: c_uint = 0xFDC;

// fchan interface (dmem)
pub const FDMA_CH_CMD_OFST: c_uint = 0x200;

// req interface
pub const FDMA_REQ_CTRL_OFST: c_uint = 0x240;

// node interface
pub const FDMA_NODE_SZ: c_int = 128;
pub const FDMA_PTRN_OFST: c_uint = 0x800;
pub const FDMA_CNTN_OFST: c_uint = 0x808;
pub const FDMA_SADDRN_OFST: c_uint = 0x80c;
pub const FDMA_DADDRN_OFST: c_uint = 0x810;

//
// request control bits
//

// bits used by client to configure request control

