//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/disp/outp.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_outp {
    pub func: *const nvkm_outp_func,
    pub disp: *mut nvkm_disp,
    pub index: c_int,
    pub info: dcb_output,
    pub i2c: *mut nvkm_i2c_bus,
    pub head: list_head,
    pub conn: *mut nvkm_conn,
    pub identity: bool,
// Assembly state.
pub const NVKM_OUTP_PRIV: c_int = 1;
pub const NVKM_OUTP_USER: c_int = 2;
    pub acquired:2: u8,
    pub ior: *mut nvkm_ior,
    pub dual: bool,
    pub bpc8: bool,
    pub lvds: },
    pub info: nvbios_dpout,
    pub version: u8,
    pub mst: bool,
    pub increased_wm: bool,
    pub aux: *mut nvkm_i2c_aux,
    pub enabled: bool,
    pub aux_pwr: bool,
    pub aux_pwr_pu: bool,
    pub lttpr: [u8; 6],
    pub lttprs: u8,
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub /: *mut *mut int dpcd; / -1, or index into SUPPORTED_LINK_RATES table,
    pub rate: u32,
    pub rate: [}; 8],
    pub rates: c_int,
    pub mutex: mutex,
    pub nr: u8,
    pub bw: u8,
    pub mst: bool,
    pub post_adj: bool,
    pub lt: },
    pub dp: },
}

extern "C" {
    pub fn nvkm_outp_new(: *mut nvkm_disp, index: c_int, : *mut dcb_output, : *mut nvkm_outp) -> c_int;
}
extern "C" {
    pub fn nvkm_outp_del(: *mut nvkm_outp);
}
extern "C" {
    pub fn nvkm_outp_init(: *mut nvkm_outp);
}
extern "C" {
    pub fn nvkm_outp_fini(: *mut nvkm_outp);
}
extern "C" {
    pub fn nvkm_outp_detect(: *mut nvkm_outp) -> c_int;
}
extern "C" {
    pub fn nvkm_outp_acquire(: *mut nvkm_outp, hda: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_outp_acquire_or(: *mut nvkm_outp, user: u8, hda: bool) -> c_int;
}
extern "C" {
    pub fn nvkm_outp_acquire_ior(: *mut nvkm_outp, user: u8, : *mut nvkm_ior) -> c_int;
}
extern "C" {
    pub fn nvkm_outp_release(: *mut nvkm_outp);
}
extern "C" {
    pub fn nvkm_outp_release_or(: *mut nvkm_outp, user: u8);
}
extern "C" {
    pub fn nvkm_outp_bl_get(: *mut nvkm_outp) -> c_int;
}
extern "C" {
    pub fn nvkm_outp_bl_set(: *mut nvkm_outp, level: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_outp_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_outp,
    pub ): *mut *mut void (init)(struct nvkm_outp,
    pub ): *mut *mut void (fini)(struct nvkm_outp,
    pub ): *mut *mut int (detect)(struct nvkm_outp,
    pub size): *mut *mut *mut *mut int (edid_get)(struct nvkm_outp , u8 data, u16,
    pub ): *mut *mut *mut nvkm_ior (inherit)(nvkm_outp,
    pub hda): *mut *mut *mut int (acquire)(struct nvkm_outp , bool,
    pub ): *mut *mut void (release)(struct nvkm_outp,
    pub ): *mut *mut int (get)(struct nvkm_outp,
    pub level): *mut *mut *mut int (set)(struct nvkm_outp , int,
    pub bl: },
    pub pu): *mut *mut *mut int (aux_pwr)(struct nvkm_outp , bool,
    pub size): *mut *mut *mut *mut int (aux_xfer)(struct nvkm_outp , u8 type, u32 addr, u8 data, u8,
    pub ): *mut *mut int (rates)(struct nvkm_outp,
    pub retrain): *mut *mut *mut int (train)(struct nvkm_outp , bool,
    pub vs[4]): *mut *mut *mut int (drive)(struct nvkm_outp , u8 lanes, u8 pe[4], u8,
    pub id): *mut *mut *mut int (mst_id_get)(struct nvkm_outp , u32,
    pub id): *mut *mut *mut int (mst_id_put)(struct nvkm_outp , u32,
    pub dp: },
}

