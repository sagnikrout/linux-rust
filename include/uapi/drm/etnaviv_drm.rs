//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/etnaviv_drm.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (C) 2015 Etnaviv Project
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <http://www.gnu.org/licenses/>.
//

// Please note that modifications to all structs defined here are
// subject to backwards-compatibility constraints:
// 1) Do not use pointers, use __u64 instead for 32 bit / 64 bit
// user/kernel compatibility
// 2) Keep fields aligned to their size
// 3) Because of how drm_ioctl() works, we can add new fields at
// the end of an ioctl if some care is taken: drm_ioctl() will
// zero out the new fields at the tail of the ioctl, so a zero
// value should have a backwards compatible meaning.  And for
// output params, userspace won't see the newly added output
// fields.. so that has to be somehow ok.
//
// timeouts are specified in clock-monotonic absolute times (to simplify
// restarting interrupted ioctls).  The following struct is logically the
// same as 'struct timespec' but 32/64b ABI safe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_timespec {
    pub /: *mut *mut __s64 tv_sec; / seconds,
    pub /: *mut *mut __s64 tv_nsec; / nanoseconds,
}

pub const ETNAVIV_PARAM_GPU_MODEL: c_uint = 0x01;
pub const ETNAVIV_PARAM_GPU_REVISION: c_uint = 0x02;
pub const ETNAVIV_PARAM_GPU_FEATURES_0: c_uint = 0x03;
pub const ETNAVIV_PARAM_GPU_FEATURES_1: c_uint = 0x04;
pub const ETNAVIV_PARAM_GPU_FEATURES_2: c_uint = 0x05;
pub const ETNAVIV_PARAM_GPU_FEATURES_3: c_uint = 0x06;
pub const ETNAVIV_PARAM_GPU_FEATURES_4: c_uint = 0x07;
pub const ETNAVIV_PARAM_GPU_FEATURES_5: c_uint = 0x08;
pub const ETNAVIV_PARAM_GPU_FEATURES_6: c_uint = 0x09;
pub const ETNAVIV_PARAM_GPU_FEATURES_7: c_uint = 0x0a;
pub const ETNAVIV_PARAM_GPU_FEATURES_8: c_uint = 0x0b;
pub const ETNAVIV_PARAM_GPU_FEATURES_9: c_uint = 0x0c;
pub const ETNAVIV_PARAM_GPU_FEATURES_10: c_uint = 0x0d;
pub const ETNAVIV_PARAM_GPU_FEATURES_11: c_uint = 0x0e;
pub const ETNAVIV_PARAM_GPU_FEATURES_12: c_uint = 0x0f;
pub const ETNAVIV_PARAM_GPU_STREAM_COUNT: c_uint = 0x10;
pub const ETNAVIV_PARAM_GPU_REGISTER_MAX: c_uint = 0x11;
pub const ETNAVIV_PARAM_GPU_THREAD_COUNT: c_uint = 0x12;
pub const ETNAVIV_PARAM_GPU_VERTEX_CACHE_SIZE: c_uint = 0x13;
pub const ETNAVIV_PARAM_GPU_SHADER_CORE_COUNT: c_uint = 0x14;
pub const ETNAVIV_PARAM_GPU_PIXEL_PIPES: c_uint = 0x15;
pub const ETNAVIV_PARAM_GPU_VERTEX_OUTPUT_BUFFER_SIZE: c_uint = 0x16;
pub const ETNAVIV_PARAM_GPU_BUFFER_SIZE: c_uint = 0x17;
pub const ETNAVIV_PARAM_GPU_INSTRUCTION_COUNT: c_uint = 0x18;
pub const ETNAVIV_PARAM_GPU_NUM_CONSTANTS: c_uint = 0x19;
pub const ETNAVIV_PARAM_GPU_NUM_VARYINGS: c_uint = 0x1a;
pub const ETNAVIV_PARAM_SOFTPIN_START_ADDR: c_uint = 0x1b;
pub const ETNAVIV_PARAM_GPU_PRODUCT_ID: c_uint = 0x1c;
pub const ETNAVIV_PARAM_GPU_CUSTOMER_ID: c_uint = 0x1d;
pub const ETNAVIV_PARAM_GPU_ECO_ID: c_uint = 0x1e;
pub const ETNA_MAX_PIPES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_param {
    pub /: *mut *mut __u32 pipe; / in,
    pub /: *mut *mut __u32 param; / in, ETNAVIV_PARAM_x,
    pub /: *mut *mut __u64 value; / out (get_param) or in (set_param),
}

