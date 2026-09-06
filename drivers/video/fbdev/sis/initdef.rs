//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sis/initdef.h
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
// Global definitions for init.c and init301.c
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

// SiS_VBType
pub const VB_SIS301: c_uint = 0x0001;
pub const VB_SIS301B: c_uint = 0x0002;
pub const VB_SIS302B: c_uint = 0x0004;
pub const VB_SIS301LV: c_uint = 0x0008;
pub const VB_SIS302LV: c_uint = 0x0010;
pub const VB_SIS302ELV: c_uint = 0x0020;
pub const VB_SIS301C: c_uint = 0x0040;
pub const VB_SIS307T: c_uint = 0x0080;
pub const VB_SIS307LV: c_uint = 0x0100;
pub const VB_UMC: c_uint = 0x4000;
pub const VB_NoLCD: c_uint = 0x8000;

// VBInfo
pub const SetSimuScanMode: c_uint = 0x0001   /* CR 30 */;
pub const SwitchCRT2: c_uint = 0x0002;
pub const SetCRT2ToAVIDEO: c_uint = 0x0004;
pub const SetCRT2ToSVIDEO: c_uint = 0x0008;
pub const SetCRT2ToSCART: c_uint = 0x0010;
pub const SetCRT2ToLCD: c_uint = 0x0020;
pub const SetCRT2ToRAMDAC: c_uint = 0x0040;
pub const SetCRT2ToHiVision: c_uint = 0x0080   		/* for SiS bridge */;

pub const SetNTSCTV: c_uint = 0x0000   /* CR 31 */;
pub const SetPALTV: c_uint = 0x0100   		/* Deprecated here, now in TVMode */;
pub const SetInSlaveMode: c_uint = 0x0200;
pub const SetNotSimuMode: c_uint = 0x0400;

pub const SetDispDevSwitch: c_uint = 0x0800;
pub const SetCRT2ToYPbPr525750: c_uint = 0x0800;
pub const LoadDACFlag: c_uint = 0x1000;
pub const DisableCRT2Display: c_uint = 0x2000;
pub const DriverMode: c_uint = 0x4000;
pub const HotKeySwitch: c_uint = 0x8000;
pub const SetCRT2ToLCDA: c_uint = 0x8000;
// v-- Needs change in sis_vga.c if changed (GPIO) --v

