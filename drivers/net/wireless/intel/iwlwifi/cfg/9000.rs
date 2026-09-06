//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/9000.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2015-2017 Intel Deutschland GmbH
// Copyright (C) 2018-2021, 2023, 2025-2026 Intel Corporation
//

// Highest firmware API version supported
pub const IWL9000_UCODE_API_MAX: c_int = 46;
// Lowest firmware API version supported
pub const IWL9000_UCODE_API_MIN: c_int = 30;
// Memory offsets and lengths
pub const IWL9000_SMEM_OFFSET: c_uint = 0x400000;
pub const IWL9000_SMEM_LEN: c_uint = 0x68000;

    IWL9000_FW_PRE "-" __stringify(api) ".ucode"

    IWL9260_FW_PRE "-" __stringify(api) ".ucode"
    static const struct iwl_family_base_params iwl9000_base = {
    .eeprom_size = OTP_LOW_IMAGE_SIZE_32K,
    .num_of_queues = 31,
    .max_tfd_queue_size = 256,
    .wd_timeout = IWL_LONG_WD_TIMEOUT,
    .shadow_reg_enable = true,
    .pcie_l1_allowed = true,
    .smem_offset = IWL9000_SMEM_OFFSET,
    .smem_len = IWL9000_SMEM_LEN,
    .features = IWL_TX_CSUM_NETIF_FLAGS | NETIF_F_RXCSUM,
    .apmg_not_supported = true,
    .mac_addr_from_csr = 0x380,
    .d3_debug_data_base_addr = 0x401000,
    .d3_debug_data_length = 92 * 1024,
    .nvm_hw_section_num = 10,
    .mon_smem_regs = {
    .write_ptr = {
    .addr = LDBG_M2S_BUF_WPTR,
    .mask = LDBG_M2S_BUF_WPTR_VAL_MSK,
    },
    .cycle_cnt = {
    .addr = LDBG_M2S_BUF_WRAP_CNT,
    .mask = LDBG_M2S_BUF_WRAP_CNT_VAL_MSK,
    },
    },
    .mon_dram_regs = {
    .write_ptr = {
    .addr = MON_BUFF_WRPTR_VER2,
    .mask = 0xffffffff,
    },
    .cycle_cnt = {
    .addr = MON_BUFF_CYCLE_CNT_VER2,
    .mask = 0xffffffff,
    },
    },
    .ucode_api_max = IWL9000_UCODE_API_MAX,
    .ucode_api_min = IWL9000_UCODE_API_MIN,
    };
    const struct iwl_mac_cfg iwl9000_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_9000,
    .base = &iwl9000_base,
    .mq_rx_supported = true,
    };
    const struct iwl_mac_cfg iwl9560_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_9000,
    .base = &iwl9000_base,
    .mq_rx_supported = true,
    .integrated = true,
    .xtal_latency = 650,
    };
    const struct iwl_mac_cfg iwl9560_long_latency_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_9000,
    .base = &iwl9000_base,
    .mq_rx_supported = true,
    .integrated = true,
    .xtal_latency = 2820,
    };
    const struct iwl_mac_cfg iwl9560_shared_clk_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_9000,
    .base = &iwl9000_base,
    .mq_rx_supported = true,
    .integrated = true,
    .xtal_latency = 670,
    .extra_phy_cfg_flags = FW_PHY_CFG_SHARED_CLK
    };
    MODULE_FIRMWARE(IWL9000_MODULE_FIRMWARE(IWL9000_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL9260_MODULE_FIRMWARE(IWL9000_UCODE_API_MAX));
