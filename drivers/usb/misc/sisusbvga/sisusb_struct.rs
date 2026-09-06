//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/misc/sisusbvga/sisusb_struct.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// General structure definitions for universal mode switching modules
//
// Copyright (C) 2001-2005 by Thomas Winischhofer, Vienna, Austria
//
// If distributed as part of the Linux kernel, the following license terms
// apply:
//
// * This program is free software; you can redistribute it and/or modify
// * it under the terms of the GNU General Public License as published by
// * the Free Software Foundation; either version 2 of the named License,
// * or any later version.
//
// * This program is distributed in the hope that it will be useful,
// * but WITHOUT ANY WARRANTY; without even the implied warranty of
// * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// * GNU General Public License for more details.
//
// * You should have received a copy of the GNU General Public License
// * along with this program; if not, write to the Free Software
// * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307, USA
//
// Otherwise, the following license terms apply:
//
// * Redistribution and use in source and binary forms, with or without
// * modification, are permitted provided that the following conditions
// * are met:
// * 1) Redistributions of source code must retain the above copyright
// *    notice, this list of conditions and the following disclaimer.
// * 2) Redistributions in binary form must reproduce the above copyright
// *    notice, this list of conditions and the following disclaimer in the
// *    documentation and/or other materials provided with the distribution.
// * 3) The name of the author may not be used to endorse or promote products
// *    derived from this software without specific prior written permission.
//
// * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
// * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Author:	Thomas Winischhofer <thomas@winischhofer.net>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_St {
    pub St_ModeID: c_uchar,
    pub St_ModeFlag: c_ushort,
    pub St_StTableIndex: c_uchar,
    pub St_CRT2CRTC: c_uchar,
    pub St_ResInfo: c_uchar,
    pub VB_StTVFlickerIndex: c_uchar,
    pub VB_StTVEdgeIndex: c_uchar,
    pub VB_StTVYFilterIndex: c_uchar,
    pub St_PDC: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_StandTable {
    pub CRT_COLS: c_uchar,
    pub ROWS: c_uchar,
    pub CHAR_HEIGHT: c_uchar,
    pub CRT_LEN: c_ushort,
    pub SR: [c_uchar; 4],
    pub MISC: c_uchar,
    pub CRTC: [c_uchar; 0x19],
    pub ATTR: [c_uchar; 0x14],
    pub GRC: [c_uchar; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_StResInfo_S {
    pub HTotal: c_ushort,
    pub VTotal: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_Ext {
    pub Ext_ModeID: c_uchar,
    pub Ext_ModeFlag: c_ushort,
    pub Ext_VESAID: c_ushort,
    pub Ext_RESINFO: c_uchar,
    pub VB_ExtTVFlickerIndex: c_uchar,
    pub VB_ExtTVEdgeIndex: c_uchar,
    pub VB_ExtTVYFilterIndex: c_uchar,
    pub VB_ExtTVYFilterIndexROM661: c_uchar,
    pub REFindex: c_uchar,
    pub ROMMODEIDX661: signed char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_Ext2 {
    pub Ext_InfoFlag: c_ushort,
    pub Ext_CRT1CRTC: c_uchar,
    pub Ext_CRTVCLK: c_uchar,
    pub Ext_CRT2CRTC: c_uchar,
    pub Ext_CRT2CRTC_NS: c_uchar,
    pub ModeID: c_uchar,
    pub XRes: c_ushort,
    pub YRes: c_ushort,
    pub Ext_PDC: c_uchar,
    pub Ext_FakeCRT2CRTC: c_uchar,
    pub Ext_FakeCRT2Clk: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_CRT1Table {
    pub CR: [c_uchar; 17],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_VCLKData {
    pub SR2C: unsigned char SR2B,,
    pub CLOCK: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_ModeResInfo {
    pub HTotal: c_ushort,
    pub VTotal: c_ushort,
    pub XChar: c_uchar,
    pub YChar: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_Private {
    pub sisusb: *mut c_void,
    pub IOAddress: c_ulong,
    pub SiS_P3c4: c_ulong,
    pub SiS_P3d4: c_ulong,
    pub SiS_P3c0: c_ulong,
    pub SiS_P3ce: c_ulong,
    pub SiS_P3c2: c_ulong,
    pub SiS_P3ca: c_ulong,
    pub SiS_P3c6: c_ulong,
    pub SiS_P3c7: c_ulong,
    pub SiS_P3c8: c_ulong,
    pub SiS_P3c9: c_ulong,
    pub SiS_P3cb: c_ulong,
    pub SiS_P3cc: c_ulong,
    pub SiS_P3cd: c_ulong,
    pub SiS_P3da: c_ulong,
    pub SiS_Part1Port: c_ulong,
    pub SiS_MyCR63: c_uchar,
    pub SiS_CRT1Mode: c_ushort,
    pub SiS_ModeType: c_ushort,
    pub SiS_SetFlag: c_ushort,
    pub SiS_StandTable: *const SiS_StandTable,
    pub SiS_SModeIDTable: *const SiS_St,
    pub SiS_EModeIDTable: *const SiS_Ext,
    pub SiS_RefIndex: *const SiS_Ext2,
    pub SiS_CRT1Table: *const SiS_CRT1Table,
    pub SiS_VCLKData: *const SiS_VCLKData,
    pub SiS_ModeResInfo: *const SiS_ModeResInfo,
}
