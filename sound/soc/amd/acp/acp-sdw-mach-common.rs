//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/acp/acp-sdw-mach-common.c
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
// Copyright(c) 2024 Advanced Micro Devices, Inc.
//
// acp-sdw-mach-common - Common machine driver helper functions for
// legacy(No DSP) stack and SOF stack.
//

#[no_mangle]
pub unsafe extern "C" fn get_acp63_cpu_pin_id(sdw_link_id: u32, be_id: c_int, cpu_pin_id: *mut c_int, dev: *mut device) -> c_int {
    int get_acp63_cpu_pin_id(u32 sdw_link_id, int be_id, int *cpu_pin_id, struct device *dev)
    {
    switch (sdw_link_id) {
    case AMD_SDW0:
    switch (be_id) {
    case SOC_SDW_JACK_OUT_DAI_ID:
// cpu_pin_id = ACP63_SW0_AUDIO0_TX;
    break;
    case SOC_SDW_JACK_IN_DAI_ID:
// cpu_pin_id = ACP63_SW0_AUDIO0_RX;
    break;
    case SOC_SDW_AMP_OUT_DAI_ID:
// cpu_pin_id = ACP63_SW0_AUDIO1_TX;
    break;
    case SOC_SDW_AMP_IN_DAI_ID:
// cpu_pin_id = ACP63_SW0_AUDIO1_RX;
    break;
    case SOC_SDW_DMIC_DAI_ID:
// cpu_pin_id = ACP63_SW0_AUDIO2_RX;
    break;
    default:
    dev_err(dev, "Invalid be id:%d\n", be_id);
    return -EINVAL;
    }
    break;
    case AMD_SDW1:
    switch (be_id) {
    case SOC_SDW_JACK_OUT_DAI_ID:
    case SOC_SDW_AMP_OUT_DAI_ID:
// cpu_pin_id = ACP63_SW1_AUDIO0_TX;
    break;
    case SOC_SDW_JACK_IN_DAI_ID:
    case SOC_SDW_AMP_IN_DAI_ID:
    case SOC_SDW_DMIC_DAI_ID:
// cpu_pin_id = ACP63_SW1_AUDIO0_RX;
    break;
    default:
    dev_err(dev, "invalid be_id:%d\n", be_id);
    return -EINVAL;
    }
    break;
    default:
    dev_err(dev, "Invalid link id:%d\n", sdw_link_id);
    return -EINVAL;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(get_acp63_cpu_pin_id, "SND_SOC_AMD_SDW_MACH");
#[no_mangle]
pub unsafe extern "C" fn get_acp70_cpu_pin_id(sdw_link_id: u32, be_id: c_int, cpu_pin_id: *mut c_int, dev: *mut device) -> c_int {
    int get_acp70_cpu_pin_id(u32 sdw_link_id, int be_id, int *cpu_pin_id, struct device *dev)
    {
    switch (sdw_link_id) {
    case AMD_SDW0:
    case AMD_SDW1:
    switch (be_id) {
    case SOC_SDW_JACK_OUT_DAI_ID:
// cpu_pin_id = ACP70_SW_AUDIO0_TX;
    break;
    case SOC_SDW_JACK_IN_DAI_ID:
// cpu_pin_id = ACP70_SW_AUDIO0_RX;
    break;
    case SOC_SDW_AMP_OUT_DAI_ID:
// cpu_pin_id = ACP70_SW_AUDIO1_TX;
    break;
    case SOC_SDW_AMP_IN_DAI_ID:
// cpu_pin_id = ACP70_SW_AUDIO1_RX;
    break;
    case SOC_SDW_DMIC_DAI_ID:
// cpu_pin_id = ACP70_SW_AUDIO2_RX;
    break;
    default:
    dev_err(dev, "Invalid be id:%d\n", be_id);
    return -EINVAL;
    }
    break;
    default:
    return -EINVAL;
    }
    dev_dbg(dev, "sdw_link_id:%d, be_id:%d, cpu_pin_id:%d\n", sdw_link_id, be_id, *cpu_pin_id);
    return 0;
    }
    EXPORT_SYMBOL_NS_GPL(get_acp70_cpu_pin_id, "SND_SOC_AMD_SDW_MACH");
    MODULE_DESCRIPTION("AMD SoundWire Common Machine driver");
    MODULE_AUTHOR("Vijendar Mukunda <Vijendar.Mukunda@amd.com>");
    MODULE_LICENSE("GPL");
