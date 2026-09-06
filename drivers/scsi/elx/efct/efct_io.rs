//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/efct/efct_io.h
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

pub const SCSI_CMD_BUF_LENGTH: c_int = 48;

pub const EFCT_NUM_SCSI_IOS: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_io_type {
    EFCT_IO_TYPE_IO = 0,
    EFCT_IO_TYPE_ELS,
    EFCT_IO_TYPE_CT,
    EFCT_IO_TYPE_CT_RESP,
    EFCT_IO_TYPE_BLS_RESP,
    EFCT_IO_TYPE_ABORT,

    EFCT_IO_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efct_els_state {
    EFCT_ELS_REQUEST = 0,
    EFCT_ELS_REQUEST_DELAYED,
    EFCT_ELS_REQUEST_DELAY_ABORT,
    EFCT_ELS_REQ_ABORT,
    EFCT_ELS_REQ_ABORTED,
    EFCT_ELS_ABORT_IO_COMPL,
}

//
// Scsi target IO object
// @efct:		pointer back to efct
// @instance_index:	unique instance index value
// @io:			IO display name
// @node:		pointer to node
// @list_entry:		io list entry
// @io_pending_link:	io pending list entry
// @ref:		reference counter
// @release:		release callback function
// @init_task_tag:	initiator task tag (OX_ID) for back-end and SCSI logging
// @tgt_task_tag:	target task tag (RX_ID) for back-end and SCSI logging
// @hw_tag:		HW layer unique IO id
// @tag:		unique IO identifier
// @sgl:		SGL
// @sgl_allocated:	Number of allocated SGEs
// @sgl_count:		Number of SGEs in this SGL
// @tgt_io:		backend target private IO data
// @exp_xfer_len:	expected data transfer length, based on FC header
// @hw_priv:		Declarations private to HW/SLI
// @io_type:		indicates what this struct efct_io structure is used for
// @hio:		hw io object
// @transferred:	Number of bytes transferred
// @auto_resp:		set if auto_trsp was set
// @low_latency:	set if low latency request
// @wq_steering:	selected WQ steering request
// @wq_class:		selected WQ class if steering is class
// @xfer_req:		transfer size for current request
// @scsi_tgt_cb:	target callback function
// @scsi_tgt_cb_arg:	target callback function argument
// @abort_cb:		abort callback function
// @abort_cb_arg:	abort callback function argument
// @bls_cb:		BLS callback function
// @bls_cb_arg:		BLS callback function argument
// @tmf_cmd:		TMF command being processed
// @abort_rx_id:	rx_id from the ABTS that initiated the command abort
// @cmd_tgt:		True if this is a Target command
// @send_abts:		when aborting, indicates ABTS is to be sent
// @cmd_ini:		True if this is an Initiator command
// @seq_init:		True if local node has sequence initiative
// @iparam:		iparams for hw io send call
// @hio_type:		HW IO type
// @wire_len:		wire length
// @hw_cb:		saved HW callback
// @io_to_abort:	for abort handling, pointer to IO to abort
// @rspbuf:		SCSI Response buffer
// @timeout:		Timeout value in seconds for this IO
// @cs_ctl:		CS_CTL priority for this IO
// @io_free:		Is io object in freelist
// @app_id:		application id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_io {
    pub efct: *mut efct,
    pub instance_index: u32,
    pub display_name: *const c_char,
    pub node: *mut efct_node,
    pub list_entry: list_head,
    pub io_pending_link: list_head,
    pub ref: kref,
    pub arg): *mut *mut void (release)(struct kref,
    pub init_task_tag: u32,
    pub tgt_task_tag: u32,
    pub hw_tag: u32,
    pub tag: u32,
    pub sgl: *mut efct_scsi_sgl,
    pub sgl_allocated: u32,
    pub sgl_count: u32,
    pub tgt_io: efct_scsi_tgt_io,
    pub exp_xfer_len: u32,
    pub hw_priv: *mut c_void,
    pub io_type: efct_io_type,
    pub hio: *mut efct_hw_io,
    pub transferred: usize,
    pub auto_resp: bool,
    pub low_latency: bool,
    pub wq_steering: u8,
    pub wq_class: u8,
    pub xfer_req: u64,
    pub scsi_tgt_cb: efct_scsi_io_cb_t,
    pub scsi_tgt_cb_arg: *mut c_void,
    pub abort_cb: efct_scsi_io_cb_t,
    pub abort_cb_arg: *mut c_void,
    pub bls_cb: efct_scsi_io_cb_t,
    pub bls_cb_arg: *mut c_void,
    pub tmf_cmd: efct_scsi_tmf_cmd,
    pub abort_rx_id: u16,
    pub cmd_tgt: bool,
    pub send_abts: bool,
    pub cmd_ini: bool,
    pub seq_init: bool,
    pub iparam: efct_hw_io_param_u,
    pub hio_type: efct_hw_io_type,
    pub wire_len: u64,
    pub hw_cb: *mut c_void,
    pub io_to_abort: *mut efct_io,
    pub rspbuf: efc_dma,
    pub timeout: u32,
    pub cs_ctl: u8,
    pub io_free: u8,
    pub app_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_io_cb_arg {
    pub status: c_int,
    pub ext_status: c_int,
    pub app: *mut c_void,
}
