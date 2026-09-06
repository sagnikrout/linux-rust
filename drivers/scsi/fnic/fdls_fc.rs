//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fnic/fdls_fc.h
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//
// This file contains the declarations for FC fabric services
// and target discovery
//
// Request and Response for
// 1. FLOGI
// 2. PLOGI to Fabric Controller
// 3. GPN_ID, GPN_FT
// 4. RSCN
// 5. PLOGI to Target
// 6. PRLI to Target
//

pub const FNIC_FCP_SP_RD_XRDY_DIS: c_uint = 0x00000002;
pub const FNIC_FCP_SP_TARGET: c_uint = 0x00000010;
pub const FNIC_FCP_SP_INITIATOR: c_uint = 0x00000020;
pub const FNIC_FCP_SP_CONF_CMPL: c_uint = 0x00000080;
pub const FNIC_FCP_SP_RETRY: c_uint = 0x00000100;
pub const FNIC_NVME_SP_INITIATOR: c_uint = 0x00000020;
pub const FNIC_NVME_SP_SLER: c_uint = 0x00000100;

// Little Endian

// FLOGI/PLOGI struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_flogi {
    pub fchdr: fc_frame_header,
    pub els: fc_els_flogi,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_els_acc_rsp {
    pub fchdr: fc_frame_header,
    pub acc: fc_els_ls_acc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_els_rjt_rsp {
    pub fchdr: fc_frame_header,
    pub rej: fc_els_ls_rjt,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_els_adisc {
    pub fchdr: fc_frame_header,
    pub els: fc_els_adisc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_rls_acc {
    pub fchdr: fc_frame_header,
    pub els: fc_els_rls_resp,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_abts_ba_acc {
    pub fchdr: fc_frame_header,
    pub acc: fc_ba_acc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_abts_ba_rjt {
    pub fchdr: fc_frame_header,
    pub rjt: fc_ba_rjt,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_els_prli {
    pub fchdr: fc_frame_header,
    pub els_prli: fc_els_prli,
    pub sp: fc_els_spp,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_rpn_id {
    pub fchdr: fc_frame_header,
    pub fc_std_ct_hdr: fc_ct_hdr,
    pub rpn_id: fc_ns_rn_id,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_fdmi_rhba {
    pub fchdr: fc_frame_header,
    pub fc_std_ct_hdr: fc_ct_hdr,
    pub rhba: fc_fdmi_rhba,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_fdmi_rpa {
    pub fchdr: fc_frame_header,
    pub fc_std_ct_hdr: fc_ct_hdr,
    pub rpa: fc_fdmi_rpa,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_rft_id {
    pub fchdr: fc_frame_header,
    pub fc_std_ct_hdr: fc_ct_hdr,
    pub rft_id: fc_ns_rft_id,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_rff_id {
    pub fchdr: fc_frame_header,
    pub fc_std_ct_hdr: fc_ct_hdr,
    pub rff_id: fc_ns_rff_id,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_gpn_ft {
    pub fchdr: fc_frame_header,
    pub fc_std_ct_hdr: fc_ct_hdr,
    pub gpn_ft: fc_ns_gid_ft,
    pub __packed: },
// Accept CT_IU	for	GPN_FT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_gpn_ft_rsp_iu {
    pub ctrl: u8,
    pub fcid: [u8; 3],
    pub rsvd: u32,
    pub wwpn: __be64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_rls {
    pub fchdr: fc_frame_header,
    pub els: fc_els_rls,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_scr {
    pub fchdr: fc_frame_header,
    pub scr: fc_els_scr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_rscn {
    pub fchdr: fc_frame_header,
    pub els: fc_els_rscn,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_std_logo {
    pub fchdr: fc_frame_header,
    pub els: fc_els_logo,
    pub __packed: },

