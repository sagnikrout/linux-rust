//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/mb862xx/mb862xxfb.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mb862xx_l1_cfg {
    pub sx: c_ushort,
    pub sy: c_ushort,
    pub sw: c_ushort,
    pub sh: c_ushort,
    pub dx: c_ushort,
    pub dy: c_ushort,
    pub dw: c_ushort,
    pub dh: c_ushort,
    pub mirror: c_int,
}

pub const PCI_VENDOR_ID_FUJITSU_LIMITED: c_uint = 0x10cf;
pub const PCI_DEVICE_ID_FUJITSU_CORALP: c_uint = 0x2019;
pub const PCI_DEVICE_ID_FUJITSU_CORALPA: c_uint = 0x201e;
pub const PCI_DEVICE_ID_FUJITSU_CARMINE: c_uint = 0x202b;
pub const GC_MMR_CORALP_EVB_VAL: c_uint = 0x11d7fa13;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdctype {
    BT_NONE,
    BT_LIME,
    BT_MINT,
    BT_CORAL,
    BT_CORALP,
    BT_CARMINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mb862xx_gc_mode {
    pub /: *mut *mut fb_videomode def_mode; / mode of connected display,
    pub /: *mut *mut unsigned int def_bpp; / default depth,
    pub /: *mut *mut unsigned long max_vram; / connected SDRAM size,
    pub /: *mut *mut unsigned long ccf; / gdc clk,
    pub /: *mut *mut unsigned long mmr; / memory mode for SDRAM,
}

// private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mb862xxfb_par {
    pub /: *mut *mut *mut fb_info info; / fb info head,
    pub dev: *mut device,
    pub pdev: *mut pci_dev,
    pub /: *mut *mut *mut resource res; / framebuffer/mmio resource,
    pub /: *mut *mut resource_size_t fb_base_phys; / fb base, 36-bit PPC440EPx,
    pub /: *mut *mut resource_size_t mmio_base_phys; / io base addr,
    pub /: *mut *mut *mut void __iomem fb_base; / remapped framebuffer,
    pub /: *mut *mut *mut void __iomem mmio_base; / remapped registers,
    pub /: *mut *mut size_t mapped_vram; / length of remapped vram,
    pub /: *mut *mut size_t mmio_len; / length of register region,
    pub /: *mut *mut unsigned long cap_buf; / capture buffers offset,
    pub /: *mut *mut size_t cap_len; / length of capture buffers,
    pub /: *mut *mut *mut void __iomem host; / relocatable reg. bases,
    pub i2c: *mut void __iomem,
    pub disp: *mut void __iomem,
    pub disp1: *mut void __iomem,
    pub cap: *mut void __iomem,
    pub cap1: *mut void __iomem,
    pub draw: *mut void __iomem,
    pub geo: *mut void __iomem,
    pub pio: *mut void __iomem,
    pub ctrl: *mut void __iomem,
    pub dram_ctrl: *mut void __iomem,
    pub wrback: *mut void __iomem,
    pub irq: c_uint,
    pub /: *mut *mut unsigned int type; / GDC type,
    pub /: *mut *mut unsigned int refclk; / disp. reference clock,
    pub /: *mut *mut *mut mb862xx_gc_mode gc_mode; / GDC mode init data,
    pub /: *mut *mut int pre_init; / don't init display if 1,
    pub /: *mut *mut *mut i2c_adapter adap; / GDC I2C bus adapter,
    pub i2c_rs: c_int,
    pub l1_cfg: mb862xx_l1_cfg,
    pub l1_stride: c_int,
    pub pseudo_palette: [u32; 16],
}

extern "C" {
    pub fn mb862xxfb_init_accel(info: *mut fb_info, fbops: *mut fb_ops, xres: c_int);
}

extern "C" {
    pub fn mb862xx_i2c_init(par: *mut mb862xxfb_par) -> c_int;
}
extern "C" {
    pub fn mb862xx_i2c_exit(par: *mut mb862xxfb_par);
}

