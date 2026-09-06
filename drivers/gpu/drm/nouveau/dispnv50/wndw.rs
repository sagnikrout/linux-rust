//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/wndw.h
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
pub struct nv50_wndw_ctxdma {
    pub head: list_head,
    pub object: nvif_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_wndw {
    pub func: *const nv50_wndw_func,
    pub immd: *const nv50_wimm_func,
    pub id: c_int,
    pub interlock: nv50_disp_interlock,
    pub parent: *mut nvif_object,
    pub list: list_head,
    pub ctxdma: },
    pub plane: drm_plane,
    pub ilut: nv50_lut,
    pub wndw: nv50_dmac,
    pub wimm: nv50_dmac,
    pub ntfy: u16,
    pub sema: u16,
    pub data: u32,
}

extern "C" {
    pub fn nv50_wndw_ntfy_enable(: *mut nv50_wndw, : *mut nv50_wndw_atom);
}
extern "C" {
    pub fn nv50_wndw_wait_armed(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn nv50_wndw_default_state(wndw: *mut nv50_wndw);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_wndw_func {
    pub asyh): *mut nv50_head_atom,
    pub asyh): *mut nv50_head_atom,
    pub asyw): *mut nv50_wndw_atom,
    pub ): *mut *mut *mut int (sema_set)(struct nv50_wndw , struct nv50_wndw_atom,
    pub ): *mut *mut int (sema_clr)(struct nv50_wndw,
    pub offset): *mut *mut *mut void (ntfy_reset)(struct nouveau_bo , u32,
    pub ): *mut *mut *mut int (ntfy_set)(struct nv50_wndw , struct nv50_wndw_atom,
    pub ): *mut *mut int (ntfy_clr)(struct nv50_wndw,
    pub ): *mut nvif_device,
    pub size): *mut *mut *mut *mut void (ilut)(struct nv50_wndw wndw, struct nv50_wndw_atom asyh, int,
    pub ): *const drm_color_ctm,
    pub ): *mut *mut *mut int (csc_set)(struct nv50_wndw , struct nv50_wndw_atom,
    pub ): *mut *mut int (csc_clr)(struct nv50_wndw,
    pub ilut_identity: bool,
    pub ilut_size: c_int,
    pub olut_core: bool,
    pub ): *mut *mut *mut int (xlut_set)(struct nv50_wndw , struct nv50_wndw_atom,
    pub ): *mut *mut int (xlut_clr)(struct nv50_wndw,
    pub ): *mut *mut *mut int (image_set)(struct nv50_wndw , struct nv50_wndw_atom,
    pub ): *mut *mut int (image_clr)(struct nv50_wndw,
    pub ): *mut *mut *mut int (scale_set)(struct nv50_wndw , struct nv50_wndw_atom,
    pub ): *mut *mut *mut int (blend_set)(struct nv50_wndw , struct nv50_wndw_atom,
    pub blend_modes: c_uint,
    pub interlock): *mut *mut *mut int (update)(struct nv50_wndw , u32,
}

extern "C" {
    pub fn base507c_ntfy_reset(: *mut nouveau_bo, _arg: u32);
}
extern "C" {
    pub fn base507c_ntfy_set(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn base507c_ntfy_clr(: *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn base507c_ntfy_wait_begun(: *mut nouveau_bo, _arg: u32, : *mut nvif_device) -> c_int;
}
extern "C" {
    pub fn base507c_image_clr(: *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn base507c_update(: *mut nv50_wndw, : *mut u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_wimm_func {
    pub ): *mut *mut *mut int (point)(struct nv50_wndw , struct nv50_wndw_atom,
    pub interlock): *mut *mut *mut int (update)(struct nv50_wndw , u32,
}

extern "C" {
    pub fn curs507a_space(: *mut nv50_wndw) -> bool;
}
extern "C" {
    pub fn wndwc37e_sema_set(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn wndwc37e_sema_clr(: *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn wndwc37e_ntfy_set(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn wndwc37e_ntfy_clr(: *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn wndwc37e_image_clr(: *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn wndwc37e_blend_set(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn wndwc37e_update(: *mut nv50_wndw, : *mut u32) -> c_int;
}
extern "C" {
    pub fn wndwc57e_ilut(: *mut nv50_wndw, : *mut nv50_wndw_atom, _arg: c_int);
}
extern "C" {
    pub fn wndwc57e_ilut_set(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn wndwc57e_ilut_clr(: *mut nv50_wndw) -> c_int;
}
extern "C" {
    pub fn wndwc57e_csc_set(: *mut nv50_wndw, : *mut nv50_wndw_atom) -> c_int;
}
extern "C" {
    pub fn wndwc57e_csc_clr(: *mut nv50_wndw) -> c_int;
}
// A set of blend modes supported by all wndws

