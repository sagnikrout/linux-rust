//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/fsl-dpaa2-qdma/dpdmai.h
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
// Copyright 2019 NXP
// DPDMAI Version
pub const DPDMAI_VER_MAJOR: c_int = 3;
pub const DPDMAI_VER_MINOR: c_int = 3;
pub const DPDMAI_CMD_BASE_VERSION: c_int = 1;
pub const DPDMAI_CMD_ID_OFFSET: c_int = 4;
//
// Maximum number of Tx/Rx queues per DPDMAI object
//
pub const DPDMAI_MAX_QUEUE_NUM: c_int = 8;

// Command IDs

// Data Path DMA Interface API
// Contains initialization APIs and runtime control APIs for DPDMAI
//
// Maximum number of Tx/Rx priorities per DPDMAI object
//
pub const DPDMAI_PRIO_NUM: c_int = 2;
// DPDMAI queue modification options
//
// Select to modify the user's context associated with the queue
//
pub const DPDMAI_QUEUE_OPT_USER_CTX: c_uint = 0x1;
//
// Select to modify the queue's destination
//
pub const DPDMAI_QUEUE_OPT_DEST: c_uint = 0x2;
//
// struct dpdmai_cfg - Structure representing DPDMAI configuration
// @num_queues:	Number of the DMA queues
// @priorities: Priorities for the DMA hardware processing; valid priorities are
// configured with values 1-8; the entry following last valid entry
// should be configured with 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpdmai_cfg {
    pub num_queues: u8,
    pub priorities: [u8; DPDMAI_PRIO_NUM],
}

//
// struct dpdmai_attr - Structure representing DPDMAI attributes
// @id: DPDMAI object ID
// @version: DPDMAI version
// @version.major: DPDMAI major version
// @version.minor: DPDMAI minor version
// @num_of_priorities: number of priorities
// @num_of_queues: number of the DMA queues
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpdmai_attr {
    pub id: c_int,
    pub major: u16,
    pub minor: u16,
    pub version: },
    pub num_of_priorities: u8,
    pub num_of_queues: u8,
}

//
// enum dpdmai_dest - DPDMAI destination types
// @DPDMAI_DEST_NONE: Unassigned destination; The queue is set in parked mode
// and does not generate FQDAN notifications; user is expected to dequeue
// from the queue based on polling or other user-defined method
// @DPDMAI_DEST_DPIO: The queue is set in schedule mode and generates FQDAN
// notifications to the specified DPIO; user is expected to dequeue
// from the queue only after notification is received
// @DPDMAI_DEST_DPCON: The queue is set in schedule mode and does not generate
// FQDAN notifications, but is connected to the specified DPCON object;
// user is expected to dequeue from the DPCON channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpdmai_dest {
    DPDMAI_DEST_NONE = 0,
    DPDMAI_DEST_DPIO = 1,
    DPDMAI_DEST_DPCON = 2
}

//
// struct dpdmai_dest_cfg - Structure representing DPDMAI destination parameters
// @dest_type: Destination type
// @dest_id: Either DPIO ID or DPCON ID, depending on the destination type
// @priority: Priority selection within the DPIO or DPCON channel; valid values
// are 0-1 or 0-7, depending on the number of priorities in that
// channel; not relevant for 'DPDMAI_DEST_NONE' option
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpdmai_dest_cfg {
    pub dest_type: dpdmai_dest,
    pub dest_id: c_int,
    pub priority: u8,
}

//
// struct dpdmai_rx_queue_cfg - DPDMAI RX queue configuration
// @options: Flags representing the suggested modifications to the queue;
// Use any combination of 'DPDMAI_QUEUE_OPT_<X>' flags
// @user_ctx: User context value provided in the frame descriptor of each
// dequeued frame;
// valid only if 'DPDMAI_QUEUE_OPT_USER_CTX' is contained in 'options'
// @dest_cfg: Queue destination parameters;
// valid only if 'DPDMAI_QUEUE_OPT_DEST' is contained in 'options'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpdmai_rx_queue_cfg {
    pub dest_cfg: dpdmai_dest_cfg,
    pub options: u32,
    pub user_ctx: u64,
}

//
// struct dpdmai_rx_queue_attr - Structure representing attributes of Rx queues
// @user_ctx:  User context value provided in the frame descriptor of each
// dequeued frame
// @dest_cfg: Queue destination configuration
// @fqid: Virtual FQID value to be used for dequeue operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpdmai_rx_queue_attr {
    pub dest_cfg: dpdmai_dest_cfg,
    pub user_ctx: u64,
    pub fqid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpdmai_tx_queue_attr {
    pub fqid: u32,
}

extern "C" {
    pub fn dpdmai_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpdmai_destroy(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpdmai_id: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpdmai_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpdmai_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpdmai_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
