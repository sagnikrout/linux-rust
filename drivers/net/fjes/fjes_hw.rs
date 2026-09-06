//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/fjes/fjes_hw.h
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
// FUJITSU Extended Socket Network Device driver
// Copyright (c) 2015 FUJITSU LIMITED
//

pub const EP_BUFFER_SUPPORT_VLAN_MAX: c_int = 4;
pub const EP_BUFFER_INFO_SIZE: c_int = 4096;
pub const FJES_DEBUG_PAGE_SIZE: c_int = 4096;

// Frame & MTU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esmem_frame {
    pub frame_size: __le32,
    pub frame_data: [u8; ],
}

// EP partner status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ep_partner_status {
    EP_PARTNER_UNSHARE,
    EP_PARTNER_SHARED,
    EP_PARTNER_WAITING,
    EP_PARTNER_COMPLETE,
    EP_PARTNER_STATUS_MAX,
}

// shared status region
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fjes_device_shared_info {
    pub epnum: c_int,
    pub ep_status: [u8; ],
}

// structures for command control request data
#[repr(C)]
#[derive(Copy, Clone)]
pub union fjes_device_command_req {
    pub length: __le32,
    pub info: },
    pub length: __le32,
    pub epid: __le32,
    pub buffer: [__le64; ],
    pub share_buffer: },
    pub length: __le32,
    pub epid: __le32,
    pub unshare_buffer: },
    pub length: __le32,
    pub mode: __le32,
    pub buffer_len: __le64,
    pub buffer: [__le64; ],
    pub start_trace: },
    pub length: __le32,
    pub stop_trace: },
}

// structures for command control response data
#[repr(C)]
#[derive(Copy, Clone)]
pub union fjes_device_command_res {
    pub length: __le32,
    pub code: __le32,
    pub es_status: u8,
    pub zone: u8,
    pub info: [}; ],
    pub info: },
    pub length: __le32,
    pub code: __le32,
    pub share_buffer: },
    pub length: __le32,
    pub code: __le32,
    pub unshare_buffer: },
    pub length: __le32,
    pub code: __le32,
    pub start_trace: },
    pub length: __le32,
    pub code: __le32,
    pub stop_trace: },
}

// request command type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fjes_dev_command_request_type {
    FJES_CMD_REQ_INFO		= 0x0001,
    FJES_CMD_REQ_SHARE_BUFFER	= 0x0002,
    FJES_CMD_REQ_UNSHARE_BUFFER	= 0x0004,
    FJES_CMD_REQ_START_DEBUG	= 0x0100,
    FJES_CMD_REQ_STOP_DEBUG		= 0x0200,
}

// parameter for command control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fjes_device_command_param {
    pub req_len: u32,
    pub req_start: phys_addr_t,
    pub res_len: u32,
    pub res_start: phys_addr_t,
    pub share_start: phys_addr_t,
}

// error code for command control
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fjes_dev_command_response_e {
    FJES_CMD_STATUS_UNKNOWN,
    FJES_CMD_STATUS_NORMAL,
    FJES_CMD_STATUS_TIMEOUT,
    FJES_CMD_STATUS_ERROR_PARAM,
    FJES_CMD_STATUS_ERROR_STATUS,
}

// EP buffer information
#[repr(C)]
#[derive(Copy, Clone)]
pub union ep_buffer_info {
    pub raw: [u8; EP_BUFFER_INFO_SIZE],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ep_buffer_info_common_t {
    pub version: u32,
    pub common: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ep_buffer_info_v1_t {
    pub version: u32,
    pub info_size: u32,
    pub buffer_size: u32,
    pub count_max: u16,
    pub _rsv_1: u16,
    pub frame_max: u32,
    pub mac_addr: [u8; ETH_ALEN],
    pub _rsv_2: u16,
    pub _rsv_3: u32,
    pub tx_status: u16,
    pub rx_status: u16,
    pub head: u32,
    pub tail: u32,
    pub vlan_id: [u16; EP_BUFFER_SUPPORT_VLAN_MAX],
    pub v1i: },
}

// statistics of EP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fjes_drv_ep_stats {
    pub com_regist_buf_exec: u64,
    pub com_unregist_buf_exec: u64,
    pub send_intr_rx: u64,
    pub send_intr_unshare: u64,
    pub send_intr_zoneupdate: u64,
    pub recv_intr_rx: u64,
    pub recv_intr_unshare: u64,
    pub recv_intr_stop: u64,
    pub recv_intr_zoneupdate: u64,
    pub tx_buffer_full: u64,
    pub tx_dropped_not_shared: u64,
    pub tx_dropped_ver_mismatch: u64,
    pub tx_dropped_buf_size_mismatch: u64,
    pub tx_dropped_vlanid_mismatch: u64,
}

