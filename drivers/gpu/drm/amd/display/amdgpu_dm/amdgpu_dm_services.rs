//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_services.c
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
//
// Copyright 2015 Advanced Micro Devices, Inc.
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

    unsigned long long
    dm_get_elapse_time_in_ns(struct dc_context *ctx,
    unsigned long long current_time_stamp,
    unsigned long long last_time_stamp)
    {
    return current_time_stamp - last_time_stamp;
    }
    EXPORT_IF_KUNIT(dm_get_elapse_time_in_ns);
#[no_mangle]
pub unsafe extern "C" fn dm_perf_trace_timestamp(func_name: *const c_char, line: c_uint, ctx: *mut dc_context) {
    void dm_perf_trace_timestamp(const char *func_name, unsigned int line, struct dc_context *ctx)
    {
    trace_amdgpu_dc_performance(ctx.perf_trace.read_count,
    ctx.perf_trace.write_count,
    &ctx.perf_trace.last_entry_read,
    &ctx.perf_trace.last_entry_write,
    func_name, line);
    }
    EXPORT_IF_KUNIT(dm_perf_trace_timestamp);
#[no_mangle]
pub unsafe extern "C" fn dm_trace_smu_enter(msg_id: u32, param_in: u32, delay: c_uint, ctx: *mut dc_context) {
    void dm_trace_smu_enter(uint32_t msg_id, uint32_t param_in, unsigned int delay, struct dc_context *ctx)
    {
    }
    EXPORT_IF_KUNIT(dm_trace_smu_enter);
#[no_mangle]
pub unsafe extern "C" fn dm_trace_smu_exit(success: bool, response: u32, ctx: *mut dc_context) {
    void dm_trace_smu_exit(bool success, uint32_t response, struct dc_context *ctx)
    {
    }
    EXPORT_IF_KUNIT(dm_trace_smu_exit);
// power component interfaces
    bool dm_query_extended_brightness_caps(struct dc_context *ctx,
    enum dm_acpi_display_type display, struct dm_acpi_atif_backlight_caps *pCaps)
    {
    struct amdgpu_device *adev;
    struct amdgpu_display_manager *dm;
    let mut bl_index: c_int = (display == AcpiDisplayType_LCD1) ? 0 : 1;
    if (!ctx || !pCaps || !ctx.driver_context)
    return false;
    adev = (struct amdgpu_device *)ctx.driver_context;
    dm = &adev.dm;
    amdgpu_dm_update_backlight_caps(dm, bl_index);
    pCaps.num_data_points = dm.backlight_caps[bl_index].data_points;
    pCaps.max_input_signal = dm.backlight_caps[bl_index].max_input_signal;
    pCaps.min_input_signal = dm.backlight_caps[bl_index].min_input_signal;
    pCaps.ac_level_percentage = dm.backlight_caps[bl_index].ac_level;
    pCaps.dc_level_percentage = dm.backlight_caps[bl_index].dc_level;
    if (pCaps.num_data_points > 0)
    memcpy(pCaps.data_points, dm.backlight_caps[bl_index].luminance_data,
    sizeof(struct dm_bl_data_point) * pCaps.num_data_points);
    return true;
    }
    EXPORT_IF_KUNIT(dm_query_extended_brightness_caps);
    void*
    dm_allocate_gpu_mem(
    struct amdgpu_device *adev,
    enum dc_gpu_mem_alloc_type type,
    size_t size,
    long long *addr)
    {
    struct dal_allocation *da;
    u32 domain = (type == DC_MEM_ALLOC_TYPE_GART) ?
    AMDGPU_GEM_DOMAIN_GTT : AMDGPU_GEM_DOMAIN_VRAM;
    int ret;
    da = kzalloc_obj(*da);
    if (!da)
    return core::ptr::null_mut();
    ret = amdgpu_bo_create_kernel(adev, size, PAGE_SIZE,
    domain, &da.bo,
    &da.gpu_addr, &da.cpu_ptr);
// addr = da->gpu_addr;
    if (ret) {
    kfree(da);
    return core::ptr::null_mut();
    }
// add da to list in dm
    list_add(&da.list, &adev.dm.da_list);
    return da.cpu_ptr;
    }
    void
    dm_free_gpu_mem(
    struct amdgpu_device *adev,
    enum dc_gpu_mem_alloc_type type,
    void *pvMem)
    {
    struct dal_allocation *da;
// walk the da list in DM
    list_for_each_entry(da, &adev.dm.da_list, list) {
    if (pvMem == da.cpu_ptr) {
    amdgpu_bo_free_kernel(&da.bo, &da.gpu_addr, &da.cpu_ptr);
    list_del(&da.list);
    kfree(da);
    break;
    }
    }
    }