// SiS_ModeType
pub const ModeText: c_uint = 0x00;
pub const ModeCGA: c_uint = 0x01;
pub const ModeEGA: c_uint = 0x02;
pub const ModeVGA: c_uint = 0x03;
pub const Mode15Bpp: c_uint = 0x04;
pub const Mode16Bpp: c_uint = 0x05;
pub const Mode24Bpp: c_uint = 0x06;
pub const Mode32Bpp: c_uint = 0x07;
pub const ModeTypeMask: c_uint = 0x07;
pub const IsTextMode: c_uint = 0x07;
pub const DACInfoFlag: c_uint = 0x0018;
pub const MemoryInfoFlag: c_uint = 0x01E0;
pub const MemorySizeShift: c_int = 5;
// modeflag
pub const Charx8Dot: c_uint = 0x0200;
pub const LineCompareOff: c_uint = 0x0400;
pub const CRT2Mode: c_uint = 0x0800;
pub const HalfDCLK: c_uint = 0x1000;
pub const NoSupportSimuTV: c_uint = 0x2000;
pub const NoSupportLCDScale: c_uint = 0x4000 /* SiS bridge: No scaling possible (no matter what panel) */;
pub const DoubleScanMode: c_uint = 0x8000;
// Infoflag
pub const SupportTV: c_uint = 0x0008;
pub const SupportTV1024: c_uint = 0x0800;
pub const SupportCHTV: c_uint = 0x0800;
pub const Support64048060Hz: c_uint = 0x0800  /* Special for 640x480 LCD */;
pub const SupportHiVision: c_uint = 0x0010;
pub const SupportYPbPr750p: c_uint = 0x1000;
pub const SupportLCD: c_uint = 0x0020;
pub const SupportRAMDAC2: c_uint = 0x0040	/* All           (<= 100Mhz) */;
pub const SupportRAMDAC2_135: c_uint = 0x0100  /* All except DH (<= 135Mhz) */;
pub const SupportRAMDAC2_162: c_uint = 0x0200  /* B, C          (<= 162Mhz) */;
pub const SupportRAMDAC2_202: c_uint = 0x0400  /* C             (<= 202Mhz) */;
pub const InterlaceMode: c_uint = 0x0080;
pub const SyncPP: c_uint = 0x0000;
pub const HaveWideTiming: c_uint = 0x2000	/* Have specific wide- and non-wide timing */;
pub const SyncPN: c_uint = 0x4000;
pub const SyncNP: c_uint = 0x8000;
pub const SyncNN: c_uint = 0xc000;
// SetFlag
pub const ProgrammingCRT2: c_uint = 0x0001;
pub const LowModeTests: c_uint = 0x0002;
// #define TVSimuMode           0x0002 - deprecated
// #define RPLLDIV2XO           0x0004 - deprecated
pub const LCDVESATiming: c_uint = 0x0008;
pub const EnableLVDSDDA: c_uint = 0x0010;
pub const SetDispDevSwitchFlag: c_uint = 0x0020;
pub const CheckWinDos: c_uint = 0x0040;
pub const SetDOSMode: c_uint = 0x0080;
// TVMode flag
pub const TVSetPAL: c_uint = 0x0001;
pub const TVSetNTSCJ: c_uint = 0x0002;
pub const TVSetPALM: c_uint = 0x0004;
pub const TVSetPALN: c_uint = 0x0008;
pub const TVSetCHOverScan: c_uint = 0x0010;
pub const TVSetYPbPr525i: c_uint = 0x0020 /* new 0x10 */;
pub const TVSetYPbPr525p: c_uint = 0x0040 /* new 0x20 */;
pub const TVSetYPbPr750p: c_uint = 0x0080 /* new 0x40 */;
pub const TVSetHiVision: c_uint = 0x0100 /* new 0x80; = 1080i, software-wise identical */;
pub const TVSetTVSimuMode: c_uint = 0x0200 /* new 0x200, prev. 0x800 */;
pub const TVRPLLDIV2XO: c_uint = 0x0400 /* prev 0x1000 */;
pub const TVSetNTSC1024: c_uint = 0x0800 /* new 0x100, prev. 0x2000 */;
pub const TVSet525p1024: c_uint = 0x1000 /* TW */;
pub const TVAspect43: c_uint = 0x2000;
pub const TVAspect169: c_uint = 0x4000;
pub const TVAspect43LB: c_uint = 0x8000;
// YPbPr flag (>=315, <661; converted to TVMode)
pub const YPbPr525p: c_uint = 0x0001;
pub const YPbPr750p: c_uint = 0x0002;
pub const YPbPr525i: c_uint = 0x0004;
pub const YPbPrHiVision: c_uint = 0x0008;

// SysFlags (to identify special versions)
pub const SF_Is651: c_uint = 0x0001;
pub const SF_IsM650: c_uint = 0x0002;
pub const SF_Is652: c_uint = 0x0004;
pub const SF_IsM652: c_uint = 0x0008;
pub const SF_IsM653: c_uint = 0x0010;
pub const SF_IsM661: c_uint = 0x0020;
pub const SF_IsM741: c_uint = 0x0040;
pub const SF_IsM760: c_uint = 0x0080;
pub const SF_760UMA: c_uint = 0x4000  /* 76x: We have UMA */;
pub const SF_760LFB: c_uint = 0x8000  /* 76x: We have LFB */;
// CR32 (Newer 630, and 315 series)
//
// CR35 (300 series only)
pub const TVOverScan: c_uint = 0x10;
pub const TVOverScanShift: c_int = 4;
// CR35 (661 series only)
//
// CR37
//
// CR37: LCDInfo
pub const LCDRGB18Bit: c_uint = 0x0001;
pub const LCDNonExpanding: c_uint = 0x0010;
pub const LCDSync: c_uint = 0x0020;
pub const LCDPass11: c_uint = 0x0100   /* 0: center screen, 1: Pass 1:1 data */;
pub const LCDDualLink: c_uint = 0x0200;

