//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/nvidia/nv_type.h
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

pub const NV_ARCH_04: c_uint = 0x04;
pub const NV_ARCH_10: c_uint = 0x10;
pub const NV_ARCH_20: c_uint = 0x20;
pub const NV_ARCH_30: c_uint = 0x30;
pub const NV_ARCH_40: c_uint = 0x40;

pub const V_DBLSCAN: c_int = 1;
pub const NUM_SEQ_REGS: c_uint = 0x05;
pub const NUM_CRT_REGS: c_uint = 0x41;
pub const NUM_GRC_REGS: c_uint = 0x09;
pub const NUM_ATC_REGS: c_uint = 0x15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvidia_i2c_chan {
    pub par: *mut nvidia_par,
    pub ddc_base: c_ulong,
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riva_regs {
    pub ext: RIVA_HW_STATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvidia_par {
    pub SavedReg: RIVA_HW_STATE,
    pub ModeReg: RIVA_HW_STATE,
    pub initial_state: RIVA_HW_STATE,
    pub CurrentState: *mut RIVA_HW_STATE,
    pub vgastate: vgastate,
    pub pseudo_palette: [u32; 16],
    pub pci_dev: *mut pci_dev,
    pub Architecture: u32,
    pub CursorStart: u32,
    pub Chipset: c_int,
    pub FbAddress: c_ulong,
    pub FbStart: *mut u8 __iomem,
    pub FbMapSize: u32,
    pub FbUsableSize: u32,
    pub ScratchBufferSize: u32,
    pub ScratchBufferStart: u32,
    pub FpScale: c_int,
    pub MinVClockFreqKHz: u32,
    pub MaxVClockFreqKHz: u32,
    pub CrystalFreqKHz: u32,
    pub RamAmountKBytes: u32,
    pub IOBase: u32,
    pub CurrentLayout: NVFBLayout,
    pub cursor_reset: c_int,
    pub lockup: c_int,
    pub videoKey: c_int,
    pub FlatPanel: c_int,
    pub FPDither: c_int,
    pub Television: c_int,
    pub CRTCnumber: c_int,
    pub alphaCursor: c_int,
    pub twoHeads: c_int,
    pub twoStagePLL: c_int,
    pub fpScaler: c_int,
    pub fpWidth: c_int,
    pub fpHeight: c_int,
    pub PanelTweak: c_int,
    pub paneltweak: c_int,
    pub LVDS: c_int,
    pub pm_state: c_int,
    pub reverse_i2c: c_int,
    pub crtcSync_read: u32,
    pub fpSyncs: u32,
    pub dmaPut: u32,
    pub dmaCurrent: u32,
    pub dmaFree: u32,
    pub dmaMax: u32,
    pub dmaBase: *mut u32 __iomem,
    pub currentRop: u32,
    pub WaitVSyncPossible: c_int,
    pub BlendingPossible: c_int,
    pub paletteEnabled: u32,
    pub forceCRTC: u32,
    pub open_count: u32,
    pub DDCBase: u8,
    pub wc_cookie: c_int,
    pub chan: [nvidia_i2c_chan; 3],
    pub REGS: *mut volatile u32 __iomem,
    pub PCRTC0: *mut volatile u32 __iomem,
    pub PCRTC: *mut volatile u32 __iomem,
    pub PRAMDAC0: *mut volatile u32 __iomem,
    pub PFB: *mut volatile u32 __iomem,
    pub PFIFO: *mut volatile u32 __iomem,
    pub PGRAPH: *mut volatile u32 __iomem,
    pub PEXTDEV: *mut volatile u32 __iomem,
    pub PTIMER: *mut volatile u32 __iomem,
    pub PMC: *mut volatile u32 __iomem,
    pub PRAMIN: *mut volatile u32 __iomem,
    pub FIFO: *mut volatile u32 __iomem,
    pub CURSOR: *mut volatile u32 __iomem,
    pub PCIO0: *mut volatile u8 __iomem,
    pub PCIO: *mut volatile u8 __iomem,
    pub PVIO: *mut volatile u8 __iomem,
    pub PDIO0: *mut volatile u8 __iomem,
    pub PDIO: *mut volatile u8 __iomem,
    pub PRAMDAC: *mut volatile u32 __iomem,
}
