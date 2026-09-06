//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/tegra_drm.h
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
// Copyright (c) 2012-2020 NVIDIA Corporation

// Tegra DRM legacy UAPI. Only enabled with STAGING

//
// struct drm_tegra_gem_create - parameters for the GEM object creation IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_gem_create {
//
// @size:
//
// The size, in bytes, of the buffer object to be created.
//
    pub size: __u64,
//
// @flags:
//
// A bitmask of flags that influence the creation of GEM objects:
//
// DRM_TEGRA_GEM_CREATE_TILED
// Use the 16x16 tiling format for this buffer.
//
// DRM_TEGRA_GEM_CREATE_BOTTOM_UP
// The buffer has a bottom-up layout.
//
    pub flags: __u32,
//
// @handle:
//
// The handle of the created GEM object. Set by the kernel upon
// successful completion of the IOCTL.
//
    pub handle: __u32,
}

//
// struct drm_tegra_gem_mmap - parameters for the GEM mmap IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_gem_mmap {
//
// @handle:
//
// Handle of the GEM object to obtain an mmap offset for.
//
    pub handle: __u32,
//
// @pad:
//
// Structure padding that may be used in the future. Must be 0.
//
    pub pad: __u32,
//
// @offset:
//
// The mmap offset for the given GEM object. Set by the kernel upon
// successful completion of the IOCTL.
//
    pub offset: __u64,
}

//
// struct drm_tegra_syncpt_read - parameters for the read syncpoint IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_syncpt_read {
//
// @id:
//
// ID of the syncpoint to read the current value from.
//
    pub id: __u32,
//
// @value:
//
// The current syncpoint value. Set by the kernel upon successful
// completion of the IOCTL.
//
    pub value: __u32,
}

//
// struct drm_tegra_syncpt_incr - parameters for the increment syncpoint IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_syncpt_incr {
//
// @id:
//
// ID of the syncpoint to increment.
//
    pub id: __u32,
//
// @pad:
//
// Structure padding that may be used in the future. Must be 0.
//
    pub pad: __u32,
}

//
// struct drm_tegra_syncpt_wait - parameters for the wait syncpoint IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_syncpt_wait {
//
// @id:
//
// ID of the syncpoint to wait on.
//
    pub id: __u32,
//
// @thresh:
//
// Threshold value for which to wait.
//
    pub thresh: __u32,
//
// @timeout:
//
// Timeout, in milliseconds, to wait.
//
    pub timeout: __u32,
//
// @value:
//
// The new syncpoint value after the wait. Set by the kernel upon
// successful completion of the IOCTL.
//
    pub value: __u32,
}

//
// struct drm_tegra_open_channel - parameters for the open channel IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_open_channel {
//
// @client:
//
// The client ID for this channel.
//
    pub client: __u32,
//
// @pad:
//
// Structure padding that may be used in the future. Must be 0.
//
    pub pad: __u32,
//
// @context:
//
// The application context of this channel. Set by the kernel upon
// successful completion of the IOCTL. This context needs to be passed
// to the DRM_TEGRA_CHANNEL_CLOSE or the DRM_TEGRA_SUBMIT IOCTLs.
//
    pub context: __u64,
}

//
// struct drm_tegra_close_channel - parameters for the close channel IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_close_channel {
//
// @context:
//
// The application context of this channel. This is obtained from the
// DRM_TEGRA_OPEN_CHANNEL IOCTL.
//
    pub context: __u64,
}

//
// struct drm_tegra_get_syncpt - parameters for the get syncpoint IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_get_syncpt {
//
// @context:
//
// The application context identifying the channel for which to obtain
// the syncpoint ID.
//
    pub context: __u64,
//
// @index:
//
// Index of the client syncpoint for which to obtain the ID.
//
    pub index: __u32,
//
// @id:
//
// The ID of the given syncpoint. Set by the kernel upon successful
// completion of the IOCTL.
//
    pub id: __u32,
}

//
// struct drm_tegra_get_syncpt_base - parameters for the get wait base IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_get_syncpt_base {
//
// @context:
//
// The application context identifying for which channel to obtain the
// wait base.
//
    pub context: __u64,
