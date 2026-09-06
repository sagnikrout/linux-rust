//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/interconnect/mediatek/icc-emi.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2021 MediaTek Inc.
// Copyright (c) 2024 Collabora Ltd.
// AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
//
// struct mtk_icc_node - Mediatek EMI Interconnect Node
// @name:      The interconnect node name which is shown in debugfs
// @ep:        Type of this endpoint
// @id:        Unique node identifier
// @sum_avg:   Current sum aggregate value of all average bw requests in kBps
// @max_peak:  Current max aggregate value of all peak bw requests in kBps
// @num_links: The total number of @links
// @links:     Array of @id linked to this node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_icc_node {
    pub name: *mut c_uchar,
    pub ep: c_int,
    pub id: u16,
    pub sum_avg: u64,
    pub max_peak: u64,
    pub num_links: u16,
    pub __counted_by(num_links): u16 links[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_icc_desc {
    pub nodes: *mut mtk_icc_node,
    pub num_nodes: usize,
}

extern "C" {
    pub fn mtk_emi_icc_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn mtk_emi_icc_remove(pdev: *mut platform_device);
}
