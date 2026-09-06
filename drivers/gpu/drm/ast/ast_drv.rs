//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/ast/ast_drv.h
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
// Copyright 2012 Red Hat Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// Authors: Dave Airlie <airlied@redhat.com>
//

pub const DRIVER_MAJOR: c_int = 0;
pub const DRIVER_MINOR: c_int = 1;
pub const DRIVER_PATCHLEVEL: c_int = 0;
pub const PCI_CHIP_AST2000: c_uint = 0x2000;
pub const PCI_CHIP_AST2100: c_uint = 0x2010;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ast_chip {
// 1st gen
    AST1000 = __AST_CHIP(1, 0), // unused
    AST2000 = __AST_CHIP(1, 1),
// 2nd gen
    AST1100 = __AST_CHIP(2, 0),
    AST2100 = __AST_CHIP(2, 1),
    AST2050 = __AST_CHIP(2, 2), // unused
// 3rd gen
    AST2200 = __AST_CHIP(3, 0),
    AST2150 = __AST_CHIP(3, 1),
// 4th gen
    AST2300 = __AST_CHIP(4, 0),
    AST1300 = __AST_CHIP(4, 1),
    AST1050 = __AST_CHIP(4, 2), // unused
// 5th gen
    AST2400 = __AST_CHIP(5, 0),
    AST1400 = __AST_CHIP(5, 1),
    AST1250 = __AST_CHIP(5, 2), // unused
// 6th gen
    AST2500 = __AST_CHIP(6, 0),
    AST2510 = __AST_CHIP(6, 1),
    AST2520 = __AST_CHIP(6, 2), // unused
// 7th gen
    AST2600 = __AST_CHIP(7, 0),
    AST2620 = __AST_CHIP(7, 1), // unused
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ast_tx_chip {
    AST_TX_NONE,
    AST_TX_SIL164,
    AST_TX_DP501,
    AST_TX_ASTDP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ast_config_mode {
    ast_use_p2a,
    ast_use_dt,
    ast_use_defaults
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ast_dram_layout {
    AST_DRAM_512Mx16 = 0,
    AST_DRAM_1Gx16 = 1,
    AST_DRAM_512Mx32 = 2,
    AST_DRAM_1Gx32 = 3,
    AST_DRAM_2Gx16 = 6,
    AST_DRAM_4Gx16 = 7,
    AST_DRAM_8Gx16 = 8,
}

//
// Hardware cursor
//
pub const AST_MAX_HWC_WIDTH: c_int = 64;
pub const AST_MAX_HWC_HEIGHT: c_int = 64;

//
// Planes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_plane {
    pub base: drm_plane,
    pub offset: u64,
    pub size: c_ulong,
}

extern "C" {
    pub fn container_of(_arg: plane, ast_plane: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_cursor_plane {
    pub base: ast_plane,
    pub argb4444: [u8; AST_HWC_SIZE],
}

extern "C" {
    pub fn container_of(_arg: to_ast_plane(plane), ast_cursor_plane: struct, _arg: base) -> return;
}
//
// Connector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_connector {
    pub base: drm_connector,
    pub physical_status: drm_connector_status,
}

extern "C" {
    pub fn container_of(_arg: connector, ast_connector: struct, _arg: base) -> return;
}
//
// Device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_device_quirks {
//
// CRTC memory request threshold
//
    pub crtc_mem_req_threshold_low: c_uchar,
    pub crtc_mem_req_threshold_high: c_uchar,
//
// Adjust hsync values to load next scanline early. Signalled
// by AST2500PreCatchCRT in VBIOS mode flags.
//
    pub crtc_hsync_precatch_needed: bool,
//
// Workaround for modes with HSync Time that is not a multiple
// of 8 (e.g., 1920x1080@60Hz, HSync +44 pixels).
//
    pub crtc_hsync_add4_needed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_device {
    pub base: drm_device,
    pub quirks: *const ast_device_quirks,
    pub regs: *mut void __iomem,
    pub ioregs: *mut void __iomem,
    pub dp501_fw_buf: *mut void __iomem,
    pub config_mode: ast_config_mode,
    pub chip: ast_chip,
    pub dclk_table: *const ast_vbios_dclk_info,
    pub vram: *mut void __iomem,
    pub vram_base: c_ulong,
    pub vram_size: c_ulong,
    pub /: *mut *mut mutex modeset_lock; / Protects access to modeset I/O registers in ioregs,
    pub tx_chip: ast_tx_chip,
    pub primary_plane: ast_plane,
    pub cursor_plane: ast_cursor_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: ast_connector,
    pub vga: },
    pub encoder: drm_encoder,
    pub connector: ast_connector,
    pub sil164: },
    pub encoder: drm_encoder,
    pub connector: ast_connector,
    pub dp501: },
    pub encoder: drm_encoder,
    pub connector: ast_connector,
    pub astdp: },
    pub output: },
    pub /: *mut *mut bool support_wsxga_p; / 1680x1050,
    pub /: *mut *mut bool support_fullhd; / 1920x1080,
    pub /: *mut *mut bool support_wuxga; / 1920x1200,
    pub dp501_fw_addr: *mut u8,
    pub /: *const *const *const firmware dp501_fw; / dp501 fw,
}

