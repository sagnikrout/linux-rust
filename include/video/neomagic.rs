//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/neomagic.h
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
// linux/include/video/neo_reg.h -- NeoMagic Framebuffer Driver
//
// Copyright (c) 2001  Denis Oliver Kropp <dok@convergence.de>
//
// This file is subject to the terms and conditions of the GNU General
// Public License.  See the file COPYING in the main directory of this
// archive for more details.
//
pub const NEO_BS0_BLT_BUSY: c_uint = 0x00000001;
pub const NEO_BS0_FIFO_AVAIL: c_uint = 0x00000002;
pub const NEO_BS0_FIFO_PEND: c_uint = 0x00000004;
pub const NEO_BC0_DST_Y_DEC: c_uint = 0x00000001;
pub const NEO_BC0_X_DEC: c_uint = 0x00000002;
pub const NEO_BC0_SRC_TRANS: c_uint = 0x00000004;
pub const NEO_BC0_SRC_IS_FG: c_uint = 0x00000008;
pub const NEO_BC0_SRC_Y_DEC: c_uint = 0x00000010;
pub const NEO_BC0_FILL_PAT: c_uint = 0x00000020;
pub const NEO_BC0_SRC_MONO: c_uint = 0x00000040;
pub const NEO_BC0_SYS_TO_VID: c_uint = 0x00000080;
pub const NEO_BC1_DEPTH8: c_uint = 0x00000100;
pub const NEO_BC1_DEPTH16: c_uint = 0x00000200;
pub const NEO_BC1_X_320: c_uint = 0x00000400;
pub const NEO_BC1_X_640: c_uint = 0x00000800;
pub const NEO_BC1_X_800: c_uint = 0x00000c00;
pub const NEO_BC1_X_1024: c_uint = 0x00001000;
pub const NEO_BC1_X_1152: c_uint = 0x00001400;
pub const NEO_BC1_X_1280: c_uint = 0x00001800;
pub const NEO_BC1_X_1600: c_uint = 0x00001c00;
pub const NEO_BC1_DST_TRANS: c_uint = 0x00002000;
pub const NEO_BC1_MSTR_BLT: c_uint = 0x00004000;
pub const NEO_BC1_FILTER_Z: c_uint = 0x00008000;
pub const NEO_BC2_WR_TR_DST: c_uint = 0x00800000;
pub const NEO_BC3_SRC_XY_ADDR: c_uint = 0x01000000;
pub const NEO_BC3_DST_XY_ADDR: c_uint = 0x02000000;
pub const NEO_BC3_CLIP_ON: c_uint = 0x04000000;
pub const NEO_BC3_FIFO_EN: c_uint = 0x08000000;
pub const NEO_BC3_BLT_ON_ADDR: c_uint = 0x10000000;
pub const NEO_BC3_SKIP_MAPPING: c_uint = 0x80000000;
pub const NEO_MODE1_DEPTH8: c_uint = 0x0100;
pub const NEO_MODE1_DEPTH16: c_uint = 0x0200;
pub const NEO_MODE1_DEPTH24: c_uint = 0x0300;
pub const NEO_MODE1_X_320: c_uint = 0x0400;
pub const NEO_MODE1_X_640: c_uint = 0x0800;
pub const NEO_MODE1_X_800: c_uint = 0x0c00;
pub const NEO_MODE1_X_1024: c_uint = 0x1000;
pub const NEO_MODE1_X_1152: c_uint = 0x1400;
pub const NEO_MODE1_X_1280: c_uint = 0x1800;
pub const NEO_MODE1_X_1600: c_uint = 0x1c00;
pub const NEO_MODE1_BLT_ON_ADDR: c_uint = 0x2000;
// These are offseted in MMIO space by par->CursorOff
pub const NEOREG_CURSCNTL: c_uint = 0x00;
pub const NEOREG_CURSX: c_uint = 0x04;
pub const NEOREG_CURSY: c_uint = 0x08;
pub const NEOREG_CURSBGCOLOR: c_uint = 0x0C;
pub const NEOREG_CURSFGCOLOR: c_uint = 0x10;
pub const NEOREG_CURSMEMPOS: c_uint = 0x14;
pub const NEO_CURS_DISABLE: c_uint = 0x00000000;
pub const NEO_CURS_ENABLE: c_uint = 0x00000001;
pub const NEO_ICON64_ENABLE: c_uint = 0x00000008;
pub const NEO_ICON128_ENABLE: c_uint = 0x0000000C;
pub const NEO_ICON_BLANK: c_uint = 0x00000010;
pub const NEO_GR01_SUPPRESS_VSYNC: c_uint = 0x10;
pub const NEO_GR01_SUPPRESS_HSYNC: c_uint = 0x20;

