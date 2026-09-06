//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep/octep_main.h
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
// Marvell Octeon EP (EndPoint) Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//

pub const OCTEP_PCIID_CN93_PF: c_uint = 0xB200177d;
pub const OCTEP_PCIID_CN93_VF: c_uint = 0xB203177d;
pub const OCTEP_PCI_DEVICE_ID_CN98_PF: c_uint = 0xB100;
pub const OCTEP_PCI_DEVICE_ID_CN93_PF: c_uint = 0xB200;
pub const OCTEP_PCI_DEVICE_ID_CN93_VF: c_uint = 0xB203;
pub const OCTEP_PCI_DEVICE_ID_CNF95N_PF: c_uint = 0xB400    //95N PF;
pub const OCTEP_PCI_DEVICE_ID_CN10KA_PF: c_uint = 0xB900   //CN10KA PF;
pub const OCTEP_PCI_DEVICE_ID_CNF10KA_PF: c_uint = 0xBA00   //CNF10KA PF;
pub const OCTEP_PCI_DEVICE_ID_CNF10KB_PF: c_uint = 0xBC00   //CNF10KB PF;
pub const OCTEP_PCI_DEVICE_ID_CN10KB_PF: c_uint = 0xBD00   //CN10KB PF;
pub const OCTEP_MAX_QUEUES: c_int = 63;

pub const OCTEP_MAX_VF: c_int = 64;

// Flags to disable and enable Interrupts

pub const OCTEP_ALL_INTR: c_uint = 0xff;
pub const OCTEP_IQ_INTR_RESEND_BIT: c_int = 59;
pub const OCTEP_OQ_INTR_RESEND_BIT: c_int = 59;
pub const OCTEP_MMIO_REGIONS: c_int = 3;

// PCI address space mapping information.
// Each of the 3 address spaces given by BAR0, BAR2 and BAR4 of
// Octeon gets mapped to different physical address spaces in
// the kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_mmio {
// The physical address to which the PCI address space is mapped.
    pub hw_addr: *mut u8 __iomem,
