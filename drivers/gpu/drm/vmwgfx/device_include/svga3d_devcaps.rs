//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/device_include/svga3d_devcaps.h
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
// Copyright 1998-2021 VMware, Inc.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// svga3d_devcaps.h --
//
// SVGA 3d caps definitions
//

pub type SVGA3dDevCapIndex = uint32;

pub const SVGA3D_DEVCAP_3D: c_int = 0;
pub const SVGA3D_DEVCAP_MAX_LIGHTS: c_int = 1;
pub const SVGA3D_DEVCAP_MAX_TEXTURES: c_int = 2;
pub const SVGA3D_DEVCAP_MAX_CLIP_PLANES: c_int = 3;
pub const SVGA3D_DEVCAP_VERTEX_SHADER_VERSION: c_int = 4;
pub const SVGA3D_DEVCAP_VERTEX_SHADER: c_int = 5;
pub const SVGA3D_DEVCAP_FRAGMENT_SHADER_VERSION: c_int = 6;
pub const SVGA3D_DEVCAP_FRAGMENT_SHADER: c_int = 7;
pub const SVGA3D_DEVCAP_MAX_RENDER_TARGETS: c_int = 8;
pub const SVGA3D_DEVCAP_S23E8_TEXTURES: c_int = 9;
pub const SVGA3D_DEVCAP_S10E5_TEXTURES: c_int = 10;
pub const SVGA3D_DEVCAP_MAX_FIXED_VERTEXBLEND: c_int = 11;
pub const SVGA3D_DEVCAP_D16_BUFFER_FORMAT: c_int = 12;
pub const SVGA3D_DEVCAP_D24S8_BUFFER_FORMAT: c_int = 13;
pub const SVGA3D_DEVCAP_D24X8_BUFFER_FORMAT: c_int = 14;
pub const SVGA3D_DEVCAP_QUERY_TYPES: c_int = 15;
pub const SVGA3D_DEVCAP_TEXTURE_GRADIENT_SAMPLING: c_int = 16;
pub const SVGA3D_DEVCAP_MAX_POINT_SIZE: c_int = 17;
pub const SVGA3D_DEVCAP_MAX_SHADER_TEXTURES: c_int = 18;
pub const SVGA3D_DEVCAP_MAX_TEXTURE_WIDTH: c_int = 19;
pub const SVGA3D_DEVCAP_MAX_TEXTURE_HEIGHT: c_int = 20;
pub const SVGA3D_DEVCAP_MAX_VOLUME_EXTENT: c_int = 21;
pub const SVGA3D_DEVCAP_MAX_TEXTURE_REPEAT: c_int = 22;
pub const SVGA3D_DEVCAP_MAX_TEXTURE_ASPECT_RATIO: c_int = 23;
pub const SVGA3D_DEVCAP_MAX_TEXTURE_ANISOTROPY: c_int = 24;
pub const SVGA3D_DEVCAP_MAX_PRIMITIVE_COUNT: c_int = 25;
pub const SVGA3D_DEVCAP_MAX_VERTEX_INDEX: c_int = 26;
pub const SVGA3D_DEVCAP_MAX_VERTEX_SHADER_INSTRUCTIONS: c_int = 27;
pub const SVGA3D_DEVCAP_MAX_FRAGMENT_SHADER_INSTRUCTIONS: c_int = 28;
pub const SVGA3D_DEVCAP_MAX_VERTEX_SHADER_TEMPS: c_int = 29;
pub const SVGA3D_DEVCAP_MAX_FRAGMENT_SHADER_TEMPS: c_int = 30;
pub const SVGA3D_DEVCAP_TEXTURE_OPS: c_int = 31;
pub const SVGA3D_DEVCAP_SURFACEFMT_X8R8G8B8: c_int = 32;
pub const SVGA3D_DEVCAP_SURFACEFMT_A8R8G8B8: c_int = 33;
pub const SVGA3D_DEVCAP_SURFACEFMT_A2R10G10B10: c_int = 34;
pub const SVGA3D_DEVCAP_SURFACEFMT_X1R5G5B5: c_int = 35;
pub const SVGA3D_DEVCAP_SURFACEFMT_A1R5G5B5: c_int = 36;
pub const SVGA3D_DEVCAP_SURFACEFMT_A4R4G4B4: c_int = 37;
pub const SVGA3D_DEVCAP_SURFACEFMT_R5G6B5: c_int = 38;
pub const SVGA3D_DEVCAP_SURFACEFMT_LUMINANCE16: c_int = 39;
pub const SVGA3D_DEVCAP_SURFACEFMT_LUMINANCE8_ALPHA8: c_int = 40;
pub const SVGA3D_DEVCAP_SURFACEFMT_ALPHA8: c_int = 41;
pub const SVGA3D_DEVCAP_SURFACEFMT_LUMINANCE8: c_int = 42;
pub const SVGA3D_DEVCAP_SURFACEFMT_Z_D16: c_int = 43;
pub const SVGA3D_DEVCAP_SURFACEFMT_Z_D24S8: c_int = 44;
pub const SVGA3D_DEVCAP_SURFACEFMT_Z_D24X8: c_int = 45;
pub const SVGA3D_DEVCAP_SURFACEFMT_DXT1: c_int = 46;
pub const SVGA3D_DEVCAP_SURFACEFMT_DXT2: c_int = 47;
pub const SVGA3D_DEVCAP_SURFACEFMT_DXT3: c_int = 48;
pub const SVGA3D_DEVCAP_SURFACEFMT_DXT4: c_int = 49;
pub const SVGA3D_DEVCAP_SURFACEFMT_DXT5: c_int = 50;
pub const SVGA3D_DEVCAP_SURFACEFMT_BUMPX8L8V8U8: c_int = 51;
pub const SVGA3D_DEVCAP_SURFACEFMT_A2W10V10U10: c_int = 52;
pub const SVGA3D_DEVCAP_SURFACEFMT_BUMPU8V8: c_int = 53;
pub const SVGA3D_DEVCAP_SURFACEFMT_Q8W8V8U8: c_int = 54;
pub const SVGA3D_DEVCAP_SURFACEFMT_CxV8U8: c_int = 55;
pub const SVGA3D_DEVCAP_SURFACEFMT_R_S10E5: c_int = 56;
pub const SVGA3D_DEVCAP_SURFACEFMT_R_S23E8: c_int = 57;
pub const SVGA3D_DEVCAP_SURFACEFMT_RG_S10E5: c_int = 58;
pub const SVGA3D_DEVCAP_SURFACEFMT_RG_S23E8: c_int = 59;
pub const SVGA3D_DEVCAP_SURFACEFMT_ARGB_S10E5: c_int = 60;
pub const SVGA3D_DEVCAP_SURFACEFMT_ARGB_S23E8: c_int = 61;
pub const SVGA3D_DEVCAP_MISSING62: c_int = 62;
pub const SVGA3D_DEVCAP_MAX_VERTEX_SHADER_TEXTURES: c_int = 63;
pub const SVGA3D_DEVCAP_MAX_SIMULTANEOUS_RENDER_TARGETS: c_int = 64;
pub const SVGA3D_DEVCAP_SURFACEFMT_V16U16: c_int = 65;
pub const SVGA3D_DEVCAP_SURFACEFMT_G16R16: c_int = 66;
pub const SVGA3D_DEVCAP_SURFACEFMT_A16B16G16R16: c_int = 67;
pub const SVGA3D_DEVCAP_SURFACEFMT_UYVY: c_int = 68;
pub const SVGA3D_DEVCAP_SURFACEFMT_YUY2: c_int = 69;
pub const SVGA3D_DEVCAP_DEAD4: c_int = 70;
pub const SVGA3D_DEVCAP_DEAD5: c_int = 71;
pub const SVGA3D_DEVCAP_DEAD7: c_int = 72;
pub const SVGA3D_DEVCAP_DEAD6: c_int = 73;
pub const SVGA3D_DEVCAP_AUTOGENMIPMAPS: c_int = 74;
pub const SVGA3D_DEVCAP_SURFACEFMT_NV12: c_int = 75;
pub const SVGA3D_DEVCAP_DEAD10: c_int = 76;
pub const SVGA3D_DEVCAP_MAX_CONTEXT_IDS: c_int = 77;
pub const SVGA3D_DEVCAP_MAX_SURFACE_IDS: c_int = 78;
pub const SVGA3D_DEVCAP_SURFACEFMT_Z_DF16: c_int = 79;
pub const SVGA3D_DEVCAP_SURFACEFMT_Z_DF24: c_int = 80;
pub const SVGA3D_DEVCAP_SURFACEFMT_Z_D24S8_INT: c_int = 81;
pub const SVGA3D_DEVCAP_SURFACEFMT_ATI1: c_int = 82;
pub const SVGA3D_DEVCAP_SURFACEFMT_ATI2: c_int = 83;
pub const SVGA3D_DEVCAP_DEAD1: c_int = 84;
pub const SVGA3D_DEVCAP_DEAD8: c_int = 85;
pub const SVGA3D_DEVCAP_DEAD9: c_int = 86;
pub const SVGA3D_DEVCAP_LINE_AA: c_int = 87;
pub const SVGA3D_DEVCAP_LINE_STIPPLE: c_int = 88;
pub const SVGA3D_DEVCAP_MAX_LINE_WIDTH: c_int = 89;
pub const SVGA3D_DEVCAP_MAX_AA_LINE_WIDTH: c_int = 90;
pub const SVGA3D_DEVCAP_SURFACEFMT_YV12: c_int = 91;
pub const SVGA3D_DEVCAP_DEAD3: c_int = 92;
pub const SVGA3D_DEVCAP_TS_COLOR_KEY: c_int = 93;
pub const SVGA3D_DEVCAP_DEAD2: c_int = 94;
pub const SVGA3D_DEVCAP_DXCONTEXT: c_int = 95;
pub const SVGA3D_DEVCAP_DEAD11: c_int = 96;
pub const SVGA3D_DEVCAP_DX_MAX_VERTEXBUFFERS: c_int = 97;
pub const SVGA3D_DEVCAP_DX_MAX_CONSTANT_BUFFERS: c_int = 98;
pub const SVGA3D_DEVCAP_DX_PROVOKING_VERTEX: c_int = 99;
pub const SVGA3D_DEVCAP_DXFMT_X8R8G8B8: c_int = 100;
pub const SVGA3D_DEVCAP_DXFMT_A8R8G8B8: c_int = 101;
pub const SVGA3D_DEVCAP_DXFMT_R5G6B5: c_int = 102;
pub const SVGA3D_DEVCAP_DXFMT_X1R5G5B5: c_int = 103;
pub const SVGA3D_DEVCAP_DXFMT_A1R5G5B5: c_int = 104;
pub const SVGA3D_DEVCAP_DXFMT_A4R4G4B4: c_int = 105;
pub const SVGA3D_DEVCAP_DXFMT_Z_D32: c_int = 106;
pub const SVGA3D_DEVCAP_DXFMT_Z_D16: c_int = 107;
pub const SVGA3D_DEVCAP_DXFMT_Z_D24S8: c_int = 108;
pub const SVGA3D_DEVCAP_DXFMT_Z_D15S1: c_int = 109;
pub const SVGA3D_DEVCAP_DXFMT_LUMINANCE8: c_int = 110;
pub const SVGA3D_DEVCAP_DXFMT_LUMINANCE4_ALPHA4: c_int = 111;
pub const SVGA3D_DEVCAP_DXFMT_LUMINANCE16: c_int = 112;
pub const SVGA3D_DEVCAP_DXFMT_LUMINANCE8_ALPHA8: c_int = 113;
pub const SVGA3D_DEVCAP_DXFMT_DXT1: c_int = 114;
pub const SVGA3D_DEVCAP_DXFMT_DXT2: c_int = 115;
pub const SVGA3D_DEVCAP_DXFMT_DXT3: c_int = 116;
pub const SVGA3D_DEVCAP_DXFMT_DXT4: c_int = 117;
pub const SVGA3D_DEVCAP_DXFMT_DXT5: c_int = 118;
pub const SVGA3D_DEVCAP_DXFMT_BUMPU8V8: c_int = 119;
pub const SVGA3D_DEVCAP_DXFMT_BUMPL6V5U5: c_int = 120;
pub const SVGA3D_DEVCAP_DXFMT_BUMPX8L8V8U8: c_int = 121;
pub const SVGA3D_DEVCAP_DXFMT_FORMAT_DEAD1: c_int = 122;
pub const SVGA3D_DEVCAP_DXFMT_ARGB_S10E5: c_int = 123;
pub const SVGA3D_DEVCAP_DXFMT_ARGB_S23E8: c_int = 124;
pub const SVGA3D_DEVCAP_DXFMT_A2R10G10B10: c_int = 125;
pub const SVGA3D_DEVCAP_DXFMT_V8U8: c_int = 126;
pub const SVGA3D_DEVCAP_DXFMT_Q8W8V8U8: c_int = 127;
pub const SVGA3D_DEVCAP_DXFMT_CxV8U8: c_int = 128;
pub const SVGA3D_DEVCAP_DXFMT_X8L8V8U8: c_int = 129;
pub const SVGA3D_DEVCAP_DXFMT_A2W10V10U10: c_int = 130;
pub const SVGA3D_DEVCAP_DXFMT_ALPHA8: c_int = 131;
pub const SVGA3D_DEVCAP_DXFMT_R_S10E5: c_int = 132;
pub const SVGA3D_DEVCAP_DXFMT_R_S23E8: c_int = 133;
pub const SVGA3D_DEVCAP_DXFMT_RG_S10E5: c_int = 134;
pub const SVGA3D_DEVCAP_DXFMT_RG_S23E8: c_int = 135;
pub const SVGA3D_DEVCAP_DXFMT_BUFFER: c_int = 136;
pub const SVGA3D_DEVCAP_DXFMT_Z_D24X8: c_int = 137;
pub const SVGA3D_DEVCAP_DXFMT_V16U16: c_int = 138;
pub const SVGA3D_DEVCAP_DXFMT_G16R16: c_int = 139;
pub const SVGA3D_DEVCAP_DXFMT_A16B16G16R16: c_int = 140;
pub const SVGA3D_DEVCAP_DXFMT_UYVY: c_int = 141;
pub const SVGA3D_DEVCAP_DXFMT_YUY2: c_int = 142;
pub const SVGA3D_DEVCAP_DXFMT_NV12: c_int = 143;
pub const SVGA3D_DEVCAP_DXFMT_FORMAT_DEAD2: c_int = 144;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32A32_TYPELESS: c_int = 145;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32A32_UINT: c_int = 146;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32A32_SINT: c_int = 147;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32_TYPELESS: c_int = 148;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32_FLOAT: c_int = 149;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32_UINT: c_int = 150;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32_SINT: c_int = 151;
pub const SVGA3D_DEVCAP_DXFMT_R16G16B16A16_TYPELESS: c_int = 152;
pub const SVGA3D_DEVCAP_DXFMT_R16G16B16A16_UINT: c_int = 153;
pub const SVGA3D_DEVCAP_DXFMT_R16G16B16A16_SNORM: c_int = 154;
pub const SVGA3D_DEVCAP_DXFMT_R16G16B16A16_SINT: c_int = 155;
pub const SVGA3D_DEVCAP_DXFMT_R32G32_TYPELESS: c_int = 156;
pub const SVGA3D_DEVCAP_DXFMT_R32G32_UINT: c_int = 157;
pub const SVGA3D_DEVCAP_DXFMT_R32G32_SINT: c_int = 158;
pub const SVGA3D_DEVCAP_DXFMT_R32G8X24_TYPELESS: c_int = 159;
pub const SVGA3D_DEVCAP_DXFMT_D32_FLOAT_S8X24_UINT: c_int = 160;
pub const SVGA3D_DEVCAP_DXFMT_R32_FLOAT_X8X24: c_int = 161;
pub const SVGA3D_DEVCAP_DXFMT_X32_G8X24_UINT: c_int = 162;
pub const SVGA3D_DEVCAP_DXFMT_R10G10B10A2_TYPELESS: c_int = 163;
pub const SVGA3D_DEVCAP_DXFMT_R10G10B10A2_UINT: c_int = 164;
pub const SVGA3D_DEVCAP_DXFMT_R11G11B10_FLOAT: c_int = 165;
pub const SVGA3D_DEVCAP_DXFMT_R8G8B8A8_TYPELESS: c_int = 166;
pub const SVGA3D_DEVCAP_DXFMT_R8G8B8A8_UNORM: c_int = 167;
pub const SVGA3D_DEVCAP_DXFMT_R8G8B8A8_UNORM_SRGB: c_int = 168;
pub const SVGA3D_DEVCAP_DXFMT_R8G8B8A8_UINT: c_int = 169;
pub const SVGA3D_DEVCAP_DXFMT_R8G8B8A8_SINT: c_int = 170;
pub const SVGA3D_DEVCAP_DXFMT_R16G16_TYPELESS: c_int = 171;
pub const SVGA3D_DEVCAP_DXFMT_R16G16_UINT: c_int = 172;
pub const SVGA3D_DEVCAP_DXFMT_R16G16_SINT: c_int = 173;
pub const SVGA3D_DEVCAP_DXFMT_R32_TYPELESS: c_int = 174;
pub const SVGA3D_DEVCAP_DXFMT_D32_FLOAT: c_int = 175;
pub const SVGA3D_DEVCAP_DXFMT_R32_UINT: c_int = 176;
pub const SVGA3D_DEVCAP_DXFMT_R32_SINT: c_int = 177;
pub const SVGA3D_DEVCAP_DXFMT_R24G8_TYPELESS: c_int = 178;
pub const SVGA3D_DEVCAP_DXFMT_D24_UNORM_S8_UINT: c_int = 179;
pub const SVGA3D_DEVCAP_DXFMT_R24_UNORM_X8: c_int = 180;
pub const SVGA3D_DEVCAP_DXFMT_X24_G8_UINT: c_int = 181;
pub const SVGA3D_DEVCAP_DXFMT_R8G8_TYPELESS: c_int = 182;
pub const SVGA3D_DEVCAP_DXFMT_R8G8_UNORM: c_int = 183;
pub const SVGA3D_DEVCAP_DXFMT_R8G8_UINT: c_int = 184;
pub const SVGA3D_DEVCAP_DXFMT_R8G8_SINT: c_int = 185;
pub const SVGA3D_DEVCAP_DXFMT_R16_TYPELESS: c_int = 186;
pub const SVGA3D_DEVCAP_DXFMT_R16_UNORM: c_int = 187;
pub const SVGA3D_DEVCAP_DXFMT_R16_UINT: c_int = 188;
pub const SVGA3D_DEVCAP_DXFMT_R16_SNORM: c_int = 189;
pub const SVGA3D_DEVCAP_DXFMT_R16_SINT: c_int = 190;
pub const SVGA3D_DEVCAP_DXFMT_R8_TYPELESS: c_int = 191;
pub const SVGA3D_DEVCAP_DXFMT_R8_UNORM: c_int = 192;
pub const SVGA3D_DEVCAP_DXFMT_R8_UINT: c_int = 193;
pub const SVGA3D_DEVCAP_DXFMT_R8_SNORM: c_int = 194;
pub const SVGA3D_DEVCAP_DXFMT_R8_SINT: c_int = 195;
pub const SVGA3D_DEVCAP_DXFMT_P8: c_int = 196;
pub const SVGA3D_DEVCAP_DXFMT_R9G9B9E5_SHAREDEXP: c_int = 197;
pub const SVGA3D_DEVCAP_DXFMT_R8G8_B8G8_UNORM: c_int = 198;
pub const SVGA3D_DEVCAP_DXFMT_G8R8_G8B8_UNORM: c_int = 199;
pub const SVGA3D_DEVCAP_DXFMT_BC1_TYPELESS: c_int = 200;
pub const SVGA3D_DEVCAP_DXFMT_BC1_UNORM_SRGB: c_int = 201;
pub const SVGA3D_DEVCAP_DXFMT_BC2_TYPELESS: c_int = 202;
pub const SVGA3D_DEVCAP_DXFMT_BC2_UNORM_SRGB: c_int = 203;
pub const SVGA3D_DEVCAP_DXFMT_BC3_TYPELESS: c_int = 204;
pub const SVGA3D_DEVCAP_DXFMT_BC3_UNORM_SRGB: c_int = 205;
pub const SVGA3D_DEVCAP_DXFMT_BC4_TYPELESS: c_int = 206;
pub const SVGA3D_DEVCAP_DXFMT_ATI1: c_int = 207;
pub const SVGA3D_DEVCAP_DXFMT_BC4_SNORM: c_int = 208;
pub const SVGA3D_DEVCAP_DXFMT_BC5_TYPELESS: c_int = 209;
pub const SVGA3D_DEVCAP_DXFMT_ATI2: c_int = 210;
pub const SVGA3D_DEVCAP_DXFMT_BC5_SNORM: c_int = 211;
pub const SVGA3D_DEVCAP_DXFMT_R10G10B10_XR_BIAS_A2_UNORM: c_int = 212;
pub const SVGA3D_DEVCAP_DXFMT_B8G8R8A8_TYPELESS: c_int = 213;
pub const SVGA3D_DEVCAP_DXFMT_B8G8R8A8_UNORM_SRGB: c_int = 214;
pub const SVGA3D_DEVCAP_DXFMT_B8G8R8X8_TYPELESS: c_int = 215;
pub const SVGA3D_DEVCAP_DXFMT_B8G8R8X8_UNORM_SRGB: c_int = 216;
pub const SVGA3D_DEVCAP_DXFMT_Z_DF16: c_int = 217;
pub const SVGA3D_DEVCAP_DXFMT_Z_DF24: c_int = 218;
pub const SVGA3D_DEVCAP_DXFMT_Z_D24S8_INT: c_int = 219;
pub const SVGA3D_DEVCAP_DXFMT_YV12: c_int = 220;
pub const SVGA3D_DEVCAP_DXFMT_R32G32B32A32_FLOAT: c_int = 221;
pub const SVGA3D_DEVCAP_DXFMT_R16G16B16A16_FLOAT: c_int = 222;
pub const SVGA3D_DEVCAP_DXFMT_R16G16B16A16_UNORM: c_int = 223;
pub const SVGA3D_DEVCAP_DXFMT_R32G32_FLOAT: c_int = 224;
pub const SVGA3D_DEVCAP_DXFMT_R10G10B10A2_UNORM: c_int = 225;
pub const SVGA3D_DEVCAP_DXFMT_R8G8B8A8_SNORM: c_int = 226;
pub const SVGA3D_DEVCAP_DXFMT_R16G16_FLOAT: c_int = 227;
pub const SVGA3D_DEVCAP_DXFMT_R16G16_UNORM: c_int = 228;
pub const SVGA3D_DEVCAP_DXFMT_R16G16_SNORM: c_int = 229;
pub const SVGA3D_DEVCAP_DXFMT_R32_FLOAT: c_int = 230;
pub const SVGA3D_DEVCAP_DXFMT_R8G8_SNORM: c_int = 231;
pub const SVGA3D_DEVCAP_DXFMT_R16_FLOAT: c_int = 232;
pub const SVGA3D_DEVCAP_DXFMT_D16_UNORM: c_int = 233;
pub const SVGA3D_DEVCAP_DXFMT_A8_UNORM: c_int = 234;
pub const SVGA3D_DEVCAP_DXFMT_BC1_UNORM: c_int = 235;
pub const SVGA3D_DEVCAP_DXFMT_BC2_UNORM: c_int = 236;
pub const SVGA3D_DEVCAP_DXFMT_BC3_UNORM: c_int = 237;
pub const SVGA3D_DEVCAP_DXFMT_B5G6R5_UNORM: c_int = 238;
pub const SVGA3D_DEVCAP_DXFMT_B5G5R5A1_UNORM: c_int = 239;
pub const SVGA3D_DEVCAP_DXFMT_B8G8R8A8_UNORM: c_int = 240;
pub const SVGA3D_DEVCAP_DXFMT_B8G8R8X8_UNORM: c_int = 241;
pub const SVGA3D_DEVCAP_DXFMT_BC4_UNORM: c_int = 242;
pub const SVGA3D_DEVCAP_DXFMT_BC5_UNORM: c_int = 243;
pub const SVGA3D_DEVCAP_SM41: c_int = 244;
pub const SVGA3D_DEVCAP_MULTISAMPLE_2X: c_int = 245;
pub const SVGA3D_DEVCAP_MULTISAMPLE_4X: c_int = 246;
pub const SVGA3D_DEVCAP_MS_FULL_QUALITY: c_int = 247;
pub const SVGA3D_DEVCAP_LOGICOPS: c_int = 248;
pub const SVGA3D_DEVCAP_LOGIC_BLENDOPS: c_int = 249;
pub const SVGA3D_DEVCAP_DEAD12: c_int = 250;
pub const SVGA3D_DEVCAP_DXFMT_BC6H_TYPELESS: c_int = 251;
pub const SVGA3D_DEVCAP_DXFMT_BC6H_UF16: c_int = 252;
pub const SVGA3D_DEVCAP_DXFMT_BC6H_SF16: c_int = 253;
pub const SVGA3D_DEVCAP_DXFMT_BC7_TYPELESS: c_int = 254;
pub const SVGA3D_DEVCAP_DXFMT_BC7_UNORM: c_int = 255;
pub const SVGA3D_DEVCAP_DXFMT_BC7_UNORM_SRGB: c_int = 256;
pub const SVGA3D_DEVCAP_DEAD13: c_int = 257;
pub const SVGA3D_DEVCAP_SM5: c_int = 258;
pub const SVGA3D_DEVCAP_MULTISAMPLE_8X: c_int = 259;
pub const SVGA3D_DEVCAP_MAX_FORCED_SAMPLE_COUNT: c_int = 260;
pub const SVGA3D_DEVCAP_GL43: c_int = 261;
pub const SVGA3D_DEVCAP_MAX: c_int = 262;

