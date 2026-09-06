//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/video/sisfb.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// sisfb.h - definitions for the SiS framebuffer driver
//
// Copyright (C) 2001-2005 by Thomas Winischhofer, Vienna, Austria.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the named License,
// or any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307, USA
//

//
// PUBLIC
//
// vbflags, public (others in sis.h)
pub const CRT2_DEFAULT: c_uint = 0x00000001;
pub const CRT2_LCD: c_uint = 0x00000002;
pub const CRT2_TV: c_uint = 0x00000004;
pub const CRT2_VGA: c_uint = 0x00000008;
pub const TV_NTSC: c_uint = 0x00000010;
pub const TV_PAL: c_uint = 0x00000020;
pub const TV_HIVISION: c_uint = 0x00000040;
pub const TV_YPBPR: c_uint = 0x00000080;
pub const TV_AVIDEO: c_uint = 0x00000100;
pub const TV_SVIDEO: c_uint = 0x00000200;
pub const TV_SCART: c_uint = 0x00000400;
pub const TV_PALM: c_uint = 0x00001000;
pub const TV_PALN: c_uint = 0x00002000;
pub const TV_NTSCJ: c_uint = 0x00001000;
pub const TV_CHSCART: c_uint = 0x00008000;
pub const TV_CHYPBPR525I: c_uint = 0x00010000;
pub const CRT1_VGA: c_uint = 0x00000000;
pub const CRT1_LCDA: c_uint = 0x00020000;
pub const VGA2_CONNECTED: c_uint = 0x00040000;
pub const VB_DISPTYPE_CRT1: c_uint = 0x00080000	/* CRT1 connected and used */;
pub const VB_SINGLE_MODE: c_uint = 0x20000000	/* CRT1 or CRT2; determined by DISPTYPE_CRTx */;
pub const VB_MIRROR_MODE: c_uint = 0x40000000	/* CRT1 + CRT2 identical (mirror mode) */;
pub const VB_DUALVIEW_MODE: c_uint = 0x80000000	/* CRT1 + CRT2 independent (dual head mode) */;
// Aliases:

// Only if TV_YPBPR is set:

// Structure argument for SISFB_GET_INFO ioctl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisfb_info {
    pub /: *mut *mut __u32 sisfb_id; / for identifying sisfb,

pub const SISFB_ID: c_uint = 0x53495346    /* Identify myself with 'SISF' */;

    pub /: *mut *mut __u32 chip_id; / PCI-ID of detected chip,
    pub /: *mut *mut __u32 memory; / total video memory in KB,
    pub /: *mut *mut __u32 heapstart; / heap start offset in KB,
    pub /: *mut *mut __u8 fbvidmode; / current sisfb mode,
    pub sisfb_version: __u8,
    pub sisfb_revision: __u8,
    pub sisfb_patchlevel: __u8,
    pub /: *mut *mut __u8 sisfb_caps; / sisfb capabilities,
    pub /: *mut *mut __u32 sisfb_tqlen; / turbo queue length (in KB),
    pub /: *mut *mut __u32 sisfb_pcibus; / The card's PCI ID,
    pub sisfb_pcislot: __u32,
    pub sisfb_pcifunc: __u32,
    pub /: *mut *mut __u8 sisfb_lcdpdc; / PanelDelayCompensation,
    pub /: *mut *mut __u8 sisfb_lcda; / Detected status of LCDA for low res/text modes,
    pub sisfb_vbflags: __u32,
    pub sisfb_currentvbflags: __u32,
    pub sisfb_scalelcd: __u32,
    pub sisfb_specialtiming: __u32,
    pub sisfb_haveemi: __u8,
    pub sisfb_emi30,sisfb_emi31,sisfb_emi32,sisfb_emi33: __u8,
    pub sisfb_haveemilcd: __u8,
    pub /: *mut *mut __u8 sisfb_lcdpdca; / PanelDelayCompensation for LCD-via-CRT1,
    pub /: *mut *mut __u16 sisfb_tvxpos, sisfb_tvypos; / Warning: Values + 32 !,
    pub /: *mut *mut __u32 sisfb_heapsize; / heap size (in KB),
    pub /: *mut *mut __u32 sisfb_videooffset; / Offset of viewport in video memory (in bytes),
    pub /: *mut *mut __u32 sisfb_curfstn; / currently running FSTN/DSTN mode,
    pub sisfb_curdstn: __u32,
    pub /: *mut *mut __u16 sisfb_pci_vendor; / PCI vendor (SiS or XGI),
    pub /: *mut *mut __u32 sisfb_vbflags2; / ivideo->vbflags2,
    pub /: *mut *mut __u8 sisfb_can_post; / sisfb can POST this card,
    pub /: *mut *mut __u8 sisfb_card_posted; / card is POSTED,
    pub /: *mut *mut __u8 sisfb_was_boot_device; / This card was the boot video device (ie is primary),
    pub /: *mut *mut __u8 reserved[183]; / for future use,
}

pub const SISFB_CMD_GETVBFLAGS: c_uint = 0x55AA0001	/* no arg; result[1] = vbflags */;
pub const SISFB_CMD_SWITCHCRT1: c_uint = 0x55AA0010	/* arg[0]: 99 = query, 0 = off, 1 = on */;
// more to come
pub const SISFB_CMD_ERR_OK: c_uint = 0x80000000	/* command succeeded */;
pub const SISFB_CMD_ERR_LOCKED: c_uint = 0x80000001	/* sisfb is locked */;
pub const SISFB_CMD_ERR_EARLY: c_uint = 0x80000002	/* request before sisfb took over gfx system */;
pub const SISFB_CMD_ERR_NOVB: c_uint = 0x80000003	/* No video bridge */;
pub const SISFB_CMD_ERR_NOCRT2: c_uint = 0x80000004	/* can't change CRT1 status, CRT2 disabled */;
// more to come
pub const SISFB_CMD_ERR_UNKNOWN: c_uint = 0x8000ffff	/* Unknown command */;
pub const SISFB_CMD_ERR_OTHER: c_uint = 0x80010000	/* Other error */;
// Argument for SISFB_CMD ioctl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisfb_cmd {
    pub sisfb_cmd: __u32,
    pub sisfb_arg: [__u32; 16],
    pub sisfb_result: [__u32; 4],
}

// Additional IOCTLs for communication sisfb <> X driver
// If changing this, vgatypes.h must also be changed (for X driver)
// ioctl for identifying and giving some info (esp. memory heap start)

// ioctrl to get current vertical retrace status

// ioctl to enable/disable panning auto-maximize (like nomax parameter)

// ioctls to relocate TV output (x=D[31:16], y=D[15:0], + 32)

// ioctl for internal sisfb commands (sisfbctrl)

// ioctl for locking sisfb (no register access during lock)
// As of now, only used to avoid register access during
// the ioctls listed above.
//

// ioctls 0xF3 up to 0x3F reserved for sisfb
//
// The following are deprecated and should not be used anymore:
//
// ioctl for identifying and giving some info (esp. memory heap start)

// ioctrl to get current vertical retrace status

// ioctl to enable/disable panning auto-maximize (like nomax parameter)

//
// End of deprecated ioctl numbers
//
// For fb memory manager (FBIO_ALLOC, FBIO_FREE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sis_memreq {
    pub offset: __u32,
    pub size: __u32,
}

//
// PRIVATE
// (for IN-KERNEL usage only)
//