// buffer pair for Extended Partition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep_share_mem_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct epbuf_handler {
    pub buffer: *mut c_void,
    pub size: usize,
    pub info: *mut ep_buffer_info,
    pub ring: *mut u8,
    pub rx: } tx,,
    pub net_stats: rtnl_link_stats64,
    pub ep_stats: fjes_drv_ep_stats,
    pub tx_status_work: u16,
    pub es_status: u8,
    pub zone: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct es_device_trace {
    pub record_num: u32,
    pub current_record: u32,
    pub status_flag: u32,
    pub _rsv: u32,
    pub epid: u16,
    pub dir_offset: u16,
    pub data: u32,
    pub tsc: u64,
    pub record: [}; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fjes_hw_info {
    pub share: *mut fjes_device_shared_info,
    pub req_buf: *mut fjes_device_command_req,
    pub req_buf_size: u64,
    pub res_buf: *mut fjes_device_command_res,
    pub res_buf_size: u64,
    pub my_epid: *mut c_int,
    pub max_epid: *mut c_int,
    pub trace: *mut es_device_trace,
    pub trace_size: u64,
    pub lock*/: *mut *mut mutex lock; / buffer,
    pub buffer_share_bit: c_ulong,
    pub buffer_unshare_reserve_bit: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fjes_hw {
    pub back: *mut c_void,
    pub txrx_stop_req_bit: c_ulong,
    pub epstop_req_bit: c_ulong,
    pub update_zone_task: work_struct,
    pub epstop_task: work_struct,
    pub my_epid: c_int,
    pub max_epid: c_int,
    pub ep_shm_info: *mut ep_share_mem_info,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fjes_hw_resource {
    pub start: u64,
    pub size: u64,
    pub irq: c_int,
    pub hw_res: },
    pub base: *mut u8,
    pub hw_info: fjes_hw_info,
    pub /: *mut *mut spinlock_t rx_status_lock; / spinlock for rx_status,
    pub debug_mode: u32,
}

extern "C" {
    pub fn fjes_hw_init(: *mut fjes_hw) -> c_int;
}
extern "C" {
    pub fn fjes_hw_exit(: *mut fjes_hw);
}
extern "C" {
    pub fn fjes_hw_reset(: *mut fjes_hw) -> c_int;
}
extern "C" {
    pub fn fjes_hw_request_info(: *mut fjes_hw) -> c_int;
}
extern "C" {
    pub fn fjes_hw_unregister_buff_addr(: *mut fjes_hw, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn fjes_hw_setup_epbuf(: *mut epbuf_handler, : *const u8, _arg: u32);
}
extern "C" {
    pub fn fjes_hw_raise_interrupt(: *mut fjes_hw, _arg: c_int, REG_ICTL_MASK: enum) -> c_int;
}
extern "C" {
    pub fn fjes_hw_set_irqmask(: *mut fjes_hw, REG_ICTL_MASK: enum, _arg: bool);
}
extern "C" {
    pub fn fjes_hw_capture_interrupt_status(: *mut fjes_hw) -> u32;
}
extern "C" {
    pub fn fjes_hw_raise_epstop(: *mut fjes_hw);
}
extern "C" {
    pub fn fjes_hw_wait_epstop(: *mut fjes_hw) -> c_int;
}
extern "C" {
    pub fn fjes_hw_epid_is_same_zone(: *mut fjes_hw, _arg: c_int) -> bool;
}
extern "C" {
    pub fn fjes_hw_epid_is_shared(: *mut fjes_device_shared_info, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn fjes_hw_check_epbuf_version(: *mut epbuf_handler, _arg: u32) -> bool;
}
extern "C" {
    pub fn fjes_hw_check_mtu(: *mut epbuf_handler, _arg: u32) -> bool;
}
extern "C" {
    pub fn fjes_hw_check_vlan_id(: *mut epbuf_handler, _arg: u16) -> bool;
}
extern "C" {
    pub fn fjes_hw_set_vlan_id(: *mut epbuf_handler, _arg: u16) -> bool;
}
extern "C" {
    pub fn fjes_hw_del_vlan_id(: *mut epbuf_handler, _arg: u16);
}
extern "C" {
    pub fn fjes_hw_epbuf_rx_is_empty(: *mut epbuf_handler) -> bool;
}
extern "C" {
    pub fn fjes_hw_epbuf_rx_curpkt_drop(: *mut epbuf_handler);
}
extern "C" {
    pub fn fjes_hw_epbuf_tx_pkt_send(: *mut epbuf_handler, : *mut c_void, _arg: usize) -> c_int;
}
extern "C" {
    pub fn fjes_hw_start_debug(: *mut fjes_hw) -> c_int;
}
extern "C" {
    pub fn fjes_hw_stop_debug(: *mut fjes_hw) -> c_int;
}
