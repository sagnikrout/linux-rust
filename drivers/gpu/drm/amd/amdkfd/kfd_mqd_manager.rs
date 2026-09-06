//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_mqd_manager.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2014-2022 Advanced Micro Devices, Inc.
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

pub const KFD_MAX_NUM_SE: c_int = 8;
pub const KFD_MAX_NUM_SH_PER_SE: c_int = 2;
//
// struct mqd_manager
//
// @init_mqd: Allocates the mqd buffer on local gpu memory and initialize it.
//
// @load_mqd: Loads the mqd to a concrete hqd slot. Used only for no cp
// scheduling mode.
//
// @update_mqd: Handles a update call for the MQD
//
// @destroy_mqd: Destroys the HQD slot and by that preempt the relevant queue.
// Used only for no cp scheduling.
//
// @free_mqd: Releases the mqd buffer from local gpu memory.
//
// @is_occupied: Checks if the relevant HQD slot is occupied.
//
// @get_wave_state: Retrieves context save state and optionally copies the
// control stack, if kept in the MQD, to the given userspace address.
//
// @mqd_mutex: Mqd manager mutex.
//
// @dev: The kfd device structure coupled with this module.
//
// MQD stands for Memory Queue Descriptor which represents the current queue
// state in the memory and initiate the HQD (Hardware Queue Descriptor) state.
// This structure is actually a base class for the different types of MQDs
// structures for the variant ASICs that should be supported in the future.
// This base class is also contains all the MQD specific operations.
// Another important thing to mention is that each queue has a MQD that keeps
// his state (or context) after each preemption or reassignment.
// Basically there are a instances of the mqd manager class per MQD type per
// ASIC. Currently the kfd driver supports only Kaveri so there are instances
// per KFD_MQD_TYPE for each device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mqd_manager {
    pub q): *mut queue_properties,
    pub q): *mut queue_properties,
    pub mms): *mut mm_struct,
    pub minfo): *mut mqd_update_info,
    pub queue_id): u32,
    pub mqd_mem_obj): *mut kfd_mem_obj,
    pub queue_id): u32,
    pub save_area_used_size): *mut u32,
    pub ctl_stack_size): *mut u32,
    pub ctl_stack_dst): *mut c_void,
    pub ctl_stack_size): u32,
// Patch the MQD's cached self GPU address after the MQD BO has moved
// (e.g. repinned to a new VRAM location on hibernation resume). The MQD
// contents are otherwise preserved.
//
    pub p): *mut queue_properties,

    pub data): *mut *mut *mut int (debugfs_show_mqd)(struct seq_file m, void,

    pub mqd): *mut *mut *mut bool (check_preemption_failed)(struct mqd_manager mm, void,
    pub p): *mut queue_properties,
    pub mqd_mutex: mutex,
    pub dev: *mut kfd_node,
    pub mqd_size: u32,
    pub ctl_stack_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mqd_user_context_save_area_header {
// Byte offset from start of user context
// save area to the last saved top (lowest
// address) of control stack data. Must be
// 4 byte aligned.
//
    pub control_stack_offset: u32,
// Byte size of the last saved control stack
// data. Must be 4 byte aligned.
//
    pub control_stack_size: u32,
// Byte offset from start of user context save
// area to the last saved base (lowest address)
// of wave state data. Must be 4 byte aligned.
//
    pub wave_state_offset: u32,
// Byte size of the last saved wave state data.
// Must be 4 byte aligned.
//
    pub wave_state_size: u32,
}

extern "C" {
    pub fn kfd_hiq_mqd_stride(dev: *mut kfd_node) -> u64;
}
extern "C" {
    pub fn mqd_on_vram(adev: *mut amdgpu_device) -> bool;
}