pub const LCDNonExpandingShift: c_int = 4;

pub const LCDSyncBit: c_uint = 0x00e0;
pub const LCDSyncShift: c_int = 6;
// CR38 (315 series)
pub const EnableDualEdge: c_uint = 0x01;
pub const SetToLCDA: c_uint = 0x02   /* LCD channel A (301C/302B/30x(E)LV and 650+LVDS only) */;
pub const EnableCHScart: c_uint = 0x04   /* Scart on Ch7019 (unofficial definition - TW) */;
pub const EnableCHYPbPr: c_uint = 0x08   /* YPbPr on Ch7019 (480i HDTV); only on 650/Ch7019 systems */;
pub const EnableSiSYPbPr: c_uint = 0x08   /* Enable YPbPr mode (30xLV/301C only) */;
pub const EnableYPbPr525i: c_uint = 0x00   /* Enable 525i YPbPr mode (30xLV/301C only) (mask 0x30) */;
pub const EnableYPbPr525p: c_uint = 0x10   /* Enable 525p YPbPr mode (30xLV/301C only) (mask 0x30) */;
pub const EnableYPbPr750p: c_uint = 0x20   /* Enable 750p YPbPr mode (30xLV/301C only) (mask 0x30) */;
pub const EnableYPbPr1080i: c_uint = 0x30   /* Enable 1080i YPbPr mode (30xLV/301C only) (mask 0x30) */;
pub const EnablePALM: c_uint = 0x40   /* 1 = Set PALM */;
pub const EnablePALN: c_uint = 0x80   /* 1 = Set PALN */;

