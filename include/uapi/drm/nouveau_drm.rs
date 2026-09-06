//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/nouveau_drm.h
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
// Copyright 2005 Stephane Marchesin.
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
// VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
pub const DRM_NOUVEAU_EVENT_NVIF: c_uint = 0x80000000;

pub const NOUVEAU_GETPARAM_PCI_VENDOR: c_int = 3;
pub const NOUVEAU_GETPARAM_PCI_DEVICE: c_int = 4;
pub const NOUVEAU_GETPARAM_BUS_TYPE: c_int = 5;
pub const NOUVEAU_GETPARAM_FB_SIZE: c_int = 8;
pub const NOUVEAU_GETPARAM_AGP_SIZE: c_int = 9;
pub const NOUVEAU_GETPARAM_CHIPSET_ID: c_int = 11;
pub const NOUVEAU_GETPARAM_VM_VRAM_BASE: c_int = 12;
pub const NOUVEAU_GETPARAM_GRAPH_UNITS: c_int = 13;
pub const NOUVEAU_GETPARAM_PTIMER_TIME: c_int = 14;
pub const NOUVEAU_GETPARAM_HAS_BO_USAGE: c_int = 15;
pub const NOUVEAU_GETPARAM_HAS_PAGEFLIP: c_int = 16;
//
// NOUVEAU_GETPARAM_EXEC_PUSH_MAX - query max pushes through getparam
//
// Query the maximum amount of IBs that can be pushed through a single
// &drm_nouveau_exec structure and hence a single &DRM_IOCTL_NOUVEAU_EXEC
// ioctl().
//
pub const NOUVEAU_GETPARAM_EXEC_PUSH_MAX: c_int = 17;
//
// NOUVEAU_GETPARAM_VRAM_BAR_SIZE - query bar size
//
// Query the VRAM BAR size.
//
pub const NOUVEAU_GETPARAM_VRAM_BAR_SIZE: c_int = 18;
//
// NOUVEAU_GETPARAM_VRAM_USED
//
// Get remaining VRAM size.
//
pub const NOUVEAU_GETPARAM_VRAM_USED: c_int = 19;
//
// NOUVEAU_GETPARAM_HAS_VMA_TILEMODE
//
// Query whether tile mode and PTE kind are accepted with VM allocs or not.
//
pub const NOUVEAU_GETPARAM_HAS_VMA_TILEMODE: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_getparam {
    pub param: __u64,
    pub value: __u64,
}

