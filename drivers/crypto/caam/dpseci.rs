//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/dpseci.h
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
// Copyright 2017-2018 NXP
//
// Data Path SEC Interface API
// Contains initialization APIs and runtime control APIs for DPSECI
//
// General DPSECI macros
//
// Maximum number of Tx/Rx queues per DPSECI object
//
pub const DPSECI_MAX_QUEUE_NUM: c_int = 16;
//
// All queues considered; see dpseci_set_rx_queue()
//

extern "C" {
    pub fn dpseci_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
//
// Enable the Congestion Group support
//
pub const DPSECI_OPT_HAS_CG: c_uint = 0x000020;
//
// struct dpseci_cfg - Structure representing DPSECI configuration
// @options: Any combination of the following flags:
// DPSECI_OPT_HAS_CG
// @num_tx_queues: num of queues towards the SEC
// @num_rx_queues: num of queues back from the SEC
// @priorities: Priorities for the SEC hardware processing;
// each place in the array is the priority of the tx queue
// towards the SEC;
// valid priorities are configured with values 1-8;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_cfg {
    pub options: u32,
    pub num_tx_queues: u8,
    pub num_rx_queues: u8,
    pub priorities: [u8; DPSECI_MAX_QUEUE_NUM],
}

extern "C" {
    pub fn dpseci_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpseci_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpseci_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
//
// struct dpseci_attr - Structure representing DPSECI attributes
// @id: DPSECI object ID
// @num_tx_queues: number of queues towards the SEC
// @num_rx_queues: number of queues back from the SEC
// @options: any combination of the following flags:
// DPSECI_OPT_HAS_CG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_attr {
    pub id: c_int,
    pub num_tx_queues: u8,
    pub num_rx_queues: u8,
    pub options: u32,
}

//
// enum dpseci_dest - DPSECI destination types
// @DPSECI_DEST_NONE: Unassigned destination; The queue is set in parked mode
// and does not generate FQDAN notifications; user is expected to dequeue
// from the queue based on polling or other user-defined method
// @DPSECI_DEST_DPIO: The queue is set in schedule mode and generates FQDAN
// notifications to the specified DPIO; user is expected to dequeue from
// the queue only after notification is received
// @DPSECI_DEST_DPCON: The queue is set in schedule mode and does not generate
// FQDAN notifications, but is connected to the specified DPCON object;
// user is expected to dequeue from the DPCON channel
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpseci_dest {
    DPSECI_DEST_NONE = 0,
    DPSECI_DEST_DPIO,
    DPSECI_DEST_DPCON
}

//
// struct dpseci_dest_cfg - Structure representing DPSECI destination parameters
// @dest_type: Destination type
// @dest_id: Either DPIO ID or DPCON ID, depending on the destination type
// @priority: Priority selection within the DPIO or DPCON channel; valid values
// are 0-1 or 0-7, depending on the number of priorities in that channel;
// not relevant for 'DPSECI_DEST_NONE' option
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_dest_cfg {
    pub dest_type: dpseci_dest,
    pub dest_id: c_int,
    pub priority: u8,
}

//
// DPSECI queue modification options
//
// Select to modify the user's context associated with the queue
//
pub const DPSECI_QUEUE_OPT_USER_CTX: c_uint = 0x00000001;
//
// Select to modify the queue's destination
//
pub const DPSECI_QUEUE_OPT_DEST: c_uint = 0x00000002;
//
// Select to modify the queue's order preservation
//
pub const DPSECI_QUEUE_OPT_ORDER_PRESERVATION: c_uint = 0x00000004;
//
// struct dpseci_rx_queue_cfg - DPSECI RX queue configuration
// @options: Flags representing the suggested modifications to the queue;
// Use any combination of 'DPSECI_QUEUE_OPT_<X>' flags
// @order_preservation_en: order preservation configuration for the rx queue
// valid only if 'DPSECI_QUEUE_OPT_ORDER_PRESERVATION' is contained in 'options'
// @user_ctx: User context value provided in the frame descriptor of each
// dequeued frame;	valid only if 'DPSECI_QUEUE_OPT_USER_CTX' is contained
// in 'options'
// @dest_cfg: Queue destination parameters; valid only if
// 'DPSECI_QUEUE_OPT_DEST' is contained in 'options'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_rx_queue_cfg {
    pub options: u32,
    pub order_preservation_en: c_int,
    pub user_ctx: u64,
    pub dest_cfg: dpseci_dest_cfg,
}

//
// struct dpseci_rx_queue_attr - Structure representing attributes of Rx queues
// @user_ctx: User context value provided in the frame descriptor of each
// dequeued frame
// @order_preservation_en: Status of the order preservation configuration on the
// queue
// @dest_cfg: Queue destination configuration
// @fqid: Virtual FQID value to be used for dequeue operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_rx_queue_attr {
    pub user_ctx: u64,
    pub order_preservation_en: c_int,
    pub dest_cfg: dpseci_dest_cfg,
    pub fqid: u32,
}

//
// struct dpseci_tx_queue_attr - Structure representing attributes of Tx queues
// @fqid: Virtual FQID to be used for sending frames to SEC hardware
// @priority: SEC hardware processing priority for the queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_tx_queue_attr {
    pub fqid: u32,
    pub priority: u8,
}

