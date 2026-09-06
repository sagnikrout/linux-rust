//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/radeon_drm.h
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


// radeon_drm.h -- Public header for the radeon driver -*- linux-c -*-
//
// Copyright 2000 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Fremont, California.
// Copyright 2002 Tungsten Graphics, Inc., Cedar Park, Texas.
// All rights reserved.
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
// PRECISION INSIGHT AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Authors:
// Kevin E. Martin <martin@valinux.com>
// Gareth Hughes <gareth@valinux.com>
// Keith Whitwell <keith@tungstengraphics.com>
//

// WARNING: If you change any of these defines, make sure to change the
// defines in the X server file (radeon_sarea.h)
//
// Old style state flags, required for sarea interface (1.1 and 1.2
// clears) and 1.2 drm_vertex2 ioctl.
//
pub const RADEON_UPLOAD_CONTEXT: c_uint = 0x00000001;
pub const RADEON_UPLOAD_VERTFMT: c_uint = 0x00000002;
pub const RADEON_UPLOAD_LINE: c_uint = 0x00000004;
pub const RADEON_UPLOAD_BUMPMAP: c_uint = 0x00000008;
pub const RADEON_UPLOAD_MASKS: c_uint = 0x00000010;
pub const RADEON_UPLOAD_VIEWPORT: c_uint = 0x00000020;
pub const RADEON_UPLOAD_SETUP: c_uint = 0x00000040;
pub const RADEON_UPLOAD_TCL: c_uint = 0x00000080;
pub const RADEON_UPLOAD_MISC: c_uint = 0x00000100;
pub const RADEON_UPLOAD_TEX0: c_uint = 0x00000200;
pub const RADEON_UPLOAD_TEX1: c_uint = 0x00000400;
pub const RADEON_UPLOAD_TEX2: c_uint = 0x00000800;
pub const RADEON_UPLOAD_TEX0IMAGES: c_uint = 0x00001000;
pub const RADEON_UPLOAD_TEX1IMAGES: c_uint = 0x00002000;
pub const RADEON_UPLOAD_TEX2IMAGES: c_uint = 0x00004000;
pub const RADEON_UPLOAD_CLIPRECTS: c_uint = 0x00008000	/* handled client-side */;
pub const RADEON_REQUIRE_QUIESCENCE: c_uint = 0x00010000;
pub const RADEON_UPLOAD_ZBIAS: c_uint = 0x00020000	/* version 1.2 and newer */;
pub const RADEON_UPLOAD_ALL: c_uint = 0x003effff;
pub const RADEON_UPLOAD_CONTEXT_ALL: c_uint = 0x003e01ff;
// New style per-packet identifiers for use in cmd_buffer ioctl with
// the RADEON_EMIT_PACKET command.  Comments relate new packets to old
// state bits and the packet size:
//