//
// @syncpt:
//
// ID of the syncpoint for which to obtain the wait base.
//
    pub syncpt: __u32,
//
// @id:
//
// The ID of the wait base corresponding to the client syncpoint. Set
// by the kernel upon successful completion of the IOCTL.
//
    pub id: __u32,
}

//
// struct drm_tegra_syncpt - syncpoint increment operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_syncpt {
//
// @id:
//
// ID of the syncpoint to operate on.
//
    pub id: __u32,
//
// @incrs:
//
// Number of increments to perform for the syncpoint.
//
    pub incrs: __u32,
}

//
// struct drm_tegra_cmdbuf - structure describing a command buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_cmdbuf {
//
// @handle:
//
// Handle to a GEM object containing the command buffer.
//
    pub handle: __u32,
//
// @offset:
//
// Offset, in bytes, into the GEM object identified by @handle at
// which the command buffer starts.
//
    pub offset: __u32,
//
// @words:
//
// Number of 32-bit words in this command buffer.
//
    pub words: __u32,
//
// @pad:
//
// Structure padding that may be used in the future. Must be 0.
//
    pub pad: __u32,
}

//
// struct drm_tegra_reloc - GEM object relocation structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_reloc {
// @cmdbuf: cmd information
//
// @cmdbuf.handle:
//
// Handle to the GEM object containing the command buffer for
// which to perform this GEM object relocation.
//
    pub handle: __u32,
//
// @cmdbuf.offset:
//
// Offset, in bytes, into the command buffer at which to
// insert the relocated address.
//
    pub offset: __u32,
    pub cmdbuf: },
// @target: relocate target information
//
// @target.handle:
//
// Handle to the GEM object to be relocated.
//
    pub handle: __u32,
//
// @target.offset:
//
// Offset, in bytes, into the target GEM object at which the
// relocated data starts.
//
    pub offset: __u32,
    pub target: },
//
// @shift:
//
// The number of bits by which to shift relocated addresses.
//
    pub shift: __u32,
//
// @pad:
//
// Structure padding that may be used in the future. Must be 0.
//
    pub pad: __u32,
}

//
// struct drm_tegra_waitchk - wait check structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_waitchk {
//
// @handle:
//
// Handle to the GEM object containing a command stream on which to
// perform the wait check.
//
    pub handle: __u32,
//
// @offset:
//
// Offset, in bytes, of the location in the command stream to perform
// the wait check on.
//
    pub offset: __u32,
//
// @syncpt:
//
// ID of the syncpoint to wait check.
//
    pub syncpt: __u32,
//
// @thresh:
//
// Threshold value for which to check.
//
    pub thresh: __u32,
}

//
// struct drm_tegra_submit - job submission structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_submit {
//
// @context:
//
// The application context identifying the channel to use for the
// execution of this job.
//
    pub context: __u64,
//
// @num_syncpts:
//
// The number of syncpoints operated on by this job. This defines the
// length of the array pointed to by @syncpts.
//
    pub num_syncpts: __u32,
//
// @num_cmdbufs:
//
// The number of command buffers to execute as part of this job. This
// defines the length of the array pointed to by @cmdbufs.
//
    pub num_cmdbufs: __u32,
//
// @num_relocs:
//
// The number of relocations to perform before executing this job.
// This defines the length of the array pointed to by @relocs.
//
    pub num_relocs: __u32,
//
// @num_waitchks:
//
// The number of wait checks to perform as part of this job. This
// defines the length of the array pointed to by @waitchks.
//
    pub num_waitchks: __u32,
//
// @waitchk_mask:
//
// Bitmask of valid wait checks.
//
    pub waitchk_mask: __u32,
//
// @timeout:
//
// Timeout, in milliseconds, before this job is cancelled.
//
    pub timeout: __u32,
//
// @syncpts:
//
// A pointer to an array of &struct drm_tegra_syncpt structures that
// specify the syncpoint operations performed as part of this job.
// The number of elements in the array must be equal to the value
// given by @num_syncpts.
//
    pub syncpts: __u64,
