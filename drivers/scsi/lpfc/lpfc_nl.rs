//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_nl.h
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


//
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2018 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2010 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//
// Event definitions for RegisterForEvent
pub const FC_REG_LINK_EVENT: c_uint = 0x0001	/* link up / down events */;
pub const FC_REG_RSCN_EVENT: c_uint = 0x0002	/* RSCN events */;
pub const FC_REG_CT_EVENT: c_uint = 0x0004	/* CT request events */;
pub const FC_REG_DUMP_EVENT: c_uint = 0x0010	/* Dump events */;
pub const FC_REG_TEMPERATURE_EVENT: c_uint = 0x0020	/* temperature events */;
pub const FC_REG_VPORTRSCN_EVENT: c_uint = 0x0040	/* Vport RSCN events */;
pub const FC_REG_ELS_EVENT: c_uint = 0x0080	/* lpfc els events */;
pub const FC_REG_FABRIC_EVENT: c_uint = 0x0100	/* lpfc fabric events */;
pub const FC_REG_SCSI_EVENT: c_uint = 0x0200	/* lpfc scsi events */;
pub const FC_REG_BOARD_EVENT: c_uint = 0x0400	/* lpfc board events */;
pub const FC_REG_ADAPTER_EVENT: c_uint = 0x0800	/* lpfc adapter events */;

// Temperature events
pub const LPFC_CRIT_TEMP: c_uint = 0x1;
pub const LPFC_THRESHOLD_TEMP: c_uint = 0x2;
pub const LPFC_NORMAL_TEMP: c_uint = 0x3;
//
// All net link event payloads will begin with and event type
// and subcategory. The event type must come first.
// The subcategory further defines the data that follows in the rest
// of the payload. Each category will have its own unique header plus
// any additional data unique to the subcategory.
// The payload sent via the fc transport is one-way driver->application.
//
// RSCN event header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_rscn_event_header {
    pub event_type: u32,
    pub /: *mut *mut uint32_t payload_length; / RSCN data length in bytes,
    pub rscn_payload: [u32; ],
}

// els event header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_els_event_header {
    pub event_type: u32,
    pub subcategory: u32,
    pub wwpn: [u8; 8],
    pub wwnn: [u8; 8],
}

// subcategory codes for FC_REG_ELS_EVENT
pub const LPFC_EVENT_PLOGI_RCV: c_uint = 0x01;
pub const LPFC_EVENT_PRLO_RCV: c_uint = 0x02;
pub const LPFC_EVENT_ADISC_RCV: c_uint = 0x04;
pub const LPFC_EVENT_LSRJT_RCV: c_uint = 0x08;
pub const LPFC_EVENT_LOGO_RCV: c_uint = 0x10;
// special els lsrjt event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_lsrjt_event {
    pub header: lpfc_els_event_header,
    pub command: u32,
    pub reason_code: u32,
    pub explanation: u32,
}

// special els logo event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_logo_event {
    pub header: lpfc_els_event_header,
    pub logo_wwpn: [u8; 8],
}

// fabric event header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fabric_event_header {
    pub event_type: u32,
    pub subcategory: u32,
    pub wwpn: [u8; 8],
    pub wwnn: [u8; 8],
}

// subcategory codes for FC_REG_FABRIC_EVENT
pub const LPFC_EVENT_FABRIC_BUSY: c_uint = 0x01;
pub const LPFC_EVENT_PORT_BUSY: c_uint = 0x02;
pub const LPFC_EVENT_FCPRDCHKERR: c_uint = 0x04;
// special case fabric fcprdchkerr event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_fcprdchkerr_event {
    pub header: lpfc_fabric_event_header,
    pub lun: u32,
    pub opcode: u32,
    pub fcpiparam: u32,
}

// scsi event header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_scsi_event_header {
    pub event_type: u32,
    pub subcategory: u32,
    pub lun: u32,
    pub wwpn: [u8; 8],
    pub wwnn: [u8; 8],
}

// subcategory codes for FC_REG_SCSI_EVENT
pub const LPFC_EVENT_QFULL: c_uint = 0x0001;
pub const LPFC_EVENT_DEVBSY: c_uint = 0x0002;
pub const LPFC_EVENT_CHECK_COND: c_uint = 0x0004;
pub const LPFC_EVENT_LUNRESET: c_uint = 0x0008;
pub const LPFC_EVENT_TGTRESET: c_uint = 0x0010;
pub const LPFC_EVENT_BUSRESET: c_uint = 0x0020;
pub const LPFC_EVENT_VARQUEDEPTH: c_uint = 0x0040;
// special case scsi varqueuedepth event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_scsi_varqueuedepth_event {
    pub scsi_event: lpfc_scsi_event_header,
    pub oldval: u32,
    pub newval: u32,
}

// special case scsi check condition event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_scsi_check_condition_event {
    pub scsi_event: lpfc_scsi_event_header,
    pub opcode: u8,
    pub sense_key: u8,
    pub asc: u8,
    pub ascq: u8,
}

// event codes for FC_REG_BOARD_EVENT
pub const LPFC_EVENT_PORTINTERR: c_uint = 0x01;
// board event header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_board_event_header {
    pub event_type: u32,
    pub subcategory: u32,
}

// event codes for FC_REG_ADAPTER_EVENT
pub const LPFC_EVENT_ARRIVAL: c_uint = 0x01;
// adapter event header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpfc_adapter_event_header {
    pub event_type: u32,
    pub subcategory: u32,
}

// event codes for temp_event
pub const LPFC_CRIT_TEMP: c_uint = 0x1;
pub const LPFC_THRESHOLD_TEMP: c_uint = 0x2;
pub const LPFC_NORMAL_TEMP: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct temp_event {
    pub event_type: u32,
    pub event_code: u32,
    pub data: u32,
}
