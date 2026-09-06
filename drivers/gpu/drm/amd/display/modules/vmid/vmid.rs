//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/modules/vmid/vmid.c
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
// Copyright 2019 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_vmid {
    pub public: mod_vmid,
    pub dc: *mut dc,
    pub num_vmid: c_uint,
    pub num_vmids_available: c_uint,
    pub ptb_assigned_to_vmid: [u64; MAX_VMID],
    pub base_config: dc_virtual_addr_space_config,
}

// Macro flag: #define MOD_VMID_TO_CORE(mod_vmid)\
    container_of(mod_vmid, struct core_vmid, public)
#[no_mangle]
unsafe extern "C" fn add_ptb_to_table(core_vmid: *mut core_vmid, vmid: c_uint, ptb: u64) {
    static void add_ptb_to_table(struct core_vmid *core_vmid, unsigned int vmid, uint64_t ptb)
    {
    if (vmid < MAX_VMID) {
    core_vmid.ptb_assigned_to_vmid[vmid] = ptb;
    core_vmid.num_vmids_available--;
    }
    }
#[no_mangle]
unsafe extern "C" fn clear_entry_from_vmid_table(core_vmid: *mut core_vmid, vmid: c_uint) {
    static void clear_entry_from_vmid_table(struct core_vmid *core_vmid, unsigned int vmid)
    {
    if (vmid < MAX_VMID) {
    core_vmid.ptb_assigned_to_vmid[vmid] = 0;
    core_vmid.num_vmids_available++;
    }
    }
#[no_mangle]
unsafe extern "C" fn evict_vmids(core_vmid: *mut core_vmid) {
    static void evict_vmids(struct core_vmid *core_vmid)
    {
    unsigned int i;
    let mut ord_int: c_int = dc_get_vmid_use_vector(core_vmid.dc);
    ASSERT(ord_int >= 0 && ord_int <= 0xFFFF);
    let mut ord: u16 = (uint16_t)ord_int;
// At this point any positions with value 0 are unused vmids, evict them
    for (i = 1; i < core_vmid.num_vmid; i++) {
    if (!(ord & (1u << i)))
    clear_entry_from_vmid_table(core_vmid, i);
    }
    }
// Return value of -1 indicates vmid table uninitialized or ptb dne in the table
#[no_mangle]
unsafe extern "C" fn get_existing_vmid_for_ptb(core_vmid: *mut core_vmid, ptb: u64) -> c_int {
    static int get_existing_vmid_for_ptb(struct core_vmid *core_vmid, uint64_t ptb)
    {
    unsigned int i;
    for (i = 0; i < core_vmid.num_vmid; i++) {
    if (core_vmid.ptb_assigned_to_vmid[i] == ptb)
    return i;
    }
    return -1;
    }
// Expected to be called only when there's an available vmid
#[no_mangle]
unsafe extern "C" fn get_next_available_vmid(core_vmid: *mut core_vmid) -> c_int {
    static int get_next_available_vmid(struct core_vmid *core_vmid)
    {
    unsigned int i;
    for (i = 1; i < core_vmid.num_vmid; i++) {
    if (core_vmid.ptb_assigned_to_vmid[i] == 0)
    return i;
    }
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn mod_vmid_get_for_ptb(mod_vmid: *mut mod_vmid, ptb: u64) -> u8 {
    uint8_t mod_vmid_get_for_ptb(struct mod_vmid *mod_vmid, uint64_t ptb)
    {
    struct core_vmid *core_vmid = MOD_VMID_TO_CORE(mod_vmid);
    let mut vmid: c_int = 0;
// Physical address gets vmid 0
    if (ptb == 0)
    return 0;
    vmid = get_existing_vmid_for_ptb(core_vmid, ptb);
    if (vmid == -1) {
    let mut va_config: dc_virtual_addr_space_config = core_vmid.base_config;
    va_config.page_table_base_addr = ptb;
    if (core_vmid.num_vmids_available == 0)
    evict_vmids(core_vmid);
    vmid = get_next_available_vmid(core_vmid);
    if (vmid != -1) {
    add_ptb_to_table(core_vmid, vmid, ptb);
    dc_setup_vm_context(core_vmid.dc, &va_config, vmid);
    } else
    ASSERT(0);
    }
    ASSERT(vmid >= 0 && vmid <= 0xFF);
    return (uint8_t)vmid;
    }
#[no_mangle]
pub unsafe extern "C" fn mod_vmid_reset(mod_vmid: *mut mod_vmid) {
    void mod_vmid_reset(struct mod_vmid *mod_vmid)
    {
    struct core_vmid *core_vmid = MOD_VMID_TO_CORE(mod_vmid);
    core_vmid.num_vmids_available = core_vmid.num_vmid - 1;
    memset(core_vmid.ptb_assigned_to_vmid, 0, sizeof(core_vmid.ptb_assigned_to_vmid[0]) * MAX_VMID);
    }
    struct mod_vmid *mod_vmid_create(
    struct dc *dc,
    unsigned int num_vmid,
    struct dc_virtual_addr_space_config *va_config)
    {
    struct core_vmid *core_vmid;
    if (num_vmid <= 1)
    goto fail_no_vm_ctx;
    if (dc == core::ptr::null_mut())
    goto fail_dc_null;
    core_vmid = kzalloc_obj(struct core_vmid);
    if (core_vmid == core::ptr::null_mut())
    goto fail_alloc_context;
    core_vmid.dc = dc;
    core_vmid.num_vmid = num_vmid;
    core_vmid.num_vmids_available = num_vmid - 1;
    core_vmid.base_config = *va_config;
    memset(core_vmid.ptb_assigned_to_vmid, 0, sizeof(core_vmid.ptb_assigned_to_vmid[0]) * MAX_VMID);
    return &core_vmid.public;
    fail_no_vm_ctx:
    fail_alloc_context:
    fail_dc_null:
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn mod_vmid_destroy(mod_vmid: *mut mod_vmid) {
    void mod_vmid_destroy(struct mod_vmid *mod_vmid)
    {
    if (mod_vmid != core::ptr::null_mut()) {
    struct core_vmid *core_vmid = MOD_VMID_TO_CORE(mod_vmid);
    kfree(core_vmid);
    }
    }