pub const PCI_CHIP_NM2070: c_uint = 0x0001;
pub const PCI_CHIP_NM2090: c_uint = 0x0002;
pub const PCI_CHIP_NM2093: c_uint = 0x0003;
pub const PCI_CHIP_NM2097: c_uint = 0x0083;
pub const PCI_CHIP_NM2160: c_uint = 0x0004;
pub const PCI_CHIP_NM2200: c_uint = 0x0005;
pub const PCI_CHIP_NM2230: c_uint = 0x0025;
pub const PCI_CHIP_NM2360: c_uint = 0x0006;
pub const PCI_CHIP_NM2380: c_uint = 0x0016;
// ---------------------------------------------------------------------
pub const MMIO_SIZE: c_uint = 0x200000;
pub const NEO_EXT_CR_MAX: c_uint = 0x85;
pub const NEO_EXT_GR_MAX: c_uint = 0xC7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct neofb_par {
    pub state: vgastate,
    pub ref_count: c_uint,
    pub /: *mut *mut unsigned char MiscOutReg; / Misc,
    pub /: *mut *mut unsigned char CRTC[25]; / Crtc Controller,
    pub /: *mut *mut unsigned char Sequencer[5]; / Video Sequencer,
    pub /: *mut *mut unsigned char Graphics[9]; / Video Graphics,
    pub /: *mut *mut unsigned char Attribute[21]; / Video Attribute,
    pub GeneralLockReg: c_uchar,
    pub ExtCRTDispAddr: c_uchar,
    pub ExtCRTOffset: c_uchar,
    pub SysIfaceCntl1: c_uchar,
    pub SysIfaceCntl2: c_uchar,
    pub ExtColorModeSelect: c_uchar,
    pub biosMode: c_uchar,
    pub PanelDispCntlReg1: c_uchar,
    pub PanelDispCntlReg2: c_uchar,
    pub PanelDispCntlReg3: c_uchar,
    pub PanelDispCntlRegRead: c_uchar,
    pub PanelVertCenterReg1: c_uchar,
    pub PanelVertCenterReg2: c_uchar,
    pub PanelVertCenterReg3: c_uchar,
    pub PanelVertCenterReg4: c_uchar,
    pub PanelVertCenterReg5: c_uchar,
    pub PanelHorizCenterReg1: c_uchar,
    pub PanelHorizCenterReg2: c_uchar,
    pub PanelHorizCenterReg3: c_uchar,
    pub PanelHorizCenterReg4: c_uchar,
    pub PanelHorizCenterReg5: c_uchar,
    pub ProgramVCLK: c_int,
    pub VCLK3NumeratorLow: c_uchar,
    pub VCLK3NumeratorHigh: c_uchar,
    pub VCLK3Denominator: c_uchar,
    pub VerticalExt: c_uchar,
    pub wc_cookie: c_int,
    pub mmio_vbase: *mut u8 __iomem,
    pub cursorOff: u8,
    pub /: *mut *mut *mut u8 cursorPad; / Must die !!,
    pub neo2200: *mut Neo2200 __iomem,
// Panels size
    pub NeoPanelWidth: c_int,
    pub NeoPanelHeight: c_int,
    pub maxClock: c_int,
    pub pci_burst: c_int,
    pub lcd_stretch: c_int,
    pub internal_display: c_int,
    pub external_display: c_int,
    pub libretto: c_int,
    pub palette: [u32; 16],
}
