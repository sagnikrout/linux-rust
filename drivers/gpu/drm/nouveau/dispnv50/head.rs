//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/head.h
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
pub struct nv50_head {
    pub func: *const nv50_head_func,
    pub disp: *mut nv50_disp,
    pub base: nouveau_crtc,
    pub crc: nv50_crc,
    pub olut: nv50_lut,
    pub msto: *mut nv50_msto,
}

extern "C" {
    pub fn nv50_head_flush_set(head: *mut nv50_head, asyh: *mut nv50_head_atom);
}
extern "C" {
    pub fn nv50_head_flush_set_wndw(head: *mut nv50_head, asyh: *mut nv50_head_atom);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_head_func {
    pub ): *mut *mut *mut int (view)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut *mut int (mode)(struct nv50_head , struct nv50_head_atom,
    pub int): *mut *mut *mut *mut bool (olut)(struct nv50_head , struct nv50_head_atom ,,
    pub size): *mut *mut bool (ilut_check)(int,
    pub olut_identity: bool,
    pub olut_size: c_int,
    pub ): *mut *mut *mut int (olut_set)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut int (olut_clr)(struct nv50_head,
    pub ): *mut *mut *mut void (core_calc)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut *mut int (core_set)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut int (core_clr)(struct nv50_head,
    pub ): *mut nv50_head_atom,
    pub ): *mut nv50_head_atom,
    pub ): *mut *mut *mut int (curs_set)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut int (curs_clr)(struct nv50_head,
    pub ): *mut *mut *mut int (base)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut *mut int (ovly)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut *mut int (dither)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut *mut int (procamp)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut *mut int (or)(struct nv50_head , struct nv50_head_atom,
    pub ): *mut *mut *mut void (static_wndw_map)(struct nv50_head , struct nv50_head_atom,
    pub display_id): *mut *mut *mut int (display_id)(struct nv50_head , u32,
}

extern "C" {
    pub fn head507d_view(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head507d_mode(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head507d_olut(: *mut nv50_head, : *mut nv50_head_atom, _arg: c_int) -> bool;
}
extern "C" {
    pub fn head507d_core_calc(: *mut nv50_head, : *mut nv50_head_atom);
}
extern "C" {
    pub fn head507d_core_clr(: *mut nv50_head) -> c_int;
}
extern "C" {
    pub fn head507d_base(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head507d_ovly(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head507d_dither(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head507d_procamp(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_view(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_mode(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_olut(: *mut nv50_head, : *mut nv50_head_atom, _arg: c_int) -> bool;
}
extern "C" {
    pub fn head907d_ilut_check(size: c_int) -> bool;
}
extern "C" {
    pub fn head907d_olut_set(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_olut_clr(: *mut nv50_head) -> c_int;
}
extern "C" {
    pub fn head907d_core_set(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_core_clr(: *mut nv50_head) -> c_int;
}
extern "C" {
    pub fn head907d_curs_set(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_curs_clr(: *mut nv50_head) -> c_int;
}
extern "C" {
    pub fn head907d_ovly(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_procamp(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn head907d_or(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn headc37d_view(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn headc37d_curs_set(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn headc37d_curs_clr(: *mut nv50_head) -> c_int;
}
extern "C" {
    pub fn headc37d_dither(: *mut nv50_head, : *mut nv50_head_atom) -> c_int;
}
extern "C" {
    pub fn headc37d_static_wndw_map(: *mut nv50_head, : *mut nv50_head_atom);
}
extern "C" {
    pub fn headc57d_olut(: *mut nv50_head, : *mut nv50_head_atom, size: c_int) -> bool;
}