pub const R200_EMIT_PP_CUBIC_FACES_0: c_int = 61;
pub const R200_EMIT_PP_CUBIC_OFFSETS_0: c_int = 62;
pub const R200_EMIT_PP_CUBIC_FACES_1: c_int = 63;
pub const R200_EMIT_PP_CUBIC_OFFSETS_1: c_int = 64;
pub const R200_EMIT_PP_CUBIC_FACES_2: c_int = 65;
pub const R200_EMIT_PP_CUBIC_OFFSETS_2: c_int = 66;
pub const R200_EMIT_PP_CUBIC_FACES_3: c_int = 67;
pub const R200_EMIT_PP_CUBIC_OFFSETS_3: c_int = 68;
pub const R200_EMIT_PP_CUBIC_FACES_4: c_int = 69;
pub const R200_EMIT_PP_CUBIC_OFFSETS_4: c_int = 70;
pub const R200_EMIT_PP_CUBIC_FACES_5: c_int = 71;
pub const R200_EMIT_PP_CUBIC_OFFSETS_5: c_int = 72;
pub const RADEON_EMIT_PP_TEX_SIZE_0: c_int = 73;
pub const RADEON_EMIT_PP_TEX_SIZE_1: c_int = 74;
pub const RADEON_EMIT_PP_TEX_SIZE_2: c_int = 75;
pub const R200_EMIT_RB3D_BLENDCOLOR: c_int = 76;
pub const R200_EMIT_TCL_POINT_SPRITE_CNTL: c_int = 77;
pub const RADEON_EMIT_PP_CUBIC_FACES_0: c_int = 78;
pub const RADEON_EMIT_PP_CUBIC_OFFSETS_T0: c_int = 79;
pub const RADEON_EMIT_PP_CUBIC_FACES_1: c_int = 80;
pub const RADEON_EMIT_PP_CUBIC_OFFSETS_T1: c_int = 81;
pub const RADEON_EMIT_PP_CUBIC_FACES_2: c_int = 82;
pub const RADEON_EMIT_PP_CUBIC_OFFSETS_T2: c_int = 83;
pub const R200_EMIT_PP_TRI_PERF_CNTL: c_int = 84;
pub const R200_EMIT_PP_AFS_0: c_int = 85;
pub const R200_EMIT_PP_AFS_1: c_int = 86;
pub const R200_EMIT_ATF_TFACTOR: c_int = 87;
pub const R200_EMIT_PP_TXCTLALL_0: c_int = 88;
pub const R200_EMIT_PP_TXCTLALL_1: c_int = 89;
pub const R200_EMIT_PP_TXCTLALL_2: c_int = 90;
pub const R200_EMIT_PP_TXCTLALL_3: c_int = 91;
pub const R200_EMIT_PP_TXCTLALL_4: c_int = 92;
pub const R200_EMIT_PP_TXCTLALL_5: c_int = 93;
pub const R200_EMIT_VAP_PVS_CNTL: c_int = 94;
pub const RADEON_MAX_STATE_PACKETS: c_int = 95;
// Commands understood by cmd_buffer ioctl.  More can be added but
// obviously these can't be removed or changed:
//

// doesn't make the cpu wait, just
// the graphics hardware

pub const RADEON_WAIT_2D: c_uint = 0x1;
pub const RADEON_WAIT_3D: c_uint = 0x2;
// Allowed parameters for R300_CMD_PACKET3
//
pub const R300_CMD_PACKET3_CLEAR: c_int = 0;
pub const R300_CMD_PACKET3_RAW: c_int = 1;
// Commands understood by cmd_buffer ioctl for R300.
// The interface has not been stabilized, so some of these may be removed
// and eventually reordered before stabilization.
//
pub const R300_CMD_PACKET0: c_int = 1;

pub const R300_CMD_CP_DELAY: c_int = 5;
pub const R300_CMD_DMA_DISCARD: c_int = 6;
pub const R300_CMD_WAIT: c_int = 7;

// these two defines are DOING IT WRONG - however
// we have userspace which relies on using these.
// The wait interface is backwards compat new
// code should use the NEW_WAIT defines below
// THESE ARE NOT BIT FIELDS
//

pub const R300_CMD_SCRATCH: c_int = 8;
pub const R300_CMD_R500FP: c_int = 9;
pub const RADEON_FRONT: c_uint = 0x1;
pub const RADEON_BACK: c_uint = 0x2;
pub const RADEON_DEPTH: c_uint = 0x4;
pub const RADEON_STENCIL: c_uint = 0x8;
pub const RADEON_CLEAR_FASTZ: c_uint = 0x80000000;
pub const RADEON_USE_HIERZ: c_uint = 0x40000000;
pub const RADEON_USE_COMP_ZBUF: c_uint = 0x20000000;

