//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/vc4_drm.h
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
// Copyright © 2014-2015 Broadcom
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

pub const DRM_VC4_SUBMIT_CL: c_uint = 0x00;
pub const DRM_VC4_WAIT_SEQNO: c_uint = 0x01;
pub const DRM_VC4_WAIT_BO: c_uint = 0x02;
pub const DRM_VC4_CREATE_BO: c_uint = 0x03;
pub const DRM_VC4_MMAP_BO: c_uint = 0x04;
pub const DRM_VC4_CREATE_SHADER_BO: c_uint = 0x05;
pub const DRM_VC4_GET_HANG_STATE: c_uint = 0x06;
pub const DRM_VC4_GET_PARAM: c_uint = 0x07;
pub const DRM_VC4_SET_TILING: c_uint = 0x08;
pub const DRM_VC4_GET_TILING: c_uint = 0x09;
pub const DRM_VC4_LABEL_BO: c_uint = 0x0a;
pub const DRM_VC4_GEM_MADVISE: c_uint = 0x0b;
pub const DRM_VC4_PERFMON_CREATE: c_uint = 0x0c;
pub const DRM_VC4_PERFMON_DESTROY: c_uint = 0x0d;
pub const DRM_VC4_PERFMON_GET_VALUES: c_uint = 0x0e;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_submit_rcl_surface {
    pub /: *mut *mut __u32 hindex; / Handle index, or ~0 if not present.,
    pub /: *mut *mut __u32 offset; / Offset to start of buffer.,
//
// Bits for either render config (color_write) or load/store packet.
// Bits should all be 0 for MSAA load/stores.
//
    pub bits: __u16,

    pub flags: __u16,
}

//
// struct drm_vc4_submit_cl - ioctl argument for submitting commands to the 3D
// engine.
//
// Drivers typically use GPU BOs to store batchbuffers / command lists and
// their associated state.  However, because the VC4 lacks an MMU, we have to
// do validation of memory accesses by the GPU commands.  If we were to store
// our commands in BOs, we'd need to do uncached readback from them to do the
// validation process, which is too expensive.  Instead, userspace accumulates
// commands and associated state in plain memory, then the kernel copies the
// data to its own address space, and then validates and stores it in a GPU
// BO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_submit_cl {
// Pointer to the binner command list.
//
// This is the first set of commands executed, which runs the
// coordinate shader to determine where primitives land on the screen,
// then writes out the state updates and draw calls necessary per tile
// to the tile allocation BO.
//
    pub bin_cl: __u64,
// Pointer to the shader records.
//
// Shader records are the structures read by the hardware that contain
// pointers to uniforms, shaders, and vertex attributes.  The
// reference to the shader record has enough information to determine
// how many pointers are necessary (fixed number for shaders/uniforms,
// and an attribute count), so those BO indices into bo_handles are
// just stored as __u32s before each shader record passed in.
//
    pub shader_rec: __u64,
// Pointer to uniform data and texture handles for the textures
// referenced by the shader.
//
// For each shader state record, there is a set of uniform data in the
// order referenced by the record (FS, VS, then CS).  Each set of
// uniform data has a __u32 index into bo_handles per texture
// sample operation, in the order the QPU_W_TMUn_S writes appear in
// the program.  Following the texture BO handle indices is the actual
// uniform data.
//
// The individual uniform state blocks don't have sizes passed in,
// because the kernel has to determine the sizes anyway during shader
// code validation.
//
    pub uniforms: __u64,
    pub bo_handles: __u64,
// Size in bytes of the binner command list.
    pub bin_cl_size: __u32,
// Size in bytes of the set of shader records.
    pub shader_rec_size: __u32,
// Number of shader records.
//
// This could just be computed from the contents of shader_records and
// the address bits of references to them from the bin CL, but it
// keeps the kernel from having to resize some allocations it makes.
//
    pub shader_rec_count: __u32,
