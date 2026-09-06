//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_bo.h
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
pub struct nouveau_bo {
    pub bo: ttm_buffer_object,
    pub placement: ttm_placement,
    pub valid_domains: u32,
    pub placements: [ttm_place; 3],
    pub force_coherent: bool,
    pub kmap: ttm_bo_kmap_obj,
    pub head: list_head,
    pub io_reserve_lru: list_head,
// protected by ttm_bo_reserve()
    pub reserved_by: *mut drm_file,
    pub entry: list_head,
    pub pbbo_index: c_int,
    pub validate_mapped: bool,
// Root GEM object we derive the dma_resv of in case this BO is not
// shared between VMs.
//
    pub r_obj: *mut drm_gem_object,
    pub no_share: bool,
// GPU address space is independent of CPU word size
    pub offset: u64,
    pub vma_list: list_head,
    pub contig:1: unsigned,
    pub page:5: unsigned,
    pub kind:8: unsigned,
    pub comp:3: unsigned,
    pub zeta:3: unsigned,
    pub mode: unsigned,
    pub tile: *mut nouveau_drm_tile,
}

extern "C" {
    pub fn container_of(_arg: bo, nouveau_bo: struct, _arg: bo) -> return;
}
extern "C" {
    pub fn nouveau_bo_move_init(: *mut nouveau_drm);
}
extern "C" {
    pub fn nouveau_bo_pin_locked(nvbo: *mut nouveau_bo, domain: u32, contig: bool) -> c_int;
}
extern "C" {
    pub fn nouveau_bo_unpin_locked(nvbo: *mut nouveau_bo);
}
extern "C" {
    pub fn nouveau_bo_pin(: *mut nouveau_bo, flags: u32, contig: bool) -> c_int;
}
extern "C" {
    pub fn nouveau_bo_unpin(: *mut nouveau_bo) -> c_int;
}
extern "C" {
    pub fn nouveau_bo_map(: *mut nouveau_bo) -> c_int;
}
extern "C" {
    pub fn nouveau_bo_unmap(: *mut nouveau_bo);
}
extern "C" {
    pub fn nouveau_bo_placement_set(: *mut nouveau_bo, type: u32, busy: u32);
}
extern "C" {
    pub fn nouveau_bo_wr16(: *mut nouveau_bo, index: unsigned, val: u16);
}
extern "C" {
    pub fn nouveau_bo_rd32(: *mut nouveau_bo, index: unsigned) -> u32;
}
extern "C" {
    pub fn nouveau_bo_wr32(: *mut nouveau_bo, index: unsigned, val: u32);
}
extern "C" {
    pub fn nouveau_ttm_fault_reserve_notify(bo: *mut ttm_buffer_object) -> vm_fault_t;
}
extern "C" {
    pub fn nouveau_bo_fence(: *mut nouveau_bo, : *mut nouveau_fence, exclusive: bool);
}
extern "C" {
    pub fn nouveau_bo_sync_for_device(nvbo: *mut nouveau_bo);
}
extern "C" {
    pub fn nouveau_bo_sync_for_cpu(nvbo: *mut nouveau_bo);
}
extern "C" {
    pub fn nouveau_bo_add_io_reserve_lru(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn nouveau_bo_del_io_reserve_lru(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn nouveau_bo_new_pin(: *mut nouveau_cli, domain: u32, size: u32, : *mut nouveau_bo) -> c_int;
}
extern "C" {
    pub fn nouveau_bo_new_map(: *mut nouveau_cli, domain: u32, size: u32, : *mut nouveau_bo) -> c_int;
}
extern "C" {
    pub fn nouveau_bo_unpin_del(: *mut nouveau_bo);
}
// TODO: submit equivalent to TTM generic API upstream?
extern "C" {
    pub fn nv04_bo_move_init(: *mut nouveau_channel, _arg: u32) -> c_int;
}
extern "C" {
    pub fn nv50_bo_move_init(: *mut nouveau_channel, _arg: u32) -> c_int;
}
extern "C" {
    pub fn nvc0_bo_move_init(: *mut nouveau_channel, _arg: u32) -> c_int;
}
extern "C" {
    pub fn nve0_bo_move_init(: *mut nouveau_channel, _arg: u32) -> c_int;
}

