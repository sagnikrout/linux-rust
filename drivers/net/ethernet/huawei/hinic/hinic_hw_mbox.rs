//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_mbox.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//
pub const HINIC_MBOX_PF_SEND_ERR: c_uint = 0x1;
pub const HINIC_MBOX_PF_BUSY_ACTIVE_FW: c_uint = 0x2;
pub const HINIC_MBOX_VF_CMD_ERROR: c_uint = 0x3;
pub const HINIC_MAX_FUNCTIONS: c_int = 512;
pub const HINIC_MAX_PF_FUNCS: c_int = 16;

pub const HINIC_FUNC_CSR_MAILBOX_DATA_OFF: c_uint = 0x80;
pub const HINIC_FUNC_CSR_MAILBOX_CONTROL_OFF: c_uint = 0x0100;
pub const HINIC_FUNC_CSR_MAILBOX_INT_OFFSET_OFF: c_uint = 0x0104;
pub const HINIC_FUNC_CSR_MAILBOX_RESULT_H_OFF: c_uint = 0x0108;
pub const HINIC_FUNC_CSR_MAILBOX_RESULT_L_OFF: c_uint = 0x010C;
pub const MAX_FUNCTION_NUM: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_cmd_check_handle {
    pub cmd: u8,
    pub in_size): *mut *mut void buf_in, u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_mbox_ack_type {
    MBOX_ACK,
    MBOX_NO_ACK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_msg_info {
    pub msg_id: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_recv_mbox {
    pub recv_done: completion,
    pub mbox: *mut c_void,
    pub cmd: u8,
    pub mod: hinic_mod_type,
    pub mbox_len: u16,
    pub buf_out: *mut c_void,
    pub ack_type: hinic_mbox_ack_type,
    pub msg_info: mbox_msg_info,
    pub seq_id: u8,
    pub msg_cnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_send_mbox {
    pub send_done: completion,
    pub data: *mut u8,
    pub wb_status: *mut u64,
    pub wb_vaddr: *mut c_void,
    pub wb_paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mbox_event_state {
    EVENT_START = 0,
    EVENT_FAIL,
    EVENT_TIMEOUT,
    EVENT_END,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_mbox_cb_state {
    HINIC_VF_MBOX_CB_REG = 0,
    HINIC_VF_MBOX_CB_RUNNING,
    HINIC_PF_MBOX_CB_REG,
    HINIC_PF_MBOX_CB_RUNNING,
    HINIC_PPF_MBOX_CB_REG,
    HINIC_PPF_MBOX_CB_RUNNING,
    HINIC_PPF_TO_PF_MBOX_CB_REG,
    HINIC_PPF_TO_PF_MBOX_CB_RUNNIG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_mbox_func_to_func {
    pub hwdev: *mut hinic_hwdev,
    pub hwif: *mut hinic_hwif,
    pub mbox_send_sem: semaphore,
    pub msg_send_sem: semaphore,
    pub send_mbox: hinic_send_mbox,
    pub workq: *mut workqueue_struct,
    pub mbox_resp: [hinic_recv_mbox; HINIC_MAX_FUNCTIONS],
    pub mbox_send: [hinic_recv_mbox; HINIC_MAX_FUNCTIONS],
    pub vf_mbox_cb: [hinic_vf_mbox_cb; HINIC_MOD_MAX],
    pub pf_mbox_cb: [hinic_pf_mbox_cb; HINIC_MOD_MAX],
    pub pf_mbox_cb_state: [c_ulong; HINIC_MOD_MAX],
    pub vf_mbox_cb_state: [c_ulong; HINIC_MOD_MAX],
    pub send_msg_id: u8,
    pub event_flag: mbox_event_state,
// lock for mbox event flag
    pub mbox_lock: spinlock_t,
    pub vf_mbx_old_rand_id: [u32; MAX_FUNCTION_NUM],
    pub vf_mbx_rand_id: [u32; MAX_FUNCTION_NUM],
    pub support_vf_random: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_mbox_work {
    pub work: work_struct,
    pub src_func_idx: u16,
    pub func_to_func: *mut hinic_mbox_func_to_func,
    pub recv_mbox: *mut hinic_recv_mbox,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_cmd_msg_handle {
    pub cmd: u8,
    pub out_size): *mut *mut void buf_out, u16,
}

extern "C" {
    pub fn hinic_func_to_func_init(hwdev: *mut hinic_hwdev) -> c_int;
}
extern "C" {
    pub fn hinic_func_to_func_free(hwdev: *mut hinic_hwdev);
}
extern "C" {
    pub fn hinic_vf_mbox_random_id_init(hwdev: *mut hinic_hwdev) -> c_int;
}
