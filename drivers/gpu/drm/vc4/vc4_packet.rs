//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vc4/vc4_packet.h
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
// Copyright © 2014 Broadcom
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_packet {
    VC4_PACKET_HALT = 0,
    VC4_PACKET_NOP = 1,

    VC4_PACKET_FLUSH = 4,
    VC4_PACKET_FLUSH_ALL = 5,
    VC4_PACKET_START_TILE_BINNING = 6,
    VC4_PACKET_INCREMENT_SEMAPHORE = 7,
    VC4_PACKET_WAIT_ON_SEMAPHORE = 8,

    VC4_PACKET_BRANCH = 16,
    VC4_PACKET_BRANCH_TO_SUB_LIST = 17,

    VC4_PACKET_STORE_MS_TILE_BUFFER = 24,
    VC4_PACKET_STORE_MS_TILE_BUFFER_AND_EOF = 25,
    VC4_PACKET_STORE_FULL_RES_TILE_BUFFER = 26,
    VC4_PACKET_LOAD_FULL_RES_TILE_BUFFER = 27,
    VC4_PACKET_STORE_TILE_BUFFER_GENERAL = 28,
    VC4_PACKET_LOAD_TILE_BUFFER_GENERAL = 29,

    VC4_PACKET_GL_INDEXED_PRIMITIVE = 32,
    VC4_PACKET_GL_ARRAY_PRIMITIVE = 33,

    VC4_PACKET_COMPRESSED_PRIMITIVE = 48,
    VC4_PACKET_CLIPPED_COMPRESSED_PRIMITIVE = 49,

    VC4_PACKET_PRIMITIVE_LIST_FORMAT = 56,

    VC4_PACKET_GL_SHADER_STATE = 64,
    VC4_PACKET_NV_SHADER_STATE = 65,
    VC4_PACKET_VG_SHADER_STATE = 66,

    VC4_PACKET_CONFIGURATION_BITS = 96,
    VC4_PACKET_FLAT_SHADE_FLAGS = 97,
    VC4_PACKET_POINT_SIZE = 98,
    VC4_PACKET_LINE_WIDTH = 99,
    VC4_PACKET_RHT_X_BOUNDARY = 100,
    VC4_PACKET_DEPTH_OFFSET = 101,
    VC4_PACKET_CLIP_WINDOW = 102,
    VC4_PACKET_VIEWPORT_OFFSET = 103,
    VC4_PACKET_Z_CLIPPING = 104,
    VC4_PACKET_CLIPPER_XY_SCALING = 105,
    VC4_PACKET_CLIPPER_Z_SCALING = 106,

    VC4_PACKET_TILE_BINNING_MODE_CONFIG = 112,
    VC4_PACKET_TILE_RENDERING_MODE_CONFIG = 113,
    VC4_PACKET_CLEAR_COLORS = 114,
    VC4_PACKET_TILE_COORDINATES = 115,

// Not an actual hardware packet -- this is what we use to put
// references to GEM bos in the command stream, since we need the u32
// int the actual address packet in order to store the offset from the
// start of the BO.
//
    VC4_PACKET_GEM_HANDLES = 254,
    } __attribute__ ((__packed__));