//
// @cmdbufs:
//
// A pointer to an array of &struct drm_tegra_cmdbuf structures that
// define the command buffers to execute as part of this job. The
// number of elements in the array must be equal to the value given
// by @num_syncpts.
//
    pub cmdbufs: __u64,
//
// @relocs:
//
// A pointer to an array of &struct drm_tegra_reloc structures that
// specify the relocations that need to be performed before executing
// this job. The number of elements in the array must be equal to the
// value given by @num_relocs.
//
    pub relocs: __u64,
//
// @waitchks:
//
// A pointer to an array of &struct drm_tegra_waitchk structures that
// specify the wait checks to be performed while executing this job.
// The number of elements in the array must be equal to the value
// given by @num_waitchks.
//
    pub waitchks: __u64,
//
// @fence:
//
// The threshold of the syncpoint associated with this job after it
// has been completed. Set by the kernel upon successful completion of
// the IOCTL. This can be used with the DRM_TEGRA_SYNCPT_WAIT IOCTL to
// wait for this job to be finished.
//
    pub fence: __u32,
//
// @reserved:
//
// This field is reserved for future use. Must be 0.
//
    pub reserved: [__u32; 5],
}

pub const DRM_TEGRA_GEM_TILING_MODE_PITCH: c_int = 0;
pub const DRM_TEGRA_GEM_TILING_MODE_TILED: c_int = 1;
pub const DRM_TEGRA_GEM_TILING_MODE_BLOCK: c_int = 2;
//
// struct drm_tegra_gem_set_tiling - parameters for the set tiling IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_gem_set_tiling {
//
// @handle:
//
// Handle to the GEM object for which to set the tiling parameters.
//
    pub handle: __u32,
//
// @mode:
//
// The tiling mode to set. Must be one of:
//
// DRM_TEGRA_GEM_TILING_MODE_PITCH
// pitch linear format
//
// DRM_TEGRA_GEM_TILING_MODE_TILED
// 16x16 tiling format
//
// DRM_TEGRA_GEM_TILING_MODE_BLOCK
// 16Bx2 tiling format
//
    pub mode: __u32,
//
// @value:
//
// The value to set for the tiling mode parameter.
//
    pub value: __u32,
//
// @pad:
//
// Structure padding that may be used in the future. Must be 0.
//
    pub pad: __u32,
}

//
// struct drm_tegra_gem_get_tiling - parameters for the get tiling IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_gem_get_tiling {
//
// @handle:
//
// Handle to the GEM object for which to query the tiling parameters.
//
    pub handle: __u32,
//
// @mode:
//
// The tiling mode currently associated with the GEM object. Set by
// the kernel upon successful completion of the IOCTL.
//
    pub mode: __u32,
//
// @value:
//
// The tiling mode parameter currently associated with the GEM object.
// Set by the kernel upon successful completion of the IOCTL.
//
    pub value: __u32,
//
// @pad:
//
// Structure padding that may be used in the future. Must be 0.
//
    pub pad: __u32,
}

//
// struct drm_tegra_gem_set_flags - parameters for the set flags IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_gem_set_flags {
//
// @handle:
//
// Handle to the GEM object for which to set the flags.
//
    pub handle: __u32,
//
// @flags:
//
// The flags to set for the GEM object.
//
    pub flags: __u32,
}

//
// struct drm_tegra_gem_get_flags - parameters for the get flags IOCTL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_gem_get_flags {
//
// @handle:
//
// Handle to the GEM object for which to query the flags.
//
    pub handle: __u32,
//
// @flags:
//
// The flags currently associated with the GEM object. Set by the
// kernel upon successful completion of the IOCTL.
//
    pub flags: __u32,
}

