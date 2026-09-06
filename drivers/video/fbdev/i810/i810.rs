//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/i810/i810.h
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


// -*- linux-c -*-
// linux/drivers/video/i810.h -- Intel 810 General Definitions/Declarations
//
// Copyright (C) 2001 Antonino Daplas<adaplas@pol.net>
// All Rights Reserved
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//

// Fence

// Raster ops
pub const COLOR_COPY_ROP: c_uint = 0xF0;
pub const PAT_COPY_ROP: c_uint = 0xCC;
pub const CLEAR_ROP: c_uint = 0x00;
pub const WHITE_ROP: c_uint = 0xFF;
pub const INVERT_ROP: c_uint = 0x55;
pub const XOR_ROP: c_uint = 0x5A;
// 2D Engine definitions
pub const SOLIDPATTERN: c_uint = 0x80000000;
pub const NONSOLID: c_uint = 0x00000000;

pub const INCREMENT: c_uint = 0x00000000;

pub const ARB_ON: c_uint = 0x00000001;
pub const ARB_OFF: c_uint = 0x00000000;
pub const SYNC_FLIP: c_uint = 0x00000000;
pub const ASYNC_FLIP: c_uint = 0x00000040;
pub const OPTYPE_MASK: c_uint = 0xE0000000;
pub const PARSER_MASK: c_uint = 0x001F8000;
pub const D2_MASK: c_uint = 0x001FC000         /* 2D mask */;
// Instruction type
// There are more but pertains to 3D
pub const PARSER: c_uint = 0x00000000;

// Parser
pub const NOP: c_uint = 0x00               /* No operation, padding */;

// Blit
pub const SETUP_BLIT: c_uint = 0x00;

pub const VERSION_MAJOR: c_int = 0;
pub const VERSION_MINOR: c_int = 9;
pub const VERSION_TEENIE: c_int = 0;

// mvo: intel i815

pub const PCI_DEVICE_ID_INTEL_82815_100: c_uint = 0x1102;

pub const PCI_DEVICE_ID_INTEL_82815_NOAGP: c_uint = 0x1112;

pub const PCI_DEVICE_ID_INTEL_82815_FULL_CTRL: c_uint = 0x1130;

// General Defines
pub const I810_PAGESIZE: c_int = 4096;

pub const SAREA_SIZE: c_int = 4096;
pub const PCI_I810_MISCC: c_uint = 0x72;

pub const CURSOR_SIZE: c_int = 4096;
pub const OFF: c_int = 0;
pub const ON: c_int = 1;
pub const MAX_KEY: c_int = 256;
pub const WAIT_COUNT: c_int = 10000000;
pub const IRING_PAD: c_int = 8;
pub const FONTDATAMAX: c_int = 8192;
// Masks (AND ops) and OR's

pub const SCR_OFF: c_uint = 0x20;
pub const DRAM_ON: c_uint = 0x08;
pub const DRAM_OFF: c_uint = 0xE7;
pub const PG_ENABLE_MASK: c_uint = 0x01;

// defines for restoring registers partially

pub const MN_MASK: c_uint = 0x3FF03FF;

// Power Management
pub const DPMS_MASK: c_uint = 0xF0000;
pub const POWERON: c_uint = 0x00000;
pub const STANDBY: c_uint = 0x20000;
pub const SUSPEND: c_uint = 0x80000;
pub const POWERDOWN: c_uint = 0xA0000;

// Ringbuffer
pub const RBUFFER_START_MASK: c_uint = 0xFFFFF000;
pub const RBUFFER_SIZE_MASK: c_uint = 0x001FF000;
pub const RBUFFER_HEAD_MASK: c_uint = 0x001FFFFC;
pub const RBUFFER_TAIL_MASK: c_uint = 0x001FFFF8;
// Video Timings
pub const REF_FREQ: c_int = 24000000;
pub const TARGET_N_MAX: c_int = 30;
pub const MAX_PIXELCLOCK: c_int = 230000000;
pub const MIN_PIXELCLOCK: c_int = 15000000;
pub const VFMAX: c_int = 60;
pub const VFMIN: c_int = 60;
pub const HFMAX: c_int = 30000;
pub const HFMIN: c_int = 29000;
// Cursor
pub const CURSOR_ENABLE_MASK: c_uint = 0x1000;
pub const CURSOR_MODE_64_TRANS: c_int = 4;
pub const CURSOR_MODE_64_XOR: c_int = 5;
pub const CURSOR_MODE_64_3C: c_int = 6;
pub const COORD_INACTIVE: c_int = 0;

