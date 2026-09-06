//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/efct/efct_lio.h
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
#[derive(Copy, Clone)]
pub struct efct_lio_wq_data {
    pub efct: *mut efct,
    pub ptr: *mut c_void,
    pub work: work_struct,
}

// Target private efct structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_scsi_tgt {
    pub max_sge: u32,
    pub max_sgl: u32,
//
// Variables used to send task set full. We are using a high watermark
// method to send task set full. We will reserve a fixed number of IOs
// per initiator plus a fudge factor. Once we reach this number,
// then the target will start sending task set full/busy responses.
//
    pub initiator_count: core::sync::atomic::AtomicI32,
    pub ios_in_use: core::sync::atomic::AtomicI32,
    pub io_high_watermark: core::sync::atomic::AtomicI32,
    pub watermark_hit: core::sync::atomic::AtomicI32,
    pub watermark_min: c_int,
    pub watermark_max: c_int,
    pub lio_nport: *mut efct_lio_nport,
    pub tpg: *mut efct_lio_tpg,
    pub vport_list: list_head,
// Protects vport list
    pub efct_lio_lock: spinlock_t,
    pub wwnn: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_scsi_tgt_nport {
    pub lio_nport: *mut efct_lio_nport,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_node {
    pub list_entry: list_head,
    pub ref: kref,
    pub arg): *mut *mut void (release)(struct kref,
    pub efct: *mut efct,
    pub node: *mut efc_node,
    pub session: *mut se_session,
    pub active_ios_lock: spinlock_t,
    pub active_ios: list_head,
    pub display_name: [c_char; EFC_NAME_LENGTH],
    pub port_fc_id: u32,
    pub node_fc_id: u32,
    pub vpi: u32,
    pub rpi: u32,
    pub abort_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_scsi_tgt_io {
    pub cmd: se_cmd,
    pub sense_buffer: [c_uchar; TRANSPORT_SENSE_BUFFER],
    pub ddir: dma_data_direction,
    pub task_attr: c_int,
    pub lun: u64,
    pub state: u32,
    pub tmf: u8,
    pub io_to_abort: *mut efct_io,
    pub seg_map_cnt: u32,
    pub seg_cnt: u32,
    pub cur_seg: u32,
    pub err: efct_scsi_io_status,
    pub aborting: bool,
    pub rsp_sent: bool,
    pub transferred_len: u32,
}

// Handler return codes
pub const WWN_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_lio_vport {
    pub wwpn: u64,
    pub npiv_wwpn: u64,
    pub npiv_wwnn: u64,
    pub wwpn_str: [c_uchar; WWN_NAME_LEN],
    pub vport_wwn: se_wwn,
    pub tpg: *mut efct_lio_tpg,
    pub efct: *mut efct,
    pub shost: *mut Scsi_Host,
    pub fc_vport: *mut fc_vport,
    pub enable: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_lio_nport {
    pub wwpn: u64,
    pub wwpn_str: [c_uchar; WWN_NAME_LEN],
    pub nport_wwn: se_wwn,
    pub tpg: *mut efct_lio_tpg,
    pub efct: *mut efct,
    pub enable: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_lio_tpg_attrib {
    pub generate_node_acls: u32,
    pub cache_dynamic_acls: u32,
    pub demo_mode_write_protect: u32,
    pub prod_mode_write_protect: u32,
    pub demo_mode_login_only: u32,
    pub session_deletion_wait: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_lio_tpg {
    pub tpg: se_portal_group,
    pub nport: *mut efct_lio_nport,
    pub vport: *mut efct_lio_vport,
    pub tpg_attrib: efct_lio_tpg_attrib,
    pub tpgt: c_ushort,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_lio_nacl {
    pub nport_wwnn: u64,
    pub nport_name: [c_char; WWN_NAME_LEN],
    pub session: *mut se_session,
    pub se_node_acl: se_node_acl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_lio_vport_list_t {
    pub list_entry: list_head,
    pub lio_vport: *mut efct_lio_vport,
}

extern "C" {
    pub fn efct_scsi_tgt_driver_init() -> c_int;
}
extern "C" {
    pub fn efct_scsi_tgt_driver_exit() -> c_int;
}