pub const DRM_TEGRA_GEM_CREATE: c_uint = 0x00;
pub const DRM_TEGRA_GEM_MMAP: c_uint = 0x01;
pub const DRM_TEGRA_SYNCPT_READ: c_uint = 0x02;
pub const DRM_TEGRA_SYNCPT_INCR: c_uint = 0x03;
pub const DRM_TEGRA_SYNCPT_WAIT: c_uint = 0x04;
pub const DRM_TEGRA_OPEN_CHANNEL: c_uint = 0x05;
pub const DRM_TEGRA_CLOSE_CHANNEL: c_uint = 0x06;
pub const DRM_TEGRA_GET_SYNCPT: c_uint = 0x07;
pub const DRM_TEGRA_SUBMIT: c_uint = 0x08;
pub const DRM_TEGRA_GET_SYNCPT_BASE: c_uint = 0x09;
pub const DRM_TEGRA_GEM_SET_TILING: c_uint = 0x0a;
pub const DRM_TEGRA_GEM_GET_TILING: c_uint = 0x0b;
pub const DRM_TEGRA_GEM_SET_FLAGS: c_uint = 0x0c;
pub const DRM_TEGRA_GEM_GET_FLAGS: c_uint = 0x0d;

// New Tegra DRM UAPI
//
// Reported by the driver in the `capabilities` field.
//
// DRM_TEGRA_CHANNEL_CAP_CACHE_COHERENT: If set, the engine is cache coherent
// with regard to the system memory.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_channel_open {
//
// @host1x_class: [in]
//
// Host1x class of the engine that will be programmed using this
// channel.
//
    pub host1x_class: __u32,
//
// @flags: [in]
//
// Flags.
//
    pub flags: __u32,
//
// @context: [out]
//
// Opaque identifier corresponding to the opened channel.
//
    pub context: __u32,
//
// @version: [out]
//
// Version of the engine hardware. This can be used by userspace
// to determine how the engine needs to be programmed.
//
    pub version: __u32,
//
// @capabilities: [out]
//
// Flags describing the hardware capabilities.
//
    pub capabilities: __u32,
    pub padding: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_channel_close {
//
// @context: [in]
//
// Identifier of the channel to close.
//
    pub context: __u32,
    pub padding: __u32,
}

//
// Mapping flags that can be used to influence how the mapping is created.
//
// DRM_TEGRA_CHANNEL_MAP_READ: create mapping that allows HW read access
// DRM_TEGRA_CHANNEL_MAP_WRITE: create mapping that allows HW write access
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_channel_map {
//
// @context: [in]
//
// Identifier of the channel to which make memory available for.
//
    pub context: __u32,
//
// @handle: [in]
//
// GEM handle of the memory to map.
//
    pub handle: __u32,
//
// @flags: [in]
//
// Flags.
//
    pub flags: __u32,
//
// @mapping: [out]
//
// Identifier corresponding to the mapping, to be used for
// relocations or unmapping later.
//
    pub mapping: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_channel_unmap {
//
// @context: [in]
//
// Channel identifier of the channel to unmap memory from.
//
    pub context: __u32,
//
// @mapping: [in]
//
// Mapping identifier of the memory mapping to unmap.
//
    pub mapping: __u32,
}

// Submission
//
// define DRM_TEGRA_SUBMIT_RELOC_SECTOR_LAYOUT - \
// Select sector layout swizzling for in-memory buffers.
//
// Specify that bit 39 of the patched-in address should be set to switch
// swizzling between Tegra and non-Tegra sector layout on systems that store
// surfaces in system memory in non-Tegra sector layout.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_submit_buf {
//
// @mapping: [in]
//
// Identifier of the mapping to use in the submission.
//
    pub mapping: __u32,
//
// @flags: [in]
//
// Flags.
//
    pub flags: __u32,
//
// Information for relocation patching.
//
// @target_offset: [in]
//
// Offset from the start of the mapping of the data whose
// address is to be patched into the gather.
//
    pub target_offset: __u64,
//
// @gather_offset_words: [in]
//
// Offset in words from the start of the gather data to
// where the address should be patched into.
//
    pub gather_offset_words: __u32,
//
// @shift: [in]
//
// Number of bits the address should be shifted right before
// patching in.
//
    pub shift: __u32,
    pub reloc: },
}

