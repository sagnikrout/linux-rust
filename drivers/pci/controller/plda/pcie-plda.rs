//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/plda/pcie-plda.h
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
// PLDA PCIe host controller driver
//
// Number of MSI IRQs
pub const PLDA_MAX_NUM_MSI_IRQS: c_int = 32;
// PCIe Bridge Phy Regs
pub const GEN_SETTINGS: c_uint = 0x80;
pub const RP_ENABLE: c_int = 1;
pub const PCIE_PCI_IDS_DW1: c_uint = 0x9c;
pub const IDS_CLASS_CODE_SHIFT: c_int = 16;

pub const PCIE_PCI_IRQ_DW0: c_uint = 0xa8;

pub const NUM_MSI_MSGS_SHIFT: c_int = 4;
pub const PCI_MISC: c_uint = 0xb4;

pub const PCIE_WINROM: c_uint = 0xfc;

pub const IMASK_LOCAL: c_uint = 0x180;
pub const DMA_END_ENGINE_0_MASK: c_uint = 0x00000000u;
pub const DMA_END_ENGINE_0_SHIFT: c_int = 0;
pub const DMA_END_ENGINE_1_MASK: c_uint = 0x00000000u;
pub const DMA_END_ENGINE_1_SHIFT: c_int = 1;
pub const DMA_ERROR_ENGINE_0_MASK: c_uint = 0x00000100u;
pub const DMA_ERROR_ENGINE_0_SHIFT: c_int = 8;
pub const DMA_ERROR_ENGINE_1_MASK: c_uint = 0x00000200u;
pub const DMA_ERROR_ENGINE_1_SHIFT: c_int = 9;
pub const A_ATR_EVT_POST_ERR_MASK: c_uint = 0x00010000u;
pub const A_ATR_EVT_POST_ERR_SHIFT: c_int = 16;
pub const A_ATR_EVT_FETCH_ERR_MASK: c_uint = 0x00020000u;
pub const A_ATR_EVT_FETCH_ERR_SHIFT: c_int = 17;
pub const A_ATR_EVT_DISCARD_ERR_MASK: c_uint = 0x00040000u;
pub const A_ATR_EVT_DISCARD_ERR_SHIFT: c_int = 18;
pub const A_ATR_EVT_DOORBELL_MASK: c_uint = 0x00000000u;
pub const A_ATR_EVT_DOORBELL_SHIFT: c_int = 19;
pub const P_ATR_EVT_POST_ERR_MASK: c_uint = 0x00100000u;
pub const P_ATR_EVT_POST_ERR_SHIFT: c_int = 20;
pub const P_ATR_EVT_FETCH_ERR_MASK: c_uint = 0x00200000u;
pub const P_ATR_EVT_FETCH_ERR_SHIFT: c_int = 21;
pub const P_ATR_EVT_DISCARD_ERR_MASK: c_uint = 0x00400000u;
pub const P_ATR_EVT_DISCARD_ERR_SHIFT: c_int = 22;
pub const P_ATR_EVT_DOORBELL_MASK: c_uint = 0x00000000u;
pub const P_ATR_EVT_DOORBELL_SHIFT: c_int = 23;
pub const PM_MSI_INT_INTA_MASK: c_uint = 0x01000000u;
pub const PM_MSI_INT_INTA_SHIFT: c_int = 24;
pub const PM_MSI_INT_INTB_MASK: c_uint = 0x02000000u;
pub const PM_MSI_INT_INTB_SHIFT: c_int = 25;
pub const PM_MSI_INT_INTC_MASK: c_uint = 0x04000000u;
pub const PM_MSI_INT_INTC_SHIFT: c_int = 26;
pub const PM_MSI_INT_INTD_MASK: c_uint = 0x08000000u;
pub const PM_MSI_INT_INTD_SHIFT: c_int = 27;
pub const PM_MSI_INT_INTX_MASK: c_uint = 0x0f000000u;
pub const PM_MSI_INT_INTX_SHIFT: c_int = 24;
pub const PM_MSI_INT_MSI_MASK: c_uint = 0x10000000u;
pub const PM_MSI_INT_MSI_SHIFT: c_int = 28;
pub const PM_MSI_INT_AER_EVT_MASK: c_uint = 0x20000000u;
pub const PM_MSI_INT_AER_EVT_SHIFT: c_int = 29;
pub const PM_MSI_INT_EVENTS_MASK: c_uint = 0x40000000u;
pub const PM_MSI_INT_EVENTS_SHIFT: c_int = 30;
pub const PM_MSI_INT_SYS_ERR_MASK: c_uint = 0x80000000u;
pub const PM_MSI_INT_SYS_ERR_SHIFT: c_int = 31;

pub const NUM_LOCAL_EVENTS: c_int = 15;
pub const ISTATUS_LOCAL: c_uint = 0x184;
pub const IMASK_HOST: c_uint = 0x188;
pub const ISTATUS_HOST: c_uint = 0x18c;
pub const IMSI_ADDR: c_uint = 0x190;
pub const ISTATUS_MSI: c_uint = 0x194;
pub const PMSG_SUPPORT_RX: c_uint = 0x3f0;

