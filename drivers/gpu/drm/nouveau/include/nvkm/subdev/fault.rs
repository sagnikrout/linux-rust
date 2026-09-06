//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/fault.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fault {
    pub func: *const nvkm_fault_func,
    pub subdev: nvkm_subdev,
    pub info_fault: nvkm_inth,
    pub buffer: [*mut nvkm_fault_buffer; 2],
    pub buffer_nr: c_int,

    pub event: nvkm_event,
    pub nrpfb: nvkm_event_ntfy,
    pub nrpfb_work: work_struct,
    pub user: nvkm_device_oclass,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fault_data {
    pub addr: u64,
    pub inst: u64,
    pub time: u64,
    pub engine: u8,
    pub valid: u8,
    pub gpc: u8,
    pub hub: u8,
    pub access: u8,
    pub client: u8,
    pub reason: u8,
}

extern "C" {
    pub fn gp100_fault_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fault) -> c_int;
}
extern "C" {
    pub fn gp10b_fault_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fault) -> c_int;
}
extern "C" {
    pub fn gv100_fault_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fault) -> c_int;
}
extern "C" {
    pub fn tu102_fault_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fault) -> c_int;
}