// CR38 (661 and later)
//
pub const EnablePALMN: c_uint = 0x40   /* Romflag: 1 = Allow PALM/PALN */;
// CR39 (650 only)
pub const LCDPass1_1: c_uint = 0x01   /* 0: center screen, 1: pass 1:1 data output  */;
pub const Enable302LV_DualLink: c_uint = 0x04   /* 302LV only; enable dual link */;
// CR39 (661 and later)
//
// CR3B (651+301C)
//
// CR79 (315/330 series only; not 661 and later)
//
// CR7C - 661 and later
//
// CR7E - 661 and later
//
// LCDResInfo
pub const Panel300_800x600: c_uint = 0x01	/* CR36 */;
pub const Panel300_1024x768: c_uint = 0x02;
pub const Panel300_1280x1024: c_uint = 0x03;
pub const Panel300_1280x960: c_uint = 0x04;
pub const Panel300_640x480: c_uint = 0x05;
pub const Panel300_1024x600: c_uint = 0x06;
pub const Panel300_1152x768: c_uint = 0x07;
pub const Panel300_1280x768: c_uint = 0x0a;
pub const Panel300_Custom: c_uint = 0x0f;
pub const Panel300_Barco1366: c_uint = 0x10;
pub const Panel310_800x600: c_uint = 0x01;
pub const Panel310_1024x768: c_uint = 0x02;
pub const Panel310_1280x1024: c_uint = 0x03;
pub const Panel310_640x480: c_uint = 0x04;
pub const Panel310_1024x600: c_uint = 0x05;
pub const Panel310_1152x864: c_uint = 0x06;
pub const Panel310_1280x960: c_uint = 0x07;
pub const Panel310_1152x768: c_uint = 0x08	/* LVDS only */;
pub const Panel310_1400x1050: c_uint = 0x09;
pub const Panel310_1280x768: c_uint = 0x0a;
pub const Panel310_1600x1200: c_uint = 0x0b;
pub const Panel310_320x240_2: c_uint = 0x0c    /* xSTN */;
pub const Panel310_320x240_3: c_uint = 0x0d    /* xSTN */;
pub const Panel310_320x240_1: c_uint = 0x0e    /* xSTN - This is fake, can be any */;
pub const Panel310_Custom: c_uint = 0x0f;
pub const Panel661_800x600: c_uint = 0x01;
pub const Panel661_1024x768: c_uint = 0x02;
pub const Panel661_1280x1024: c_uint = 0x03;
pub const Panel661_640x480: c_uint = 0x04;
pub const Panel661_1024x600: c_uint = 0x05;
pub const Panel661_1152x864: c_uint = 0x06;
pub const Panel661_1280x960: c_uint = 0x07;
pub const Panel661_1280x854: c_uint = 0x08;
pub const Panel661_1400x1050: c_uint = 0x09;
pub const Panel661_1280x768: c_uint = 0x0a;
pub const Panel661_1600x1200: c_uint = 0x0b;
pub const Panel661_1280x800: c_uint = 0x0c;
pub const Panel661_1680x1050: c_uint = 0x0d;
pub const Panel661_1280x720: c_uint = 0x0e;
pub const Panel661_Custom: c_uint = 0x0f;
pub const Panel_800x600: c_uint = 0x01	/* Unified values */;
pub const Panel_1024x768: c_uint = 0x02    /* MUST match BIOS values from 0-e */;
pub const Panel_1280x1024: c_uint = 0x03;
pub const Panel_640x480: c_uint = 0x04;
pub const Panel_1024x600: c_uint = 0x05;
pub const Panel_1152x864: c_uint = 0x06;
pub const Panel_1280x960: c_uint = 0x07;
pub const Panel_1152x768: c_uint = 0x08	/* LVDS only */;
pub const Panel_1400x1050: c_uint = 0x09;
pub const Panel_1280x768: c_uint = 0x0a    /* 30xB/C and LVDS only (BIOS: all) */;
pub const Panel_1600x1200: c_uint = 0x0b;
pub const Panel_1280x800: c_uint = 0x0c    /* 661etc (TMDS) */;
pub const Panel_1680x1050: c_uint = 0x0d    /* 661etc  */;
pub const Panel_1280x720: c_uint = 0x0e    /* 661etc  */;
pub const Panel_Custom: c_uint = 0x0f	/* MUST BE 0x0f (for DVI DDC detection) */;
pub const Panel_320x240_1: c_uint = 0x10    /* SiS 550 xSTN */;
pub const Panel_Barco1366: c_uint = 0x11;
pub const Panel_848x480: c_uint = 0x12;
pub const Panel_320x240_2: c_uint = 0x13    /* SiS 550 xSTN */;
pub const Panel_320x240_3: c_uint = 0x14    /* SiS 550 xSTN */;
pub const Panel_1280x768_2: c_uint = 0x15	/* 30xLV */;
pub const Panel_1280x768_3: c_uint = 0x16    /* (unused) */;
pub const Panel_1280x800_2: c_uint = 0x17    /* 30xLV */;
pub const Panel_856x480: c_uint = 0x18;
pub const Panel_1280x854: c_uint = 0x19	/* 661etc */;
// Index in ModeResInfo table
pub const SIS_RI_320x200: c_int = 0;
pub const SIS_RI_320x240: c_int = 1;
pub const SIS_RI_320x400: c_int = 2;
pub const SIS_RI_400x300: c_int = 3;
pub const SIS_RI_512x384: c_int = 4;
pub const SIS_RI_640x400: c_int = 5;
pub const SIS_RI_640x480: c_int = 6;
pub const SIS_RI_800x600: c_int = 7;
pub const SIS_RI_1024x768: c_int = 8;
pub const SIS_RI_1280x1024: c_int = 9;
pub const SIS_RI_1600x1200: c_int = 10;
pub const SIS_RI_1920x1440: c_int = 11;
pub const SIS_RI_2048x1536: c_int = 12;
pub const SIS_RI_720x480: c_int = 13;
pub const SIS_RI_720x576: c_int = 14;
pub const SIS_RI_1280x960: c_int = 15;
pub const SIS_RI_800x480: c_int = 16;
pub const SIS_RI_1024x576: c_int = 17;
pub const SIS_RI_1280x720: c_int = 18;
pub const SIS_RI_856x480: c_int = 19;
pub const SIS_RI_1280x768: c_int = 20;
pub const SIS_RI_1400x1050: c_int = 21;

