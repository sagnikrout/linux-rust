//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/amdgpu_drm.h
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


// amdgpu_drm.h -- Public header for the amdgpu driver -*- linux-c -*-
//
// Copyright 2000 Precision Insight, Inc., Cedar Park, Texas.
// Copyright 2000 VA Linux Systems, Inc., Fremont, California.
// Copyright 2002 Tungsten Graphics, Inc., Cedar Park, Texas.
// Copyright 2014 Advanced Micro Devices, Inc.
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
// Authors:
// Kevin E. Martin <martin@valinux.com>
// Gareth Hughes <gareth@valinux.com>
// Keith Whitwell <keith@tungstengraphics.com>
//

pub const DRM_AMDGPU_GEM_CREATE: c_uint = 0x00;
pub const DRM_AMDGPU_GEM_MMAP: c_uint = 0x01;
pub const DRM_AMDGPU_CTX: c_uint = 0x02;
pub const DRM_AMDGPU_BO_LIST: c_uint = 0x03;
pub const DRM_AMDGPU_CS: c_uint = 0x04;
pub const DRM_AMDGPU_INFO: c_uint = 0x05;
pub const DRM_AMDGPU_GEM_METADATA: c_uint = 0x06;
pub const DRM_AMDGPU_GEM_WAIT_IDLE: c_uint = 0x07;
pub const DRM_AMDGPU_GEM_VA: c_uint = 0x08;
pub const DRM_AMDGPU_WAIT_CS: c_uint = 0x09;
pub const DRM_AMDGPU_GEM_OP: c_uint = 0x10;
pub const DRM_AMDGPU_GEM_USERPTR: c_uint = 0x11;
pub const DRM_AMDGPU_WAIT_FENCES: c_uint = 0x12;
pub const DRM_AMDGPU_VM: c_uint = 0x13;
pub const DRM_AMDGPU_FENCE_TO_HANDLE: c_uint = 0x14;
pub const DRM_AMDGPU_SCHED: c_uint = 0x15;
pub const DRM_AMDGPU_USERQ: c_uint = 0x16;
pub const DRM_AMDGPU_USERQ_SIGNAL: c_uint = 0x17;
pub const DRM_AMDGPU_USERQ_WAIT: c_uint = 0x18;
pub const DRM_AMDGPU_GEM_LIST_HANDLES: c_uint = 0x19;
pub const DRM_AMDGPU_PROC_OPTIONS: c_uint = 0x1A;

//
// DOC: memory domains
//
// %AMDGPU_GEM_DOMAIN_CPU	System memory that is not GPU accessible.
// Memory in this pool could be swapped out to disk if there is pressure.
//
// %AMDGPU_GEM_DOMAIN_GTT	GPU accessible system memory, mapped into the
// GPU's virtual address space via gart. Gart memory linearizes non-contiguous
// pages of system memory, allows GPU access system memory in a linearized
// fashion.
//
// %AMDGPU_GEM_DOMAIN_VRAM	Local video memory. For APUs, it is memory
// carved out by the BIOS.
//
// %AMDGPU_GEM_DOMAIN_GDS	Global on-chip data storage used to share data
// across shader threads.
//
// %AMDGPU_GEM_DOMAIN_GWS	Global wave sync, used to synchronize the
// execution of all the waves on a device.
//
// %AMDGPU_GEM_DOMAIN_OA	Ordered append, used by 3D or Compute engines
// for appending data.
//
// %AMDGPU_GEM_DOMAIN_DOORBELL	Doorbell. It is an MMIO region for
// signalling user mode queues.
//
pub const AMDGPU_GEM_DOMAIN_CPU: c_uint = 0x1;
pub const AMDGPU_GEM_DOMAIN_GTT: c_uint = 0x2;
pub const AMDGPU_GEM_DOMAIN_VRAM: c_uint = 0x4;
pub const AMDGPU_GEM_DOMAIN_GDS: c_uint = 0x8;
pub const AMDGPU_GEM_DOMAIN_GWS: c_uint = 0x10;
pub const AMDGPU_GEM_DOMAIN_OA: c_uint = 0x20;
pub const AMDGPU_GEM_DOMAIN_DOORBELL: c_uint = 0x40;

// Flag that CPU access will be required for the case of VRAM domain

// Flag that CPU access will not work, this VRAM domain is invisible

// Flag that USWC attributes should be used for GTT

// Flag that the memory should be in VRAM and cleared

// Flag that allocating the BO should use linear VRAM

// Flag that BO is always valid in this VM

// Flag that BO sharing will be explicitly synchronized

// Flag that indicates allocating MQD gart on GFX9, where the mtype
// for the second page onward should be set to NC. It should never
// be used by user space applications.
//

// Flag that BO may contain sensitive data that must be wiped before
// releasing the memory
//

// Flag that BO will be encrypted and that the TMZ bit should be
// set in the PTEs when mapping this buffer via GPUVM or
// accessing it with various hw blocks
//

// Flag that BO will be used only in preemptible context, which does
// not require GTT memory accounting
//

// Flag that BO can be discarded under memory pressure without keeping the
// content.
//

// Flag that BO is shared coherently between multiple devices or CPU threads.
// May depend on GPU instructions to flush caches to system scope explicitly.
//
// This influences the choice of MTYPE in the PTEs on GFXv9 and later GPUs and
// may override the MTYPE selected in AMDGPU_VA_OP_MAP.
//

// Flag that BO should not be cached by GPU. Coherent without having to flush
// GPU caches explicitly
//
// This influences the choice of MTYPE in the PTEs on GFXv9 and later GPUs and
// may override the MTYPE selected in AMDGPU_VA_OP_MAP.
//

// Flag that BO should be coherent across devices when using device-level
// atomics. May depend on GPU instructions to flush caches to device scope
// explicitly, promoting them to system scope automatically.
//
// This influences the choice of MTYPE in the PTEs on GFXv9 and later GPUs and
// may override the MTYPE selected in AMDGPU_VA_OP_MAP.
//

// Set PTE.D and recompress during GTT->VRAM moves according to TILING flags.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_create_in {
// the requested memory size
    pub bo_size: __u64,
// physical start_addr alignment in bytes for some HW requirements
    pub alignment: __u64,
// the requested memory domains
    pub domains: __u64,
