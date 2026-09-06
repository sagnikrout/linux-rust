//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/vmm.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvif_vmm_type {
    UNMANAGED,
    MANAGED,
    RAW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvif_vmm_get {
    ADDR,
    PTES,
    LAZY
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vma {
    pub addr: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_vmm {
    pub object: nvif_object,
    pub start: u64,
    pub limit: u64,
    pub shift: u8,
    pub sparse:1: bool,
    pub vram:1: bool,
    pub host:1: bool,
    pub comp:1: bool,
    pub page: *mut },
    pub page_nr: c_int,
}

extern "C" {
    pub fn nvif_vmm_dtor(: *mut nvif_vmm);
}
extern "C" {
    pub fn nvif_vmm_put(: *mut nvif_vmm, : *mut nvif_vma);
}
extern "C" {
    pub fn nvif_vmm_unmap(: *mut nvif_vmm, _arg: u64) -> c_int;
}
extern "C" {
    pub fn nvif_vmm_raw_get(vmm: *mut nvif_vmm, addr: u64, size: u64, shift: u8) -> c_int;
}
extern "C" {
    pub fn nvif_vmm_raw_put(vmm: *mut nvif_vmm, addr: u64, size: u64, shift: u8) -> c_int;
}
extern "C" {
    pub fn nvif_vmm_raw_sparse(vmm: *mut nvif_vmm, addr: u64, size: u64, ref: bool) -> c_int;
}