pub const EXTENDED_PALETTE: c_int = 1;
// AGP Memory Types
pub const AGP_NORMAL_MEMORY: c_int = 0;
pub const AGP_DCACHE_MEMORY: c_int = 1;
pub const AGP_PHYSICAL_MEMORY: c_int = 2;
// Allocated resource Flags
pub const FRAMEBUFFER_REQ: c_int = 1;
pub const MMIO_REQ: c_int = 2;
pub const PCI_DEVICE_ENABLED: c_int = 4;
pub const HAS_FONTCACHE: c_int = 8;
// driver flags
pub const HAS_ACCELERATION: c_int = 2;
pub const ALWAYS_SYNC: c_int = 4;
pub const LOCKUP: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gtt_data {
    pub i810_fb_memory: *mut agp_memory,
    pub i810_cursor_memory: *mut agp_memory,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_registers {
    pub P: u32 pixclock, M, N,,
    pub cr03: u8 cr00, cr01, cr02,,
    pub cr07: u8 cr04, cr05, cr06,,
    pub cr12: u8 cr09, cr10, cr11,,
    pub cr30: u8 cr13, cr15, cr16,,
    pub cr39: u8 cr31, cr32, cr33, cr35,,
    pub bpp16_100: u32 bpp8_100,,
    pub bpp8_133: u32 bpp24_100,,
    pub bpp24_133: u32 bpp16_133,,
    pub msr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct heap_data {
    pub physical: c_ulong,
    pub virtual: *mut __u8 __iomem,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct state_registers {
    pub dclk_0ds: u32 dclk_1d, dclk_2d,,
    pub pgtbl_ctl: u32 pixconf, fw_blc,,
    pub dplystas: u32 fence0, hws_pga,,
    pub imr: u16 bltcntl, hwstam, ier, iir,,
    pub cr04: u8 cr00, cr01, cr02, cr03,,
    pub cr09: u8 cr05, cr06, cr07, cr08,,
    pub cr14: u8 cr10, cr11, cr12, cr13,,
    pub gr10: u8 cr15, cr16, cr17, cr80,,
    pub cr35: u8 cr30, cr31, cr32, cr33,,
    pub msr: u8 cr39, cr41, cr70, sr01,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i810fb_i2c_chan {
    pub par: *mut i810fb_par,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
    pub ddc_base: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i810fb_par {
    pub regs: mode_registers,
    pub hw_state: state_registers,
    pub i810_gtt: gtt_data,
    pub i810fb_ops: fb_ops,
    pub dev: *mut pci_dev,
    pub aperture: heap_data,
    pub fb: heap_data,
    pub iring: heap_data,
    pub cursor_heap: heap_data,
    pub state: vgastate,
    pub chan: [i810fb_i2c_chan; 3],
    pub open_lock: mutex,
    pub use_count: c_uint,
    pub pseudo_palette: [u32; 16],
    pub mmio_start_phys: c_ulong,
    pub mmio_start_virtual: *mut u8 __iomem,
    pub edid: *mut u8,
    pub pitch: u32,
    pub pixconf: u32,
    pub watermark: u32,
    pub mem_freq: u32,
    pub res_flags: u32,
    pub dev_flags: u32,
    pub cur_tail: u32,
    pub depth: u32,
    pub blit_bpp: u32,
    pub ovract: u32,
    pub cur_state: u32,
    pub ddc_num: u32,
    pub wc_cookie: c_int,
    pub bltcntl: u16,
    pub interlace: u8,
}

//
// Register I/O
//

