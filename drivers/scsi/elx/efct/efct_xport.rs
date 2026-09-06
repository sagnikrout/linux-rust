//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/efct/efct_xport.h
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
//
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_xport_ctrl {
    EFCT_XPORT_PORT_ONLINE = 1,
    EFCT_XPORT_PORT_OFFLINE,
    EFCT_XPORT_SHUTDOWN,
    EFCT_XPORT_POST_NODE_EVENT,
    EFCT_XPORT_WWNN_SET,
    EFCT_XPORT_WWPN_SET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_xport_status {
    EFCT_XPORT_PORT_STATUS,
    EFCT_XPORT_CONFIG_PORT_STATUS,
    EFCT_XPORT_LINK_SPEED,
    EFCT_XPORT_IS_SUPPORTED_LINK_SPEED,
    EFCT_XPORT_LINK_STATISTICS,
    EFCT_XPORT_LINK_STAT_RESET,
    EFCT_XPORT_IS_QUIESCED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_xport_link_stats {
    pub rec: bool,
    pub gec: bool,
    pub w02of: bool,
    pub w03of: bool,
    pub w04of: bool,
    pub w05of: bool,
    pub w06of: bool,
    pub w07of: bool,
    pub w08of: bool,
    pub w09of: bool,
    pub w10of: bool,
    pub w11of: bool,
    pub w12of: bool,
    pub w13of: bool,
    pub w14of: bool,
    pub w15of: bool,
    pub w16of: bool,
    pub w17of: bool,
    pub w18of: bool,
    pub w19of: bool,
    pub w20of: bool,
    pub w21of: bool,
    pub clrc: bool,
    pub clof1: bool,
    pub link_failure_error_count: u32,
    pub loss_of_sync_error_count: u32,
    pub loss_of_signal_error_count: u32,
    pub primitive_sequence_error_count: u32,
    pub invalid_transmission_word_error_count: u32,
    pub crc_error_count: u32,
    pub primitive_sequence_event_timeout_count: u32,
    pub elastic_buffer_overrun_error_count: u32,
    pub arbitration_fc_al_timeout_count: u32,
    pub advertised_receive_bufftor_to_buffer_credit: u32,
    pub current_receive_buffer_to_buffer_credit: u32,
    pub advertised_transmit_buffer_to_buffer_credit: u32,
    pub current_transmit_buffer_to_buffer_credit: u32,
    pub received_eofa_count: u32,
    pub received_eofdti_count: u32,
    pub received_eofni_count: u32,
    pub received_soff_count: u32,
    pub received_dropped_no_aer_count: u32,
    pub received_dropped_no_available_rpi_resources_count: u32,
    pub received_dropped_no_available_xri_resources_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_xport_host_stats {
    pub cc: bool,
    pub transmit_kbyte_count: u32,
    pub receive_kbyte_count: u32,
    pub transmit_frame_count: u32,
    pub receive_frame_count: u32,
    pub transmit_sequence_count: u32,
    pub receive_sequence_count: u32,
    pub total_exchanges_originator: u32,
    pub total_exchanges_responder: u32,
    pub receive_p_bsy_count: u32,
    pub receive_f_bsy_count: u32,
    pub dropped_frames_due_to_no_rq_buffer_count: u32,
    pub empty_rq_timeout_count: u32,
    pub dropped_frames_due_to_no_xri_count: u32,
    pub empty_xri_pool_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_xport_host_statistics {
    pub done: completion,
    pub link_stats: efct_xport_link_stats,
    pub host_stats: efct_xport_host_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union efct_xport_stats_u {
    pub value: u32,
    pub stats: efct_xport_host_statistics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_xport_fcp_stats {
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub input_requests: u64,
    pub output_requests: u64,
    pub control_requests: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_xport {
    pub efct: *mut efct,
// wwpn requested by user for primary nport
    pub req_wwpn: u64,
// wwnn requested by user for primary nport
    pub req_wwnn: u64,
// Nodes
// number of allocated nodes
    pub nodes_count: u32,
// used to track how often IO pool is empty
    pub io_alloc_failed_count: core::sync::atomic::AtomicI32,
// array of pointers to nodes
    pub nodes: *mut efc_node,
// Io pool and counts
// pointer to IO pool
    pub io_pool: *mut efct_io_pool,
// lock for io_pending_list
    pub io_pending_lock: spinlock_t,
// list of IOs waiting for HW resources
// lock: xport->io_pending_lock
// link: efct_io_s->io_pending_link
//
    pub io_pending_list: list_head,
// count of totals IOS allocated
    pub io_total_alloc: core::sync::atomic::AtomicI32,
// count of totals IOS free'd
    pub io_total_free: core::sync::atomic::AtomicI32,
// count of totals IOS that were pended
    pub io_total_pending: core::sync::atomic::AtomicI32,
// count of active IOS
    pub io_active_count: core::sync::atomic::AtomicI32,
// count of pending IOS
    pub io_pending_count: core::sync::atomic::AtomicI32,
// non-zero if efct_scsi_check_pending is executing
    pub io_pending_recursing: core::sync::atomic::AtomicI32,
// Port
// requested link state
    pub configured_link_state: u32,
// Timer for Statistics
    pub stats_timer: timer_list,
    pub fc_xport_stats: efct_xport_stats_u,
    pub fcp_stats: efct_xport_fcp_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_rport_data {
    pub node: *mut efc_node,
}
