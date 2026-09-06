//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/savage/savagefb.h
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
// linux/drivers/video/savagefb.h -- S3 Savage Framebuffer Driver
//
// Copyright (c) 2001  Denis Oliver Kropp <dok@convergence.de>
//
// This file is subject to the terms and conditions of the GNU General
// Public License.  See the file COPYING in the main directory of this
// archive for more details.
//

pub const PCI_CHIP_SAVAGE4: c_uint = 0x8a22;
pub const PCI_CHIP_SAVAGE3D: c_uint = 0x8a20;
pub const PCI_CHIP_SAVAGE3D_MV: c_uint = 0x8a21;
pub const PCI_CHIP_SAVAGE2000: c_uint = 0x9102;
pub const PCI_CHIP_SAVAGE_MX_MV: c_uint = 0x8c10;
pub const PCI_CHIP_SAVAGE_MX: c_uint = 0x8c11;
pub const PCI_CHIP_SAVAGE_IX_MV: c_uint = 0x8c12;
pub const PCI_CHIP_SAVAGE_IX: c_uint = 0x8c13;
pub const PCI_CHIP_PROSAVAGE_PM: c_uint = 0x8a25;
pub const PCI_CHIP_PROSAVAGE_KM: c_uint = 0x8a26;
pub const PCI_CHIP_S3TWISTER_P: c_uint = 0x8d01;
pub const PCI_CHIP_S3TWISTER_K: c_uint = 0x8d02;
pub const PCI_CHIP_PROSAVAGE_DDR: c_uint = 0x8d03;
pub const PCI_CHIP_PROSAVAGE_DDRK: c_uint = 0x8d04;
pub const PCI_CHIP_SUPSAV_MX128: c_uint = 0x8c22;
pub const PCI_CHIP_SUPSAV_MX64: c_uint = 0x8c24;
pub const PCI_CHIP_SUPSAV_MX64C: c_uint = 0x8c26;
pub const PCI_CHIP_SUPSAV_IX128SDR: c_uint = 0x8c2a;
pub const PCI_CHIP_SUPSAV_IX128DDR: c_uint = 0x8c2b;
pub const PCI_CHIP_SUPSAV_IX64SDR: c_uint = 0x8c2c;
pub const PCI_CHIP_SUPSAV_IX64DDR: c_uint = 0x8c2d;
pub const PCI_CHIP_SUPSAV_IXCSDR: c_uint = 0x8c2e;
pub const PCI_CHIP_SUPSAV_IXCDDR: c_uint = 0x8c2f;

// Chip tags.  These are used to group the adapters into
// related families.
//
pub const BIOS_BSIZE: c_int = 1024;
pub const BIOS_BASE: c_uint = 0xc0000;
pub const SAVAGE_NEWMMIO_REGBASE_S3: c_uint = 0x1000000  /* 16MB */;
pub const SAVAGE_NEWMMIO_REGBASE_S4: c_uint = 0x0000000;
pub const SAVAGE_NEWMMIO_REGSIZE: c_uint = 0x0080000  /* 512kb */;
pub const SAVAGE_NEWMMIO_VGABASE: c_uint = 0x8000;
pub const BASE_FREQ: c_int = 14318;
pub const HALF_BASE_FREQ: c_int = 7159;
pub const FIFO_CONTROL_REG: c_uint = 0x8200;
pub const MIU_CONTROL_REG: c_uint = 0x8204;
pub const STREAMS_TIMEOUT_REG: c_uint = 0x8208;
pub const MISC_TIMEOUT_REG: c_uint = 0x820c;
pub const MONO_PAT_0: c_uint = 0xa4e8;
pub const MONO_PAT_1: c_uint = 0xa4ec;
pub const MAXFIFO: c_uint = 0x7f00;
pub const BCI_CMD_NOP: c_uint = 0x40000000;
pub const BCI_CMD_SETREG: c_uint = 0x96000000;
pub const BCI_CMD_RECT: c_uint = 0x48000000;
pub const BCI_CMD_RECT_XP: c_uint = 0x01000000;
pub const BCI_CMD_RECT_YP: c_uint = 0x02000000;
pub const BCI_CMD_SEND_COLOR: c_uint = 0x00008000;
pub const BCI_CMD_DEST_GBD: c_uint = 0x00000000;
pub const BCI_CMD_SRC_GBD: c_uint = 0x00000020;
pub const BCI_CMD_SRC_SOLID: c_uint = 0x00000000;
pub const BCI_CMD_SRC_MONO: c_uint = 0x00000060;
pub const BCI_CMD_CLIP_NEW: c_uint = 0x00006000;
pub const BCI_CMD_CLIP_LR: c_uint = 0x00004000;

pub const BCI_GBD1: c_uint = 0xE0;
pub const BCI_GBD2: c_uint = 0xE1;
pub const BCI_BUFFER_OFFSET: c_uint = 0x10000;
pub const BCI_SIZE: c_uint = 0x4000;

