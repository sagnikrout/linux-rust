//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp_kms.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_kms_funcs {
    pub base: msm_kms_funcs,
    pub old_irqmask): u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_kms {
    pub base: msm_kms,
    pub funcs: *const mdp_kms_funcs,
// irq handling:
    pub in_irq: bool,
    pub /: *mut *mut list_head irq_list; / list of mdp4_irq,
    pub /: *mut *mut uint32_t vblank_mask; / irq bits set for userspace vblank,
    pub /: *mut *mut uint32_t cur_irq_mask; / current irq mask,
}

extern "C" {
    pub fn msm_kms_init(_arg: &mdp_kms->base, _arg: &funcs->base) -> return;
}
//
// irq helpers:
//
// For transiently registering for different MDP irqs that various parts
// of the KMS code need during setup/configuration.  These are not
// necessarily the same as what drm_vblank_get/put() are requesting, and
// the hysteresis in drm_vblank_put() is not necessarily desirable for
// internal housekeeping related irq usage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_irq {
    pub node: list_head,
    pub irqmask: u32,
    pub registered: bool,
    pub irqstatus): *mut *mut *mut void (irq)(struct mdp_irq irq, uint32_t,
}

extern "C" {
    pub fn mdp_dispatch_irqs(mdp_kms: *mut mdp_kms, status: u32);
}
extern "C" {
    pub fn mdp_update_vblank_mask(mdp_kms: *mut mdp_kms, mask: u32, enable: bool);
}
extern "C" {
    pub fn mdp_irq_wait(mdp_kms: *mut mdp_kms, irqmask: u32);
}
extern "C" {
    pub fn mdp_irq_register(mdp_kms: *mut mdp_kms, irq: *mut mdp_irq);
}
extern "C" {
    pub fn mdp_irq_unregister(mdp_kms: *mut mdp_kms, irq: *mut mdp_irq);
}
extern "C" {
    pub fn mdp_irq_update(mdp_kms: *mut mdp_kms);
}
//
// pixel format helpers:
//
// MDP capabilities

// MDP pipe capabilities

// MDP layer mixer caps

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csc_type {
    CSC_RGB2RGB = 0,
    CSC_YUV2RGB,
    CSC_RGB2YUV,
    CSC_YUV2YUV,
    CSC_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csc_cfg {
    pub type: csc_type,
    pub matrix: [u32; 9],
    pub pre_bias: [u32; 3],
    pub post_bias: [u32; 3],
    pub pre_clamp: [u32; 6],
    pub post_clamp: [u32; 6],
}
