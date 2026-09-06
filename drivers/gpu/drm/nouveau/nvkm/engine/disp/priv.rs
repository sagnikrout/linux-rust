//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/disp/priv.h
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


// SPDX-License-Identifier: MIT

extern "C" {
    pub fn nvkm_disp_vblank(: *mut nvkm_disp, head: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_disp_func {
    pub ): *mut *mut void (dtor)(struct nvkm_disp,
    pub ): *mut *mut int (oneinit)(struct nvkm_disp,
    pub ): *mut *mut int (init)(struct nvkm_disp,
    pub suspend): *mut *mut *mut void (fini)(struct nvkm_disp , bool,
    pub ): *mut *mut void (intr)(struct nvkm_disp,
    pub chid): *mut *mut *mut void (intr_error)(struct nvkm_disp , int,
    pub ): *mut *mut void (super)(struct work_struct,
    pub uevent: *const nvkm_event_func,
    pub mask): *mut *mut *mut int (cnt)(struct nvkm_disp , unsigned long,
    pub id): *mut *mut *mut int (new)(struct nvkm_disp , int,
    pub pior: } wndw, head, dac, sor,,
// Register programming that the GSP-RM display path (rm/r535) needs from
// the chip, everything else on that path goes through RM. The hooks are
// called unconditionally and the head table is handed to nvkm_head_new_().
//
    pub ): *mut *mut irqreturn_t (intr)(struct nvkm_inth,
// Head-timing interrupts arrive on a second DISP vector.
    pub intr_low_latency: bool,
    pub head: *const nvkm_head_func,
    pub enable): *mut *mut *mut void (hdmi_gcp)(struct nvkm_ior , int head, bool,
    pub size): *mut *mut *mut *mut void (hdmi_infoframe_avi)(struct nvkm_ior , int head, void data, u32,
    pub size): *mut *mut *mut *mut void (hdmi_infoframe_vsi)(struct nvkm_ior , int head, void data, u32,
    pub gsp: },
    pub ramht_size: u16,
    pub root: nvkm_sclass,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_disp_user {
    pub base: nvkm_sclass,
    pub ): *mut nvkm_object,
    pub chan: *const nvkm_disp_chan_user,
    pub user: [}; ],
}

extern "C" {
    pub fn nv50_disp_oneinit(: *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn nv50_disp_init(: *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn nv50_disp_fini(: *mut nvkm_disp, suspend: bool);
}
extern "C" {
    pub fn nv50_disp_intr(: *mut nvkm_disp);
}
extern "C" {
    pub fn nv50_disp_super(: *mut work_struct);
}
extern "C" {
    pub fn nv50_disp_super_1(: *mut nvkm_disp);
}
extern "C" {
    pub fn nv50_disp_super_1_0(: *mut nvkm_disp, : *mut nvkm_head);
}
extern "C" {
    pub fn nv50_disp_super_2_0(: *mut nvkm_disp, : *mut nvkm_head);
}
extern "C" {
    pub fn nv50_disp_super_2_1(: *mut nvkm_disp, : *mut nvkm_head);
}
extern "C" {
    pub fn nv50_disp_super_2_2(: *mut nvkm_disp, : *mut nvkm_head);
}
extern "C" {
    pub fn nv50_disp_super_3_0(: *mut nvkm_disp, : *mut nvkm_head);
}
extern "C" {
    pub fn gf119_disp_init(: *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn gf119_disp_fini(: *mut nvkm_disp, suspend: bool);
}
extern "C" {
    pub fn gf119_disp_intr(: *mut nvkm_disp);
}
extern "C" {
    pub fn gf119_disp_super(: *mut work_struct);
}
extern "C" {
    pub fn gf119_disp_intr_error(: *mut nvkm_disp, _arg: c_int);
}
extern "C" {
    pub fn gv100_disp_fini(: *mut nvkm_disp, suspend: bool);
}
extern "C" {
    pub fn gv100_disp_intr(: *mut nvkm_disp);
}
extern "C" {
    pub fn gv100_disp_super(: *mut work_struct);
}
extern "C" {
    pub fn gv100_disp_wndw_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn gv100_disp_caps_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn tu102_disp_init(: *mut nvkm_disp) -> c_int;
}
extern "C" {
    pub fn tu102_disp_intr(: *mut nvkm_inth) -> irqreturn_t;
}
extern "C" {
    pub fn nv50_disp_dptmds_war_2(: *mut nvkm_disp, : *mut dcb_output);
}
extern "C" {
    pub fn nv50_disp_dptmds_war_3(: *mut nvkm_disp, : *mut dcb_output);
}
extern "C" {
    pub fn nv50_disp_update_sppll1(: *mut nvkm_disp);
}
extern "C" {
    pub fn nv50_disp_chan_uevent_send(: *mut nvkm_disp, _arg: c_int);
}
extern "C" {
    pub fn nvkm_udisp_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_uconn_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_uoutp_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
extern "C" {
    pub fn nvkm_uhead_new(: *const nvkm_oclass, : *mut c_void, _arg: u32, : *mut nvkm_object) -> c_int;
}
