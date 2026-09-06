//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/event.h
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
// This file is part of wl1251
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008 Nokia Corporation
//
// Mbox events
//
// The event mechanism is based on a pair of event buffers (buffers A and
// B) at fixed locations in the target's memory. The host processes one
// buffer while the other buffer continues to collect events. If the host
// is not processing events, an interrupt is issued to signal that a buffer
// is ready. Once the host is done with processing events from one buffer,
// it signals the target (with an ACK interrupt) that the event buffer is
// free.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_debug_report {
    pub debug_event_id: u8,
    pub num_params: u8,
    pub pad: u16,
    pub report_1: u32,
    pub report_2: u32,
    pub report_3: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_mailbox {
    pub events_vector: u32,
    pub events_mask: u32,
    pub reserved_1: u32,
    pub reserved_2: u32,
    pub average_rssi_level: c_char,
    pub ps_status: u8,
    pub channel_switch_status: u8,
    pub scheduled_scan_status: u8,
// Channels scanned by the scheduled scan
    pub scheduled_scan_channels: u16,
// If bit 0 is set -> target's fatal error
    pub health_report: u16,
    pub bad_fft_counter: u16,
    pub bt_pta_sense_info: u8,
    pub bt_pta_protective_info: u8,
    pub reserved: u32,
    pub debug_report: [u32; 2],
// Number of FCS errors since last event
    pub fcs_err_counter: u32,
    pub report: event_debug_report,
    pub average_snr_level: u8,
    pub padding: [u8; 19],
    pub __packed: },
}

extern "C" {
    pub fn wl1251_event_unmask(wl: *mut wl1251) -> c_int;
}
extern "C" {
    pub fn wl1251_event_mbox_config(wl: *mut wl1251);
}
extern "C" {
    pub fn wl1251_event_handle(wl: *mut wl1251, mbox: u8) -> c_int;
}
extern "C" {
    pub fn wl1251_event_wait(wl: *mut wl1251, mask: u32, timeout_ms: c_int) -> c_int;
}
