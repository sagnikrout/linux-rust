//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/pcie.h
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
// @file mwifiex_pcie.h
//
// @brief This file contains definitions for PCI-E interface.
// driver.
//
// Copyright 2011-2020 NXP
//

pub const PCIE8897_A0: c_uint = 0x1100;
pub const PCIE8897_B0: c_uint = 0x1200;
pub const PCIE8997_A0: c_uint = 0x10;
pub const PCIE8997_A1: c_uint = 0x11;
pub const CHIP_VER_PCIEUART: c_uint = 0x3;
pub const CHIP_MAGIC_VALUE: c_uint = 0x24;
// Constants for Buffer Descriptor (BD) rings
pub const MWIFIEX_MAX_TXRX_BD: c_uint = 0x20;
pub const MWIFIEX_TXBD_MASK: c_uint = 0x3F;
pub const MWIFIEX_RXBD_MASK: c_uint = 0x3F;
pub const MWIFIEX_MAX_EVT_BD: c_uint = 0x08;
pub const MWIFIEX_EVTBD_MASK: c_uint = 0x0f;
// PCIE INTERNAL REGISTERS
pub const PCIE_SCRATCH_0_REG: c_uint = 0xC10;
pub const PCIE_SCRATCH_1_REG: c_uint = 0xC14;
pub const PCIE_CPU_INT_EVENT: c_uint = 0xC18;
pub const PCIE_CPU_INT_STATUS: c_uint = 0xC1C;
pub const PCIE_HOST_INT_STATUS: c_uint = 0xC30;
pub const PCIE_HOST_INT_MASK: c_uint = 0xC34;
pub const PCIE_HOST_INT_STATUS_MASK: c_uint = 0xC3C;
pub const PCIE_SCRATCH_2_REG: c_uint = 0xC40;
pub const PCIE_SCRATCH_3_REG: c_uint = 0xC44;
pub const PCIE_SCRATCH_4_REG: c_uint = 0xCD0;
pub const PCIE_SCRATCH_5_REG: c_uint = 0xCD4;
pub const PCIE_SCRATCH_6_REG: c_uint = 0xCD8;
pub const PCIE_SCRATCH_7_REG: c_uint = 0xCDC;
pub const PCIE_SCRATCH_8_REG: c_uint = 0xCE0;
pub const PCIE_SCRATCH_9_REG: c_uint = 0xCE4;
pub const PCIE_SCRATCH_10_REG: c_uint = 0xCE8;
pub const PCIE_SCRATCH_11_REG: c_uint = 0xCEC;
pub const PCIE_SCRATCH_12_REG: c_uint = 0xCF0;
pub const PCIE_SCRATCH_13_REG: c_uint = 0xCF4;
pub const PCIE_SCRATCH_14_REG: c_uint = 0xCF8;
pub const PCIE_SCRATCH_15_REG: c_uint = 0xCFC;
pub const PCIE_RD_DATA_PTR_Q0_Q1: c_uint = 0xC08C;
pub const PCIE_WR_DATA_PTR_Q0_Q1: c_uint = 0xC05C;

// Max retry number of command write
pub const MAX_WRITE_IOMEM_RETRY: c_int = 2;
// Define PCIE block size for firmware download
pub const MWIFIEX_PCIE_BLOCK_SIZE_FW_DNLD: c_int = 256;
// FW awake cookie after FW ready

