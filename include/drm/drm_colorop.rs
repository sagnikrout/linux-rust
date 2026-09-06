//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_colorop.h
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
// Copyright (C) 2023 Advanced Micro Devices, Inc. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

// DRM colorop flags

//
// enum drm_colorop_curve_1d_type - type of 1D curve
//
// Describes a 1D curve to be applied by the DRM_COLOROP_1D_CURVE colorop.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_colorop_curve_1d_type {
//
// @DRM_COLOROP_1D_CURVE_SRGB_EOTF:
//
// enum string "sRGB EOTF"
//
// sRGB piece-wise electro-optical transfer function. Transfer
// characteristics as defined by IEC 61966-2-1 sRGB. Equivalent
// to H.273 TransferCharacteristics code point 13 with
// MatrixCoefficients set to 0.
//
    DRM_COLOROP_1D_CURVE_SRGB_EOTF,

//
// @DRM_COLOROP_1D_CURVE_SRGB_INV_EOTF:
//
// enum string "sRGB Inverse EOTF"
//
// The inverse of &DRM_COLOROP_1D_CURVE_SRGB_EOTF
//
    DRM_COLOROP_1D_CURVE_SRGB_INV_EOTF,

//
// @DRM_COLOROP_1D_CURVE_PQ_125_EOTF:
//
// enum string "PQ 125 EOTF"
//
// The PQ transfer function, scaled by 125.0f, so that 10,000
// nits correspond to 125.0f.
//
// Transfer characteristics of the PQ function as defined by
// SMPTE ST 2084 (2014) for 10-, 12-, 14-, and 16-bit systems
// and Rec. ITU-R BT.2100-2 perceptual quantization (PQ) system,
// represented by H.273 TransferCharacteristics code point 16.
//
    DRM_COLOROP_1D_CURVE_PQ_125_EOTF,

//
// @DRM_COLOROP_1D_CURVE_PQ_125_INV_EOTF:
//
// enum string "PQ 125 Inverse EOTF"
//
// The inverse of DRM_COLOROP_1D_CURVE_PQ_125_EOTF.
//
    DRM_COLOROP_1D_CURVE_PQ_125_INV_EOTF,

//
// @DRM_COLOROP_1D_CURVE_BT2020_INV_OETF:
//
// enum string "BT.2020 Inverse OETF"
//
// The inverse of &DRM_COLOROP_1D_CURVE_BT2020_OETF
//
    DRM_COLOROP_1D_CURVE_BT2020_INV_OETF,

//
// @DRM_COLOROP_1D_CURVE_BT2020_OETF:
//
// enum string "BT.2020 OETF"
//
// The BT.2020/BT.709 transfer function. The BT.709 and BT.2020
// transfer functions are the same, the only difference is that
// BT.2020 is defined with more precision for 10 and 12-bit
// encodings.
//
    DRM_COLOROP_1D_CURVE_BT2020_OETF,

//
// @DRM_COLOROP_1D_CURVE_GAMMA22:
//
// enum string "Gamma 2.2"
//
// A gamma 2.2 power function. This applies a power curve with
// gamma value of 2.2 to the input values.
//
    DRM_COLOROP_1D_CURVE_GAMMA22,

//
// @DRM_COLOROP_1D_CURVE_GAMMA22_INV:
//
// enum string "Gamma 2.2 Inverse"
//
// The inverse of &DRM_COLOROP_1D_CURVE_GAMMA22
//
    DRM_COLOROP_1D_CURVE_GAMMA22_INV,
//
// @DRM_COLOROP_1D_CURVE_COUNT:
//
// enum value denoting the size of the enum
//
    DRM_COLOROP_1D_CURVE_COUNT
}

//
// struct drm_colorop_state - mutable colorop state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_colorop_state {
// @colorop: backpointer to the colorop
    pub colorop: *mut drm_colorop,
//
// Color properties
//
// The following fields are not always valid, their usage depends
// on the colorop type. See their associated comment for more
// information.
//
// @bypass:
//
// When the property BYPASS exists on this colorop, this stores
// the requested bypass state: true if colorop shall be bypassed,
// false if colorop is enabled.
//
    pub bypass: bool,
//
// @curve_1d_type:
//
// Type of 1D curve.
//
    pub curve_1d_type: drm_colorop_curve_1d_type,
//
// @multiplier:
//
// Multiplier to 'gain' the plane. Format is S31.32 sign-magnitude.
//
    pub multiplier: u64,
//
// @data:
//
// Data blob for any TYPE that requires such a blob. The
// interpretation of the blob is TYPE-specific.
//
// See the &drm_colorop_type documentation for how blob is laid
// out.
//
    pub data: *mut drm_property_blob,
//
// @lut1d_interpolation:
//
// Interpolation for DRM_COLOROP_1D_LUT
//
    pub lut1d_interpolation: drm_colorop_lut1d_interpolation_type,
//
// @lut3d_interpolation:
//
// Interpolation for DRM_COLOROP_3D_LUT
//
    pub lut3d_interpolation: drm_colorop_lut3d_interpolation_type,
// @state: backpointer to global drm_atomic_commit
    pub state: *mut drm_atomic_commit,
}

//
// struct drm_colorop_funcs - driver colorop control functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_colorop_funcs {
//
// @destroy:
//
// Clean up colorop resources. This is called at driver unload time
// through drm_mode_config_cleanup()
//
    pub colorop): *mut *mut void (destroy)(struct drm_colorop,
}

