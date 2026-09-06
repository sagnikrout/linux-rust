//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/amd/acp.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021, 2023 Advanced Micro Devices, Inc. All rights reserved.
//
// Author: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

pub const ACP_MAX_STREAM: c_int = 8;
pub const ACP_DSP_BAR: c_int = 0;
pub const ACP_HW_SEM_RETRY_COUNT: c_int = 10000;
pub const ACP_REG_POLL_INTERVAL: c_int = 500;
pub const ACP_REG_POLL_TIMEOUT_US: c_int = 2000;
pub const ACP_DMA_COMPLETE_TIMEOUT_US: c_int = 5000;
pub const ACP3X_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x01;
pub const ACP3X_PGFSM_STATUS_MASK: c_uint = 0x03;
pub const ACP6X_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x07;
pub const ACP6X_PGFSM_STATUS_MASK: c_uint = 0x0F;
pub const ACP70_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x1F;
pub const ACP70_PGFSM_STATUS_MASK: c_uint = 0xFF;
pub const ACP_POWERED_ON: c_uint = 0x00;
pub const ACP_ASSERT_RESET: c_uint = 0x01;
pub const ACP_RELEASE_RESET: c_uint = 0x00;
pub const ACP_SOFT_RESET_DONE_MASK: c_uint = 0x00010001;
pub const ACP_DSP_ASSERT_RESET: c_uint = 0x04;
pub const ACP_DSP_RELEASE_RESET: c_uint = 0x00;
pub const ACP_DSP_SOFT_RESET_DONE_MASK: c_uint = 0x00050004;
pub const ACP_DSP_INTR_EN_MASK: c_uint = 0x00000001;
pub const ACP3X_SRAM_PTE_OFFSET: c_uint = 0x02050000;
pub const ACP5X_SRAM_PTE_OFFSET: c_uint = 0x02050000;
pub const ACP6X_SRAM_PTE_OFFSET: c_uint = 0x03800000;

pub const PAGE_SIZE_4K_ENABLE: c_uint = 0x2;
pub const ACP_PAGE_SIZE: c_uint = 0x1000;
pub const ACP_DMA_CH_RUN: c_uint = 0x02;
pub const ACP_MAX_DESC_CNT: c_uint = 0x02;
pub const DSP_FW_RUN_ENABLE: c_uint = 0x01;
pub const ACP_SHA_RUN: c_uint = 0x01;
pub const ACP_SHA_RESET: c_uint = 0x02;
pub const ACP_SHA_HEADER: c_uint = 0x01;
pub const ACP_DMA_CH_RST: c_uint = 0x01;
pub const ACP_DMA_CH_GRACEFUL_RST_EN: c_uint = 0x10;
pub const ACP_ATU_CACHE_INVALID: c_uint = 0x01;
pub const ACP_MAX_DESC: c_int = 128;

pub const ACP_DEFAULT_DRAM_LENGTH: c_uint = 0x00080000;
pub const ACP3X_SCRATCH_MEMORY_ADDRESS: c_uint = 0x02050000;
pub const ACP_SYSTEM_MEMORY_WINDOW: c_uint = 0x4000000;
pub const ACP_IRAM_BASE_ADDRESS: c_uint = 0x000000;
pub const ACP_DRAM_BASE_ADDRESS: c_uint = 0x01000000;
pub const ACP_DRAM_PAGE_COUNT: c_int = 128;
pub const ACP_SRAM_BASE_ADDRESS: c_uint = 0x3806000;
pub const ACP7X_SRAM_BASE_ADDRESS: c_uint = 0x380C000;
pub const ACP_DSP_TO_HOST_IRQ: c_uint = 0x04;
pub const ACP_RN_PCI_ID: c_uint = 0x01;
pub const ACP_VANGOGH_PCI_ID: c_uint = 0x50;
pub const ACP_RMB_PCI_ID: c_uint = 0x6F;
pub const ACP63_PCI_ID: c_uint = 0x63;
pub const ACP70_PCI_ID: c_uint = 0x70;
pub const ACP71_PCI_ID: c_uint = 0x71;
pub const ACP72_PCI_ID: c_uint = 0x72;
pub const ACP7B_PCI_ID: c_uint = 0x7B;
pub const ACP7F_PCI_ID: c_uint = 0x7F;
pub const ACP7X_PGFSM_CNTL_POWER_ON_MASK: c_uint = 0x7F;
pub const ACP7X_PGFSM_STATUS_MASK: c_uint = 0xFFF;

pub const HOST_BRIDGE_CZN: c_uint = 0x1630;
pub const HOST_BRIDGE_VGH: c_uint = 0x1645;
pub const HOST_BRIDGE_RMB: c_uint = 0x14B5;
pub const HOST_BRIDGE_ACP63: c_uint = 0x14E8;
pub const HOST_BRIDGE_ACP70: c_uint = 0x1507;
pub const ACP_SHA_STAT: c_uint = 0x8000;
pub const ACP_PSP_TIMEOUT_US: c_int = 1000000;
pub const ACP_EXT_INTR_ERROR_STAT: c_uint = 0x20000000;
pub const MP0_C2PMSG_114_REG: c_uint = 0x3810AC8;
pub const MP0_C2PMSG_73_REG: c_uint = 0x3810A24;
pub const MBOX_ACP_SHA_DMA_COMMAND: c_uint = 0x70000;
pub const MBOX_ACP_IRAM_DRAM_FENCE_COMMAND: c_uint = 0x80000;
pub const MBOX_DELAY_US: c_int = 1000;
pub const MBOX_READY_MASK: c_uint = 0x80000000;
pub const MBOX_STATUS_MASK: c_uint = 0xFFFF;
pub const MBOX_ISREADY_FLAG: c_uint = 0x40000000;