// Flag indicating the mapping was successful.
    pub mapped: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_pci_win_regs {
    pub pci_win_wr_addr: *mut u8 __iomem,
    pub pci_win_rd_addr: *mut u8 __iomem,
    pub pci_win_wr_data: *mut u8 __iomem,
    pub pci_win_rd_data: *mut u8 __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_hw_ops {
    pub q): *mut *mut *mut void (setup_iq_regs)(struct octep_device oct, int,
    pub q): *mut *mut *mut int (setup_oq_regs)(struct octep_device oct, int,
    pub mbox): *mut *mut *mut void (setup_mbox_regs)(struct octep_device oct, int,
    pub ioq_vector): *mut *mut irqreturn_t (mbox_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (oei_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (ire_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (ore_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (vfire_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (vfore_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (dma_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (dma_vf_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (pp_vf_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (misc_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (rsvd_intr_handler)(void,
    pub ioq_vector): *mut *mut irqreturn_t (ioq_intr_handler)(void,
    pub oct): *mut *mut int (soft_reset)(struct octep_device,
    pub oct): *mut *mut void (reinit_regs)(struct octep_device,
    pub iq): *mut *mut u32 (update_iq_read_idx)(struct octep_iq,
    pub oct): *mut *mut void (enable_interrupts)(struct octep_device,
    pub oct): *mut *mut void (disable_interrupts)(struct octep_device,
    pub oct): *mut *mut void (poll_non_ioq_interrupts)(struct octep_device,
    pub oct): *mut *mut void (enable_io_queues)(struct octep_device,
    pub oct): *mut *mut void (disable_io_queues)(struct octep_device,
    pub q): *mut *mut *mut void (enable_iq)(struct octep_device oct, int,
    pub q): *mut *mut *mut void (disable_iq)(struct octep_device oct, int,
    pub q): *mut *mut *mut void (enable_oq)(struct octep_device oct, int,
    pub q): *mut *mut *mut void (disable_oq)(struct octep_device oct, int,
    pub oct): *mut *mut void (reset_io_queues)(struct octep_device,
    pub oct): *mut *mut void (dump_registers)(struct octep_device,
}

// Octeon mailbox data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_mbox_data {
    pub cmd: u32,
    pub total_len: u32,
    pub recv_len: u32,
    pub rsvd: u32,
    pub data: *mut u64,
}

pub const MAX_VF_PF_MBOX_DATA_SIZE: c_int = 384;
// wrappers around work structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_pfvf_mbox_wk {
    pub work: work_struct,
    pub ctxptr: *mut c_void,
    pub ctxul: u64,
}

// Octeon device mailbox
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_mbox {
// A mutex to protect access to this q_mbox.
    pub lock: mutex,
    pub vf_id: u32,
    pub config_data_index: u32,
    pub message_len: u32,
    pub pf_vf_data_reg: *mut u8 __iomem,
    pub vf_pf_data_reg: *mut u8 __iomem,
    pub wk: octep_pfvf_mbox_wk,
    pub oct: *mut octep_device,
    pub mbox_data: octep_mbox_data,
    pub config_data: [u8; MAX_VF_PF_MBOX_DATA_SIZE],
}

// Tx/Rx queue vector per interrupt.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ioq_vector {
    pub name: [c_char; OCTEP_MSIX_NAME_SIZE],
    pub napi: napi_struct,
    pub octep_dev: *mut octep_device,
    pub iq: *mut octep_iq,
    pub oq: *mut octep_oq,
    pub affinity_mask: cpumask_t,
}

// Octeon hardware/firmware offload capability flags.

// Link modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_link_mode_bit_indices {
    OCTEP_LINK_MODE_10GBASE_T    = 0,
    OCTEP_LINK_MODE_10GBASE_R,
    OCTEP_LINK_MODE_10GBASE_CR,
    OCTEP_LINK_MODE_10GBASE_KR,
    OCTEP_LINK_MODE_10GBASE_LR,
    OCTEP_LINK_MODE_10GBASE_SR,
    OCTEP_LINK_MODE_25GBASE_CR,
    OCTEP_LINK_MODE_25GBASE_KR,
    OCTEP_LINK_MODE_25GBASE_SR,
    OCTEP_LINK_MODE_40GBASE_CR4,
    OCTEP_LINK_MODE_40GBASE_KR4,
    OCTEP_LINK_MODE_40GBASE_LR4,
    OCTEP_LINK_MODE_40GBASE_SR4,
    OCTEP_LINK_MODE_50GBASE_CR2,
    OCTEP_LINK_MODE_50GBASE_KR2,
    OCTEP_LINK_MODE_50GBASE_SR2,
    OCTEP_LINK_MODE_50GBASE_CR,
    OCTEP_LINK_MODE_50GBASE_KR,
    OCTEP_LINK_MODE_50GBASE_LR,
    OCTEP_LINK_MODE_50GBASE_SR,
    OCTEP_LINK_MODE_100GBASE_CR4,
    OCTEP_LINK_MODE_100GBASE_KR4,
    OCTEP_LINK_MODE_100GBASE_LR4,
    OCTEP_LINK_MODE_100GBASE_SR4,
    OCTEP_LINK_MODE_NBITS
}

// Hardware interface link state information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_iface_link_info {
// Bitmap of Supported link speeds/modes.
    pub supported_modes: u64,
// Bitmap of Advertised link speeds/modes.
    pub advertised_modes: u64,
// Negotiated link speed in Mbps.
    pub speed: u32,
// MTU
    pub mtu: u16,
// Autonegotation state.

    pub autoneg: u8,
// Pause frames setting.

    pub pause: u8,
// Admin state of the link (ifconfig <iface> up/down
    pub admin_up: u8,
// Operational state of the link: physical link is up down
    pub oper_up: u8,
}

// The Octeon VF device specific info data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_pfvf_info {
    pub mac_addr: [u8; ETH_ALEN],
    pub flags: u32,
    pub mbox_version: u32,
}

// The Octeon device specific private data structure.
// Each Octeon device has this structure to represent all its components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_device {
    pub conf: *mut octep_config,
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
    pub mmio: [octep_mmio; OCTEP_MMIO_REGIONS],
// MAC address
    pub mac_addr: [u8; ETH_ALEN],
// Tx queues (IQ: Instruction Queue)
    pub num_iqs: u16,
// Pointers to Octeon Tx queues
    pub iq: [*mut octep_iq; OCTEP_MAX_IQ],
// Per iq stats
    pub stats_iq: [octep_iq_stats; OCTEP_MAX_IQ],
// Rx queues (OQ: Output Queue)
    pub num_oqs: u16,
// Pointers to Octeon Rx queues
    pub oq: [*mut octep_oq; OCTEP_MAX_OQ],
// Per oq stats
    pub stats_oq: [octep_oq_stats; OCTEP_MAX_OQ],
// Hardware port number of the PCIe interface
    pub pcie_port: u16,
// PCI Window registers to access some hardware CSRs
    pub pci_win_regs: octep_pci_win_regs,
// Hardware operations
    pub hw_ops: octep_hw_ops,
// IRQ info
    pub num_irqs: u16,
    pub num_non_ioq_irqs: u16,
    pub non_ioq_irq_names: *mut c_char,
    pub msix_entries: *mut msix_entry,
// IOq information of it's corresponding MSI-X interrupt.
    pub ioq_vector: [*mut octep_ioq_vector; OCTEP_MAX_QUEUES],
// Hardware Interface Tx statistics
    pub iface_tx_stats: octep_iface_tx_stats,
// Hardware Interface Rx statistics
    pub iface_rx_stats: octep_iface_rx_stats,
// Hardware Interface Link info like supported modes, aneg support
    pub link_info: octep_iface_link_info,
// Mailbox to talk to VFs
    pub mbox: [*mut octep_mbox; OCTEP_MAX_VF],
// VFs info
    pub vf_info: [octep_pfvf_info; OCTEP_MAX_VF],
// Work entry to handle Tx timeout
    pub tx_timeout_task: work_struct,
// control mbox over pf
    pub ctrl_mbox: octep_ctrl_mbox,
// offset for iface stats
    pub ctrl_mbox_ifstats_offset: u32,
// Work entry to handle ctrl mbox interrupt
    pub ctrl_mbox_task: work_struct,
// Wait queue for host to firmware requests
    pub ctrl_req_wait_q: wait_queue_head_t,
// List of objects waiting for h2f response
    pub ctrl_req_wait_list: list_head,
// Enable non-ioq interrupt polling
    pub poll_non_ioq_intr: bool,
// Work entry to poll non-ioq interrupts
    pub intr_poll_task: delayed_work,
// Firmware heartbeat timer
    pub hb_timer: timer_list,
// Firmware heartbeat miss count tracked by timer
    pub hb_miss_cnt: core::sync::atomic::AtomicI32,
// Task to reset device on heartbeat miss
    pub hb_task: delayed_work,
}

// Octeon CSR read/write access APIs

// Read windowed register.
// @param  oct   -  pointer to the Octeon device.
// @param  addr  -  Address of the register to read.
//
// This routine is called to read from the indirectly accessed
// Octeon registers that are visible through a PCI BAR0 mapped window
// register.
// @return  - 64 bit value read from the register.
//
// Write windowed register.
// @param  oct  -  pointer to the Octeon device.
// @param  addr -  Address of the register to write
// @param  val  -  Value to write
//
// This routine is called to write to the indirectly accessed
// Octeon registers that are visible through a PCI BAR0 mapped window
// register.
// @return   Nothing.
//
extern "C" {
    pub fn octep_device_setup(oct: *mut octep_device) -> c_int;
}
extern "C" {
    pub fn octep_setup_iqs(oct: *mut octep_device) -> c_int;
}
extern "C" {
    pub fn octep_free_iqs(oct: *mut octep_device);
}
extern "C" {
    pub fn octep_clean_iqs(oct: *mut octep_device);
}
extern "C" {
    pub fn octep_setup_oqs(oct: *mut octep_device) -> c_int;
}
extern "C" {
    pub fn octep_free_oqs(oct: *mut octep_device);
}
extern "C" {
    pub fn octep_oq_dbell_init(oct: *mut octep_device);
}
extern "C" {
    pub fn octep_device_setup_cn93_pf(oct: *mut octep_device);
}
extern "C" {
    pub fn octep_device_setup_cnxk_pf(oct: *mut octep_device);
}
extern "C" {
    pub fn octep_iq_process_completions(iq: *mut octep_iq, budget: u16) -> c_int;
}
extern "C" {
    pub fn octep_oq_process_rx(oq: *mut octep_oq, budget: c_int) -> c_int;
}
extern "C" {
    pub fn octep_set_ethtool_ops(netdev: *mut net_device);
}
