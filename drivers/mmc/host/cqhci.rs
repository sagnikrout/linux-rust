//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/cqhci.h
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
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
//

// registers
// version
pub const CQHCI_VER: c_uint = 0x00;

// capabilities
pub const CQHCI_CAP: c_uint = 0x04;
pub const CQHCI_CAP_CS: c_uint = 0x10000000 /* Crypto Support */;

// configuration
pub const CQHCI_CFG: c_uint = 0x08;
pub const CQHCI_DCMD: c_uint = 0x00001000;
pub const CQHCI_TASK_DESC_SZ: c_uint = 0x00000100;
pub const CQHCI_CRYPTO_GENERAL_ENABLE: c_uint = 0x00000002;
pub const CQHCI_ENABLE: c_uint = 0x00000001;
// control
pub const CQHCI_CTL: c_uint = 0x0C;
pub const CQHCI_CLEAR_ALL_TASKS: c_uint = 0x00000100;
pub const CQHCI_HALT: c_uint = 0x00000001;
// interrupt status
pub const CQHCI_IS: c_uint = 0x10;

// interrupt status enable
pub const CQHCI_ISTE: c_uint = 0x14;
// interrupt signal enable
pub const CQHCI_ISGE: c_uint = 0x18;
// interrupt coalescing
pub const CQHCI_IC: c_uint = 0x1C;

// task list base address
pub const CQHCI_TDLBA: c_uint = 0x20;
// task list base address upper
pub const CQHCI_TDLBAU: c_uint = 0x24;
// door-bell
pub const CQHCI_TDBR: c_uint = 0x28;
// task completion notification
pub const CQHCI_TCN: c_uint = 0x2C;
// device queue status
pub const CQHCI_DQS: c_uint = 0x30;
// device pending tasks
pub const CQHCI_DPT: c_uint = 0x34;
// task clear
pub const CQHCI_TCLR: c_uint = 0x38;
// task descriptor processing error
pub const CQHCI_TDPE: c_uint = 0x3c;
// send status config 1
pub const CQHCI_SSC1: c_uint = 0x40;

// send status config 2
pub const CQHCI_SSC2: c_uint = 0x44;
// response for dcmd
pub const CQHCI_CRDCT: c_uint = 0x48;
// response mode error mask
pub const CQHCI_RMEM: c_uint = 0x50;
// task error info
pub const CQHCI_TERRI: c_uint = 0x54;

// command response index
pub const CQHCI_CRI: c_uint = 0x58;
// command response argument
pub const CQHCI_CRA: c_uint = 0x5C;
// crypto capabilities
pub const CQHCI_CCAP: c_uint = 0x100;
pub const CQHCI_CRYPTOCAP: c_uint = 0x104;
pub const CQHCI_INT_ALL: c_uint = 0xF;
pub const CQHCI_IC_DEFAULT_ICCTH: c_int = 31;
pub const CQHCI_IC_DEFAULT_ICTOVAL: c_int = 1;
// attribute fields

// data command task descriptor fields

// direct command task descriptor fields

// crypto task descriptor fields (for bits 64-127 of task descriptor)

// transfer descriptor fields

