//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sun4i/sunxi_engine.h
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
// Copyright (C) 2017 Icenowy Zheng <icenowy@aosc.io>
//
// struct sunxi_engine_ops - helper operations for sunXi engines
//
// These hooks are used by the common part of the DRM driver to
// implement the proper behaviour.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_engine_ops {
//
// @atomic_begin:
//
// This callback allows to prepare our engine for an atomic
// update. This is mirroring the
// &drm_crtc_helper_funcs.atomic_begin callback, so any
// documentation there applies.
//
// This function is optional.
//
    pub old_state): *mut drm_crtc_state,
//
// @atomic_check:
//
// This callback allows to validate plane-update related CRTC
// constraints specific to engines. This is mirroring the
// &drm_crtc_helper_funcs.atomic_check callback, so any
// documentation there applies.
//
// This function is optional.
//
// RETURNS:
//
// 0 on success or a negative error code.
//
    pub state): *mut drm_crtc_state,
//
// @commit:
//
// This callback will trigger the hardware switch to commit
// the new configuration that has been setup during the next
// vblank period.
//
// This function is optional.
//
    pub state): *mut drm_atomic_commit,
//
// @layers_init:
//
// This callback is used to allocate, initialize and register
// the layers supported by that engine.
//
// This function is mandatory.
//
// RETURNS:
//
// The array of struct drm_plane backing the layers, or an
// error pointer on failure.
//
    pub engine): *mut sunxi_engine,
//
// @apply_color_correction:
//
// This callback will enable the color correction in the
// engine. This is useful only for the composite output.
//
// This function is optional.
//
    pub engine): *mut *mut void (apply_color_correction)(struct sunxi_engine,
//
// @disable_color_correction:
//
// This callback will stop the color correction in the
// engine. This is useful only for the composite output.
//
// This function is optional.
//
    pub engine): *mut *mut void (disable_color_correction)(struct sunxi_engine,
//
// @vblank_quirk:
//
// This callback is used to implement engine-specific
// behaviour part of the VBLANK event. It is run with all the
// constraints of an interrupt (can't sleep, all local
// interrupts disabled) and therefore should be as fast as
// possible.
//
// This function is optional.
//
    pub engine): *mut *mut void (vblank_quirk)(struct sunxi_engine,
//
// @mode_set:
//
// This callback is used to set mode related parameters
// like interlacing, screen size, etc. once per mode set.
//
// This function is optional.
//
    pub mode): *const drm_display_mode,
}

//
// struct sunxi_engine - the common parts of an engine for sun4i-drm driver
// @ops:	the operations of the engine
// @node:	the of device node of the engine
// @regs:	the regmap of the engine
// @id:		the id of the engine (-1 if not used)
// @list:	engine list management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_engine {
    pub ops: *const sunxi_engine_ops,
    pub node: *mut device_node,
    pub regs: *mut regmap,
    pub id: c_int,
    pub list: list_head,
}

//
// sunxi_engine_commit() - commit all changes of the engine
// @engine:	pointer to the engine
// @crtc:	pointer to crtc the engine is associated with
// @state:	atomic state
//
// sunxi_engine_layers_init() - Create planes (layers) for the engine
// @drm:	pointer to the drm_device for which planes will be created
// @engine:	pointer to the engine
//
// Returns: The array of struct drm_plane backing the layers, or an
// error pointer on failure.
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
//
// sunxi_engine_apply_color_correction - Apply the RGB2YUV color correction
// @engine:	pointer to the engine
//
// This functionality is optional for an engine, however, if the engine is
// intended to be used with TV Encoder, the output will be incorrect
// without the color correction, due to TV Encoder expects the engine to
// output directly YUV signal.
//
// sunxi_engine_disable_color_correction - Disable the color space correction
// @engine:	pointer to the engine
//
// This function is paired with apply_color_correction().
//
// sunxi_engine_mode_set - Inform engine of a new mode
// @engine:	pointer to the engine
// @mode:	new mode
//
// Engine can use this functionality to set specifics once per mode change.
//
