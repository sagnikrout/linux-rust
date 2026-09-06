//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/newport.h
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


// SPDX-License-Identifier: GPL-2.0
// $Id: newport.h,v 1.5 1999/08/04 06:01:51 ulfc Exp $
//
// newport.h: Defines and register layout for NEWPORT graphics
// hardware.
//
// Copyright (C) 1996 David S. Miller (davem@davemloft.net)
//
// Ulf Carlsson - Compatibility with the IRIX structures added
//
pub type npireg_t = volatile unsigned int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union npfloat {
    pub flt: volatile float,
    pub word: npireg_t,
}

pub type npfreg_t = npfloat;
#[repr(C)]
#[derive(Copy, Clone)]
pub union np_dcb {
    pub byword: npireg_t,
    pub byshort: { volatile unsigned short s0, s1; },
    pub bybytes: { volatile unsigned char b0, b1, b2, b3; },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct newport_rexregs {
    pub /: *mut *mut npireg_t drawmode1; / GL extra mode bits,
pub const DM1_PLANES: c_uint = 0x00000007;
pub const DM1_NOPLANES: c_uint = 0x00000000;
pub const DM1_RGBPLANES: c_uint = 0x00000001;
pub const DM1_RGBAPLANES: c_uint = 0x00000002;
pub const DM1_OLAYPLANES: c_uint = 0x00000004;
pub const DM1_PUPPLANES: c_uint = 0x00000005;
pub const DM1_CIDPLANES: c_uint = 0x00000006;
pub const NPORT_DMODE1_DDMASK: c_uint = 0x00000018;
pub const NPORT_DMODE1_DD4: c_uint = 0x00000000;
pub const NPORT_DMODE1_DD8: c_uint = 0x00000008;
pub const NPORT_DMODE1_DD12: c_uint = 0x00000010;
pub const NPORT_DMODE1_DD24: c_uint = 0x00000018;
pub const NPORT_DMODE1_DSRC: c_uint = 0x00000020;
pub const NPORT_DMODE1_YFLIP: c_uint = 0x00000040;
pub const NPORT_DMODE1_RWPCKD: c_uint = 0x00000080;
pub const NPORT_DMODE1_HDMASK: c_uint = 0x00000300;
pub const NPORT_DMODE1_HD4: c_uint = 0x00000000;
pub const NPORT_DMODE1_HD8: c_uint = 0x00000100;
pub const NPORT_DMODE1_HD12: c_uint = 0x00000200;
pub const NPORT_DMODE1_HD32: c_uint = 0x00000300;
pub const NPORT_DMODE1_RWDBL: c_uint = 0x00000400;
pub const NPORT_DMODE1_ESWAP: c_uint = 0x00000800 /* Endian swap */;
pub const NPORT_DMODE1_CCMASK: c_uint = 0x00007000;
pub const NPORT_DMODE1_CCLT: c_uint = 0x00001000;
pub const NPORT_DMODE1_CCEQ: c_uint = 0x00002000;
pub const NPORT_DMODE1_CCGT: c_uint = 0x00004000;
pub const NPORT_DMODE1_RGBMD: c_uint = 0x00008000;
pub const NPORT_DMODE1_DENAB: c_uint = 0x00010000 /* Dither enable */;
pub const NPORT_DMODE1_FCLR: c_uint = 0x00020000 /* Fast clear */;
pub const NPORT_DMODE1_BENAB: c_uint = 0x00040000 /* Blend enable */;
pub const NPORT_DMODE1_SFMASK: c_uint = 0x00380000;
pub const NPORT_DMODE1_SF0: c_uint = 0x00000000;
pub const NPORT_DMODE1_SF1: c_uint = 0x00080000;
pub const NPORT_DMODE1_SFDC: c_uint = 0x00100000;
pub const NPORT_DMODE1_SFMDC: c_uint = 0x00180000;
pub const NPORT_DMODE1_SFSA: c_uint = 0x00200000;
pub const NPORT_DMODE1_SFMSA: c_uint = 0x00280000;
pub const NPORT_DMODE1_DFMASK: c_uint = 0x01c00000;
pub const NPORT_DMODE1_DF0: c_uint = 0x00000000;
pub const NPORT_DMODE1_DF1: c_uint = 0x00400000;
pub const NPORT_DMODE1_DFSC: c_uint = 0x00800000;
pub const NPORT_DMODE1_DFMSC: c_uint = 0x00c00000;
pub const NPORT_DMODE1_DFSA: c_uint = 0x01000000;
pub const NPORT_DMODE1_DFMSA: c_uint = 0x01400000;
pub const NPORT_DMODE1_BBENAB: c_uint = 0x02000000 /* Back blend enable */;
pub const NPORT_DMODE1_PFENAB: c_uint = 0x04000000 /* Pre-fetch enable */;
pub const NPORT_DMODE1_ABLEND: c_uint = 0x08000000 /* Alpha blend */;
pub const NPORT_DMODE1_LOMASK: c_uint = 0xf0000000;
pub const NPORT_DMODE1_LOZERO: c_uint = 0x00000000;
pub const NPORT_DMODE1_LOAND: c_uint = 0x10000000;
pub const NPORT_DMODE1_LOANDR: c_uint = 0x20000000;
pub const NPORT_DMODE1_LOSRC: c_uint = 0x30000000;
pub const NPORT_DMODE1_LOANDI: c_uint = 0x40000000;
pub const NPORT_DMODE1_LODST: c_uint = 0x50000000;
pub const NPORT_DMODE1_LOXOR: c_uint = 0x60000000;
pub const NPORT_DMODE1_LOOR: c_uint = 0x70000000;
pub const NPORT_DMODE1_LONOR: c_uint = 0x80000000;
pub const NPORT_DMODE1_LOXNOR: c_uint = 0x90000000;
pub const NPORT_DMODE1_LONDST: c_uint = 0xa0000000;
pub const NPORT_DMODE1_LOORR: c_uint = 0xb0000000;
pub const NPORT_DMODE1_LONSRC: c_uint = 0xc0000000;
pub const NPORT_DMODE1_LOORI: c_uint = 0xd0000000;
pub const NPORT_DMODE1_LONAND: c_uint = 0xe0000000;
pub const NPORT_DMODE1_LOONE: c_uint = 0xf0000000;
    pub /: *mut *mut npireg_t drawmode0; / REX command register,
// These bits define the graphics opcode being performed.
pub const NPORT_DMODE0_OPMASK: c_uint = 0x00000003 /* Opcode mask */;
pub const NPORT_DMODE0_NOP: c_uint = 0x00000000 /* No operation */;
pub const NPORT_DMODE0_RD: c_uint = 0x00000001 /* Read operation */;
pub const NPORT_DMODE0_DRAW: c_uint = 0x00000002 /* Draw operation */;
pub const NPORT_DMODE0_S2S: c_uint = 0x00000003 /* Screen to screen operation */;
// The following decide what addressing mode(s) are to be used
pub const NPORT_DMODE0_AMMASK: c_uint = 0x0000001c /* Address mode mask */;
pub const NPORT_DMODE0_SPAN: c_uint = 0x00000000 /* Spanning address mode */;
pub const NPORT_DMODE0_BLOCK: c_uint = 0x00000004 /* Block address mode */;
pub const NPORT_DMODE0_ILINE: c_uint = 0x00000008 /* Iline address mode */;
pub const NPORT_DMODE0_FLINE: c_uint = 0x0000000c /* Fline address mode */;
pub const NPORT_DMODE0_ALINE: c_uint = 0x00000010 /* Aline address mode */;
pub const NPORT_DMODE0_TLINE: c_uint = 0x00000014 /* Tline address mode */;
pub const NPORT_DMODE0_BLINE: c_uint = 0x00000018 /* Bline address mode */;
// And now some misc. operation control bits.
pub const NPORT_DMODE0_DOSETUP: c_uint = 0x00000020;
pub const NPORT_DMODE0_CHOST: c_uint = 0x00000040;
pub const NPORT_DMODE0_AHOST: c_uint = 0x00000080;
pub const NPORT_DMODE0_STOPX: c_uint = 0x00000100;
pub const NPORT_DMODE0_STOPY: c_uint = 0x00000200;
pub const NPORT_DMODE0_SK1ST: c_uint = 0x00000400;
pub const NPORT_DMODE0_SKLST: c_uint = 0x00000800;
pub const NPORT_DMODE0_ZPENAB: c_uint = 0x00001000;
pub const NPORT_DMODE0_LISPENAB: c_uint = 0x00002000;
pub const NPORT_DMODE0_LISLST: c_uint = 0x00004000;
pub const NPORT_DMODE0_L32: c_uint = 0x00008000;
pub const NPORT_DMODE0_ZOPQ: c_uint = 0x00010000;
pub const NPORT_DMODE0_LISOPQ: c_uint = 0x00020000;
pub const NPORT_DMODE0_SHADE: c_uint = 0x00040000;
pub const NPORT_DMODE0_LRONLY: c_uint = 0x00080000;
pub const NPORT_DMODE0_XYOFF: c_uint = 0x00100000;
pub const NPORT_DMODE0_CLAMP: c_uint = 0x00200000;
pub const NPORT_DMODE0_ENDPF: c_uint = 0x00400000;
pub const NPORT_DMODE0_YSTR: c_uint = 0x00800000;
    pub /: *mut *mut npireg_t lsmode; / Mode for line stipple ops,
    pub /: *mut *mut npireg_t lspattern; / Pattern for line stipple ops,
    pub /: *mut *mut npireg_t lspatsave; / Backup save pattern,
    pub /: *mut *mut npireg_t zpattern; / Pixel zpattern,
    pub /: *mut *mut npireg_t colorback; / Background color,
    pub /: *mut *mut npireg_t colorvram; / Clear color for fast vram,
    pub /: *mut *mut npireg_t alpharef; / Reference value for afunctions,
    pub pad0: c_uint,
    pub /: *mut *mut npireg_t smask0x; / Window GL relative screen mask 0,
    pub /: *mut *mut npireg_t smask0y; / Window GL relative screen mask 0,
    pub _setup: npireg_t,
    pub _stepz: npireg_t,
    pub _lsrestore: npireg_t,
    pub _lssave: npireg_t,
    pub _pad1: [c_uint; 0x30],
// Iterators, full state for context switch
    pub /: *mut *mut npfreg_t _xstart; / X-start point (current),
    pub /: *mut *mut npfreg_t _ystart; / Y-start point (current),
    pub /: *mut *mut npfreg_t _xend; / x-end point,
    pub /: *mut *mut npfreg_t _yend; / y-end point,
    pub /: *mut *mut npireg_t xsave; / copy of xstart integer value for BLOCk addressing MODE,
    pub /: *mut *mut npireg_t xymove; / x.y offset from xstart, ystart for relative operations,
    pub bresd: npfreg_t,
    pub bress1: npfreg_t,
    pub bresoctinc1: npireg_t,
    pub bresrndinc2: volatile int,
    pub brese1: npireg_t,
    pub bress2: npireg_t,
    pub aweight0: npireg_t,
    pub aweight1: npireg_t,
    pub xstartf: npfreg_t,
    pub ystartf: npfreg_t,
    pub xendf: npfreg_t,
    pub yendf: npfreg_t,
    pub xstarti: npireg_t,
    pub xendf1: npfreg_t,
    pub xystarti: npireg_t,
    pub xyendi: npireg_t,
    pub xstartendi: npireg_t,
    pub _unused2: [c_uint; 0x29],
    pub colorred: npfreg_t,
    pub coloralpha: npfreg_t,
    pub colorgrn: npfreg_t,
    pub colorblue: npfreg_t,
    pub slopered: npfreg_t,
    pub slopealpha: npfreg_t,
    pub slopegrn: npfreg_t,
    pub slopeblue: npfreg_t,
    pub wrmask: npireg_t,
    pub colori: npireg_t,
    pub colorx: npfreg_t,
    pub slopered1: npfreg_t,
    pub hostrw0: npireg_t,
    pub hostrw1: npireg_t,
    pub dcbmode: npireg_t,
pub const NPORT_DMODE_WMASK: c_uint = 0x00000003;
pub const NPORT_DMODE_W4: c_uint = 0x00000000;
pub const NPORT_DMODE_W1: c_uint = 0x00000001;
pub const NPORT_DMODE_W2: c_uint = 0x00000002;
pub const NPORT_DMODE_W3: c_uint = 0x00000003;
pub const NPORT_DMODE_EDPACK: c_uint = 0x00000004;
pub const NPORT_DMODE_ECINC: c_uint = 0x00000008;
pub const NPORT_DMODE_CMASK: c_uint = 0x00000070;
pub const NPORT_DMODE_AMASK: c_uint = 0x00000780;
pub const NPORT_DMODE_AVC2: c_uint = 0x00000000;
pub const NPORT_DMODE_ACMALL: c_uint = 0x00000080;
pub const NPORT_DMODE_ACM0: c_uint = 0x00000100;
pub const NPORT_DMODE_ACM1: c_uint = 0x00000180;
pub const NPORT_DMODE_AXMALL: c_uint = 0x00000200;
pub const NPORT_DMODE_AXM0: c_uint = 0x00000280;
pub const NPORT_DMODE_AXM1: c_uint = 0x00000300;
pub const NPORT_DMODE_ABT: c_uint = 0x00000380;
pub const NPORT_DMODE_AVCC1: c_uint = 0x00000400;
pub const NPORT_DMODE_AVAB1: c_uint = 0x00000480;
pub const NPORT_DMODE_ALG3V0: c_uint = 0x00000500;
pub const NPORT_DMODE_A1562: c_uint = 0x00000580;
pub const NPORT_DMODE_ESACK: c_uint = 0x00000800;
pub const NPORT_DMODE_EASACK: c_uint = 0x00001000;
pub const NPORT_DMODE_CWMASK: c_uint = 0x0003e000;
pub const NPORT_DMODE_CHMASK: c_uint = 0x007c0000;
pub const NPORT_DMODE_CSMASK: c_uint = 0x0f800000;
pub const NPORT_DMODE_SENDIAN: c_uint = 0x10000000;
    pub _unused3: c_uint,
    pub dcbdata0: np_dcb,
    pub dcbdata1: npireg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct newport_cregs {
    pub smask1x: npireg_t,
    pub smask1y: npireg_t,
    pub smask2x: npireg_t,
    pub smask2y: npireg_t,
    pub smask3x: npireg_t,
    pub smask3y: npireg_t,
    pub smask4x: npireg_t,
    pub smask4y: npireg_t,
    pub topscan: npireg_t,
    pub xywin: npireg_t,
    pub clipmode: npireg_t,
pub const NPORT_CMODE_SM0: c_uint = 0x00000001;
pub const NPORT_CMODE_SM1: c_uint = 0x00000002;
pub const NPORT_CMODE_SM2: c_uint = 0x00000004;
pub const NPORT_CMODE_SM3: c_uint = 0x00000008;
pub const NPORT_CMODE_SM4: c_uint = 0x00000010;
pub const NPORT_CMODE_CMSK: c_uint = 0x00001e00;
    pub _unused0: c_uint,
    pub config: c_uint,
pub const NPORT_CFG_G32MD: c_uint = 0x00000001;
pub const NPORT_CFG_BWIDTH: c_uint = 0x00000002;
pub const NPORT_CFG_ERCVR: c_uint = 0x00000004;
pub const NPORT_CFG_BDMSK: c_uint = 0x00000078;
pub const NPORT_CFG_BFAINT: c_uint = 0x00000080;
pub const NPORT_CFG_GDMSK: c_uint = 0x00001f80;
pub const NPORT_CFG_GD0: c_uint = 0x00000100;
pub const NPORT_CFG_GD1: c_uint = 0x00000200;
pub const NPORT_CFG_GD2: c_uint = 0x00000400;
pub const NPORT_CFG_GD3: c_uint = 0x00000800;
pub const NPORT_CFG_GD4: c_uint = 0x00001000;
pub const NPORT_CFG_GFAINT: c_uint = 0x00002000;
pub const NPORT_CFG_TOMSK: c_uint = 0x0001c000;
pub const NPORT_CFG_VRMSK: c_uint = 0x000e0000;
pub const NPORT_CFG_FBTYP: c_uint = 0x00100000;
    pub _unused1: npireg_t,
    pub status: npireg_t,
pub const NPORT_STAT_VERS: c_uint = 0x00000007;
pub const NPORT_STAT_GBUSY: c_uint = 0x00000008;
pub const NPORT_STAT_BBUSY: c_uint = 0x00000010;
pub const NPORT_STAT_VRINT: c_uint = 0x00000020;
pub const NPORT_STAT_VIDINT: c_uint = 0x00000040;
pub const NPORT_STAT_GLMSK: c_uint = 0x00001f80;
pub const NPORT_STAT_BLMSK: c_uint = 0x0007e000;
pub const NPORT_STAT_BFIRQ: c_uint = 0x00080000;
pub const NPORT_STAT_GFIRQ: c_uint = 0x00100000;
    pub ustatus: npireg_t,
    pub dcbreset: npireg_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct newport_regs {
    pub set: newport_rexregs,
    pub _unused0: [c_uint; 0x16e],
    pub go: newport_rexregs,
    pub _unused1: [c_uint; 0x22e],
    pub cset: newport_cregs,
    pub _unused2: [c_uint; 0x1ef],
    pub cgo: newport_cregs,
}

// configregs
// dcb registers
// Reading/writing VC2 registers.
pub const VC2_REGADDR_INDEX: c_uint = 0x00000000;
pub const VC2_REGADDR_IREG: c_uint = 0x00000010;
pub const VC2_REGADDR_RAM: c_uint = 0x00000030;

pub const VC2_VLINET_ADDR: c_uint = 0x000;
pub const VC2_VFRAMET_ADDR: c_uint = 0x400;
pub const VC2_CGLYPH_ADDR: c_uint = 0x500;
// Now the Indexed registers of the VC2.
pub const VC2_IREG_VENTRY: c_uint = 0x00;
pub const VC2_IREG_CENTRY: c_uint = 0x01;
pub const VC2_IREG_CURSX: c_uint = 0x02;
pub const VC2_IREG_CURSY: c_uint = 0x03;
pub const VC2_IREG_CCURSX: c_uint = 0x04;
pub const VC2_IREG_DENTRY: c_uint = 0x05;
pub const VC2_IREG_SLEN: c_uint = 0x06;
pub const VC2_IREG_RADDR: c_uint = 0x07;
pub const VC2_IREG_VFPTR: c_uint = 0x08;
pub const VC2_IREG_VLSPTR: c_uint = 0x09;
pub const VC2_IREG_VLIR: c_uint = 0x0a;
pub const VC2_IREG_VLCTR: c_uint = 0x0b;
pub const VC2_IREG_CTPTR: c_uint = 0x0c;
pub const VC2_IREG_WCURSY: c_uint = 0x0d;
pub const VC2_IREG_DFPTR: c_uint = 0x0e;
pub const VC2_IREG_DLTPTR: c_uint = 0x0f;
pub const VC2_IREG_CONTROL: c_uint = 0x10;
pub const VC2_IREG_CONFIG: c_uint = 0x20;
// VC2 Control register bits
pub const VC2_CTRL_EVIRQ: c_uint = 0x0001;
pub const VC2_CTRL_EDISP: c_uint = 0x0002;
pub const VC2_CTRL_EVIDEO: c_uint = 0x0004;
pub const VC2_CTRL_EDIDS: c_uint = 0x0008;
pub const VC2_CTRL_ECURS: c_uint = 0x0010;
pub const VC2_CTRL_EGSYNC: c_uint = 0x0020;
pub const VC2_CTRL_EILACE: c_uint = 0x0040;
pub const VC2_CTRL_ECDISP: c_uint = 0x0080;
pub const VC2_CTRL_ECCURS: c_uint = 0x0100;
pub const VC2_CTRL_ECG64: c_uint = 0x0200;
pub const VC2_CTRL_GLSEL: c_uint = 0x0400;
// Controlling the color map on NEWPORT.
pub const NCMAP_REGADDR_AREG: c_uint = 0x00000000;
pub const NCMAP_REGADDR_ALO: c_uint = 0x00000000;
pub const NCMAP_REGADDR_AHI: c_uint = 0x00000010;
pub const NCMAP_REGADDR_PBUF: c_uint = 0x00000020;
pub const NCMAP_REGADDR_CREG: c_uint = 0x00000030;
pub const NCMAP_REGADDR_SREG: c_uint = 0x00000040;
pub const NCMAP_REGADDR_RREG: c_uint = 0x00000060;

// Miscellaneous NEWPORT routines.
pub const BUSY_TIMEOUT: c_int = 100000;
//
// DCBMODE register defines:
//
// Width of the data being transferred for each DCBDATA[01] word
pub const DCB_DATAWIDTH_4: c_uint = 0x0;
pub const DCB_DATAWIDTH_1: c_uint = 0x1;
pub const DCB_DATAWIDTH_2: c_uint = 0x2;
pub const DCB_DATAWIDTH_3: c_uint = 0x3;
// If set, all of DCBDATA will be moved, otherwise only DATAWIDTH bytes

// Enables DCBCRS auto increment after each DCB transfer

// shift for accessing the control register select address (DBCCRS, 3 bits)
pub const DCB_CRS_SHIFT: c_int = 4;
// DCBADDR (4 bits): display bus slave address
pub const DCB_ADDR_SHIFT: c_int = 7;

// DCB protocol ack types

pub const DCB_CSWIDTH_SHIFT: c_int = 13;
pub const DCB_CSHOLD_SHIFT: c_int = 18;
pub const DCB_CSSETUP_SHIFT: c_int = 23;
// XMAP9 specific defines
// XMAP9 -- registers as seen on the DCBMODE register

pub const BT445_REVISION_REG: c_uint = 0x01;
