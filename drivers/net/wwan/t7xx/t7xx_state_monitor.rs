//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/t7xx/t7xx_state_monitor.h
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
//
// Authors:
// Amir Hanania <amir.hanania@intel.com>
// Haijun Liu <haijun.liu@mediatek.com>
// Moises Veleta <moises.veleta@intel.com>
//
// Contributors:
// Eliot Lee <eliot.lee@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_fsm_state {
    FSM_STATE_INIT,
    FSM_STATE_PRE_START,
    FSM_STATE_STARTING,
    FSM_STATE_READY,
    FSM_STATE_EXCEPTION,
    FSM_STATE_STOPPING,
    FSM_STATE_STOPPED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_fsm_event_state {
    FSM_EVENT_INVALID,
    FSM_EVENT_MD_HS2,
    FSM_EVENT_AP_HS2,
    FSM_EVENT_MD_EX,
    FSM_EVENT_MD_EX_REC_OK,
    FSM_EVENT_MD_EX_PASS,
    FSM_EVENT_MD_HS2_EXIT,
    FSM_EVENT_AP_HS2_EXIT,
    FSM_EVENT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_fsm_cmd_state {
    FSM_CMD_INVALID,
    FSM_CMD_START,
    FSM_CMD_EXCEPTION,
    FSM_CMD_PRE_STOP,
    FSM_CMD_STOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_ex_reason {
    EXCEPTION_HS_TIMEOUT,
    EXCEPTION_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum t7xx_md_irq_type {
    MD_IRQ_WDT,
    MD_IRQ_CCIF_EX,
    MD_IRQ_PORT_ENUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum md_state {
    MD_STATE_INVALID,
    MD_STATE_WAITING_FOR_HS1,
    MD_STATE_WAITING_FOR_HS2,
    MD_STATE_READY,
    MD_STATE_EXCEPTION,
    MD_STATE_WAITING_TO_STOP,
    MD_STATE_STOPPED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_fsm_ctl {
    pub md: *mut t7xx_modem,
    pub md_state: md_state,
    pub curr_state: c_uint,
    pub command_queue: list_head,
    pub event_queue: list_head,
    pub command_wq: wait_queue_head_t,
    pub event_wq: wait_queue_head_t,
    pub async_hk_wq: wait_queue_head_t,
    pub /: *mut *mut spinlock_t event_lock; / Protects event queue,
    pub /: *mut *mut spinlock_t command_lock; / Protects command queue,
    pub fsm_thread: *mut task_struct,
    pub exp_flg: bool,
    pub /: *mut *mut spinlock_t notifier_lock; / Protects notifier list,
    pub notifier_list: list_head,
    pub /: *mut *mut u32 status; / Device boot stage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_fsm_event {
    pub entry: list_head,
    pub event_id: t7xx_fsm_event_state,
    pub length: c_uint,
    pub __counted_by(length): unsigned char data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_fsm_command {
    pub entry: list_head,
    pub cmd_id: t7xx_fsm_cmd_state,
    pub flag: c_uint,
    pub done: completion,
    pub result: c_int,
    pub refcnt: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct t7xx_fsm_notifier {
    pub entry: list_head,
    pub data): *mut *mut int (notifier_fn)(enum md_state state, void,
    pub data: *mut c_void,
}

extern "C" {
    pub fn t7xx_fsm_clr_event(ctl: *mut t7xx_fsm_ctl, event_id: t7xx_fsm_event_state);
}
extern "C" {
    pub fn t7xx_fsm_broadcast_state(ctl: *mut t7xx_fsm_ctl, state: md_state);
}
extern "C" {
    pub fn t7xx_fsm_reset(md: *mut t7xx_modem);
}
extern "C" {
    pub fn t7xx_fsm_init(md: *mut t7xx_modem) -> c_int;
}
extern "C" {
    pub fn t7xx_fsm_uninit(md: *mut t7xx_modem);
}
extern "C" {
    pub fn t7xx_fsm_recv_md_intr(ctl: *mut t7xx_fsm_ctl, type: t7xx_md_irq_type) -> c_int;
}
extern "C" {
    pub fn t7xx_fsm_get_md_state(ctl: *mut t7xx_fsm_ctl) -> md_state;
}
extern "C" {
    pub fn t7xx_fsm_get_ctl_state(ctl: *mut t7xx_fsm_ctl) -> c_uint;
}
extern "C" {
    pub fn t7xx_fsm_notifier_register(md: *mut t7xx_modem, notifier: *mut t7xx_fsm_notifier);
}
extern "C" {
    pub fn t7xx_fsm_notifier_unregister(md: *mut t7xx_modem, notifier: *mut t7xx_fsm_notifier);
}
