//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/pci.h
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
// Copyright(c) 2009-2012  Realtek Corporation.

// 1: MSDU packet queue,
// 2: Rx Command Queue
//
pub const RTL_PCI_RX_MPDU_QUEUE: c_int = 0;
pub const RTL_PCI_RX_CMD_QUEUE: c_int = 1;
pub const RTL_PCI_MAX_RX_QUEUE: c_int = 2;

pub const RTL_PCI_MAX_TX_QUEUE_COUNT: c_int = 9;
pub const RT_TXDESC_NUM: c_int = 128;
pub const TX_DESC_NUM_92E: c_int = 512;
pub const TX_DESC_NUM_8822B: c_int = 512;
pub const RT_TXDESC_NUM_BE_QUEUE: c_int = 256;
pub const BK_QUEUE: c_int = 0;
pub const BE_QUEUE: c_int = 1;
pub const VI_QUEUE: c_int = 2;
pub const VO_QUEUE: c_int = 3;
pub const BEACON_QUEUE: c_int = 4;
pub const TXCMD_QUEUE: c_int = 5;
pub const MGNT_QUEUE: c_int = 6;
pub const HIGH_QUEUE: c_int = 7;
pub const HCCA_QUEUE: c_int = 8;

pub const INTEL_VENDOR_ID: c_uint = 0x8086;
pub const SIS_VENDOR_ID: c_uint = 0x1039;
pub const ATI_VENDOR_ID: c_uint = 0x1002;
pub const ATI_DEVICE_ID: c_uint = 0x7914;
pub const AMD_VENDOR_ID: c_uint = 0x1022;
pub const U1DONTCARE: c_uint = 0xFF;
pub const U2DONTCARE: c_uint = 0xFFFF;
pub const U4DONTCARE: c_uint = 0xFFFFFFFF;
pub const RTL_PCI_8192_DID: c_uint = 0x8192	/*8192 PCI-E */;
pub const RTL_PCI_8192SE_DID: c_uint = 0x8192	/*8192 SE */;
pub const RTL_PCI_8174_DID: c_uint = 0x8174	/*8192 SE */;
pub const RTL_PCI_8173_DID: c_uint = 0x8173	/*8191 SE Crab */;
pub const RTL_PCI_8172_DID: c_uint = 0x8172	/*8191 SE RE */;
pub const RTL_PCI_8171_DID: c_uint = 0x8171	/*8191 SE Unicron */;
pub const RTL_PCI_8723AE_DID: c_uint = 0x8723	/*8723AE */;
pub const RTL_PCI_0045_DID: c_uint = 0x0045	/*8190 PCI for Ceraga */;
pub const RTL_PCI_0046_DID: c_uint = 0x0046	/*8190 Cardbus for Ceraga */;
pub const RTL_PCI_0044_DID: c_uint = 0x0044	/*8192e PCIE for Ceraga */;
pub const RTL_PCI_0047_DID: c_uint = 0x0047	/*8192e Express Card for Ceraga */;
pub const RTL_PCI_700F_DID: c_uint = 0x700F;
pub const RTL_PCI_701F_DID: c_uint = 0x701F;
pub const RTL_PCI_DLINK_DID: c_uint = 0x3304;
pub const RTL_PCI_8723AE_DID: c_uint = 0x8723	/*8723e */;
pub const RTL_PCI_8192CET_DID: c_uint = 0x8191	/*8192ce */;
pub const RTL_PCI_8192CE_DID: c_uint = 0x8178	/*8192ce */;
pub const RTL_PCI_8191CE_DID: c_uint = 0x8177	/*8192ce */;
pub const RTL_PCI_8188CE_DID: c_uint = 0x8176	/*8192ce */;
pub const RTL_PCI_8192CU_DID: c_uint = 0x8191	/*8192ce */;
pub const RTL_PCI_8192DE_DID: c_uint = 0x8193	/*8192de */;
pub const RTL_PCI_8192DE_DID2: c_uint = 0x002B	/*92DE*/;
pub const RTL_PCI_8188EE_DID: c_uint = 0x8179  /*8188ee*/;
pub const RTL_PCI_8723BE_DID: c_uint = 0xB723  /*8723be*/;
pub const RTL_PCI_8192EE_DID: c_uint = 0x818B	/*8192ee*/;
pub const RTL_PCI_8821AE_DID: c_uint = 0x8821	/*8821ae*/;
pub const RTL_PCI_8812AE_DID: c_uint = 0x8812	/*8812ae*/;
pub const RTL_PCI_8822BE_DID: c_uint = 0xB822	/*8822be*/;
// 8192 support 16 pages of IO registers
pub const RTL_MEM_MAPPED_IO_RANGE_8190PCI: c_uint = 0x1000;
pub const RTL_MEM_MAPPED_IO_RANGE_8192PCIE: c_uint = 0x4000;
pub const RTL_MEM_MAPPED_IO_RANGE_8192SE: c_uint = 0x4000;
pub const RTL_MEM_MAPPED_IO_RANGE_8192CE: c_uint = 0x4000;
pub const RTL_MEM_MAPPED_IO_RANGE_8192DE: c_uint = 0x4000;
pub const RTL_PCI_REVISION_ID_8190PCI: c_uint = 0x00;
pub const RTL_PCI_REVISION_ID_8192PCIE: c_uint = 0x01;
pub const RTL_PCI_REVISION_ID_8192SE: c_uint = 0x10;
pub const RTL_PCI_REVISION_ID_8192CE: c_uint = 0x1;
pub const RTL_PCI_REVISION_ID_8192DE: c_uint = 0x0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_bridge_vendor {
    PCI_BRIDGE_VENDOR_INTEL = 0x0,	/*0b'0000,0001 */
    PCI_BRIDGE_VENDOR_ATI,		/*0b'0000,0010*/
    PCI_BRIDGE_VENDOR_AMD,		/*0b'0000,0100*/
    PCI_BRIDGE_VENDOR_SIS,		/*0b'0000,1000*/
    PCI_BRIDGE_VENDOR_UNKNOWN,	/*0b'0100,0000*/
    PCI_BRIDGE_VENDOR_MAX,
}

