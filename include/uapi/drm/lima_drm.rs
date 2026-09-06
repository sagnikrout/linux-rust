//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/lima_drm.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR MIT
// Copyright 2017-2018 Qiang Yu <yuq825@gmail.com>

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_lima_param_gpu_id {
    DRM_LIMA_PARAM_GPU_ID_UNKNOWN,
    DRM_LIMA_PARAM_GPU_ID_MALI400,
    DRM_LIMA_PARAM_GPU_ID_MALI450,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_lima_param {
    DRM_LIMA_PARAM_GPU_ID,
    DRM_LIMA_PARAM_NUM_PP,
    DRM_LIMA_PARAM_GP_VERSION,
    DRM_LIMA_PARAM_PP_VERSION,
}

//
// get various information of the GPU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_get_param {
    pub /: *mut *mut __u32 param; / in, value in enum drm_lima_param,
    pub /: *mut *mut __u32 pad; / pad, must be zero,
    pub /: *mut *mut __u64 value; / out, parameter value,
}

//
// heap buffer dynamically increase backup memory size when GP task fail
// due to lack of heap memory. size field of heap buffer is an up bound of
// the backup memory which can be set to a fairly large value.
//

//
// create a buffer for used by GPU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_create {
    pub /: *mut *mut __u32 size; / in, buffer size,
    pub /: *mut *mut __u32 flags; / in, buffer flags,
    pub /: *mut *mut __u32 handle; / out, GEM buffer handle,
    pub /: *mut *mut __u32 pad; / pad, must be zero,
}

//
// get information of a buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_info {
    pub /: *mut *mut __u32 handle; / in, GEM buffer handle,
    pub /: *mut *mut __u32 va; / out, virtual address mapped into GPU MMU,
    pub /: *mut *mut __u64 offset; / out, used to mmap this buffer to CPU,
}

pub const LIMA_SUBMIT_BO_READ: c_uint = 0x01;
pub const LIMA_SUBMIT_BO_WRITE: c_uint = 0x02;
// buffer information used by one task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_submit_bo {
    pub /: *mut *mut __u32 handle; / in, GEM buffer handle,
    pub /: *mut *mut __u32 flags; / in, buffer read/write by GPU,
}

pub const LIMA_GP_FRAME_REG_NUM: c_int = 6;
// frame used to setup GP for each task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gp_frame {
    pub frame: [__u32; LIMA_GP_FRAME_REG_NUM],
}

pub const LIMA_PP_FRAME_REG_NUM: c_int = 23;
pub const LIMA_PP_WB_REG_NUM: c_int = 12;
// frame used to setup mali400 GPU PP for each task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_m400_pp_frame {
    pub frame: [__u32; LIMA_PP_FRAME_REG_NUM],
    pub num_pp: __u32,
    pub LIMA_PP_WB_REG_NUM]: *mut *mut __u32 wb[3,
    pub plbu_array_address: [__u32; 4],
    pub fragment_stack_address: [__u32; 4],
}

// frame used to setup mali450 GPU PP for each task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_m450_pp_frame {
    pub frame: [__u32; LIMA_PP_FRAME_REG_NUM],
    pub num_pp: __u32,
    pub LIMA_PP_WB_REG_NUM]: *mut *mut __u32 wb[3,
    pub use_dlbu: __u32,
    pub _pad: __u32,
    pub plbu_array_address: [__u32; 8],
    pub dlbu_regs: [__u32; 4],
}

pub const LIMA_PIPE_GP: c_uint = 0x00;
pub const LIMA_PIPE_PP: c_uint = 0x01;

//
// submit a task to GPU
//
// User can always merge multi sync_file and drm_syncobj
// into one drm_syncobj as in_sync[0], but we reserve
// in_sync[1] for another task's out_sync to avoid the
// export/import/merge pass when explicit sync.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_submit {
    pub /: *mut *mut __u32 ctx; / in, context handle task is submitted to,
    pub /: *mut *mut __u32 pipe; / in, which pipe to use, GP/PP,
    pub /: *mut *mut __u32 nr_bos; / in, array length of bos field,
    pub /: *mut *mut __u32 frame_size; / in, size of frame field,
    pub /: *mut *mut __u64 bos; / in, array of drm_lima_gem_submit_bo,
    pub /: *mut *mut __u64 frame; / in, GP/PP frame,
    pub /: *mut *mut __u32 flags; / in, submit flags,
    pub /: *mut *mut __u32 out_sync; / in, drm_syncobj handle used to wait task finish after submission,
    pub /: *mut *mut __u32 in_sync[2]; / in, drm_syncobj handle used to wait before start this task,
}

pub const LIMA_GEM_WAIT_READ: c_uint = 0x01;
pub const LIMA_GEM_WAIT_WRITE: c_uint = 0x02;
//
// wait pending GPU task finish of a buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_gem_wait {
    pub /: *mut *mut __u32 handle; / in, GEM buffer handle,
    pub /: *mut *mut __u32 op; / in, CPU want to read/write this buffer,
    pub /: *mut *mut __s64 timeout_ns; / in, wait timeout in absulute time,
}

//
// create a context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_ctx_create {
    pub /: *mut *mut __u32 id; / out, context handle,
    pub /: *mut *mut __u32 _pad; / pad, must be zero,
}

//
// free a context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_lima_ctx_free {
    pub /: *mut *mut __u32 id; / in, context handle,
    pub /: *mut *mut __u32 _pad; / pad, must be zero,
}

pub const DRM_LIMA_GET_PARAM: c_uint = 0x00;
pub const DRM_LIMA_GEM_CREATE: c_uint = 0x01;
pub const DRM_LIMA_GEM_INFO: c_uint = 0x02;
pub const DRM_LIMA_GEM_SUBMIT: c_uint = 0x03;
pub const DRM_LIMA_GEM_WAIT: c_uint = 0x04;
pub const DRM_LIMA_CTX_CREATE: c_uint = 0x05;
pub const DRM_LIMA_CTX_FREE: c_uint = 0x06;

