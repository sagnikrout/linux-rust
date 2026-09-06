//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/disp.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_disp {
    pub disp: *mut nvif_disp,
    pub core: *mut nv50_core,
    pub caps: nvif_object,

    pub sync: *mut nouveau_bo,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_disp_interlock {
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nv50_disp_interlock_type {
    NV50_DISP_INTERLOCK_CORE = 0,
    NV50_DISP_INTERLOCK_CURS,
    NV50_DISP_INTERLOCK_BASE,
    NV50_DISP_INTERLOCK_OVLY,
    NV50_DISP_INTERLOCK_WNDW,
    NV50_DISP_INTERLOCK_WIMM,
    NV50_DISP_INTERLOCK__SIZE
    } type;
    u32 data;
    u32 wimm;
}

    pub u32): *mut *mut void corec37d_ntfy_init(struct nouveau_bo ,,
    pub ): *mut *mut void head907d_olut_load(struct drm_color_lut , int size, void __iomem,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_chan {
    pub user: nvif_object,
    pub device: *mut nvif_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_dmac {
    pub base: nv50_chan,
    pub push: nvif_push,
    pub sync: nvif_object,
    pub vram: nvif_object,
    pub cur: u32,
    pub put: u32,
    pub max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_outp_atom {
    pub head: list_head,
    pub encoder: *mut drm_encoder,
    pub disabled: bool,
    pub enabled: bool,
#[repr(C)]
#[derive(Copy, Clone)]
pub union nv50_outp_atom_mask {
    pub ctrl:1: bool,
}

extern "C" {
    pub fn nv50_dmac_destroy(: *mut nv50_dmac);
}
//
// For normal encoders this just returns the encoder. For active MST encoders,
// this returns the real outp that's driving displays on the topology.
// Inactive MST encoders return NULL, since they would have no real outp to
// return anyway.
//