// Size in bytes of the uniform state.
    pub uniforms_size: __u32,
// Number of BO handles passed in (size is that times 4).
    pub bo_handle_count: __u32,
// RCL setup:
    pub width: __u16,
    pub height: __u16,
    pub min_x_tile: __u8,
    pub min_y_tile: __u8,
    pub max_x_tile: __u8,
    pub max_y_tile: __u8,
    pub color_read: drm_vc4_submit_rcl_surface,
    pub color_write: drm_vc4_submit_rcl_surface,
    pub zs_read: drm_vc4_submit_rcl_surface,
    pub zs_write: drm_vc4_submit_rcl_surface,
    pub msaa_color_write: drm_vc4_submit_rcl_surface,
    pub msaa_zs_write: drm_vc4_submit_rcl_surface,
    pub clear_color: [__u32; 2],
    pub clear_z: __u32,
    pub clear_s: __u8,
    pub pad:24: __u32,

// By default, the kernel gets to choose the order that the tiles are
// rendered in.  If this is set, then the tiles will be rendered in a
// raster order, with the right-to-left vs left-to-right and
// top-to-bottom vs bottom-to-top dictated by
// VC4_SUBMIT_CL_RCL_ORDER_INCREASING_*.  This allows overlapping
// blits to be implemented using the 3D engine.
//

    pub flags: __u32,
// Returned value of the seqno of this render job (for the
// wait ioctl).
//
    pub seqno: __u64,
// ID of the perfmon to attach to this job. 0 means no perfmon.
    pub perfmonid: __u32,
// Syncobj handle to wait on. If set, processing of this render job
// will not start until the syncobj is signaled. 0 means ignore.
//
    pub in_sync: __u32,
// Syncobj handle to export fence to. If set, the fence in the syncobj
// will be replaced with a fence that signals upon completion of this
// render job. 0 means ignore.
//
    pub out_sync: __u32,
    pub pad2: __u32,
}

//
// struct drm_vc4_wait_seqno - ioctl argument for waiting for
// DRM_VC4_SUBMIT_CL completion using its returned seqno.
//
// timeout_ns is the timeout in nanoseconds, where "0" means "don't
// block, just return the status."
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_wait_seqno {
    pub seqno: __u64,
    pub timeout_ns: __u64,
}

//
// struct drm_vc4_wait_bo - ioctl argument for waiting for
// completion of the last DRM_VC4_SUBMIT_CL on a BO.
//
// This is useful for cases where multiple processes might be
// rendering to a BO and you want to wait for all rendering to be
// completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_wait_bo {
    pub handle: __u32,
    pub pad: __u32,
    pub timeout_ns: __u64,
}

//
// struct drm_vc4_create_bo - ioctl argument for creating VC4 BOs.
//
// There are currently no values for the flags argument, but it may be
// used in a future extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_create_bo {
    pub size: __u32,
    pub flags: __u32,
// Returned GEM handle for the BO.
    pub handle: __u32,
    pub pad: __u32,
}

//
// struct drm_vc4_mmap_bo - ioctl argument for mapping VC4 BOs.
//
// This doesn't actually perform an mmap.  Instead, it returns the
// offset you need to use in an mmap on the DRM device node.  This
// means that tools like valgrind end up knowing about the mapped
// memory.
//
// There are currently no values for the flags argument, but it may be
// used in a future extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_mmap_bo {
// Handle for the object being mapped.
    pub handle: __u32,
    pub flags: __u32,
// offset into the drm node to use for subsequent mmap call.
    pub offset: __u64,
}

//
// struct drm_vc4_create_shader_bo - ioctl argument for creating VC4
// shader BOs.
//
// Since allowing a shader to be overwritten while it's also being
// executed from would allow privlege escalation, shaders must be
// created using this ioctl, and they can't be mmapped later.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_create_shader_bo {
// Size of the data argument.
    pub size: __u32,
