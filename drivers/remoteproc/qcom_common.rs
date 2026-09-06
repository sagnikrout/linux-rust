//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/qcom_common.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rproc_glink {
    pub subdev: rproc_subdev,
    pub ssr_name: *const c_char,
    pub dev: *mut device,
    pub node: *mut device_node,
    pub edge: *mut qcom_glink_smem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rproc_subdev {
    pub subdev: rproc_subdev,
    pub dev: *mut device,
    pub node: *mut device_node,
    pub edge: *mut qcom_smd_edge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rproc_ssr {
    pub subdev: rproc_subdev,
    pub info: *mut qcom_ssr_subsystem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_rproc_pdm {
    pub subdev: rproc_subdev,
    pub dev: *mut device,
    pub index: c_int,
    pub adev: *mut auxiliary_device,
}

extern "C" {
    pub fn qcom_remove_glink_subdev(rproc: *mut rproc, glink: *mut qcom_rproc_glink);
}
extern "C" {
    pub fn qcom_register_dump_segments(rproc: *mut rproc, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn qcom_add_smd_subdev(rproc: *mut rproc, smd: *mut qcom_rproc_subdev);
}
extern "C" {
    pub fn qcom_remove_smd_subdev(rproc: *mut rproc, smd: *mut qcom_rproc_subdev);
}
extern "C" {
    pub fn qcom_remove_ssr_subdev(rproc: *mut rproc, ssr: *mut qcom_rproc_ssr);
}
extern "C" {
    pub fn qcom_add_pdm_subdev(rproc: *mut rproc, pdm: *mut qcom_rproc_pdm);
}
extern "C" {
    pub fn qcom_remove_pdm_subdev(rproc: *mut rproc, pdm: *mut qcom_rproc_pdm);
}

extern "C" {
    pub fn qcom_remove_sysmon_subdev(sysmon: *mut qcom_sysmon);
}
extern "C" {
    pub fn qcom_sysmon_shutdown_acked(sysmon: *mut qcom_sysmon) -> bool;
}
extern "C" {
    pub fn qcom_sysmon_shutdown_irq_state(sysmon: *mut qcom_sysmon) -> bool;
}