// Primitive types
//
pub const RADEON_POINTS: c_uint = 0x1;
pub const RADEON_LINES: c_uint = 0x2;
pub const RADEON_LINE_STRIP: c_uint = 0x3;
pub const RADEON_TRIANGLES: c_uint = 0x4;
pub const RADEON_TRIANGLE_FAN: c_uint = 0x5;
pub const RADEON_TRIANGLE_STRIP: c_uint = 0x6;
// Vertex/indirect buffer size
//
pub const RADEON_BUFFER_SIZE: c_int = 65536;
// Byte offsets for indirect buffer data
//
pub const RADEON_INDEX_PRIM_OFFSET: c_int = 20;
pub const RADEON_SCRATCH_REG_OFFSET: c_int = 32;
pub const R600_SCRATCH_REG_OFFSET: c_int = 256;
pub const RADEON_NR_SAREA_CLIPRECTS: c_int = 12;
// There are 2 heaps (local/GART).  Each region within a heap is a
// minimum of 64k, and there are at most 64 of them per heap.
//
pub const RADEON_LOCAL_TEX_HEAP: c_int = 0;
pub const RADEON_GART_TEX_HEAP: c_int = 1;
pub const RADEON_NR_TEX_HEAPS: c_int = 2;
pub const RADEON_NR_TEX_REGIONS: c_int = 64;
pub const RADEON_LOG_TEX_GRANULARITY: c_int = 16;
pub const RADEON_MAX_TEXTURE_LEVELS: c_int = 12;
pub const RADEON_MAX_TEXTURE_UNITS: c_int = 3;
pub const RADEON_MAX_SURFACES: c_int = 8;
// Blits have strict offset rules.  All blit offset must be aligned on
// a 1K-byte boundary.
//
pub const RADEON_OFFSET_SHIFT: c_int = 10;

// Context state
// Vertex format state
// Line state
// Bumpmap state
// Mask state
// Viewport state
// Setup state
// Misc state
// Zbias state
// Setup registers for each texture unit
//
// The channel for communication of state information to the
// kernel on firing a vertex buffer with either of the
// obsoleted vertex/index ioctls.
//
// The current cliprects, or a subset thereof.
//
// Counters for client-side throttling of rendering clients.
//
// WARNING: If you change any of these defines, make sure to change the
// defines in the Xserver file (xf86drmRadeon.h)
//
// KW: actually it's illegal to change any of this (backwards compatibility).
//
// Radeon specific ioctls
// The device specific ioctl range is 0x40 to 0x79.
//
pub const DRM_RADEON_CP_INIT: c_uint = 0x00;
pub const DRM_RADEON_CP_START: c_uint = 0x01;
pub const DRM_RADEON_CP_STOP: c_uint = 0x02;
pub const DRM_RADEON_CP_RESET: c_uint = 0x03;
pub const DRM_RADEON_CP_IDLE: c_uint = 0x04;
pub const DRM_RADEON_RESET: c_uint = 0x05;
pub const DRM_RADEON_FULLSCREEN: c_uint = 0x06;
pub const DRM_RADEON_SWAP: c_uint = 0x07;
pub const DRM_RADEON_CLEAR: c_uint = 0x08;
pub const DRM_RADEON_VERTEX: c_uint = 0x09;
pub const DRM_RADEON_INDICES: c_uint = 0x0A;
// Macro flag: #define DRM_RADEON_NOT_USED
pub const DRM_RADEON_STIPPLE: c_uint = 0x0C;
pub const DRM_RADEON_INDIRECT: c_uint = 0x0D;
pub const DRM_RADEON_TEXTURE: c_uint = 0x0E;
pub const DRM_RADEON_VERTEX2: c_uint = 0x0F;
pub const DRM_RADEON_CMDBUF: c_uint = 0x10;
pub const DRM_RADEON_GETPARAM: c_uint = 0x11;
pub const DRM_RADEON_FLIP: c_uint = 0x12;
pub const DRM_RADEON_ALLOC: c_uint = 0x13;
pub const DRM_RADEON_FREE: c_uint = 0x14;
pub const DRM_RADEON_INIT_HEAP: c_uint = 0x15;
pub const DRM_RADEON_IRQ_EMIT: c_uint = 0x16;
pub const DRM_RADEON_IRQ_WAIT: c_uint = 0x17;
pub const DRM_RADEON_CP_RESUME: c_uint = 0x18;
pub const DRM_RADEON_SETPARAM: c_uint = 0x19;
pub const DRM_RADEON_SURF_ALLOC: c_uint = 0x1a;
pub const DRM_RADEON_SURF_FREE: c_uint = 0x1b;
// KMS ioctl
pub const DRM_RADEON_GEM_INFO: c_uint = 0x1c;
pub const DRM_RADEON_GEM_CREATE: c_uint = 0x1d;
pub const DRM_RADEON_GEM_MMAP: c_uint = 0x1e;
pub const DRM_RADEON_GEM_PREAD: c_uint = 0x21;
pub const DRM_RADEON_GEM_PWRITE: c_uint = 0x22;
pub const DRM_RADEON_GEM_SET_DOMAIN: c_uint = 0x23;
pub const DRM_RADEON_GEM_WAIT_IDLE: c_uint = 0x24;
pub const DRM_RADEON_CS: c_uint = 0x26;
pub const DRM_RADEON_INFO: c_uint = 0x27;
pub const DRM_RADEON_GEM_SET_TILING: c_uint = 0x28;
pub const DRM_RADEON_GEM_GET_TILING: c_uint = 0x29;
pub const DRM_RADEON_GEM_BUSY: c_uint = 0x2a;
pub const DRM_RADEON_GEM_VA: c_uint = 0x2b;
pub const DRM_RADEON_GEM_OP: c_uint = 0x2c;
pub const DRM_RADEON_GEM_USERPTR: c_uint = 0x2d;

