//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sis/sis.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SiS 300/540/630[S]/730[S],
// SiS 315[E|PRO]/550/[M]65x/[M]661[F|M]X/740/[M]741[GX]/330/[M]76x[GX],
// XGI V3XT/V5/V8, Z7
// frame buffer driver for Linux kernels >=2.4.14 and >=2.6.3
//
// Copyright (C) 2001-2005 Thomas Winischhofer, Vienna, Austria.
//

pub const VER_MAJOR: c_int = 1;
pub const VER_MINOR: c_int = 8;
pub const VER_LEVEL: c_int = 9;

// Macro flag: #define SIS_NEW_CONFIG_COMPAT

// Macro flag: #define TWDEBUG(x)

// To be included in pci_ids.h

pub const PCI_DEVICE_ID_SI_650_VGA: c_uint = 0x6325;

pub const PCI_DEVICE_ID_SI_650: c_uint = 0x0650;

pub const PCI_DEVICE_ID_SI_651: c_uint = 0x0651;

pub const PCI_DEVICE_ID_SI_740: c_uint = 0x0740;

pub const PCI_DEVICE_ID_SI_330: c_uint = 0x0330;

pub const PCI_DEVICE_ID_SI_660_VGA: c_uint = 0x6330;

pub const PCI_DEVICE_ID_SI_661: c_uint = 0x0661;

pub const PCI_DEVICE_ID_SI_741: c_uint = 0x0741;

pub const PCI_DEVICE_ID_SI_660: c_uint = 0x0660;

pub const PCI_DEVICE_ID_SI_760: c_uint = 0x0760;

pub const PCI_DEVICE_ID_SI_761: c_uint = 0x0761;

pub const PCI_VENDOR_ID_XGI: c_uint = 0x18ca;

pub const PCI_DEVICE_ID_XGI_20: c_uint = 0x0020;

pub const PCI_DEVICE_ID_XGI_40: c_uint = 0x0040;

// To be included in fb.h

// ivideo->caps
pub const HW_CURSOR_CAP: c_uint = 0x80;
pub const TURBO_QUEUE_CAP: c_uint = 0x40;
pub const AGP_CMD_QUEUE_CAP: c_uint = 0x20;
pub const VM_CMD_QUEUE_CAP: c_uint = 0x10;
pub const MMIO_CMD_QUEUE_CAP: c_uint = 0x08;
// For 300 series

// For 315/Xabre series

pub const COMMAND_QUEUE_THRESHOLD: c_uint = 0x1F;
pub const SIS_OH_ALLOC_SIZE: c_int = 4000;
pub const SENTINEL: c_uint = 0x7fffffff;
pub const SEQ_ADR: c_uint = 0x14;
pub const SEQ_DATA: c_uint = 0x15;
pub const DAC_ADR: c_uint = 0x18;
pub const DAC_DATA: c_uint = 0x19;
pub const CRTC_ADR: c_uint = 0x24;
pub const CRTC_DATA: c_uint = 0x25;