pub const MWIFIEX_DEF_SLEEP_COOKIE: c_uint = 0xBEEFBEEF;
pub const MWIFIEX_SLEEP_COOKIE_SIZE: c_int = 4;
pub const MWIFIEX_MAX_DELAY_COUNT: c_int = 100;
pub const MWIFIEX_PCIE_FLR_HAPPENS: c_uint = 0xFEDCBABA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_pcie_card_reg {
    pub cmd_addr_lo: u16,
    pub cmd_addr_hi: u16,
    pub fw_status: u16,
    pub cmd_size: u16,
    pub cmdrsp_addr_lo: u16,
    pub cmdrsp_addr_hi: u16,
    pub tx_rdptr: u16,
    pub tx_wrptr: u16,
    pub rx_rdptr: u16,
    pub rx_wrptr: u16,
    pub evt_rdptr: u16,
    pub evt_wrptr: u16,
    pub drv_rdy: u16,
    pub tx_start_ptr: u16,
    pub tx_mask: u32,
    pub tx_wrap_mask: u32,
    pub rx_mask: u32,
    pub rx_wrap_mask: u32,
    pub tx_rollover_ind: u32,
    pub rx_rollover_ind: u32,
    pub evt_rollover_ind: u32,
    pub ring_flag_sop: u8,
    pub ring_flag_eop: u8,
    pub ring_flag_xs_sop: u8,
    pub ring_flag_xs_eop: u8,
    pub ring_tx_start_ptr: u32,
    pub pfu_enabled: u8,
    pub sleep_cookie: u8,
    pub fw_dump_ctrl: u16,
    pub fw_dump_start: u16,
    pub fw_dump_end: u16,
    pub fw_dump_host_ready: u8,
    pub fw_dump_read_done: u8,
    pub msix_support: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_pcie_device {
    pub reg: *const mwifiex_pcie_card_reg,
    pub blksz_fw_dl: u16,
    pub tx_buf_size: u16,
    pub can_dump_fw: bool,
    pub mem_type_mapping_tbl: *mut memory_type_mapping,
    pub num_mem_types: u8,
    pub can_ext_scan: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_evt_buf_desc {
    pub paddr: u64,
    pub len: u16,
    pub flags: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_pcie_buf_desc {
    pub paddr: u64,
    pub len: u16,
    pub flags: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_pfu_buf_desc {
    pub flags: u16,
    pub offset: u16,
    pub frag_len: u16,
    pub len: u16,
    pub paddr: u64,
    pub reserved: u32,
    pub __packed: },
pub const MWIFIEX_NUM_MSIX_VECTORS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_msix_context {
    pub dev: *mut pci_dev,
    pub msg_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_service_card {
    pub dev: *mut pci_dev,
    pub adapter: *mut mwifiex_adapter,
    pub pcie: mwifiex_pcie_device,
    pub fw_done: completion,
    pub txbd_flush: u8,
    pub txbd_wrptr: u32,
    pub txbd_rdptr: u32,
    pub txbd_ring_size: u32,
    pub txbd_ring_vbase: *mut u8,
    pub txbd_ring_pbase: dma_addr_t,
    pub txbd_ring: [*mut c_void; MWIFIEX_MAX_TXRX_BD],
    pub tx_buf_list: [*mut sk_buff; MWIFIEX_MAX_TXRX_BD],
    pub rxbd_wrptr: u32,
    pub rxbd_rdptr: u32,
    pub rxbd_ring_size: u32,
    pub rxbd_ring_vbase: *mut u8,
    pub rxbd_ring_pbase: dma_addr_t,
    pub rxbd_ring: [*mut c_void; MWIFIEX_MAX_TXRX_BD],
    pub rx_buf_list: [*mut sk_buff; MWIFIEX_MAX_TXRX_BD],
    pub evtbd_wrptr: u32,
    pub evtbd_rdptr: u32,
    pub evtbd_ring_size: u32,
    pub evtbd_ring_vbase: *mut u8,
    pub evtbd_ring_pbase: dma_addr_t,
    pub evtbd_ring: [*mut c_void; MWIFIEX_MAX_EVT_BD],
    pub evt_buf_list: [*mut sk_buff; MWIFIEX_MAX_EVT_BD],
    pub cmd_buf: *mut sk_buff,
    pub cmdrsp_buf: *mut sk_buff,
    pub sleep_cookie_vbase: *mut u8,
    pub sleep_cookie_pbase: dma_addr_t,
    pub pci_mmap: *mut void __iomem,
    pub pci_mmap1: *mut void __iomem,
    pub msi_enable: c_int,
    pub msix_enable: c_int,
    pub msix_entries: [msix_entry; MWIFIEX_NUM_MSIX_VECTORS],    pub msix_ctx: [mwifiex_msix_context; MWIFIEX_NUM_MSIX_VECTORS],
    pub share_irq_ctx: mwifiex_msix_context,
    pub work: work_struct,
    pub work_flags: c_ulong,
    pub pci_reset_ongoing: bool,
    pub quirks: c_ulong,
}