// CCAP - Crypto Capability 100h
#[repr(C)]
#[derive(Copy, Clone)]
pub union cqhci_crypto_capabilities {
    pub reg_val: __le32,
    pub num_crypto_cap: u8,
    pub config_count: u8,
    pub reserved: u8,
    pub config_array_ptr: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqhci_crypto_key_size {
    CQHCI_CRYPTO_KEY_SIZE_INVALID	= 0,
    CQHCI_CRYPTO_KEY_SIZE_128	= 1,
    CQHCI_CRYPTO_KEY_SIZE_192	= 2,
    CQHCI_CRYPTO_KEY_SIZE_256	= 3,
    CQHCI_CRYPTO_KEY_SIZE_512	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqhci_crypto_alg {
    CQHCI_CRYPTO_ALG_AES_XTS		= 0,
    CQHCI_CRYPTO_ALG_BITLOCKER_AES_CBC	= 1,
    CQHCI_CRYPTO_ALG_AES_ECB		= 2,
    CQHCI_CRYPTO_ALG_ESSIV_AES_CBC		= 3,
}

// x-CRYPTOCAP - Crypto Capability X
#[repr(C)]
#[derive(Copy, Clone)]
pub union cqhci_crypto_cap_entry {
    pub reg_val: __le32,
    pub algorithm_id: u8,
    pub /: *mut *mut u8 sdus_mask; / Supported data unit size mask,
    pub key_size: u8,
    pub reserved: u8,
}

pub const CQHCI_CRYPTO_KEY_MAX_SIZE: c_int = 64;
// x-CRYPTOCFG - Crypto Configuration X
#[repr(C)]
#[derive(Copy, Clone)]
pub union cqhci_crypto_cfg_entry {
    pub reg_val: [__le32; 32],
    pub crypto_key: [u8; CQHCI_CRYPTO_KEY_MAX_SIZE],
    pub data_unit_size: u8,
    pub crypto_cap_idx: u8,
    pub reserved_1: u8,
    pub config_enable: u8,
    pub reserved_multi_host: u8,
    pub reserved_2: u8,
    pub vsb: [u8; 2],
    pub reserved_3: [u8; 56],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqhci_host {
    pub ops: *const cqhci_host_ops,
    pub mmio: *mut void __iomem,
    pub mmc: *mut mmc_host,
    pub lock: spinlock_t,
// relative card address of device
    pub rca: c_uint,
// 64 bit DMA
    pub dma64: bool,
    pub num_slots: c_int,
    pub qcnt: c_int,
    pub dcmd_slot: u32,
    pub caps: u32,
pub const CQHCI_TASK_DESC_SZ_128: c_uint = 0x1;
    pub quirks: u32,
pub const CQHCI_QUIRK_SHORT_TXFR_DESC_SZ: c_uint = 0x1;
    pub enabled: bool,
    pub halted: bool,
    pub init_done: bool,
    pub activated: bool,
    pub waiting_for_idle: bool,
    pub recovery_halt: bool,
    pub desc_size: usize,
    pub data_size: usize,
    pub desc_base: *mut u8,
// total descriptor size
    pub slot_sz: u8,
// 64/128 bit depends on CQHCI_CFG
    pub task_desc_len: u8,
// 64 bit on 32-bit arch, 128 bit on 64-bit
    pub link_desc_len: u8,
    pub trans_desc_base: *mut u8,
// same length as transfer descriptor
    pub trans_desc_len: u8,
    pub desc_dma_base: dma_addr_t,
    pub trans_desc_dma_base: dma_addr_t,
    pub halt_comp: completion,
    pub wait_queue: wait_queue_head_t,
    pub slot: *mut cqhci_slot,

    pub crypto_capabilities: cqhci_crypto_capabilities,
    pub crypto_cap_array: *mut cqhci_crypto_cap_entry,
    pub crypto_cfg_register: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqhci_host_ops {
    pub mmc): *mut *mut void (dumpregs)(struct mmc_host,
    pub reg): *mut *mut *mut void (write_l)(struct cqhci_host host, u32 val, int,
    pub reg): *mut *mut *mut u32 (read_l)(struct cqhci_host host, int,
    pub mmc): *mut *mut void (enable)(struct mmc_host,
    pub recovery): *mut *mut *mut void (disable)(struct mmc_host mmc, bool,
    pub data): *mut u64,
    pub mmc): *mut *mut void (pre_enable)(struct mmc_host,
    pub mmc): *mut *mut void (post_disable)(struct mmc_host,
    pub dma64): dma_addr_t addr, int len, bool end, bool,

    pub uses_custom_crypto_profile: bool,

}

extern "C" {
    pub fn readl_relaxed(reg: host->mmio +) -> return;
}
extern "C" {
    pub fn cqhci_irq(mmc: *mut mmc_host, cmd_error: c_int, data_error: c_int) -> irqreturn_t;
}
extern "C" {
    pub fn cqhci_init(cq_host: *mut cqhci_host, mmc: *mut mmc_host, dma64: bool) -> c_int;
}
extern "C" {
    pub fn cqhci_deactivate(mmc: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn cqhci_set_tran_desc(desc: *mut u8, addr: dma_addr_t, len: c_int, end: bool, dma64: bool);
}
extern "C" {
    pub fn cqhci_deactivate(_arg: mmc) -> return;
}
extern "C" {
    pub fn cqhci_resume(mmc: *mut mmc_host) -> c_int;
}
