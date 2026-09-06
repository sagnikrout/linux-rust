//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/ahb.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const ATH12K_AHB_SMP2P_SMEM_VALUE_MASK: c_uint = 0xFFFFFFFF;
pub const ATH12K_PCI_CE_WAKE_IRQ: c_int = 2;
pub const ATH12K_PCI_IRQ_CE0_OFFSET: c_int = 3;

pub const ATH12K_AHB_UPD_SWID: c_uint = 0x12;

pub const ATH12K_USERPD_FW_NAME_LEN: c_int = 35;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_ahb_userpd_id {
    ATH12K_AHB_USERPD_ID_0 = 1,
    ATH12K_AHB_USERPD_ID_1,
    ATH12K_AHB_USERPD_ID_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ahb_userpd_map {
    pub io_start: phys_addr_t,
    pub node_name: *const c_char,
    pub upd_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ahb_desc {
    pub hw_rev: ath12k_hw_rev,
    pub auth_enabled: bool,
    pub ops: *const ath12k_hif_ops,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_ahb_smp2p_msg_id {
    ATH12K_AHB_POWER_SAVE_ENTER = 1,
    ATH12K_AHB_POWER_SAVE_EXIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath12k_ahb_userpd_irq {
    ATH12K_USERPD_SPAWN_IRQ,
    ATH12K_USERPD_READY_IRQ,
    ATH12K_USERPD_STOP_ACK_IRQ,
    ATH12K_USERPD_MAX_IRQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ahb_device_family_ops {
    pub pdev): *mut *mut int (probe)(struct platform_device,
    pub ab): *mut *mut int (arch_init)(struct ath12k_base,
    pub ab): *mut *mut void (arch_deinit)(struct ath12k_base,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ahb_rproc_info {
    pub tgt_rproc: *mut rproc,
    pub root_pd_nb: notifier_block,
    pub root_pd_notifier: *mut c_void,
    pub rootpd_ready: completion,
    pub num_userpd: u8,
    pub rootpd_booted_by_driver: bool,
    pub userpd: [*mut ath12k_ahb; ATH12K_MAX_DEVICES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ahb {
    pub ab: *mut ath12k_base,
    pub xo_clk: *mut clk,
    pub spawn_state: *mut qcom_smem_state,
    pub stop_state: *mut qcom_smem_state,
    pub userpd_spawned: completion,
    pub userpd_ready: completion,
    pub userpd_stopped: completion,
    pub userpd_id: u32,
    pub spawn_bit: u32,
    pub stop_bit: u32,
    pub userpd_irq_num: [c_int; ATH12K_USERPD_MAX_IRQ],
    pub ahb_ops: *const ath12k_ahb_ops,
    pub device_family_ops: *const ath12k_ahb_device_family_ops,
    pub scm_auth_enabled: bool,
    pub rproc_info: *mut ath12k_ahb_rproc_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ahb_driver {
    pub name: *const c_char,
    pub id_table: *const of_device_id,
    pub ops: ath12k_ahb_device_family_ops,
    pub driver: platform_driver,
}

extern "C" {
    pub fn ath12k_ahb_unregister_driver(device_id: ath12k_device_family);
}
