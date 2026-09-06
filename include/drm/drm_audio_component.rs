//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_audio_component.h
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
// Copyright © 2014 Intel Corporation

//
// struct drm_audio_component_ops - Ops implemented by DRM driver, called by hda driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_audio_component_ops {
//
// @owner: drm module to pin down
//
    pub owner: *mut module,
//
// @get_power: get the POWER_DOMAIN_AUDIO power well
//
// Request the power well to be turned on.
//
// Returns a wakeref cookie to be passed back to the corresponding
// call to @put_power.
//
    pub ): *mut *mut unsigned long (get_power)(struct device,
//
// @put_power: put the POWER_DOMAIN_AUDIO power well
//
// Allow the power well to be turned off.
//
    pub long): *mut *mut *mut void (put_power)(struct device , unsigned,
//
// @codec_wake_override: Enable/disable codec wake signal
//
    pub enable): *mut *mut *mut void (codec_wake_override)(struct device , bool,
//
// @get_cdclk_freq: Get the Core Display Clock in kHz
//
    pub ): *mut *mut int (get_cdclk_freq)(struct device,
//
// @sync_audio_rate: set n/cts based on the sample rate
//
// Called from audio driver. After audio driver sets the
// sample rate, it will call this function to set n/cts
//
    pub rate): *mut *mut *mut int (sync_audio_rate)(struct device , int port, int pipe, int,
//
// @get_eld: fill the audio state and ELD bytes for the given port
//
// Called from audio driver to get the HDMI/DP audio state of the given
// digital port, and also fetch ELD bytes to the given pointer.
//
// It returns the byte size of the original ELD (not the actually
// copied size), zero for an invalid ELD, or a negative error code.
//
// Note that the returned size may be over @max_bytes.  Then it
// implies that only a part of ELD has been copied to the buffer.
//
    pub max_bytes): *mut *mut unsigned char buf, int,
}

//
// struct drm_audio_component_audio_ops - Ops implemented by hda driver, called by DRM driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_audio_component_audio_ops {
//
// @audio_ptr: Pointer to be used in call to pin_eld_notify
//
    pub audio_ptr: *mut c_void,
//
// @pin_eld_notify: Notify the HDA driver that pin sense and/or ELD information has changed
//
// Called when the DRM driver has set up audio pipeline or has just
// begun to tear it down. This allows the HDA driver to update its
// status accordingly (even when the HDA controller is in power save
// mode).
//
    pub pipe): *mut *mut *mut void (pin_eld_notify)(void audio_ptr, int port, int,
//
// @pin2port: Check and convert from pin node to port number
//
// Called by HDA driver to check and convert from the pin widget node
// number to a port number in the graphics side.
//
    pub pin): *mut *mut *mut int (pin2port)(void audio_ptr, int,
//
// @master_bind: (Optional) component master bind callback
//
// Called at binding master component, for HDA codec-specific
// handling of dynamic binding.
//
    pub ): *mut *mut *mut int (master_bind)(struct device dev, struct drm_audio_component,
//
// @master_unbind: (Optional) component master unbind callback
//
// Called at unbinding master component, for HDA codec-specific
// handling of dynamic unbinding.
//
    pub ): *mut *mut *mut void (master_unbind)(struct device dev, struct drm_audio_component,
}

//
// struct drm_audio_component - Used for direct communication between DRM and hda drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_audio_component {
//
// @dev: DRM device, used as parameter for ops
//
    pub dev: *mut device,
//
// @ops: Ops implemented by DRM driver, called by hda driver
//
    pub ops: *const drm_audio_component_ops,
//
// @audio_ops: Ops implemented by hda driver, called by DRM driver
//
    pub audio_ops: *const drm_audio_component_audio_ops,
//
// @master_bind_complete: completion held during component master binding
//
    pub master_bind_complete: completion,
}
