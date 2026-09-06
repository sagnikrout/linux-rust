//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sis/init301.h
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
// Data and prototypes for init301.c
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

extern "C" {
    pub fn SiS_UnLockCRT2(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_EnableCRT2(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_GetRatePtr(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort, ModeIdIndex: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_WaitRetrace1(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_IsDualEdge(SiS_Pr: *mut SiS_Private) -> bool;
}
extern "C" {
    pub fn SiS_IsVAMode(SiS_Pr: *mut SiS_Private) -> bool;
}
extern "C" {
    pub fn SiS_SetYPbPr(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_GetResInfo(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort, ModeIdIndex: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_DisableBridge(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_SetCRT2Group(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort) -> bool;
}
extern "C" {
    pub fn SiS_SiS30xBLOn(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_SiS30xBLOff(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_SetCH700x(SiS_Pr: *mut SiS_Private, reg: c_ushort, val: c_uchar);
}
extern "C" {
    pub fn SiS_GetCH700x(SiS_Pr: *mut SiS_Private, tempax: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_SetCH701x(SiS_Pr: *mut SiS_Private, reg: c_ushort, val: c_uchar);
}
extern "C" {
    pub fn SiS_GetCH701x(SiS_Pr: *mut SiS_Private, tempax: c_ushort) -> c_ushort;
}

extern "C" {
    pub fn SiS_Chrontel701xBLOn(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_Chrontel701xBLOff(SiS_Pr: *mut SiS_Private);
}

extern "C" {
    pub fn SiS_SetChrontelGPIO(SiS_Pr: *mut SiS_Private, myvbinfo: c_ushort);
}

extern "C" {
    pub fn SiS_DDC2Delay(SiS_Pr: *mut SiS_Private, delaytime: c_uint);
}
extern "C" {
    pub fn SiS_ReadDDC1Bit(SiS_Pr: *mut SiS_Private) -> c_ushort;
}
extern "C" {
    pub fn SiS_DisplayOff(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_DisplayOn(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_SearchModeID(: *mut SiS_Private, : *mut c_ushort, : *mut c_ushort) -> bool;
}
extern "C" {
    pub fn SiS_GetModePtr(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort, ModeIdIndex: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_GetColorDepth(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort, ModeIdIndex: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_CalcCRRegisters(SiS_Pr: *mut SiS_Private, depth: c_int);
}
extern "C" {
    pub fn SiS_GetRefCRTVCLK(SiS_Pr: *mut SiS_Private, Index: c_ushort, UseWide: c_int) -> c_ushort;
}
extern "C" {
    pub fn SiS_GetRefCRT1CRTC(SiS_Pr: *mut SiS_Private, Index: c_ushort, UseWide: c_int) -> c_ushort;
}

extern "C" {
    pub fn SiS_GetFIFOThresholdB300(tempbx: c_ushort, tempcl: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_GetLatencyFactor630(SiS_Pr: *mut SiS_Private, index: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn sisfb_read_nbridge_pci_dword(SiS_Pr: *mut SiS_Private, reg: c_int) -> c_uint;
}
extern "C" {
    pub fn sisfb_read_lpc_pci_dword(SiS_Pr: *mut SiS_Private, reg: c_int) -> c_uint;
}

