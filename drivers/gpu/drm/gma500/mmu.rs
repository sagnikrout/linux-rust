//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/mmu.h
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
// Copyright (c) 2007-2011, Intel Corporation.
// All Rights Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_mmu_driver {
// protects driver- and pd structures. Always take in read mode
// before taking the page table spinlock.
//
    pub sem: rw_semaphore,
// protects page tables, directory tables and pt tables.
// and pt structures.
//
    pub lock: spinlock_t,
    pub needs_tlbflush: core::sync::atomic::AtomicI32,
    pub msvdx_mmu_invaldc: *mut core::sync::atomic::AtomicI32,
    pub default_pd: *mut psb_mmu_pd,
    pub bif_ctrl: u32,
    pub has_clflush: c_int,
    pub clflush_add: c_int,
    pub clflush_mask: c_ulong,
    pub dev: *mut drm_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_mmu_pt {
    pub pd: *mut psb_mmu_pd,
    pub index: u32,
    pub count: u32,
    pub p: *mut page,
    pub v: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_mmu_pd {
    pub driver: *mut psb_mmu_driver,
    pub hw_context: c_int,
    pub tables: *mut psb_mmu_pt,
    pub p: *mut page,
    pub dummy_pt: *mut page,
    pub dummy_page: *mut page,
    pub pd_mask: u32,
    pub invalid_pde: u32,
    pub invalid_pte: u32,
}

extern "C" {
    pub fn psb_mmu_driver_takedown(driver: *mut psb_mmu_driver);
}
// driver);
extern "C" {
    pub fn psb_mmu_free_pagedir(pd: *mut psb_mmu_pd);
}
extern "C" {
    pub fn psb_mmu_flush(driver: *mut psb_mmu_driver);
}
extern "C" {
    pub fn psb_mmu_set_pd_context(pd: *mut psb_mmu_pd, hw_context: c_int);
}