//
// Those are used to support selecting the main engine used on Kepler.
// This goes into drm_nouveau_channel_alloc::tt_ctxdma_handle
//
pub const NOUVEAU_FIFO_ENGINE_GR: c_uint = 0x01;
pub const NOUVEAU_FIFO_ENGINE_VP: c_uint = 0x02;
pub const NOUVEAU_FIFO_ENGINE_PPP: c_uint = 0x04;
pub const NOUVEAU_FIFO_ENGINE_BSP: c_uint = 0x08;
pub const NOUVEAU_FIFO_ENGINE_CE: c_uint = 0x30;
pub const NOUVEAU_FIFO_ENGINE_NVDEC: c_uint = 0x300;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_channel_alloc {
    pub fb_ctxdma_handle: __u32,
    pub tt_ctxdma_handle: __u32,
    pub channel: __s32,
    pub pushbuf_domains: __u32,
// Notifier memory
    pub notifier_handle: __u32,
// DRM-enforced subchannel assignments
    pub handle: __u32,
    pub grclass: __u32,
    pub subchan: [}; 8],
    pub nr_subchan: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_channel_free {
    pub channel: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_notifierobj_alloc {
    pub channel: __u32,
    pub handle: __u32,
    pub size: __u32,
    pub offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gpuobj_free {
    pub channel: __s32,
    pub handle: __u32,
}

// The BO will never be shared via import or export.

pub const NOUVEAU_GEM_TILE_COMP: c_uint = 0x00030000 /* nv50-only */;
pub const NOUVEAU_GEM_TILE_LAYOUT_MASK: c_uint = 0x0000ff00;
pub const NOUVEAU_GEM_TILE_16BPP: c_uint = 0x00000001;
pub const NOUVEAU_GEM_TILE_32BPP: c_uint = 0x00000002;
pub const NOUVEAU_GEM_TILE_ZETA: c_uint = 0x00000004;
pub const NOUVEAU_GEM_TILE_NONCONTIG: c_uint = 0x00000008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_info {
    pub handle: __u32,
    pub domain: __u32,
    pub size: __u64,
    pub offset: __u64,
    pub map_handle: __u64,
    pub tile_mode: __u32,
    pub tile_flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_new {
    pub info: drm_nouveau_gem_info,
    pub channel_hint: __u32,
    pub align: __u32,
}

pub const NOUVEAU_GEM_MAX_BUFFERS: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_pushbuf_bo_presumed {
    pub valid: __u32,
    pub domain: __u32,
    pub offset: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_pushbuf_bo {
    pub user_priv: __u64,
    pub handle: __u32,
    pub read_domains: __u32,
    pub write_domains: __u32,
    pub valid_domains: __u32,
    pub presumed: drm_nouveau_gem_pushbuf_bo_presumed,
}

pub const NOUVEAU_GEM_MAX_RELOCS: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_pushbuf_reloc {
    pub reloc_bo_index: __u32,
    pub reloc_bo_offset: __u32,
    pub bo_index: __u32,
    pub flags: __u32,
    pub data: __u32,
    pub vor: __u32,
    pub tor: __u32,
}

pub const NOUVEAU_GEM_MAX_PUSH: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_pushbuf_push {
    pub bo_index: __u32,
    pub pad: __u32,
    pub offset: __u64,
    pub length: __u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_pushbuf {
    pub channel: __u32,
    pub nr_buffers: __u32,
    pub buffers: __u64,
    pub nr_relocs: __u32,
    pub nr_push: __u32,
    pub relocs: __u64,
    pub push: __u64,
    pub suffix0: __u32,
    pub suffix1: __u32,

    pub vram_available: __u64,
    pub gart_available: __u64,
}

pub const NOUVEAU_GEM_CPU_PREP_NOWAIT: c_uint = 0x00000001;
pub const NOUVEAU_GEM_CPU_PREP_WRITE: c_uint = 0x00000004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_cpu_prep {
    pub handle: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_gem_cpu_fini {
    pub handle: __u32,
}

//
// struct drm_nouveau_sync - sync object
//
// This structure serves as synchronization mechanism for (potentially)
// asynchronous operations such as EXEC or VM_BIND.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_sync {
//
// @flags: the flags for a sync object
//
// The first 8 bits are used to determine the type of the sync object.
//
    pub flags: __u32,
pub const DRM_NOUVEAU_SYNC_SYNCOBJ: c_uint = 0x0;
pub const DRM_NOUVEAU_SYNC_TIMELINE_SYNCOBJ: c_uint = 0x1;
pub const DRM_NOUVEAU_SYNC_TYPE_MASK: c_uint = 0xf;
//
// @handle: the handle of the sync object
//
    pub handle: __u32,
//
// @timeline_value:
//
// The timeline point of the sync object in case the syncobj is of
// type DRM_NOUVEAU_SYNC_TIMELINE_SYNCOBJ.
//
    pub timeline_value: __u64,
}

//
// struct drm_nouveau_vm_init - GPU VA space init structure
//
// Used to initialize the GPU's VA space for a user client, telling the kernel
// which portion of the VA space is managed by the UMD and kernel respectively.
//
// For the UMD to use the VM_BIND uAPI, this must be called before any BOs or
// channels are created; if called afterwards DRM_IOCTL_NOUVEAU_VM_INIT fails
// with -ENOSYS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_vm_init {
//
// @kernel_managed_addr: start address of the kernel managed VA space
// region
//
    pub kernel_managed_addr: __u64,
//
// @kernel_managed_size: size of the kernel managed VA space region in
// bytes
//
    pub kernel_managed_size: __u64,
}

//
// struct drm_nouveau_vm_bind_op - VM_BIND operation
//
// This structure represents a single VM_BIND operation. UMDs should pass
// an array of this structure via struct drm_nouveau_vm_bind's &op_ptr field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_vm_bind_op {
//
// @op: the operation type
//
// Supported values:
//
// %DRM_NOUVEAU_VM_BIND_OP_MAP - Map a GEM object to the GPU's VA
// space. Optionally, the &DRM_NOUVEAU_VM_BIND_SPARSE flag can be
// passed to instruct the kernel to create sparse mappings for the
// given range.
//
// %DRM_NOUVEAU_VM_BIND_OP_UNMAP - Unmap an existing mapping in the
// GPU's VA space. If the region the mapping is located in is a
// sparse region, new sparse mappings are created where the unmapped
// (memory backed) mapping was mapped previously. To remove a sparse
// region the &DRM_NOUVEAU_VM_BIND_SPARSE must be set.
//
    pub op: __u32,
pub const DRM_NOUVEAU_VM_BIND_OP_MAP: c_uint = 0x0;
pub const DRM_NOUVEAU_VM_BIND_OP_UNMAP: c_uint = 0x1;
//
// @flags: the flags for a &drm_nouveau_vm_bind_op
//
// Supported values:
//
// %DRM_NOUVEAU_VM_BIND_SPARSE - Indicates that an allocated VA
// space region should be sparse.
//
    pub flags: __u32,

//
// @handle: the handle of the DRM GEM object to map
//
    pub handle: __u32,
//
// @pad: 32 bit padding, should be 0
//
    pub pad: __u32,
//
// @addr:
//
// the address the VA space region or (memory backed) mapping should be mapped to
//
    pub addr: __u64,
//
// @bo_offset: the offset within the BO backing the mapping
//
    pub bo_offset: __u64,
//
// @range: the size of the requested mapping in bytes
//
    pub range: __u64,
}

//
// struct drm_nouveau_vm_bind - structure for DRM_IOCTL_NOUVEAU_VM_BIND
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_vm_bind {
//
// @op_count: the number of &drm_nouveau_vm_bind_op
//
    pub op_count: __u32,
//
// @flags: the flags for a &drm_nouveau_vm_bind ioctl
//
// Supported values:
//
// %DRM_NOUVEAU_VM_BIND_RUN_ASYNC - Indicates that the given VM_BIND
// operation should be executed asynchronously by the kernel.
//
// If this flag is not supplied the kernel executes the associated
// operations synchronously and doesn't accept any &drm_nouveau_sync
// objects.
//
    pub flags: __u32,
pub const DRM_NOUVEAU_VM_BIND_RUN_ASYNC: c_uint = 0x1;
//
// @wait_count: the number of wait &drm_nouveau_syncs
//
    pub wait_count: __u32,
//
// @sig_count: the number of &drm_nouveau_syncs to signal when finished
//
    pub sig_count: __u32,
//
// @wait_ptr: pointer to &drm_nouveau_syncs to wait for
//
    pub wait_ptr: __u64,
//
// @sig_ptr: pointer to &drm_nouveau_syncs to signal when finished
//
    pub sig_ptr: __u64,
//
// @op_ptr: pointer to the &drm_nouveau_vm_bind_ops to execute
//
    pub op_ptr: __u64,
}

//
// struct drm_nouveau_exec_push - EXEC push operation
//
// This structure represents a single EXEC push operation. UMDs should pass an
// array of this structure via struct drm_nouveau_exec's &push_ptr field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_exec_push {
//
// @va: the virtual address of the push buffer mapping
//
    pub va: __u64,
//
// @va_len: the length of the push buffer mapping
//
    pub va_len: __u32,
//
// @flags: the flags for this push buffer mapping
//
    pub flags: __u32,
pub const DRM_NOUVEAU_EXEC_PUSH_NO_PREFETCH: c_uint = 0x1;
}

//
// struct drm_nouveau_exec - structure for DRM_IOCTL_NOUVEAU_EXEC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_exec {
//
// @channel: the channel to execute the push buffer in
//
    pub channel: __u32,
//
// @push_count: the number of &drm_nouveau_exec_push ops
//
    pub push_count: __u32,
//
// @wait_count: the number of wait &drm_nouveau_syncs
//
    pub wait_count: __u32,
//
// @sig_count: the number of &drm_nouveau_syncs to signal when finished
//
    pub sig_count: __u32,
//
// @wait_ptr: pointer to &drm_nouveau_syncs to wait for
//
    pub wait_ptr: __u64,
//
// @sig_ptr: pointer to &drm_nouveau_syncs to signal when finished
//
    pub sig_ptr: __u64,
//
// @push_ptr: pointer to &drm_nouveau_exec_push ops
//
    pub push_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_get_zcull_info {
//
// @width_align_pixels: required alignment for region widths, in pixels
// (typically #TPC's * 16).
//
    pub width_align_pixels: __u32,
//
// @height_align_pixels: required alignment for region heights, in
// pixels (typically 32).
//
    pub height_align_pixels: __u32,
//
// @pixel_squares_by_aliquots: the pixel area covered by an aliquot
// (typically #Zcull_banks * 16 * 16).
//
    pub pixel_squares_by_aliquots: __u32,
//
// @aliquot_total: the total aliquot pool available in hardware
//
    pub aliquot_total: __u32,
//
// @zcull_region_byte_multiplier: the size of an aliquot in bytes, which
// is used for save/restore operations on a region
//
    pub zcull_region_byte_multiplier: __u32,
//
// @zcull_region_header_size: the region header size in bytes, which is
// used for save/restore operations on a region
//
    pub zcull_region_header_size: __u32,
//
// @zcull_subregion_header_size: the subregion header size in bytes,
// which is used for save/restore operations on a region
//
    pub zcull_subregion_header_size: __u32,
//
// @subregion_count: the total number of subregions the hardware
// supports
//
    pub subregion_count: __u32,
//
// @subregion_width_align_pixels: required alignment for subregion
// widths, in pixels (typically #TPC's * 16).
//
    pub subregion_width_align_pixels: __u32,
//
// @subregion_height_align_pixels: required alignment for subregion
// heights, in pixels
//
    pub subregion_height_align_pixels: __u32,
//
// @ctxsw_size: the size, in bytes, of a zcull context switching region.
// Will be zero if the kernel does not support zcull context switching.
//
    pub ctxsw_size: __u32,
//
// @ctxsw_align: the alignment, in bytes, of a zcull context switching
// region
//
    pub ctxsw_align: __u32,
}

pub const DRM_NOUVEAU_GETPARAM: c_uint = 0x00;
pub const DRM_NOUVEAU_SETPARAM: c_uint = 0x01 /* deprecated */;
pub const DRM_NOUVEAU_CHANNEL_ALLOC: c_uint = 0x02;
pub const DRM_NOUVEAU_CHANNEL_FREE: c_uint = 0x03;
pub const DRM_NOUVEAU_GROBJ_ALLOC: c_uint = 0x04 /* deprecated */;
pub const DRM_NOUVEAU_NOTIFIEROBJ_ALLOC: c_uint = 0x05 /* deprecated */;
pub const DRM_NOUVEAU_GPUOBJ_FREE: c_uint = 0x06 /* deprecated */;
pub const DRM_NOUVEAU_NVIF: c_uint = 0x07;
pub const DRM_NOUVEAU_SVM_INIT: c_uint = 0x08;
pub const DRM_NOUVEAU_SVM_BIND: c_uint = 0x09;
pub const DRM_NOUVEAU_VM_INIT: c_uint = 0x10;
pub const DRM_NOUVEAU_VM_BIND: c_uint = 0x11;
pub const DRM_NOUVEAU_EXEC: c_uint = 0x12;
pub const DRM_NOUVEAU_GET_ZCULL_INFO: c_uint = 0x13;
pub const DRM_NOUVEAU_GEM_NEW: c_uint = 0x40;
pub const DRM_NOUVEAU_GEM_PUSHBUF: c_uint = 0x41;
pub const DRM_NOUVEAU_GEM_CPU_PREP: c_uint = 0x42;
pub const DRM_NOUVEAU_GEM_CPU_FINI: c_uint = 0x43;
pub const DRM_NOUVEAU_GEM_INFO: c_uint = 0x44;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_svm_init {
    pub unmanaged_addr: __u64,
    pub unmanaged_size: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nouveau_svm_bind {
    pub header: __u64,
    pub va_start: __u64,
    pub va_end: __u64,
    pub npages: __u64,
    pub stride: __u64,
    pub result: __u64,
    pub reserved0: __u64,
    pub reserved1: __u64,
}

pub const NOUVEAU_SVM_BIND_COMMAND_SHIFT: c_int = 0;
pub const NOUVEAU_SVM_BIND_COMMAND_BITS: c_int = 8;

pub const NOUVEAU_SVM_BIND_PRIORITY_SHIFT: c_int = 8;
pub const NOUVEAU_SVM_BIND_PRIORITY_BITS: c_int = 8;

pub const NOUVEAU_SVM_BIND_TARGET_SHIFT: c_int = 16;
pub const NOUVEAU_SVM_BIND_TARGET_BITS: c_int = 32;
pub const NOUVEAU_SVM_BIND_TARGET_MASK: c_uint = 0xffffffff;
//
// Below is use to validate ioctl argument, userspace can also use it to make
// sure that no bit are set beyond known fields for a given kernel version.
//
pub const NOUVEAU_SVM_BIND_VALID_BITS: c_int = 48;

//
// NOUVEAU_BIND_COMMAND__MIGRATE: synchronous migrate to target memory.
// result: number of page successfuly migrate to the target memory.
//
pub const NOUVEAU_SVM_BIND_COMMAND__MIGRATE: c_int = 0;
//
// NOUVEAU_SVM_BIND_HEADER_TARGET__GPU_VRAM: target the GPU VRAM memory.
//

