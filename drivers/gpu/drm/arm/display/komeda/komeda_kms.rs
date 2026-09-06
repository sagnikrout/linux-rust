//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_kms.h
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
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

//
// struct komeda_plane - komeda instance of drm_plane
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_plane {
// @base: &drm_plane
    pub base: drm_plane,
//
// @layer:
//
// represents available layer input pipelines for this plane.
//
// NOTE:
// the layer is not for a specific Layer, but indicate a group of
// Layers with same capabilities.
//
    pub layer: *mut komeda_layer,
}

//
// struct komeda_plane_state
//
// The plane_state can be split into two data flow (left/right) and handled
// by two layers &komeda_plane.layer and &komeda_plane.layer.right
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_plane_state {
// @base: &drm_plane_state
    pub base: drm_plane_state,
// @zlist_node: zorder list node
    pub zlist_node: list_head,
// @layer_split: on/off layer_split
    pub 1: u8 layer_split :,
}

//
// struct komeda_wb_connector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_wb_connector {
// @base: &drm_writeback_connector
    pub base: drm_writeback_connector,
// @wb_layer: represents associated writeback pipeline of komeda
    pub wb_layer: *mut komeda_layer,
}

//
// struct komeda_crtc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_crtc {
// @base: &drm_crtc
    pub base: drm_crtc,
// @master: only master has display output
    pub master: *mut komeda_pipeline,
//
// @slave: optional
//
// Doesn't have its own display output, the handled data flow will
// merge into the master.
//
    pub slave: *mut komeda_pipeline,
// @slave_planes: komeda slave planes mask
    pub slave_planes: u32,
// @wb_conn: komeda write back connector
    pub wb_conn: *mut komeda_wb_connector,
// @disable_done: this flip_done is for tracing the disable
    pub disable_done: *mut completion,
// @encoder: encoder at the end of the pipeline
    pub encoder: drm_encoder,
}

//
// struct komeda_crtc_state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_crtc_state {
// @base: &drm_crtc_state
    pub base: drm_crtc_state,
// private properties
// computed state which are used by validate/check
//
// @affected_pipes:
// the affected pipelines in once display instance
//
    pub affected_pipes: u32,
//
// @active_pipes:
// the active pipelines in once display instance
//
    pub active_pipes: u32,
// @clock_ratio: ratio of (aclk << 32)/pxlclk
    pub clock_ratio: u64,
// @max_slave_zorder: the maximum of slave zorder
    pub max_slave_zorder: u32,
}

// struct komeda_kms_dev - for gather KMS related things
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_kms_dev {
// @base: &drm_device
    pub base: drm_device,
// @n_crtcs: valid numbers of crtcs in &komeda_kms_dev.crtcs
    pub n_crtcs: c_int,
// @crtcs: crtcs list
    pub crtcs: [komeda_crtc; KOMEDA_MAX_PIPELINES],
}

extern "C" {
    pub fn komeda_crtc_get_aclk(kcrtc_st: *mut komeda_crtc_state) -> c_ulong;
}
extern "C" {
    pub fn komeda_kms_setup_crtcs(kms: *mut komeda_kms_dev, mdev: *mut komeda_dev) -> c_int;
}
extern "C" {
    pub fn komeda_kms_add_crtcs(kms: *mut komeda_kms_dev, mdev: *mut komeda_dev) -> c_int;
}
extern "C" {
    pub fn komeda_kms_add_planes(kms: *mut komeda_kms_dev, mdev: *mut komeda_dev) -> c_int;
}
extern "C" {
    pub fn komeda_kms_cleanup_private_objs(kms: *mut komeda_kms_dev);
}
extern "C" {
    pub fn komeda_kms_detach(kms: *mut komeda_kms_dev);
}
extern "C" {
    pub fn komeda_kms_shutdown(kms: *mut komeda_kms_dev);
}
extern "C" {
    pub fn komeda_pipeline_dump(pipe: *mut komeda_pipeline);
}
