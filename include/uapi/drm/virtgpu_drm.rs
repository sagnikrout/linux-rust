//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/virtgpu_drm.h
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
// Copyright 2013 Red Hat
// All Rights Reserved.
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
// THE AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Please note that modifications to all structs defined here are
// subject to backwards-compatibility constraints.
//
// Do not use pointers, use __u64 instead for 32 bit / 64 bit user/kernel
// compatibility Keep fields aligned to their size
//
pub const DRM_VIRTGPU_MAP: c_uint = 0x01;
pub const DRM_VIRTGPU_EXECBUFFER: c_uint = 0x02;
pub const DRM_VIRTGPU_GETPARAM: c_uint = 0x03;
pub const DRM_VIRTGPU_RESOURCE_CREATE: c_uint = 0x04;
pub const DRM_VIRTGPU_RESOURCE_INFO: c_uint = 0x05;
pub const DRM_VIRTGPU_TRANSFER_FROM_HOST: c_uint = 0x06;
pub const DRM_VIRTGPU_TRANSFER_TO_HOST: c_uint = 0x07;
pub const DRM_VIRTGPU_WAIT: c_uint = 0x08;
pub const DRM_VIRTGPU_GET_CAPS: c_uint = 0x09;
pub const DRM_VIRTGPU_RESOURCE_CREATE_BLOB: c_uint = 0x0a;
pub const DRM_VIRTGPU_CONTEXT_INIT: c_uint = 0x0b;
pub const VIRTGPU_EXECBUF_FENCE_FD_IN: c_uint = 0x01;
pub const VIRTGPU_EXECBUF_FENCE_FD_OUT: c_uint = 0x02;
pub const VIRTGPU_EXECBUF_RING_IDX: c_uint = 0x04;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_map {
    pub /: *mut *mut __u64 offset; / use for mmap system call,
    pub handle: __u32,
    pub pad: __u32,
}

pub const VIRTGPU_EXECBUF_SYNCOBJ_RESET: c_uint = 0x01;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_execbuffer_syncobj {
    pub handle: __u32,
    pub flags: __u32,
    pub point: __u64,
}

// fence_fd is modified on success if VIRTGPU_EXECBUF_FENCE_FD_OUT flag is set.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_execbuffer {
    pub flags: __u32,
    pub size: __u32,
    pub /: *mut *mut *mut __u64 command; / void,
    pub bo_handles: __u64,
    pub num_bo_handles: __u32,
    pub /: *mut *mut __s32 fence_fd; / in/out fence fd (see VIRTGPU_EXECBUF_FENCE_FD_IN/OUT),
    pub /: *mut *mut __u32 ring_idx; / command ring index (see VIRTGPU_EXECBUF_RING_IDX),
    pub /: *mut *mut __u32 syncobj_stride; / size of @drm_virtgpu_execbuffer_syncobj,
    pub num_in_syncobjs: __u32,
    pub num_out_syncobjs: __u32,
    pub in_syncobjs: __u64,
    pub out_syncobjs: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_getparam {
    pub param: __u64,
    pub value: __u64,
}

// NO_BO flags? NO resource flag?
// resource flag for y_0_top
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_resource_create {
    pub target: __u32,
    pub format: __u32,
    pub bind: __u32,
    pub width: __u32,
    pub height: __u32,
    pub depth: __u32,
    pub array_size: __u32,
    pub last_level: __u32,
    pub nr_samples: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u32 bo_handle; / if this is set - recreate a new resource attached to this bo ?,
    pub /: *mut *mut __u32 res_handle; / returned by kernel,
    pub /: *mut *mut __u32 size; / validate transfer in the host,
    pub /: *mut *mut __u32 stride; / validate transfer in the host,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_resource_info {
    pub bo_handle: __u32,
    pub res_handle: __u32,
    pub size: __u32,
    pub blob_mem: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_box {
    pub x: __u32,
    pub y: __u32,
    pub z: __u32,
    pub w: __u32,
    pub h: __u32,
    pub d: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_transfer_to_host {
    pub bo_handle: __u32,
    pub box: drm_virtgpu_3d_box,
    pub level: __u32,
    pub offset: __u32,
    pub stride: __u32,
    pub layer_stride: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_transfer_from_host {
    pub bo_handle: __u32,
    pub box: drm_virtgpu_3d_box,
    pub level: __u32,
    pub offset: __u32,
    pub stride: __u32,
    pub layer_stride: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_3d_wait {
    pub /: *mut *mut __u32 handle; / 0 is an invalid handle,
    pub flags: __u32,
}

pub const VIRTGPU_DRM_CAPSET_VIRGL: c_int = 1;
pub const VIRTGPU_DRM_CAPSET_VIRGL2: c_int = 2;
pub const VIRTGPU_DRM_CAPSET_GFXSTREAM_VULKAN: c_int = 3;
pub const VIRTGPU_DRM_CAPSET_VENUS: c_int = 4;
pub const VIRTGPU_DRM_CAPSET_CROSS_DOMAIN: c_int = 5;
pub const VIRTGPU_DRM_CAPSET_DRM: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_get_caps {
    pub cap_set_id: __u32,
    pub cap_set_ver: __u32,
    pub addr: __u64,
    pub size: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_resource_create_blob {
pub const VIRTGPU_BLOB_MEM_GUEST: c_uint = 0x0001;
pub const VIRTGPU_BLOB_MEM_HOST3D: c_uint = 0x0002;
pub const VIRTGPU_BLOB_MEM_HOST3D_GUEST: c_uint = 0x0003;
pub const VIRTGPU_BLOB_FLAG_USE_MAPPABLE: c_uint = 0x0001;
pub const VIRTGPU_BLOB_FLAG_USE_SHAREABLE: c_uint = 0x0002;
pub const VIRTGPU_BLOB_FLAG_USE_CROSS_DEVICE: c_uint = 0x0004;
// zero is invalid blob_mem
    pub blob_mem: __u32,
    pub blob_flags: __u32,
    pub bo_handle: __u32,
    pub res_handle: __u32,
    pub size: __u64,
//
// for 3D contexts with VIRTGPU_BLOB_MEM_HOST3D_GUEST and
// VIRTGPU_BLOB_MEM_HOST3D otherwise, must be zero.
//
    pub pad: __u32,
    pub cmd_size: __u32,
    pub cmd: __u64,
    pub blob_id: __u64,
pub const DRM_VIRTGPU_BLOB_FLAG_HINT_DEFER_MAPPING: c_uint = 0x0001;
    pub blob_hints: __u32,
    pub pad2: __u32,
}

pub const VIRTGPU_CONTEXT_PARAM_CAPSET_ID: c_uint = 0x0001;
pub const VIRTGPU_CONTEXT_PARAM_NUM_RINGS: c_uint = 0x0002;
pub const VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK: c_uint = 0x0003;
pub const VIRTGPU_CONTEXT_PARAM_DEBUG_NAME: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_context_set_param {
    pub param: __u64,
    pub value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_virtgpu_context_init {
    pub num_params: __u32,
    pub pad: __u32,
// pointer to drm_virtgpu_context_set_param array
    pub ctx_set_params: __u64,
}

//
// Event code that's given when VIRTGPU_CONTEXT_PARAM_POLL_RINGS_MASK is in
// effect.  The event size is sizeof(drm_event), since there is no additional
// payload.
//
pub const VIRTGPU_EVENT_FENCE_SIGNALED: c_uint = 0x90000000;

