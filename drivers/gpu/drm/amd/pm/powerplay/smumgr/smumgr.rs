//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/pm/powerplay/smumgr/smumgr.c
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

    MODULE_FIRMWARE("amdgpu/bonaire_smc.bin");
    MODULE_FIRMWARE("amdgpu/bonaire_k_smc.bin");
    MODULE_FIRMWARE("amdgpu/hawaii_smc.bin");
    MODULE_FIRMWARE("amdgpu/hawaii_k_smc.bin");
    MODULE_FIRMWARE("amdgpu/topaz_smc.bin");
    MODULE_FIRMWARE("amdgpu/topaz_k_smc.bin");
    MODULE_FIRMWARE("amdgpu/tonga_smc.bin");
    MODULE_FIRMWARE("amdgpu/tonga_k_smc.bin");
    MODULE_FIRMWARE("amdgpu/fiji_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris10_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris10_smc_sk.bin");
    MODULE_FIRMWARE("amdgpu/polaris10_k_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris10_k2_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris11_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris11_smc_sk.bin");
    MODULE_FIRMWARE("amdgpu/polaris11_k_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris11_k2_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris12_smc.bin");
    MODULE_FIRMWARE("amdgpu/polaris12_k_smc.bin");
    MODULE_FIRMWARE("amdgpu/vegam_smc.bin");
    MODULE_FIRMWARE("amdgpu/vega10_smc.bin");
    MODULE_FIRMWARE("amdgpu/vega10_acg_smc.bin");
    MODULE_FIRMWARE("amdgpu/vega12_smc.bin");
    MODULE_FIRMWARE("amdgpu/vega20_smc.bin");
