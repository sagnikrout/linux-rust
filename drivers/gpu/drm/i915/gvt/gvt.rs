//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/gvt.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
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
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Kevin Tian <kevin.tian@intel.com>
// Eddie Dong <eddie.dong@intel.com>
//
// Contributors:
// Niu Bing <bing.niu@intel.com>
// Zhi Wang <zhi.a.wang@intel.com>
//

pub const GVT_MAX_VGPU: c_int = 8;
// Describe per-platform limitations.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_device_info {
    pub max_support_vgpus: u32,
    pub cfg_space_size: u32,
    pub mmio_size: u32,
    pub mmio_bar: u32,
    pub msi_cap_offset: c_ulong,
    pub gtt_start_offset: u32,
    pub gtt_entry_size: u32,
    pub gtt_entry_size_shift: u32,
    pub gmadr_bytes_in_cmd: c_int,
    pub max_surface_size: u32,
}

// GM resources owned by a vGPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_gm {
    pub aperture_sz: u64,
    pub hidden_sz: u64,
    pub low_gm_node: drm_mm_node,
    pub high_gm_node: drm_mm_node,
}

pub const INTEL_GVT_MAX_NUM_FENCES: c_int = 32;
// Fences owned by a vGPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_fence {
    pub regs: [*mut i915_fence_reg; INTEL_GVT_MAX_NUM_FENCES],
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_mmio {
    pub vreg: *mut c_void,
}

