//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sti/sti_compositor.h
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
//
// Copyright (C) STMicroelectronics SA 2014
// Authors: Benjamin Gaignard <benjamin.gaignard@st.com>
// Fabien Dessenne <fabien.dessenne@st.com>
// for STMicroelectronics.
//

pub const STI_MAX_MIXER: c_int = 2;
pub const STI_MAX_VID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sti_compositor_subdev_type {
    STI_MIXER_MAIN_SUBDEV,
    STI_MIXER_AUX_SUBDEV,
    STI_GPD_SUBDEV,
    STI_VID_SUBDEV,
    STI_CURSOR_SUBDEV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_compositor_subdev_descriptor {
    pub type: sti_compositor_subdev_type,
    pub id: c_int,
    pub offset: c_uint,
}

//
// STI Compositor data structure
//
// @nb_subdev: number of subdevices supported by the compositor
// @subdev_desc: subdev list description
//
pub const MAX_SUBDEV: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_compositor_data {
    pub nb_subdev: c_uint,
    pub subdev_desc: [sti_compositor_subdev_descriptor; MAX_SUBDEV],
}

//
// STI Compositor structure
//
// @dev: driver device
// @regs: registers (main)
// @data: device data
// @clk_compo_main: clock for main compo
// @clk_compo_aux: clock for aux compo
// @clk_pix_main: pixel clock for main path
// @clk_pix_aux: pixel clock for aux path
// @rst_main: reset control of the main path
// @rst_aux: reset control of the aux path
// @mixer: array of mixers
// @vid: array of vids
// @vtg: array of vtgs
// @vtg_vblank_nb: array of callbacks for VTG VSYNC notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_compositor {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub data: sti_compositor_data,
    pub clk_compo_main: *mut clk,
    pub clk_compo_aux: *mut clk,
    pub clk_pix_main: *mut clk,
    pub clk_pix_aux: *mut clk,
    pub rst_main: *mut reset_control,
    pub rst_aux: *mut reset_control,
    pub mixer: [*mut sti_mixer; STI_MAX_MIXER],
    pub vid: [*mut sti_vid; STI_MAX_VID],
    pub vtg: [*mut sti_vtg; STI_MAX_MIXER],
    pub vtg_vblank_nb: [notifier_block; STI_MAX_MIXER],
}
