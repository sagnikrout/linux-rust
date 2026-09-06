//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/pci.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_pcie_speed {
    NVKM_PCIE_SPEED_2_5,
    NVKM_PCIE_SPEED_5_0,
    NVKM_PCIE_SPEED_8_0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_pci {
    pub func: *const nvkm_pci_func,
    pub subdev: nvkm_subdev,
    pub pdev: *mut pci_dev,
    pub bridge: *mut agp_bridge_data,
    pub mode: u32,
    pub base: u64,
    pub size: u64,
    pub mtrr: c_int,
    pub cma: bool,
    pub acquired: bool,
    pub agp: },
    pub speed: nvkm_pcie_speed,
    pub width: u8,
    pub pcie: },
    pub msi: bool,
}

extern "C" {
    pub fn nvkm_pci_rd32(: *mut nvkm_pci, addr: u16) -> u32;
}
extern "C" {
    pub fn nvkm_pci_wr08(: *mut nvkm_pci, addr: u16, data: u8);
}
extern "C" {
    pub fn nvkm_pci_wr32(: *mut nvkm_pci, addr: u16, data: u32);
}
extern "C" {
    pub fn nvkm_pci_mask(: *mut nvkm_pci, addr: u16, mask: u32, value: u32) -> u32;
}
extern "C" {
    pub fn nvkm_pci_rom_shadow(: *mut nvkm_pci, shadow: bool);
}
extern "C" {
    pub fn nvkm_pci_msi_rearm(: *mut nvkm_device);
}
extern "C" {
    pub fn nv04_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn nv40_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn nv46_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn nv4c_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn g84_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn g92_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn g94_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gf100_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gf106_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gk104_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gp100_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
extern "C" {
    pub fn gh100_pci_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_pci) -> c_int;
}
// pcie functions
extern "C" {
    pub fn nvkm_pcie_set_link(: *mut nvkm_pci, nvkm_pcie_speed: enum, width: u8) -> c_int;
}
