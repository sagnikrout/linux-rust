//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/bng_re/bng_fw.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2025 Broadcom.

// FW DB related
pub const BNG_FW_CMDQ_TRIG_VAL: c_int = 1;
pub const BNG_FW_COMM_PCI_BAR_REGION: c_int = 0;
pub const BNG_FW_COMM_CONS_PCI_BAR_REGION: c_int = 2;
pub const BNG_FW_DBR_BASE_PAGE_SHIFT: c_int = 12;
pub const BNG_FW_COMM_SIZE: c_uint = 0x104;
pub const BNG_FW_COMM_BASE_OFFSET: c_uint = 0x600;
pub const BNG_FW_COMM_TRIG_OFFSET: c_uint = 0x100;
pub const BNG_FW_PF_VF_COMM_PROD_OFFSET: c_uint = 0xc;
pub const BNG_FW_CREQ_DB_LEN: c_int = 8;
// CREQ

pub const BNG_FW_CREQE_UNITS: c_int = 16;
pub const BNG_FW_CREQ_ENTRY_POLL_BUDGET: c_uint = 0x100;

pub const BNG_FW_CREQ_ENTRY_POLL_BUDGET: c_uint = 0x100;
// CMDQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_fw_cmdqe {
    pub data: [u8; 16],
}

pub const BNG_FW_CMDQE_MAX_CNT: c_int = 8192;

pub const BNG_FW_CMD_IS_BLOCKING: c_uint = 0x8000;
// Crsq buf is 1024-Byte
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_crsbe {
    pub data: [u8; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_cmdq_mbox {
    pub reg: bng_re_reg_desc,
    pub prod: *mut void __iomem,
    pub db: *mut void __iomem,
}

// HWQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_cmdq_ctx {
    pub hwq: bng_re_hwq,
    pub cmdq_mbox: bng_re_cmdq_mbox,
    pub flags: c_ulong,

    pub waitq: wait_queue_head_t,
    pub seq_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_creq_db {
    pub reg: bng_re_reg_desc,
    pub dbinfo: bng_re_db_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_creq_stat {
    pub creq_qp_event_processed: u64,
    pub creq_func_event_processed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_creq_ctx {
    pub hwq: bng_re_hwq,
    pub creq_db: bng_re_creq_db,
    pub stats: bng_re_creq_stat,
    pub creq_tasklet: tasklet_struct,
    pub ring_id: u16,
    pub msix_vec: c_int,
    pub irq_handler_avail: bool,
    pub irq_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_crsqe {
    pub resp: *mut creq_qp_event,
    pub req_size: u32,
// Free slots at the time of submission
    pub free_slots: u32,
    pub opcode: u8,
    pub is_waiter_alive: bool,
    pub is_in_used: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_rcfw_sbuf {
    pub sb: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: u32,
}

// RoCE FW Communication Channels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_rcfw {
    pub pdev: *mut pci_dev,
    pub res: *mut bng_re_res,
    pub cmdq: bng_re_cmdq_ctx,
    pub creq: bng_re_creq_ctx,
    pub crsqe_tbl: *mut bng_re_crsqe,
// To synchronize the qp-handle hash table
    pub tbl_lock: spinlock_t,
    pub cmdq_depth: u32,
// cached from chip cctx for quick reference in slow path
    pub max_timeout: u16,
    pub rcfw_intr_enabled: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bng_re_cmdqmsg {
    pub req: *mut cmdq_base,
    pub resp: *mut creq_base,
    pub sb: *mut c_void,
    pub req_sz: u32,
    pub res_sz: u32,
    pub block: u8,
}

// Get the number of command units required for the req. The
// function returns correct value only if called before
// setting using bng_re_set_cmd_slots
//
extern "C" {
    pub fn bng_re_free_rcfw_channel(rcfw: *mut bng_re_rcfw);
}
extern "C" {
    pub fn bng_re_disable_rcfw_channel(rcfw: *mut bng_re_rcfw);
}
extern "C" {
    pub fn bng_re_rcfw_stop_irq(rcfw: *mut bng_re_rcfw, kill: bool);
}
extern "C" {
    pub fn bng_re_deinit_rcfw(rcfw: *mut bng_re_rcfw) -> c_int;
}
