//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_panic.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
//
// Copyright (c) 2024 Intel
// Copyright (c) 2024 Red Hat
//

//
// struct drm_scanout_buffer - DRM scanout buffer
//
// This structure holds the information necessary for drm_panic to draw the
// panic screen, and display it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_scanout_buffer {
//
// @format:
//
// drm format of the scanout buffer.
//
    pub format: *const drm_format_info,
//
// @map:
//
// Virtual address of the scanout buffer, either in memory or iomem.
// The scanout buffer should be in linear format, and can be directly
// sent to the display hardware. Tearing is not an issue for the panic
// screen.
//
    pub map: [iosys_map; DRM_FORMAT_MAX_PLANES],
//
// @pages: Optional, if the scanout buffer is not mapped, set this field
// to the array of pages of the scanout buffer. The panic code will use
// kmap_local_page_try_from_panic() to map one page at a time to write
// all the pixels. This array shouldn't be allocated from the
// get_scanoutbuffer() callback.
// The scanout buffer should be in linear format.
//
    pub pages: *mut page,
//
// @width: Width of the scanout buffer, in pixels.
//
    pub width: c_uint,
//
// @height: Height of the scanout buffer, in pixels.
//
    pub height: c_uint,
//
// @pitch: Length in bytes between the start of two consecutive lines.
//
    pub pitch: [c_uint; DRM_FORMAT_MAX_PLANES],
//
// @set_pixel: Optional function, to set a pixel color on the
// framebuffer. It allows to handle special tiling format inside the
// driver. It takes precedence over the @map and @pages fields.
//
    pub color): unsigned int y, u32,
//
// @private: private pointer that you can use in the callbacks
// set_pixel()
//
    pub private: *mut c_void,
}

//
// drm_panic_trylock - try to enter the panic printing critical section
// @dev: struct drm_device
// @flags: unsigned long irq flags you need to pass to the unlock() counterpart
//
// This function must be called by any panic printing code. The panic printing
// attempt must be aborted if the trylock fails.
//
// Panic printing code can make the following assumptions while holding the
// panic lock:
//
// - Anything protected by drm_panic_lock() and drm_panic_unlock() pairs is safe
// to access.
//
// - Furthermore the panic printing code only registers in drm_dev_unregister()
// and gets removed in drm_dev_unregister(). This allows the panic code to
// safely access any state which is invariant in between these two function
// calls, like the list of planes &drm_mode_config.plane_list or most of the
// struct drm_plane structure.
//
// Specifically thanks to the protection around plane updates in
// drm_atomic_helper_swap_state() the following additional guarantees hold:
//
// - It is safe to deference the drm_plane.state pointer.
//
// - Anything in struct drm_plane_state or the driver's subclass thereof which
// stays invariant after the atomic check code has finished is safe to access.
// Specifically this includes the reference counted pointers to framebuffer
// and buffer objects.
//
// - Anything set up by &drm_plane_helper_funcs.fb_prepare and cleaned up
// &drm_plane_helper_funcs.fb_cleanup is safe to access, as long as it stays
// invariant between these two calls. This also means that for drivers using
// dynamic buffer management the framebuffer is pinned, and therefer all
// relevant datastructures can be accessed without taking any further locks
// (which would be impossible in panic context anyway).
//
// - Importantly, software and hardware state set up by
// &drm_plane_helper_funcs.begin_fb_access and
// &drm_plane_helper_funcs.end_fb_access is not safe to access.
//
// Drivers must not make any assumptions about the actual state of the hardware,
// unless they explicitly protected these hardware access with drm_panic_lock()
// and drm_panic_unlock().
//
// Return:
// %0 when failing to acquire the raw spinlock, nonzero on success.
//

//
// drm_panic_lock - protect panic printing relevant state
// @dev: struct drm_device
// @flags: unsigned long irq flags you need to pass to the unlock() counterpart
//
// This function must be called to protect software and hardware state that the
// panic printing code must be able to rely on. The protected sections must be
// as small as possible. It uses the irqsave/irqrestore variant, and can be
// called from irq handler. Examples include:
//
// - Access to peek/poke or other similar registers, if that is the way the
// driver prints the pixels into the scanout buffer at panic time.
//
// - Updates to pointers like &drm_plane.state, allowing the panic handler to
// safely deference these. This is done in drm_atomic_helper_swap_state().
//
// - An state that isn't invariant and that the driver must be able to access
// during panic printing.
//

//
// drm_panic_unlock - end of the panic printing critical section
// @dev: struct drm_device
// @flags: irq flags that were returned when acquiring the lock
//
// Unlocks the raw spinlock acquired by either drm_panic_lock() or
// drm_panic_trylock().
//

extern "C" {
    pub fn drm_panic_qr_max_data_size(version: u8, url_len: usize) -> usize;
}