pub const SIS_RI_848x480: c_int = 23;
pub const SIS_RI_1360x768: c_int = 24;
pub const SIS_RI_1024x600: c_int = 25;
pub const SIS_RI_1152x768: c_int = 26;
pub const SIS_RI_768x576: c_int = 27;
pub const SIS_RI_1360x1024: c_int = 28;
pub const SIS_RI_1680x1050: c_int = 29;
pub const SIS_RI_1280x800: c_int = 30;
pub const SIS_RI_1920x1080: c_int = 31;
pub const SIS_RI_960x540: c_int = 32;
pub const SIS_RI_960x600: c_int = 33;
pub const SIS_RI_1280x854: c_int = 34;
// CR5F
pub const IsM650: c_uint = 0x80;
// Timing data
pub const NTSCHT: c_int = 1716;
pub const NTSC2HT: c_int = 1920;
pub const NTSCVT: c_int = 525;
pub const PALHT: c_int = 1728;
pub const PALVT: c_int = 625;
pub const StHiTVHT: c_int = 892;
pub const StHiTVVT: c_int = 1126;
pub const StHiTextTVHT: c_int = 1000;
pub const StHiTextTVVT: c_int = 1126;
pub const ExtHiTVHT: c_int = 2100;
pub const ExtHiTVVT: c_int = 1125;
// Indices in (VB)VCLKData tables
pub const VCLK28: c_uint = 0x00   /* Index in VCLKData table (300 and 315) */;
pub const VCLK40: c_uint = 0x04   /* Index in VCLKData table (300 and 315) */;
pub const VCLK65_300: c_uint = 0x09   /* Index in VCLKData table (300) */;
pub const VCLK108_2_300: c_uint = 0x14   /* Index in VCLKData table (300) */;
pub const VCLK81_300: c_uint = 0x3f   /* Index in VCLKData table (300) */;
pub const VCLK108_3_300: c_uint = 0x42   /* Index in VCLKData table (300) */;
pub const VCLK100_300: c_uint = 0x43   /* Index in VCLKData table (300) */;
pub const VCLK34_300: c_uint = 0x3d   /* Index in VCLKData table (300) */;
pub const VCLK_CUSTOM_300: c_uint = 0x47;
pub const VCLK65_315: c_uint = 0x0b   /* Indices in (VB)VCLKData table (315) */;
pub const VCLK108_2_315: c_uint = 0x19;
pub const VCLK81_315: c_uint = 0x5b;
pub const VCLK162_315: c_uint = 0x5e;
pub const VCLK108_3_315: c_uint = 0x45;
pub const VCLK100_315: c_uint = 0x46;
pub const VCLK34_315: c_uint = 0x55;
pub const VCLK68_315: c_uint = 0x0d;
pub const VCLK_1280x800_315_2: c_uint = 0x5c;
pub const VCLK121_315: c_uint = 0x5d;
pub const VCLK130_315: c_uint = 0x72;
pub const VCLK_1280x720: c_uint = 0x5f;
pub const VCLK_1280x768_2: c_uint = 0x60;
pub const VCLK_1280x768_3: c_uint = 0x61   /* (unused?) */;
pub const VCLK_CUSTOM_315: c_uint = 0x62;
pub const VCLK_1280x720_2: c_uint = 0x63;
pub const VCLK_720x480: c_uint = 0x67;
pub const VCLK_720x576: c_uint = 0x68;
pub const VCLK_768x576: c_uint = 0x68;
pub const VCLK_848x480: c_uint = 0x65;
pub const VCLK_856x480: c_uint = 0x66;
pub const VCLK_800x480: c_uint = 0x65;
pub const VCLK_1024x576: c_uint = 0x51;
pub const VCLK_1152x864: c_uint = 0x64;
pub const VCLK_1360x768: c_uint = 0x58;
pub const VCLK_1280x800_315: c_uint = 0x6c;
pub const VCLK_1280x854: c_uint = 0x76;
pub const TVCLKBASE_300: c_uint = 0x21   /* Indices on TV clocks in VCLKData table (300) */;
pub const TVCLKBASE_315: c_uint = 0x3a   /* Indices on TV clocks in (VB)VCLKData table (315) */;
pub const TVVCLKDIV2: c_uint = 0x00   /* Index relative to TVCLKBASE */;
pub const TVVCLK: c_uint = 0x01   /* Index relative to TVCLKBASE */;
pub const HiTVVCLKDIV2: c_uint = 0x02   /* Index relative to TVCLKBASE */;
pub const HiTVVCLK: c_uint = 0x03   /* Index relative to TVCLKBASE */;
pub const HiTVSimuVCLK: c_uint = 0x04   /* Index relative to TVCLKBASE */;
pub const HiTVTextVCLK: c_uint = 0x05   /* Index relative to TVCLKBASE */;
pub const YPbPr750pVCLK: c_uint = 0x25   /* Index relative to TVCLKBASE; was 0x0f NOT relative */;
// ------------------------------
pub const SetSCARTOutput: c_uint = 0x01;
pub const HotPlugFunction: c_uint = 0x08;
pub const StStructSize: c_uint = 0x06;
pub const SIS_VIDEO_CAPTURE: c_uint = 0x00 - 0x30;
pub const SIS_VIDEO_PLAYBACK: c_uint = 0x02 - 0x30;
pub const SIS_CRT2_PORT_04: c_uint = 0x04 - 0x30;
pub const SIS_CRT2_PORT_10: c_uint = 0x10 - 0x30;
pub const SIS_CRT2_PORT_12: c_uint = 0x12 - 0x30;
pub const SIS_CRT2_PORT_14: c_uint = 0x14 - 0x30;
pub const ADR_CRT2PtrData: c_uint = 0x20E;
pub const offset_Zurac: c_uint = 0x210   /* TW: Trumpion Zurac data pointer */;
pub const ADR_LVDSDesPtrData: c_uint = 0x212;
pub const ADR_LVDSCRT1DataPtr: c_uint = 0x214;
pub const ADR_CHTVVCLKPtr: c_uint = 0x216;
pub const ADR_CHTVRegDataPtr: c_uint = 0x218;
pub const LCDDataLen: c_int = 8;
pub const HiTVDataLen: c_int = 12;
pub const TVDataLen: c_int = 16;
pub const LVDSDataLen: c_int = 6;
pub const LVDSDesDataLen: c_int = 3;
pub const ActiveNonExpanding: c_uint = 0x40;
pub const ActiveNonExpandingShift: c_int = 6;
pub const ActivePAL: c_uint = 0x20;
pub const ActivePALShift: c_int = 5;
pub const ModeSwitchStatus: c_uint = 0x0F;
pub const SoftTVType: c_uint = 0x40;
pub const SoftSettingAddr: c_uint = 0x52;
pub const ModeSettingAddr: c_uint = 0x53;
pub const _PanelType00: c_uint = 0x00;
pub const _PanelType01: c_uint = 0x08;
pub const _PanelType02: c_uint = 0x10;
pub const _PanelType03: c_uint = 0x18;
pub const _PanelType04: c_uint = 0x20;
pub const _PanelType05: c_uint = 0x28;
pub const _PanelType06: c_uint = 0x30;
pub const _PanelType07: c_uint = 0x38;
pub const _PanelType08: c_uint = 0x40;
pub const _PanelType09: c_uint = 0x48;
pub const _PanelType0A: c_uint = 0x50;
pub const _PanelType0B: c_uint = 0x58;
pub const _PanelType0C: c_uint = 0x60;
pub const _PanelType0D: c_uint = 0x68;
pub const _PanelType0E: c_uint = 0x70;
pub const _PanelType0F: c_uint = 0x78;