pub const BOX_SIZE_512: c_uint = 0x200;
pub const BOX_SIZE_1024: c_uint = 0x400;
pub const EXCEPT_MAX_HDR_SIZE: c_uint = 0x400;
pub const AMD_STACK_DUMP_SIZE: c_int = 32;
pub const SRAM1_SIZE: c_uint = 0x280000;

pub const ACP_FIRMWARE_SIGNATURE: c_uint = 0x100;

pub const ACP_IMAGE_HDR_SIZE_FW_SIGNED_OFF: c_uint = 0x14;

pub const SDW_ACPI_ADDR_ACP63: c_int = 5;

pub const ACP_DEFAULT_SRAM_LENGTH: c_uint = 0x00080000;
pub const ACP_SRAM_PAGE_COUNT: c_int = 128;
pub const ACP6X_SDW_MAX_MANAGER_COUNT: c_int = 2;

pub const ACP_DSP_MSG_SET: c_int = 1;
pub const ACP_DSP_ACK_SET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clock_source {
    ACP_CLOCK_96M = 0,
    ACP_CLOCK_48M,
    ACP_CLOCK_24M,
    ACP_CLOCK_ACLK,
    ACP_CLOCK_MCLK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_atu_grp_pte {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dma_tx_cnt {
    pub 19: unsigned int count :,
    pub 12: unsigned int reserved :,
    pub 1: unsigned ioc :,
    pub bits: } bitfields,,
    pub u32_all: c_uint,
    pub i32_all: signed int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_descriptor {
    pub src_addr: c_uint,
    pub dest_addr: c_uint,
    pub tx_cnt: dma_tx_cnt,
    pub reserved: c_uint,
}

// Scratch memory structure for communication b/w host and dsp
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scratch_ipc_conf {
// Debug memory
    pub sof_debug_box: [u8; 1024],
// Exception memory
    pub sof_except_box: [u8; 1024],
// Stream buffer
    pub sof_stream_box: [u8; 1024],
// Trace buffer
    pub sof_trace_box: [u8; 1024],
// Host msg flag
    pub sof_host_msg_write: u32,
// Host ack flag
    pub sof_host_ack_write: u32,
// DSP msg flag
    pub sof_dsp_msg_write: u32,
// Dsp ack flag
    pub sof_dsp_ack_write: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scratch_reg_conf {
    pub info: scratch_ipc_conf,
    pub grp1_pte: [acp_atu_grp_pte; 16],
    pub grp2_pte: [acp_atu_grp_pte; 16],
    pub grp3_pte: [acp_atu_grp_pte; 16],
    pub grp4_pte: [acp_atu_grp_pte; 16],
    pub grp5_pte: [acp_atu_grp_pte; 16],
    pub grp6_pte: [acp_atu_grp_pte; 16],
    pub grp7_pte: [acp_atu_grp_pte; 16],
    pub grp8_pte: [acp_atu_grp_pte; 16],
    pub dma_desc: [dma_descriptor; 64],
    pub reg_offset: [c_uint; 8],
    pub buf_size: [c_uint; 8],
    pub acp_tx_fifo_buf: [u8; 256],
    pub acp_rx_fifo_buf: [u8; 256],
    pub reserve: [c_uint; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_dsp_stream {
    pub list: list_head,
    pub sdev: *mut snd_sof_dev,
    pub substream: *mut snd_pcm_substream,
    pub dmab: *mut snd_dma_buffer,
    pub num_pages: c_int,
    pub stream_tag: c_int,
    pub active: c_int,
    pub reg_offset: c_uint,
    pub posn_offset: usize,
    pub cstream: *mut snd_compr_stream,
    pub cstream_posn: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_amd_acp_desc {
    pub name: *const c_char,
    pub pgfsm_base: u32,
    pub ext_intr_enb: u32,
    pub ext_intr_cntl: u32,
    pub ext_intr_stat: u32,
    pub ext_intr_stat1: u32,
    pub dsp_intr_base: u32,
    pub sram_pte_offset: u32,
    pub hw_semaphore_offset: u32,
    pub acp_clkmux_sel: u32,
    pub fusion_dsp_offset: u32,
    pub probe_reg_offset: u32,
    pub reg_start_addr: u32,
    pub reg_end_addr: u32,
    pub acp_error_stat: u32,
    pub acp_sw0_i2s_err_reason: u32,
    pub sdw_max_link_count: u32,
    pub sdw_acpi_dev_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_quirk_entry {
    pub signed_fw_image: bool,
    pub skip_iram_dram_size_mod: bool,
    pub post_fw_run_delay: bool,
}

// Common device data struct for ACP devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_dev_data {
    pub dev: *mut snd_sof_dev,
    pub fw_dbin: *const firmware,
// DMIC device
    pub dmic_dev: *mut platform_device,
// mutex lock to protect ACP common registers access
    pub acp_lock: mutex,
// ACPI information stored between scan and probe steps
    pub info: sdw_amd_acpi_info,
// sdw context allocated by SoundWire driver
    pub sdw: *mut sdw_amd_ctx,
    pub fw_bin_size: c_uint,
    pub fw_data_bin_size: c_uint,
    pub fw_sram_data_bin_size: c_uint,
    pub fw_code_bin: *const c_char,
    pub fw_data_bin: *const c_char,
    pub fw_sram_data_bin: *const c_char,
    pub fw_bin_page_count: u32,
    pub fw_data_bin_page_count: u32,
    pub addr: u32,
    pub reg_range: u32,
    pub blk_type: u32,
    pub sha_dma_addr: dma_addr_t,
    pub bin_buf: *mut u8,
    pub dma_addr: dma_addr_t,
    pub data_buf: *mut u8,
    pub sram_dma_addr: dma_addr_t,
    pub sram_data_buf: *mut u8,
    pub quirks: *mut acp_quirk_entry,
    pub dscr_info: [dma_descriptor; ACP_MAX_DESC],
    pub stream_buf: [acp_dsp_stream; ACP_MAX_STREAM],
    pub dtrace_stream: *mut acp_dsp_stream,
    pub probe_stream: *mut acp_dsp_stream,
    pub enable_fw_debug: bool,
    pub is_dram_in_use: bool,
    pub is_sram_in_use: bool,
    pub sdw_en_stat: bool,
// acp70_sdw0_wake_event flag set to true when wake irq asserted for SW0 instance
    pub acp70_sdw0_wake_event: bool,
// acp70_sdw1_wake_event flag set to true when wake irq asserted for SW1 instance
    pub acp70_sdw1_wake_event: bool,
    pub pci_rev: c_uint,
    pub acp_sof_signed_firmware_image: c_int,
}

extern "C" {
    pub fn memcpy_to_scratch(sdev: *mut snd_sof_dev, offset: u32, src: *mut c_uint, bytes: usize);
}
extern "C" {
    pub fn memcpy_from_scratch(sdev: *mut snd_sof_dev, offset: u32, dst: *mut c_uint, bytes: usize);
}
extern "C" {
    pub fn acp_dma_status(adata: *mut acp_dev_data, ch: c_uchar) -> c_int;
}
// ACP device probe/remove
extern "C" {
    pub fn amd_sof_acp_probe(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn amd_sof_acp_remove(sdev: *mut snd_sof_dev);
}
// DSP Loader callbacks
extern "C" {
    pub fn acp_sof_dsp_run(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn acp_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn acp_sof_load_signed_firmware(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn acp_get_bar_index(sdev: *mut snd_sof_dev, type: u32) -> c_int;
}
// Block IO callbacks
// IPC callbacks
extern "C" {
    pub fn acp_sof_ipc_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn acp_sof_ipc_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn acp_sof_ipc_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;
}
extern "C" {
    pub fn acp_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, message: *mut c_void, bytes: usize);
}
extern "C" {
    pub fn acp_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, message: *mut c_void, bytes: usize);
}
// ACP - DSP  stream callbacks
extern "C" {
    pub fn acp_dsp_stream_config(sdev: *mut snd_sof_dev, stream: *mut acp_dsp_stream) -> c_int;
}
extern "C" {
    pub fn acp_dsp_stream_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn acp_dsp_stream_put(sdev: *mut snd_sof_dev, acp_stream: *mut acp_dsp_stream) -> c_int;
}
//
// DSP PCM Operations.
//
extern "C" {
    pub fn acp_pcm_open(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn acp_pcm_close(sdev: *mut snd_sof_dev, substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn sof_renoir_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_vangogh_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_rembrandt_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_acp63_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_acp70_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_acp7x_ops_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn amd_sof_acp7x_probe(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn amd_sof_acp7x_remove(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn amd_sof_acp7x_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int;
}
extern "C" {
    pub fn amd_sof_acp7x_resume(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn amd_sof_acp7x_suspend_runtime(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn amd_sof_acp7x_resume_runtime(sdev: *mut snd_sof_dev) -> c_int;
}
// Machine configuration
extern "C" {
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;
}
// Trace
extern "C" {
    pub fn acp_sof_trace_release(sdev: *mut snd_sof_dev) -> c_int;
}
// PM Callbacks
extern "C" {
    pub fn amd_sof_acp_suspend(sdev: *mut snd_sof_dev, target_state: u32) -> c_int;
}
extern "C" {
    pub fn amd_sof_acp_resume(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn amd_sof_ipc_dump(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn amd_sof_dump(sdev: *mut snd_sof_dev, flags: u32);
}
extern "C" {
    pub fn acp_probes_register(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn acp_probes_unregister(sdev: *mut snd_sof_dev);
}