#[no_mangle]
pub unsafe extern "C" fn smum_thermal_avfs_enable(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_thermal_avfs_enable(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.thermal_avfs_enable)
    return hwmgr.smumgr_funcs.thermal_avfs_enable(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_thermal_setup_fan_table(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_thermal_setup_fan_table(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.thermal_setup_fan_table)
    return hwmgr.smumgr_funcs.thermal_setup_fan_table(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_update_sclk_threshold(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_update_sclk_threshold(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.update_sclk_threshold)
    return hwmgr.smumgr_funcs.update_sclk_threshold(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_update_smc_table(hwmgr: *mut pp_hwmgr, type: u32) -> c_int {
    int smum_update_smc_table(struct pp_hwmgr *hwmgr, uint32_t type)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.update_smc_table)
    return hwmgr.smumgr_funcs.update_smc_table(hwmgr, type);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_get_offsetof(hwmgr: *mut pp_hwmgr, type: u32, member: u32) -> u32 {
    uint32_t smum_get_offsetof(struct pp_hwmgr *hwmgr, uint32_t type, uint32_t member)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.get_offsetof)
    return hwmgr.smumgr_funcs.get_offsetof(type, member);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_process_firmware_header(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_process_firmware_header(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.process_firmware_header)
    return hwmgr.smumgr_funcs.process_firmware_header(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_get_mac_definition(hwmgr: *mut pp_hwmgr, value: u32) -> u32 {
    uint32_t smum_get_mac_definition(struct pp_hwmgr *hwmgr, uint32_t value)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.get_mac_definition)
    return hwmgr.smumgr_funcs.get_mac_definition(value);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_download_powerplay_table(hwmgr: *mut pp_hwmgr, table: *mut c_void) -> c_int {
    int smum_download_powerplay_table(struct pp_hwmgr *hwmgr, void **table)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.download_pptable_settings)
    return hwmgr.smumgr_funcs.download_pptable_settings(hwmgr,
    table);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_upload_powerplay_table(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_upload_powerplay_table(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.upload_pptable_settings)
    return hwmgr.smumgr_funcs.upload_pptable_settings(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_send_msg_to_smc(hwmgr: *mut pp_hwmgr, msg: u16, resp: *mut u32) -> c_int {
    int smum_send_msg_to_smc(struct pp_hwmgr *hwmgr, uint16_t msg, uint32_t *resp)
    {
    let mut ret: c_int = 0;
    if (hwmgr == core::ptr::null_mut() ||
    hwmgr.smumgr_funcs.send_msg_to_smc == core::ptr::null_mut() ||
    (resp && !hwmgr.smumgr_funcs.get_argument))
    return -EINVAL;
    mutex_lock(&hwmgr.msg_lock);
    ret = hwmgr.smumgr_funcs.send_msg_to_smc(hwmgr, msg);
    if (ret) {
    mutex_unlock(&hwmgr.msg_lock);
    return ret;
    }
    if (resp)
// resp = hwmgr->smumgr_funcs->get_argument(hwmgr);
    mutex_unlock(&hwmgr.msg_lock);
    return ret;
    }
    int smum_send_msg_to_smc_with_parameter(struct pp_hwmgr *hwmgr,
    uint16_t msg,
    uint32_t parameter,
    uint32_t *resp)
    {
    let mut ret: c_int = 0;
    if (hwmgr == core::ptr::null_mut() ||
    hwmgr.smumgr_funcs.send_msg_to_smc_with_parameter == core::ptr::null_mut() ||
    (resp && !hwmgr.smumgr_funcs.get_argument))
    return -EINVAL;
    mutex_lock(&hwmgr.msg_lock);
    ret = hwmgr.smumgr_funcs.send_msg_to_smc_with_parameter(
    hwmgr, msg, parameter);
    if (ret) {
    mutex_unlock(&hwmgr.msg_lock);
    return ret;
    }
    if (resp)
// resp = hwmgr->smumgr_funcs->get_argument(hwmgr);
    mutex_unlock(&hwmgr.msg_lock);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_init_smc_table(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_init_smc_table(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.init_smc_table)
    return hwmgr.smumgr_funcs.init_smc_table(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_populate_all_graphic_levels(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_populate_all_graphic_levels(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.populate_all_graphic_levels)
    return hwmgr.smumgr_funcs.populate_all_graphic_levels(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_populate_all_memory_levels(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_populate_all_memory_levels(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.populate_all_memory_levels)
    return hwmgr.smumgr_funcs.populate_all_memory_levels(hwmgr);
    return 0;
    }
// this interface is needed by island ci/vi
#[no_mangle]
pub unsafe extern "C" fn smum_initialize_mc_reg_table(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_initialize_mc_reg_table(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.initialize_mc_reg_table)
    return hwmgr.smumgr_funcs.initialize_mc_reg_table(hwmgr);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_is_dpm_running(hwmgr: *mut pp_hwmgr) -> bool {
    bool smum_is_dpm_running(struct pp_hwmgr *hwmgr)
    {
    if (core::ptr::null_mut() != hwmgr.smumgr_funcs.is_dpm_running)
    return hwmgr.smumgr_funcs.is_dpm_running(hwmgr);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_is_hw_avfs_present(hwmgr: *mut pp_hwmgr) -> bool {
    bool smum_is_hw_avfs_present(struct pp_hwmgr *hwmgr)
    {
    if (hwmgr.smumgr_funcs.is_hw_avfs_present)
    return hwmgr.smumgr_funcs.is_hw_avfs_present(hwmgr);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_update_dpm_settings(hwmgr: *mut pp_hwmgr, profile_setting: *mut c_void) -> c_int {
    int smum_update_dpm_settings(struct pp_hwmgr *hwmgr, void *profile_setting)
    {
    if (hwmgr.smumgr_funcs.update_dpm_settings)
    return hwmgr.smumgr_funcs.update_dpm_settings(hwmgr, profile_setting);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_smc_table_manager(hwmgr: *mut pp_hwmgr, table: *mut u8, table_id: u16, rw: bool) -> c_int {
    int smum_smc_table_manager(struct pp_hwmgr *hwmgr, uint8_t *table, uint16_t table_id, bool rw)
    {
    if (hwmgr.smumgr_funcs.smc_table_manager)
    return hwmgr.smumgr_funcs.smc_table_manager(hwmgr, table, table_id, rw);
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn smum_stop_smc(hwmgr: *mut pp_hwmgr) -> c_int {
    int smum_stop_smc(struct pp_hwmgr *hwmgr)
    {
    if (hwmgr.smumgr_funcs.stop_smc)
    return hwmgr.smumgr_funcs.stop_smc(hwmgr);
    return 0;
    }