pub const BIOSIDCodeAddr: c_uint = 0x235  /* Offsets to ptrs in BIOS image */;
pub const OEMUtilIDCodeAddr: c_uint = 0x237;
pub const VBModeIDTableAddr: c_uint = 0x239;
pub const OEMTVPtrAddr: c_uint = 0x241;
pub const PhaseTableAddr: c_uint = 0x243;
pub const NTSCFilterTableAddr: c_uint = 0x245;
pub const PALFilterTableAddr: c_uint = 0x247;
pub const OEMLCDPtr_1Addr: c_uint = 0x249;
pub const OEMLCDPtr_2Addr: c_uint = 0x24B;
pub const LCDHPosTable_1Addr: c_uint = 0x24D;
pub const LCDHPosTable_2Addr: c_uint = 0x24F;
pub const LCDVPosTable_1Addr: c_uint = 0x251;
pub const LCDVPosTable_2Addr: c_uint = 0x253;
pub const OEMLCDPIDTableAddr: c_uint = 0x255;
pub const VBModeStructSize: c_int = 5;
pub const PhaseTableSize: c_int = 4;
pub const FilterTableSize: c_int = 4;
pub const LCDHPosTableSize: c_int = 7;
pub const LCDVPosTableSize: c_int = 5;
pub const OEMLVDSPIDTableSize: c_int = 4;
pub const LVDSHPosTableSize: c_int = 4;
pub const LVDSVPosTableSize: c_int = 6;
pub const VB_ModeID: c_int = 0;
pub const VB_TVTableIndex: c_int = 1;
pub const VB_LCDTableIndex: c_int = 2;
pub const VB_LCDHIndex: c_int = 3;
pub const VB_LCDVIndex: c_int = 4;
pub const OEMLCDEnable: c_uint = 0x0001;
pub const OEMLCDDelayEnable: c_uint = 0x0002;
pub const OEMLCDPOSEnable: c_uint = 0x0004;
pub const OEMTVEnable: c_uint = 0x0100;
pub const OEMTVDelayEnable: c_uint = 0x0200;
pub const OEMTVFlickerEnable: c_uint = 0x0400;
pub const OEMTVPhaseEnable: c_uint = 0x0800;
pub const OEMTVFilterEnable: c_uint = 0x1000;
pub const OEMLCDPanelIDSupport: c_uint = 0x0080;
//
pub const SoftDRAMType: c_uint = 0x80;
pub const SoftSetting_OFFSET: c_uint = 0x52;
pub const SR07_OFFSET: c_uint = 0x7C;
pub const SR15_OFFSET: c_uint = 0x7D;
pub const SR16_OFFSET: c_uint = 0x81;
pub const SR17_OFFSET: c_uint = 0x85;
pub const SR19_OFFSET: c_uint = 0x8D;
pub const SR1F_OFFSET: c_uint = 0x99;
pub const SR21_OFFSET: c_uint = 0x9A;
pub const SR22_OFFSET: c_uint = 0x9B;
pub const SR23_OFFSET: c_uint = 0x9C;
pub const SR24_OFFSET: c_uint = 0x9D;
pub const SR25_OFFSET: c_uint = 0x9E;
pub const SR31_OFFSET: c_uint = 0x9F;
pub const SR32_OFFSET: c_uint = 0xA0;
pub const SR33_OFFSET: c_uint = 0xA1;
pub const CR40_OFFSET: c_uint = 0xA2;
pub const SR25_1_OFFSET: c_uint = 0xF6;
pub const CR49_OFFSET: c_uint = 0xF7;
pub const VB310Data_1_2_Offset: c_uint = 0xB6;
pub const VB310Data_4_D_Offset: c_uint = 0xB7;
pub const VB310Data_4_E_Offset: c_uint = 0xB8;
pub const VB310Data_4_10_Offset: c_uint = 0xBB;
pub const RGBSenseDataOffset: c_uint = 0xBD;
pub const YCSenseDataOffset: c_uint = 0xBF;
pub const VideoSenseDataOffset: c_uint = 0xC1;
pub const OutputSelectOffset: c_uint = 0xF3;
pub const ECLK_MCLK_DISTANCE: c_uint = 0x14;
pub const VBIOSTablePointerStart: c_uint = 0x100;

