//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic3/hinic3_eqs.h
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
// Copyright (c) Huawei Technologies Co., Ltd. 2025. All rights reserved.

pub const HINIC3_MAX_AEQS: c_int = 4;
pub const HINIC3_MAX_CEQS: c_int = 32;
pub const HINIC3_AEQ_MAX_PAGES: c_int = 4;
pub const HINIC3_CEQ_MAX_PAGES: c_int = 8;
pub const HINIC3_AEQE_SIZE: c_int = 64;
pub const HINIC3_CEQE_SIZE: c_int = 4;
pub const HINIC3_AEQE_DESC_SIZE: c_int = 4;

pub const HINIC3_DEFAULT_AEQ_LEN: c_uint = 0x10000;
pub const HINIC3_DEFAULT_CEQ_LEN: c_uint = 0x10000;
pub const HINIC3_EQ_IRQ_NAME_LEN: c_int = 64;
pub const HINIC3_EQ_USLEEP_LOW_BOUND: c_int = 900;
pub const HINIC3_EQ_USLEEP_HIGH_BOUND: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_eq_type {
    HINIC3_AEQ = 0,
    HINIC3_CEQ = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_eq_intr_mode {
    HINIC3_INTR_MODE_ARMED  = 0,
    HINIC3_INTR_MODE_ALWAYS = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_eq_ci_arm_state {
    HINIC3_EQ_NOT_ARMED = 0,
    HINIC3_EQ_ARMED     = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_eq {
    pub hwdev: *mut hinic3_hwdev,
    pub qpages: hinic3_queue_pages,
    pub q_id: u16,
    pub type: hinic3_eq_type,
    pub eq_len: u32,
    pub cons_idx: u32,
    pub wrapped: u8,
    pub irq_id: u32,
    pub msix_entry_idx: u16,
    pub irq_name: [c_char; HINIC3_EQ_IRQ_NAME_LEN],
    pub aeq_work: work_struct,
    pub soft_intr_jif: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_aeq_elem {
    pub aeqe_data: [u8; HINIC3_AEQE_DATA_SIZE],
    pub desc: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_aeq_type {
    HINIC3_HW_INTER_INT   = 0,
    HINIC3_MBX_FROM_FUNC  = 1,
    HINIC3_MSG_FROM_FW    = 2,
    HINIC3_MAX_AEQ_EVENTS = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_aeqs {
    pub hwdev: *mut hinic3_hwdev,
    pub aeq_cb: [hinic3_aeq_event_cb; HINIC3_MAX_AEQ_EVENTS],
    pub aeq: [hinic3_eq; HINIC3_MAX_AEQS],
    pub num_aeqs: u16,
    pub workq: *mut workqueue_struct,
// lock for aeq event flag
    pub aeq_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic3_ceq_event {
    HINIC3_CMDQ           = 3,
    HINIC3_MAX_CEQ_EVENTS = 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic3_ceqs {
    pub hwdev: *mut hinic3_hwdev,
    pub ceq_cb: [hinic3_ceq_event_cb; HINIC3_MAX_CEQ_EVENTS],
    pub ceq: [hinic3_eq; HINIC3_MAX_CEQS],
    pub num_ceqs: u16,
// lock for ceq event flag
    pub ceq_lock: spinlock_t,
}

extern "C" {
    pub fn hinic3_aeqs_free(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_dump_aeq_info(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_ceqs_free(hwdev: *mut hinic3_hwdev);
}
extern "C" {
    pub fn hinic3_dump_ceq_info(hwdev: *mut hinic3_hwdev);
}
