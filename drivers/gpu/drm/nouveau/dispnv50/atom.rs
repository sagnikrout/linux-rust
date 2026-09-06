//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/atom.h
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
pub struct nv50_atom {
    pub state: drm_atomic_commit,
    pub outp: list_head,
    pub lock_core: bool,
    pub flush_disable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_head_atom {
    pub state: drm_crtc_state,
    pub mask: u32,
    pub owned: u32,
    pub olut: u32,
    pub wndw: },
    pub iW: u16,
    pub iH: u16,
    pub oW: u16,
    pub oH: u16,
    pub view: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_head_mode {
    pub interlace: bool,
    pub clock: u32,
    pub active: u16,
    pub synce: u16,
    pub blanke: u16,
    pub blanks: u16,
    pub h: },
    pub active: u32,
    pub synce: u16,
    pub blanke: u16,
    pub blanks: u16,
    pub blank2s: u16,
    pub blank2e: u16,
    pub blankus: u16,
    pub v: },
    pub mode: },
    pub visible: bool,
    pub handle: u32,
    pub offset:40: u64,
    pub buffer:1: u8,
    pub mode:4: u8,
    pub size:11: u16,
    pub range:2: u8,
    pub output_mode:2: u8,
    pub ): *mut *mut *mut void (load)(struct drm_color_lut , int size, void __iomem,
    pub olut: },
    pub visible: bool,
    pub handle: u32,
    pub offset:40: u64,
    pub format: u8,
    pub kind:7: u8,
    pub layout:1: u8,
    pub blockh:4: u8,
    pub blocks:12: u16,
    pub pitch:20: u32,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub core: },
    pub visible: bool,
    pub handle: u32,
    pub offset:40: u64,
    pub layout:2: u8,
    pub format:8: u8,
    pub curs: },
    pub depth: u8,
    pub cpp: u8,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub base: },
    pub cpp: u8,
    pub ovly: },
    pub enable:1: bool,
    pub bits:2: u8,
    pub mode:4: u8,
    pub dither: },
    pub cos:12: u16,
    pub sin:12: u16,
    pub sat: },
    pub procamp: },
    pub nhsync:1: u8,
    pub nvsync:1: u8,
    pub depth:4: u8,
    pub crc_raster:2: u8,
    pub bpc: u8,
    pub or: },
    pub crc: nv50_crc_atom,
// Currently only used for MST
    pub pbn: c_int,
    pub tu:6: u8,
    pub dp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nv50_head_atom_mask {
    pub olut:1: bool,
    pub core:1: bool,
    pub curs:1: bool,
    pub view:1: bool,
    pub mode:1: bool,
    pub base:1: bool,
    pub ovly:1: bool,
    pub dither:1: bool,
    pub procamp:1: bool,
    pub crc:1: bool,
    pub or:1: bool,
}

extern "C" {
    pub fn nv50_head_atom(_arg: statec) -> return;
}
extern "C" {
    pub fn nv50_head_atom(_arg: statec) -> return;
}
// We only ever have a single encoder

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_wndw_atom {
    pub state: drm_plane_state,
    pub ilut: *mut drm_property_blob,
    pub visible: bool,
    pub handle: u32,
    pub offset:12: u16,
    pub awaken:1: bool,
    pub ntfy: },
    pub handle: u32,
    pub offset:12: u16,
    pub acquire: u32,
    pub release: u32,
    pub sema: },
    pub handle: u32,
    pub offset:40: u64,
    pub buffer:1: u8,
    pub enable:2: u8,
    pub mode:4: u8,
    pub size:11: u16,
    pub range:2: u8,
    pub output_mode:2: u8,
    pub ): *mut void __iomem,
    pub i: },
    pub xlut: },
    pub matrix: [u32; 12],
    pub valid: bool,
    pub csc: },
    pub mode:2: u8,
    pub interval:4: u8,
    pub colorspace:2: u8,
    pub format: u8,
    pub kind:7: u8,
    pub layout:1: u8,
    pub blockh:4: u8,
    pub blocks: [u16; 3],
    pub pitch: [u32; 3],
    pub w: u16,
    pub h: u16,
    pub handle: [u32; 6],
    pub offset: [u64; 6],
    pub image: },
    pub sx: u16,
    pub sy: u16,
    pub sw: u16,
    pub sh: u16,
    pub dw: u16,
    pub dh: u16,
    pub scale: },
    pub x: u16,
    pub y: u16,
    pub point: },
    pub depth: u8,
    pub k1: u8,
    pub src_color:4: u8,
    pub dst_color:4: u8,
    pub blend: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nv50_wndw_atom_mask {
    pub ntfy:1: bool,
    pub sema:1: bool,
    pub xlut:1: bool,
    pub csc:1: bool,
    pub image:1: bool,
    pub scale:1: bool,
    pub point:1: bool,
    pub blend:1: bool,
}
