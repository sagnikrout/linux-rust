//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pci_clp.h
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
// Call Logical Processor - Command Codes
//
pub const CLP_SLPC: c_uint = 0x0001;
pub const CLP_LIST_PCI: c_uint = 0x0002;
pub const CLP_QUERY_PCI_FN: c_uint = 0x0003;
pub const CLP_QUERY_PCI_FNGRP: c_uint = 0x0004;
pub const CLP_SET_PCI_FN: c_uint = 0x0005;
// PCI function handle list entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_fh_list_entry {
    pub device_id: u16,
    pub vendor_id: u16,
    pub 1: u32 config_state :,
    pub 31: u32 :,
    pub /: *mut *mut u32 fid; / PCI function id,
    pub /: *mut *mut u32 fh; / PCI function handle,
    pub __packed: },
pub const CLP_RC_SETPCIFN_FH: c_uint = 0x0101	/* Invalid PCI fn handle */;
pub const CLP_RC_SETPCIFN_FHOP: c_uint = 0x0102	/* Fn handle not valid for op */;
pub const CLP_RC_SETPCIFN_DMAAS: c_uint = 0x0103	/* Invalid DMA addr space */;
pub const CLP_RC_SETPCIFN_RES: c_uint = 0x0104	/* Insufficient resources */;
pub const CLP_RC_SETPCIFN_ALRDY: c_uint = 0x0105	/* Fn already in requested state */;
pub const CLP_RC_SETPCIFN_ERR: c_uint = 0x0106	/* Fn in permanent error state */;
pub const CLP_RC_SETPCIFN_RECPND: c_uint = 0x0107	/* Error recovery pending */;
pub const CLP_RC_SETPCIFN_BUSY: c_uint = 0x0108	/* Fn busy */;
pub const CLP_RC_LISTPCI_BADRT: c_uint = 0x010a	/* Resume token not recognized */;
pub const CLP_RC_QUERYPCIFG_PFGID: c_uint = 0x010b	/* Unrecognized PFGID */;
// request or response block header length
pub const LIST_PCI_HDR_LEN: c_int = 32;
// Number of function handles fitting in response block

pub const CLP_SET_ENABLE_MIO: c_int = 2;
pub const CLP_SET_DISABLE_MIO: c_int = 3;
pub const CLP_UTIL_STR_LEN: c_int = 64;
pub const CLP_PFIP_NR_SEGMENTS: c_int = 4;
// PCI function type numbers
pub const PCI_FUNC_TYPE_ISM: c_uint = 0x5	/* ISM device */;
    pub zpci_unique_uid: extern bool,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_rsp_slpc_pci {
    pub hdr: clp_rsp_hdr,
    pub reserved2: [u32; 4],
    pub lpif: [u32; 8],
    pub reserved3: [u32; 4],
    pub 1: u32 vwb :,
    pub 1: u32 :,
    pub 6: u32 mio_wb :,
    pub 24: u32 :,
    pub reserved5: [u32; 3],
    pub lpic: [u32; 8],
    pub __packed: },
// List PCI functions request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_list_pci {
    pub hdr: clp_req_hdr,
    pub resume_token: u64,
    pub reserved2: u64,
    pub __packed: },
// List PCI functions response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_rsp_list_pci {
    pub hdr: clp_rsp_hdr,
    pub resume_token: u64,
    pub reserved2: u32,
    pub max_fn: u16,
    pub 7: u8 :,
    pub 1: u8 uid_checking :,
    pub entry_size: u8,
    pub fh_list: [clp_fh_list_entry; CLP_FH_LIST_NR_ENTRIES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mio_info {
    pub 6: u32 valid :,
    pub 26: u32 :,
    pub 32: u32 :,
    pub wb: u64,
    pub wt: u64,
    pub addr: [}; PCI_STD_NUM_BARS],
    pub reserved: [u32; 6],
    pub __packed: },
// Query PCI function request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_query_pci {
    pub hdr: clp_req_hdr,
    pub /: *mut *mut u32 fh; / function handle,
    pub reserved2: u32,
    pub reserved3: u64,
    pub __packed: },