//
// define DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR - \
// Execute Host1x opcodes from user pointer.
//
// Execute `words` words of Host1x opcodes specified in the `gather_data_ptr`
// buffer. Each GATHER_UPTR command uses successive words from the buffer.
//
pub const DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR: c_int = 0;
//
// define DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT - \
// Wait for syncpoint (absolute).
//
// Wait for a syncpoint to reach a value before continuing with further
// commands.
//
pub const DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT: c_int = 1;
//
// define DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT_RELATIVE - \
// Wait for syncpoint (relative).
//
// Wait for a syncpoint to reach a value before continuing with further
// commands. The threshold is calculated relative to the start of the job.
//
pub const DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT_RELATIVE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_submit_cmd_gather_uptr {
    pub words: __u32,
    pub reserved: [__u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_submit_cmd_wait_syncpt {
    pub id: __u32,
    pub value: __u32,
    pub reserved: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_submit_cmd {
//
// @type: [in]
//
// Command type to execute. One of the DRM_TEGRA_SUBMIT_CMD
// defines.
//
    pub type: __u32,
//
// @flags: [in]
//
// Flags.
//
    pub flags: __u32,
    pub gather_uptr: drm_tegra_submit_cmd_gather_uptr,
    pub wait_syncpt: drm_tegra_submit_cmd_wait_syncpt,
    pub reserved: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_submit_syncpt {
//
// @id: [in]
//
// ID of the syncpoint that the job will increment.
//
    pub id: __u32,
//
// @flags: [in]
//
// Flags.
//
    pub flags: __u32,
//
// @increments: [in]
//
// Number of times the job will increment this syncpoint.
//
    pub increments: __u32,
//
// @value: [out]
//
// Value the syncpoint will have once the job has completed all
// its specified syncpoint increments.
//
// Note that the kernel may increment the syncpoint before or after
// the job. These increments are not reflected in this field.
//
// If the job hangs or times out, not all of the increments may
// get executed.
//
    pub value: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_channel_submit {
//
// @context: [in]
//
// Identifier of the channel to submit this job to.
//
    pub context: __u32,
//
// @num_bufs: [in]
//
// Number of elements in the `bufs_ptr` array.
//
    pub num_bufs: __u32,
//
// @num_cmds: [in]
//
// Number of elements in the `cmds_ptr` array.
//
    pub num_cmds: __u32,
//
// @gather_data_words: [in]
//
// Number of 32-bit words in the `gather_data_ptr` array.
//
    pub gather_data_words: __u32,
//
// @bufs_ptr: [in]
//
// Pointer to an array of drm_tegra_submit_buf structures.
//
    pub bufs_ptr: __u64,
//
// @cmds_ptr: [in]
//
// Pointer to an array of drm_tegra_submit_cmd structures.
//
    pub cmds_ptr: __u64,
//
// @gather_data_ptr: [in]
//
// Pointer to an array of Host1x opcodes to be used by GATHER_UPTR
// commands.
//
    pub gather_data_ptr: __u64,
//
// @syncobj_in: [in]
//
// Handle for DRM syncobj that will be waited before submission.
// Ignored if zero.
//
    pub syncobj_in: __u32,
//
// @syncobj_out: [in]
//
// Handle for DRM syncobj that will have its fence replaced with
// the job's completion fence. Ignored if zero.
//
    pub syncobj_out: __u32,
//
// @syncpt_incr: [in,out]
//
// Information about the syncpoint the job will increment.
//
    pub syncpt: drm_tegra_submit_syncpt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_syncpoint_allocate {
//
// @id: [out]
//
// ID of allocated syncpoint.
//
    pub id: __u32,
    pub padding: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_syncpoint_free {
//
// @id: [in]
//
// ID of syncpoint to free.
//
    pub id: __u32,
    pub padding: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_syncpoint_wait {
//
// @timeout: [in]
//
// Absolute timestamp at which the wait will time out.
//
    pub timeout_ns: __s64,
//
// @id: [in]
//
// ID of syncpoint to wait on.
//
    pub id: __u32,
//
// @threshold: [in]
//
// Threshold to wait for.
//
    pub threshold: __u32,
//
// @value: [out]
//
// Value of the syncpoint upon wait completion.
//
    pub value: __u32,
    pub padding: __u32,
}

