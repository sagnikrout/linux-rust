//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dvo_dev.h
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
// Copyright © 2006 Eric Anholt
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dvo_device {
    pub name: *const c_char,
    pub type: c_int,
// DVOA/B/C
    pub port: port,
// GPIO register used for i2c bus to control this device
    pub gpio: u32,
    pub target_addr: c_int,
    pub dev_ops: *const intel_dvo_dev_ops,
    pub dev_priv: *mut c_void,
    pub i2c_bus: *mut i2c_adapter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dvo_dev_ops {
//
// Initialize the device at startup time.
// Returns NULL if the device does not exist.
//
    pub i2cbus): *mut i2c_adapter,
//
// Turn on/off output.
//
// Because none of our dvo drivers support an intermediate power levels,
// we don't expose this in the interface.
//
    pub enable): *mut *mut *mut void (dpms)(struct intel_dvo_device dvo, bool,
//
// Callback for testing a video mode for a given output.
//
// This function should only check for cases where a mode can't
// be supported on the output specifically, and not represent
// generic CRTC limitations.
//
// \return MODE_OK if the mode is valid, or another MODE_* otherwise.
//
    pub mode): *const drm_display_mode,
//
// Callback for setting up a video mode after fixups have been made.
//
// This is only called while the output is disabled.  The dpms callback
// must be all that's necessary for the output, to turn the output on
// after this function is called.
//
    pub adjusted_mode): *const drm_display_mode,
//
// Probe for a connected output, and return detect_status.
//
    pub dvo): *mut *mut drm_connector_status (detect)(struct intel_dvo_device,
//
// Probe the current hw status, returning true if the connected output
// is active.
//
    pub dev): *mut *mut bool (get_hw_state)(struct intel_dvo_device,
//
// Clean up driver-specific bits of the output
//
    pub dvo): *mut *mut void (destroy) (struct intel_dvo_device,
//
// Debugging hook to dump device registers to log file
//
    pub dvo): *mut *mut void (dump_regs)(struct intel_dvo_device,
}
