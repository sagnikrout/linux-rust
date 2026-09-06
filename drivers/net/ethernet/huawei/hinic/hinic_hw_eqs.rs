//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_hw_eqs.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const HINIC_AEQ_CTRL_0_INT_IDX_SHIFT: c_int = 0;
pub const HINIC_AEQ_CTRL_0_DMA_ATTR_SHIFT: c_int = 12;
pub const HINIC_AEQ_CTRL_0_PCI_INTF_IDX_SHIFT: c_int = 20;
pub const HINIC_AEQ_CTRL_0_INT_MODE_SHIFT: c_int = 31;
pub const HINIC_AEQ_CTRL_0_INT_IDX_MASK: c_uint = 0x3FF;
pub const HINIC_AEQ_CTRL_0_DMA_ATTR_MASK: c_uint = 0x3F;
pub const HINIC_AEQ_CTRL_0_PCI_INTF_IDX_MASK: c_uint = 0x3;
pub const HINIC_AEQ_CTRL_0_INT_MODE_MASK: c_uint = 0x1;

pub const HINIC_AEQ_CTRL_1_LEN_SHIFT: c_int = 0;
pub const HINIC_AEQ_CTRL_1_ELEM_SIZE_SHIFT: c_int = 24;
pub const HINIC_AEQ_CTRL_1_PAGE_SIZE_SHIFT: c_int = 28;
pub const HINIC_AEQ_CTRL_1_LEN_MASK: c_uint = 0x1FFFFF;
pub const HINIC_AEQ_CTRL_1_ELEM_SIZE_MASK: c_uint = 0x3;
pub const HINIC_AEQ_CTRL_1_PAGE_SIZE_MASK: c_uint = 0xF;

pub const HINIC_CEQ_CTRL_0_INTR_IDX_SHIFT: c_int = 0;
pub const HINIC_CEQ_CTRL_0_DMA_ATTR_SHIFT: c_int = 12;
pub const HINIC_CEQ_CTRL_0_KICK_THRESH_SHIFT: c_int = 20;
pub const HINIC_CEQ_CTRL_0_PCI_INTF_IDX_SHIFT: c_int = 24;
pub const HINIC_CEQ_CTRL_0_INTR_MODE_SHIFT: c_int = 31;
pub const HINIC_CEQ_CTRL_0_INTR_IDX_MASK: c_uint = 0x3FF;
pub const HINIC_CEQ_CTRL_0_DMA_ATTR_MASK: c_uint = 0x3F;
pub const HINIC_CEQ_CTRL_0_KICK_THRESH_MASK: c_uint = 0xF;
pub const HINIC_CEQ_CTRL_0_PCI_INTF_IDX_MASK: c_uint = 0x3;
pub const HINIC_CEQ_CTRL_0_INTR_MODE_MASK: c_uint = 0x1;

pub const HINIC_CEQ_CTRL_1_LEN_SHIFT: c_int = 0;
pub const HINIC_CEQ_CTRL_1_PAGE_SIZE_SHIFT: c_int = 28;
pub const HINIC_CEQ_CTRL_1_LEN_MASK: c_uint = 0x1FFFFF;
pub const HINIC_CEQ_CTRL_1_PAGE_SIZE_MASK: c_uint = 0xF;

pub const HINIC_EQ_ELEM_DESC_TYPE_SHIFT: c_int = 0;
pub const HINIC_EQ_ELEM_DESC_SRC_SHIFT: c_int = 7;
pub const HINIC_EQ_ELEM_DESC_SIZE_SHIFT: c_int = 8;
pub const HINIC_EQ_ELEM_DESC_WRAPPED_SHIFT: c_int = 31;
pub const HINIC_EQ_ELEM_DESC_TYPE_MASK: c_uint = 0x7F;
pub const HINIC_EQ_ELEM_DESC_SRC_MASK: c_uint = 0x1;
pub const HINIC_EQ_ELEM_DESC_SIZE_MASK: c_uint = 0xFF;
pub const HINIC_EQ_ELEM_DESC_WRAPPED_MASK: c_uint = 0x1;

