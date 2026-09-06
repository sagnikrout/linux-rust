//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/atmel-hlcdc/atmel_hlcdc_dc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2014 Traphandler
// Copyright (C) 2014 Free Electrons
// Copyright (C) 2014 Atmel
//
// Author: Jean-Jacques Hiblot <jjhiblot@traphandler.com>
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//

// LCD controller common registers
pub const ATMEL_HLCDC_LAYER_CHER: c_uint = 0x0;
pub const ATMEL_HLCDC_LAYER_CHDR: c_uint = 0x4;
pub const ATMEL_HLCDC_LAYER_CHSR: c_uint = 0x8;

pub const ATMEL_HLCDC_LAYER_IER: c_uint = 0xc;
pub const ATMEL_HLCDC_LAYER_IDR: c_uint = 0x10;
pub const ATMEL_HLCDC_LAYER_IMR: c_uint = 0x14;
pub const ATMEL_HLCDC_LAYER_ISR: c_uint = 0x18;

pub const ATMEL_HLCDC_LAYER_DMA_CFG: c_int = 0;

pub const ATMEL_HLCDC_LAYER_FORMAT_CFG: c_int = 1;

pub const ATMEL_HLCDC_LAYER_GA_SHIFT: c_int = 16;

pub const ATMEL_HLCDC_LAYER_MAX_PLANES: c_int = 3;

pub const ATMEL_HLCDC_CLUT_SIZE: c_int = 256;
pub const ATMEL_HLCDC_MAX_LAYERS: c_int = 6;
// XLCDC controller specific registers
pub const ATMEL_XLCDC_LAYER_ENR: c_uint = 0x10;

pub const ATMEL_XLCDC_LAYER_IER: c_uint = 0x0;
pub const ATMEL_XLCDC_LAYER_IDR: c_uint = 0x4;
pub const ATMEL_XLCDC_LAYER_ISR: c_uint = 0xc;

pub const ATMEL_XLCDC_LAYER_DMA_CFG: c_int = 0;

pub const ATMEL_XLCDC_LAYER_A0_SHIFT: c_int = 16;

//
// Atmel HLCDC Layer registers layout structure
//
// Each HLCDC layer has its own register organization and a given register
// can be placed differently on 2 different layers depending on its
// capabilities.
// This structure stores common registers layout for a given layer and is
// used by HLCDC layer code to choose the appropriate register to write to
// or to read from.
//
// For all fields, a value of zero means "unsupported".
//
// See Atmel's datasheet for a detailled description of these registers.
//
// @xstride: xstride registers
// @pstride: pstride registers
// @pos: position register
// @size: displayed size register
// @memsize: memory size register
// @default_color: default color register
// @chroma_key: chroma key register
// @chroma_key_mask: chroma key mask register
// @general_config: general layer config register
// @sacler_config: scaler factors register
// @phicoeffs: X/Y PHI coefficient registers
// @disc_pos: discard area position register
// @disc_size: discard area size register
// @csc: color space conversion register
// @vxs_config: vertical scalar filter taps control register
// @hxs_config: horizontal scalar filter taps control register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_layer_cfg_layout {
    pub xstride: [c_int; ATMEL_HLCDC_LAYER_MAX_PLANES],
    pub pstride: [c_int; ATMEL_HLCDC_LAYER_MAX_PLANES],
    pub pos: c_int,
    pub size: c_int,
    pub memsize: c_int,
    pub default_color: c_int,
    pub chroma_key: c_int,
    pub chroma_key_mask: c_int,
    pub general_config: c_int,
    pub scaler_config: c_int,
    pub x: c_int,
    pub y: c_int,
    pub phicoeffs: },
    pub disc_pos: c_int,
    pub disc_size: c_int,
    pub csc: c_int,
    pub vxs_config: c_int,
    pub hxs_config: c_int,
}

//
// Atmel HLCDC DMA descriptor structure
//
// This structure is used by the HLCDC DMA engine to schedule a DMA transfer.
//
// The structure fields must remain in this specific order, because they're
// used by the HLCDC DMA engine, which expect them in this order.
// HLCDC DMA descriptors must be aligned on 64 bits.
//
// @addr: buffer DMA address
// @ctrl: DMA transfer options
// @next: next DMA descriptor to fetch
// @self: descriptor DMA address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_dma_channel_dscr {
    pub addr: dma_addr_t,
    pub ctrl: u32,
    pub next: dma_addr_t,
    pub self: dma_addr_t,
    pub __aligned(sizeof(u64)): },
//
// Atmel HLCDC layer types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atmel_hlcdc_layer_type {
    ATMEL_HLCDC_NO_LAYER,
    ATMEL_HLCDC_BASE_LAYER,
    ATMEL_HLCDC_OVERLAY_LAYER,
    ATMEL_HLCDC_CURSOR_LAYER,
    ATMEL_HLCDC_PP_LAYER,
}

//
// Atmel HLCDC Supported formats structure
//
// This structure list all the formats supported by a given layer.
//
// @nformats: number of supported formats
// @formats: supported formats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_formats {
    pub nformats: c_int,
    pub formats: *mut u32,
}

