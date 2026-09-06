//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_kms.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (c) 2009-2025 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

//
// struct vmw_du_update_plane - Closure structure for vmw_du_helper_plane_update
// @plane: Plane which is being updated.
// @old_state: Old state of plane.
// @dev_priv: Device private.
// @du: Display unit on which to update the plane.
// @vfb: Framebuffer which is blitted to display unit.
// @out_fence: Out fence for resource finish.
// @mutex: The mutex used to protect resource reservation.
// @cpu_blit: True if need cpu blit.
// @intr: Whether to perform waits interruptible if possible.
//
// This structure loosely represent the set of operations needed to perform a
// plane update on a display unit. Implementer will define that functionality
// according to the function callbacks for this structure. In brief it involves
// surface/buffer object validation, populate FIFO commands and command
// submission to the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_du_update_plane {
//
// @calc_fifo_size: Calculate fifo size.
//
// Determine fifo size for the commands needed for update. The number of
// damage clips on display unit @num_hits will be passed to allocate
// sufficient fifo space.
//
// Return: Fifo size needed
//
    pub num_hits): u32,
//
// @post_prepare: Populate fifo for resource preparation.
//
// Some surface resource or buffer object need some extra cmd submission
// like update GB image for proxy surface and define a GMRFB for screen
// object. That should be done here as this callback will be
// called after FIFO allocation with the address of command buufer.
//
// This callback is optional.
//
// Return: Size of commands populated to command buffer.
//
    pub cmd): *mut *mut *mut uint32_t (post_prepare)(struct vmw_du_update_plane update, void,
//
// @pre_clip: Populate fifo before clip.
//
// This is where pre clip related command should be populated like
// surface copy/DMA, etc.
//
// This callback is optional.
//
// Return: Size of commands populated to command buffer.
//
    pub num_hits): u32,
//
// @clip: Populate fifo for clip.
//
// This is where to populate clips for surface copy/dma or blit commands
// if needed. This will be called times have damage in display unit,
// which is one if doing full update. @clip is the damage in destination
// coordinates which is crtc/DU and @src_x, @src_y is damage clip src in
// framebuffer coordinate.
//
// This callback is optional.
//
// Return: Size of commands populated to command buffer.
//
    pub src_y): *mut *mut drm_rect clip, uint32_t src_x, uint32_t,
//
// @post_clip: Populate fifo after clip.
//
// This is where to populate display unit update commands or blit
// commands.
//
// Return: Size of commands populated to command buffer.
//
    pub bb): *mut drm_rect,
    pub plane: *mut drm_plane,
    pub old_state: *mut drm_plane_state,
    pub dev_priv: *mut vmw_private,
    pub du: *mut vmw_display_unit,
    pub vfb: *mut vmw_framebuffer,
    pub out_fence: *mut vmw_fence_obj,
    pub mutex: *mut mutex,
    pub intr: bool,
}

//
// struct vmw_du_update_plane_surface - closure structure for surface
// @base: base closure structure.
// @cmd_start: FIFO command start address (used by SOU only).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_du_update_plane_surface {
    pub base: vmw_du_update_plane,
// This member is to handle special case SOU surface update
    pub cmd_start: *mut c_void,
}

//
// struct vmw_du_update_plane_buffer - Closure structure for buffer object
// @base: Base closure structure.
// @fb_left: x1 for fb damage bounding box.
// @fb_top: y1 for fb damage bounding box.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_du_update_plane_buffer {
    pub base: vmw_du_update_plane,
    pub fb_top: int fb_left,,
}

//
// struct vmw_kms_dirty - closure structure for the vmw_kms_helper_dirty
// function.
//
// @fifo_commit: Callback that is called once for each display unit after
// all clip rects. This function must commit the fifo space reserved by the
// helper. Set up by the caller.
// @clip: Callback that is called for each cliprect on each display unit.
// Set up by the caller.
// @fifo_reserve_size: Fifo size that the helper should try to allocat for
// each display unit. Set up by the caller.
// @dev_priv: Pointer to the device private. Set up by the helper.
// @unit: The current display unit. Set up by the helper before a call to @clip.
// @cmd: The allocated fifo space. Set up by the helper before the first @clip
// call.
// @crtc: The crtc for which to build dirty commands.
// @num_hits: Number of clip rect commands for this display unit.
// Cleared by the helper before the first @clip call. Updated by the @clip
// callback.
// @fb_x: Clip rect left side in framebuffer coordinates.
// @fb_y: Clip rect right side in framebuffer coordinates.
// @unit_x1: Clip rect left side in crtc coordinates.
// @unit_y1: Clip rect top side in crtc coordinates.
// @unit_x2: Clip rect right side in crtc coordinates.
// @unit_y2: Clip rect bottom side in crtc coordinates.
//
// The clip rect coordinates are updated by the helper for each @clip call.
// Note that this may be derived from if more info needs to be passed between
// helper caller and helper callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_kms_dirty {
    pub ): *mut *mut void (fifo_commit)(struct vmw_kms_dirty,
    pub ): *mut *mut void (clip)(struct vmw_kms_dirty,
    pub fifo_reserve_size: usize,
    pub dev_priv: *mut vmw_private,
    pub unit: *mut vmw_display_unit,
    pub cmd: *mut c_void,
    pub crtc: *mut drm_crtc,
    pub num_hits: u32,
    pub fb_x: i32,
    pub fb_y: i32,
    pub unit_x1: i32,
    pub unit_y1: i32,
    pub unit_x2: i32,
    pub unit_y2: i32,
}