// allocation flags
    pub domain_flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_create_out {
// returned GEM object handle
    pub handle: __u32,
    pub _pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_gem_create {
    pub in: drm_amdgpu_gem_create_in,
    pub out: drm_amdgpu_gem_create_out,
}

// Opcode to create new residency list.
pub const AMDGPU_BO_LIST_OP_CREATE: c_int = 0;
// Opcode to destroy previously created residency list
pub const AMDGPU_BO_LIST_OP_DESTROY: c_int = 1;
// Opcode to update resource information in the list
pub const AMDGPU_BO_LIST_OP_UPDATE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_bo_list_in {
// Type of operation
    pub operation: __u32,
// Handle of list or 0 if we want to create one
    pub list_handle: __u32,
// Number of BOs in list
    pub bo_number: __u32,
// Size of each element describing BO
    pub bo_info_size: __u32,
// Pointer to array describing BOs
    pub bo_info_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_bo_list_entry {
// Handle of BO
    pub bo_handle: __u32,
// New (if specified) BO priority to be used during migration
    pub bo_priority: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_bo_list_out {
// Handle of resource list
    pub list_handle: __u32,
    pub _pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_bo_list {
    pub in: drm_amdgpu_bo_list_in,
    pub out: drm_amdgpu_bo_list_out,
}

// context related
pub const AMDGPU_CTX_OP_ALLOC_CTX: c_int = 1;
pub const AMDGPU_CTX_OP_FREE_CTX: c_int = 2;
pub const AMDGPU_CTX_OP_QUERY_STATE: c_int = 3;
pub const AMDGPU_CTX_OP_QUERY_STATE2: c_int = 4;
pub const AMDGPU_CTX_OP_GET_STABLE_PSTATE: c_int = 5;
pub const AMDGPU_CTX_OP_SET_STABLE_PSTATE: c_int = 6;
// GPU reset status
pub const AMDGPU_CTX_NO_RESET: c_int = 0;
// this the context caused it
pub const AMDGPU_CTX_GUILTY_RESET: c_int = 1;
// some other context caused it
pub const AMDGPU_CTX_INNOCENT_RESET: c_int = 2;
// unknown cause
pub const AMDGPU_CTX_UNKNOWN_RESET: c_int = 3;
// indicate gpu reset occurred after ctx created

// indicate vram lost occurred after ctx created

// indicate some job from this context once cause gpu hang

// indicate some errors are detected by RAS

// indicate that the reset hasn't completed yet

// Context priority level

pub const AMDGPU_CTX_PRIORITY_NORMAL: c_int = 0;
//
// When used in struct drm_amdgpu_ctx_in, a priority above NORMAL requires
// CAP_SYS_NICE or DRM_MASTER
//
pub const AMDGPU_CTX_PRIORITY_HIGH: c_int = 512;
pub const AMDGPU_CTX_PRIORITY_VERY_HIGH: c_int = 1023;
// select a stable profiling pstate for perfmon tools
pub const AMDGPU_CTX_STABLE_PSTATE_FLAGS_MASK: c_uint = 0xf;
pub const AMDGPU_CTX_STABLE_PSTATE_NONE: c_int = 0;
pub const AMDGPU_CTX_STABLE_PSTATE_STANDARD: c_int = 1;
pub const AMDGPU_CTX_STABLE_PSTATE_MIN_SCLK: c_int = 2;
pub const AMDGPU_CTX_STABLE_PSTATE_MIN_MCLK: c_int = 3;
pub const AMDGPU_CTX_STABLE_PSTATE_PEAK: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_ctx_in {
// AMDGPU_CTX_OP_*
    pub op: __u32,
// Flags
    pub flags: __u32,
    pub ctx_id: __u32,
// AMDGPU_CTX_PRIORITY_*
    pub priority: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_ctx_out {
    pub ctx_id: __u32,
    pub _pad: __u32,
    pub alloc: },
// For future use, no flags defined so far
    pub flags: __u64,
// Number of resets caused by this context so far.
    pub hangs: __u32,
// Reset status since the last call of the ioctl.
    pub reset_status: __u32,
    pub state: },
    pub flags: __u32,
    pub _pad: __u32,
    pub pstate: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_ctx {
    pub in: drm_amdgpu_ctx_in,
    pub out: drm_amdgpu_ctx_out,
}

// user queue IOCTL operations
pub const AMDGPU_USERQ_OP_CREATE: c_int = 1;
pub const AMDGPU_USERQ_OP_FREE: c_int = 2;
// queue priority levels
// low < normal low < normal high < high
pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_MASK: c_uint = 0x3;
pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_SHIFT: c_int = 0;
pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_NORMAL_LOW: c_int = 0;
pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_LOW: c_int = 1;
pub const AMDGPU_USERQ_CREATE_FLAGS_QUEUE_PRIORITY_NORMAL_HIGH: c_int = 2;

// for queues that need access to protected content

//
// This structure is a container to pass input configuration
// info for all supported userqueue related operations.
// For operation AMDGPU_USERQ_OP_CREATE: user is expected
// to set all fields, excep the parameter 'queue_id'.
// For operation AMDGPU_USERQ_OP_FREE: the only input parameter expected
// to be set is 'queue_id', eveything else is ignored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_in {
// AMDGPU_USERQ_OP_*
    pub op: __u32,
// Queue id passed for operation USERQ_OP_FREE
    pub queue_id: __u32,
// the target GPU engine to execute workload (AMDGPU_HW_IP_*)
    pub ip_type: __u32,
//
// @doorbell_handle: the handle of doorbell GEM object
// associated with this userqueue client.
//
    pub doorbell_handle: __u32,
//
// @doorbell_offset: 32-bit offset of the doorbell in the doorbell bo.
// Kernel will generate absolute doorbell offset using doorbell_handle
// and doorbell_offset in the doorbell bo.
//
    pub doorbell_offset: __u32,
//
// @flags: flags used for queue parameters
//
    pub flags: __u32,
//
// @queue_va: Virtual address of the GPU memory which holds the queue
// object. The queue holds the workload packets.
//
    pub queue_va: __u64,
//
// @queue_size: Size of the queue in bytes, this needs to be 256-byte
// aligned.
//
    pub queue_size: __u64,
//
// @rptr_va : Virtual address of the GPU memory which holds the ring RPTR.
// This object must be at least 8 byte in size and aligned to 8-byte offset.
//
    pub rptr_va: __u64,
//
// @wptr_va : Virtual address of the GPU memory which holds the ring WPTR.
// This object must be at least 8 byte in size and aligned to 8-byte offset.
//
// Queue, RPTR and WPTR can come from the same object, as long as the size
// and alignment related requirements are met.
//
    pub wptr_va: __u64,
//
// @mqd: MQD (memory queue descriptor) is a set of parameters which allow
// the GPU to uniquely define and identify a usermode queue.
//
// MQD data can be of different size for different GPU IP/engine and
// their respective versions/revisions, so this points to a __u64
// which holds IP specific MQD of this usermode queue.
//
    pub mqd: __u64,
//
// @size: size of MQD data in bytes, it must match the MQD structure
// size of the respective engine/revision defined in UAPI for ex, for
// gfx11 workloads, size = sizeof(drm_amdgpu_userq_mqd_gfx11).
//
    pub mqd_size: __u64,
}

// The structure to carry output of userqueue ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_out {
//
// For operation AMDGPU_USERQ_OP_CREATE: This field contains a unique
// queue ID to represent the newly created userqueue in the system, otherwise
// it should be ignored.
//
    pub queue_id: __u32,
    pub _pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_userq {
    pub in: drm_amdgpu_userq_in,
    pub out: drm_amdgpu_userq_out,
}

// GFX V11 IP specific MQD parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_mqd_gfx11 {
//
// @shadow_va: Virtual address of the GPU memory to hold the shadow buffer.
// Use AMDGPU_INFO_IOCTL to find the exact size of the object.
//
    pub shadow_va: __u64,
//
// @csa_va: Virtual address of the GPU memory to hold the CSA buffer.
// Use AMDGPU_INFO_IOCTL to find the exact size of the object.
//
    pub csa_va: __u64,
}

// GFX V11 SDMA IP specific MQD parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_mqd_sdma_gfx11 {
//
// @csa_va: Virtual address of the GPU memory to hold the CSA buffer.
// This must be a from a separate GPU object, and use AMDGPU_INFO IOCTL
// to get the size.
//
    pub csa_va: __u64,
}

// GFX V11 Compute IP specific MQD parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_mqd_compute_gfx11 {
//
// @eop_va: Virtual address of the GPU memory to hold the EOP buffer.
// This must be a from a separate GPU object, and use AMDGPU_INFO IOCTL
// to get the size.
//
    pub eop_va: __u64,
}

