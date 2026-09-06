//! Automatically rewritten from C to Rust
//! Source: sound/soc/intel/avs/tgl.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2024 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
// Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn avs_tgl_dsp_core_power(adev: *mut avs_dev, core_mask: u32, power: bool) -> c_int {
    static int avs_tgl_dsp_core_power(struct avs_dev *adev, u32 core_mask, bool power)
    {
    core_mask &= AVS_MAIN_CORE_MASK;
    if (!core_mask)
    return 0;
    return avs_dsp_core_power(adev, core_mask, power);
    }
#[no_mangle]
unsafe extern "C" fn avs_tgl_dsp_core_reset(adev: *mut avs_dev, core_mask: u32, reset: bool) -> c_int {
    static int avs_tgl_dsp_core_reset(struct avs_dev *adev, u32 core_mask, bool reset)
    {
    core_mask &= AVS_MAIN_CORE_MASK;
    if (!core_mask)
    return 0;
    return avs_dsp_core_reset(adev, core_mask, reset);
    }
#[no_mangle]
unsafe extern "C" fn avs_tgl_dsp_core_stall(adev: *mut avs_dev, core_mask: u32, stall: bool) -> c_int {
    static int avs_tgl_dsp_core_stall(struct avs_dev *adev, u32 core_mask, bool stall)
    {
    core_mask &= AVS_MAIN_CORE_MASK;
    if (!core_mask)
    return 0;
    return avs_dsp_core_stall(adev, core_mask, stall);
    }
//
// Succeed if CPUID(0x15) is not available, or if the nominal core crystal clock
// frequency cannot be enumerated from it.  There is nothing to do in both cases.
//
#[no_mangle]
unsafe extern "C" fn avs_tgl_set_xtal_freq(adev: *mut avs_dev) -> c_int {
    static int avs_tgl_set_xtal_freq(struct avs_dev *adev)
    {
    unsigned int freq;
    int ret;
    if (boot_cpu_data.cpuid_level < CPUID_LEAF_TSC)
    return 0;
    freq = cpuid_ecx(CPUID_LEAF_TSC);
    if (freq) {
    ret = avs_ipc_set_fw_config(adev, 1, AVS_FW_CFG_XTAL_FREQ_HZ, sizeof(freq), &freq);
    if (ret)
    return AVS_IPC_RET(ret);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn avs_tgl_config_basefw(adev: *mut avs_dev) -> c_int {
    static int avs_tgl_config_basefw(struct avs_dev *adev)
    {
    struct pci_dev *pci = adev.base.pci;
    struct avs_bus_hwid hwid;
    int ret;
    ret = avs_tgl_set_xtal_freq(adev);
    if (ret)
    return ret;
    hwid.device = pci.device;
    hwid.subsystem = pci.subsystem_vendor | (pci.subsystem_device << 16);
    hwid.revision = pci.revision;
    ret = avs_ipc_set_fw_config(adev, 1, AVS_FW_CFG_BUS_HARDWARE_ID, sizeof(hwid), &hwid);
    if (ret)
    return AVS_IPC_RET(ret);
    return 0;
    }
    const struct avs_dsp_ops avs_tgl_dsp_ops = {
    .power = avs_tgl_dsp_core_power,
    .reset = avs_tgl_dsp_core_reset,
    .stall = avs_tgl_dsp_core_stall,
    .dsp_interrupt = avs_cnl_dsp_interrupt,
    .int_control = avs_dsp_interrupt_control,
    .load_basefw = avs_icl_load_basefw,
    .load_lib = avs_hda_load_library,
    .transfer_mods = avs_hda_transfer_modules,
    .config_basefw = avs_tgl_config_basefw,
    .log_buffer_offset = avs_icl_log_buffer_offset,
    .log_buffer_status = avs_apl_log_buffer_status,
    .coredump = avs_apl_coredump,
    .d0ix_toggle = avs_icl_d0ix_toggle,
    .set_d0ix = avs_icl_set_d0ix,
    AVS_SET_ENABLE_LOGS_OP(icl)
    };
