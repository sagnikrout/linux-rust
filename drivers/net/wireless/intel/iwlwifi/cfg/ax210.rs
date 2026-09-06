//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/ax210.c
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
// Copyright (C) 2018-2026 Intel Corporation
//

// Highest firmware API version supported
pub const IWL_AX210_UCODE_API_MAX: c_int = 89;
// Lowest firmware API version supported
pub const IWL_AX210_UCODE_API_MIN: c_int = 89;

    IWL_SO_A_HR_B_FW_PRE "-" __stringify(api) ".ucode"

    IWL_MA_A_HR_B_FW_PRE "-" __stringify(api) ".ucode"

    IWL_MA_B_HR_B_FW_PRE "-" __stringify(api) ".ucode"
    static const struct iwl_family_base_params iwl_ax210_base = {
    .num_of_queues = 512,
    .max_tfd_queue_size = 65536,
    .wd_timeout = IWL_LONG_WD_TIMEOUT,
    .shadow_reg_enable = true,
    .pcie_l1_allowed = true,
    .features = IWL_TX_CSUM_NETIF_FLAGS | NETIF_F_RXCSUM,
    .apmg_not_supported = true,
    .mac_addr_from_csr = 0x380,
    .d3_debug_data_base_addr = 0x401000,
    .d3_debug_data_length = 60 * 1024,
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
    .min_txq_size = 128,
    .gp2_reg_addr = 0xd02c68,
    .min_ba_txq_size = IWL_DEFAULT_QUEUE_SIZE_HE,
    .mon_dram_regs = {
    .write_ptr = {
    .addr = DBGC_CUR_DBGBUF_STATUS,
    .mask = DBGC_CUR_DBGBUF_STATUS_OFFSET_MSK,
    },
    .cycle_cnt = {
    .addr = DBGC_DBGBUF_WRAP_AROUND,
    .mask = 0xffffffff,
    },
    .cur_frag = {
    .addr = DBGC_CUR_DBGBUF_STATUS,
    .mask = DBGC_CUR_DBGBUF_STATUS_IDX_MSK,
    },
    },
    .ucode_api_min = IWL_AX210_UCODE_API_MIN,
    .ucode_api_max = IWL_AX210_UCODE_API_MAX,
    };
    const struct iwl_mac_cfg iwl_ty_mac_cfg = {
    .mq_rx_supported = true,
    .gen2 = true,
    .device_family = IWL_DEVICE_FAMILY_AX210,
    .base = &iwl_ax210_base,
    .umac_prph_offset = 0x300000,
// TODO: the following values need to be checked
    .xtal_latency = 500,
    };
    const struct iwl_mac_cfg iwl_so_mac_cfg = {
    .mq_rx_supported = true,
    .gen2 = true,
    .device_family = IWL_DEVICE_FAMILY_AX210,
    .base = &iwl_ax210_base,
    .umac_prph_offset = 0x300000,
    .integrated = true,
// TODO: the following values need to be checked
    .xtal_latency = 500,
    .ltr_delay = IWL_CFG_TRANS_LTR_DELAY_200US,
    };
    const struct iwl_mac_cfg iwl_so_long_latency_mac_cfg = {
    .mq_rx_supported = true,
    .gen2 = true,
    .device_family = IWL_DEVICE_FAMILY_AX210,
    .base = &iwl_ax210_base,
    .umac_prph_offset = 0x300000,
    .integrated = true,
    .low_latency_xtal = true,
    .xtal_latency = 12000,
    .ltr_delay = IWL_CFG_TRANS_LTR_DELAY_2500US,
    };
    const struct iwl_mac_cfg iwl_so_long_latency_imr_mac_cfg = {
    .mq_rx_supported = true,
    .gen2 = true,
    .device_family = IWL_DEVICE_FAMILY_AX210,
    .base = &iwl_ax210_base,
    .umac_prph_offset = 0x300000,
    .integrated = true,
    .low_latency_xtal = true,
    .xtal_latency = 12000,
    .ltr_delay = IWL_CFG_TRANS_LTR_DELAY_2500US,
    .imr_enabled = true,
    };
    const struct iwl_mac_cfg iwl_ma_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_AX210,
    .base = &iwl_ax210_base,
    .mq_rx_supported = true,
    .gen2 = true,
    .integrated = true,
    .umac_prph_offset = 0x300000
    };
    IWL_FW_AND_PNVM(IWL_SO_A_GF_A_FW_PRE, IWL_AX210_UCODE_API_MAX);
    IWL_FW_AND_PNVM(IWL_TY_A_GF_A_FW_PRE, IWL_AX210_UCODE_API_MAX);
    IWL_FW_AND_PNVM(IWL_MA_A_GF_A_FW_PRE, IWL_AX210_UCODE_API_MAX);
    IWL_FW_AND_PNVM(IWL_MA_B_GF_A_FW_PRE, IWL_AX210_UCODE_API_MAX);
    IWL_FW_AND_PNVM(IWL_SO_A_GF4_A_FW_PRE, IWL_AX210_UCODE_API_MAX);
    IWL_FW_AND_PNVM(IWL_MA_A_GF4_A_FW_PRE, IWL_AX210_UCODE_API_MAX);
    IWL_FW_AND_PNVM(IWL_MA_B_GF4_A_FW_PRE, IWL_AX210_UCODE_API_MAX);
    MODULE_FIRMWARE(IWL_SO_A_HR_B_MODULE_FIRMWARE(IWL_AX210_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_MA_A_HR_B_MODULE_FIRMWARE(IWL_AX210_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_MA_B_HR_B_MODULE_FIRMWARE(IWL_AX210_UCODE_API_MAX));