pub const IND_SIS_PASSWORD: c_uint = 0x05  /* SRs */;
pub const IND_SIS_COLOR_MODE: c_uint = 0x06;
pub const IND_SIS_RAMDAC_CONTROL: c_uint = 0x07;
pub const IND_SIS_DRAM_SIZE: c_uint = 0x14;
pub const IND_SIS_MODULE_ENABLE: c_uint = 0x1E;
pub const IND_SIS_PCI_ADDRESS_SET: c_uint = 0x20;
pub const IND_SIS_TURBOQUEUE_ADR: c_uint = 0x26;
pub const IND_SIS_TURBOQUEUE_SET: c_uint = 0x27;
pub const IND_SIS_POWER_ON_TRAP: c_uint = 0x38;
pub const IND_SIS_POWER_ON_TRAP2: c_uint = 0x39;
pub const IND_SIS_CMDQUEUE_SET: c_uint = 0x26;
pub const IND_SIS_CMDQUEUE_THRESHOLD: c_uint = 0x27;
pub const IND_SIS_AGP_IO_PAD: c_uint = 0x48;
pub const SIS_CRT2_WENABLE_300: c_uint = 0x24  /* Part1 */;
pub const SIS_CRT2_WENABLE_315: c_uint = 0x2F;
pub const SIS_PASSWORD: c_uint = 0x86  /* SR05 */;
pub const SIS_INTERLACED_MODE: c_uint = 0x20  /* SR06 */;
pub const SIS_8BPP_COLOR_MODE: c_uint = 0x0;
pub const SIS_15BPP_COLOR_MODE: c_uint = 0x1;
pub const SIS_16BPP_COLOR_MODE: c_uint = 0x2;
pub const SIS_32BPP_COLOR_MODE: c_uint = 0x4;
pub const SIS_ENABLE_2D: c_uint = 0x40  /* SR1E */;
pub const SIS_MEM_MAP_IO_ENABLE: c_uint = 0x01  /* SR20 */;
pub const SIS_PCI_ADDR_ENABLE: c_uint = 0x80;
pub const SIS_AGP_CMDQUEUE_ENABLE: c_uint = 0x80  /* 315/330/340 series SR26 */;
pub const SIS_VRAM_CMDQUEUE_ENABLE: c_uint = 0x40;
pub const SIS_MMIO_CMD_ENABLE: c_uint = 0x20;
pub const SIS_CMD_QUEUE_SIZE_512k: c_uint = 0x00;
pub const SIS_CMD_QUEUE_SIZE_1M: c_uint = 0x04;
pub const SIS_CMD_QUEUE_SIZE_2M: c_uint = 0x08;
pub const SIS_CMD_QUEUE_SIZE_4M: c_uint = 0x0C;
pub const SIS_CMD_QUEUE_RESET: c_uint = 0x01;
pub const SIS_CMD_AUTO_CORR: c_uint = 0x02;
pub const SIS_CMD_QUEUE_SIZE_Z7_64k: c_uint = 0x00 /* XGI Z7 */;
pub const SIS_CMD_QUEUE_SIZE_Z7_128k: c_uint = 0x04;
pub const SIS_SIMULTANEOUS_VIEW_ENABLE: c_uint = 0x01  /* CR30 */;
pub const SIS_MODE_SELECT_CRT2: c_uint = 0x02;
pub const SIS_VB_OUTPUT_COMPOSITE: c_uint = 0x04;
pub const SIS_VB_OUTPUT_SVIDEO: c_uint = 0x08;
pub const SIS_VB_OUTPUT_SCART: c_uint = 0x10;
pub const SIS_VB_OUTPUT_LCD: c_uint = 0x20;
pub const SIS_VB_OUTPUT_CRT2: c_uint = 0x40;
pub const SIS_VB_OUTPUT_HIVISION: c_uint = 0x80;
pub const SIS_VB_OUTPUT_DISABLE: c_uint = 0x20  /* CR31 */;
pub const SIS_DRIVER_MODE: c_uint = 0x40;
pub const SIS_VB_COMPOSITE: c_uint = 0x01  /* CR32 */;
pub const SIS_VB_SVIDEO: c_uint = 0x02;
pub const SIS_VB_SCART: c_uint = 0x04;
pub const SIS_VB_LCD: c_uint = 0x08;
pub const SIS_VB_CRT2: c_uint = 0x10;
pub const SIS_CRT1: c_uint = 0x20;
pub const SIS_VB_HIVISION: c_uint = 0x40;
pub const SIS_VB_YPBPR: c_uint = 0x80;

pub const SIS_EXTERNAL_CHIP_MASK: c_uint = 0x0E  /* CR37 (< SiS 660) */;
pub const SIS_EXTERNAL_CHIP_SIS301: c_uint = 0x01  /* in CR37 << 1 ! */;
pub const SIS_EXTERNAL_CHIP_LVDS: c_uint = 0x02;
pub const SIS_EXTERNAL_CHIP_TRUMPION: c_uint = 0x03;
pub const SIS_EXTERNAL_CHIP_LVDS_CHRONTEL: c_uint = 0x04;
pub const SIS_EXTERNAL_CHIP_CHRONTEL: c_uint = 0x05;
pub const SIS310_EXTERNAL_CHIP_LVDS: c_uint = 0x02;
pub const SIS310_EXTERNAL_CHIP_LVDS_CHRONTEL: c_uint = 0x03;
pub const SIS_AGP_2X: c_uint = 0x20  /* CR48 */;
// vbflags, private entries (others in sisfb.h)
pub const VB_CONEXANT: c_uint = 0x00000800	/* 661 series only */;

