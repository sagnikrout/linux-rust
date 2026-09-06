//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/dispnv04/i2c/encoder_i2c.h
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
// Copyright (C) 2009 Francisco Jerez.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

//
// struct nouveau_i2c_encoder_funcs - Entry points exposed by a I2C encoder driver
//
// Most of its members are analogous to the function pointers in
// &drm_encoder_helper_funcs and they can optionally be used to
// initialize the latter. Connector-like methods (e.g. @get_modes and
// @set_property) will typically be wrapped around and only be called
// if the encoder is the currently selected one for the connector.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_i2c_encoder_funcs {
//
// @set_config: Initialize any encoder-specific modesetting parameters.
// The meaning of the @params parameter is implementation dependent. It
// will usually be a structure with DVO port data format settings or
// timings. It's not required for the new parameters to take effect
// until the next mode is set.
//
    pub params): *mut c_void,
//
// @destroy: Analogous to &drm_encoder_funcs @destroy callback.
//
    pub encoder): *mut *mut void (destroy)(struct drm_encoder,
//
// @dpms: Analogous to &drm_encoder_helper_funcs @dpms callback.
//
    pub mode): *mut *mut *mut void (dpms)(struct drm_encoder encoder, int,
//
// @save: Save state. Wrapped by nouveau_i2c_encoder_save().
//
    pub encoder): *mut *mut void (save)(struct drm_encoder,
//
// @restore: Restore state. Wrapped by nouveau_i2c_encoder_restore().
//
    pub encoder): *mut *mut void (restore)(struct drm_encoder,
//
// @mode_fixup: Analogous to &drm_encoder_helper_funcs @mode_fixup
// callback. Wrapped by nouveau_i2c_encoder_mode_fixup().
//
    pub adjusted_mode): *mut drm_display_mode,
//
// @mode_valid: Analogous to &drm_encoder_helper_funcs @mode_valid.
//
    pub mode): *const drm_display_mode,
//
// @mode_set: Analogous to &drm_encoder_helper_funcs @mode_set
// callback.
//
    pub adjusted_mode): *mut drm_display_mode,
//
// @detect: Analogous to &drm_encoder_helper_funcs @detect
// callback. Wrapped by nouveau_i2c_encoder_detect().
//
    pub connector): *mut drm_connector,
//
// @get_modes: Get modes.
//
    pub connector): *mut drm_connector,
//
// @create_resources: Create resources.
//
    pub connector): *mut drm_connector,
//
// @set_property: Set property.
//
    pub val): u64,
}

//
// struct nouveau_i2c_encoder - I2C encoder struct
//
// A &nouveau_i2c_encoder has two sets of callbacks, @encoder_i2c_funcs and the
// ones in @base. The former are never actually called by the common
// CRTC code, it's just a convenience for splitting the encoder
// functions in an upper, GPU-specific layer and a (hopefully)
// GPU-agnostic lower layer: It's the GPU driver responsibility to
// call the nouveau_i2c_encoder methods when appropriate.
//
// nouveau_i2c_encoder_init() provides a way to get an implementation of
// this.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_i2c_encoder {
//
// @base: DRM encoder object.
//
    pub base: drm_encoder,
//
// @encoder_i2c_funcs: I2C encoder callbacks.
//
    pub encoder_i2c_funcs: *const nouveau_i2c_encoder_funcs,
//
// @encoder_i2c_priv: I2C encoder private data.
//
    pub encoder_i2c_priv: *mut c_void,
//
// @i2c_client: corresponding I2C client structure
//
    pub i2c_client: *mut i2c_client,
}

//
// struct nouveau_i2c_encoder_driver
//
// Describes a device driver for an encoder connected to the GPU through an I2C
// bus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_i2c_encoder_driver {
//
// @i2c_driver: I2C device driver description.
//
    pub i2c_driver: i2c_driver,
//
// @encoder_init: Callback to allocate any per-encoder data structures
// and to initialize the @encoder_i2c_funcs and (optionally) @encoder_i2c_priv
// members of @encoder.
//
    pub encoder): *mut nouveau_i2c_encoder,
}

//
// nouveau_i2c_encoder_get_client - Get the I2C client corresponding to an encoder
// @encoder: The encoder
//
// nouveau_i2c_encoder_destroy - Unregister the I2C device backing an encoder
// @drm_encoder:        Encoder to be unregistered.
//
// This should be called from the @destroy method of an I2C slave
// encoder driver once I2C access is no longer needed.
//
// Wrapper fxns which can be plugged in to drm_encoder_helper_funcs:
//
extern "C" {
    pub fn nouveau_i2c_encoder_save(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn nouveau_i2c_encoder_restore(encoder: *mut drm_encoder);
}
