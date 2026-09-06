//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_logmsg.h
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
// Copyright (C) 2017-2025 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2004-2009 Emulex.  All rights reserved.
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
pub const LOG_ELS: c_uint = 0x00000001	/* ELS events */;
pub const LOG_DISCOVERY: c_uint = 0x00000002	/* Link discovery events */;
pub const LOG_MBOX: c_uint = 0x00000004	/* Mailbox events */;
pub const LOG_INIT: c_uint = 0x00000008	/* Initialization events */;
pub const LOG_LINK_EVENT: c_uint = 0x00000010	/* Link events */;
pub const LOG_NODE_VERBOSE: c_uint = 0x00000020	/* Node verbose events */;
pub const LOG_FCP: c_uint = 0x00000040	/* FCP traffic history */;
pub const LOG_NODE: c_uint = 0x00000080	/* Node table events */;
pub const LOG_TEMP: c_uint = 0x00000100	/* Temperature sensor events */;
pub const LOG_BG: c_uint = 0x00000200	/* BlockGuard events */;
pub const LOG_MISC: c_uint = 0x00000400	/* Miscellaneous events */;
pub const LOG_SLI: c_uint = 0x00000800	/* SLI events */;
pub const LOG_FCP_ERROR: c_uint = 0x00001000	/* log errors, not underruns */;
pub const LOG_LIBDFC: c_uint = 0x00002000	/* Libdfc events */;
pub const LOG_VPORT: c_uint = 0x00004000	/* NPIV events */;
pub const LOG_LDS_EVENT: c_uint = 0x00008000	/* Link Degrade Signaling events */;
pub const LOG_EVENT: c_uint = 0x00010000	/* CT,TEMP,DUMP, logging */;
pub const LOG_FIP: c_uint = 0x00020000	/* FIP events */;
pub const LOG_FCP_UNDER: c_uint = 0x00040000	/* FCP underruns errors */;
pub const LOG_SCSI_CMD: c_uint = 0x00080000	/* ALL SCSI commands */;
pub const LOG_NVME: c_uint = 0x00100000	/* NVME general events. */;
pub const LOG_NVME_DISC: c_uint = 0x00200000      /* NVME Discovery/Connect events. */;
pub const LOG_NVME_ABTS: c_uint = 0x00400000      /* NVME ABTS events. */;
pub const LOG_NVME_IOERR: c_uint = 0x00800000      /* NVME IO Error events. */;
pub const LOG_RSVD1: c_uint = 0x01000000	/* Reserved */;
pub const LOG_RSVD2: c_uint = 0x02000000	/* Reserved */;
pub const LOG_CGN_MGMT: c_uint = 0x04000000	/* Congestion Mgmt events */;
pub const LOG_ENCRYPTION: c_uint = 0x40000000      /* EDIF Encryption events. */;
pub const LOG_TRACE_EVENT: c_uint = 0x80000000	/* Dmp the DBG log on this err */;
pub const LOG_ALL_MSG: c_uint = 0x7fffffff	/* LOG all messages */;
extern "C" {
    pub fn lpfc_dmp_dbg(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_dbg_print(phba: *mut lpfc_hba, fmt: *const c_char, ...);
}
// generate message by verbose log setting or severity

