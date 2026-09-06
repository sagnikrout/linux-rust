//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_edif.h
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
// Marvell Fibre Channel HBA Driver
// Copyright (c)  2021    Marvell
//
pub const EDIF_APP_ID: c_uint = 0x73730001;
pub const EDIF_MAX_INDEX: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct edif_sa_ctl {
    pub next: list_head,
    pub del_index: u16,
    pub index: u16,
    pub slot: u16,
    pub flags: u16,

// Invalidate Index bit and mirrors QLA_SA_UPDATE_FLAGS_DELETE
    pub state: c_ulong,

    pub fcport: *mut fc_port,
    pub bsg_job: *mut bsg_job,
    pub sa_frame: qla_sa_update_frame,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enode_flags_t {
    ENODE_ACTIVE = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pur_core {
    pub enode_flags: enode_flags_t,
    pub pur_lock: spinlock_t,
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum db_flags_t {
    EDB_ACTIVE = BIT_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edif_dbell {
    pub db_flags: db_flags_t,
    pub db_lock: spinlock_t,
    pub head: list_head,
    pub dbell_bsg_job: *mut bsg_job,
    pub bsg_expire: c_ulong,
}

pub const SA_UPDATE_IOCB_TYPE: c_uint = 0x71    /* Security Association Update IOCB entry */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_update_28xx {
    pub /: *mut *mut uint8_t entry_type; / Entry type.,
    pub /: *mut *mut uint8_t entry_count; / Entry count.,
    pub /: *mut *mut uint8_t sys_define; / System Defined.,
    pub /: *mut *mut uint8_t entry_status; / Entry Status.,
    pub /: *mut *mut uint32_t handle; / IOCB System handle.,
    pub /: *mut *mut __le16 nport_handle; / in: N_PORT handle.,
    pub /: *mut *mut __le16 comp_sts; / out: completion status,
pub const CS_PORT_EDIF_UNAVAIL: c_uint = 0x28;
pub const CS_PORT_EDIF_LOGOUT: c_uint = 0x29;
pub const CS_PORT_EDIF_SUPP_NOT_RDY: c_uint = 0x64;
pub const CS_PORT_EDIF_INV_REQ: c_uint = 0x66;
    pub u: },
    pub vp_index: u8,
    pub reserved_1: u8,
    pub port_id: [u8; 3],
    pub flags: u8,

    pub /: *mut *mut uint8_t sa_key[32]; / 256 bit key,
    pub salt: __le32,
    pub spi: __le32,
    pub sa_control: u8,

pub const SA_CNTL_KEY128: c_int = 0;
    pub reserved_2: u8,
    pub 11-15: __le16 sa_index; // reserve: bit,
    pub old_sa_info: __le16,
    pub new_sa_info: __le16,
}

pub const NUM_ENTRIES: c_int = 256;
pub const PUR_GET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dinfo {
    pub nodecnt: c_int,
    pub lstate: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pur_ninfo {
    pub pur_sid: port_id_t,
    pub pur_did: port_id_t,
    pub vp_idx: u8,
    pub pur_bytes_rcvd: c_short,
    pub pur_nphdl: c_ushort,
    pub pur_rx_xchg_address: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct purexevent {
    pub pur_info: pur_ninfo,
    pub msgp: *mut c_uchar,
    pub msgp_len: u32,
}

pub const N_UNDEF: c_int = 0;
pub const N_PUREX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enode {
    pub list: list_head,
    pub dinfo: dinfo,
    pub ntype: u32,
    pub purexinfo: purexevent,
    pub u: },
}

