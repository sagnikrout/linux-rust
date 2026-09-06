//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_encoder.h
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
// Copyright (C) 2008 Maarten Maathuis.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const NV_DPMS_CLEARED: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_encoder {
    pub base: nouveau_i2c_encoder,
    pub dcb: *mut dcb_output,
    pub outp: nvif_outp,
    pub or: c_int,
    pub conn: *mut nouveau_connector,
    pub i2c: *mut i2c_adapter,
// different to drm_encoder.crtc, this reflects what's
// actually programmed on the hw, not the proposed crtc
    pub crtc: *mut drm_crtc,
    pub ctrl: u32,
// Protected by nouveau_drm.audio.lock
    pub enabled: bool,
    pub audio: },
    pub mode: drm_display_mode,
    pub last_dpms: c_int,
    pub restore: nv04_output_reg,
    pub enabled: bool,
    pub hdmi: },
    pub mstm: *mut nv50_mstm,
    pub caps: [u8; DP_LTTPR_COMMON_CAP_SIZE],
    pub nr: u8,
    pub lttpr: },
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub rate: [nvif_outp_dp_rate; 8],
    pub rate_nr: c_int,
    pub link_nr: c_int,
    pub link_bw: c_int,
    pub mst: bool,
    pub nr: u8,
    pub bw: u32,
    pub lt: },
// Protects DP state that needs to be accessed outside
// connector reprobing contexts
//
    pub hpd_irq_lock: mutex,
    pub downstream_ports: [u8; DP_MAX_DOWNSTREAM_PORTS],
    pub desc: drm_dp_desc,
    pub sink_count: u8,
    pub dp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_mstm {
    pub outp: *mut nouveau_encoder,
    pub mgr: drm_dp_mst_topology_mgr,
// Protected under nouveau_encoder->dp.hpd_irq_lock
    pub can_mst: bool,
    pub is_mst: bool,
    pub suspended: bool,
    pub modified: bool,
    pub disabled: bool,
    pub links: c_int,
}

extern "C" {
    pub fn container_of(_arg: slave, nouveau_encoder: struct, _arg: base) -> return;
}
// nouveau_dp.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nouveau_dp_status {
    NOUVEAU_DP_NONE,
    NOUVEAU_DP_SST,
    NOUVEAU_DP_MST,
}

extern "C" {
    pub fn nouveau_dp_detect(: *mut nouveau_connector, : *mut nouveau_encoder) -> c_int;
}
extern "C" {
    pub fn nouveau_dp_train(: *mut nouveau_encoder, mst: bool, khz: u32, bpc: u8) -> bool;
}
extern "C" {
    pub fn nouveau_dp_power_down(: *mut nouveau_encoder);
}
extern "C" {
    pub fn nouveau_dp_link_check(: *mut nouveau_connector) -> bool;
}
extern "C" {
    pub fn nouveau_dp_irq(: *mut work_struct);
}
extern "C" {
    pub fn nv50_mstm_detect(encoder: *mut nouveau_encoder) -> c_int;
}
extern "C" {
    pub fn nv50_mstm_remove(mstm: *mut nv50_mstm);
}