//
// GEM buffers:
//
pub const ETNA_BO_CACHE_MASK: c_uint = 0x000f0000;
// cache modes
pub const ETNA_BO_CACHED: c_uint = 0x00010000;
pub const ETNA_BO_WC: c_uint = 0x00020000;
pub const ETNA_BO_UNCACHED: c_uint = 0x00040000;
// map flags
pub const ETNA_BO_FORCE_MMU: c_uint = 0x00100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_new {
    pub /: *mut *mut __u64 size; / in,
    pub /: *mut *mut __u32 flags; / in, mask of ETNA_BO_x,
    pub /: *mut *mut __u32 handle; / out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_info {
    pub /: *mut *mut __u32 handle; / in,
    pub pad: __u32,
    pub /: *mut *mut __u64 offset; / out, offset to pass to mmap(),
}

pub const ETNA_PREP_READ: c_uint = 0x01;
pub const ETNA_PREP_WRITE: c_uint = 0x02;
pub const ETNA_PREP_NOSYNC: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_cpu_prep {
    pub /: *mut *mut __u32 handle; / in,
    pub /: *mut *mut __u32 op; / in, mask of ETNA_PREP_x,
    pub /: *mut *mut drm_etnaviv_timespec timeout; / in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_cpu_fini {
    pub /: *mut *mut __u32 handle; / in,
    pub /: *mut *mut __u32 flags; / in, placeholder for now, no defined values,
}

//
// Cmdstream Submission:
//
// The value written into the cmdstream is logically:
// relocbuf->gpuaddr + reloc_offset
//
// NOTE that reloc's must be sorted by order of increasing submit_offset,
// otherwise EINVAL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_submit_reloc {
    pub /: *mut *mut __u32 submit_offset; / in, offset from submit_bo,
    pub /: *mut *mut __u32 reloc_idx; / in, index of reloc_bo buffer,
    pub /: *mut *mut __u64 reloc_offset; / in, offset from start of reloc_bo,
    pub /: *mut *mut __u32 flags; / in, placeholder for now, no defined values,
}

// Each buffer referenced elsewhere in the cmdstream submit (ie. the
// cmdstream buffer(s) themselves or reloc entries) has one (and only
// one) entry in the submit->bos[] table.
//
// As a optimization, the current buffer (gpu virtual address) can be
// passed back through the 'presumed' field.  If on a subsequent reloc,
// userspace passes back a 'presumed' address that is still valid,
// then patching the cmdstream for this entry is skipped.  This can
// avoid kernel needing to map/access the cmdstream bo in the common
// case.
// If the submit is a softpin submit (ETNA_SUBMIT_SOFTPIN) the 'presumed'
// field is interpreted as the fixed location to map the bo into the gpu
// virtual address space. If the kernel is unable to map the buffer at
// this location the submit will fail. This means userspace is responsible
// for the whole gpu virtual address management.
//
pub const ETNA_SUBMIT_BO_READ: c_uint = 0x0001;
pub const ETNA_SUBMIT_BO_WRITE: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_submit_bo {
    pub /: *mut *mut __u32 flags; / in, mask of ETNA_SUBMIT_BO_x,
    pub /: *mut *mut __u32 handle; / in, GEM handle,
    pub /: *mut *mut __u64 presumed; / in/out, presumed buffer address,
}

// performance monitor request (pmr)
pub const ETNA_PM_PROCESS_PRE: c_uint = 0x0001;
pub const ETNA_PM_PROCESS_POST: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_submit_pmr {
    pub /: *mut *mut __u32 flags; / in, when to process request (ETNA_PM_PROCESS_x),
    pub /: *mut *mut __u8 domain; / in, pm domain,
    pub pad: __u8,
    pub /: *mut *mut __u16 signal; / in, pm signal,
    pub /: *mut *mut __u32 sequence; / in, sequence number,
    pub /: *mut *mut __u32 read_offset; / in, offset from read_bo,
    pub /: *mut *mut __u32 read_idx; / in, index of read_bo buffer,
}

// Each cmdstream submit consists of a table of buffers involved, and
// one or more cmdstream buffers.  This allows for conditional execution
// (context-restore), and IB buffers needed for per tile/bin draw cmds.
//
pub const ETNA_SUBMIT_NO_IMPLICIT: c_uint = 0x0001;
pub const ETNA_SUBMIT_FENCE_FD_IN: c_uint = 0x0002;
pub const ETNA_SUBMIT_FENCE_FD_OUT: c_uint = 0x0004;
pub const ETNA_SUBMIT_SOFTPIN: c_uint = 0x0008;

pub const ETNA_PIPE_3D: c_uint = 0x00;
pub const ETNA_PIPE_2D: c_uint = 0x01;
pub const ETNA_PIPE_VG: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_submit {
    pub /: *mut *mut __u32 fence; / out,
    pub /: *mut *mut __u32 pipe; / in,
    pub /: *mut *mut __u32 exec_state; / in, initial execution state (ETNA_PIPE_x),
    pub /: *mut *mut __u32 nr_bos; / in, number of submit_bo's,
    pub /: *mut *mut __u32 nr_relocs; / in, number of submit_reloc's,
    pub /: *mut *mut __u32 stream_size; / in, cmdstream size,
    pub /: *mut *mut __u64 bos; / in, ptr to array of submit_bo's,
    pub /: *mut *mut __u64 relocs; / in, ptr to array of submit_reloc's,
    pub /: *mut *mut __u64 stream; / in, ptr to cmdstream,
    pub /: *mut *mut __u32 flags; / in, mask of ETNA_SUBMIT_x,
    pub /: *mut *mut __s32 fence_fd; / in/out, fence fd (see ETNA_SUBMIT_FENCE_FD_x),
    pub /: *mut *mut __u64 pmrs; / in, ptr to array of submit_pmr's,
    pub /: *mut *mut __u32 nr_pmrs; / in, number of submit_pmr's,
    pub pad: __u32,
}

// The normal way to synchronize with the GPU is just to CPU_PREP on
// a buffer if you need to access it from the CPU (other cmdstream
// submission from same or other contexts, PAGE_FLIP ioctl, etc, all
// handle the required synchronization under the hood).  This ioctl
// mainly just exists as a way to implement the gallium pipe_fence
// APIs without requiring a dummy bo to synchronize on.
//
pub const ETNA_WAIT_NONBLOCK: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_wait_fence {
    pub /: *mut *mut __u32 pipe; / in,
    pub /: *mut *mut __u32 fence; / in,
    pub /: *mut *mut __u32 flags; / in, mask of ETNA_WAIT_x,
    pub pad: __u32,
    pub /: *mut *mut drm_etnaviv_timespec timeout; / in,
}

pub const ETNA_USERPTR_READ: c_uint = 0x01;
pub const ETNA_USERPTR_WRITE: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_userptr {
    pub /: *mut *mut __u64 user_ptr; / in, page aligned user pointer,
    pub /: *mut *mut __u64 user_size; / in, page aligned user size,
    pub /: *mut *mut __u32 flags; / in, flags,
    pub /: *mut *mut __u32 handle; / out, non-zero handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_gem_wait {
    pub /: *mut *mut __u32 pipe; / in,
    pub /: *mut *mut __u32 handle; / in, bo to be waited for,
    pub /: *mut *mut __u32 flags; / in, mask of ETNA_WAIT_x,
    pub pad: __u32,
    pub /: *mut *mut drm_etnaviv_timespec timeout; / in,
}

//
// Performance Monitor (PM):
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_pm_domain {
    pub /: *mut *mut __u32 pipe; / in,
    pub /: *mut *mut __u8 iter; / in/out, select pm domain at index iter,
    pub /: *mut *mut __u8 id; / out, id of domain,
    pub /: *mut *mut __u16 nr_signals; / out, how many signals does this domain provide,
    pub /: *mut *mut char name[64]; / out, name of domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_etnaviv_pm_signal {
    pub /: *mut *mut __u32 pipe; / in,
    pub /: *mut *mut __u8 domain; / in, pm domain index,
    pub pad: __u8,
    pub /: *mut *mut __u16 iter; / in/out, select pm source at index iter,
    pub /: *mut *mut __u16 id; / out, id of signal,
    pub /: *mut *mut char name[64]; / out, name of domain,
}

pub const DRM_ETNAVIV_GET_PARAM: c_uint = 0x00;
// placeholder:
pub const DRM_ETNAVIV_SET_PARAM: c_uint = 0x01;
//
pub const DRM_ETNAVIV_GEM_NEW: c_uint = 0x02;
pub const DRM_ETNAVIV_GEM_INFO: c_uint = 0x03;
pub const DRM_ETNAVIV_GEM_CPU_PREP: c_uint = 0x04;
pub const DRM_ETNAVIV_GEM_CPU_FINI: c_uint = 0x05;
pub const DRM_ETNAVIV_GEM_SUBMIT: c_uint = 0x06;
pub const DRM_ETNAVIV_WAIT_FENCE: c_uint = 0x07;
pub const DRM_ETNAVIV_GEM_USERPTR: c_uint = 0x08;
pub const DRM_ETNAVIV_GEM_WAIT: c_uint = 0x09;
pub const DRM_ETNAVIV_PM_QUERY_DOM: c_uint = 0x0a;
pub const DRM_ETNAVIV_PM_QUERY_SIG: c_uint = 0x0b;
pub const DRM_ETNAVIV_NUM_IOCTLS: c_uint = 0x0c;

