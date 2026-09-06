//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pci/priv.h
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
pub struct nvkm_pci_func {
    pub addr: u32,
    pub size: u16,
    pub cfg: },
    pub ): *mut *mut void (init)(struct nvkm_pci,
    pub ): *mut *mut void (msi_rearm)(struct nvkm_pci,
    pub ): *mut *mut int (init)(struct nvkm_pci,
    pub u8): *mut *mut *mut int (set_link)(struct nvkm_pci , enum nvkm_pcie_speed,,
    pub ): *mut *mut nvkm_pcie_speed (max_speed)(struct nvkm_pci,
    pub ): *mut *mut nvkm_pcie_speed (cur_speed)(struct nvkm_pci,
    pub u8): *mut *mut *mut void (set_version)(struct nvkm_pci ,,
    pub ): *mut *mut int (version)(struct nvkm_pci,
    pub ): *mut *mut int (version_supported)(struct nvkm_pci,
    pub pcie: },
}

extern "C" {
    pub fn nv40_pci_msi_rearm(: *mut nvkm_pci);
}
extern "C" {
    pub fn nv46_pci_msi_rearm(: *mut nvkm_pci);
}
extern "C" {
    pub fn g84_pci_init(pci: *mut nvkm_pci);
}
// pcie functions
extern "C" {
    pub fn g84_pcie_set_version(: *mut nvkm_pci, _arg: u8);
}
extern "C" {
    pub fn g84_pcie_version(: *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn g84_pcie_set_link_speed(: *mut nvkm_pci, nvkm_pcie_speed: enum);
}
extern "C" {
    pub fn g84_pcie_cur_speed(: *mut nvkm_pci) -> nvkm_pcie_speed;
}
extern "C" {
    pub fn g84_pcie_max_speed(: *mut nvkm_pci) -> nvkm_pcie_speed;
}
extern "C" {
    pub fn g84_pcie_init(: *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn g84_pcie_set_link(: *mut nvkm_pci, nvkm_pcie_speed: enum, _arg: u8) -> c_int;
}
extern "C" {
    pub fn g92_pcie_version_supported(: *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gf100_pcie_set_version(: *mut nvkm_pci, _arg: u8);
}
extern "C" {
    pub fn gf100_pcie_version(: *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gf100_pcie_set_cap_speed(: *mut nvkm_pci, _arg: bool);
}
extern "C" {
    pub fn gf100_pcie_cap_speed(: *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gf100_pcie_init(: *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gf100_pcie_set_link(: *mut nvkm_pci, nvkm_pcie_speed: enum, _arg: u8) -> c_int;
}
extern "C" {
    pub fn nvkm_pcie_oneinit(: *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn nvkm_pcie_init(: *mut nvkm_pci) -> c_int;
}