//
// struct drm_colorop - DRM color operation control structure
//
// A colorop represents one color operation. They can be chained via
// the 'next' pointer to build a color pipeline.
//
// Since colorops cannot stand-alone and are used to describe colorop
// operations on a plane they don't have their own locking mechanism but
// are locked and programmed along with their associated &drm_plane.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_colorop {
// @dev: parent DRM device
    pub dev: *mut drm_device,
//
// @head:
//
// List of all colorops on @dev, linked from &drm_mode_config.colorop_list.
// Invariant over the lifetime of @dev and therefore does not need
// locking.
//
    pub head: list_head,
//
// @index: Position inside the mode_config.list, can be used as an array
// index. It is invariant over the lifetime of the colorop.
//
    pub index: c_uint,
// @base: base mode object
    pub base: drm_mode_object,
//
// @plane:
//
// The plane on which the colorop sits. A drm_colorop is always unique
// to a plane.
//
    pub plane: *mut drm_plane,
//
// @state:
//
// Current atomic state for this colorop.
//
// This is protected by @mutex. Note that nonblocking atomic commits
// access the current colorop state without taking locks.
//
    pub state: *mut drm_colorop_state,
//
// Color properties
//
// The following fields are not always valid, their usage depends
// on the colorop type. See their associated comment for more
// information.
//
// @properties: property tracking for this colorop
    pub properties: drm_object_properties,
//
// @type:
//
// Read-only
// Type of color operation
//
    pub type: drm_colorop_type,
//
// @next:
//
// Read-only
// Pointer to next drm_colorop in pipeline
//
    pub next: *mut drm_colorop,
//
// @type_property:
//
// Read-only "TYPE" property for specifying the type of
// this color operation. The type is enum drm_colorop_type.
//
    pub type_property: *mut drm_property,
//
// @bypass_property:
//
// Boolean property to control enablement of the color
// operation. Only present if DRM_COLOROP_FLAG_ALLOW_BYPASS
// flag is set. When present, setting bypass to "true" shall
// always be supported to allow compositors to quickly fall
// back to alternate methods of color processing. This is
// important since setting color operations can fail due to
// unique HW constraints.
//
    pub bypass_property: *mut drm_property,
//
// @size:
//
// Number of entries of the custom LUT. This should be read-only.
//
    pub size: u32,
//
// @lut1d_interpolation_property:
//
// Property for DRM_COLOROP_1D_LUT interpolation
//
    pub lut1d_interpolation_property: *mut drm_property,
//
// @curve_1d_type_property:
//
// Sub-type for DRM_COLOROP_1D_CURVE type.
//
    pub curve_1d_type_property: *mut drm_property,
//
// @multiplier_property:
//
// Multiplier property for plane gain
//
    pub multiplier_property: *mut drm_property,
//
// @size_property:
//
// Size property for custom LUT from userspace.
//
    pub size_property: *mut drm_property,
//
// @lut3d_interpolation_property:
//
// Property for DRM_COLOROP_3D_LUT interpolation
//
    pub lut3d_interpolation_property: *mut drm_property,
//
// @data_property:
//
// blob property for any TYPE that requires a blob of data,
// such as 1DLUT, CTM, 3DLUT, etc.
//
// The way this blob is interpreted depends on the TYPE of
// this
//
    pub data_property: *mut drm_property,
//
// @next_property:
//
// Read-only property to next colorop in the pipeline
//
    pub next_property: *mut drm_property,
// @funcs: colorop control functions
    pub funcs: *const drm_colorop_funcs,
}

//
// drm_colorop_find - look up a Colorop object from its ID
// @dev: DRM device
// @file_priv: drm file to check for lease against.
// @id: &drm_mode_object ID
//
// This can be used to look up a Colorop from its userspace ID. Only used by
// drivers for legacy IOCTLs and interface, nowadays extensions to the KMS
// userspace interface should be done using &drm_property.
//
extern "C" {
    pub fn drm_colorop_pipeline_destroy(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_colorop_cleanup(colorop: *mut drm_colorop);
}
//
// drm_colorop_reset - reset colorop atomic state
// @colorop: drm colorop
//
// Resets the atomic state for @colorop by freeing the state pointer (which might
// be NULL, e.g. at driver load time) and allocating a new empty state object.
//
extern "C" {
    pub fn drm_colorop_reset(colorop: *mut drm_colorop);
}
extern "C" {
    pub fn drm_colorop_destroy(colorop: *mut drm_colorop);
}
//
// drm_colorop_index - find the index of a registered colorop
// @colorop: colorop to find index for
//
// Given a registered colorop, return the index of that colorop within a DRM
// device's list of colorops.
//

//
// drm_get_colorop_type_name - return a string for colorop type
// @type: colorop type to compute name of
//
// In contrast to the other drm_get_*_name functions this one here returns a
// const pointer and hence is threadsafe.
//
// drm_get_colorop_curve_1d_type_name - return a string for 1D curve type
// @type: 1d curve type to compute name of
//
// In contrast to the other drm_get_*_name functions this one here returns a
// const pointer and hence is threadsafe.
//
extern "C" {
    pub fn drm_colorop_set_next_property(colorop: *mut drm_colorop, next: *mut drm_colorop);
}
