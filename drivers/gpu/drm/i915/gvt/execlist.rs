//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/execlist.h
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
// Zhiyuan Lv <zhiyuan.lv@intel.com>
// Zhi Wang <zhi.a.wang@intel.com>
//
// Contributors:
// Min He <min.he@intel.com>
// Bing Niu <bing.niu@intel.com>
// Ping Gao <ping.a.gao@intel.com>
// Tina Zhang <tina.zhang@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct execlist_ctx_descriptor_format {
    pub ldw: u32,
    pub 1: u32 valid :,
    pub 1: u32 force_pd_restore :,
    pub 1: u32 force_restore :,
    pub 2: u32 addressing_mode :,
    pub 1: u32 llc_coherency :,
    pub 2: u32 fault_handling :,
    pub 1: u32 privilege_access :,
    pub 3: u32 reserved :,
    pub 20: u32 lrca :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct execlist_status_format {
    pub ldw: u32,
    pub :1: u32 current_execlist_pointer,
    pub :1: u32 execlist_write_pointer,
    pub :1: u32 execlist_queue_full,
    pub :1: u32 execlist_1_valid,
    pub :1: u32 execlist_0_valid,
    pub :9: u32 last_ctx_switch_reason,
    pub :2: u32 current_active_elm_status,
    pub :1: u32 arbitration_enable,
    pub :1: u32 execlist_1_active,
    pub :1: u32 execlist_0_active,
    pub :13: u32 reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct execlist_context_status_pointer_format {
    pub dw: u32,
    pub :3: u32 write_ptr,
    pub :5: u32 reserved,
    pub :3: u32 read_ptr,
    pub :5: u32 reserved2,
    pub :16: u32 mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct execlist_context_status_format {
    pub ldw: u32,
    pub :1: u32 idle_to_active,
    pub :1: u32 preempted,
    pub :1: u32 element_switch,
    pub :1: u32 active_to_idle,
    pub :1: u32 context_complete,
    pub :1: u32 wait_on_sync_flip,
    pub :1: u32 wait_on_vblank,
    pub :1: u32 wait_on_semaphore,
    pub :1: u32 wait_on_scanline,
    pub :2: u32 reserved,
    pub :1: u32 semaphore_wait_mode,
    pub :3: u32 display_plane,
    pub :1: u32 lite_restore,
    pub :16: u32 reserved_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct execlist_mmio_pair {
    pub addr: u32,
    pub val: u32,
}

// The first 52 dwords in register state context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct execlist_ring_context {
    pub nop1: u32,
    pub lri_cmd_1: u32,
    pub ctx_ctrl: execlist_mmio_pair,
    pub ring_header: execlist_mmio_pair,
    pub ring_tail: execlist_mmio_pair,
    pub rb_start: execlist_mmio_pair,
    pub rb_ctrl: execlist_mmio_pair,
    pub bb_cur_head_UDW: execlist_mmio_pair,
    pub bb_cur_head_LDW: execlist_mmio_pair,
    pub bb_state: execlist_mmio_pair,
    pub second_bb_addr_UDW: execlist_mmio_pair,
    pub second_bb_addr_LDW: execlist_mmio_pair,
    pub second_bb_state: execlist_mmio_pair,
    pub bb_per_ctx_ptr: execlist_mmio_pair,
    pub rcs_indirect_ctx: execlist_mmio_pair,
    pub rcs_indirect_ctx_offset: execlist_mmio_pair,
    pub nop2: u32,
    pub nop3: u32,
    pub nop4: u32,
    pub lri_cmd_2: u32,
    pub ctx_timestamp: execlist_mmio_pair,
//
// pdps[8]={ pdp3_UDW, pdp3_LDW, pdp2_UDW, pdp2_LDW,
// pdp1_UDW, pdp1_LDW, pdp0_UDW, pdp0_LDW}
//
    pub pdps: [execlist_mmio_pair; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_elsp_dwords {
    pub data: [u32; 4],
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_execlist_slot {
    pub ctx: [execlist_ctx_descriptor_format; 2],
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_execlist {
    pub slot: [intel_vgpu_execlist_slot; 2],
    pub running_slot: *mut intel_vgpu_execlist_slot,
    pub pending_slot: *mut intel_vgpu_execlist_slot,
    pub running_context: *mut execlist_ctx_descriptor_format,
    pub vgpu: *mut intel_vgpu,
    pub elsp_dwords: intel_vgpu_elsp_dwords,
    pub engine: *const intel_engine_cs,
}

extern "C" {
    pub fn intel_vgpu_clean_execlist(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_init_execlist(vgpu: *mut intel_vgpu) -> c_int;
}
