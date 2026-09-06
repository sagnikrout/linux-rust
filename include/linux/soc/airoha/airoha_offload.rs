//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/airoha/airoha_offload.h
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
// Copyright (c) 2025 AIROHA Inc
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_ppe_dev {
    pub type_data): *mut c_void,
    pub rx_wlan): bool,
    pub ops: },
    pub priv: *mut c_void,
}

extern "C" {
    pub fn airoha_ppe_put_dev(dev: *mut airoha_ppe_dev);
}

pub const NPU_NUM_CORES: c_int = 8;
pub const NPU_NUM_IRQ: c_int = 6;
pub const NPU_RX0_DESC_NUM: c_int = 512;
pub const NPU_RX1_DESC_NUM: c_int = 512;
// CTRL

// INFO

// DATA

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_npu_rx_dma_desc {
    pub ctrl: u32,
    pub info: u32,
    pub data: u32,
    pub addr: u32,
    pub rsv: u64,
    pub __packed: },
// CTRL

pub const NPU_TXWI_LEN: c_int = 192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_npu_tx_dma_desc {
    pub ctrl: u32,
    pub addr: u32,
    pub rsv: u64,
    pub txwi: [u8; NPU_TXWI_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_npu_wlan_set_cmd {
    WLAN_FUNC_SET_WAIT_PCIE_ADDR,
    WLAN_FUNC_SET_WAIT_DESC,
    WLAN_FUNC_SET_WAIT_NPU_INIT_DONE,
    WLAN_FUNC_SET_WAIT_TRAN_TO_CPU,
    WLAN_FUNC_SET_WAIT_BA_WIN_SIZE,
    WLAN_FUNC_SET_WAIT_DRIVER_MODEL,
    WLAN_FUNC_SET_WAIT_DEL_STA,
    WLAN_FUNC_SET_WAIT_DRAM_BA_NODE_ADDR,
    WLAN_FUNC_SET_WAIT_PKT_BUF_ADDR,
    WLAN_FUNC_SET_WAIT_IS_TEST_NOBA,
    WLAN_FUNC_SET_WAIT_FLUSHONE_TIMEOUT,
    WLAN_FUNC_SET_WAIT_FLUSHALL_TIMEOUT,
    WLAN_FUNC_SET_WAIT_IS_FORCE_TO_CPU,
    WLAN_FUNC_SET_WAIT_PCIE_STATE,
    WLAN_FUNC_SET_WAIT_PCIE_PORT_TYPE,
    WLAN_FUNC_SET_WAIT_ERROR_RETRY_TIMES,
    WLAN_FUNC_SET_WAIT_BAR_INFO,
    WLAN_FUNC_SET_WAIT_FAST_FLAG,
    WLAN_FUNC_SET_WAIT_NPU_BAND0_ONCPU,
    WLAN_FUNC_SET_WAIT_TX_RING_PCIE_ADDR,
    WLAN_FUNC_SET_WAIT_TX_DESC_HW_BASE,
    WLAN_FUNC_SET_WAIT_TX_BUF_SPACE_HW_BASE,
    WLAN_FUNC_SET_WAIT_RX_RING_FOR_TXDONE_HW_BASE,
    WLAN_FUNC_SET_WAIT_TX_PKT_BUF_ADDR,
    WLAN_FUNC_SET_WAIT_INODE_TXRX_REG_ADDR,
    WLAN_FUNC_SET_WAIT_INODE_DEBUG_FLAG,
    WLAN_FUNC_SET_WAIT_INODE_HW_CFG_INFO,
    WLAN_FUNC_SET_WAIT_INODE_STOP_ACTION,
    WLAN_FUNC_SET_WAIT_INODE_PCIE_SWAP,
    WLAN_FUNC_SET_WAIT_RATELIMIT_CTRL,
    WLAN_FUNC_SET_WAIT_HWNAT_INIT,
    WLAN_FUNC_SET_WAIT_ARHT_CHIP_INFO,
    WLAN_FUNC_SET_WAIT_TX_BUF_CHECK_ADDR,
    WLAN_FUNC_SET_WAIT_TOKEN_ID_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum airoha_npu_wlan_get_cmd {
    WLAN_FUNC_GET_WAIT_NPU_INFO,
    WLAN_FUNC_GET_WAIT_LAST_RATE,
    WLAN_FUNC_GET_WAIT_COUNTER,
    WLAN_FUNC_GET_WAIT_DBG_COUNTER,
    WLAN_FUNC_GET_WAIT_RXDESC_BASE,
    WLAN_FUNC_GET_WAIT_WCID_DBG_COUNTER,
    WLAN_FUNC_GET_WAIT_DMA_ADDR,
    WLAN_FUNC_GET_WAIT_RING_SIZE,
    WLAN_FUNC_GET_WAIT_NPU_SUPPORT_MAP,
    WLAN_FUNC_GET_WAIT_MDC_LOCK_ADDRESS,
    WLAN_FUNC_GET_WAIT_NPU_VERSION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_npu {

    pub dev: *mut device,
    pub regmap: *mut regmap,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct airoha_npu_core {
    pub npu: *mut airoha_npu,
// protect concurrent npu memory accesses
    pub lock: spinlock_t,
    pub wdt_work: work_struct,
    pub cores: [}; NPU_NUM_CORES],
    pub irqs: [c_int; NPU_NUM_IRQ],
    pub stats: *mut airoha_foe_stats __iomem,
    pub npu): *mut *mut int (ppe_init)(struct airoha_npu,
    pub npu): *mut *mut int (ppe_deinit)(struct airoha_npu,
    pub num_stats_entries): dma_addr_t addr, u32,
    pub sram_num_entries): c_int,
    pub ppe2): bool,
    pub npu): *mut *mut int (wlan_init_reserved_memory)(struct airoha_npu,
    pub gfp): *mut *mut void data, int data_len, gfp_t,
    pub gfp): *mut *mut void data, int data_len, gfp_t,
    pub xmit): bool,
    pub val): *mut *mut *mut void (wlan_set_irq_status)(struct airoha_npu npu, u32,
    pub q): *mut *mut *mut u32 (wlan_get_irq_status)(struct airoha_npu npu, int,
    pub q): *mut *mut *mut void (wlan_enable_irq)(struct airoha_npu npu, int,
    pub q): *mut *mut *mut void (wlan_disable_irq)(struct airoha_npu npu, int,
    pub ops: },

}

extern "C" {
    pub fn airoha_npu_put(npu: *mut airoha_npu);
}