extern "C" {
    pub fn container_of(_arg: dev, ast_device: struct, _arg: base) -> return;
}
extern "C" {
    pub fn __AST_CHIP_GEN(_arg: ast->chip) -> return;
}

//
// MMIO access
//
extern "C" {
    pub fn ioread8(reg: addr +) -> return;
}
extern "C" {
    pub fn __ast_read8(_arg: addr, 1: reg +) -> return;
}
extern "C" {
    pub fn __ast_read8(_arg: ast->ioregs, _arg: reg) -> return;
}
extern "C" {
    pub fn __ast_read8_i(_arg: ast->ioregs, _arg: base, _arg: index) -> return;
}
extern "C" {
    pub fn __ast_read8_i_masked(_arg: ast->ioregs, _arg: base, _arg: index, _arg: preserve_mask) -> return;
}
//
// Register access
//
extern "C" {
    pub fn ioread32(reg: addr +) -> return;
}
extern "C" {
    pub fn __ast_read32(_arg: ast->regs, _arg: reg) -> return;
}
extern "C" {
    pub fn __ast_mindwm(regs: *mut void __iomem, r: u32) -> u32;
}
extern "C" {
    pub fn __ast_moutdwm(regs: *mut void __iomem, r: u32, v: u32);
}
extern "C" {
    pub fn ast_mindwm(ast: *mut ast_device, r: u32) -> u32;
}
extern "C" {
    pub fn ast_moutdwm(ast: *mut ast_device, r: u32, v: u32);
}
extern "C" {
    pub fn ast_moutdwm_poll(ast: *mut ast_device, r: u32, v: u32, res: u32);
}
//
// VBIOS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vbios_stdtable {
    pub misc: u8,
    pub seq: [u8; 4],
    pub crtc: [u8; 25],
    pub ar: [u8; 20],
    pub gr: [u8; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_vbios_dclk_info {
    pub param1: u8,
    pub param2: u8,
    pub param3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ast_crtc_state {
    pub base: drm_crtc_state,
// Last known format of primary plane
    pub format: *const drm_format_info,
    pub std_table: *const ast_vbios_stdtable,
    pub vmode: *const ast_vbios_enhtable,
}

pub const AST_MM_ALIGN_SHIFT: c_int = 4;

pub const AST_DP501_DEFAULT_DCLK: c_int = 65;
pub const AST_DP501_GBL_VERSION: c_uint = 0xf000;
pub const AST_DP501_PNPMONITOR: c_uint = 0xf010;
pub const AST_DP501_LINKRATE: c_uint = 0xf014;
pub const AST_DP501_EDID_DATA: c_uint = 0xf020;
//
// ASTDP resoultion table:
// EX:	ASTDP_A_B_C:
// A: Resolution
// B: Refresh Rate
// C: Misc information, such as CVT, Reduce Blanked
//
pub const ASTDP_640x480_60: c_uint = 0x00;
pub const ASTDP_640x480_72: c_uint = 0x01;
pub const ASTDP_640x480_75: c_uint = 0x02;
pub const ASTDP_640x480_85: c_uint = 0x03;
pub const ASTDP_800x600_56: c_uint = 0x04;
pub const ASTDP_800x600_60: c_uint = 0x05;
pub const ASTDP_800x600_72: c_uint = 0x06;
pub const ASTDP_800x600_75: c_uint = 0x07;
pub const ASTDP_800x600_85: c_uint = 0x08;
pub const ASTDP_1024x768_60: c_uint = 0x09;
pub const ASTDP_1024x768_70: c_uint = 0x0A;
pub const ASTDP_1024x768_75: c_uint = 0x0B;
pub const ASTDP_1024x768_85: c_uint = 0x0C;
pub const ASTDP_1280x1024_60: c_uint = 0x0D;
pub const ASTDP_1280x1024_75: c_uint = 0x0E;
pub const ASTDP_1280x1024_85: c_uint = 0x0F;
pub const ASTDP_1600x1200_60: c_uint = 0x10;
pub const ASTDP_320x240_60: c_uint = 0x11;
pub const ASTDP_400x300_60: c_uint = 0x12;
pub const ASTDP_512x384_60: c_uint = 0x13;
pub const ASTDP_1920x1200_60: c_uint = 0x14;
pub const ASTDP_1920x1080_60: c_uint = 0x15;
pub const ASTDP_1280x800_60: c_uint = 0x16;
pub const ASTDP_1280x800_60_RB: c_uint = 0x17;
pub const ASTDP_1440x900_60: c_uint = 0x18;
pub const ASTDP_1440x900_60_RB: c_uint = 0x19;
pub const ASTDP_1680x1050_60: c_uint = 0x1A;
pub const ASTDP_1680x1050_60_RB: c_uint = 0x1B;
pub const ASTDP_1600x900_60: c_uint = 0x1C;
pub const ASTDP_1600x900_60_RB: c_uint = 0x1D;
pub const ASTDP_1366x768_60: c_uint = 0x1E;
pub const ASTDP_1152x864_75: c_uint = 0x1F;
extern "C" {
    pub fn ast_mm_init(ast: *mut ast_device) -> c_int;
}
// ast_drv.c
extern "C" {
    pub fn __ast_device_set_tx_chip(ast: *mut ast_device, tx_chip: ast_tx_chip);
}
// ast_2000.c
extern "C" {
    pub fn ast_2000_post(ast: *mut ast_device) -> c_int;
}
extern "C" {
    pub fn ast_2000_detect_tx_chip(ast: *mut ast_device, need_post: bool);
}
// ast_2100.c
extern "C" {
    pub fn ast_2100_post(ast: *mut ast_device) -> c_int;
}
extern "C" {
    pub fn __ast_2100_detect_wsxga_p(ast: *mut ast_device) -> bool;
}
extern "C" {
    pub fn __ast_2100_detect_wuxga(ast: *mut ast_device) -> bool;
}
// ast_2200.c
// ast_2300.c
extern "C" {
    pub fn ast_2300_post(ast: *mut ast_device) -> c_int;
}
extern "C" {
    pub fn ast_2300_detect_tx_chip(ast: *mut ast_device);
}
// ast_2400.c
// ast_2500.c
extern "C" {
    pub fn ast_2500_patch_ahb(regs: *mut void __iomem);
}
extern "C" {
    pub fn ast_2500_post(ast: *mut ast_device) -> c_int;
}
// ast_2600.c
extern "C" {
    pub fn ast_2600_post(ast: *mut ast_device) -> c_int;
}
// ast post
extern "C" {
    pub fn ast_post_gpu(ast: *mut ast_device) -> c_int;
}
extern "C" {
    pub fn ast_vga_output_init(ast: *mut ast_device) -> c_int;
}
extern "C" {
    pub fn ast_sil164_output_init(ast: *mut ast_device) -> c_int;
}
// ast_cursor.c
extern "C" {
    pub fn ast_cursor_vram_offset(ast: *mut ast_device) -> c_long;
}
extern "C" {
    pub fn ast_cursor_plane_init(ast: *mut ast_device) -> c_int;
}
// ast dp501
extern "C" {
    pub fn ast_backup_fw(ast: *mut ast_device, addr: *mut u8, size: u32) -> bool;
}
extern "C" {
    pub fn ast_init_3rdtx(ast: *mut ast_device);
}
extern "C" {
    pub fn ast_dp501_output_init(ast: *mut ast_device) -> c_int;
}
// aspeed DP
extern "C" {
    pub fn ast_dp_launch(ast: *mut ast_device) -> c_int;
}
extern "C" {
    pub fn ast_astdp_output_init(ast: *mut ast_device) -> c_int;
}
// ast_mode.c
extern "C" {
    pub fn ast_mode_config_init(ast: *mut ast_device) -> c_int;
}