// In new TRX flow, Buffer_desc is new concept
// But TX wifi info == TX descriptor in old flow
// RX wifi info == RX descriptor in old flow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_tx_buffer_desc {
    pub 1))]: *mut *mut u32 dword[4  (1 << (BUFDESC_SEG_NUM +,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_tx_desc {
    pub dword: [u32; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_rx_buffer_desc {
    pub dword: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_rx_desc {
    pub dword: [u32; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_tx_cmd_desc {
    pub dword: [u32; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8192_tx_ring {
    pub desc: *mut rtl_tx_desc,
    pub dma: dma_addr_t,
    pub idx: c_uint,
    pub entries: c_uint,
    pub queue: sk_buff_head,
// add for new trx flow
    pub descriptor*/: *mut *mut *mut rtl_tx_buffer_desc buffer_desc; /tx buffer,
    pub memory*/: *mut *mut dma_addr_t buffer_desc_dma; /tx bufferd desc dma,
    pub /: *mut *mut u16 cur_tx_wp; / current_tx_write_point,
    pub /: *mut *mut u16 cur_tx_rp; / current_tx_read_point,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8192_rx_ring {
    pub desc: *mut rtl_rx_desc,
    pub dma: dma_addr_t,
    pub idx: c_uint,
    pub rx_buf: [*mut sk_buff; RTL_PCI_MAX_RX_COUNT],
// add for new trx flow
    pub descriptor*/: *mut *mut *mut rtl_rx_buffer_desc buffer_desc; /rx buffer,
    pub /: *mut *mut u16 next_rx_rp; / next_rx_read_point,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_pci {
    pub pdev: *mut pci_dev,
    pub irq_enabled: bool,
    pub driver_is_goingto_unload: bool,
    pub up_first_time: bool,
    pub first_init: bool,
    pub being_init_adapter: bool,
    pub init_ready: bool,
// Tx
    pub tx_ring: [rtl8192_tx_ring; RTL_PCI_MAX_TX_QUEUE_COUNT],
    pub txringcount: [c_int; RTL_PCI_MAX_TX_QUEUE_COUNT],
    pub transmit_config: u32,
// Rx
    pub rx_ring: [rtl8192_rx_ring; RTL_PCI_MAX_RX_QUEUE],
    pub rxringcount: c_int,
    pub rxbuffersize: u16,
    pub receive_config: u32,
// irq
    pub irq_alloc: u8,
    pub /: *mut *mut u32 irq_mask[4]; / 0-1: normal, 2: unused, 3: h2c,
    pub sys_irq_mask: u32,
// Bcn control register setting
    pub reg_bcn_ctrl_val: u32,
// ASPM*/ u8 const_pci_aspm;
    pub const_hwsw_rfoff_d3: u8,
    pub const_support_pciaspm: u8,
// pci-e bridge
    pub const_hostpci_aspm_setting: u8,
// pci-e device
    pub const_devicepci_aspm_setting: u8,
// If it supports ASPM, Offset[560h] = 0x40,
// otherwise Offset[560h] = 0x00.
//
    pub support_aspm: bool,
    pub support_backdoor: bool,
// QOS & EDCA
    pub acm_method: acm_method,
    pub shortretry_limit: u16,
    pub longretry_limit: u16,
// MSI support
    pub msi_support: bool,
    pub using_msi: bool,
// interrupt clear before set
    pub int_clear: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mp_adapter {
    pub linkctrl_reg: u8,
    pub busnumber: u8,
    pub devnumber: u8,
    pub funcnumber: u8,
    pub pcibridge_busnum: u8,
    pub pcibridge_devnum: u8,
    pub pcibridge_funcnum: u8,
    pub pcibridge_vendor: u8,
    pub amd_l1_patch: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_pci_priv {
    pub bt_coexist: bt_coexist_info,
    pub ledctl: rtl_led_ctl,
    pub dev: rtl_pci,
    pub ndis_adapter: mp_adapter,
}

extern "C" {
    pub fn rtl_pci_reset_trx_ring(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn rtl_pci_disconnect(pdev: *mut pci_dev);
}

extern "C" {
    pub fn rtl_pci_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn rtl_pci_resume(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn readb(addr: *mut *mut (u8 __iomem )rtlpriv->io.pci_mem_start +) -> return;
}
extern "C" {
    pub fn readw(addr: *mut *mut (u8 __iomem )rtlpriv->io.pci_mem_start +) -> return;
}
extern "C" {
    pub fn readl(addr: *mut *mut (u8 __iomem )rtlpriv->io.pci_mem_start +) -> return;
}