// KMS

pub const CLEAR_X1: c_int = 0;
pub const CLEAR_Y1: c_int = 1;
pub const CLEAR_X2: c_int = 2;
pub const CLEAR_Y2: c_int = 3;
pub const CLEAR_DEPTH: c_int = 4;
// v1.2 - obsoletes drm_radeon_vertex and drm_radeon_indices
// - allows multiple primitives and state changes in a single ioctl
// - supports driver change to emit native primitives
//
// v1.3 - obsoletes drm_radeon_vertex2
// - allows arbitrarily large cliprect list
// - allows updating of tcl packet, vector and scalar state
// - allows memory-efficient description of state updates
// - allows state to be emitted without a primitive
// (for clears, ctx switches)
// - allows more than one dma buffer to be referenced per ioctl
// - supports tcl driver
// - may be extended in future versions with new cmd types, packets
//
// enum for card type parameters
pub const RADEON_CARD_PCI: c_int = 0;
pub const RADEON_CARD_AGP: c_int = 1;
pub const RADEON_CARD_PCIE: c_int = 2;
// 1.3: An ioctl to get parameters that aren't available to the 3d
// client any other way.
//

pub const RADEON_PARAM_LAST_FRAME: c_int = 2;
pub const RADEON_PARAM_LAST_DISPATCH: c_int = 3;
pub const RADEON_PARAM_LAST_CLEAR: c_int = 4;
// Added with DRM version 1.6.
pub const RADEON_PARAM_IRQ_NR: c_int = 5;

// Added with DRM version 1.8.

pub const RADEON_PARAM_STATUS_HANDLE: c_int = 8;
pub const RADEON_PARAM_SAREA_HANDLE: c_int = 9;
pub const RADEON_PARAM_GART_TEX_HANDLE: c_int = 10;
pub const RADEON_PARAM_SCRATCH_OFFSET: c_int = 11;
pub const RADEON_PARAM_CARD_TYPE: c_int = 12;

pub const RADEON_PARAM_DEVICE_ID: c_int = 16;

// 1.6: Set up a memory manager for regions of shared memory:
//
pub const RADEON_MEM_REGION_GART: c_int = 1;
pub const RADEON_MEM_REGION_FB: c_int = 2;
// 1.6: Userspace can request & wait on irq's:
//
// 1.10: Clients tell the DRM where they think the framebuffer is located in
// the card's address space, via a new generic ioctl to set parameters
//

// 1.14: Clients can allocate/free a surface
//
pub const DRM_RADEON_VBLANK_CRTC1: c_int = 1;
pub const DRM_RADEON_VBLANK_CRTC2: c_int = 2;
//
// Kernel modesetting world below.
//
pub const RADEON_GEM_DOMAIN_CPU: c_uint = 0x1;
pub const RADEON_GEM_DOMAIN_GTT: c_uint = 0x2;
pub const RADEON_GEM_DOMAIN_VRAM: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_info {
    pub gart_size: __u64,
    pub vram_size: __u64,
    pub vram_visible: __u64,
}

