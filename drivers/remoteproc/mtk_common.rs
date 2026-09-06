//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/remoteproc/mtk_common.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2019 MediaTek Inc.
//

pub const MT8183_SW_RSTN: c_uint = 0x0;

pub const MT8183_SCP_TO_HOST: c_uint = 0x1C;

pub const MT8183_HOST_TO_SCP: c_uint = 0x28;

pub const MT8183_WDT_CFG: c_uint = 0x84;
pub const MT8183_SCP_CLK_SW_SEL: c_uint = 0x4000;
pub const MT8183_SCP_CLK_DIV_SEL: c_uint = 0x4024;
pub const MT8183_SCP_SRAM_PDN: c_uint = 0x402C;
pub const MT8183_SCP_L1_SRAM_PD: c_uint = 0x4080;
pub const MT8183_SCP_TCM_TAIL_SRAM_PD: c_uint = 0x4094;

pub const MT8186_SCP_L1_SRAM_PD_P1: c_uint = 0x40B0;
pub const MT8186_SCP_L1_SRAM_PD_p2: c_uint = 0x40B4;
pub const MT8192_L2TCM_SRAM_PD_0: c_uint = 0x10C0;
pub const MT8192_L2TCM_SRAM_PD_1: c_uint = 0x10C4;
pub const MT8192_L2TCM_SRAM_PD_2: c_uint = 0x10C8;
pub const MT8192_L1TCM_SRAM_PDN: c_uint = 0x102C;
pub const MT8192_CPU0_SRAM_PD: c_uint = 0x1080;
pub const MT8192_SCP2APMCU_IPC_SET: c_uint = 0x4080;
pub const MT8192_SCP2APMCU_IPC_CLR: c_uint = 0x4084;

pub const MT8192_SCP2SPM_IPC_CLR: c_uint = 0x4094;
pub const MT8192_GIPC_IN_SET: c_uint = 0x4098;

pub const MT8192_CORE0_SW_RSTN_CLR: c_uint = 0x10000;
pub const MT8192_CORE0_SW_RSTN_SET: c_uint = 0x10004;
pub const MT8192_CORE0_MEM_ATT_PREDEF: c_uint = 0x10008;
pub const MT8192_CORE0_WDT_IRQ: c_uint = 0x10030;
pub const MT8192_CORE0_WDT_CFG: c_uint = 0x10034;
pub const MT8195_SYS_STATUS: c_uint = 0x4004;

pub const MT8195_CPU1_SRAM_PD: c_uint = 0x1084;
pub const MT8195_SSHUB2APMCU_IPC_SET: c_uint = 0x4088;
pub const MT8195_SSHUB2APMCU_IPC_CLR: c_uint = 0x408C;
pub const MT8195_CORE1_SW_RSTN_CLR: c_uint = 0x20000;
pub const MT8195_CORE1_SW_RSTN_SET: c_uint = 0x20004;
pub const MT8195_CORE1_MEM_ATT_PREDEF: c_uint = 0x20008;
pub const MT8195_CORE1_WDT_IRQ: c_uint = 0x20030;
pub const MT8195_CORE1_WDT_CFG: c_uint = 0x20034;
pub const MT8195_SEC_CTRL: c_uint = 0x85000;

pub const MT8195_L2TCM_OFFSET_RANGE_0_LOW: c_uint = 0x850b0;
pub const MT8195_L2TCM_OFFSET_RANGE_0_HIGH: c_uint = 0x850b4;
pub const MT8195_L2TCM_OFFSET: c_uint = 0x850d0;
pub const SCP_FW_VER_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scp_run {
    pub signaled: u32,
    pub fw_ver: [i8; SCP_FW_VER_LEN],
    pub dec_capability: u32,
    pub enc_capability: u32,
    pub wq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scp_ipi_desc {
// For protecting handler.
    pub lock: mutex,
    pub handler: scp_ipi_handler_t,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_scp_sizes_data {
    pub max_dram_size: usize,
    pub ipi_share_buffer_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_scp_of_data {
    pub scp): *mut *mut int (scp_clk_get)(struct mtk_scp,
    pub scp): *mut *mut int (scp_before_load)(struct mtk_scp,
    pub scp): *mut *mut void (scp_irq_handler)(struct mtk_scp,
    pub scp): *mut *mut void (scp_reset_assert)(struct mtk_scp,
    pub scp): *mut *mut void (scp_reset_deassert)(struct mtk_scp,
    pub scp): *mut *mut void (scp_stop)(struct mtk_scp,
    pub len): *mut *mut *mut *mut void (scp_da_to_va)(struct mtk_scp scp, u64 da, size_t,
    pub host_to_scp_reg: u32,
    pub host_to_scp_int_bit: u32,
    pub ipi_buf_offset: usize,
    pub scp_sizes: *const mtk_scp_sizes_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_scp_of_cluster {
    pub reg_base: *mut void __iomem,
    pub l1tcm_base: *mut void __iomem,
    pub l1tcm_size: usize,
    pub l1tcm_phys: phys_addr_t,
    pub mtk_scp_list: list_head,
// Prevent concurrent operations of this structure and L2TCM power control.
    pub cluster_lock: mutex,
    pub l2tcm_refcnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_scp {
    pub dev: *mut device,
    pub rproc: *mut rproc,
    pub clk: *mut clk,
    pub sram_base: *mut void __iomem,
    pub sram_size: usize,
    pub sram_phys: phys_addr_t,
    pub data: *const mtk_scp_of_data,
    pub recv_buf: *mut mtk_share_obj __iomem,
    pub send_buf: *mut mtk_share_obj __iomem,
    pub run: scp_run,
// To prevent multiple ipi_send run concurrently.
    pub send_lock: mutex,
    pub ipi_desc: [scp_ipi_desc; SCP_IPI_MAX],
    pub ipi_id_ack: [bool; SCP_IPI_MAX],
    pub ack_wq: wait_queue_head_t,
    pub share_buf: *mut u8,
    pub cpu_addr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub rpmsg_subdev: *mut rproc_subdev,
    pub elem: list_head,
    pub cluster: *mut mtk_scp_of_cluster,
}

//
// struct mtk_share_obj - SRAM buffer shared with AP and SCP
//
// @id:		IPI id
// @len:	share buffer length
// @share_buf:	share buffer data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_share_obj {
    pub id: u32,
    pub len: u32,
    pub share_buf: *mut u8,
}

extern "C" {
    pub fn scp_memcpy_aligned(dst: *mut void __iomem, src: *const c_void, len: c_uint);
}
extern "C" {
    pub fn scp_ipi_lock(scp: *mut mtk_scp, id: u32);
}
extern "C" {
    pub fn scp_ipi_unlock(scp: *mut mtk_scp, id: u32);
}
