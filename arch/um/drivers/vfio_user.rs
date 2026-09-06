//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/drivers/vfio_user.h
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
pub struct uml_vfio_user_device {
    pub device: c_int,
    pub size: u64,
    pub offset: u64,
    pub region: *mut },
    pub num_regions: c_int,
    pub irqfd: *mut i32,
    pub irq_count: c_int,
}

extern "C" {
    pub fn uml_vfio_user_open_container() -> c_int;
}
extern "C" {
    pub fn uml_vfio_user_setup_iommu(container: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vfio_user_get_group_id(device: *const c_char) -> c_int;
}
extern "C" {
    pub fn uml_vfio_user_open_group(group_id: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vfio_user_set_container(container: c_int, group: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vfio_user_unset_container(container: c_int, group: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vfio_user_teardown_device(dev: *mut uml_vfio_user_device);
}
extern "C" {
    pub fn uml_vfio_user_activate_irq(dev: *mut uml_vfio_user_device, index: c_int) -> c_int;
}
extern "C" {
    pub fn uml_vfio_user_deactivate_irq(dev: *mut uml_vfio_user_device, index: c_int);
}
extern "C" {
    pub fn uml_vfio_user_update_irqs(dev: *mut uml_vfio_user_device) -> c_int;
}
