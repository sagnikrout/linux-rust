//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/hif.h
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
// Copyright (c) 2017-2026 Morse Micro
//

// Hardware IF interrupt mask. We may use any interrupts in this range

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_hif_flags {
    MM81X_HIF_FLAGS_DIR_TO_HOST = BIT(0),
    MM81X_HIF_FLAGS_DIR_TO_CHIP = BIT(1),
    MM81X_HIF_FLAGS_COMMAND = BIT(2),
    MM81X_HIF_FLAGS_BEACON = BIT(3),
    MM81X_HIF_FLAGS_DATA = BIT(4)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_hif_ops {
    pub mors): *mut *mut int (init)(struct mm81x,
    pub mors): *mut *mut void (flush_tx_data)(struct mm81x,
    pub mors): *mut *mut void (flush_cmds)(struct mm81x,
    pub mors): *mut *mut void (finish)(struct mm81x,
    pub num_qs): *mut c_int,
    pub mors): *mut *mut *mut mm81x_skbq (get_tx_cmd_queue)(mm81x,
    pub mors): *mut *mut *mut mm81x_skbq (get_tx_beacon_queue)(mm81x,
    pub mors): *mut *mut *mut mm81x_skbq (get_tx_mgmt_queue)(mm81x,
    pub aci): *mut *mut *mut *mut mm81x_skbq (get_tx_data_queue)(mm81x mors, int,
    pub status): *mut *mut *mut int (handle_irq)(struct mm81x mors, u32,
    pub mors): *mut *mut int (get_tx_buffered_count)(struct mm81x,
    pub mors): *mut *mut int (get_tx_status_pending_count)(struct mm81x,
}