pub const BCI_CMD_SEND_COLOR: c_uint = 0x00008000;
pub const DISP_CRT: c_int = 1;
pub const DISP_LCD: c_int = 2;
pub const DISP_DFP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtimings {
    pub Clock: c_uint,
    pub HDisplay: c_uint,
    pub HSyncStart: c_uint,
    pub HSyncEnd: c_uint,
    pub HTotal: c_uint,
    pub HAdjusted: c_uint,
    pub VDisplay: c_uint,
    pub VSyncStart: c_uint,
    pub VSyncEnd: c_uint,
    pub VTotal: c_uint,
    pub sync: c_uint,
    pub dblscan: c_int,
    pub interlaced: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savage_reg {
    pub /: *mut *mut unsigned char MiscOutReg; / Misc,
    pub /: *mut *mut unsigned char CRTC[25]; / Crtc Controller,
    pub /: *mut *mut unsigned char Sequencer[5]; / Video Sequencer,
    pub /: *mut *mut unsigned char Graphics[9]; / Video Graphics,
    pub /: *mut *mut unsigned char Attribute[21]; / Video Attribute,
    pub refresh: unsigned int mode,,
    pub SR0F: unsigned char SR08, SR0E,,
    pub SR30: unsigned char SR10, SR11, SR12, SR13, SR15, SR18, SR29,,
    pub SR54: [c_uchar; 8],
    pub Clock: c_uchar,
    pub CR3C: unsigned char CR31, CR32, CR33, CR34, CR36, CR3A, CR3B,,
    pub CR45: unsigned char CR40, CR41, CR42, CR43,,
    pub CR5E: unsigned char CR50, CR51, CR53, CR55, CR58, CR5B, CR5D,,
    pub CR6F: unsigned char CR60, CR63, CR65, CR66, CR67, CR68, CR69, CR6D,,
    pub CR88: unsigned char CR86,,
    pub CRB0: unsigned char CR90, CR91,,
    pub /: *mut *mut unsigned int STREAMS[22]; / yuck, streams regs,
    pub MMPR3: unsigned int MMPR0, MMPR1, MMPR2,,
}

// ---------------------------------------------------------------------
pub const NR_PALETTE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct savagefb_i2c_chan {
    pub par: *mut savagefb_par,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
    pub ioaddr: *mut volatile u8 __iomem,
    pub reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savagefb_par {
    pub pcidev: *mut pci_dev,
    pub chip: savage_chipset,
    pub chan: savagefb_i2c_chan,
    pub state: savage_reg,
    pub save: savage_reg,
    pub initial: savage_reg,
    pub vgastate: vgastate,
    pub open_lock: mutex,
    pub pseudo_palette: [u32; 16],
    pub open_count: u32,
    pub paletteEnabled: c_int,
    pub pm_state: c_int,
    pub display_type: c_int,
    pub dvi: c_int,
    pub crtonly: c_int,
    pub dacSpeedBpp: c_int,
    pub maxClock: c_int,
    pub minClock: c_int,
    pub numClocks: c_int,
    pub clock: [c_int; 4],
    pub LCDclk: int MCLK, REFCLK,,
    pub vbase: *mut void __iomem,
    pub pbase: u32,
    pub len: u32,
    pub wc_cookie: c_int,
    pub video: },
    pub vbase: *mut void __iomem,
    pub pbase: u32,
    pub len: u32,
    pub mmio: },
    pub bci_base: *mut volatile u32 __iomem,
    pub bci_ptr: c_uint,
    pub cob_offset: u32,
    pub cob_size: u32,
    pub cob_index: c_int,
    pub par): *mut *mut void (SavageWaitIdle) (struct savagefb_par,
    pub space): *mut *mut *mut void (SavageWaitFifo) (struct savagefb_par par, int,
    pub HorizScaleFactor: c_int,
// Panels size
    pub SavagePanelWidth: c_int,
    pub SavagePanelHeight: c_int,
    pub transp: u16 red, green, blue,,
    pub palette: [}; NR_PALETTE],
    pub depth: c_int,
    pub vwidth: c_int,
}

pub const BCI_BD_BW_DISABLE: c_uint = 0x10000000;

// IO functions
extern "C" {
    pub fn readb(addr: par->mmio.vbase +) -> return;
}
extern "C" {
    pub fn readw(addr: par->mmio.vbase +) -> return;
}
extern "C" {
    pub fn readl(addr: par->mmio.vbase +) -> return;
}
extern "C" {
    pub fn savage_in8(addr: 0x8000 +, _arg: par) -> return;
}
extern "C" {
    pub fn savage_in16(addr: 0x8000 +, _arg: par) -> return;
}
extern "C" {
    pub fn savage_in32(addr: 0x8000 +, _arg: par) -> return;
}
extern "C" {
    pub fn vga_in8(_arg: 0x3d5, _arg: par) -> return;
}
extern "C" {
    pub fn vga_in8(_arg: 0x3cf, _arg: par) -> return;
}
extern "C" {
    pub fn vga_in8(_arg: 0x3c5, _arg: par) -> return;
}

// Macro flag: #define savagefb_set_clip(x)

extern "C" {
    pub fn savagefb_create_i2c_busses(info: *mut fb_info);
}
extern "C" {
    pub fn savagefb_delete_i2c_busses(info: *mut fb_info);
}
extern "C" {
    pub fn savagefb_sync(info: *mut fb_info) -> c_int;
}
