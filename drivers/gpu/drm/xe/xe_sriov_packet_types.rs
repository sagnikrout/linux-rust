//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_packet_types.h
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
//
// Copyright © 2025 Intel Corporation
//

//
// enum xe_sriov_packet_type - Xe SR-IOV VF migration data packet type
// @XE_SRIOV_PACKET_TYPE_DESCRIPTOR: Descriptor with VF device metadata
// @XE_SRIOV_PACKET_TYPE_TRAILER: Trailer indicating end-of-stream
// @XE_SRIOV_PACKET_TYPE_GGTT: Global GTT migration data
// @XE_SRIOV_PACKET_TYPE_MMIO: MMIO registers migration data
// @XE_SRIOV_PACKET_TYPE_GUC: GuC firmware migration data
// @XE_SRIOV_PACKET_TYPE_VRAM: VRAM migration data
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_sriov_packet_type {
// Skipping 0 to catch uninitialized data
    XE_SRIOV_PACKET_TYPE_DESCRIPTOR = 1,
    XE_SRIOV_PACKET_TYPE_TRAILER,
    XE_SRIOV_PACKET_TYPE_GGTT,
    XE_SRIOV_PACKET_TYPE_MMIO,
    XE_SRIOV_PACKET_TYPE_GUC,
    XE_SRIOV_PACKET_TYPE_VRAM,
}

//
// struct xe_sriov_packet_hdr - Xe SR-IOV VF migration data packet header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_packet_hdr {
// @version: migration data protocol version
    pub version: u8,
// @type: migration data type
    pub type: u8,
// @tile_id: migration data tile id
    pub tile_id: u8,
// @gt_id: migration data gt id
    pub gt_id: u8,
// @flags: migration data flags
    pub flags: u32,
//
// @offset: offset into the resource;
// used when multiple packets of given type are used for migration
//
    pub offset: u64,
// @size: migration data size
    pub size: u64,
    pub __packed: },
//
// struct xe_sriov_packet - Xe SR-IOV VF migration data packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sriov_packet {
// @xe: the PF &xe_device this data packet belongs to
    pub xe: *mut xe_device,
// @vaddr: CPU pointer to payload data
    pub vaddr: *mut c_void,
// @remaining: payload data remaining
    pub remaining: usize,
// @hdr_remaining: header data remaining
    pub hdr_remaining: usize,
// @bo: Buffer object with migration data
    pub bo: *mut xe_bo,
// @buff: Buffer with migration data
    pub buff: *mut c_void,
}

// @hdr: data packet header
