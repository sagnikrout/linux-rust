//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/crc.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nv50_crc_source {
    NV50_CRC_SOURCE_NONE = 0,
    NV50_CRC_SOURCE_AUTO,
    NV50_CRC_SOURCE_RG,
    NV50_CRC_SOURCE_OUTP_ACTIVE,
    NV50_CRC_SOURCE_OUTP_COMPLETE,
    NV50_CRC_SOURCE_OUTP_INACTIVE,
}

// RG -> SF (DP only)
// -> SOR
// -> PIOR
// -> DAC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nv50_crc_source_type {
    NV50_CRC_SOURCE_TYPE_NONE = 0,
    NV50_CRC_SOURCE_TYPE_SOR,
    NV50_CRC_SOURCE_TYPE_PIOR,
    NV50_CRC_SOURCE_TYPE_DAC,
    NV50_CRC_SOURCE_TYPE_RG,
    NV50_CRC_SOURCE_TYPE_SF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_crc_notifier_ctx {
    pub mem: nvif_mem,
    pub ntfy: nvif_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_crc_atom {
    pub src: nv50_crc_source,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_crc_func {
    pub ctx): *mut nv50_crc_notifier_ctx,
    pub ): *mut *mut *mut int (set_ctx)(struct nv50_head , struct nv50_crc_notifier_ctx,
    pub idx): nv50_crc_source, int,
    pub ): *mut nv50_crc_notifier_ctx,
    pub flip_threshold: c_short,
    pub num_entries: c_short,
    pub notifier_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_crc {
    pub lock: spinlock_t,
    pub ctx: [nv50_crc_notifier_ctx; 2],
    pub flip_work: drm_vblank_work,
    pub src: nv50_crc_source,
    pub frame: u64,
    pub entry_idx: c_short,
    pub flip_threshold: c_short,
    pub 1: u8 ctx_idx :,
    pub 1: bool ctx_changed :,
}

extern "C" {
    pub fn nv50_crc_init(dev: *mut drm_device);
}
extern "C" {
    pub fn nv50_head_crc_late_register(: *mut nv50_head) -> c_int;
}
extern "C" {
    pub fn nv50_crc_handle_vblank(head: *mut nv50_head);
}
extern "C" {
    pub fn nv50_crc_verify_source(: *mut drm_crtc, : *const c_char, : *mut usize) -> c_int;
}
extern "C" {
    pub fn nv50_crc_set_source(: *mut drm_crtc, : *const c_char) -> c_int;
}
extern "C" {
    pub fn nv50_crc_atomic_check_outp(atom: *mut nv50_atom);
}
extern "C" {
    pub fn nv50_crc_atomic_stop_reporting(: *mut drm_atomic_commit);
}
extern "C" {
    pub fn nv50_crc_atomic_init_notifier_contexts(: *mut drm_atomic_commit);
}
extern "C" {
    pub fn nv50_crc_atomic_release_notifier_contexts(: *mut drm_atomic_commit);
}
extern "C" {
    pub fn nv50_crc_atomic_start_reporting(: *mut drm_atomic_commit);
}
extern "C" {
    pub fn nv50_crc_atomic_set(: *mut nv50_head, : *mut nv50_head_atom);
}
extern "C" {
    pub fn nv50_crc_atomic_clr(: *mut nv50_head);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_crc {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_crc_func {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv50_crc_atom {

    pub }: *mut *mut nv50_head_crc_late_register(struct nv50_head head) { return 0;,
    pub }: *mut *mut nv50_head_atom armh) { return 0;,

