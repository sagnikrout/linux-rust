//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/core/dc_vm_helper.c
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

#[no_mangle]
pub unsafe extern "C" fn vm_helper_mark_vmid_used(vm_helper: *mut vm_helper, pos: c_uint, hubp_idx: u8) {
    void vm_helper_mark_vmid_used(struct vm_helper *vm_helper, unsigned int pos, uint8_t hubp_idx)
    {
    let mut vmids: vmid_usage = vm_helper.hubp_vmid_usage[hubp_idx];
    vmids.vmid_usage[0] = vmids.vmid_usage[1];
    vmids.vmid_usage[1] = 1 << pos;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_setup_system_context(dc: *mut dc, pa_config: *mut dc_phy_addr_space_config) -> c_uint {
    unsigned int dc_setup_system_context(struct dc *dc, struct dc_phy_addr_space_config *pa_config)
    {
    let mut num_vmids: c_uint = 0;
// Call HWSS to setup HUBBUB for address config
    if (dc.hwss.init_sys_ctx) {
    num_vmids = dc.hwss.init_sys_ctx(dc.hwseq, dc, pa_config);
// Pre-init system aperture start/end for all HUBP instances (if not gating?)
// or cache system aperture if using power gating
//
    memcpy(&dc.vm_pa_config, pa_config, sizeof(struct dc_phy_addr_space_config));
    dc.vm_pa_config.valid = true;
    dc.dml2_options.gpuvm_enable = true;
    dc_z10_save_init(dc);
    }
    return num_vmids;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_setup_vm_context(dc: *mut dc, va_config: *mut dc_virtual_addr_space_config, vmid: c_int) {
    void dc_setup_vm_context(struct dc *dc, struct dc_virtual_addr_space_config *va_config, int vmid)
    {
    dc.hwss.init_vm_ctx(dc.hwseq, dc, va_config, vmid);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_get_vmid_use_vector(dc: *mut dc) -> c_int {
    int dc_get_vmid_use_vector(struct dc *dc)
    {
    int i;
    let mut in_use: c_int = 0;
    for (i = 0; i < MAX_HUBP; i++)
    in_use |= dc.vm_helper.hubp_vmid_usage[i].vmid_usage[0]
    | dc.vm_helper.hubp_vmid_usage[i].vmid_usage[1];
    return in_use;
    }
#[no_mangle]
pub unsafe extern "C" fn vm_helper_init(vm_helper: *mut vm_helper, num_vmid: c_uint) {
    void vm_helper_init(struct vm_helper *vm_helper, unsigned int num_vmid)
    {
    vm_helper.num_vmid = num_vmid;
    memset(vm_helper.hubp_vmid_usage, 0, sizeof(vm_helper.hubp_vmid_usage[0]) * MAX_HUBP);
    }