// PCIe Master table init defines
pub const ATR0_PCIE_WIN0_SRCADDR_PARAM: c_uint = 0x600u;
pub const ATR0_PCIE_ATR_SIZE: c_uint = 0x25;
pub const ATR0_PCIE_ATR_SIZE_SHIFT: c_int = 1;
pub const ATR0_PCIE_WIN0_SRC_ADDR: c_uint = 0x604u;
pub const ATR0_PCIE_WIN0_TRSL_ADDR_LSB: c_uint = 0x608u;
pub const ATR0_PCIE_WIN0_TRSL_ADDR_UDW: c_uint = 0x60cu;
pub const ATR0_PCIE_WIN0_TRSL_PARAM: c_uint = 0x610u;
// PCIe AXI slave table init defines
pub const ATR0_AXI4_SLV0_SRCADDR_PARAM: c_uint = 0x800u;

pub const ATR0_AXI4_SLV0_SRC_ADDR: c_uint = 0x804u;
pub const ATR0_AXI4_SLV0_TRSL_ADDR_LSB: c_uint = 0x808u;
pub const ATR0_AXI4_SLV0_TRSL_ADDR_UDW: c_uint = 0x80cu;
pub const ATR0_AXI4_SLV0_TRSL_PARAM: c_uint = 0x810u;
pub const PCIE_TX_RX_INTERFACE: c_uint = 0x00000000u;
pub const PCIE_CONFIG_INTERFACE: c_uint = 0x00000001u;
pub const TRSL_ID_AXI4_MASTER_0: c_uint = 0x00000004u;
pub const CONFIG_SPACE_ADDR_OFFSET: c_uint = 0x1000u;
pub const ATR_ENTRY_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum plda_int_event {
    PLDA_AXI_POST_ERR,
    PLDA_AXI_FETCH_ERR,
    PLDA_AXI_DISCARD_ERR,
    PLDA_AXI_DOORBELL,
    PLDA_PCIE_POST_ERR,
    PLDA_PCIE_FETCH_ERR,
    PLDA_PCIE_DISCARD_ERR,
    PLDA_PCIE_DOORBELL,
    PLDA_INTX,
    PLDA_MSI,
    PLDA_AER_EVENT,
    PLDA_MISC_EVENTS,
    PLDA_SYS_ERR,
    PLDA_INT_EVENT_NUM
}

pub const PLDA_NUM_DMA_EVENTS: c_int = 16;

//
// PLDA interrupt register
//
// 31         27     23              15           7          0
// +--+--+--+-+------+-+-+-+-+-+-+-+-+-----------+-----------+
// |12|11|10|9| intx |7|6|5|4|3|2|1|0| DMA error | DMA end   |
// +--+--+--+-+------+-+-+-+-+-+-+-+-+-----------+-----------+
// event  bit
// 0-7   (0-7)   DMA interrupt end : reserved for vendor implement
// 8-15  (8-15)  DMA error : reserved for vendor implement
// 16    (16)    AXI post error (PLDA_AXI_POST_ERR)
// 17    (17)    AXI fetch error (PLDA_AXI_FETCH_ERR)
// 18    (18)    AXI discard error (PLDA_AXI_DISCARD_ERR)
// 19    (19)    AXI doorbell (PLDA_PCIE_DOORBELL)
// 20    (20)    PCIe post error (PLDA_PCIE_POST_ERR)
// 21    (21)    PCIe fetch error (PLDA_PCIE_FETCH_ERR)
// 22    (22)    PCIe discard error (PLDA_PCIE_DISCARD_ERR)
// 23    (23)    PCIe doorbell (PLDA_PCIE_DOORBELL)
// 24    (27-24) INTx interruts (PLDA_INTX)
// 25    (28):   MSI interrupt (PLDA_MSI)
// 26    (29):   AER event (PLDA_AER_EVENT)
// 27    (30):   PM/LTR/Hotplug (PLDA_MISC_EVENTS)
// 28    (31):   System error (PLDA_SYS_ERR)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plda_event_ops {
    pub pcie): *mut *mut u32 (get_events)(struct plda_pcie_rp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plda_pcie_host_ops {
    pub pcie): *mut *mut int (host_init)(struct plda_pcie_rp,
    pub pcie): *mut *mut void (host_deinit)(struct plda_pcie_rp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plda_msi {
    pub /: *mut *mut mutex lock; / Protect used bitmap,
    pub dev_domain: *mut irq_domain,
    pub num_vectors: u32,
    pub vector_phy: u64,
    pub PLDA_MAX_NUM_MSI_IRQS): DECLARE_BITMAP(used,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plda_pcie_rp {
    pub dev: *mut device,
    pub bridge: *mut pci_host_bridge,
    pub intx_domain: *mut irq_domain,
    pub event_domain: *mut irq_domain,
    pub lock: raw_spinlock_t,
    pub msi: plda_msi,
    pub event_ops: *const plda_event_ops,
    pub event_irq_chip: *const irq_chip,
    pub host_ops: *const plda_pcie_host_ops,
    pub bridge_addr: *mut void __iomem,
    pub config_base: *mut void __iomem,
    pub events_bitmap: c_ulong,
    pub irq: c_int,
    pub msi_irq: c_int,
    pub intx_irq: c_int,
    pub num_events: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plda_event {
    pub event): int event_irq, int,
    pub intx_event: c_int,
    pub msi_event: c_int,
}

extern "C" {
    pub fn plda_pcie_setup_inbound_address_translation(port: *mut plda_pcie_rp);
}
extern "C" {
    pub fn plda_pcie_host_deinit(pcie: *mut plda_pcie_rp);
}
// set class code and reserve revision id
