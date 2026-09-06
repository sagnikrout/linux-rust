//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep_vf/octep_vf_main.h
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
// Marvell Octeon EP (EndPoint) VF Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//

pub const OCTEP_PCI_DEVICE_ID_CN93_VF: c_uint = 0xB203    //93xx VF;
pub const OCTEP_PCI_DEVICE_ID_CNF95N_VF: c_uint = 0xB403    //95N VF;
pub const OCTEP_PCI_DEVICE_ID_CN98_VF: c_uint = 0xB103;
pub const OCTEP_PCI_DEVICE_ID_CN10KA_VF: c_uint = 0xB903;
pub const OCTEP_PCI_DEVICE_ID_CNF10KA_VF: c_uint = 0xBA03;
pub const OCTEP_PCI_DEVICE_ID_CNF10KB_VF: c_uint = 0xBC03;
pub const OCTEP_PCI_DEVICE_ID_CN10KB_VF: c_uint = 0xBD03;
pub const OCTEP_VF_MAX_QUEUES: c_int = 63;

pub const OCTEP_VF_IQ_INTR_RESEND_BIT: c_int = 59;
pub const OCTEP_VF_OQ_INTR_RESEND_BIT: c_int = 59;

// PCI address space mapping information.
// Each of the 3 address spaces given by BAR0, BAR2 and BAR4 of
// Octeon gets mapped to different physical address spaces in
// the kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_mmio {
// The physical address to which the PCI address space is mapped.
    pub hw_addr: *mut u8 __iomem,
// Flag indicating the mapping was successful.
    pub mapped: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_hw_ops {
    pub q): *mut *mut *mut void (setup_iq_regs)(struct octep_vf_device oct, int,
    pub q): *mut *mut *mut int (setup_oq_regs)(struct octep_vf_device oct, int,
    pub mbox): *mut *mut *mut void (setup_mbox_regs)(struct octep_vf_device oct, int,
    pub ioq_vector): *mut *mut irqreturn_t (non_ioq_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (ioq_intr_handler)(void,
    pub oct): *mut *mut void (reinit_regs)(struct octep_vf_device,
    pub iq): *mut *mut u32 (update_iq_read_idx)(struct octep_vf_iq,
    pub oct): *mut *mut void (enable_interrupts)(struct octep_vf_device,
    pub oct): *mut *mut void (disable_interrupts)(struct octep_vf_device,
    pub oct): *mut *mut void (enable_io_queues)(struct octep_vf_device,
    pub oct): *mut *mut void (disable_io_queues)(struct octep_vf_device,
    pub q): *mut *mut *mut void (enable_iq)(struct octep_vf_device oct, int,
    pub q): *mut *mut *mut void (disable_iq)(struct octep_vf_device oct, int,
    pub q): *mut *mut *mut void (enable_oq)(struct octep_vf_device oct, int,
    pub q): *mut *mut *mut void (disable_oq)(struct octep_vf_device oct, int,
    pub oct): *mut *mut void (reset_io_queues)(struct octep_vf_device,
    pub oct): *mut *mut void (dump_registers)(struct octep_vf_device,
}

// Octeon mailbox data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_mbox_data {
// Holds the offset of received data via mailbox.
    pub data_index: u32,
// Holds the received data via mailbox.
    pub recv_data: [u8; OCTEP_PFVF_MBOX_MAX_DATA_BUF_SIZE],
}

// wrappers around work structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_mbox_wk {
    pub work: work_struct,
    pub ctxptr: *mut c_void,
}

// Octeon device mailbox
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_mbox {
// A mutex to protect access to this q_mbox.
    pub lock: mutex,
    pub state: u32,
// SLI_MAC_PF_MBOX_INT for PF, SLI_PKT_MBOX_INT for VF.
    pub mbox_int_reg: *mut u8 __iomem,
// SLI_PKT_PF_VF_MBOX_SIG(0) for PF,
// SLI_PKT_PF_VF_MBOX_SIG(1) for VF.
//
    pub mbox_write_reg: *mut u8 __iomem,
// SLI_PKT_PF_VF_MBOX_SIG(1) for PF,
// SLI_PKT_PF_VF_MBOX_SIG(0) for VF.
//
    pub mbox_read_reg: *mut u8 __iomem,
// Octeon mailbox data
    pub mbox_data: octep_vf_mbox_data,
// Octeon mailbox work handler to process Mbox messages
    pub wk: octep_vf_mbox_wk,
}

