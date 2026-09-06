//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxfw/mlxfw.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2017-2019 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_dev {
    pub ops: *const mlxfw_dev_ops,
    pub psid: *const c_char,
    pub psid_size: u16,
    pub devlink: *mut devlink,
}

extern "C" {
    pub fn devlink_to_dev(_arg: mlxfw_dev->devlink) -> return;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxfw_fsm_state {
    MLXFW_FSM_STATE_IDLE,
    MLXFW_FSM_STATE_LOCKED,
    MLXFW_FSM_STATE_INITIALIZE,
    MLXFW_FSM_STATE_DOWNLOAD,
    MLXFW_FSM_STATE_VERIFY,
    MLXFW_FSM_STATE_APPLY,
    MLXFW_FSM_STATE_ACTIVATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxfw_fsm_state_err {
    MLXFW_FSM_STATE_ERR_OK,
    MLXFW_FSM_STATE_ERR_ERROR,
    MLXFW_FSM_STATE_ERR_REJECTED_DIGEST_ERR,
    MLXFW_FSM_STATE_ERR_REJECTED_NOT_APPLICABLE,
    MLXFW_FSM_STATE_ERR_REJECTED_UNKNOWN_KEY,
    MLXFW_FSM_STATE_ERR_REJECTED_AUTH_FAILED,
    MLXFW_FSM_STATE_ERR_REJECTED_UNSIGNED,
    MLXFW_FSM_STATE_ERR_REJECTED_KEY_NOT_APPLICABLE,
    MLXFW_FSM_STATE_ERR_REJECTED_BAD_FORMAT,
    MLXFW_FSM_STATE_ERR_BLOCKED_PENDING_RESET,
    MLXFW_FSM_STATE_ERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxfw_fsm_reactivate_status {
    MLXFW_FSM_REACTIVATE_STATUS_OK,
    MLXFW_FSM_REACTIVATE_STATUS_BUSY,
    MLXFW_FSM_REACTIVATE_STATUS_PROHIBITED_FW_VER_ERR,
    MLXFW_FSM_REACTIVATE_STATUS_FIRST_PAGE_COPY_FAILED,
    MLXFW_FSM_REACTIVATE_STATUS_FIRST_PAGE_ERASE_FAILED,
    MLXFW_FSM_REACTIVATE_STATUS_FIRST_PAGE_RESTORE_FAILED,
    MLXFW_FSM_REACTIVATE_STATUS_CANDIDATE_FW_DEACTIVATION_FAILED,
    MLXFW_FSM_REACTIVATE_STATUS_FW_ALREADY_ACTIVATED,
    MLXFW_FSM_REACTIVATE_STATUS_ERR_DEVICE_RESET_REQUIRED,
    MLXFW_FSM_REACTIVATE_STATUS_ERR_FW_PROGRAMMING_NEEDED,
    MLXFW_FSM_REACTIVATE_STATUS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_dev_ops {
    pub p_max_write_size): *mut u16,
    pub fwhandle): *mut *mut *mut int (fsm_lock)(struct mlxfw_dev mlxfw_dev, u32,
    pub component_size): u16 component_index, u32,
    pub offset): *mut *mut u8 data, u16 size, u32,
    pub component_index): u16,
    pub fwhandle): *mut *mut *mut int (fsm_activate)(struct mlxfw_dev mlxfw_dev, u32,
    pub status): *mut *mut *mut int (fsm_reactivate)(struct mlxfw_dev mlxfw_dev, u8,
    pub fsm_state_err): *mut mlxfw_fsm_state_err,
    pub fwhandle): *mut *mut *mut void (fsm_cancel)(struct mlxfw_dev mlxfw_dev, u32,
    pub fwhandle): *mut *mut *mut void (fsm_release)(struct mlxfw_dev mlxfw_dev, u32,
}