//
// Base class for framebuffers
//
// @pin is called the when ever a crtc uses this framebuffer
// @unpin is called
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_framebuffer {
    pub base: drm_framebuffer,
    pub bo: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_framebuffer_surface {
    pub base: vmw_framebuffer,
    pub uo: vmw_user_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_framebuffer_bo {
    pub base: vmw_framebuffer,
    pub buffer: *mut vmw_bo,
}

//
// Derived class for crtc state object
//
// @base DRM crtc object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_crtc_state {
    pub base: drm_crtc_state,
}

//
// Derived class for plane state object
//
// @base DRM plane object
// @surf Display surface for STDU
// @bo display bo for SOU
// @content_fb_type Used by STDU.
// @bo_size Size of the bo, used by Screen Object Display Unit
// @pinned pin count for STDU display surface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_plane_state {
    pub base: drm_plane_state,
    pub uo: vmw_user_object,
    pub content_fb_type: c_int,
    pub bo_size: c_ulong,
    pub pinned: c_int,
// For CPU Blit
    pub cpp: c_uint,
    pub cursor: vmw_cursor_plane_state,
}

//
// Derived class for connector state object
//
// @base DRM connector object
// @is_implicit connector property
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_connector_state {
    pub base: drm_connector_state,
//
// @gui_x:
//
// vmwgfx connector property representing the x position of this display
// unit (connector is synonymous to display unit) in overall topology.
// This is what the device expect as xRoot while creating screen.
//
    pub gui_x: c_int,
//
// @gui_y:
//
// vmwgfx connector property representing the y position of this display
// unit (connector is synonymous to display unit) in overall topology.
// This is what the device expect as yRoot while creating screen.
//
    pub gui_y: c_int,
}

//
// Base class display unit.
//
// Since the SVGA hw doesn't have a concept of a crtc, encoder or connector
// so the display unit is all of them at the same time. This is true for both
// legacy multimon and screen objects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmw_display_unit {
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub primary: drm_plane,
    pub cursor: vmw_cursor_plane,
    pub unit: unsigned,
//
// Prefered mode tracking.
//
    pub pref_width: unsigned,
    pub pref_height: unsigned,
    pub pref_active: bool,
//
// Gui positioning
//
    pub gui_x: c_int,
    pub gui_y: c_int,
    pub is_implicit: bool,
    pub set_gui_x: c_int,
    pub set_gui_y: c_int,
    pub crc_generator_work: work_struct,
// protects concurrent access to the vblank handler
    pub atomic_lock: core::sync::atomic::AtomicI32,
// protected by @atomic_lock
    pub crc_enabled: bool,
    pub surface: *mut vmw_surface,
// protects concurrent access to the crc worker
    pub crc_state_lock: spinlock_t,
// protected by @crc_state_lock
    pub crc_pending: bool,
    pub frame_start: u64,
    pub frame_end: u64,
    pub vkms: },
}

//
// Shared display unit functions - vmwgfx_kms.c
//
extern "C" {
    pub fn vmw_du_init(du: *mut vmw_display_unit);
}
extern "C" {
    pub fn vmw_du_cleanup(du: *mut vmw_display_unit);
}
extern "C" {
    pub fn vmw_du_connector_dpms(connector: *mut drm_connector, mode: c_int) -> c_int;
}
extern "C" {
    pub fn vmw_du_connector_save(connector: *mut drm_connector);
}
extern "C" {
    pub fn vmw_du_connector_restore(connector: *mut drm_connector);
}
extern "C" {
    pub fn vmw_connector_get_modes(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn vmw_guess_mode_timing(mode: *mut drm_display_mode);
}
extern "C" {
    pub fn vmw_kms_update_implicit_fb(dev_priv: *mut vmw_private);
}
extern "C" {
    pub fn vmw_kms_create_implicit_placement_property(dev_priv: *mut vmw_private);
}
// Universal Plane Helpers
extern "C" {
    pub fn vmw_du_primary_plane_destroy(plane: *mut drm_plane);
}
// Atomic Helpers
extern "C" {
    pub fn vmw_du_plane_reset(plane: *mut drm_plane);
}
extern "C" {
    pub fn vmw_du_plane_unpin_surf(vps: *mut vmw_plane_state);
}
extern "C" {
    pub fn vmw_du_crtc_reset(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vmw_du_connector_reset(connector: *mut drm_connector);
}
//
// Legacy display unit functions - vmwgfx_ldu.c
//
extern "C" {
    pub fn vmw_kms_ldu_init_display(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_kms_ldu_close_display(dev_priv: *mut vmw_private) -> c_int;
}
//
// Screen Objects display functions - vmwgfx_scrn.c
//
extern "C" {
    pub fn vmw_kms_sou_init_display(dev_priv: *mut vmw_private) -> c_int;
}
//
// Screen Target Display Unit functions - vmwgfx_stdu.c
//
extern "C" {
    pub fn vmw_kms_stdu_init_display(dev_priv: *mut vmw_private) -> c_int;
}
extern "C" {
    pub fn vmw_du_helper_plane_update(update: *mut vmw_du_update_plane) -> c_int;
}
//
// vmw_du_translate_to_crtc - Translate a rect from framebuffer to crtc
// @state: Plane state.
// @r: Rectangle to translate.
//
