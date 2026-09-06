//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/fault/priv.h
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
pub struct nvkm_fault_buffer {
    pub object: nvkm_object,
    pub fault: *mut nvkm_fault,
    pub id: c_int,
    pub entries: c_int,
    pub get: u32,
    pub put: u32,
    pub mem: *mut nvkm_memory,
    pub addr: u64,
    pub inth: nvkm_inth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fault_func {
    pub ): *mut *mut int (oneinit)(struct nvkm_fault,
    pub ): *mut *mut void (init)(struct nvkm_fault,
    pub ): *mut *mut void (fini)(struct nvkm_fault,
    pub ): *mut *mut void (intr)(struct nvkm_fault,
    pub nr: c_int,
    pub entry_size: u32,
    pub ): *mut *mut void (info)(struct nvkm_fault_buffer,
    pub ): *mut *mut u64 (pin)(struct nvkm_fault_buffer,
    pub ): *mut *mut void (init)(struct nvkm_fault_buffer,
    pub ): *mut *mut void (fini)(struct nvkm_fault_buffer,
    pub enable): *mut *mut *mut void (intr)(struct nvkm_fault_buffer , bool,
    pub buffer: },
    pub base: nvkm_sclass,
    pub rp: c_int,
    pub user: },
}

extern "C" {
    pub fn gp100_fault_buffer_intr(: *mut nvkm_fault_buffer, enable: bool);
}
extern "C" {
    pub fn gp100_fault_buffer_fini(: *mut nvkm_fault_buffer);
}
extern "C" {
    pub fn gp100_fault_buffer_init(: *mut nvkm_fault_buffer);
}
extern "C" {
    pub fn gp100_fault_buffer_pin(: *mut nvkm_fault_buffer) -> u64;
}
extern "C" {
    pub fn gp100_fault_buffer_info(: *mut nvkm_fault_buffer);
}
extern "C" {
    pub fn gv100_fault_buffer_process(: *mut work_struct);
}
extern "C" {
    pub fn gp100_fault_intr(: *mut nvkm_fault);
}
extern "C" {
    pub fn gp10b_fault_buffer_pin(: *mut nvkm_fault_buffer) -> u64;
}
extern "C" {
    pub fn gv100_fault_oneinit(: *mut nvkm_fault) -> c_int;
}