pub const VB_302ELV: c_uint = 0x00004000;
pub const VB_301: c_uint = 0x00100000	/* Video bridge type */;
pub const VB_301B: c_uint = 0x00200000;
pub const VB_302B: c_uint = 0x00400000;
pub const VB_30xBDH: c_uint = 0x00800000	/* 30xB DH version (w/o LCD support) */;
pub const VB_LVDS: c_uint = 0x01000000;
pub const VB_CHRONTEL: c_uint = 0x02000000;
pub const VB_301LV: c_uint = 0x04000000;
pub const VB_302LV: c_uint = 0x08000000;
pub const VB_301C: c_uint = 0x10000000;

// vbflags2 (static stuff only!)
pub const VB2_SISUMC: c_uint = 0x00000001;
pub const VB2_301: c_uint = 0x00000002	/* Video bridge type */;
pub const VB2_301B: c_uint = 0x00000004;
pub const VB2_301C: c_uint = 0x00000008;
pub const VB2_307T: c_uint = 0x00000010;
pub const VB2_302B: c_uint = 0x00000800;
pub const VB2_301LV: c_uint = 0x00001000;
pub const VB2_302LV: c_uint = 0x00002000;
pub const VB2_302ELV: c_uint = 0x00004000;
pub const VB2_307LV: c_uint = 0x00008000;
pub const VB2_30xBDH: c_uint = 0x08000000      /* 30xB DH version (w/o LCD support) */;
pub const VB2_CONEXANT: c_uint = 0x10000000;
pub const VB2_TRUMPION: c_uint = 0x20000000;
pub const VB2_LVDS: c_uint = 0x40000000;
pub const VB2_CHRONTEL: c_uint = 0x80000000;