pub const VC4_PACKET_HALT_SIZE: c_int = 1;
pub const VC4_PACKET_NOP_SIZE: c_int = 1;
pub const VC4_PACKET_FLUSH_SIZE: c_int = 1;
pub const VC4_PACKET_FLUSH_ALL_SIZE: c_int = 1;
pub const VC4_PACKET_START_TILE_BINNING_SIZE: c_int = 1;
pub const VC4_PACKET_INCREMENT_SEMAPHORE_SIZE: c_int = 1;
pub const VC4_PACKET_WAIT_ON_SEMAPHORE_SIZE: c_int = 1;
pub const VC4_PACKET_BRANCH_SIZE: c_int = 5;
pub const VC4_PACKET_BRANCH_TO_SUB_LIST_SIZE: c_int = 5;
pub const VC4_PACKET_STORE_MS_TILE_BUFFER_SIZE: c_int = 1;
pub const VC4_PACKET_STORE_MS_TILE_BUFFER_AND_EOF_SIZE: c_int = 1;
pub const VC4_PACKET_STORE_FULL_RES_TILE_BUFFER_SIZE: c_int = 5;
pub const VC4_PACKET_LOAD_FULL_RES_TILE_BUFFER_SIZE: c_int = 5;
pub const VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE: c_int = 7;
pub const VC4_PACKET_LOAD_TILE_BUFFER_GENERAL_SIZE: c_int = 7;
pub const VC4_PACKET_GL_INDEXED_PRIMITIVE_SIZE: c_int = 14;
pub const VC4_PACKET_GL_ARRAY_PRIMITIVE_SIZE: c_int = 10;
pub const VC4_PACKET_COMPRESSED_PRIMITIVE_SIZE: c_int = 1;
pub const VC4_PACKET_CLIPPED_COMPRESSED_PRIMITIVE_SIZE: c_int = 1;
pub const VC4_PACKET_PRIMITIVE_LIST_FORMAT_SIZE: c_int = 2;
pub const VC4_PACKET_GL_SHADER_STATE_SIZE: c_int = 5;
pub const VC4_PACKET_NV_SHADER_STATE_SIZE: c_int = 5;
pub const VC4_PACKET_VG_SHADER_STATE_SIZE: c_int = 5;
pub const VC4_PACKET_CONFIGURATION_BITS_SIZE: c_int = 4;
pub const VC4_PACKET_FLAT_SHADE_FLAGS_SIZE: c_int = 5;
pub const VC4_PACKET_POINT_SIZE_SIZE: c_int = 5;
pub const VC4_PACKET_LINE_WIDTH_SIZE: c_int = 5;
pub const VC4_PACKET_RHT_X_BOUNDARY_SIZE: c_int = 3;
pub const VC4_PACKET_DEPTH_OFFSET_SIZE: c_int = 5;
pub const VC4_PACKET_CLIP_WINDOW_SIZE: c_int = 9;
pub const VC4_PACKET_VIEWPORT_OFFSET_SIZE: c_int = 5;
pub const VC4_PACKET_Z_CLIPPING_SIZE: c_int = 9;
pub const VC4_PACKET_CLIPPER_XY_SCALING_SIZE: c_int = 9;
pub const VC4_PACKET_CLIPPER_Z_SCALING_SIZE: c_int = 9;
pub const VC4_PACKET_TILE_BINNING_MODE_CONFIG_SIZE: c_int = 16;
pub const VC4_PACKET_TILE_RENDERING_MODE_CONFIG_SIZE: c_int = 11;
pub const VC4_PACKET_CLEAR_COLORS_SIZE: c_int = 14;
pub const VC4_PACKET_TILE_COORDINATES_SIZE: c_int = 3;
pub const VC4_PACKET_GEM_HANDLES_SIZE: c_int = 9;

// Number of multisamples supported.
pub const VC4_MAX_SAMPLES: c_int = 4;
// Size of a full resolution color or Z tile buffer load/store.

// @{
// Bits used by packets like VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
// VC4_PACKET_TILE_RENDERING_MODE_CONFIG.
//
pub const VC4_TILING_FORMAT_LINEAR: c_int = 0;
pub const VC4_TILING_FORMAT_T: c_int = 1;
pub const VC4_TILING_FORMAT_LT: c_int = 2;
// @}

// @{
//
// low bits of VC4_PACKET_STORE_FULL_RES_TILE_BUFFER and
// VC4_PACKET_LOAD_FULL_RES_TILE_BUFFER.
//

// @{
//
// low bits of VC4_PACKET_STORE_FULL_RES_TILE_BUFFER and
// VC4_PACKET_LOAD_FULL_RES_TILE_BUFFER.
//

// @{
//
// byte 2 of VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
// VC4_PACKET_LOAD_TILE_BUFFER_GENERAL (low bits of the address)
//

// @}

// @{
//
// byte 0-1 of VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
// VC4_PACKET_LOAD_TILE_BUFFER_GENERAL
//

pub const VC4_LOADSTORE_TILE_BUFFER_FORMAT_SHIFT: c_int = 8;
pub const VC4_LOADSTORE_TILE_BUFFER_RGBA8888: c_int = 0;
pub const VC4_LOADSTORE_TILE_BUFFER_BGR565_DITHER: c_int = 1;
pub const VC4_LOADSTORE_TILE_BUFFER_BGR565: c_int = 2;
// @}

// @{
//
// byte 0 of VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
// VC4_PACKET_LOAD_TILE_BUFFER_GENERAL
//

pub const VC4_STORE_TILE_BUFFER_MODE_SHIFT: c_int = 6;

// The values of the field are VC4_TILING_FORMAT_*

pub const VC4_LOADSTORE_TILE_BUFFER_TILING_SHIFT: c_int = 4;

pub const VC4_LOADSTORE_TILE_BUFFER_BUFFER_SHIFT: c_int = 0;
pub const VC4_LOADSTORE_TILE_BUFFER_NONE: c_int = 0;
pub const VC4_LOADSTORE_TILE_BUFFER_COLOR: c_int = 1;
pub const VC4_LOADSTORE_TILE_BUFFER_ZS: c_int = 2;
pub const VC4_LOADSTORE_TILE_BUFFER_Z: c_int = 3;
pub const VC4_LOADSTORE_TILE_BUFFER_VG_MASK: c_int = 4;
pub const VC4_LOADSTORE_TILE_BUFFER_FULL: c_int = 5;
// @}

