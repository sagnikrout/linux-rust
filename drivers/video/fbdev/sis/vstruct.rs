//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sis/vstruct.h
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


// $XFree86$
// $XdotOrg$
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
// Author: 	Thomas Winischhofer <thomas@winischhofer.net>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_PanelDelayTbl {
    pub timer: [c_uchar; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_LCDData {
    pub RVBHCMAX: c_ushort,
    pub RVBHCFACT: c_ushort,
    pub VGAHT: c_ushort,
    pub VGAVT: c_ushort,
    pub LCDHT: c_ushort,
    pub LCDVT: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_TVData {
    pub RVBHCMAX: c_ushort,
    pub RVBHCFACT: c_ushort,
    pub VGAHT: c_ushort,
    pub VGAVT: c_ushort,
    pub TVHDE: c_ushort,
    pub TVVDE: c_ushort,
    pub RVBHRS: c_ushort,
    pub FlickerMode: c_uchar,
    pub HALFRVBHRS: c_ushort,
    pub RVBHRS2: c_ushort,
    pub RY1COE: c_uchar,
    pub RY2COE: c_uchar,
    pub RY3COE: c_uchar,
    pub RY4COE: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_LVDSData {
    pub VGAHT: c_ushort,
    pub VGAVT: c_ushort,
    pub LCDHT: c_ushort,
    pub LCDVT: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_LVDSDes {
    pub LCDHDES: c_ushort,
    pub LCDVDES: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_LVDSCRT1Data {
    pub CR: [c_uchar; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_CHTVRegData {
    pub Reg: [c_uchar; 16],
}

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
pub struct SiS_VBMode {
    pub ModeID: c_uchar,
    pub VB_TVDelayIndex: c_uchar,
    pub VB_TVFlickerIndex: c_uchar,
    pub VB_TVPhaseIndex: c_uchar,
    pub VB_TVYFilterIndex: c_uchar,
    pub VB_LCDDelayIndex: c_uchar,
    pub _VB_LCDHIndex: c_uchar,
    pub _VB_LCDVIndex: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_StandTable_S {
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
    pub Ext_CRT1CRTC_NORM: c_uchar,
    pub Ext_CRTVCLK_NORM: c_uchar,
    pub Ext_CRT1CRTC_WIDE: c_uchar,
    pub Ext_CRTVCLK_WIDE: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_Part2PortTbl {
    pub CR: [c_uchar; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_CRT1Table {
    pub CR: [c_uchar; 17],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_MCLKData {
    pub SR28,SR29,SR2A: c_uchar,
    pub CLOCK: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_VCLKData {
    pub SR2B,SR2C: c_uchar,
    pub CLOCK: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_VBVCLKData {
    pub Part4_A,Part4_B: c_uchar,
    pub CLOCK: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_StResInfo_S {
    pub HTotal: c_ushort,
    pub VTotal: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SiS_ModeResInfo_S {
    pub HTotal: c_ushort,
    pub VTotal: c_ushort,
    pub XChar: c_uchar,
    pub YChar: c_uchar,
}

// Defines for SiS_CustomT
// Never change these for sisfb compatibility
pub const CUT_NONE: c_int = 0;
pub const CUT_FORCENONE: c_int = 1;
pub const CUT_BARCO1366: c_int = 2;
pub const CUT_BARCO1024: c_int = 3;
pub const CUT_COMPAQ1280: c_int = 4;
pub const CUT_COMPAQ12802: c_int = 5;
pub const CUT_PANEL848: c_int = 6;
pub const CUT_CLEVO1024: c_int = 7;
pub const CUT_CLEVO10242: c_int = 8;
pub const CUT_CLEVO1400: c_int = 9;
pub const CUT_CLEVO14002: c_int = 10;
pub const CUT_UNIWILL1024: c_int = 11;
pub const CUT_ASUSL3000D: c_int = 12;
pub const CUT_UNIWILL10242: c_int = 13;
pub const CUT_ACER1280: c_int = 14;
pub const CUT_COMPAL1400_1: c_int = 15;
pub const CUT_COMPAL1400_2: c_int = 16;
pub const CUT_ASUSA2H_1: c_int = 17;
pub const CUT_ASUSA2H_2: c_int = 18;
pub const CUT_UNKNOWNLCD: c_int = 19;
pub const CUT_AOP8060: c_int = 20;
pub const CUT_PANEL856: c_int = 21;
// SiS bridge

// LVDS, Chrontel
