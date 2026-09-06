//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/sc.c
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

// Highest firmware core release supported
pub const IWL_SC_UCODE_CORE_MAX: c_int = 107;
// Lowest firmware core release supported
pub const IWL_SC_UCODE_CORE_MIN: c_int = 102;

    static const struct iwl_family_base_params iwl_sc_base = {
    .num_of_queues = 512,
    .max_tfd_queue_size = 65536,
    .wd_timeout = IWL_LONG_WD_TIMEOUT,
    .shadow_reg_enable = true,
    .pcie_l1_allowed = true,
    .apmg_not_supported = true,
    .mac_addr_from_csr = 0x30,
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
    .min_ba_txq_size = IWL_DEFAULT_QUEUE_SIZE_EHT,
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
    .mon_dbgi_regs = {
    .write_ptr = {
    .addr = DBGI_SRAM_FIFO_POINTERS,
    .mask = DBGI_SRAM_FIFO_POINTERS_WR_PTR_MSK,
    },
    },
    .features = IWL_TX_CSUM_NETIF_FLAGS | NETIF_F_RXCSUM,
    .ucode_api_max = ENCODE_CORE_AS_API(IWL_SC_UCODE_CORE_MAX),
    .ucode_api_min = ENCODE_CORE_AS_API(IWL_SC_UCODE_CORE_MIN),
    };
    const struct iwl_mac_cfg iwl_sc_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_SC,
    .base = &iwl_sc_base,
    .mq_rx_supported = true,
    .gen2 = true,
    .integrated = true,
    .umac_prph_offset = 0x300000,
    .xtal_latency = 12000,
    .low_latency_xtal = true,
    .ltr_delay = IWL_CFG_TRANS_LTR_DELAY_2500US,
    };
    IWL_CORE_FW(IWL_SC_A_FM_B_FW_PRE, IWL_SC_UCODE_CORE_MAX);
    IWL_CORE_FW(IWL_SC_A_FM_C_FW_PRE, IWL_SC_UCODE_CORE_MAX);
    IWL_CORE_FW(IWL_SC_A_WH_A_FW_PRE, IWL_SC_UCODE_CORE_MAX);
    IWL_CORE_FW(IWL_SC2_A_FM_C_FW_PRE, IWL_SC_UCODE_CORE_MAX);
    IWL_CORE_FW(IWL_SC2_A_WH_A_FW_PRE, IWL_SC_UCODE_CORE_MAX);
