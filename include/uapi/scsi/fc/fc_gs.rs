//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/scsi/fc/fc_gs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// Fibre Channel Services - Common Transport.
// From T11.org FC-GS-2 Rev 5.3 November 1998.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_ct_hdr {
    pub /: *mut *mut __u8 ct_rev; / revision,
    pub /: *mut *mut __u8 ct_in_id[3]; / N_Port ID of original requestor,
    pub /: *mut *mut __u8 ct_fs_type; / type of fibre channel service,
    pub /: *mut *mut __u8 ct_fs_subtype; / subtype,
    pub ct_options: __u8,
    pub _ct_resvd1: __u8,
    pub /: *mut *mut __be16 ct_cmd; / command / response code,
    pub /: *mut *mut __be16 ct_mr_size; / maximum / residual size,
    pub _ct_resvd2: __u8,
    pub /: *mut *mut __u8 ct_reason; / reject reason,
    pub /: *mut *mut __u8 ct_explan; / reason code explanation,
    pub /: *mut *mut __u8 ct_vendor; / vendor unique data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ct_rev {
    FC_CT_REV = 1		/* common transport revision */
}

//
// ct_fs_type values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ct_fs_type {
    FC_FST_ALIAS =	0xf8,	/* alias service */
    FC_FST_MGMT =	0xfa,	/* management service */
    FC_FST_TIME =	0xfb,	/* time service */
    FC_FST_DIR =	0xfc,	/* directory service */
}

//
// ct_cmd: Command / response codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ct_cmd {
    FC_FS_RJT =	0x8001,	/* reject */
    FC_FS_ACC =	0x8002,	/* accept */
}

//
// FS_RJT reason codes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ct_reason {
    FC_FS_RJT_CMD =		0x01,	/* invalid command code */
    FC_FS_RJT_VER =		0x02,	/* invalid version level */
    FC_FS_RJT_LOG =		0x03,	/* logical error */
    FC_FS_RJT_IUSIZ =	0x04,	/* invalid IU size */
    FC_FS_RJT_BSY =		0x05,	/* logical busy */
    FC_FS_RJT_PROTO =	0x07,	/* protocol error */
    FC_FS_RJT_UNABL =	0x09,	/* unable to perform command request */
    FC_FS_RJT_UNSUP =	0x0b,	/* command not supported */
}

//
// FS_RJT reason code explanations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_ct_explan {
    FC_FS_EXP_NONE =	0x00,	/* no additional explanation */
    FC_FS_EXP_PID =		0x01,	/* port ID not registered */
    FC_FS_EXP_PNAM =	0x02,	/* port name not registered */
    FC_FS_EXP_NNAM =	0x03,	/* node name not registered */
    FC_FS_EXP_COS =		0x04,	/* class of service not registered */
    FC_FS_EXP_FTNR =	0x07,	/* FC-4 types not registered */
// definitions not complete
}
