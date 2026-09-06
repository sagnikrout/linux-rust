//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp5/mdp5_kms.h
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_kms {
    pub base: mdp_kms,
    pub dev: *mut drm_device,
    pub pdev: *mut platform_device,
    pub num_hwpipes: unsigned,
    pub hwpipes: [*mut mdp5_hw_pipe; SSPP_MAX],
    pub num_hwmixers: unsigned,
    pub hwmixers: [*mut mdp5_hw_mixer; 8],
    pub num_intfs: unsigned,
    pub intfs: [*mut mdp5_interface; 5],
    pub cfg: *mut mdp5_cfg_handler,
    pub /: *mut *mut uint32_t caps; / MDP capabilities (MDP_CAP_XXX bits),
//
// Global private object state, Do not access directly, use
// mdp5_global_get_state()
//
    pub glob_state: drm_private_obj,
    pub smp: *mut mdp5_smp,
    pub ctlm: *mut mdp5_ctl_manager,
// io/register spaces:
    pub mmio: *mut void __iomem,
    pub axi_clk: *mut clk,
    pub ahb_clk: *mut clk,
    pub core_clk: *mut clk,
    pub lut_clk: *mut clk,
    pub tbu_clk: *mut clk,
    pub tbu_rt_clk: *mut clk,
    pub vsync_clk: *mut clk,
//
// lock to protect access to global resources: ie., following register:
// - REG_MDP5_DISP_INTF_SEL
//
    pub resource_lock: spinlock_t,
    pub rpm_enabled: bool,
    pub error_handler: mdp_irq,
    pub enable_count: c_int,
}

// Global private object state for tracking resources that are shared across
// multiple kms objects (planes/crtcs/etc).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_global_state {
    pub base: drm_private_state,
    pub state: *mut drm_atomic_commit,
    pub mdp5_kms: *mut mdp5_kms,
    pub hwpipe: mdp5_hw_pipe_state,
    pub hwmixer: mdp5_hw_mixer_state,
    pub smp: mdp5_smp_state,
}

extern "C" {
    pub fn mdp5_get_existing_global_state(mdp5_kms: *mut mdp5_kms) -> *mut mdp5_global_state;
}
extern "C" {
    pub fn mdp5_get_global_state(s: *mut drm_atomic_commit) -> *mut mdp5_global_state __must_check;
}
// Atomic plane state.  Subclasses the base drm_plane_state in order to
// track assigned hwpipe and hw specific state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_plane_state {
    pub base: drm_plane_state,
    pub hwpipe: *mut mdp5_hw_pipe,
    pub /: *mut *mut *mut mdp5_hw_pipe r_hwpipe; / right hwpipe,
// assigned by crtc blender
    pub stage: mdp_mixer_stage_id,
// whether attached CRTC needs pixel data explicitly flushed to
// display (ex. DSI command mode display)
//
    pub needs_dirtyfb: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_pipeline {
    pub intf: *mut mdp5_interface,
    pub mixer: *mut mdp5_hw_mixer,
    pub /: *mut *mut *mut mdp5_hw_mixer r_mixer; / right mixer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_crtc_state {
    pub base: drm_crtc_state,
    pub ctl: *mut mdp5_ctl,
    pub pipeline: mdp5_pipeline,
// these are derivatives of intf/mixer state in mdp5_pipeline
    pub vblank_irqmask: u32,
    pub err_irqmask: u32,
    pub pp_done_irqmask: u32,
    pub cmd_mode: bool,
// should we not write CTL[n].START register on flush?  If the
// encoder has changed this is set to true, since encoder->enable()
// is called after crtc state is committed, but we only want to
// write the CTL[n].START register once.  This lets us defer
// writing CTL[n].START until encoder->enable()
//
    pub defer_start: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp5_intf_mode {
    MDP5_INTF_MODE_NONE = 0,

// Modes used for DSI interface (INTF_DSI type):
    MDP5_INTF_DSI_MODE_VIDEO,
    MDP5_INTF_DSI_MODE_COMMAND,

// Modes used for WB interface (INTF_WB type):
    MDP5_INTF_WB_MODE_BLOCK,
    MDP5_INTF_WB_MODE_LINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_interface {
    pub idx: c_int,
    pub /: *mut *mut int num; / display interface number,
    pub type: mdp5_intf_type,
    pub mode: mdp5_intf_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp5_encoder {
    pub base: drm_encoder,
    pub /: *mut *mut *mut spinlock_t intf_lock; / protect REG_MDP5_INTF_ registers,
    pub enabled: bool,
    pub bsc: u32,
    pub intf: *mut mdp5_interface,
    pub ctl: *mut mdp5_ctl,
}

extern "C" {
    pub fn readl(reg: mdp5_kms->mmio +) -> return;
}

//
// In case of DSI Command Mode, the Ping Pong's read pointer IRQ
// acts as a Vblank signal. The Ping Pong buffer used is bound to
// layer mixer.
//
extern "C" {
    pub fn mdp5_irq_preinstall(kms: *mut msm_kms);
}
extern "C" {
    pub fn mdp5_irq_postinstall(kms: *mut msm_kms) -> c_int;
}
extern "C" {
    pub fn mdp5_irq_uninstall(kms: *mut msm_kms);
}
extern "C" {
    pub fn mdp5_irq(kms: *mut msm_kms) -> irqreturn_t;
}
extern "C" {
    pub fn mdp5_enable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn mdp5_disable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc);
}
extern "C" {
    pub fn mdp5_irq_domain_init(mdp5_kms: *mut mdp5_kms) -> c_int;
}
extern "C" {
    pub fn mdp5_irq_domain_fini(mdp5_kms: *mut mdp5_kms);
}
extern "C" {
    pub fn mdp5_plane_get_flush(plane: *mut drm_plane) -> u32;
}
extern "C" {
    pub fn mdp5_plane_pipe(plane: *mut drm_plane) -> mdp5_pipe;
}
extern "C" {
    pub fn mdp5_plane_right_pipe(plane: *mut drm_plane) -> mdp5_pipe;
}
extern "C" {
    pub fn mdp5_crtc_vblank(crtc: *mut drm_crtc) -> u32;
}
extern "C" {
    pub fn mdp5_crtc_set_pipeline(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn mdp5_crtc_wait_for_commit_done(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn mdp5_encoder_set_intf_mode(encoder: *mut drm_encoder, cmd_mode: bool);
}
extern "C" {
    pub fn mdp5_encoder_get_linecount(encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn mdp5_encoder_get_framecount(encoder: *mut drm_encoder) -> u32;
}

extern "C" {
    pub fn mdp5_cmd_encoder_disable(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn mdp5_cmd_encoder_enable(encoder: *mut drm_encoder);
}

