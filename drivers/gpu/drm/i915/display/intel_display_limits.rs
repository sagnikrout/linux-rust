//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_limits.h
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
//
// Copyright © 2022 Intel Corporation
//
// Keep the pipe enum values fixed: the code assumes that PIPE_A=0, the
// rest have consecutive values and match the enum values of transcoders
// with a 1:1 transcoder -> pipe mapping.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pipe {
    INVALID_PIPE = -1,

    PIPE_A = 0,
    PIPE_B,
    PIPE_C,
    PIPE_D,
    _PIPE_EDP,

    I915_MAX_PIPES = _PIPE_EDP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transcoder {
    INVALID_TRANSCODER = -1,
//
// The following transcoders have a 1:1 transcoder -> pipe mapping,
// keep their values fixed: the code assumes that TRANSCODER_A=0, the
// rest have consecutive values and match the enum values of the pipes
// they map to.
//
    TRANSCODER_A = PIPE_A,
    TRANSCODER_B = PIPE_B,
    TRANSCODER_C = PIPE_C,
    TRANSCODER_D = PIPE_D,

//
// The following transcoders can map to any pipe, their enum value
// doesn't need to stay fixed.
//
    TRANSCODER_EDP,
    TRANSCODER_DSI_0,
    TRANSCODER_DSI_1,
    TRANSCODER_DSI_A = TRANSCODER_DSI_0,	/* legacy DSI */
    TRANSCODER_DSI_C = TRANSCODER_DSI_1,	/* legacy DSI */
    TRANSCODER_CMTG0,
    TRANSCODER_CMTG1,

    I915_MAX_TRANSCODERS
}

//
// Global legacy plane identifier. Valid only for primary/sprite
// planes on pre-g4x, and only for primary planes on g4x-bdw.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i9xx_plane_id {
    PLANE_A,
    PLANE_B,
    PLANE_C,
}

//
// Per-pipe plane identifier.
// I915_MAX_PLANES in the enum below is the maximum (across all platforms)
// number of planes per CRTC.  Not all platforms really have this many planes,
// which means some arrays of size I915_MAX_PLANES may have unused entries
// between the topmost sprite plane and the cursor plane.
//
// This is expected to be passed to various register macros
// (eg. PLANE_CTL(), PS_PLANE_SEL(), etc.) so adjust with care.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum plane_id {
// skl+ universal plane names
    PLANE_1,
    PLANE_2,
    PLANE_3,
    PLANE_4,
    PLANE_5,
    PLANE_6,
    PLANE_7,

    PLANE_CURSOR,

    I915_MAX_PLANES,

// pre-skl plane names
    PLANE_PRIMARY = PLANE_1,
    PLANE_SPRITE0,
    PLANE_SPRITE1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port {
    PORT_NONE = -1,

    PORT_A = 0,
    PORT_B,
    PORT_C,
    PORT_D,
    PORT_E,
    PORT_F,
    PORT_G,
    PORT_H,
    PORT_I,

// tgl+
    PORT_TC1 = PORT_D,
    PORT_TC2,
    PORT_TC3,
    PORT_TC4,
    PORT_TC5,
    PORT_TC6,

// XE_LPD repositions D/E offsets and bitfields
    PORT_D_XELPD = PORT_TC5,
    PORT_E_XELPD,

    I915_MAX_PORTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpd_pin {
    HPD_NONE = 0,
    HPD_TV = HPD_NONE,     /* TV is known to be unreliable */
    HPD_CRT,
    HPD_SDVO_B,
    HPD_SDVO_C,
    HPD_PORT_A,
    HPD_PORT_B,
    HPD_PORT_C,
    HPD_PORT_D,
    HPD_PORT_E,
    HPD_PORT_TC1,
    HPD_PORT_TC2,
    HPD_PORT_TC3,
    HPD_PORT_TC4,
    HPD_PORT_TC5,
    HPD_PORT_TC6,

    HPD_NUM_PINS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_ch {
    AUX_CH_NONE = -1,

    AUX_CH_A,
    AUX_CH_B,
    AUX_CH_C,
    AUX_CH_D,
    AUX_CH_E, /* ICL+ */
    AUX_CH_F,
    AUX_CH_G,
    AUX_CH_H,
    AUX_CH_I,

// tgl+
    AUX_CH_USBC1 = AUX_CH_D,
    AUX_CH_USBC2,
    AUX_CH_USBC3,
    AUX_CH_USBC4,
    AUX_CH_USBC5,
    AUX_CH_USBC6,

// XE_LPD repositions D/E offsets and bitfields
    AUX_CH_D_XELPD = AUX_CH_USBC5,
    AUX_CH_E_XELPD,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_color_block {
    INTEL_PLANE_CB_PRE_CSC_LUT,
    INTEL_PLANE_CB_CSC,
    INTEL_PLANE_CB_POST_CSC_LUT,
    INTEL_PLANE_CB_3DLUT,

    INTEL_CB_MAX
}
