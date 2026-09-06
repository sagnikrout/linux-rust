//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fb/priv.h
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
pub struct nvkm_fb_func {
    pub ): *mut *mut *mut void (dtor)(struct nvkm_fb,
    pub ): *mut *mut u32 (tags)(struct nvkm_fb,
    pub ): *mut *mut int (oneinit)(struct nvkm_fb,
    pub ): *mut *mut void (init)(struct nvkm_fb,
    pub ): *mut *mut void (init_remapper)(struct nvkm_fb,
    pub ): *mut *mut int (init_page)(struct nvkm_fb,
    pub ): *mut *mut void (init_unkn)(struct nvkm_fb,
    pub ): *mut *mut void (intr)(struct nvkm_fb,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fb_func_sysmem {
    pub ): *mut *mut void (flush_page_init)(struct nvkm_fb,
    pub sysmem: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fb_func_vidmem {
    pub ): *mut *mut u64 (size)(struct nvkm_fb,
    pub vidmem: },
    pub ): *mut *mut bool (scrub_required)(struct nvkm_fb,
    pub ): *mut *mut int (scrub)(struct nvkm_fb,
    pub vpr: },
    pub regions: c_int,
    pub ): *mut u32 pitch, u32 flags, struct nvkm_fb_tile,
    pub ): *mut nvkm_fb_tile,
    pub ): *mut *mut *mut void (fini)(struct nvkm_fb , int i, struct nvkm_fb_tile,
    pub ): *mut *mut *mut void (prog)(struct nvkm_fb , int i, struct nvkm_fb_tile,
    pub tile: },
    pub ): *mut *mut *mut int (ram_new)(struct nvkm_fb , struct nvkm_ram,
    pub default_bigpage: u8,
    pub clkgate_pack: *const nvkm_therm_clkgate_pack,
}

extern "C" {
    pub fn nvkm_fb_bios_memtype(: *mut nvkm_bios) -> c_int;
}
extern "C" {
    pub fn nv10_fb_tile_fini(: *mut nvkm_fb, i: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn nv10_fb_tile_prog(: *mut nvkm_fb, _arg: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn nv20_fb_tags(: *mut nvkm_fb) -> u32;
}
extern "C" {
    pub fn nv20_fb_tile_fini(: *mut nvkm_fb, i: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn nv20_fb_tile_prog(: *mut nvkm_fb, _arg: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn nv30_fb_init(: *mut nvkm_fb);
}
extern "C" {
    pub fn nv41_fb_init(: *mut nvkm_fb);
}
extern "C" {
    pub fn nv41_fb_tile_prog(: *mut nvkm_fb, _arg: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn nv44_fb_init(: *mut nvkm_fb);
}
extern "C" {
    pub fn nv44_fb_tile_prog(: *mut nvkm_fb, _arg: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn gf100_fb_oneinit(: *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gf100_fb_init_page(: *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gf100_fb_sysmem_flush_page_init(: *mut nvkm_fb);
}
extern "C" {
    pub fn gm200_fb_init_page(: *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gp100_fb_init_remapper(: *mut nvkm_fb);
}
extern "C" {
    pub fn gp100_fb_init_unkn(: *mut nvkm_fb);
}
extern "C" {
    pub fn gp102_fb_oneinit(: *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gp102_fb_vidmem_size(: *mut nvkm_fb) -> u64;
}
extern "C" {
    pub fn gp102_fb_vpr_scrub_required(: *mut nvkm_fb) -> bool;
}
extern "C" {
    pub fn gp102_fb_vpr_scrub(: *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gv100_fb_init_page(: *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn tu102_fb_vpr_scrub_required(: *mut nvkm_fb) -> bool;
}
extern "C" {
    pub fn ga102_fb_vidmem_size(: *mut nvkm_fb) -> u64;
}
