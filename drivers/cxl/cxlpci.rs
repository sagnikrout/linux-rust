//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cxl/cxlpci.h
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
// Copyright(c) 2020 Intel Corporation. All rights reserved.

pub const CXL_MEMORY_PROGIF: c_uint = 0x10;
//
// NOTE: Currently all the functions which are enabled for CXL require their
// vectors to be in the first 16.  Use this as the default max.
//
pub const CXL_PCI_DEFAULT_MAX_VECTORS: c_int = 16;
//
// Table Access DOE, CDAT Read Entry Response
//
// Spec refs:
//
// CXL 3.1 8.1.11, Table 8-14: Read Entry Response
// CDAT Specification 1.03: 2 CDAT Data Structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdat_header {
    pub length: __le32,
    pub revision: u8,
    pub checksum: u8,
    pub reserved: [u8; 6],
    pub sequence: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdat_entry_header {
    pub type: u8,
    pub reserved: u8,
    pub length: __le16,
    pub __packed: },
//
// The DOE CDAT read response contains a CDAT read entry (either the
// CDAT header or a structure).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union cdat_data {
    pub header: cdat_header,
    pub entry: cdat_entry_header,
    pub __packed: },
// There is an additional CDAT response header of 4 bytes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdat_doe_rsp {
    pub doe_header: __le32,
    pub data: [u8; ],
    pub __packed: },
//
// CXL v3.0 6.2.3 Table 6-4
// The table indicates that if PCIe Flit Mode is set, then CXL is in 256B flits
// mode, otherwise it's 68B flits mode.
//
    pub lnksta2: u16,
    pub &lnksta2): pcie_capability_read_word(pdev, PCI_EXP_LNKSTA2,,
    pub PCI_EXP_LNKSTA2_FLIT: return lnksta2 &,
//
// Assume that the caller has already validated that @pdev has CXL
// capabilities, any RCiEP with CXL capabilities is treated as a
// Restricted CXL Device (RCD) and finds upstream port and endpoint
// registers in a Root Complex Register Block (RCRB).
//
    pub PCI_EXP_TYPE_RC_END: return pci_pcie_type(pdev) ==,
    pub cxl_dev_state: struct,
    pub port): *mut void read_cdat_data(struct cxl_port,

    pub pdev): *mut void cxl_cor_error_detected(struct pci_dev,
    pub state): pci_channel_state_t,
    pub dport): *mut void devm_cxl_dport_rch_ras_setup(struct cxl_dport,
    pub port): *mut void devm_cxl_port_ras_setup(struct cxl_port,

    pub PCI_ERS_RESULT_NONE: return,