// BO is expected to be accessed by the CPU

// CPU access is not expected to work for this BO

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_create {
    pub size: __u64,
    pub alignment: __u64,
    pub handle: __u32,
    pub initial_domain: __u32,
    pub flags: __u32,
}

//
// This is not a reliable API and you should expect it to fail for any
// number of reasons and have fallback path that do not use userptr to
// perform any operation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_userptr {
    pub addr: __u64,
    pub size: __u64,
    pub flags: __u32,
    pub handle: __u32,
}

pub const RADEON_TILING_MACRO: c_uint = 0x1;
pub const RADEON_TILING_MICRO: c_uint = 0x2;
pub const RADEON_TILING_SWAP_16BIT: c_uint = 0x4;
pub const RADEON_TILING_SWAP_32BIT: c_uint = 0x8;
// this object requires a surface when mapped - i.e. front buffer
pub const RADEON_TILING_SURFACE: c_uint = 0x10;
pub const RADEON_TILING_MICRO_SQUARE: c_uint = 0x20;
pub const RADEON_TILING_EG_BANKW_SHIFT: c_int = 8;
pub const RADEON_TILING_EG_BANKW_MASK: c_uint = 0xf;
pub const RADEON_TILING_EG_BANKH_SHIFT: c_int = 12;
pub const RADEON_TILING_EG_BANKH_MASK: c_uint = 0xf;
pub const RADEON_TILING_EG_MACRO_TILE_ASPECT_SHIFT: c_int = 16;
pub const RADEON_TILING_EG_MACRO_TILE_ASPECT_MASK: c_uint = 0xf;
pub const RADEON_TILING_EG_TILE_SPLIT_SHIFT: c_int = 24;
pub const RADEON_TILING_EG_TILE_SPLIT_MASK: c_uint = 0xf;
pub const RADEON_TILING_EG_STENCIL_TILE_SPLIT_SHIFT: c_int = 28;
pub const RADEON_TILING_EG_STENCIL_TILE_SPLIT_MASK: c_uint = 0xf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_set_tiling {
    pub handle: __u32,
    pub tiling_flags: __u32,
    pub pitch: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_get_tiling {
    pub handle: __u32,
    pub tiling_flags: __u32,
    pub pitch: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_mmap {
    pub handle: __u32,
    pub pad: __u32,
    pub offset: __u64,
    pub size: __u64,
    pub addr_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_set_domain {
    pub handle: __u32,
    pub read_domains: __u32,
    pub write_domain: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_wait_idle {
    pub handle: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_busy {
    pub handle: __u32,
    pub domain: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_pread {
// Handle for the object being read.
    pub handle: __u32,
    pub pad: __u32,
// Offset into the object to read from
    pub offset: __u64,
// Length of data to read
    pub size: __u64,
// Pointer to write the data into.
// void *, but pointers are not 32/64 compatible
    pub data_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_pwrite {
// Handle for the object being written to.
    pub handle: __u32,
    pub pad: __u32,
// Offset into the object to write to
    pub offset: __u64,
// Length of data to write
    pub size: __u64,
// Pointer to read the data from.
// void *, but pointers are not 32/64 compatible
    pub data_ptr: __u64,
}

// Sets or returns a value associated with a buffer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_op {
    pub /: *mut *mut __u32 handle; / buffer,
    pub /: *mut *mut *mut __u32 op; / RADEON_GEM_OP_,
    pub /: *mut *mut __u64 value; / input or return value,
}

pub const RADEON_GEM_OP_GET_INITIAL_DOMAIN: c_int = 0;
pub const RADEON_GEM_OP_SET_INITIAL_DOMAIN: c_int = 1;
pub const RADEON_VA_MAP: c_int = 1;
pub const RADEON_VA_UNMAP: c_int = 2;
pub const RADEON_VA_RESULT_OK: c_int = 0;
pub const RADEON_VA_RESULT_ERROR: c_int = 1;
pub const RADEON_VA_RESULT_VA_EXIST: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_gem_va {
    pub handle: __u32,
    pub operation: __u32,
    pub vm_id: __u32,
    pub flags: __u32,
    pub offset: __u64,
}

pub const RADEON_CHUNK_ID_RELOCS: c_uint = 0x01;
pub const RADEON_CHUNK_ID_IB: c_uint = 0x02;
pub const RADEON_CHUNK_ID_FLAGS: c_uint = 0x03;
pub const RADEON_CHUNK_ID_CONST_IB: c_uint = 0x04;
// The first dword of RADEON_CHUNK_ID_FLAGS is a uint32 of these flags:
pub const RADEON_CS_KEEP_TILING_FLAGS: c_uint = 0x01;
pub const RADEON_CS_USE_VM: c_uint = 0x02;
pub const RADEON_CS_END_OF_FRAME: c_uint = 0x04 /* a hint from userspace which CS is the last one */;
// The second dword of RADEON_CHUNK_ID_FLAGS is a uint32 that sets the ring type
pub const RADEON_CS_RING_GFX: c_int = 0;
pub const RADEON_CS_RING_COMPUTE: c_int = 1;
pub const RADEON_CS_RING_DMA: c_int = 2;
pub const RADEON_CS_RING_UVD: c_int = 3;
pub const RADEON_CS_RING_VCE: c_int = 4;
// The third dword of RADEON_CHUNK_ID_FLAGS is a sint32 that sets the priority
// 0 = normal, + = higher priority, - = lower priority
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_cs_chunk {
    pub chunk_id: __u32,
    pub length_dw: __u32,
    pub chunk_data: __u64,
}

// drm_radeon_cs_reloc.flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_cs_reloc {
    pub handle: __u32,
    pub read_domains: __u32,
    pub write_domain: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_cs {
    pub num_chunks: __u32,
    pub cs_id: __u32,
// this points to __u64 * which point to cs chunks
    pub chunks: __u64,
// updates to the limits after this CS ioctl
    pub gart_limit: __u64,
    pub vram_limit: __u64,
}

pub const RADEON_INFO_DEVICE_ID: c_uint = 0x00;
pub const RADEON_INFO_NUM_GB_PIPES: c_uint = 0x01;
pub const RADEON_INFO_NUM_Z_PIPES: c_uint = 0x02;
pub const RADEON_INFO_ACCEL_WORKING: c_uint = 0x03;
pub const RADEON_INFO_CRTC_FROM_ID: c_uint = 0x04;
pub const RADEON_INFO_ACCEL_WORKING2: c_uint = 0x05;
pub const RADEON_INFO_TILING_CONFIG: c_uint = 0x06;
pub const RADEON_INFO_WANT_HYPERZ: c_uint = 0x07;
pub const RADEON_INFO_WANT_CMASK: c_uint = 0x08 /* get access to CMASK on r300 */;
pub const RADEON_INFO_CLOCK_CRYSTAL_FREQ: c_uint = 0x09 /* clock crystal frequency */;
pub const RADEON_INFO_NUM_BACKENDS: c_uint = 0x0a /* DB/backends for r600+ - need for OQ */;
pub const RADEON_INFO_NUM_TILE_PIPES: c_uint = 0x0b /* tile pipes for r600+ */;
pub const RADEON_INFO_FUSION_GART_WORKING: c_uint = 0x0c /* fusion writes to GTT were broken before this */;
pub const RADEON_INFO_BACKEND_MAP: c_uint = 0x0d /* pipe to backend map, needed by mesa */;
// virtual address start, va < start are reserved by the kernel
pub const RADEON_INFO_VA_START: c_uint = 0x0e;
// maximum size of ib using the virtual memory cs
pub const RADEON_INFO_IB_VM_MAX_SIZE: c_uint = 0x0f;
// max pipes - needed for compute shaders
pub const RADEON_INFO_MAX_PIPES: c_uint = 0x10;
// timestamp for GL_ARB_timer_query (OpenGL), returns the current GPU clock
pub const RADEON_INFO_TIMESTAMP: c_uint = 0x11;
// max shader engines (SE) - needed for geometry shaders, etc.
pub const RADEON_INFO_MAX_SE: c_uint = 0x12;
// max SH per SE
pub const RADEON_INFO_MAX_SH_PER_SE: c_uint = 0x13;
// fast fb access is enabled
pub const RADEON_INFO_FASTFB_WORKING: c_uint = 0x14;
// query if a RADEON_CS_RING_* submission is supported
pub const RADEON_INFO_RING_WORKING: c_uint = 0x15;
// SI tile mode array
pub const RADEON_INFO_SI_TILE_MODE_ARRAY: c_uint = 0x16;
// query if CP DMA is supported on the compute ring
pub const RADEON_INFO_SI_CP_DMA_COMPUTE: c_uint = 0x17;
// CIK macrotile mode array
pub const RADEON_INFO_CIK_MACROTILE_MODE_ARRAY: c_uint = 0x18;
// query the number of render backends
pub const RADEON_INFO_SI_BACKEND_ENABLED_MASK: c_uint = 0x19;
// max engine clock - needed for OpenCL
pub const RADEON_INFO_MAX_SCLK: c_uint = 0x1a;
// version of VCE firmware
pub const RADEON_INFO_VCE_FW_VERSION: c_uint = 0x1b;
// version of VCE feedback
pub const RADEON_INFO_VCE_FB_VERSION: c_uint = 0x1c;
pub const RADEON_INFO_NUM_BYTES_MOVED: c_uint = 0x1d;
pub const RADEON_INFO_VRAM_USAGE: c_uint = 0x1e;
pub const RADEON_INFO_GTT_USAGE: c_uint = 0x1f;
pub const RADEON_INFO_ACTIVE_CU_COUNT: c_uint = 0x20;
pub const RADEON_INFO_CURRENT_GPU_TEMP: c_uint = 0x21;
pub const RADEON_INFO_CURRENT_GPU_SCLK: c_uint = 0x22;
pub const RADEON_INFO_CURRENT_GPU_MCLK: c_uint = 0x23;
pub const RADEON_INFO_READ_REG: c_uint = 0x24;
pub const RADEON_INFO_VA_UNMAP_WORKING: c_uint = 0x25;
pub const RADEON_INFO_GPU_RESET_COUNTER: c_uint = 0x26;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_radeon_info {
    pub request: __u32,
    pub pad: __u32,
    pub value: __u64,
}

// Those correspond to the tile index to use, this is to explicitly state
// the API that is implicitly defined by the tile mode array.
//
pub const SI_TILE_MODE_COLOR_LINEAR_ALIGNED: c_int = 8;
pub const SI_TILE_MODE_COLOR_1D: c_int = 13;
pub const SI_TILE_MODE_COLOR_1D_SCANOUT: c_int = 9;
pub const SI_TILE_MODE_COLOR_2D_8BPP: c_int = 14;
pub const SI_TILE_MODE_COLOR_2D_16BPP: c_int = 15;
pub const SI_TILE_MODE_COLOR_2D_32BPP: c_int = 16;
pub const SI_TILE_MODE_COLOR_2D_64BPP: c_int = 17;
pub const SI_TILE_MODE_COLOR_2D_SCANOUT_16BPP: c_int = 11;
pub const SI_TILE_MODE_COLOR_2D_SCANOUT_32BPP: c_int = 12;
pub const SI_TILE_MODE_DEPTH_STENCIL_1D: c_int = 4;
pub const SI_TILE_MODE_DEPTH_STENCIL_2D: c_int = 0;
pub const SI_TILE_MODE_DEPTH_STENCIL_2D_2AA: c_int = 3;
pub const SI_TILE_MODE_DEPTH_STENCIL_2D_4AA: c_int = 3;
pub const SI_TILE_MODE_DEPTH_STENCIL_2D_8AA: c_int = 2;
pub const CIK_TILE_MODE_DEPTH_STENCIL_1D: c_int = 5;

