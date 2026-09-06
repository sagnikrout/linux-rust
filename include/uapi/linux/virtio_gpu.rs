//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_gpu.h
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
// Virtio GPU Device
//
// Copyright Red Hat, Inc. 2013-2014
//
// Authors:
// Dave Airlie <airlied@redhat.com>
// Gerd Hoffmann <kraxel@redhat.com>
//
// This header is BSD licensed so anyone can use the definitions
// to implement compatible drivers/servers:
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of IBM nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
// USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
// OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

//
// VIRTIO_GPU_CMD_CTX_
// VIRTIO_GPU_CMD_*_3D
//
pub const VIRTIO_GPU_F_VIRGL: c_int = 0;
//
// VIRTIO_GPU_CMD_GET_EDID
//
pub const VIRTIO_GPU_F_EDID: c_int = 1;
//
// VIRTIO_GPU_CMD_RESOURCE_ASSIGN_UUID
//
pub const VIRTIO_GPU_F_RESOURCE_UUID: c_int = 2;
//
// VIRTIO_GPU_CMD_RESOURCE_CREATE_BLOB
//
pub const VIRTIO_GPU_F_RESOURCE_BLOB: c_int = 3;
//
// VIRTIO_GPU_CMD_CREATE_CONTEXT with
// context_init and multiple timelines
//
pub const VIRTIO_GPU_F_CONTEXT_INIT: c_int = 4;
//
// The device provides a valid blob_alignment
// field in its configuration and both
// VIRTIO_GPU_CMD_RESOURCE_CREATE_BLOB and
// VIRTIO_GPU_CMD_RESOURCE_MAP_BLOB requests
// must be aligned to that value.
//
pub const VIRTIO_GPU_F_BLOB_ALIGNMENT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_gpu_ctrl_type {
    VIRTIO_GPU_UNDEFINED = 0,

// 2d commands
    VIRTIO_GPU_CMD_GET_DISPLAY_INFO = 0x0100,
    VIRTIO_GPU_CMD_RESOURCE_CREATE_2D,
    VIRTIO_GPU_CMD_RESOURCE_UNREF,
    VIRTIO_GPU_CMD_SET_SCANOUT,
    VIRTIO_GPU_CMD_RESOURCE_FLUSH,
    VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D,
    VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING,
    VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING,
    VIRTIO_GPU_CMD_GET_CAPSET_INFO,
    VIRTIO_GPU_CMD_GET_CAPSET,
    VIRTIO_GPU_CMD_GET_EDID,
    VIRTIO_GPU_CMD_RESOURCE_ASSIGN_UUID,
    VIRTIO_GPU_CMD_RESOURCE_CREATE_BLOB,
    VIRTIO_GPU_CMD_SET_SCANOUT_BLOB,

// 3d commands
    VIRTIO_GPU_CMD_CTX_CREATE = 0x0200,
    VIRTIO_GPU_CMD_CTX_DESTROY,
    VIRTIO_GPU_CMD_CTX_ATTACH_RESOURCE,
    VIRTIO_GPU_CMD_CTX_DETACH_RESOURCE,
    VIRTIO_GPU_CMD_RESOURCE_CREATE_3D,
    VIRTIO_GPU_CMD_TRANSFER_TO_HOST_3D,
    VIRTIO_GPU_CMD_TRANSFER_FROM_HOST_3D,
    VIRTIO_GPU_CMD_SUBMIT_3D,
    VIRTIO_GPU_CMD_RESOURCE_MAP_BLOB,
    VIRTIO_GPU_CMD_RESOURCE_UNMAP_BLOB,

// cursor commands
    VIRTIO_GPU_CMD_UPDATE_CURSOR = 0x0300,
    VIRTIO_GPU_CMD_MOVE_CURSOR,

// success responses
    VIRTIO_GPU_RESP_OK_NODATA = 0x1100,
    VIRTIO_GPU_RESP_OK_DISPLAY_INFO,
    VIRTIO_GPU_RESP_OK_CAPSET_INFO,
    VIRTIO_GPU_RESP_OK_CAPSET,
    VIRTIO_GPU_RESP_OK_EDID,
    VIRTIO_GPU_RESP_OK_RESOURCE_UUID,
    VIRTIO_GPU_RESP_OK_MAP_INFO,

// error responses
    VIRTIO_GPU_RESP_ERR_UNSPEC = 0x1200,
    VIRTIO_GPU_RESP_ERR_OUT_OF_MEMORY,
    VIRTIO_GPU_RESP_ERR_INVALID_SCANOUT_ID,
    VIRTIO_GPU_RESP_ERR_INVALID_RESOURCE_ID,
    VIRTIO_GPU_RESP_ERR_INVALID_CONTEXT_ID,
    VIRTIO_GPU_RESP_ERR_INVALID_PARAMETER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_gpu_shm_id {
    VIRTIO_GPU_SHM_ID_UNDEFINED = 0,
//
// VIRTIO_GPU_CMD_RESOURCE_MAP_BLOB
// VIRTIO_GPU_CMD_RESOURCE_UNMAP_BLOB
//
    VIRTIO_GPU_SHM_ID_HOST_VISIBLE = 1
}

//
// If the following flag is set, then ring_idx contains the index
// of the command ring that needs to used when creating the fence
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_ctrl_hdr {
    pub type: __le32,
    pub flags: __le32,
    pub fence_id: __le64,
    pub ctx_id: __le32,
    pub ring_idx: __u8,
    pub padding: [__u8; 3],
}

// data passed in the cursor vq
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_cursor_pos {
    pub scanout_id: __le32,
    pub x: __le32,
    pub y: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_CMD_UPDATE_CURSOR, VIRTIO_GPU_CMD_MOVE_CURSOR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_update_cursor {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub /: *mut *mut virtio_gpu_cursor_pos pos; / update & move,
    pub /: *mut *mut __le32 resource_id; / update only,
    pub /: *mut *mut __le32 hot_x; / update only,
    pub /: *mut *mut __le32 hot_y; / update only,
    pub padding: __le32,
}

// data passed in the control vq, 2d related
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_rect {
    pub x: __le32,
    pub y: __le32,
    pub width: __le32,
    pub height: __le32,
}

// VIRTIO_GPU_CMD_RESOURCE_UNREF
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_unref {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: create a 2d resource with a format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_create_2d {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub format: __le32,
    pub width: __le32,
    pub height: __le32,
}

// VIRTIO_GPU_CMD_SET_SCANOUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_set_scanout {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub r: virtio_gpu_rect,
    pub scanout_id: __le32,
    pub resource_id: __le32,
}

// VIRTIO_GPU_CMD_RESOURCE_FLUSH
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_flush {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub r: virtio_gpu_rect,
    pub resource_id: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: simple transfer to_host
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_transfer_to_host_2d {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub r: virtio_gpu_rect,
    pub offset: __le64,
    pub resource_id: __le32,
    pub padding: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_mem_entry {
    pub addr: __le64,
    pub length: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_attach_backing {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub nr_entries: __le32,
}

// VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_detach_backing {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_RESP_OK_DISPLAY_INFO
pub const VIRTIO_GPU_MAX_SCANOUTS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resp_display_info {
    pub hdr: virtio_gpu_ctrl_hdr,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_display_one {
    pub r: virtio_gpu_rect,
    pub enabled: __le32,
    pub flags: __le32,
    pub pmodes: [}; VIRTIO_GPU_MAX_SCANOUTS],
}

// data passed in the control vq, 3d related
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_box {
    pub z: __le32 x, y,,
    pub d: __le32 w, h,,
}

// VIRTIO_GPU_CMD_TRANSFER_TO_HOST_3D, VIRTIO_GPU_CMD_TRANSFER_FROM_HOST_3D
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_transfer_host_3d {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub box: virtio_gpu_box,
    pub offset: __le64,
    pub resource_id: __le32,
    pub level: __le32,
    pub stride: __le32,
    pub layer_stride: __le32,
}

// VIRTIO_GPU_CMD_RESOURCE_CREATE_3D

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_create_3d {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub target: __le32,
    pub format: __le32,
    pub bind: __le32,
    pub width: __le32,
    pub height: __le32,
    pub depth: __le32,
    pub array_size: __le32,
    pub last_level: __le32,
    pub nr_samples: __le32,
    pub flags: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_CMD_CTX_CREATE
pub const VIRTIO_GPU_CONTEXT_INIT_CAPSET_ID_MASK: c_uint = 0x000000ff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_ctx_create {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub nlen: __le32,
    pub context_init: __le32,
    pub debug_name: [c_char; 64],
}

// VIRTIO_GPU_CMD_CTX_DESTROY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_ctx_destroy {
    pub hdr: virtio_gpu_ctrl_hdr,
}

// VIRTIO_GPU_CMD_CTX_ATTACH_RESOURCE, VIRTIO_GPU_CMD_CTX_DETACH_RESOURCE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_ctx_resource {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_CMD_SUBMIT_3D
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_cmd_submit {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub size: __le32,
    pub padding: __le32,
}

pub const VIRTIO_GPU_CAPSET_VIRGL: c_int = 1;
pub const VIRTIO_GPU_CAPSET_VIRGL2: c_int = 2;
pub const VIRTIO_GPU_CAPSET_GFXSTREAM_VULKAN: c_int = 3;
pub const VIRTIO_GPU_CAPSET_VENUS: c_int = 4;
pub const VIRTIO_GPU_CAPSET_CROSS_DOMAIN: c_int = 5;
pub const VIRTIO_GPU_CAPSET_DRM: c_int = 6;
// VIRTIO_GPU_CMD_GET_CAPSET_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_get_capset_info {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub capset_index: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_RESP_OK_CAPSET_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resp_capset_info {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub capset_id: __le32,
    pub capset_max_version: __le32,
    pub capset_max_size: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_CMD_GET_CAPSET
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_get_capset {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub capset_id: __le32,
    pub capset_version: __le32,
}

// VIRTIO_GPU_RESP_OK_CAPSET
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resp_capset {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub capset_data: [__u8; ],
}

// VIRTIO_GPU_CMD_GET_EDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_cmd_get_edid {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub scanout: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_RESP_OK_EDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resp_edid {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub size: __le32,
    pub padding: __le32,
    pub edid: [__u8; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_config {
    pub events_read: __le32,
    pub events_clear: __le32,
    pub num_scanouts: __le32,
    pub num_capsets: __le32,
    pub blob_alignment: __le32,
}

// simple formats for fbcon/X use
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum virtio_gpu_formats {
    VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM  = 1,
    VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM  = 2,
    VIRTIO_GPU_FORMAT_A8R8G8B8_UNORM  = 3,
    VIRTIO_GPU_FORMAT_X8R8G8B8_UNORM  = 4,

    VIRTIO_GPU_FORMAT_R8G8B8A8_UNORM  = 67,
    VIRTIO_GPU_FORMAT_X8B8G8R8_UNORM  = 68,

    VIRTIO_GPU_FORMAT_A8B8G8R8_UNORM  = 121,
    VIRTIO_GPU_FORMAT_R8G8B8X8_UNORM  = 134,
}

// VIRTIO_GPU_CMD_RESOURCE_ASSIGN_UUID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_assign_uuid {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub padding: __le32,
}

// VIRTIO_GPU_RESP_OK_RESOURCE_UUID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resp_resource_uuid {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub uuid: [__u8; 16],
}

// VIRTIO_GPU_CMD_RESOURCE_CREATE_BLOB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_create_blob {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
pub const VIRTIO_GPU_BLOB_MEM_GUEST: c_uint = 0x0001;
pub const VIRTIO_GPU_BLOB_MEM_HOST3D: c_uint = 0x0002;
pub const VIRTIO_GPU_BLOB_MEM_HOST3D_GUEST: c_uint = 0x0003;
pub const VIRTIO_GPU_BLOB_FLAG_USE_MAPPABLE: c_uint = 0x0001;
pub const VIRTIO_GPU_BLOB_FLAG_USE_SHAREABLE: c_uint = 0x0002;
pub const VIRTIO_GPU_BLOB_FLAG_USE_CROSS_DEVICE: c_uint = 0x0004;
// zero is invalid blob mem
    pub blob_mem: __le32,
    pub blob_flags: __le32,
    pub nr_entries: __le32,
    pub blob_id: __le64,
    pub size: __le64,
//
// sizeof(nr_entries * virtio_gpu_mem_entry) bytes follow
//
}

// VIRTIO_GPU_CMD_SET_SCANOUT_BLOB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_set_scanout_blob {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub r: virtio_gpu_rect,
    pub scanout_id: __le32,
    pub resource_id: __le32,
    pub width: __le32,
    pub height: __le32,
    pub format: __le32,
    pub padding: __le32,
    pub strides: [__le32; 4],
    pub offsets: [__le32; 4],
}

// VIRTIO_GPU_CMD_RESOURCE_MAP_BLOB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_map_blob {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub padding: __le32,
    pub offset: __le64,
}

// VIRTIO_GPU_RESP_OK_MAP_INFO
pub const VIRTIO_GPU_MAP_CACHE_MASK: c_uint = 0x0f;
pub const VIRTIO_GPU_MAP_CACHE_NONE: c_uint = 0x00;
pub const VIRTIO_GPU_MAP_CACHE_CACHED: c_uint = 0x01;
pub const VIRTIO_GPU_MAP_CACHE_UNCACHED: c_uint = 0x02;
pub const VIRTIO_GPU_MAP_CACHE_WC: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resp_map_info {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub map_info: __u32,
    pub padding: __u32,
}

// VIRTIO_GPU_CMD_RESOURCE_UNMAP_BLOB
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpu_resource_unmap_blob {
    pub hdr: virtio_gpu_ctrl_hdr,
    pub resource_id: __le32,
    pub padding: __le32,
}
