//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_gem_framebuffer_helper.h
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


extern "C" {
    pub fn drm_gem_fb_destroy(fb: *mut drm_framebuffer);
}
extern "C" {
    pub fn drm_gem_fb_vunmap(fb: *mut drm_framebuffer, map: *mut iosys_map);
}
extern "C" {
    pub fn drm_gem_fb_begin_cpu_access(fb: *mut drm_framebuffer, dir: dma_data_direction) -> c_int;
}
extern "C" {
    pub fn drm_gem_fb_end_cpu_access(fb: *mut drm_framebuffer, dir: dma_data_direction);
}

