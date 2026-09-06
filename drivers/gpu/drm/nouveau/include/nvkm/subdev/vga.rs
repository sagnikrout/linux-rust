//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/vga.h
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

// access to various legacy io ports
extern "C" {
    pub fn nvkm_rdport(: *mut nvkm_device, head: c_int, port: u16) -> u8;
}
extern "C" {
    pub fn nvkm_wrport(: *mut nvkm_device, head: c_int, port: u16, value: u8);
}
// VGA Sequencer
extern "C" {
    pub fn nvkm_rdvgas(: *mut nvkm_device, head: c_int, index: u8) -> u8;
}
extern "C" {
    pub fn nvkm_wrvgas(: *mut nvkm_device, head: c_int, index: u8, value: u8);
}
// VGA Graphics
extern "C" {
    pub fn nvkm_rdvgag(: *mut nvkm_device, head: c_int, index: u8) -> u8;
}
extern "C" {
    pub fn nvkm_wrvgag(: *mut nvkm_device, head: c_int, index: u8, value: u8);
}
// VGA CRTC
extern "C" {
    pub fn nvkm_rdvgac(: *mut nvkm_device, head: c_int, index: u8) -> u8;
}
extern "C" {
    pub fn nvkm_wrvgac(: *mut nvkm_device, head: c_int, index: u8, value: u8);
}
// VGA indexed port access dispatcher
extern "C" {
    pub fn nvkm_rdvgai(: *mut nvkm_device, head: c_int, port: u16, index: u8) -> u8;
}
extern "C" {
    pub fn nvkm_wrvgai(: *mut nvkm_device, head: c_int, port: u16, index: u8, value: u8);
}
extern "C" {
    pub fn nvkm_lockvgac(: *mut nvkm_device, lock: bool) -> bool;
}
extern "C" {
    pub fn nvkm_rdvgaowner(: *mut nvkm_device) -> u8;
}
extern "C" {
    pub fn nvkm_wrvgaowner(: *mut nvkm_device, _arg: u8);
}