// I/O port access functions
extern "C" {
    pub fn SiS_SetReg(_arg: SISIOADDRESS, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn SiS_SetRegByte(_arg: SISIOADDRESS, _arg: u8);
}
extern "C" {
    pub fn SiS_SetRegShort(_arg: SISIOADDRESS, _arg: u16);
}
extern "C" {
    pub fn SiS_SetRegLong(_arg: SISIOADDRESS, _arg: u32);
}
extern "C" {
    pub fn SiS_SetRegANDOR(_arg: SISIOADDRESS, _arg: u8, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn SiS_SetRegAND(_arg: SISIOADDRESS, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn SiS_SetRegOR(_arg: SISIOADDRESS, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn SiS_GetReg(_arg: SISIOADDRESS, _arg: u8) -> u8;
}
extern "C" {
    pub fn SiS_GetRegByte(_arg: SISIOADDRESS) -> u8;
}
extern "C" {
    pub fn SiS_GetRegShort(_arg: SISIOADDRESS) -> u16;
}
extern "C" {
    pub fn SiS_GetRegLong(_arg: SISIOADDRESS) -> u32;
}
// Chrontel TV, DDC and DPMS functions
// from init.c
extern "C" {
    pub fn SiSInitPtr(SiS_Pr: *mut SiS_Private) -> bool;
}
extern "C" {
    pub fn SiS_DisplayOn(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_DisplayOff(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiSRegInit(SiS_Pr: *mut SiS_Private, BaseAddr: SISIOADDRESS);
}
extern "C" {
    pub fn SiS_SetEnableDstn(SiS_Pr: *mut SiS_Private, enable: c_int);
}
extern "C" {
    pub fn SiS_SetEnableFstn(SiS_Pr: *mut SiS_Private, enable: c_int);
}
extern "C" {
    pub fn SiSDetermineROMLayout661(SiS_Pr: *mut SiS_Private) -> bool;
}
extern "C" {
    pub fn SiS_GetRefCRTVCLK(SiS_Pr: *mut SiS_Private, Index: c_ushort, UseWide: c_int) -> c_ushort;
}
extern "C" {
    pub fn SiS_GetRefCRT1CRTC(SiS_Pr: *mut SiS_Private, Index: c_ushort, UseWide: c_int) -> c_ushort;
}

extern "C" {
    pub fn SiS_GetFIFOThresholdB300(idx1: c_ushort, idx2: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_GetLatencyFactor630(SiS_Pr: *mut SiS_Private, index: c_ushort) -> c_ushort;
}

extern "C" {
    pub fn SiS_LoadDAC(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort, ModeIdIndex: c_ushort);
}
extern "C" {
    pub fn SiSSetMode(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort) -> bool;
}
extern "C" {
    pub fn SiS_CalcCRRegisters(SiS_Pr: *mut SiS_Private, depth: c_int);
}
// From init301.c:
extern "C" {
    pub fn SiS_SetYPbPr(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_UnLockCRT2(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_DisableBridge(: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_SetCRT2Group(: *mut SiS_Private, short: unsigned) -> bool;
}
extern "C" {
    pub fn SiS_WaitRetrace1(SiS_Pr: *mut SiS_Private);
}
extern "C" {
    pub fn SiS_GetCH700x(SiS_Pr: *mut SiS_Private, tempax: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn SiS_IsVAMode(: *mut SiS_Private) -> bool;
}
extern "C" {
    pub fn SiS_IsDualEdge(: *mut SiS_Private) -> bool;
}

extern "C" {
    pub fn sisfb_read_nbridge_pci_dword(SiS_Pr: *mut SiS_Private, reg: c_int) -> c_uint;
}

extern "C" {
    pub fn sisfb_read_mio_pci_word(SiS_Pr: *mut SiS_Private, reg: c_int) -> c_uint;
}

// MMIO access macros

// Queue control MMIO registers
pub const Q_BASE_ADDR: c_uint = 0x85C0  /* Base address of software queue */;
pub const Q_WRITE_PTR: c_uint = 0x85C4  /* Current write pointer */;
pub const Q_READ_PTR: c_uint = 0x85C8  /* Current read pointer */;
pub const Q_STATUS: c_uint = 0x85CC  /* queue status */;

pub const FB_BLANK_UNBLANK: c_int = 0;

pub const FB_BLANK_NORMAL: c_int = 1;

pub const FB_BLANK_VSYNC_SUSPEND: c_int = 2;

pub const FB_BLANK_HSYNC_SUSPEND: c_int = 3;

pub const FB_BLANK_POWERDOWN: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _SIS_LCD_TYPE {
    LCD_INVALID = 0,
    LCD_800x600,
    LCD_1024x768,
    LCD_1280x1024,
    LCD_1280x960,
    LCD_640x480,
    LCD_1600x1200,
    LCD_1920x1440,
    LCD_2048x1536,
    LCD_320x240,	/* FSTN */
    LCD_1400x1050,
    LCD_1152x864,
    LCD_1152x768,
    LCD_1280x768,
    LCD_1024x600,
    LCD_320x240_2,	/* DSTN */
    LCD_320x240_3,	/* DSTN */
    LCD_848x480,
    LCD_1280x800,
    LCD_1680x1050,
    LCD_1280x720,
    LCD_1280x854,
    LCD_CUSTOM,
    LCD_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _SIS_CMDTYPE {
    MMIO_CMD = 0,
    AGP_CMD_QUEUE,
    VM_CMD_QUEUE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SIS_OH {
    pub poh_next: *mut SIS_OH,
    pub poh_prev: *mut SIS_OH,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SIS_OHALLOC {
    pub poha_next: *mut SIS_OHALLOC,
    pub aoh: [SIS_OH; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SIS_HEAP {
    pub oh_free: SIS_OH,
    pub oh_used: SIS_OH,
    pub poh_freelist: *mut SIS_OH,
    pub poha_chain: *mut SIS_OHALLOC,
    pub max_freesize: u32,
    pub vinfo: *mut sis_video_info,
}

// Our "par"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sis_video_info {
    pub cardnumber: c_int,
    pub memyselfandi: *mut fb_info,
    pub SiS_Pr: SiS_Private,
    pub /: *mut *mut sisfb_info sisfbinfo; / For ioctl SISFB_GET_INFO,
    pub default_var: fb_var_screeninfo,
    pub sisfb_fix: fb_fix_screeninfo,
    pub pseudo_palette: [u32; 16],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sisfb_monitor {
    pub hmin: u16,
    pub hmax: u16,
    pub vmin: u16,
    pub vmax: u16,
    pub dclockmax: u32,
    pub feature: u8,
    pub datavalid: bool,
    pub sisfb_thismonitor: },
    pub /: *mut *mut unsigned short chip_id; / PCI ID of chip,
    pub /: *mut *mut unsigned short chip_vendor; / PCI ID of vendor,
    pub myid: [c_char; 40],
    pub nbridge: *mut pci_dev,
    pub lpcdev: *mut pci_dev,
    pub /: *mut *mut int mni; / Mode number index,
    pub video_size: c_ulong,
    pub video_base: c_ulong,
    pub mmio_size: c_ulong,
    pub mmio_base: c_ulong,
    pub vga_base: c_ulong,
    pub video_offset: c_ulong,
    pub LFBsize: unsigned long UMAsize,,
    pub video_vbase: *mut void __iomem,
    pub mmio_vbase: *mut void __iomem,
    pub bios_abase: *mut c_uchar,
    pub wc_cookie: c_int,
    pub sisfb_mem: u32,
    pub sisfb_parm_mem: u32,
    pub sisfb_accel: c_int,
    pub sisfb_ypan: c_int,
    pub sisfb_max: c_int,
    pub sisfb_userom: c_int,
    pub sisfb_useoem: c_int,
    pub sisfb_mode_idx: c_int,
    pub sisfb_parm_rate: c_int,
    pub sisfb_crt1off: c_int,
    pub sisfb_forcecrt1: c_int,
    pub sisfb_crt2type: c_int,
    pub sisfb_crt2flags: c_int,
    pub sisfb_dstn: c_int,
    pub sisfb_fstn: c_int,
    pub sisfb_tvplug: c_int,
    pub sisfb_tvstd: c_int,
    pub sisfb_nocrt2rate: c_int,
    pub /: *mut *mut u32 heapstart; / offset,
    pub /: *mut *mut *mut void __iomem sisfb_heap_start; / address,
    pub /: *mut *mut *mut void __iomem sisfb_heap_end; / address,
    pub sisfb_heap_size: u32,
    pub havenoheap: c_int,
    pub /: *mut *mut SIS_HEAP sisfb_heap; / This card's vram heap,
    pub video_bpp: c_int,
    pub video_cmap_len: c_int,
    pub video_width: c_int,
    pub video_height: c_int,
    pub refresh_rate: c_uint,
    pub chip: c_uint,
    pub chip_real_id: c_uint,
    pub revision_id: u8,
    pub /: *mut *mut int sisvga_enabled; / PCI device was enabled,
    pub /: *mut *mut int video_linelength; / real pitch,
    pub /: *mut *mut int scrnpitchCRT1; / pitch regarding interlace,
    pub /: *mut *mut u16 DstColor; / For 2d acceleration,
    pub SiS310_AccelDepth: u32,
    pub CommandReg: u32,
    pub /: *mut *mut int cmdqueuelength; / Current (for accel),
    pub /: *mut *mut u32 cmdQueueSize; / Total size in KB,
    pub /: *mut *mut spinlock_t lockaccel; / Do not use outside of kernel!,
    pub pcibus: c_uint,
    pub pcislot: c_uint,
    pub pcifunc: c_uint,
    pub accel: c_int,
    pub engineok: c_int,
    pub subsysvendor: u16,
    pub subsysdevice: u16,
    pub /: *mut *mut u32 vbflags; / Replacing deprecated stuff from above,
    pub currentvbflags: u32,
    pub vbflags2: u32,
    pub lcdyres: int lcdxres,,
    pub defmodeidx: int lcddefmodeidx, tvdefmodeidx,,
    pub /: *mut *mut u32 CRT2LCDType; / defined in "SIS_LCD_TYPE",
    pub curDSTN: u32 curFSTN,,
    pub current_bpp: c_int,
    pub current_width: c_int,
    pub current_height: c_int,
    pub current_htotal: c_int,
    pub current_vtotal: c_int,
    pub current_linelength: c_int,
    pub current_pixclock: __u32,
    pub current_refresh_rate: c_int,
    pub current_base: c_uint,
    pub mode_no: u8,
    pub rate_idx: u8,
    pub modechanged: c_int,
    pub modeprechange: c_uchar,
    pub sisfb_lastrates: [u8; 128],
    pub newrom: c_int,
    pub haveXGIROM: c_int,
    pub registered: c_int,
    pub warncount: c_int,
    pub sisvga_engine: c_int,
    pub hwcursor_size: c_int,
    pub CRT2_write_enable: c_int,
    pub caps: u8,
    pub detectedpdc: u8,
    pub detectedpdca: u8,
    pub detectedlcda: u8,
    pub hwcursor_vbase: *mut void __iomem,
    pub chronteltype: c_int,
    pub tvypos: int tvxpos,,
    pub p2_1f,p2_20,p2_2b,p2_42,p2_43,p2_01,p2_02: u8,
    pub tvy: int tvx,,
    pub sisfblocked: u8,
    pub sisfb_infoblock: sisfb_info,
    pub sisfb_command: sisfb_cmd,
    pub sisfb_id: u32,
    pub sisfb_can_post: u8,
    pub sisfb_card_posted: u8,
    pub sisfb_was_boot_device: u8,
    pub next: *mut sis_video_info,
}

// from sis_accel.c
extern "C" {
    pub fn fbcon_sis_sync(info: *mut fb_info) -> c_int;
}
// Internal 2D accelerator functions
extern "C" {
    pub fn sisfb_initaccel(ivideo: *mut sis_video_info) -> c_int;
}
extern "C" {
    pub fn sisfb_syncaccel(ivideo: *mut sis_video_info);
}
// Internal general routines

extern "C" {
    pub fn sisfb_read_nbridge_pci_dword(SiS_Pr: *mut SiS_Private, reg: c_int) -> c_uint;
}
extern "C" {
    pub fn sisfb_write_nbridge_pci_dword(SiS_Pr: *mut SiS_Private, reg: c_int, val: c_uint);
}
extern "C" {
    pub fn sisfb_read_lpc_pci_dword(SiS_Pr: *mut SiS_Private, reg: c_int) -> c_uint;
}

extern "C" {
    pub fn sisfb_write_nbridge_pci_byte(SiS_Pr: *mut SiS_Private, reg: c_int, val: c_uchar);
}
extern "C" {
    pub fn sisfb_read_mio_pci_word(SiS_Pr: *mut SiS_Private, reg: c_int) -> c_uint;
}

// SiS-specific exported functions
extern "C" {
    pub fn sis_malloc(req: *mut sis_memreq);
}
extern "C" {
    pub fn sis_free(base: u32);
}
// Routines from init.c/init301.c
extern "C" {
    pub fn SiSRegInit(SiS_Pr: *mut SiS_Private, BaseAddr: SISIOADDRESS);
}
extern "C" {
    pub fn SiSSetMode(SiS_Pr: *mut SiS_Private, ModeNo: c_ushort) -> bool;
}
extern "C" {
    pub fn SiS_SetEnableDstn(SiS_Pr: *mut SiS_Private, enable: c_int);
}
extern "C" {
    pub fn SiS_SetEnableFstn(SiS_Pr: *mut SiS_Private, enable: c_int);
}
extern "C" {
    pub fn SiSDetermineROMLayout661(SiS_Pr: *mut SiS_Private) -> bool;
}