// This flag is only present in NV shader state.

// @{ byte 2 of config bits.

// @}

// @{ byte 1 of config bits.

// same values in this 3-bit field as PIPE_FUNC_*
pub const VC4_CONFIG_BITS_DEPTH_FUNC_SHIFT: c_int = 4;

// @}

// @{ byte 0 of config bits.

// @}

// @{ bits in the last u8 of VC4_PACKET_TILE_BINNING_MODE_CONFIG

pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_SHIFT: c_int = 5;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_32: c_int = 0;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_64: c_int = 1;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_128: c_int = 2;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_256: c_int = 3;

pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_SHIFT: c_int = 3;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_32: c_int = 0;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_64: c_int = 1;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_128: c_int = 2;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_256: c_int = 3;

// @}

// @{ bits in the last u16 of VC4_PACKET_TILE_RENDERING_MODE_CONFIG

// The values of the field are VC4_TILING_FORMAT_*

pub const VC4_RENDER_CONFIG_MEMORY_FORMAT_SHIFT: c_int = 6;

pub const VC4_RENDER_CONFIG_FORMAT_SHIFT: c_int = 2;
pub const VC4_RENDER_CONFIG_FORMAT_BGR565_DITHERED: c_int = 0;
pub const VC4_RENDER_CONFIG_FORMAT_RGBA8888: c_int = 1;
pub const VC4_RENDER_CONFIG_FORMAT_BGR565: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vc4_texture_data_type {
    VC4_TEXTURE_TYPE_RGBA8888 = 0,
    VC4_TEXTURE_TYPE_RGBX8888 = 1,
    VC4_TEXTURE_TYPE_RGBA4444 = 2,
    VC4_TEXTURE_TYPE_RGBA5551 = 3,
    VC4_TEXTURE_TYPE_RGB565 = 4,
    VC4_TEXTURE_TYPE_LUMINANCE = 5,
    VC4_TEXTURE_TYPE_ALPHA = 6,
    VC4_TEXTURE_TYPE_LUMALPHA = 7,
    VC4_TEXTURE_TYPE_ETC1 = 8,
    VC4_TEXTURE_TYPE_S16F = 9,
    VC4_TEXTURE_TYPE_S8 = 10,
    VC4_TEXTURE_TYPE_S16 = 11,
    VC4_TEXTURE_TYPE_BW1 = 12,
    VC4_TEXTURE_TYPE_A4 = 13,
    VC4_TEXTURE_TYPE_A1 = 14,
    VC4_TEXTURE_TYPE_RGBA64 = 15,
    VC4_TEXTURE_TYPE_RGBA32R = 16,
    VC4_TEXTURE_TYPE_YUV422R = 17,
}

pub const VC4_TEX_P0_OFFSET_SHIFT: c_int = 12;

pub const VC4_TEX_P0_CSWIZ_SHIFT: c_int = 10;

pub const VC4_TEX_P0_CMMODE_SHIFT: c_int = 9;

pub const VC4_TEX_P0_FLIPY_SHIFT: c_int = 8;

pub const VC4_TEX_P0_TYPE_SHIFT: c_int = 4;

pub const VC4_TEX_P0_MIPLVLS_SHIFT: c_int = 0;

pub const VC4_TEX_P1_TYPE4_SHIFT: c_int = 31;

pub const VC4_TEX_P1_HEIGHT_SHIFT: c_int = 20;

pub const VC4_TEX_P1_ETCFLIP_SHIFT: c_int = 19;

pub const VC4_TEX_P1_WIDTH_SHIFT: c_int = 8;

pub const VC4_TEX_P1_MAGFILT_SHIFT: c_int = 7;

pub const VC4_TEX_P1_MINFILT_SHIFT: c_int = 4;

pub const VC4_TEX_P1_WRAP_T_SHIFT: c_int = 2;

pub const VC4_TEX_P1_WRAP_S_SHIFT: c_int = 0;

pub const VC4_TEX_P2_PTYPE_SHIFT: c_int = 30;

// VC4_TEX_P2_PTYPE_CUBE_MAP_STRIDE bits

pub const VC4_TEX_P2_CMST_SHIFT: c_int = 12;

pub const VC4_TEX_P2_BSLOD_SHIFT: c_int = 0;
// VC4_TEX_P2_PTYPE_CHILD_IMAGE_DIMENSIONS

pub const VC4_TEX_P2_CHEIGHT_SHIFT: c_int = 12;

pub const VC4_TEX_P2_CWIDTH_SHIFT: c_int = 0;
// VC4_TEX_P2_PTYPE_CHILD_IMAGE_OFFSETS

pub const VC4_TEX_P2_CYOFF_SHIFT: c_int = 12;

pub const VC4_TEX_P2_CXOFF_SHIFT: c_int = 0;
