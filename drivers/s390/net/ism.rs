//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/net/ism.h
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

pub const UTIL_STR_LEN: c_int = 16;
pub const ISM_ERROR: c_uint = 0xFFFF;
pub const ISM_NR_DMBS: c_int = 1920;
//
// Do not use the first word of the DMB bits to ensure 8 byte aligned access.
//
pub const ISM_DMB_WORD_OFFSET: c_int = 1;

pub const ISM_REG_SBA: c_uint = 0x1;
pub const ISM_REG_IEQ: c_uint = 0x2;
pub const ISM_READ_GID: c_uint = 0x3;
pub const ISM_ADD_VLAN_ID: c_uint = 0x4;
pub const ISM_DEL_VLAN_ID: c_uint = 0x5;
pub const ISM_SET_VLAN: c_uint = 0x6;
pub const ISM_RESET_VLAN: c_uint = 0x7;
pub const ISM_QUERY_INFO: c_uint = 0x8;
pub const ISM_QUERY_RGID: c_uint = 0x9;
pub const ISM_REG_DMB: c_uint = 0xA;
pub const ISM_UNREG_DMB: c_uint = 0xB;
pub const ISM_SIGNAL_IEQ: c_uint = 0xE;
pub const ISM_UNREG_SBA: c_uint = 0x11;
pub const ISM_UNREG_IEQ: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ism_event_type {
    ISM_EVENT_BUF = 0x00,
    ISM_EVENT_DEV = 0x01,
    ISM_EVENT_SWR = 0x02
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ism_event_code {
    ISM_BUF_DMB_UNREGISTERED = 0x04,
    ISM_BUF_USING_ISM_DEV_DISABLED = 0x08,
    ISM_BUF_OWNING_ISM_DEV_IN_ERR_STATE = 0x02,
    ISM_BUF_USING_ISM_DEV_IN_ERR_STATE = 0x03,
    ISM_BUF_VLAN_MISMATCH_WITH_OWNER = 0x05,
    ISM_BUF_VLAN_MISMATCH_WITH_USER = 0x06,
    ISM_DEV_GID_DISABLED = 0x07,
    ISM_DEV_GID_ERR_STATE = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_req_hdr {
    pub cmd: u32,
    pub 16: u16 :,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_resp_hdr {
    pub cmd: u32,
    pub ret: u16,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_reg_sba {
    pub hdr: ism_req_hdr,
    pub sba: u64,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub response: },
    pub __aligned(16): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_reg_ieq {
    pub hdr: ism_req_hdr,
    pub ieq: u64,
    pub len: u64,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub response: },
    pub __aligned(16): },
// ISM-vPCI devices provide 64 Bit GIDs
// Map them to ISM UUID GIDs like this:
// _________________________________________
// | 64 Bit ISM-vPCI GID | 00000000_00000000 |
// -----------------------------------------
// This will be interpreted as a UIID variant, that is reserved
// for NCS backward compatibility. So it will not collide with
// proper UUIDs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_read_gid {
    pub hdr: ism_req_hdr,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub gid: u64,
    pub response: },
    pub __aligned(16): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_qi {
    pub hdr: ism_req_hdr,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub version: u32,
    pub max_len: u32,
    pub ism_state: u64,
    pub my_gid: u64,
    pub sba: u64,
    pub ieq: u64,
    pub ieq_len: u32,
    pub 32: u32 :,
    pub dmbs_owned: u32,
    pub dmbs_used: u32,
    pub vlan_required: u32,
    pub vlan_nr_ids: u32,
    pub vlan_id: [u16; 64],
    pub response: },
    pub __aligned(64): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_query_rgid {
    pub hdr: ism_req_hdr,
    pub rgid: u64,
    pub vlan_valid: u32,
    pub vlan_id: u32,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub response: },
    pub __aligned(16): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_reg_dmb {
    pub hdr: ism_req_hdr,
    pub dmb: u64,
    pub dmb_len: u32,
    pub sba_idx: u32,
    pub vlan_valid: u32,
    pub vlan_id: u32,
    pub rgid: u64,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub dmb_tok: u64,
    pub response: },
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_sig_ieq {
    pub hdr: ism_req_hdr,
    pub rgid: u64,
    pub trigger_irq: u32,
    pub event_code: u32,
    pub info: u64,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub response: },
    pub __aligned(32): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_unreg_dmb {
    pub hdr: ism_req_hdr,
    pub dmb_tok: u64,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub response: },
    pub __aligned(16): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_cmd_simple {
    pub hdr: ism_req_hdr,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub response: },
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ism_set_vlan_id {
    pub hdr: ism_req_hdr,
    pub vlan_id: u64,
    pub request: },
    pub hdr: ism_resp_hdr,
    pub response: },
    pub __aligned(16): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_eq_header {
    pub idx: u64,
    pub ieq_len: u64,
    pub entry_len: u64,
    pub 64: u64 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_event {
    pub type: u32,
    pub code: u32,
    pub tok: u64,
    pub time: u64,
    pub info: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_eq {
    pub header: ism_eq_header,
    pub entry: [ism_event; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_sba {
    pub /: *mut *mut u32 s : 1; / summary bit,
    pub /: *mut *mut u32 e : 1; / event bit,
    pub 30: u32 :,
    pub 32]: u32 dmb_bits[ISM_NR_DMBS /,
    pub reserved: [u32; 3],
    pub dmbe_mask: [u16; ISM_NR_DMBS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ism_dev {
    pub /: *mut *mut spinlock_t cmd_lock; / serializes cmds,
    pub dibs: *mut dibs_dev,
    pub pdev: *mut pci_dev,
    pub sba: *mut ism_sba,
    pub sba_dma_addr: dma_addr_t,
    pub ISM_NR_DMBS): DECLARE_BITMAP(sba_bitmap,,
    pub ieq: *mut ism_eq,
    pub ieq_dma_addr: dma_addr_t,
    pub ieq_idx: c_int,
}

extern "C" {
    pub fn __zpci_store_block(_arg: data, _arg: req, _arg: dmb_req) -> return;
}