// Tx/Rx queue vector per interrupt.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_ioq_vector {
    pub name: [c_char; OCTEP_VF_MSIX_NAME_SIZE],
    pub napi: napi_struct,
    pub octep_vf_dev: *mut octep_vf_device,
    pub iq: *mut octep_vf_iq,
    pub oq: *mut octep_vf_oq,
    pub affinity_mask: cpumask_t,
}

// Octeon hardware/firmware offload capability flags.

// Link modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_vf_link_mode_bit_indices {
    OCTEP_VF_LINK_MODE_10GBASE_T    = 0,
    OCTEP_VF_LINK_MODE_10GBASE_R,
    OCTEP_VF_LINK_MODE_10GBASE_CR,
    OCTEP_VF_LINK_MODE_10GBASE_KR,
    OCTEP_VF_LINK_MODE_10GBASE_LR,
    OCTEP_VF_LINK_MODE_10GBASE_SR,
    OCTEP_VF_LINK_MODE_25GBASE_CR,
    OCTEP_VF_LINK_MODE_25GBASE_KR,
    OCTEP_VF_LINK_MODE_25GBASE_SR,
    OCTEP_VF_LINK_MODE_40GBASE_CR4,
    OCTEP_VF_LINK_MODE_40GBASE_KR4,
    OCTEP_VF_LINK_MODE_40GBASE_LR4,
    OCTEP_VF_LINK_MODE_40GBASE_SR4,
    OCTEP_VF_LINK_MODE_50GBASE_CR2,
    OCTEP_VF_LINK_MODE_50GBASE_KR2,
    OCTEP_VF_LINK_MODE_50GBASE_SR2,
    OCTEP_VF_LINK_MODE_50GBASE_CR,
    OCTEP_VF_LINK_MODE_50GBASE_KR,
    OCTEP_VF_LINK_MODE_50GBASE_LR,
    OCTEP_VF_LINK_MODE_50GBASE_SR,
    OCTEP_VF_LINK_MODE_100GBASE_CR4,
    OCTEP_VF_LINK_MODE_100GBASE_KR4,
    OCTEP_VF_LINK_MODE_100GBASE_LR4,
    OCTEP_VF_LINK_MODE_100GBASE_SR4,
    OCTEP_VF_LINK_MODE_NBITS
}

// Hardware interface link state information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_iface_link_info {
// Bitmap of Supported link speeds/modes.
    pub supported_modes: u64,
// Bitmap of Advertised link speeds/modes.
    pub advertised_modes: u64,
// Negotiated link speed in Mbps.
    pub speed: u32,
// MTU
    pub mtu: u16,
// Autonegotiation state.

    pub autoneg: u8,
// Pause frames setting.

    pub pause: u8,
// Admin state of the link (ifconfig <iface> up/down
    pub admin_up: u8,
// Operational state of the link: physical link is up down
    pub oper_up: u8,
}

// Hardware interface stats information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_iface_rxtx_stats {
// Hardware Interface Rx statistics
    pub iface_rx_stats: octep_vf_iface_rx_stats,
// Hardware Interface Tx statistics
    pub iface_tx_stats: octep_vf_iface_tx_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_fw_info {
// pkind value to be used in every Tx hardware descriptor
    pub pkind: u8,
// front size data
    pub fsz: u8,
// supported rx offloads OCTEP_VF_RX_OFFLOAD_*
    pub rx_ol_flags: u16,
// supported tx offloads OCTEP_VF_TX_OFFLOAD_*
    pub tx_ol_flags: u16,
}

// The Octeon device specific private data structure.
// Each Octeon device has this structure to represent all its components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_vf_device {
    pub conf: *mut octep_vf_config,
// Octeon Chip type.
    pub chip_id: u16,
    pub rev_id: u16,
