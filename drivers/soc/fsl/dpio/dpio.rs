//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/fsl/dpio/dpio.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2016 NXP
//
// enum dpio_channel_mode - DPIO notification channel mode
// @DPIO_NO_CHANNEL: No support for notification channel
// @DPIO_LOCAL_CHANNEL: Notifications on data availability can be received by a
// dedicated channel in the DPIO; user should point the queue's
// destination in the relevant interface to this DPIO
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpio_channel_mode {
    DPIO_NO_CHANNEL = 0,
    DPIO_LOCAL_CHANNEL = 1,
}

//
// struct dpio_cfg - Structure representing DPIO configuration
// @channel_mode: Notification channel mode
// @num_priorities: Number of priorities for the notification channel (1-8);
// relevant only if 'channel_mode = DPIO_LOCAL_CHANNEL'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpio_cfg {
    pub channel_mode: dpio_channel_mode,
    pub num_priorities: u8,
}

//
// struct dpio_attr - Structure representing DPIO attributes
// @id: DPIO object ID
// @qbman_portal_ce_offset: offset of the software portal cache-enabled area
// @qbman_portal_ci_offset: offset of the software portal cache-inhibited area
// @qbman_portal_id: Software portal ID
// @channel_mode: Notification channel mode
// @num_priorities: Number of priorities for the notification channel (1-8);
// relevant only if 'channel_mode = DPIO_LOCAL_CHANNEL'
// @qbman_version: QBMAN version
// @clk: QBMAN clock frequency value in Hz
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpio_attr {
    pub id: c_int,
    pub qbman_portal_ce_offset: u64,
    pub qbman_portal_ci_offset: u64,
    pub qbman_portal_id: u16,
    pub channel_mode: dpio_channel_mode,
    pub num_priorities: u8,
    pub qbman_version: u32,
    pub clk: u32,
}
