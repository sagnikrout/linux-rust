//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_mmu.h
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
// Copyright (C) 2020-2023 Intel Corporation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_mmu_cdtab {
    pub base: *mut c_void,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_mmu_strtab {
    pub base: *mut c_void,
    pub dma: dma_addr_t,
    pub dma_q: u64,
    pub base_cfg: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_mmu_queue {
    pub base: *mut c_void,
    pub dma: dma_addr_t,
    pub dma_q: u64,
    pub prod: u32,
    pub cons: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_mmu_info {
    pub /: *mut *mut mutex lock; / Protects cdtab, strtab, cmdq, on,
    pub cdtab: ivpu_mmu_cdtab,
    pub strtab: ivpu_mmu_strtab,
    pub cmdq: ivpu_mmu_queue,
    pub evtq: ivpu_mmu_queue,
    pub on: bool,
}

extern "C" {
    pub fn ivpu_mmu_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_mmu_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_enable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_mmu_cd_set(vdev: *mut ivpu_device, ssid: c_int, pgtable: *mut ivpu_mmu_pgtable) -> c_int;
}
extern "C" {
    pub fn ivpu_mmu_cd_clear(vdev: *mut ivpu_device, ssid: c_int);
}
extern "C" {
    pub fn ivpu_mmu_invalidate_tlb(vdev: *mut ivpu_device, ssid: u16) -> c_int;
}
extern "C" {
    pub fn ivpu_mmu_irq_evtq_handler(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_irq_gerr_handler(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_evtq_dump(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_discard_events(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_disable_ssid_events(vdev: *mut ivpu_device, ssid: u32) -> c_int;
}