// Device capabilities enabled
    pub caps_enabled: u64,
// Device capabilities supported
    pub caps_supported: u64,
// Pointer to basic Linux device
    pub dev: *mut device,
// Linux PCI device pointer
    pub pdev: *mut pci_dev,
// Netdev corresponding to the Octeon device
    pub netdev: *mut net_device,
// memory mapped io range
    pub mmio: octep_vf_mmio,
// MAC address
    pub mac_addr: [u8; ETH_ALEN],
// Tx queues (IQ: Instruction Queue)
    pub num_iqs: u16,
// Pointers to Octeon Tx queues
    pub iq: [*mut octep_vf_iq; OCTEP_VF_MAX_IQ],
// Per iq stats
    pub stats_iq: [octep_vf_iq_stats; OCTEP_VF_MAX_IQ],
// Rx queues (OQ: Output Queue)
    pub num_oqs: u16,
// Pointers to Octeon Rx queues
    pub oq: [*mut octep_vf_oq; OCTEP_VF_MAX_OQ],
// Per oq stats
    pub stats_oq: [octep_vf_oq_stats; OCTEP_VF_MAX_OQ],
// Hardware port number of the PCIe interface
    pub pcie_port: u16,
// Hardware operations
    pub hw_ops: octep_vf_hw_ops,
// IRQ info
    pub num_irqs: u16,
    pub num_non_ioq_irqs: u16,
    pub non_ioq_irq_names: *mut c_char,
    pub msix_entries: *mut msix_entry,
// IOq information of it's corresponding MSI-X interrupt.
    pub ioq_vector: [*mut octep_vf_ioq_vector; OCTEP_VF_MAX_QUEUES],
// Hardware Interface Tx statistics
    pub iface_tx_stats: octep_vf_iface_tx_stats,
// Hardware Interface Rx statistics
    pub iface_rx_stats: octep_vf_iface_rx_stats,
// Hardware Interface Link info like supported modes, aneg support
    pub link_info: octep_vf_iface_link_info,
// Mailbox to talk to VFs
    pub mbox: *mut octep_vf_mbox,
// Work entry to handle Tx timeout
    pub tx_timeout_task: work_struct,
// offset for iface stats
    pub ctrl_mbox_ifstats_offset: u32,
// Negotiated Mbox version
    pub mbox_neg_ver: u32,
// firmware info
    pub fw_info: octep_vf_fw_info,
}

// Octeon CSR read/write access APIs

extern "C" {
    pub fn octep_vf_device_setup(oct: *mut octep_vf_device) -> c_int;
}
extern "C" {
    pub fn octep_vf_setup_iqs(oct: *mut octep_vf_device) -> c_int;
}
extern "C" {
    pub fn octep_vf_free_iqs(oct: *mut octep_vf_device);
}
extern "C" {
    pub fn octep_vf_clean_iqs(oct: *mut octep_vf_device);
}
extern "C" {
    pub fn octep_vf_setup_oqs(oct: *mut octep_vf_device) -> c_int;
}
extern "C" {
    pub fn octep_vf_free_oqs(oct: *mut octep_vf_device);
}
extern "C" {
    pub fn octep_vf_oq_dbell_init(oct: *mut octep_vf_device);
}
extern "C" {
    pub fn octep_vf_device_setup_cn93(oct: *mut octep_vf_device);
}
extern "C" {
    pub fn octep_vf_device_setup_cnxk(oct: *mut octep_vf_device);
}
extern "C" {
    pub fn octep_vf_iq_process_completions(iq: *mut octep_vf_iq, budget: u16) -> c_int;
}
extern "C" {
    pub fn octep_vf_oq_process_rx(oq: *mut octep_vf_oq, budget: c_int) -> c_int;
}
extern "C" {
    pub fn octep_vf_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn octep_vf_get_link_info(oct: *mut octep_vf_device) -> c_int;
}
extern "C" {
    pub fn octep_vf_get_if_stats(oct: *mut octep_vf_device) -> c_int;
}
extern "C" {
    pub fn octep_vf_mbox_work(work: *mut work_struct);
}