//
// struct dpseci_sec_attr - Structure representing attributes of the SEC
// hardware accelerator
// @ip_id: ID for SEC
// @major_rev: Major revision number for SEC
// @minor_rev: Minor revision number for SEC
// @era: SEC Era
// @deco_num: The number of copies of the DECO that are implemented in this
// version of SEC
// @zuc_auth_acc_num: The number of copies of ZUCA that are implemented in this
// version of SEC
// @zuc_enc_acc_num: The number of copies of ZUCE that are implemented in this
// version of SEC
// @snow_f8_acc_num: The number of copies of the SNOW-f8 module that are
// implemented in this version of SEC
// @snow_f9_acc_num: The number of copies of the SNOW-f9 module that are
// implemented in this version of SEC
// @crc_acc_num: The number of copies of the CRC module that are implemented in
// this version of SEC
// @pk_acc_num:  The number of copies of the Public Key module that are
// implemented in this version of SEC
// @kasumi_acc_num: The number of copies of the Kasumi module that are
// implemented in this version of SEC
// @rng_acc_num: The number of copies of the Random Number Generator that are
// implemented in this version of SEC
// @md_acc_num: The number of copies of the MDHA (Hashing module) that are
// implemented in this version of SEC
// @arc4_acc_num: The number of copies of the ARC4 module that are implemented
// in this version of SEC
// @des_acc_num: The number of copies of the DES module that are implemented in
// this version of SEC
// @aes_acc_num: The number of copies of the AES module that are implemented in
// this version of SEC
// @ccha_acc_num: The number of copies of the ChaCha20 module that are
// implemented in this version of SEC.
// @ptha_acc_num: The number of copies of the Poly1305 module that are
// implemented in this version of SEC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_sec_attr {
    pub ip_id: u16,
    pub major_rev: u8,
    pub minor_rev: u8,
    pub era: u8,
    pub deco_num: u8,
    pub zuc_auth_acc_num: u8,
    pub zuc_enc_acc_num: u8,
    pub snow_f8_acc_num: u8,
    pub snow_f9_acc_num: u8,
    pub crc_acc_num: u8,
    pub pk_acc_num: u8,
    pub kasumi_acc_num: u8,
    pub rng_acc_num: u8,
    pub md_acc_num: u8,
    pub arc4_acc_num: u8,
    pub des_acc_num: u8,
    pub aes_acc_num: u8,
    pub ccha_acc_num: u8,
    pub ptha_acc_num: u8,
}

//
// enum dpseci_congestion_unit - DPSECI congestion units
// @DPSECI_CONGESTION_UNIT_BYTES: bytes units
// @DPSECI_CONGESTION_UNIT_FRAMES: frames units
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpseci_congestion_unit {
    DPSECI_CONGESTION_UNIT_BYTES = 0,
    DPSECI_CONGESTION_UNIT_FRAMES
}

//
// CSCN message is written to message_iova once entering a
// congestion state (see 'threshold_entry')
//
pub const DPSECI_CGN_MODE_WRITE_MEM_ON_ENTER: c_uint = 0x00000001;
//
// CSCN message is written to message_iova once exiting a
// congestion state (see 'threshold_exit')
//
pub const DPSECI_CGN_MODE_WRITE_MEM_ON_EXIT: c_uint = 0x00000002;
//
// CSCN write will attempt to allocate into a cache (coherent write);
// valid only if 'DPSECI_CGN_MODE_WRITE_MEM_<X>' is selected
//
pub const DPSECI_CGN_MODE_COHERENT_WRITE: c_uint = 0x00000004;
//
// if 'dpseci_dest_cfg.dest_type != DPSECI_DEST_NONE' CSCN message is sent to
// DPIO/DPCON's WQ channel once entering a congestion state
// (see 'threshold_entry')
//
pub const DPSECI_CGN_MODE_NOTIFY_DEST_ON_ENTER: c_uint = 0x00000008;
//
// if 'dpseci_dest_cfg.dest_type != DPSECI_DEST_NONE' CSCN message is sent to
// DPIO/DPCON's WQ channel once exiting a congestion state
// (see 'threshold_exit')
//
pub const DPSECI_CGN_MODE_NOTIFY_DEST_ON_EXIT: c_uint = 0x00000010;
//
// if 'dpseci_dest_cfg.dest_type != DPSECI_DEST_NONE' when the CSCN is written
// to the sw-portal's DQRR, the DQRI interrupt is asserted immediately
// (if enabled)
//
pub const DPSECI_CGN_MODE_INTR_COALESCING_DISABLED: c_uint = 0x00000020;
//
// struct dpseci_congestion_notification_cfg - congestion notification
// configuration
// @units: units type
// @threshold_entry: above this threshold we enter a congestion state.
// set it to '0' to disable it
// @threshold_exit: below this threshold we exit the congestion state.
// @message_ctx: The context that will be part of the CSCN message
// @message_iova: I/O virtual address (must be in DMA-able memory),
// must be 16B aligned;
// @dest_cfg: CSCN can be send to either DPIO or DPCON WQ channel
// @notification_mode: Mask of available options; use 'DPSECI_CGN_MODE_<X>'
// values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpseci_congestion_notification_cfg {
    pub units: dpseci_congestion_unit,
    pub threshold_entry: u32,
    pub threshold_exit: u32,
    pub message_ctx: u64,
    pub message_iova: u64,
    pub dest_cfg: dpseci_dest_cfg,
    pub notification_mode: u16,
}
