//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvfw/flcn.h
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
pub struct loader_config {
    pub dma_idx: u32,
    pub code_dma_base: u32,
    pub code_size_total: u32,
    pub code_size_to_load: u32,
    pub code_entry_point: u32,
    pub data_dma_base: u32,
    pub data_size: u32,
    pub overlay_dma_base: u32,
    pub argc: u32,
    pub argv: u32,
    pub code_dma_base1: u32,
    pub data_dma_base1: u32,
    pub overlay_dma_base1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loader_config_v1 {
    pub reserved: u32,
    pub dma_idx: u32,
    pub code_dma_base: u64,
    pub code_size_total: u32,
    pub code_size_to_load: u32,
    pub code_entry_point: u32,
    pub data_dma_base: u64,
    pub data_size: u32,
    pub overlay_dma_base: u64,
    pub argc: u32,
    pub argv: u32,
    pub __packed: },
    pub ): *const *const loader_config_v1_dump(struct nvkm_subdev , struct loader_config_v1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flcn_bl_dmem_desc {
    pub reserved: [u32; 4],
    pub signature: [u32; 4],
    pub ctx_dma: u32,
    pub code_dma_base: u32,
    pub non_sec_code_off: u32,
    pub non_sec_code_size: u32,
    pub sec_code_off: u32,
    pub sec_code_size: u32,
    pub code_entry_point: u32,
    pub data_dma_base: u32,
    pub data_size: u32,
    pub code_dma_base1: u32,
    pub data_dma_base1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flcn_bl_dmem_desc_v1 {
    pub reserved: [u32; 4],
    pub signature: [u32; 4],
    pub ctx_dma: u32,
    pub code_dma_base: u64,
    pub non_sec_code_off: u32,
    pub non_sec_code_size: u32,
    pub sec_code_off: u32,
    pub sec_code_size: u32,
    pub code_entry_point: u32,
    pub data_dma_base: u64,
    pub data_size: u32,
    pub __packed: },
    pub ): *const flcn_bl_dmem_desc_v1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flcn_bl_dmem_desc_v2 {
    pub reserved: [u32; 4],
    pub signature: [u32; 4],
    pub ctx_dma: u32,
    pub code_dma_base: u64,
    pub non_sec_code_off: u32,
    pub non_sec_code_size: u32,
    pub sec_code_off: u32,
    pub sec_code_size: u32,
    pub code_entry_point: u32,
    pub data_dma_base: u64,
    pub data_size: u32,
    pub argc: u32,
    pub argv: u32,
    pub __packed: },
    pub ): *const flcn_bl_dmem_desc_v2,