//
// Atmel HLCDC Layer description structure
//
// This structure describes the capabilities provided by a given layer.
//
// @name: layer name
// @type: layer type
// @id: layer id
// @regs_offset: offset of the layer registers from the HLCDC registers base
// @cfgs_offset: CFGX registers offset from the layer registers base
// @formats: supported formats
// @layout: config registers layout
// @max_width: maximum width supported by this layer (0 means unlimited)
// @max_height: maximum height supported by this layer (0 means unlimited)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_layer_desc {
    pub name: *const c_char,
    pub type: atmel_hlcdc_layer_type,
    pub id: c_int,
    pub regs_offset: c_int,
    pub cfgs_offset: c_int,
    pub clut_offset: c_int,
    pub formats: *mut atmel_hlcdc_formats,
    pub layout: atmel_hlcdc_layer_cfg_layout,
    pub max_width: c_int,
    pub max_height: c_int,
}

//
// Atmel HLCDC Layer.
//
// A layer can be a DRM plane of a post processing layer used to render
// HLCDC composition into memory.
//
// @desc: layer description
// @regmap: pointer to the HLCDC regmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_layer {
    pub desc: *const atmel_hlcdc_layer_desc,
    pub regmap: *mut regmap,
}

//
// Atmel HLCDC Plane.
//
// @base: base DRM plane structure
// @layer: HLCDC layer structure
// @properties: pointer to the property definitions structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_plane {
    pub base: drm_plane,
    pub layer: atmel_hlcdc_layer,
}

extern "C" {
    pub fn container_of(_arg: p, atmel_hlcdc_plane: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: layer, atmel_hlcdc_plane: struct, _arg: layer) -> return;
}
//
// struct atmel_hlcdc_dc - Atmel HLCDC Display Controller.
// @desc: HLCDC Display Controller description
// @dscrpool: DMA coherent pool used to allocate DMA descriptors
// @hlcdc: pointer to the atmel_hlcdc structure provided by the MFD device
// @crtc: CRTC provided by the display controller
// @layers: active HLCDC layers
// @suspend: used to store the HLCDC state when entering suspend
// @suspend.imr: used to read/write LCDC Interrupt Mask Register
// @suspend.state: Atomic commit structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_dc {
    pub desc: *const atmel_hlcdc_dc_desc,
    pub dscrpool: *mut dma_pool,
    pub hlcdc: *mut atmel_hlcdc,
    pub crtc: *mut drm_crtc,
    pub dev: drm_device,
    pub layers: [*mut atmel_hlcdc_layer; ATMEL_HLCDC_MAX_LAYERS],
    pub imr: u32,
    pub state: *mut drm_atomic_commit,
    pub suspend: },
}

//
// struct atmel_lcdc_dc_ops - describes atmel_lcdc ops group
// to differentiate HLCDC and XLCDC IP code support
// @plane_setup_scaler: update the vertical and horizontal scaling factors
// @update_lcdc_buffers: update the each LCDC layers DMA registers
// @lcdc_atomic_disable: disable LCDC interrupts and layers
// @lcdc_update_general_settings: update each LCDC layers general
// configuration register
// @lcdc_atomic_update: enable the LCDC layers and interrupts
// @lcdc_csc_init: update the color space conversion co-efficient of
// High-end overlay register
// @lcdc_irq_dbg: to raise alert incase of interrupt overrun in any LCDC layer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_lcdc_dc_ops {
    pub state): *mut atmel_hlcdc_plane_state,
    pub i): u32 sr, int,
    pub dc): *mut atmel_hlcdc_dc,
    pub state): *mut atmel_hlcdc_plane_state,
    pub dc): *mut atmel_hlcdc_dc,
    pub desc): *const atmel_hlcdc_layer_desc,
    pub desc): *const atmel_hlcdc_layer_desc,
}

//
// Atmel HLCDC Display Controller description structure.
//
// This structure describes the HLCDC IP capabilities and depends on the
// HLCDC IP version (or Atmel SoC family).
//
// @min_width: minimum width supported by the Display Controller
// @min_height: minimum height supported by the Display Controller
// @max_width: maximum width supported by the Display Controller
// @max_height: maximum height supported by the Display Controller
// @max_spw: maximum vertical/horizontal pulse width
// @max_vpw: maximum vertical back/front porch width
// @max_hpw: maximum horizontal back/front porch width
// @conflicting_output_formats: true if RGBXXX output formats conflict with
// each other.
// @fixed_clksrc: true if clock source is fixed
// @is_xlcdc: true if XLCDC IP is supported
// @layers: a layer description table describing available layers
// @nlayers: layer description table size
// @ops: atmel lcdc dc ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_dc_desc {
    pub min_width: c_int,
    pub min_height: c_int,
    pub max_width: c_int,
    pub max_height: c_int,
    pub max_spw: c_int,
    pub max_vpw: c_int,
    pub max_hpw: c_int,
    pub conflicting_output_formats: bool,
    pub fixed_clksrc: bool,
    pub is_xlcdc: bool,
    pub layers: *const atmel_hlcdc_layer_desc,
    pub nlayers: c_int,
    pub ops: *const atmel_lcdc_dc_ops,
}

extern "C" {
    pub fn atmel_hlcdc_create_planes(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn atmel_hlcdc_plane_irq(plane: *mut atmel_hlcdc_plane);
}
extern "C" {
    pub fn atmel_hlcdc_plane_prepare_disc_area(c_state: *mut drm_crtc_state) -> c_int;
}
extern "C" {
    pub fn atmel_hlcdc_plane_prepare_ahb_routing(c_state: *mut drm_crtc_state) -> c_int;
}
extern "C" {
    pub fn atmel_hlcdc_crtc_irq(c: *mut drm_crtc);
}
extern "C" {
    pub fn atmel_hlcdc_crtc_create(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn atmel_hlcdc_create_outputs(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn atmel_hlcdc_encoder_get_bus_fmt(encoder: *mut drm_encoder) -> c_int;
}