// Flags, currently must be 0.
    pub flags: __u32,
// Pointer to the data.
    pub data: __u64,
// Returned GEM handle for the BO.
    pub handle: __u32,
// Pad, must be 0.
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_hang_state_bo {
    pub handle: __u32,
    pub paddr: __u32,
    pub size: __u32,
    pub pad: __u32,
}

//
// struct drm_vc4_hang_state - ioctl argument for collecting state
// from a GPU hang for analysis.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_hang_state {
// Pointer to array of struct drm_vc4_get_hang_state_bo.
    pub bo: __u64,
//
// On input, the size of the bo array.  Output is the number
// of bos to be returned.
//
    pub bo_count: __u32,
    pub start_render: __u32 start_bin,,
    pub ct0ea: __u32 ct0ca,,
    pub ct1ea: __u32 ct1ca,,
    pub ct1cs: __u32 ct0cs,,
    pub ct1ra0: __u32 ct0ra0,,
    pub bpcs: __u32 bpca,,
    pub bpos: __u32 bpoa,,
    pub vpmbase: __u32,
    pub dbge: __u32,
    pub fdbgo: __u32,
    pub fdbgb: __u32,
    pub fdbgr: __u32,
    pub fdbgs: __u32,
    pub errstat: __u32,
// Pad that we may save more registers into in the future.
    pub pad: [__u32; 16],
}

pub const DRM_VC4_PARAM_V3D_IDENT0: c_int = 0;
pub const DRM_VC4_PARAM_V3D_IDENT1: c_int = 1;
pub const DRM_VC4_PARAM_V3D_IDENT2: c_int = 2;
pub const DRM_VC4_PARAM_SUPPORTS_BRANCHES: c_int = 3;
pub const DRM_VC4_PARAM_SUPPORTS_ETC1: c_int = 4;
pub const DRM_VC4_PARAM_SUPPORTS_THREADED_FS: c_int = 5;
pub const DRM_VC4_PARAM_SUPPORTS_FIXED_RCL_ORDER: c_int = 6;
pub const DRM_VC4_PARAM_SUPPORTS_MADVISE: c_int = 7;
pub const DRM_VC4_PARAM_SUPPORTS_PERFMON: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_param {
    pub param: __u32,
    pub pad: __u32,
    pub value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_get_tiling {
    pub handle: __u32,
    pub flags: __u32,
    pub modifier: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_set_tiling {
    pub handle: __u32,
    pub flags: __u32,
    pub modifier: __u64,
}

//
// struct drm_vc4_label_bo - Attach a name to a BO for debug purposes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_label_bo {
    pub handle: __u32,
    pub len: __u32,
    pub name: __u64,
}

//
// States prefixed with '__' are internal states and cannot be passed to the
// DRM_IOCTL_VC4_GEM_MADVISE ioctl.
//
pub const VC4_MADV_WILLNEED: c_int = 0;
pub const VC4_MADV_DONTNEED: c_int = 1;
pub const __VC4_MADV_PURGED: c_int = 2;
pub const __VC4_MADV_NOTSUPP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_gem_madvise {
    pub handle: __u32,
    pub madv: __u32,
    pub retained: __u32,
    pub pad: __u32,
}

pub const DRM_VC4_MAX_PERF_COUNTERS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_perfmon_create {
    pub id: __u32,
    pub ncounters: __u32,
    pub events: [__u8; DRM_VC4_MAX_PERF_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_perfmon_destroy {
    pub id: __u32,
}

//
// Returns the values of the performance counters tracked by this
// perfmon (as an array of ncounters u64 values).
//
// No implicit synchronization is performed, so the user has to
// guarantee that any jobs using this perfmon have already been
// completed  (probably by blocking on the seqno returned by the
// last exec that used the perfmon).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_vc4_perfmon_get_values {
    pub id: __u32,
    pub values_ptr: __u64,
}