// userq signal/wait ioctl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_signal {
//
// @queue_id: Queue handle used by the userq fence creation function
// to retrieve the WPTR.
//
    pub queue_id: __u32,
    pub pad: __u32,
//
// @syncobj_handles: The list of syncobj handles submitted by the user queue
// job to be signaled.
//
    pub syncobj_handles: __u64,
//
// @num_syncobj_handles: A count that represents the number of syncobj handles in
// @syncobj_handles.
//
    pub num_syncobj_handles: __u16,
    pub pad0: __u16,
    pub pad1: __u32,
//
// @bo_read_handles: The list of BO handles that the submitted user queue job
// is using for read only. This will update BO fences in the kernel.
//
    pub bo_read_handles: __u64,
//
// @bo_write_handles: The list of BO handles that the submitted user queue job
// is using for write only. This will update BO fences in the kernel.
//
    pub bo_write_handles: __u64,
//
// @num_bo_read_handles: A count that represents the number of read BO handles in
// @bo_read_handles.
//
    pub num_bo_read_handles: __u32,
//
// @num_bo_write_handles: A count that represents the number of write BO handles in
// @bo_write_handles.
//
    pub num_bo_write_handles: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_fence_info {
//
// @va: A gpu address allocated for each queue which stores the
// read pointer (RPTR) value.
//
    pub va: __u64,
//
// @value: A 64 bit value represents the write pointer (WPTR) of the
// queue commands which compared with the RPTR value to signal the
// fences.
//
    pub value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_userq_wait {
//
// @waitq_id: Queue handle used by the userq wait IOCTL to retrieve the
// wait queue and maintain the fence driver references in it.
//
    pub waitq_id: __u32,
    pub pad: __u32,
//
// @syncobj_handles: The list of syncobj handles submitted by the user queue
// job to get the va/value pairs.
//
    pub syncobj_handles: __u64,
//
// @syncobj_timeline_handles: The list of timeline syncobj handles submitted by
// the user queue job to get the va/value pairs at given @syncobj_timeline_points.
//
    pub syncobj_timeline_handles: __u64,
//
// @syncobj_timeline_points: The list of timeline syncobj points submitted by the
// user queue job for the corresponding @syncobj_timeline_handles.
//
    pub syncobj_timeline_points: __u64,
//
// @bo_read_handles: The list of read BO handles submitted by the user queue
// job to get the va/value pairs.
//
    pub bo_read_handles: __u64,
//
// @bo_write_handles: The list of write BO handles submitted by the user queue
// job to get the va/value pairs.
//
    pub bo_write_handles: __u64,
//
// @num_syncobj_timeline_handles: A count that represents the number of timeline
// syncobj handles in @syncobj_timeline_handles.
//
    pub num_syncobj_timeline_handles: __u16,
//
// @num_fences: This field can be used both as input and output. As input it defines
// the maximum number of fences that can be returned and as output it will specify
// how many fences were actually returned from the ioctl.
//
    pub num_fences: __u16,
//
// @num_syncobj_handles: A count that represents the number of syncobj handles in
// @syncobj_handles.
//
    pub num_syncobj_handles: __u16,
    pub pad0: __u16,
//
// @num_bo_read_handles: A count that represents the number of read BO handles in
// @bo_read_handles.
//
    pub num_bo_read_handles: __u32,
//
// @num_bo_write_handles: A count that represents the number of write BO handles in
// @bo_write_handles.
//
    pub num_bo_write_handles: __u32,
//
// @out_fences: The field is a return value from the ioctl containing the list of
// address/value pairs to wait for.
//
    pub out_fences: __u64,
}

// vm ioctl
pub const AMDGPU_VM_OP_RESERVE_VMID: c_int = 1;
pub const AMDGPU_VM_OP_UNRESERVE_VMID: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_vm_in {
// AMDGPU_VM_OP_*
    pub op: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_vm_out {
// For future use, no flags defined so far
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_vm {
    pub in: drm_amdgpu_vm_in,
    pub out: drm_amdgpu_vm_out,
}

// sched ioctl
pub const AMDGPU_SCHED_OP_PROCESS_PRIORITY_OVERRIDE: c_int = 1;
pub const AMDGPU_SCHED_OP_CONTEXT_PRIORITY_OVERRIDE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_sched_in {
// AMDGPU_SCHED_OP_*
    pub op: __u32,
    pub fd: __u32,
// AMDGPU_CTX_PRIORITY_*
    pub priority: __s32,
    pub ctx_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_sched {
    pub in: drm_amdgpu_sched_in,
}

//
// This is not a reliable API and you should expect it to fail for any
// number of reasons and have fallback path that do not use userptr to
// perform any operation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_userptr {
    pub addr: __u64,
    pub size: __u64,
// AMDGPU_GEM_USERPTR_*
    pub flags: __u32,
// Resulting GEM handle
    pub handle: __u32,
}

// SI-CI-VI:
// same meaning as the GB_TILE_MODE and GL_MACRO_TILE_MODE fields
pub const AMDGPU_TILING_ARRAY_MODE_SHIFT: c_int = 0;
pub const AMDGPU_TILING_ARRAY_MODE_MASK: c_uint = 0xf;
pub const AMDGPU_TILING_PIPE_CONFIG_SHIFT: c_int = 4;
pub const AMDGPU_TILING_PIPE_CONFIG_MASK: c_uint = 0x1f;
pub const AMDGPU_TILING_TILE_SPLIT_SHIFT: c_int = 9;
pub const AMDGPU_TILING_TILE_SPLIT_MASK: c_uint = 0x7;
pub const AMDGPU_TILING_MICRO_TILE_MODE_SHIFT: c_int = 12;
pub const AMDGPU_TILING_MICRO_TILE_MODE_MASK: c_uint = 0x7;
pub const AMDGPU_TILING_BANK_WIDTH_SHIFT: c_int = 15;
pub const AMDGPU_TILING_BANK_WIDTH_MASK: c_uint = 0x3;
pub const AMDGPU_TILING_BANK_HEIGHT_SHIFT: c_int = 17;
pub const AMDGPU_TILING_BANK_HEIGHT_MASK: c_uint = 0x3;
pub const AMDGPU_TILING_MACRO_TILE_ASPECT_SHIFT: c_int = 19;
pub const AMDGPU_TILING_MACRO_TILE_ASPECT_MASK: c_uint = 0x3;
pub const AMDGPU_TILING_NUM_BANKS_SHIFT: c_int = 21;
pub const AMDGPU_TILING_NUM_BANKS_MASK: c_uint = 0x3;
// GFX9 - GFX11:
pub const AMDGPU_TILING_SWIZZLE_MODE_SHIFT: c_int = 0;
pub const AMDGPU_TILING_SWIZZLE_MODE_MASK: c_uint = 0x1f;
pub const AMDGPU_TILING_DCC_OFFSET_256B_SHIFT: c_int = 5;
pub const AMDGPU_TILING_DCC_OFFSET_256B_MASK: c_uint = 0xFFFFFF;
pub const AMDGPU_TILING_DCC_PITCH_MAX_SHIFT: c_int = 29;
pub const AMDGPU_TILING_DCC_PITCH_MAX_MASK: c_uint = 0x3FFF;
pub const AMDGPU_TILING_DCC_INDEPENDENT_64B_SHIFT: c_int = 43;
pub const AMDGPU_TILING_DCC_INDEPENDENT_64B_MASK: c_uint = 0x1;
pub const AMDGPU_TILING_DCC_INDEPENDENT_128B_SHIFT: c_int = 44;
pub const AMDGPU_TILING_DCC_INDEPENDENT_128B_MASK: c_uint = 0x1;
pub const AMDGPU_TILING_SCANOUT_SHIFT: c_int = 63;
pub const AMDGPU_TILING_SCANOUT_MASK: c_uint = 0x1;
// GFX12 and later:
pub const AMDGPU_TILING_GFX12_SWIZZLE_MODE_SHIFT: c_int = 0;
pub const AMDGPU_TILING_GFX12_SWIZZLE_MODE_MASK: c_uint = 0x7;
// These are DCC recompression settings for memory management:
pub const AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_SHIFT: c_int = 3;
pub const AMDGPU_TILING_GFX12_DCC_MAX_COMPRESSED_BLOCK_MASK: c_uint = 0x3 /* 0:64B, 1:128B, 2:256B */;
pub const AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_SHIFT: c_int = 5;
pub const AMDGPU_TILING_GFX12_DCC_NUMBER_TYPE_MASK: c_uint = 0x7 /* CB_COLOR0_INFO.NUMBER_TYPE */;
pub const AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_SHIFT: c_int = 8;
pub const AMDGPU_TILING_GFX12_DCC_DATA_FORMAT_MASK: c_uint = 0x3f /* [0:4]:CB_COLOR0_INFO.FORMAT, [5]:MM */;
// When clearing the buffer or moving it from VRAM to GTT, don't compress and set DCC metadata
// to uncompressed. Set when parts of an allocation bypass DCC and read raw data.
pub const AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_SHIFT: c_int = 14;
pub const AMDGPU_TILING_GFX12_DCC_WRITE_COMPRESS_DISABLE_MASK: c_uint = 0x1;
// bit gap
pub const AMDGPU_TILING_GFX12_SCANOUT_SHIFT: c_int = 63;
pub const AMDGPU_TILING_GFX12_SCANOUT_MASK: c_uint = 0x1;
// Set/Get helpers for tiling flags.

pub const AMDGPU_GEM_METADATA_OP_SET_METADATA: c_int = 1;
pub const AMDGPU_GEM_METADATA_OP_GET_METADATA: c_int = 2;
// The same structure is shared for input/output
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_metadata {
// GEM Object handle
    pub handle: __u32,
// Do we want get or set metadata
    pub op: __u32,
// For future use, no flags defined so far
    pub flags: __u64,
// family specific tiling info
    pub tiling_info: __u64,
    pub data_size_bytes: __u32,
    pub data: [__u32; 64],
    pub data: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_mmap_in {
// the GEM object handle
    pub handle: __u32,
    pub _pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_mmap_out {
// mmap offset from the vma offset manager
    pub addr_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_gem_mmap {
    pub in: drm_amdgpu_gem_mmap_in,
    pub out: drm_amdgpu_gem_mmap_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_wait_idle_in {
// GEM object handle
    pub handle: __u32,
// For future use, no flags defined so far
    pub flags: __u32,
// Absolute timeout to wait
    pub timeout: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_wait_idle_out {
// BO status:  0 - BO is idle, 1 - BO is busy
    pub status: __u32,
// Returned current memory domain
    pub domain: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_gem_wait_idle {
    pub in: drm_amdgpu_gem_wait_idle_in,
    pub out: drm_amdgpu_gem_wait_idle_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_cs_in {
// Command submission handle
// handle equals 0 means none to wait for
// handle equals ~0ull means wait for the latest sequence number
//
    pub handle: __u64,
// Absolute timeout to wait
    pub timeout: __u64,
    pub ip_type: __u32,
    pub ip_instance: __u32,
    pub ring: __u32,
    pub ctx_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_cs_out {
// CS status:  0 - CS completed, 1 - CS still busy
    pub status: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_wait_cs {
    pub in: drm_amdgpu_wait_cs_in,
    pub out: drm_amdgpu_wait_cs_out,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_fence {
    pub ctx_id: __u32,
    pub ip_type: __u32,
    pub ip_instance: __u32,
    pub ring: __u32,
    pub seq_no: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_fences_in {
// This points to uint64_t * which points to fences
    pub fences: __u64,
    pub fence_count: __u32,
    pub wait_all: __u32,
    pub timeout_ns: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_wait_fences_out {
    pub status: __u32,
    pub first_signaled: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_wait_fences {
    pub in: drm_amdgpu_wait_fences_in,
    pub out: drm_amdgpu_wait_fences_out,
}

pub const AMDGPU_GEM_OP_GET_GEM_CREATE_INFO: c_int = 0;
pub const AMDGPU_GEM_OP_SET_PLACEMENT: c_int = 1;
pub const AMDGPU_GEM_OP_GET_MAPPING_INFO: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_vm_entry {
// Start of mapping (in bytes)
    pub addr: __u64,
// Size of mapping (in bytes)
    pub size: __u64,
// Mapping offset
    pub offset: __u64,
// flags needed to recreate mapping
    pub flags: __u64,
}

// Sets or returns a value associated with a buffer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_op {
// GEM object handle
    pub handle: __u32,
// AMDGPU_GEM_OP_*
    pub op: __u32,
// Input or return value. For MAPPING_INFO op: pointer to array of struct drm_amdgpu_gem_vm_entry
    pub value: __u64,
// For MAPPING_INFO op: number of mappings (in/out)
    pub num_entries: __u32,
    pub padding: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_list_handles {
// User pointer to array of drm_amdgpu_gem_bo_info_entry
    pub entries: __u64,
// Size of entries buffer / Number of handles in process (if larger than size of buffer, must retry)
    pub num_entries: __u32,
    pub padding: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_list_handles_entry {
// gem handle of buffer object
    pub gem_handle: __u32,
// Currently just one flag: IS_IMPORT
    pub flags: __u32,
// Size of bo
    pub size: __u64,
// Preferred domains for GEM_CREATE
    pub preferred_domains: __u64,
// GEM_CREATE flags for re-creation of buffer
    pub alloc_flags: __u64,
// physical start_addr alignment in bytes for some HW requirements
    pub alignment: __u64,
}

pub const AMDGPU_VA_OP_MAP: c_int = 1;
pub const AMDGPU_VA_OP_UNMAP: c_int = 2;
pub const AMDGPU_VA_OP_CLEAR: c_int = 3;
pub const AMDGPU_VA_OP_REPLACE: c_int = 4;
// Delay the page table update till the next CS

// Mapping flags
// readable mapping

// writable mapping

// executable mapping, new for VI

// unmapped page of partially resident textures

// MTYPE flags use bit 5 to 8

// Default MTYPE. Pre-AI must use this.  Recommended for newer ASICs.

// Use Non Coherent MTYPE instead of default MTYPE

// Use Write Combine MTYPE instead of default MTYPE

// Use Cache Coherent MTYPE instead of default MTYPE

// Use UnCached MTYPE instead of default MTYPE

// Use Read Write MTYPE instead of default MTYPE

// don't allocate MALL

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_gem_va {
// GEM object handle
    pub handle: __u32,
    pub _pad: __u32,
// AMDGPU_VA_OP_*
    pub operation: __u32,
// AMDGPU_VM_PAGE_*
    pub flags: __u32,
// va address to assign . Must be correctly aligned.
    pub va_address: __u64,
// Specify offset inside of BO to assign. Must be correctly aligned.
    pub offset_in_bo: __u64,
// Specify mapping size. Must be correctly aligned.
    pub map_size: __u64,
//
// vm_timeline_point is a sequence number used to add new timeline point.
//
    pub vm_timeline_point: __u64,
//
// The vm page table update fence is installed in given vm_timeline_syncobj_out
// at vm_timeline_point.
//
    pub vm_timeline_syncobj_out: __u32,
// the number of syncobj handles in @input_fence_syncobj_handles
    pub num_syncobj_handles: __u32,
// Array of sync object handle to wait for given input fences
    pub input_fence_syncobj_handles: __u64,
}

pub const AMDGPU_HW_IP_GFX: c_int = 0;
pub const AMDGPU_HW_IP_COMPUTE: c_int = 1;
pub const AMDGPU_HW_IP_DMA: c_int = 2;
pub const AMDGPU_HW_IP_UVD: c_int = 3;
pub const AMDGPU_HW_IP_VCE: c_int = 4;
pub const AMDGPU_HW_IP_UVD_ENC: c_int = 5;
pub const AMDGPU_HW_IP_VCN_DEC: c_int = 6;
//
// From VCN4, AMDGPU_HW_IP_VCN_ENC is re-used to support
// both encoding and decoding jobs.
//
pub const AMDGPU_HW_IP_VCN_ENC: c_int = 7;
pub const AMDGPU_HW_IP_VCN_JPEG: c_int = 8;
pub const AMDGPU_HW_IP_VPE: c_int = 9;
pub const AMDGPU_HW_IP_NUM: c_int = 10;
pub const AMDGPU_HW_IP_INSTANCE_MAX_COUNT: c_int = 1;
pub const AMDGPU_CHUNK_ID_IB: c_uint = 0x01;
pub const AMDGPU_CHUNK_ID_FENCE: c_uint = 0x02;
pub const AMDGPU_CHUNK_ID_DEPENDENCIES: c_uint = 0x03;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_IN: c_uint = 0x04;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_OUT: c_uint = 0x05;
pub const AMDGPU_CHUNK_ID_BO_HANDLES: c_uint = 0x06;
pub const AMDGPU_CHUNK_ID_SCHEDULED_DEPENDENCIES: c_uint = 0x07;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_WAIT: c_uint = 0x08;
pub const AMDGPU_CHUNK_ID_SYNCOBJ_TIMELINE_SIGNAL: c_uint = 0x09;
pub const AMDGPU_CHUNK_ID_CP_GFX_SHADOW: c_uint = 0x0a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk {
    pub chunk_id: __u32,
    pub length_dw: __u32,
    pub chunk_data: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_in {
// Rendering context id
    pub ctx_id: __u32,
// Handle of resource list associated with CS
    pub bo_list_handle: __u32,
    pub num_chunks: __u32,
    pub flags: __u32,
// this points to __u64 * which point to cs chunks
    pub chunks: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_out {
    pub handle: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_cs {
    pub in: drm_amdgpu_cs_in,
    pub out: drm_amdgpu_cs_out,
}

// Specify flags to be used for IB
// This IB should be submitted to CE

// Preamble flag, which means the IB could be dropped if no context switch

// Preempt flag, IB should set Pre_enb bit if PREEMPT flag detected

// The IB fence should do the L2 writeback but not invalidate any shader
// caches (L2/vL1/sL1/I$).

// Set GDS_COMPUTE_MAX_WAVE_ID = DEFAULT before PACKET3_INDIRECT_BUFFER.
// This will reset wave ID counters for the IB.
//

// Flag the IB as secure (TMZ)
//

// Tell KMD to flush and invalidate caches
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_ib {
    pub _pad: __u32,
// AMDGPU_IB_FLAG_*
    pub flags: __u32,
// Virtual address to begin IB execution
    pub va_start: __u64,
// Size of submission
    pub ib_bytes: __u32,
// HW IP to submit to
    pub ip_type: __u32,
// HW IP index of the same type to submit to
    pub ip_instance: __u32,
// Ring index to submit to
    pub ring: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_dep {
    pub ip_type: __u32,
    pub ip_instance: __u32,
    pub ring: __u32,
    pub ctx_id: __u32,
    pub handle: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_fence {
    pub handle: __u32,
    pub offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_sem {
    pub handle: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_syncobj {
    pub handle: __u32,
    pub flags: __u32,
    pub point: __u64,
}

pub const AMDGPU_FENCE_TO_HANDLE_GET_SYNCOBJ: c_int = 0;
pub const AMDGPU_FENCE_TO_HANDLE_GET_SYNCOBJ_FD: c_int = 1;
pub const AMDGPU_FENCE_TO_HANDLE_GET_SYNC_FILE_FD: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_amdgpu_fence_to_handle {
    pub fence: drm_amdgpu_fence,
    pub what: __u32,
    pub pad: __u32,
    pub in: },
    pub handle: __u32,
    pub out: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_data {
    pub ib_data: drm_amdgpu_cs_chunk_ib,
    pub fence_data: drm_amdgpu_cs_chunk_fence,
}

pub const AMDGPU_CS_CHUNK_CP_GFX_SHADOW_FLAGS_INIT_SHADOW: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_cs_chunk_cp_gfx_shadow {
    pub shadow_va: __u64,
    pub csa_va: __u64,
    pub gds_va: __u64,
    pub flags: __u64,
}

//
// Query h/w info: Flag that this is integrated (a.h.a. fusion) GPU
//
pub const AMDGPU_IDS_FLAGS_FUSION: c_uint = 0x01;
pub const AMDGPU_IDS_FLAGS_PREEMPTION: c_uint = 0x02;
pub const AMDGPU_IDS_FLAGS_TMZ: c_uint = 0x04;
pub const AMDGPU_IDS_FLAGS_CONFORMANT_TRUNC_COORD: c_uint = 0x08;
pub const AMDGPU_IDS_FLAGS_GANG_SUBMIT: c_uint = 0x10;
//
// Query h/w info: Flag identifying VF/PF/PT mode
//
pub const AMDGPU_IDS_FLAGS_MODE_MASK: c_uint = 0x300;
pub const AMDGPU_IDS_FLAGS_MODE_SHIFT: c_uint = 0x8;
pub const AMDGPU_IDS_FLAGS_MODE_PF: c_uint = 0x0;
pub const AMDGPU_IDS_FLAGS_MODE_VF: c_uint = 0x1;
pub const AMDGPU_IDS_FLAGS_MODE_PT: c_uint = 0x2;
// indicate if acceleration can be working
pub const AMDGPU_INFO_ACCEL_WORKING: c_uint = 0x00;
// get the crtc_id from the mode object id?
pub const AMDGPU_INFO_CRTC_FROM_ID: c_uint = 0x01;
// query hw IP info
pub const AMDGPU_INFO_HW_IP_INFO: c_uint = 0x02;
// query hw IP instance count for the specified type
pub const AMDGPU_INFO_HW_IP_COUNT: c_uint = 0x03;
// timestamp for GL_ARB_timer_query
pub const AMDGPU_INFO_TIMESTAMP: c_uint = 0x05;
// Query the firmware version
pub const AMDGPU_INFO_FW_VERSION: c_uint = 0x0e;
// Subquery id: Query VCE firmware version
pub const AMDGPU_INFO_FW_VCE: c_uint = 0x1;
// Subquery id: Query UVD firmware version
pub const AMDGPU_INFO_FW_UVD: c_uint = 0x2;
// Subquery id: Query GMC firmware version
pub const AMDGPU_INFO_FW_GMC: c_uint = 0x03;
// Subquery id: Query GFX ME firmware version
pub const AMDGPU_INFO_FW_GFX_ME: c_uint = 0x04;
// Subquery id: Query GFX PFP firmware version
pub const AMDGPU_INFO_FW_GFX_PFP: c_uint = 0x05;
// Subquery id: Query GFX CE firmware version
pub const AMDGPU_INFO_FW_GFX_CE: c_uint = 0x06;
// Subquery id: Query GFX RLC firmware version
pub const AMDGPU_INFO_FW_GFX_RLC: c_uint = 0x07;
// Subquery id: Query GFX MEC firmware version
pub const AMDGPU_INFO_FW_GFX_MEC: c_uint = 0x08;
// Subquery id: Query SMC firmware version
pub const AMDGPU_INFO_FW_SMC: c_uint = 0x0a;
// Subquery id: Query SDMA firmware version
pub const AMDGPU_INFO_FW_SDMA: c_uint = 0x0b;
// Subquery id: Query PSP SOS firmware version
pub const AMDGPU_INFO_FW_SOS: c_uint = 0x0c;
// Subquery id: Query PSP ASD firmware version
pub const AMDGPU_INFO_FW_ASD: c_uint = 0x0d;
// Subquery id: Query VCN firmware version
pub const AMDGPU_INFO_FW_VCN: c_uint = 0x0e;
// Subquery id: Query GFX RLC SRLC firmware version
pub const AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_CNTL: c_uint = 0x0f;
// Subquery id: Query GFX RLC SRLG firmware version
pub const AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_GPM_MEM: c_uint = 0x10;
// Subquery id: Query GFX RLC SRLS firmware version
pub const AMDGPU_INFO_FW_GFX_RLC_RESTORE_LIST_SRM_MEM: c_uint = 0x11;
// Subquery id: Query DMCU firmware version
pub const AMDGPU_INFO_FW_DMCU: c_uint = 0x12;
pub const AMDGPU_INFO_FW_TA: c_uint = 0x13;
// Subquery id: Query DMCUB firmware version
pub const AMDGPU_INFO_FW_DMCUB: c_uint = 0x14;
// Subquery id: Query TOC firmware version
pub const AMDGPU_INFO_FW_TOC: c_uint = 0x15;
// Subquery id: Query CAP firmware version
pub const AMDGPU_INFO_FW_CAP: c_uint = 0x16;
// Subquery id: Query GFX RLCP firmware version
pub const AMDGPU_INFO_FW_GFX_RLCP: c_uint = 0x17;
// Subquery id: Query GFX RLCV firmware version
pub const AMDGPU_INFO_FW_GFX_RLCV: c_uint = 0x18;
// Subquery id: Query MES_KIQ firmware version
pub const AMDGPU_INFO_FW_MES_KIQ: c_uint = 0x19;
// Subquery id: Query MES firmware version
pub const AMDGPU_INFO_FW_MES: c_uint = 0x1a;
// Subquery id: Query IMU firmware version
pub const AMDGPU_INFO_FW_IMU: c_uint = 0x1b;
// Subquery id: Query VPE firmware version
pub const AMDGPU_INFO_FW_VPE: c_uint = 0x1c;
// number of bytes moved for TTM migration
pub const AMDGPU_INFO_NUM_BYTES_MOVED: c_uint = 0x0f;
// the used VRAM size
pub const AMDGPU_INFO_VRAM_USAGE: c_uint = 0x10;
// the used GTT size
pub const AMDGPU_INFO_GTT_USAGE: c_uint = 0x11;
// Information about GDS, etc. resource configuration
pub const AMDGPU_INFO_GDS_CONFIG: c_uint = 0x13;
// Query information about VRAM and GTT domains
pub const AMDGPU_INFO_VRAM_GTT: c_uint = 0x14;
// Query information about register in MMR address space
pub const AMDGPU_INFO_READ_MMR_REG: c_uint = 0x15;
// Query information about device: rev id, family, etc.
pub const AMDGPU_INFO_DEV_INFO: c_uint = 0x16;
// visible vram usage
pub const AMDGPU_INFO_VIS_VRAM_USAGE: c_uint = 0x17;
// number of TTM buffer evictions
pub const AMDGPU_INFO_NUM_EVICTIONS: c_uint = 0x18;
// Query memory about VRAM and GTT domains
pub const AMDGPU_INFO_MEMORY: c_uint = 0x19;
// Query vce clock table
pub const AMDGPU_INFO_VCE_CLOCK_TABLE: c_uint = 0x1A;
// Query vbios related information
pub const AMDGPU_INFO_VBIOS: c_uint = 0x1B;
// Subquery id: Query vbios size
pub const AMDGPU_INFO_VBIOS_SIZE: c_uint = 0x1;
// Subquery id: Query vbios image
pub const AMDGPU_INFO_VBIOS_IMAGE: c_uint = 0x2;
// Subquery id: Query vbios info
pub const AMDGPU_INFO_VBIOS_INFO: c_uint = 0x3;
// Query UVD handles
pub const AMDGPU_INFO_NUM_HANDLES: c_uint = 0x1C;
// Query sensor related information
pub const AMDGPU_INFO_SENSOR: c_uint = 0x1D;
// Subquery id: Query GPU shader clock
pub const AMDGPU_INFO_SENSOR_GFX_SCLK: c_uint = 0x1;
// Subquery id: Query GPU memory clock
pub const AMDGPU_INFO_SENSOR_GFX_MCLK: c_uint = 0x2;
// Subquery id: Query GPU temperature
pub const AMDGPU_INFO_SENSOR_GPU_TEMP: c_uint = 0x3;
// Subquery id: Query GPU load
pub const AMDGPU_INFO_SENSOR_GPU_LOAD: c_uint = 0x4;
// Subquery id: Query average GPU power
pub const AMDGPU_INFO_SENSOR_GPU_AVG_POWER: c_uint = 0x5;
// Subquery id: Query northbridge voltage
pub const AMDGPU_INFO_SENSOR_VDDNB: c_uint = 0x6;
// Subquery id: Query graphics voltage
pub const AMDGPU_INFO_SENSOR_VDDGFX: c_uint = 0x7;
// Subquery id: Query GPU stable pstate shader clock
pub const AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_SCLK: c_uint = 0x8;
// Subquery id: Query GPU stable pstate memory clock
pub const AMDGPU_INFO_SENSOR_STABLE_PSTATE_GFX_MCLK: c_uint = 0x9;
// Subquery id: Query GPU peak pstate shader clock
pub const AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_SCLK: c_uint = 0xa;
// Subquery id: Query GPU peak pstate memory clock
pub const AMDGPU_INFO_SENSOR_PEAK_PSTATE_GFX_MCLK: c_uint = 0xb;
// Subquery id: Query input GPU power
pub const AMDGPU_INFO_SENSOR_GPU_INPUT_POWER: c_uint = 0xc;
// Number of VRAM page faults on CPU access.
pub const AMDGPU_INFO_NUM_VRAM_CPU_PAGE_FAULTS: c_uint = 0x1E;
pub const AMDGPU_INFO_VRAM_LOST_COUNTER: c_uint = 0x1F;
// query ras mask of enabled features
pub const AMDGPU_INFO_RAS_ENABLED_FEATURES: c_uint = 0x20;
// RAS MASK: UMC (VRAM)

// RAS MASK: SDMA

// RAS MASK: GFX

// RAS MASK: MMHUB

// RAS MASK: ATHUB

// RAS MASK: PCIE

// RAS MASK: HDP

// RAS MASK: XGMI

// RAS MASK: DF

// RAS MASK: SMN

// RAS MASK: SEM

// RAS MASK: MP0

// RAS MASK: MP1

// RAS MASK: FUSE

// query video encode/decode caps
pub const AMDGPU_INFO_VIDEO_CAPS: c_uint = 0x21;
// Subquery id: Decode
pub const AMDGPU_INFO_VIDEO_CAPS_DECODE: c_int = 0;
// Subquery id: Encode
pub const AMDGPU_INFO_VIDEO_CAPS_ENCODE: c_int = 1;
// Query the max number of IBs per gang per submission
pub const AMDGPU_INFO_MAX_IBS: c_uint = 0x22;
// query last page fault info
pub const AMDGPU_INFO_GPUVM_FAULT: c_uint = 0x23;
// query FW object size and alignment
pub const AMDGPU_INFO_UQ_FW_AREAS: c_uint = 0x24;
pub const AMDGPU_INFO_MMR_SE_INDEX_SHIFT: c_int = 0;
pub const AMDGPU_INFO_MMR_SE_INDEX_MASK: c_uint = 0xff;
pub const AMDGPU_INFO_MMR_SH_INDEX_SHIFT: c_int = 8;
pub const AMDGPU_INFO_MMR_SH_INDEX_MASK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_query_fw {
// AMDGPU_INFO_FW_*
    pub fw_type: __u32,
//
// Index of the IP if there are more IPs of
// the same type.
//
    pub ip_instance: __u32,
//
// Index of the engine. Whether this is used depends
// on the firmware type. (e.g. MEC, SDMA)
//
    pub index: __u32,
    pub _pad: __u32,
}

// Input structure for the INFO ioctl
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info {
// Where the return value will be stored
    pub return_pointer: __u64,
// The size of the return value. Just like "size" in "snprintf",
// it limits how many bytes the kernel can write.
    pub return_size: __u32,
// The query request id.
    pub query: __u32,
    pub id: __u32,
    pub _pad: __u32,
    pub mode_crtc: },
// AMDGPU_HW_IP_*
    pub type: __u32,
//
// Index of the IP if there are more IPs of the same
// type. Ignored by AMDGPU_INFO_HW_IP_COUNT.
//
    pub ip_instance: __u32,
    pub query_hw_ip: },
    pub dword_offset: __u32,
// number of registers to read
    pub count: __u32,
    pub instance: __u32,
// For future use, no flags defined so far
    pub flags: __u32,
    pub read_mmr_reg: },
    pub query_fw: drm_amdgpu_query_fw,
    pub type: __u32,
    pub offset: __u32,
    pub vbios_info: },
    pub type: __u32,
    pub sensor_info: },
    pub type: __u32,
    pub video_cap: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_gds {
// GDS GFX partition size
    pub gds_gfx_partition_size: __u32,
// GDS compute partition size
    pub compute_partition_size: __u32,
// total GDS memory size
    pub gds_total_size: __u32,
// GWS size per GFX partition
    pub gws_per_gfx_partition: __u32,
// GSW size per compute partition
    pub gws_per_compute_partition: __u32,
// OA size per GFX partition
    pub oa_per_gfx_partition: __u32,
// OA size per compute partition
    pub oa_per_compute_partition: __u32,
    pub _pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_vram_gtt {
    pub vram_size: __u64,
    pub vram_cpu_accessible_size: __u64,
    pub gtt_size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_heap_info {
// max. physical memory
    pub total_heap_size: __u64,
// Theoretical max. available memory in the given heap
    pub usable_heap_size: __u64,
//
// Number of bytes allocated in the heap. This includes all processes
// and private allocations in the kernel. It changes when new buffers
// are allocated, freed, and moved. It cannot be larger than
// heap_size.
//
    pub heap_usage: __u64,
//
// Theoretical possible max. size of buffer which
// could be allocated in the given heap
//
    pub max_allocation: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_memory_info {
    pub vram: drm_amdgpu_heap_info,
    pub cpu_accessible_vram: drm_amdgpu_heap_info,
    pub gtt: drm_amdgpu_heap_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_firmware {
    pub ver: __u32,
    pub feature: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_vbios {
    pub name: [__u8; 64],
    pub vbios_pn: [__u8; 64],
    pub version: __u32,
    pub pad: __u32,
    pub vbios_ver_str: [__u8; 32],
    pub date: [__u8; 32],
}

pub const AMDGPU_VRAM_TYPE_UNKNOWN: c_int = 0;
pub const AMDGPU_VRAM_TYPE_GDDR1: c_int = 1;
pub const AMDGPU_VRAM_TYPE_DDR2: c_int = 2;
pub const AMDGPU_VRAM_TYPE_GDDR3: c_int = 3;
pub const AMDGPU_VRAM_TYPE_GDDR4: c_int = 4;
pub const AMDGPU_VRAM_TYPE_GDDR5: c_int = 5;
pub const AMDGPU_VRAM_TYPE_HBM: c_int = 6;
pub const AMDGPU_VRAM_TYPE_DDR3: c_int = 7;
pub const AMDGPU_VRAM_TYPE_DDR4: c_int = 8;
pub const AMDGPU_VRAM_TYPE_GDDR6: c_int = 9;
pub const AMDGPU_VRAM_TYPE_DDR5: c_int = 10;
pub const AMDGPU_VRAM_TYPE_LPDDR4: c_int = 11;
pub const AMDGPU_VRAM_TYPE_LPDDR5: c_int = 12;
pub const AMDGPU_VRAM_TYPE_HBM3E: c_int = 13;
pub const AMDGPU_VRAM_TYPE_HBM4: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_device {
// PCI Device ID
    pub device_id: __u32,
// Internal chip revision: A0, A1, etc.)
    pub chip_rev: __u32,
    pub external_rev: __u32,
// Revision id in PCI Config space
    pub pci_rev: __u32,
    pub family: __u32,
    pub num_shader_engines: __u32,
    pub num_shader_arrays_per_engine: __u32,
// in KHz
    pub gpu_counter_freq: __u32,
    pub max_engine_clock: __u64,
    pub max_memory_clock: __u64,
// cu information
    pub cu_active_number: __u32,
// NOTE: cu_ao_mask is INVALID, DON'T use it
    pub cu_ao_mask: __u32,
    pub cu_bitmap: [__u32; 4][4],
// Render backend pipe mask. One render backend is CB+DB.
    pub enabled_rb_pipes_mask: __u32,
    pub num_rb_pipes: __u32,
    pub num_hw_gfx_contexts: __u32,
// PCIe version (the smaller of the GPU and the CPU/motherboard)
    pub pcie_gen: __u32,
    pub ids_flags: __u64,
// Starting virtual address for UMDs.
    pub virtual_address_offset: __u64,
// The maximum virtual address
    pub virtual_address_max: __u64,
// Required alignment of virtual addresses.
    pub virtual_address_alignment: __u32,
// Page table entry - fragment size
    pub pte_fragment_size: __u32,
    pub gart_page_size: __u32,
// constant engine ram size
    pub ce_ram_size: __u32,
// video memory type info
    pub vram_type: __u32,
// video memory bit width
    pub vram_bit_width: __u32,
// vce harvesting instance
    pub vce_harvest_config: __u32,
// gfx double offchip LDS buffers
    pub gc_double_offchip_lds_buf: __u32,
// NGG Primitive Buffer
    pub prim_buf_gpu_addr: __u64,
// NGG Position Buffer
    pub pos_buf_gpu_addr: __u64,
// NGG Control Sideband
    pub cntl_sb_buf_gpu_addr: __u64,
// NGG Parameter Cache
    pub param_buf_gpu_addr: __u64,
    pub prim_buf_size: __u32,
    pub pos_buf_size: __u32,
    pub cntl_sb_buf_size: __u32,
    pub param_buf_size: __u32,
// wavefront size
    pub wave_front_size: __u32,
// shader visible vgprs
    pub num_shader_visible_vgprs: __u32,
// CU per shader array
    pub num_cu_per_sh: __u32,
// number of tcc blocks
    pub num_tcc_blocks: __u32,
// gs vgt table depth
    pub gs_vgt_table_depth: __u32,
// gs primitive buffer depth
    pub gs_prim_buffer_depth: __u32,
// max gs wavefront per vgt
    pub max_gs_waves_per_vgt: __u32,
// PCIe number of lanes (the smaller of the GPU and the CPU/motherboard)
    pub pcie_num_lanes: __u32,
// always on cu bitmap
    pub cu_ao_bitmap: [__u32; 4][4],
// Starting high virtual address for UMDs.
    pub high_va_offset: __u64,
// The maximum high virtual address
    pub high_va_max: __u64,
// gfx10 pa_sc_tile_steering_override
    pub pa_sc_tile_steering_override: __u32,
    pub pad: __u32,
// disabled TCCs
    pub tcc_disabled_mask: __u64,
    pub min_engine_clock: __u64,
    pub min_memory_clock: __u64,
// The following fields are only set on gfx11+, older chips set 0.
    pub /: *mut *mut __u32 tcp_cache_size; / AKA GL0, VMEM cache,
    pub num_sqc_per_wgp: __u32,
    pub /: *mut *mut __u32 sqc_data_cache_size; / AKA SMEM cache,
    pub sqc_inst_cache_size: __u32,
    pub gl1c_cache_size: __u32,
    pub gl2c_cache_size: __u32,
    pub /: *mut *mut __u64 mall_size; / AKA infinity cache,
// high 32 bits of the rb pipes mask
    pub enabled_rb_pipes_mask_hi: __u32,
// shadow area size for gfx11
    pub shadow_size: __u32,
// shadow area base virtual alignment for gfx11
    pub shadow_alignment: __u32,
// context save area size for gfx11
    pub csa_size: __u32,
// context save area base virtual alignment for gfx11
    pub csa_alignment: __u32,
// Userq IP mask (1 << AMDGPU_HW_IP_*)
    pub userq_ip_mask: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_hw_ip {
// Version of h/w IP
    pub hw_ip_version_major: __u32,
    pub hw_ip_version_minor: __u32,
// Capabilities
    pub capabilities_flags: __u64,
// command buffer address start alignment
    pub ib_start_alignment: __u32,
// command buffer size alignment
    pub ib_size_alignment: __u32,
// Bitmask of available rings. Bit 0 means ring 0, etc.
    pub available_rings: __u32,
// version info: bits 23:16 major, 15:8 minor, 7:0 revision
    pub ip_discovery_version: __u32,
// Userq available slots
    pub userq_num_slots: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_num_handles {
// Max handles as supported by firmware for UVD
    pub uvd_max_handles: __u32,
// Handles currently in use for UVD
    pub uvd_used_handles: __u32,
}

pub const AMDGPU_VCE_CLOCK_TABLE_ENTRIES: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_vce_clock_table_entry {
// System clock
    pub sclk: __u32,
// Memory clock
    pub mclk: __u32,
// VCE clock
    pub eclk: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_vce_clock_table {
    pub entries: [drm_amdgpu_info_vce_clock_table_entry; AMDGPU_VCE_CLOCK_TABLE_ENTRIES],
    pub num_valid_entries: __u32,
    pub pad: __u32,
}

// query video encode/decode caps
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG2: c_int = 0;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4: c_int = 1;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VC1: c_int = 2;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_MPEG4_AVC: c_int = 3;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_HEVC: c_int = 4;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_JPEG: c_int = 5;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_VP9: c_int = 6;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_AV1: c_int = 7;
pub const AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_video_codec_info {
    pub valid: __u32,
    pub max_width: __u32,
    pub max_height: __u32,
    pub max_pixels_per_frame: __u32,
    pub max_level: __u32,
    pub pad: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_video_caps {
    pub codec_info: [drm_amdgpu_info_video_codec_info; AMDGPU_INFO_VIDEO_CAPS_CODEC_IDX_COUNT],
}

pub const AMDGPU_VMHUB_TYPE_MASK: c_uint = 0xff;
pub const AMDGPU_VMHUB_TYPE_SHIFT: c_int = 0;
pub const AMDGPU_VMHUB_TYPE_GFX: c_int = 0;
pub const AMDGPU_VMHUB_TYPE_MM0: c_int = 1;
pub const AMDGPU_VMHUB_TYPE_MM1: c_int = 2;
pub const AMDGPU_VMHUB_IDX_MASK: c_uint = 0xff00;
pub const AMDGPU_VMHUB_IDX_SHIFT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_gpuvm_fault {
    pub addr: __u64,
    pub status: __u32,
    pub vmhub: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_uq_metadata_gfx {
// shadow area size for gfx11
    pub shadow_size: __u32,
// shadow area base virtual alignment for gfx11
    pub shadow_alignment: __u32,
// context save area size for gfx11
    pub csa_size: __u32,
// context save area base virtual alignment for gfx11
    pub csa_alignment: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_uq_metadata_compute {
// EOP size for gfx11
    pub eop_size: __u32,
// EOP base virtual alignment for gfx11
    pub eop_alignment: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_uq_metadata_sdma {
// context save area size for sdma6
    pub csa_size: __u32,
// context save area base virtual alignment for sdma6
    pub csa_alignment: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_info_uq_metadata {
    pub gfx: drm_amdgpu_info_uq_metadata_gfx,
    pub compute: drm_amdgpu_info_uq_metadata_compute,
    pub sdma: drm_amdgpu_info_uq_metadata_sdma,
}

//
// Supported GPU families
//
pub const AMDGPU_FAMILY_UNKNOWN: c_int = 0;

//
// Definition of user options
//
// option: AMDGPU_PROC_OPTIONS_OP_KFD_SIGBUS_DELAY
// 0:          Disable sigbus delay - SIGBUS will be raised immediately
// 0xFFFFFFFF: SIGBUS will not be raised
// other:      Set the sigbus delay in milliseconds
//
pub const AMDGPU_PROC_OPTIONS_OP_KFD_SIGBUS_DELAY: c_int = 0;
pub const AMDGPU_PROC_OPTIONS_KFD_SIGBUS_DELAY_DISABLED: c_uint = 0xFFFFFFFFu;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_amdgpu_proc_options {
    pub op: __u32,
    pub value: __u32,
    pub kfd_sigbus_delay: },
}