pub const INTEL_GVT_MAX_BAR_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_pci_bar {
    pub size: u64,
    pub tracked: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_cfg_space {
    pub virtual_cfg_space: [c_uchar; PCI_CFG_SPACE_EXP_SIZE],
    pub bar: [intel_vgpu_pci_bar; INTEL_GVT_MAX_BAR_NUM],
    pub pmcsr_off: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_irq {
    pub irq_warn_once: [bool; INTEL_GVT_EVENT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_opregion {
    pub va: *mut c_void,
    pub gfn: [u32; INTEL_GVT_OPREGION_PAGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_display {
    pub i2c_edid: intel_vgpu_i2c_edid,
    pub ports: [intel_vgpu_port; I915_MAX_PORTS],
    pub sbi: intel_vgpu_sbi,
    pub port_num: port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgpu_sched_ctl {
    pub weight: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_submission_ops {
    pub name: *const c_char,
    pub engine_mask): *mut *mut *mut int (init)(struct intel_vgpu vgpu, intel_engine_mask_t,
    pub engine_mask): *mut *mut *mut void (clean)(struct intel_vgpu vgpu, intel_engine_mask_t,
    pub engine_mask): *mut *mut *mut void (reset)(struct intel_vgpu vgpu, intel_engine_mask_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_submission {
    pub execlist: [intel_vgpu_execlist; I915_NUM_ENGINES],
    pub workload_q_head: [list_head; I915_NUM_ENGINES],
    pub shadow: [*mut intel_context; I915_NUM_ENGINES],
    pub workloads: *mut kmem_cache,
    pub running_workload_num: core::sync::atomic::AtomicI32,
    pub i915_context_pml4: u64,
    pub i915_context_pdps: [u64; GEN8_3LVL_PDPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu {
    pub vfio_device: vfio_device,
    pub gvt: *mut intel_gvt,
    pub vgpu_lock: mutex,
    pub id: c_int,
    pub INTEL_VGPU_STATUS_NR_BITS): DECLARE_BITMAP(status,,
    pub pv_notified: bool,
    pub failsafe: bool,
    pub resetting_eng: c_uint,
// Both sched_data and sched_ctl can be seen a part of the global gvt
// scheduler structure. So below 2 vgpu data are protected
// by sched_lock, not vgpu_lock.
//
    pub sched_data: *mut c_void,
    pub sched_ctl: vgpu_sched_ctl,
    pub fence: intel_vgpu_fence,
    pub gm: intel_vgpu_gm,
    pub cfg_space: intel_vgpu_cfg_space,
    pub mmio: intel_vgpu_mmio,
    pub irq: intel_vgpu_irq,
    pub gtt: intel_vgpu_gtt,
    pub opregion: intel_vgpu_opregion,
    pub display: intel_vgpu_display,
    pub submission: intel_vgpu_submission,
    pub page_track_tree: radix_tree_root,
    pub hws_pga: [u32; I915_NUM_ENGINES],
// Set on PCI_D3, reset on DMLR, not reflecting the actual PM state
    pub d3_entered: bool,
    pub debugfs: *mut dentry,
    pub dmabuf_obj_list_head: list_head,
    pub dmabuf_lock: mutex,
    pub object_idr: idr,
    pub vblank_timer: intel_vgpu_vblank_timer,
    pub scan_nonprivbb: u32,
    pub region: *mut vfio_region,
    pub num_regions: c_int,
    pub msi_trigger: *mut eventfd_ctx,
//
// Two caches are used to avoid mapping duplicated pages (eg.
// scratch pages). This help to reduce dma setup overhead.
//
    pub gfn_cache: rb_root,
    pub dma_addr_cache: rb_root,
    pub nr_cache_entries: c_ulong,
    pub cache_lock: mutex,
    pub track_node: kvm_page_track_notifier_node,
    pub ptable: [hlist_head; NR_BKT],
}

// validating GM healthy status

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_gm {
    pub vgpu_allocated_low_gm_size: c_ulong,
    pub vgpu_allocated_high_gm_size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_fence {
    pub vgpu_allocated_fence_num: c_ulong,
}

// Special MMIO blocks.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gvt_mmio_block {
    pub offset: i915_reg_t,
    pub size: c_uint,
    pub read: gvt_mmio_func,
    pub write: gvt_mmio_func,
}

pub const INTEL_GVT_MMIO_HASH_BITS: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_mmio {
    pub mmio_attribute: *mut u16,
// Register contains RO bits

// Register contains graphics address

// Mode mask registers with high 16 bits as the mask bits

// This reg can be accessed by GPU commands

// This reg has been accessed by a VM

// This reg requires save & restore during host PM suspend/resume

// This reg could be accessed by unaligned address

// This reg is in GVT's mmio save-restor list and in hardware
// logical context image
//

// Value of command write of this reg needs to be patched

    pub mmio_block: *mut gvt_mmio_block,
    pub num_mmio_block: c_uint,
    pub INTEL_GVT_MMIO_HASH_BITS): DECLARE_HASHTABLE(mmio_info_table,,
    pub num_tracked_mmio: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_firmware {
    pub cfg_space: *mut c_void,
    pub mmio: *mut c_void,
    pub firmware_loaded: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_config {
    pub low_mm: c_uint,
    pub high_mm: c_uint,
    pub fence: c_uint,
//
// A vGPU with a weight of 8 will get twice as much GPU as a vGPU with
// a weight of 4 on a contended host, different vGPU type has different
// weight set. Legal weights range from 1 to 16.
//
    pub weight: c_uint,
    pub edid: intel_vgpu_edid,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_type {
    pub type: mdev_type,
    pub name: [c_char; 16],
    pub conf: *const intel_vgpu_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt {
// GVT scope lock, protect GVT itself, and all resource currently
// not yet protected by special locks(vgpu and scheduler lock).
//
    pub lock: mutex,
// scheduler scope lock, protect gvt and vgpu schedule related data
    pub sched_lock: mutex,
    pub gt: *mut intel_gt,
    pub /: *mut *mut idr vgpu_idr; / vGPU IDR pool,
    pub device_info: intel_gvt_device_info,
    pub gm: intel_gvt_gm,
    pub fence: intel_gvt_fence,
    pub mmio: intel_gvt_mmio,
    pub firmware: intel_gvt_firmware,
    pub irq: intel_gvt_irq,
    pub gtt: intel_gvt_gtt,
    pub scheduler: intel_gvt_workload_scheduler,
    pub shadow_ctx_notifier_block: [notifier_block; I915_NUM_ENGINES],
    pub GVT_CMD_HASH_BITS): DECLARE_HASHTABLE(cmd_table,,
    pub parent: mdev_parent,
    pub mdev_types: *mut mdev_type,
    pub types: *mut intel_vgpu_type,
    pub num_types: c_uint,
    pub idle_vgpu: *mut intel_vgpu,
    pub service_thread: *mut task_struct,
    pub service_thread_wq: wait_queue_head_t,
// service_request is always used in bit operation, we should always
// use it with atomic bit ops so that no need to use gvt big lock.
//
    pub service_request: c_ulong,
    pub mmio: *mut engine_mmio,
    pub ctx_mmio_count: [c_int; I915_NUM_ENGINES],
    pub tlb_mmio_offset_list: *mut u32,
    pub tlb_mmio_offset_list_cnt: u32,
    pub mocs_mmio_offset_list: *mut u32,
    pub mocs_mmio_offset_list_cnt: u32,
    pub engine_mmio_list: },
    pub is_reg_whitelist_updated: bool,
    pub debugfs_root: *mut dentry,
}

// Scheduling trigger by timer
// Scheduling trigger by event
// per-vGPU vblank emulation request
extern "C" {
    pub fn intel_gvt_free_firmware(gvt: *mut intel_gvt);
}
extern "C" {
    pub fn intel_gvt_load_firmware(gvt: *mut intel_gvt) -> c_int;
}
// Aperture/GM space definitions for GVT device

pub const HOST_FENCE: c_int = 4;

// Aperture/GM space definitions for GVT device

// Aperture/GM space definitions for vGPU

// ring context size i.e. the first 0x50 dwords
pub const RING_CTX_SIZE: c_int = 320;
extern "C" {
    pub fn intel_vgpu_reset_resource(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_free_resource(vgpu: *mut intel_vgpu);
}
//
// Macros for easily accessing vGPU virtual/shadow register.
// Explicitly separate use for typed MMIO reg or real offset.
//

// BAR offset should be 32 bits algiend
//
// only update bit 31 - bit 4,
// leave the bit 3 - bit 0 unchanged.
//
// pval = (val & GENMASK(31, 4)) | (*pval & GENMASK(3, 0));
// pval = val;
extern "C" {
    pub fn intel_gvt_init_vgpu_types(gvt: *mut intel_gvt) -> c_int;
}
extern "C" {
    pub fn intel_gvt_clean_vgpu_types(gvt: *mut intel_gvt);
}
extern "C" {
    pub fn intel_gvt_destroy_idle_vgpu(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_destroy_vgpu(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_release_vgpu(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_reset_vgpu(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_activate_vgpu(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_deactivate_vgpu(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_set_opregion(vgpu: *mut intel_vgpu) -> c_int;
}
extern "C" {
    pub fn intel_gvt_set_edid(vgpu: *mut intel_vgpu, port_num: c_int) -> c_int;
}
// validating GM functions

extern "C" {
    pub fn intel_gvt_ggtt_validate_range(vgpu: *mut intel_vgpu, addr: u64, size: u32) -> bool;
}
extern "C" {
    pub fn intel_vgpu_reset_cfg_space(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_emulate_hotplug(vgpu: *mut intel_vgpu, connected: bool);
}
// We are 64bit bar.
extern "C" {
    pub fn intel_vgpu_clean_opregion(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_init_opregion(vgpu: *mut intel_vgpu) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_opregion_base_write_handler(vgpu: *mut intel_vgpu, gpa: u32) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_emulate_opregion_request(vgpu: *mut intel_vgpu, swsci: u32) -> c_int;
}
extern "C" {
    pub fn populate_pvinfo_page(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_scan_and_shadow_workload(workload: *mut intel_vgpu_workload) -> c_int;
}
extern "C" {
    pub fn enter_failsafe_mode(vgpu: *mut intel_vgpu, reason: c_int);
}
extern "C" {
    pub fn intel_vgpu_detach_regions(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_runtime_pm_get(_arg: gt->uncore->rpm) -> return;
}
//
// intel_gvt_mmio_set_accessed - mark a MMIO has been accessed
// @gvt: a GVT device
// @offset: register offset
//
// intel_gvt_mmio_is_cmd_accessible - if a MMIO could be accessed by command
// @gvt: a GVT device
// @offset: register offset
//
// Returns:
// True if an MMIO is able to be accessed by GPU commands
//
// intel_gvt_mmio_set_cmd_accessible -
// mark a MMIO could be accessible by command
// @gvt: a GVT device
// @offset: register offset
//
// intel_gvt_mmio_is_unalign - mark a MMIO could be accessed unaligned
// @gvt: a GVT device
// @offset: register offset
//
// intel_gvt_mmio_has_mode_mask - if a MMIO has a mode mask
// @gvt: a GVT device
// @offset: register offset
//
// Returns:
// True if a MMIO has a mode mask in its higher 16 bits, false if it isn't.
//
// intel_gvt_mmio_is_sr_in_ctx -
// check if an MMIO has F_SR_IN_CTX mask
// @gvt: a GVT device
// @offset: register offset
//
// Returns:
// True if an MMIO has an F_SR_IN_CTX  mask, false if it isn't.
//
// intel_gvt_mmio_set_sr_in_ctx -
// mask an MMIO in GVT's mmio save-restore list and also
// in hardware logical context image
// @gvt: a GVT device
// @offset: register offset
//
extern "C" {
    pub fn intel_gvt_debugfs_add_vgpu(vgpu: *mut intel_vgpu);
}
//
// intel_gvt_mmio_set_cmd_write_patch -
// mark an MMIO if its cmd write needs to be
// patched
// @gvt: a GVT device
// @offset: register offset
//
// intel_gvt_mmio_is_cmd_write_patch - check if an mmio's cmd access needs to
// be patched
// @gvt: a GVT device
// @offset: register offset
//
// Returns:
// True if GPU command write to an MMIO should be patched.
//
// intel_gvt_read_gpa - copy data from GPA to host data buffer
// @vgpu: a vGPU
// @gpa: guest physical address
// @buf: host data buffer
// @len: data length
//
// Returns:
// Zero on success, negative error code if failed.
//
extern "C" {
    pub fn vfio_dma_rw(_arg: &vgpu->vfio_device, _arg: gpa, _arg: buf, _arg: len, _arg: false) -> return;
}
//
// intel_gvt_write_gpa - copy data from host data buffer to GPA
// @vgpu: a vGPU
// @gpa: guest physical address
// @buf: host data buffer
// @len: data length
//
// Returns:
// Zero on success, negative error code if failed.
//
extern "C" {
    pub fn vfio_dma_rw(_arg: &vgpu->vfio_device, _arg: gpa, _arg: buf, _arg: len, _arg: true) -> return;
}
extern "C" {
    pub fn intel_gvt_debugfs_remove_vgpu(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_gvt_debugfs_init(gvt: *mut intel_gvt);
}
extern "C" {
    pub fn intel_gvt_debugfs_clean(gvt: *mut intel_gvt);
}
extern "C" {
    pub fn intel_gvt_page_track_add(info: *mut intel_vgpu, gfn: u64) -> c_int;
}
extern "C" {
    pub fn intel_gvt_page_track_remove(info: *mut intel_vgpu, gfn: u64) -> c_int;
}
extern "C" {
    pub fn intel_gvt_dma_pin_guest_page(vgpu: *mut intel_vgpu, dma_addr: dma_addr_t) -> c_int;
}

