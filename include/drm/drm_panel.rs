//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_panel.h
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


//
// Copyright (C) 2013, NVIDIA Corporation.  All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sub license,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

//
// struct drm_panel_funcs - perform operations on a given panel
//
// The .prepare() function is typically called before the display controller
// starts to transmit video data. Panel drivers can use this to turn the panel
// on and wait for it to become ready. If additional configuration is required
// (via a control bus such as I2C, SPI or DSI for example) this is a good time
// to do that.
//
// After the display controller has started transmitting video data, it's safe
// to call the .enable() function. This will typically enable the backlight to
// make the image on screen visible. Some panels require a certain amount of
// time or frames before the image is displayed. This function is responsible
// for taking this into account before enabling the backlight to avoid visual
// glitches.
//
// Before stopping video transmission from the display controller it can be
// necessary to turn off the panel to avoid visual glitches. This is done in
// the .disable() function. Analogously to .enable() this typically involves
// turning off the backlight and waiting for some time to make sure no image
// is visible on the panel. It is then safe for the display controller to
// cease transmission of video data.
//
// To save power when no video data is transmitted, a driver can power down
// the panel. This is the job of the .unprepare() function.
//
// Backlight can be handled automatically if configured using
// drm_panel_of_backlight() or drm_panel_dp_aux_backlight(). Then the driver
// does not need to implement the functionality to enable/disable backlight.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panel_funcs {
//
// @prepare:
//
// Turn on panel and perform set up.
//
// This function is optional.
//
    pub panel): *mut *mut int (prepare)(struct drm_panel,
//
// @enable:
//
// Enable panel (turn on back light, etc.).
//
// This function is optional.
//
    pub panel): *mut *mut int (enable)(struct drm_panel,
//
// @disable:
//
// Disable panel (turn off back light, etc.).
//
// This function is optional.
//
    pub panel): *mut *mut int (disable)(struct drm_panel,
//
// @unprepare:
//
// Turn off panel.
//
// This function is optional.
//
    pub panel): *mut *mut int (unprepare)(struct drm_panel,
//
// @get_modes:
//
// Add modes to the connector that the panel is attached to
// and returns the number of modes added.
//
// This function is mandatory.
//
    pub connector): *mut drm_connector,
//
// @get_orientation:
//
// Return the panel orientation set by device tree or EDID.
//
// This function is optional.
//
    pub panel): *mut *mut drm_panel_orientation (get_orientation)(struct drm_panel,
//
// @get_timings:
//
// Copy display timings into the provided array and return
// the number of display timings available.
//
// This function is optional.
//
    pub timings): *mut display_timing,
//
// @debugfs_init:
//
// Allows panels to create panels-specific debugfs files.
//
    pub root): *mut *mut *mut void (debugfs_init)(struct drm_panel panel, struct dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panel_follower_funcs {
//
// @panel_prepared:
//
// Called after the panel has been powered on.
//
    pub follower): *mut *mut int (panel_prepared)(struct drm_panel_follower,
//
// @panel_unpreparing:
//
// Called before the panel is powered off.
//
    pub follower): *mut *mut int (panel_unpreparing)(struct drm_panel_follower,
//
// @panel_enabled:
//
// Called after the panel and the backlight have been enabled.
//
    pub follower): *mut *mut int (panel_enabled)(struct drm_panel_follower,
//
// @panel_disabling:
//
// Called before the panel and the backlight are disabled.
//
    pub follower): *mut *mut int (panel_disabling)(struct drm_panel_follower,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panel_follower {
//
// @funcs:
//
// Dependent device callbacks; should be initted by the caller.
//
    pub funcs: *const drm_panel_follower_funcs,
//
// @list
//
// Used for linking into panel's list; set by drm_panel_add_follower().
//
    pub list: list_head,
//
// @panel
//
// The panel we're dependent on; set by drm_panel_add_follower().
//
    pub panel: *mut drm_panel,
}

//
// struct drm_panel - DRM panel object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_panel {
//
// @dev:
//
// Parent device of the panel.
//
    pub dev: *mut device,
//
// @backlight:
//
// Backlight device, used to turn on backlight after the call
// to enable(), and to turn off backlight before the call to
// disable().
// backlight is set by drm_panel_of_backlight() or
// drm_panel_dp_aux_backlight() and drivers shall not assign it.
//
    pub backlight: *mut backlight_device,
//
// @funcs:
//
// Operations that can be performed on the panel.
//
    pub funcs: *const drm_panel_funcs,
//
// @connector_type:
//
// Type of the panel as a DRM_MODE_CONNECTOR_* value. This is used to
// initialise the drm_connector corresponding to the panel with the
// correct connector type.
//
    pub connector_type: c_int,
//
// @list:
//
// Panel entry in registry.
//
    pub list: list_head,
//
// @followers:
//
// A list of struct drm_panel_follower dependent on this panel.
//
    pub followers: list_head,
//
// @follower_lock:
//
// Lock for followers list.
//
    pub follower_lock: mutex,
//
// @prepare_prev_first:
//
// The previous controller should be prepared first, before the prepare
// for the panel is called. This is largely required for DSI panels
// where the DSI host controller should be initialised to LP-11 before
// the panel is powered up.
//
    pub prepare_prev_first: bool,
//
// @prepared:
//
// If true then the panel has been prepared.
//
    pub prepared: bool,
//
// @enabled:
//
// If true then the panel has been enabled.
//
    pub enabled: bool,
//
// @container: Pointer to the private driver struct embedding this
// @struct drm_panel.
//
    pub container: *mut c_void,
//
// @refcount: reference count of users referencing this panel.
//
    pub refcount: kref,
}

//
// devm_drm_panel_alloc - Allocate and initialize a refcounted panel.
//
// @dev: struct device of the panel device
// @type: the type of the struct which contains struct &drm_panel
// @member: the name of the &drm_panel within @type
// @funcs: callbacks for this panel
// @connector_type: the connector type (DRM_MODE_CONNECTOR_*) corresponding to
// the panel interface
//
// The reference count of the returned panel is initialized to 1. This
// reference will be automatically dropped via devm (by calling
// drm_panel_put()) when @dev is removed.
//
// Returns:
// Pointer to container structure embedding the panel, ERR_PTR on failure.
//

extern "C" {
    pub fn drm_panel_put(panel: *mut drm_panel);
}
extern "C" {
    pub fn drm_panel_add(panel: *mut drm_panel);
}
extern "C" {
    pub fn drm_panel_remove(panel: *mut drm_panel);
}
extern "C" {
    pub fn devm_drm_panel_add(dev: *mut device, panel: *mut drm_panel) -> c_int;
}
extern "C" {
    pub fn drm_panel_prepare(panel: *mut drm_panel);
}
extern "C" {
    pub fn drm_panel_unprepare(panel: *mut drm_panel);
}
extern "C" {
    pub fn drm_panel_enable(panel: *mut drm_panel);
}
extern "C" {
    pub fn drm_panel_disable(panel: *mut drm_panel);
}
extern "C" {
    pub fn drm_panel_get_modes(panel: *mut drm_panel, connector: *mut drm_connector) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn drm_is_panel_follower(dev: *mut device) -> bool;
}
extern "C" {
    pub fn drm_panel_remove_follower(follower: *mut drm_panel_follower);
}

extern "C" {
    pub fn drm_panel_of_backlight(panel: *mut drm_panel) -> c_int;
}

