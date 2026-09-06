//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/drm.h
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
// Copyright (C) 2012 Avionic Design GmbH
// Copyright (C) 2012-2013 NVIDIA CORPORATION.  All rights reserved.
//
pub const HOST1X_DRM_H: c_int = 1;

// XXX move to include/uapi/drm/drm_fourcc.h?

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_drm {
    pub drm: *mut drm_device,
    pub domain: *mut iommu_domain,
    pub use_explicit_iommu: bool,
    pub mm_lock: mutex,
    pub mm: drm_mm,
    pub domain: iova_domain,
    pub shift: c_ulong,
    pub limit: c_ulong,
    pub carveout: },
    pub clients_lock: mutex,
    pub clients: list_head,
    pub vmask: unsigned int hmask,,
    pub pitch_align: c_uint,
    pub num_crtcs: c_uint,
    pub hub: *mut tegra_display_hub,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: tegra->drm->dev->parent) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_drm_context {
    pub client: *mut tegra_drm_client,
    pub channel: *mut host1x_channel,
// Only used by legacy UAPI.
    pub id: c_uint,
// Only used by new UAPI.
    pub mappings: xarray,
    pub memory_context: *mut host1x_memory_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_drm_client_ops {
    pub context): *mut tegra_drm_context,
    pub context): *mut *mut void (close_channel)(struct tegra_drm_context,
    pub offset): *mut *mut *mut int (is_addr_reg)(struct device dev, u32 class, u32,
    pub class): *mut *mut int (is_valid_class)(u32,
    pub file): *mut drm_file,
    pub offset): *mut *mut *mut int (get_streamid_offset)(struct tegra_drm_client client, u32,
    pub supported): *mut *mut *mut int (can_use_memory_ctx)(struct tegra_drm_client client, bool,
}

// offset = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_drm_client {
    pub base: host1x_client,
    pub list: list_head,
    pub drm: *mut tegra_drm,
    pub shared_channel: *mut host1x_channel,
// Set by driver
    pub version: c_uint,
    pub ops: *const tegra_drm_client_ops,
}

extern "C" {
    pub fn container_of(_arg: client, tegra_drm_client: struct, _arg: base) -> return;
}
extern "C" {
    pub fn host1x_client_iommu_attach(client: *mut host1x_client) -> c_int;
}
extern "C" {
    pub fn host1x_client_iommu_detach(client: *mut host1x_client);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_output {
    pub of_node: *mut device_node,
    pub dev: *mut device,
    pub bridge: *mut drm_bridge,
    pub panel: *mut drm_panel,
    pub ddc: *mut i2c_adapter,
    pub drm_edid: *const drm_edid,
    pub cec: *mut cec_notifier,
    pub hpd_irq: c_uint,
    pub hpd_gpio: *mut gpio_desc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

extern "C" {
    pub fn container_of(_arg: e, tegra_output: struct, _arg: encoder) -> return;
}
extern "C" {
    pub fn container_of(_arg: c, tegra_output: struct, _arg: connector) -> return;
}
// from output.c
extern "C" {
    pub fn tegra_output_probe(output: *mut tegra_output) -> c_int;
}
extern "C" {
    pub fn tegra_output_remove(output: *mut tegra_output);
}
extern "C" {
    pub fn tegra_output_init(drm: *mut drm_device, output: *mut tegra_output) -> c_int;
}
extern "C" {
    pub fn tegra_output_exit(output: *mut tegra_output);
}
extern "C" {
    pub fn tegra_output_suspend(output: *mut tegra_output) -> c_int;
}
extern "C" {
    pub fn tegra_output_resume(output: *mut tegra_output) -> c_int;
}
extern "C" {
    pub fn tegra_output_connector_get_modes(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn tegra_output_connector_destroy(connector: *mut drm_connector);
}
// from dpaux.c
extern "C" {
    pub fn drm_dp_aux_detect(aux: *mut drm_dp_aux) -> drm_connector_status;
}
extern "C" {
    pub fn drm_dp_aux_attach(aux: *mut drm_dp_aux, output: *mut tegra_output) -> c_int;
}
extern "C" {
    pub fn drm_dp_aux_detach(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_aux_enable(aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn drm_dp_aux_disable(aux: *mut drm_dp_aux) -> c_int;
}
// from fb.c
extern "C" {
    pub fn tegra_fb_is_bottom_up(framebuffer: *mut drm_framebuffer) -> bool;
}

