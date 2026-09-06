//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/libfc/fc_encode.h
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
// Copyright(c) 2008 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// F_CTL values for simple requests and responses.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ns_rft {
    pub /: *mut *mut fc_ns_fid fid; / port ID object,
    pub /: *mut *mut fc_ns_fts fts; / FC4-types object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ct_req {
    pub hdr: fc_ct_hdr,
    pub gid: fc_ns_gid_ft,
    pub rn: fc_ns_rn_id,
    pub rft: fc_ns_rft,
    pub rff: fc_ns_rff_id,
    pub fid: fc_ns_fid,
    pub snn: fc_ns_rsnn,
    pub spn: fc_ns_rspn,
    pub rhba: fc_fdmi_rhba,
    pub rpa: fc_fdmi_rpa,
    pub dprt: fc_fdmi_dprt,
    pub dhba: fc_fdmi_dhba,
    pub payload: },
}

//
// fc_adisc_fill() - Fill in adisc request frame
// @lport: local port.
// @fp: fc frame where payload will be placed.
//
// fc_ct_hdr_fill- fills ct header and reset ct payload
// returns pointer to ct request.
//
// fc_ct_ns_fill() - Fill in a name service request frame
// @lport: local port.
// @fc_id: FC_ID of non-destination rport for GPN_ID and similar inquiries.
// @fp: frame to contain payload.
// @op: CT opcode.
// @r_ctl: pointer to FC header R_CTL.
// @fh_type: pointer to FC-4 type.
//
// r_ctl = FC_RCTL_DD_UNSOL_CTL;
// fh_type = FC_TYPE_CT;
//
// fc_ct_ms_fill() - Fill in a mgmt service request frame
// @lport: local port.
// @fc_id: FC_ID of non-destination rport for GPN_ID and similar inquiries.
// @fp: frame to contain payload.
// @op: CT opcode.
// @r_ctl: pointer to FC header R_CTL.
// @fh_type: pointer to FC-4 type.
//
// HBA Identifier
// Number of Ports - always 1
// Port Name
// HBA Attributes
// NodeName
// Manufacturer
// SerialNumber
// Model
// Model Description
// Hardware Version
// Driver Version
// OptionROM Version
// Firmware Version
// OS Name and Version
// Max CT payload
// Node symbolic name
// Vendor specific info
// Number of ports
// Fabric name
// BIOS version
// BIOS state
// Vendor identifier
// Port Name
// Port Attributes
// FC4 types
// Supported Speed
// Current Port Speed
// Max Frame Size
// OS Device Name
// Use the sysfs device name
// Host Name
// Node name
// Port name
// Port symbolic name
// Port type
// Supported class of service
// Port Fabric name
// Port active FC-4
// Port state
// Discovered ports
// Port ID
// Port Name
// HBA Identifier
// r_ctl = FC_RCTL_DD_UNSOL_CTL;
// fh_type = FC_TYPE_CT;
//
// fc_ct_fill() - Fill in a common transport service request frame
// @lport: local port.
// @fc_id: FC_ID of non-destination rport for GPN_ID and similar inquiries.
// @fp: frame to contain payload.
// @op: CT opcode.
// @r_ctl: pointer to FC header R_CTL.
// @fh_type: pointer to FC-4 type.
//
// did = FC_FID_MGMT_SERV;
// did = FC_FID_DIR_SERV;
//
// fc_plogi_fill - Fill in plogi request frame
//
// fc_flogi_fill - Fill in a flogi request frame.
//
// fc_fdisc_fill - Fill in a fdisc request frame.
//
// fc_logo_fill - Fill in a logo request frame.
//
// fc_rtv_fill - Fill in RTV (read timeout value) request frame.
//
// fc_rec_fill - Fill in rec request frame
//
// fc_prli_fill - Fill in prli request frame
//
// fc_scr_fill - Fill in a scr request frame.
//
// fc_els_fill - Fill in an ELS  request frame
//
// r_ctl = FC_RCTL_ELS_REQ;
// fh_type = FC_TYPE_ELS;