// Query PCI function response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_rsp_query_pci {
    pub hdr: clp_rsp_hdr,
    pub /: *mut *mut u16 vfn; / virtual fn number,
    pub 2: u16 :,
    pub 1: u16 tid_avail :,
    pub 1: u16 rid_avail :,
    pub 1: u16 is_physfn :,
    pub 1: u16 reserved1 :,
    pub 1: u16 mio_addr_avail :,
    pub /: *mut *mut u16 util_str_avail : 1; / utility string available?,
    pub /: *mut *mut u16 pfgid : 8; / pci function group id,
    pub /: *mut *mut u32 fid; / pci function id,
    pub bar_size: [u8; PCI_STD_NUM_BARS],
    pub pchid: u16,
    pub bar: [__le32; PCI_STD_NUM_BARS],
    pub /: *mut *mut u8 pfip[CLP_PFIP_NR_SEGMENTS]; / pci function internal path,
    pub fidparm: u8,
    pub 4: u8 reserved3 :,
    pub 4: u8 port :,
    pub fmb_len: u8,
    pub /: *mut *mut u8 pft; / pci function type,
    pub /: *mut *mut u64 sdma; / start dma as,
    pub /: *mut *mut u64 edma; / end dma as,
pub const ZPCI_RID_MASK_DEVFN: c_uint = 0x00ff;
    pub /: *mut *mut u16 rid; / BUS/DEVFN PCI address,
    pub reserved0: u32,
    pub tid: u16,
    pub reserved: [u32; 9],
    pub /: *mut *mut u32 uid; / user defined id,
    pub /: *mut *mut u8 util_str[CLP_UTIL_STR_LEN]; / utility string,
    pub reserved2: [u32; 16],
    pub mio: mio_info,
    pub __packed: },
// Query PCI function group request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_query_pci_grp {
    pub hdr: clp_req_hdr,
    pub 24: u32 reserved2 :,
    pub /: *mut *mut u32 pfgid : 8; / function group id,
    pub reserved3: u32,
    pub reserved4: u64,
    pub __packed: },
// Query PCI function group response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_rsp_query_pci_grp {
    pub hdr: clp_rsp_hdr,
    pub 4: u16 :,
    pub /: *mut *mut u16 noi : 12; / number of interrupts,
    pub version: u8,
    pub 2: u8 :,
    pub /: *mut *mut u8 rtr : 1; / Relaxed translation requirement,
    pub 3: u8 :,
    pub 1: u8 frame :,
    pub /: *mut *mut u8 refresh : 1; / TLB refresh mode,
    pub 3: u16 :,
    pub /: *mut *mut u16 maxstbl : 13; / Maximum store block size,
    pub mui: u16,
    pub /: *mut *mut u8 dtsm; / Supported DT mask,
    pub reserved3: u8,
    pub maxfaal: u16,
    pub 4: u16 :,
    pub 12: u16 dnoi :,
    pub maxcpu: u16,
    pub /: *mut *mut u64 dasm; / dma address space mask,
    pub /: *mut *mut u64 msia; / MSI address,
    pub reserved4: u64,
    pub reserved5: u64,
    pub __packed: },
// Set PCI function request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_set_pci {
    pub hdr: clp_req_hdr,
    pub /: *mut *mut u32 fh; / function handle,
    pub reserved2: u16,
    pub /: *mut *mut u8 oc; / operation controls,
    pub /: *mut *mut u8 ndas; / number of dma spaces,
    pub reserved3: u32,
    pub /: *mut *mut u32 gisa; / GISA designation,
    pub __packed: },
// Set PCI function response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_rsp_set_pci {
    pub hdr: clp_rsp_hdr,
    pub /: *mut *mut u32 fh; / function handle,
    pub reserved1: u32,
    pub reserved2: u64,
    pub mio: mio_info,
    pub __packed: },
// Combined request/response block structures used by clp insn
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_rsp_slpc_pci {
    pub request: clp_req_slpc,
    pub response: clp_rsp_slpc_pci,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_rsp_list_pci {
    pub request: clp_req_list_pci,
    pub response: clp_rsp_list_pci,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_rsp_set_pci {
    pub request: clp_req_set_pci,
    pub response: clp_rsp_set_pci,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_rsp_query_pci {
    pub request: clp_req_query_pci,
    pub response: clp_rsp_query_pci,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_rsp_query_pci_grp {
    pub request: clp_req_query_pci_grp,
    pub response: clp_rsp_query_pci_grp,
    pub __packed: },