pub const HINIC_EQ_CI_IDX_SHIFT: c_int = 0;
pub const HINIC_EQ_CI_WRAPPED_SHIFT: c_int = 20;
pub const HINIC_EQ_CI_XOR_CHKSUM_SHIFT: c_int = 24;
pub const HINIC_EQ_CI_INT_ARMED_SHIFT: c_int = 31;
pub const HINIC_EQ_CI_IDX_MASK: c_uint = 0xFFFFF;
pub const HINIC_EQ_CI_WRAPPED_MASK: c_uint = 0x1;
pub const HINIC_EQ_CI_XOR_CHKSUM_MASK: c_uint = 0xF;
pub const HINIC_EQ_CI_INT_ARMED_MASK: c_uint = 0x1;

pub const HINIC_MAX_AEQS: c_int = 4;
pub const HINIC_MAX_CEQS: c_int = 32;
pub const HINIC_AEQE_SIZE: c_int = 64;
pub const HINIC_CEQE_SIZE: c_int = 4;
pub const HINIC_AEQE_DESC_SIZE: c_int = 4;

pub const HINIC_DEFAULT_AEQ_LEN: c_int = 64;
pub const HINIC_DEFAULT_CEQ_LEN: c_int = 1024;

pub const HINIC_CEQ_ID_CMDQ: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_eq_type {
    HINIC_AEQ,
    HINIC_CEQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_aeq_type {
    HINIC_MBX_FROM_FUNC = 1,
    HINIC_MSG_FROM_MGMT_CPU = 2,
    HINIC_MBX_SEND_RSLT = 5,
    HINIC_MAX_AEQ_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_ceq_type {
    HINIC_CEQ_CMDQ = 3,

    HINIC_MAX_CEQ_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_eqe_state {
    HINIC_EQE_ENABLED = BIT(0),
    HINIC_EQE_RUNNING = BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_aeq_elem {
    pub data: [u8; HINIC_AEQE_DATA_SIZE],
    pub desc: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_eq_work {
    pub work: work_struct,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_eq {
    pub hwif: *mut hinic_hwif,
    pub hwdev: *mut hinic_hwdev,
    pub type: hinic_eq_type,
    pub q_id: c_int,
    pub q_len: u32,
    pub page_size: u32,
    pub cons_idx: u32,
    pub wrapped: c_int,
    pub elem_size: usize,
    pub num_pages: c_int,
    pub num_elem_in_pg: c_int,
    pub msix_entry: msix_entry,
    pub irq_name: [c_char; 64],
    pub dma_addr: *mut dma_addr_t,
    pub virt_addr: *mut c_void,
    pub aeq_work: hinic_eq_work,
    pub ceq_tasklet: tasklet_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_hw_event_cb {
    pub size): *mut *mut *mut *mut void (hwe_handler)(void handle, void data, u8,
    pub handle: *mut c_void,
    pub hwe_state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_aeqs {
    pub hwif: *mut hinic_hwif,
    pub aeq: [hinic_eq; HINIC_MAX_AEQS],
    pub num_aeqs: c_int,
    pub hwe_cb: [hinic_hw_event_cb; HINIC_MAX_AEQ_EVENTS],
    pub workq: *mut workqueue_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_ceq_cb {
    pub ceqe_data): *mut *mut *mut void (handler)(void handle, u32,
    pub handle: *mut c_void,
    pub ceqe_state: hinic_eqe_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_ceqs {
    pub hwif: *mut hinic_hwif,
    pub hwdev: *mut hinic_hwdev,
    pub ceq: [hinic_eq; HINIC_MAX_CEQS],
    pub num_ceqs: c_int,
    pub ceq_cb: [hinic_ceq_cb; HINIC_MAX_CEQ_EVENTS],
}

extern "C" {
    pub fn hinic_aeqs_free(aeqs: *mut hinic_aeqs);
}
extern "C" {
    pub fn hinic_ceqs_free(ceqs: *mut hinic_ceqs);
}
extern "C" {
    pub fn hinic_dump_ceq_info(hwdev: *mut hinic_hwdev);
}
extern "C" {
    pub fn hinic_dump_aeq_info(hwdev: *mut hinic_hwdev);
}
