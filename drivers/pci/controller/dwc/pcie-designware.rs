//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/dwc/pcie-designware.h
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
// Synopsys DesignWare PCIe host controller driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// https://www.samsung.com
//
// Author: Jingoo Han <jg1.han@samsung.com>
//

// DWC PCIe IP-core versions (native support since v4.70a)
pub const DW_PCIE_VER_365A: c_uint = 0x3336352a;
pub const DW_PCIE_VER_460A: c_uint = 0x3436302a;
pub const DW_PCIE_VER_470A: c_uint = 0x3437302a;
pub const DW_PCIE_VER_480A: c_uint = 0x3438302a;
pub const DW_PCIE_VER_490A: c_uint = 0x3439302a;
pub const DW_PCIE_VER_500A: c_uint = 0x3530302a;
pub const DW_PCIE_VER_510A: c_uint = 0x3531302a;
pub const DW_PCIE_VER_520A: c_uint = 0x3532302a;
pub const DW_PCIE_VER_540A: c_uint = 0x3534302a;
pub const DW_PCIE_VER_562A: c_uint = 0x3536322a;

// DWC PCIe controller capabilities
pub const DW_PCIE_CAP_REQ_RES: c_int = 0;
pub const DW_PCIE_CAP_IATU_UNROLL: c_int = 1;
pub const DW_PCIE_CAP_CDM_CHECK: c_int = 2;

// Parameters for the waiting for iATU enabled routine
pub const LINK_WAIT_MAX_IATU_RETRIES: c_int = 5;
pub const LINK_WAIT_IATU: c_int = 9;
// Synopsys-specific PCIe configuration registers
pub const PCIE_PORT_FORCE: c_uint = 0x708;
// Bit[7:0] LINK_NUM: Link Number. Not used for endpoint

pub const PCIE_PORT_AFR: c_uint = 0x70C;

pub const PORT_AFR_L0S_ENTRANCE_LAT_SHIFT: c_int = 24;

pub const PORT_AFR_L1_ENTRANCE_LAT_SHIFT: c_int = 27;

pub const PCIE_PORT_LINK_CONTROL: c_uint = 0x710;

pub const PCIE_PORT_LANE_SKEW: c_uint = 0x714;

//
// PCIE_TIMER_CTRL_MAX_FUNC_NUM: Timer Control and Max Function Number
// Register.
//
// This register holds the ack frequency, latency, replay, fast link
// scaling timers, and max function number values.
//
// Bit[30:29] FAST_LINK_SCALING_FACTOR: Fast Link Timer Scaling Factor.
// 0x0 (SF_1024): Scaling Factor is 1024 (1ms is 1us).
// When the LTSSM is in Config or L12 Entry State, 1ms
// timer is 2us, 2ms timer is 4us and 3ms timer is 6us.
// 0x1 (SF_256): Scaling Factor is 256 (1ms is 4us)
// 0x2 (SF_64): Scaling Factor is 64 (1ms is 16us)
// 0x3 (SF_16): Scaling Factor is 16 (1ms is 64us)
//
pub const PCIE_TIMER_CTRL_MAX_FUNC_NUM: c_uint = 0x718;

pub const PORT_FLT_SF_VAL_1024: c_uint = 0x0;
pub const PORT_FLT_SF_VAL_256: c_uint = 0x1;
pub const PORT_FLT_SF_VAL_64: c_uint = 0x2;
pub const PORT_FLT_SF_VAL_16: c_uint = 0x3;
pub const PCIE_PORT_DEBUG0: c_uint = 0x728;
pub const PORT_LOGIC_LTSSM_STATE_MASK: c_uint = 0x3f;
pub const PORT_LOGIC_LTSSM_STATE_L0: c_uint = 0x11;
pub const PCIE_PORT_DEBUG1: c_uint = 0x72C;

pub const PCIE_LINK_WIDTH_SPEED_CONTROL: c_uint = 0x80C;

pub const PCIE_MSI_ADDR_LO: c_uint = 0x820;
pub const PCIE_MSI_ADDR_HI: c_uint = 0x824;
pub const PCIE_MSI_INTR0_ENABLE: c_uint = 0x828;
pub const PCIE_MSI_INTR0_MASK: c_uint = 0x82C;
pub const PCIE_MSI_INTR0_STATUS: c_uint = 0x830;
pub const GEN3_RELATED_OFF: c_uint = 0x890;

pub const GEN3_RELATED_OFF_RATE_SHADOW_SEL_SHIFT: c_int = 24;

pub const GEN3_EQ_CONTROL_OFF: c_uint = 0x8A8;

pub const GEN3_EQ_FB_MODE_DIR_CHANGE_OFF: c_uint = 0x8AC;

pub const COHERENCY_CONTROL_1_OFF: c_uint = 0x8E0;

pub const COHERENCY_CONTROL_2_OFF: c_uint = 0x8E4;
pub const COHERENCY_CONTROL_3_OFF: c_uint = 0x8E8;
pub const PCIE_PORT_MULTI_LANE_CTRL: c_uint = 0x8C0;

pub const PCIE_VERSION_NUMBER: c_uint = 0x8F8;
pub const PCIE_VERSION_TYPE: c_uint = 0x8FC;
//
// iATU inbound and outbound windows CSRs. Before the IP-core v4.80a each
// iATU region CSRs had been indirectly accessible by means of the dedicated
// viewport selector. The iATU/eDMA CSRs space was re-designed in DWC PCIe
// v4.80a in a way so the viewport was unrolled into the directly accessible
// iATU/eDMA CSRs space.
//
pub const PCIE_ATU_VIEWPORT: c_uint = 0x900;

pub const PCIE_ATU_REGION_DIR_OB: c_int = 0;
pub const PCIE_ATU_VIEWPORT_BASE: c_uint = 0x904;

pub const PCIE_ATU_VIEWPORT_SIZE: c_uint = 0x2C;
pub const PCIE_ATU_REGION_CTRL1: c_uint = 0x000;

pub const PCIE_ATU_REGION_CTRL2: c_uint = 0x004;

pub const PCIE_ATU_LOWER_BASE: c_uint = 0x008;
pub const PCIE_ATU_UPPER_BASE: c_uint = 0x00C;
pub const PCIE_ATU_LIMIT: c_uint = 0x010;
pub const PCIE_ATU_LOWER_TARGET: c_uint = 0x014;

pub const PCIE_ATU_UPPER_TARGET: c_uint = 0x018;
pub const PCIE_ATU_UPPER_LIMIT: c_uint = 0x020;
pub const PCIE_MISC_CONTROL_1_OFF: c_uint = 0x8BC;

pub const PCIE_MSIX_DOORBELL: c_uint = 0x948;
pub const PCIE_MSIX_DOORBELL_PF_SHIFT: c_int = 24;
//
// eDMA CSRs. DW PCIe IP-core v4.70a and older had the eDMA registers accessible
// over the Port Logic registers space. Afterwards the unrolled mapping was
// introduced so eDMA and iATU could be accessed via a dedicated registers
// space.
//
pub const PCIE_DMA_VIEWPORT_BASE: c_uint = 0x970;
pub const PCIE_DMA_UNROLL_BASE: c_uint = 0x80000;
pub const PCIE_DMA_CTRL: c_uint = 0x008;

pub const PCIE_PL_CHK_REG_CONTROL_STATUS: c_uint = 0xB20;

pub const PCIE_PL_CHK_REG_ERR_ADDR: c_uint = 0xB28;
//
// 16.0 GT/s (Gen 4) lane margining register definitions
//
pub const GEN4_LANE_MARGINING_1_OFF: c_uint = 0xB80;

pub const GEN4_LANE_MARGINING_2_OFF: c_uint = 0xB84;

//
// iATU Unroll-specific register definitions
// From 4.80 core version the address translation will be made by unroll
//
pub const PCIE_ATU_UNR_REGION_CTRL1: c_uint = 0x00;
pub const PCIE_ATU_UNR_REGION_CTRL2: c_uint = 0x04;
pub const PCIE_ATU_UNR_LOWER_BASE: c_uint = 0x08;
pub const PCIE_ATU_UNR_UPPER_BASE: c_uint = 0x0C;
pub const PCIE_ATU_UNR_LOWER_LIMIT: c_uint = 0x10;
pub const PCIE_ATU_UNR_LOWER_TARGET: c_uint = 0x14;
pub const PCIE_ATU_UNR_UPPER_TARGET: c_uint = 0x18;
pub const PCIE_ATU_UNR_UPPER_LIMIT: c_uint = 0x20;
//
// RAS-DES register definitions
//
pub const PCIE_RAS_DES_EVENT_COUNTER_CONTROL: c_uint = 0x8;
pub const EVENT_COUNTER_ALL_CLEAR: c_uint = 0x3;
pub const EVENT_COUNTER_ENABLE_ALL: c_uint = 0x7;
pub const EVENT_COUNTER_ENABLE_SHIFT: c_int = 2;

pub const EVENT_COUNTER_EVENT_SEL_SHIFT: c_int = 16;
pub const EVENT_COUNTER_EVENT_Tx_L0S: c_uint = 0x2;
pub const EVENT_COUNTER_EVENT_Rx_L0S: c_uint = 0x3;
pub const EVENT_COUNTER_EVENT_L1: c_uint = 0x5;
pub const EVENT_COUNTER_EVENT_L1_1: c_uint = 0x7;
pub const EVENT_COUNTER_EVENT_L1_2: c_uint = 0x8;
pub const EVENT_COUNTER_GROUP_SEL_SHIFT: c_int = 24;
pub const EVENT_COUNTER_GROUP_5: c_uint = 0x5;
pub const PCIE_RAS_DES_EVENT_COUNTER_DATA: c_uint = 0xc;
// PTM register definitions
pub const PTM_RES_REQ_CTRL: c_uint = 0x8;

pub const PTM_LOCAL_LSB: c_uint = 0x10;
pub const PTM_LOCAL_MSB: c_uint = 0x14;
pub const PTM_T1_T2_LSB: c_uint = 0x18;
pub const PTM_T1_T2_MSB: c_uint = 0x1c;
pub const PTM_T3_T4_LSB: c_uint = 0x28;
pub const PTM_T3_T4_MSB: c_uint = 0x2c;
pub const PTM_MASTER_LSB: c_uint = 0x38;
pub const PTM_MASTER_MSB: c_uint = 0x3c;
//
// The default address offset between dbi_base and atu_base. Root controller
// drivers are not required to initialize atu_base if the offset matches this
// default; the driver core automatically derives atu_base from dbi_base using
// this offset, if atu_base not set.
//

pub const MAX_MSI_IRQS: c_int = 256;
pub const MAX_MSI_IRQS_PER_CTRL: c_int = 32;

pub const MSI_REG_CTRL_BLOCK_SIZE: c_int = 12;
pub const MSI_DEF_NUM_VECTORS: c_int = 32;
// Maximum number of inbound/outbound iATUs
pub const MAX_IATU_IN: c_int = 256;
pub const MAX_IATU_OUT: c_int = 256;
// Default eDMA LLP memory size

// Common struct pci_epc_feature bits among DWC EP glue drivers

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_pcie_device_mode {
    DW_PCIE_UNKNOWN_TYPE,
    DW_PCIE_EP_TYPE,
    DW_PCIE_LEG_EP_TYPE,
    DW_PCIE_RC_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_pcie_app_clk {
    DW_PCIE_DBI_CLK,
    DW_PCIE_MSTR_CLK,
    DW_PCIE_SLV_CLK,
    DW_PCIE_NUM_APP_CLKS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_pcie_core_clk {
    DW_PCIE_PIPE_CLK,
    DW_PCIE_CORE_CLK,
    DW_PCIE_AUX_CLK,
    DW_PCIE_REF_CLK,
    DW_PCIE_NUM_CORE_CLKS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_pcie_app_rst {
    DW_PCIE_DBI_RST,
    DW_PCIE_MSTR_RST,
    DW_PCIE_SLV_RST,
    DW_PCIE_NUM_APP_RSTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_pcie_core_rst {
    DW_PCIE_NON_STICKY_RST,
    DW_PCIE_STICKY_RST,
    DW_PCIE_CORE_RST,
    DW_PCIE_PIPE_RST,
    DW_PCIE_PHY_RST,
    DW_PCIE_HOT_RST,
    DW_PCIE_PWR_RST,
    DW_PCIE_NUM_CORE_RSTS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_pcie_ltssm {
// Need to align with PCIE_PORT_DEBUG0 bits 0:5
    DW_PCIE_LTSSM_DETECT_QUIET = 0x0,
    DW_PCIE_LTSSM_DETECT_ACT = 0x1,
    DW_PCIE_LTSSM_POLL_ACTIVE = 0x2,
    DW_PCIE_LTSSM_POLL_COMPLIANCE = 0x3,
    DW_PCIE_LTSSM_POLL_CONFIG = 0x4,
    DW_PCIE_LTSSM_PRE_DETECT_QUIET = 0x5,
    DW_PCIE_LTSSM_DETECT_WAIT = 0x6,
    DW_PCIE_LTSSM_CFG_LINKWD_START = 0x7,
    DW_PCIE_LTSSM_CFG_LINKWD_ACEPT = 0x8,
    DW_PCIE_LTSSM_CFG_LANENUM_WAI = 0x9,
    DW_PCIE_LTSSM_CFG_LANENUM_ACEPT = 0xa,
    DW_PCIE_LTSSM_CFG_COMPLETE = 0xb,
    DW_PCIE_LTSSM_CFG_IDLE = 0xc,
    DW_PCIE_LTSSM_RCVRY_LOCK = 0xd,
    DW_PCIE_LTSSM_RCVRY_SPEED = 0xe,
    DW_PCIE_LTSSM_RCVRY_RCVRCFG = 0xf,
    DW_PCIE_LTSSM_RCVRY_IDLE = 0x10,
    DW_PCIE_LTSSM_L0 = 0x11,
    DW_PCIE_LTSSM_L0S = 0x12,
    DW_PCIE_LTSSM_L123_SEND_EIDLE = 0x13,
    DW_PCIE_LTSSM_L1_IDLE = 0x14,
    DW_PCIE_LTSSM_L2_IDLE = 0x15,
    DW_PCIE_LTSSM_L2_WAKE = 0x16,
    DW_PCIE_LTSSM_DISABLED_ENTRY = 0x17,
    DW_PCIE_LTSSM_DISABLED_IDLE = 0x18,
    DW_PCIE_LTSSM_DISABLED = 0x19,
    DW_PCIE_LTSSM_LPBK_ENTRY = 0x1a,
    DW_PCIE_LTSSM_LPBK_ACTIVE = 0x1b,
    DW_PCIE_LTSSM_LPBK_EXIT = 0x1c,
    DW_PCIE_LTSSM_LPBK_EXIT_TIMEOUT = 0x1d,
    DW_PCIE_LTSSM_HOT_RESET_ENTRY = 0x1e,
    DW_PCIE_LTSSM_HOT_RESET = 0x1f,
    DW_PCIE_LTSSM_RCVRY_EQ0 = 0x20,
    DW_PCIE_LTSSM_RCVRY_EQ1 = 0x21,
    DW_PCIE_LTSSM_RCVRY_EQ2 = 0x22,
    DW_PCIE_LTSSM_RCVRY_EQ3 = 0x23,

// Vendor glue drivers provide pseudo L1 substates from get_ltssm()
    DW_PCIE_LTSSM_L1_1 = 0x141,
    DW_PCIE_LTSSM_L1_2 = 0x142,

    DW_PCIE_LTSSM_UNKNOWN = 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie_ob_atu_cfg {
    pub index: c_int,
    pub type: c_int,
    pub func_no: u8,
    pub code: u8,
    pub routing: u8,
    pub ctrl2: u32,
    pub parent_bus_addr: u64,
    pub pci_addr: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie_host_ops {
    pub pp): *mut *mut int (init)(struct dw_pcie_rp,
    pub pp): *mut *mut void (deinit)(struct dw_pcie_rp,
    pub pp): *mut *mut void (post_init)(struct dw_pcie_rp,
    pub pp): *mut *mut int (msi_init)(struct dw_pcie_rp,
    pub pp): *mut *mut void (pme_turn_off)(struct dw_pcie_rp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie_rp {
    pub use_imsi_rx:1: bool,
    pub keep_rp_msi_en:1: bool,
    pub cfg0_io_shared:1: bool,
    pub cfg0_base: u64,
    pub va_cfg0_base: *mut void __iomem,
    pub cfg0_size: u32,
    pub io_base: resource_size_t,
    pub io_bus_addr: phys_addr_t,
    pub io_size: u32,
    pub irq: c_int,
    pub ops: *const dw_pcie_host_ops,
    pub msi_irq: [c_int; MAX_MSI_CTRLS],
    pub irq_domain: *mut irq_domain,
    pub msi_data: dma_addr_t,
    pub msi_irq_chip: *mut irq_chip,
    pub num_vectors: u32,
    pub irq_mask: [u32; MAX_MSI_CTRLS],
    pub bridge: *mut pci_host_bridge,
    pub lock: raw_spinlock_t,
    pub MAX_MSI_IRQS): DECLARE_BITMAP(msi_irq_in_use,,
    pub use_atu_msg: bool,
    pub msg_atu_index: c_int,
    pub msg_res: *mut resource,
    pub presets: pci_eq_presets,
    pub cfg: *mut pci_config_window,
    pub ecam_enabled: bool,
    pub native_ecam: bool,
    pub skip_l23_ready: bool,
    pub skip_pwrctrl_off: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie_ep_ops {
    pub ep): *mut *mut int (pre_init)(struct dw_pcie_ep,
    pub ep): *mut *mut int (init)(struct dw_pcie_ep,
    pub interrupt_num): unsigned int type, u16,
    pub ep): *const *const *const pci_epc_features (get_features)(dw_pcie_ep,
//
// Provide a method to implement the different func config space
// access for different platform, if different func have different
// offset, return the offset of func. if use write a register way
// return a 0, and implement code in callback function of platform
// driver.
//
    pub func_no): *mut *mut *mut unsigned int (get_dbi_offset)(struct dw_pcie_ep ep, u8,
    pub func_no): *mut *mut *mut unsigned int (get_dbi2_offset)(struct dw_pcie_ep ep, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie_ep_func {
    pub list: list_head,
    pub func_no: u8,
    pub /: *mut *mut u8 msi_cap; / MSI capability offset,
    pub /: *mut *mut u8 msix_cap; / MSI-X capability offset,
    pub bar_to_atu: [u8; PCI_STD_NUM_BARS],
    pub epf_bar: [*mut pci_epf_bar; PCI_STD_NUM_BARS],
// Only for Address Match Mode inbound iATU
    pub ib_atu_indexes: [*mut u32; PCI_STD_NUM_BARS],
    pub num_ib_atu_indexes: [c_uint; PCI_STD_NUM_BARS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie_ep {
    pub epc: *mut pci_epc,
    pub func_list: list_head,
    pub ops: *const dw_pcie_ep_ops,
    pub phys_base: phys_addr_t,
    pub addr_size: usize,
    pub page_size: usize,
    pub outbound_addr: *mut phys_addr_t,
    pub ib_window_map: *mut c_ulong,
    pub ob_window_map: *mut c_ulong,
    pub msi_mem: *mut void __iomem,
    pub msi_mem_phys: phys_addr_t,
// MSI outbound iATU state
    pub msi_iatu_mapped: bool,
    pub msi_iatu_mapped_offset: usize,
    pub msi_msg_addr: u64,
    pub msi_map_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie_ops {
    pub cpu_addr): *mut *mut *mut u64 (cpu_addr_fixup)(struct dw_pcie pcie, u64,
    pub size): usize,
    pub val): size_t size, u32,
    pub val): size_t size, u32,
    pub pcie): *mut *mut bool (link_up)(struct dw_pcie,
    pub pcie): *mut *mut dw_pcie_ltssm (get_ltssm)(struct dw_pcie,
    pub pcie): *mut *mut int (start_link)(struct dw_pcie,
    pub pcie): *mut *mut void (stop_link)(struct dw_pcie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_info {
    pub debug_dir: *mut dentry,
    pub rasdes_info: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_pcie {
    pub dev: *mut device,
    pub dbi_base: *mut void __iomem,
    pub dbi_phys_addr: resource_size_t,
    pub dbi_base2: *mut void __iomem,
    pub atu_base: *mut void __iomem,
    pub elbi_base: *mut void __iomem,
    pub atu_phys_addr: resource_size_t,
    pub atu_size: usize,
    pub parent_bus_offset: resource_size_t,
    pub num_ib_windows: u32,
    pub num_ob_windows: u32,
    pub region_align: u32,
    pub region_limit: u64,
    pub pp: dw_pcie_rp,
    pub ep: dw_pcie_ep,
    pub ops: *const dw_pcie_ops,
    pub version: u32,
    pub type: u32,
    pub caps: c_ulong,
    pub num_lanes: c_int,
    pub max_link_speed: c_int,
    pub n_fts: [u8; 2],
    pub edma: dw_edma_chip,
    pub edma_reg_phys: phys_addr_t,
    pub edma_reg_size: resource_size_t,
    pub /: *mut *mut bool l1ss_support; / L1 PM Substates support,
    pub app_clks: [clk_bulk_data; DW_PCIE_NUM_APP_CLKS],
    pub core_clks: [clk_bulk_data; DW_PCIE_NUM_CORE_CLKS],
    pub app_rsts: [reset_control_bulk_data; DW_PCIE_NUM_APP_RSTS],
    pub core_rsts: [reset_control_bulk_data; DW_PCIE_NUM_CORE_RSTS],
    pub pe_rst: *mut gpio_desc,
    pub suspended: bool,
    pub debugfs: *mut debugfs_info,
    pub mode: dw_pcie_device_mode,
    pub ptm_vsec_offset: u16,
    pub ptm_debugfs: *mut pci_ptm_debugfs,
//
// If iATU input addresses are offset from CPU physical addresses,
// we previously required .cpu_addr_fixup() to convert them.  We
// now rely on the devicetree instead.  If .cpu_addr_fixup()
// exists, we compare its results with devicetree.
//
// If .cpu_addr_fixup() does not exist, we assume the offset is
// zero and warn if devicetree claims otherwise.  If we know all
// devicetrees correctly describe the offset, set
// use_parent_dt_ranges to true to avoid this warning.
//
    pub use_parent_dt_ranges: bool,
}

extern "C" {
    pub fn dw_pcie_get_resources(pci: *mut dw_pcie) -> c_int;
}
extern "C" {
    pub fn dw_pcie_version_detect(pci: *mut dw_pcie);
}
extern "C" {
    pub fn dw_pcie_find_capability(pci: *mut dw_pcie, cap: u8) -> u8;
}
extern "C" {
    pub fn dw_pcie_find_ext_capability(pci: *mut dw_pcie, cap: u8) -> u16;
}
extern "C" {
    pub fn dw_pcie_remove_capability(pci: *mut dw_pcie, cap: u8);
}
extern "C" {
    pub fn dw_pcie_remove_ext_capability(pci: *mut dw_pcie, cap: u8);
}
extern "C" {
    pub fn dw_pcie_find_rasdes_capability(pci: *mut dw_pcie) -> u16;
}
extern "C" {
    pub fn dw_pcie_find_ptm_capability(pci: *mut dw_pcie) -> u16;
}
extern "C" {
    pub fn dw_pcie_read(addr: *mut void __iomem, size: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn dw_pcie_write(addr: *mut void __iomem, size: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn dw_pcie_read_dbi(pci: *mut dw_pcie, reg: u32, size: usize) -> u32;
}
extern "C" {
    pub fn dw_pcie_write_dbi(pci: *mut dw_pcie, reg: u32, size: usize, val: u32);
}
extern "C" {
    pub fn dw_pcie_write_dbi2(pci: *mut dw_pcie, reg: u32, size: usize, val: u32);
}
extern "C" {
    pub fn dw_pcie_link_up(pci: *mut dw_pcie) -> bool;
}
extern "C" {
    pub fn dw_pcie_upconfig_setup(pci: *mut dw_pcie);
}
extern "C" {
    pub fn dw_pcie_wait_for_link(pci: *mut dw_pcie) -> c_int;
}
extern "C" {
    pub fn dw_pcie_link_get_max_link_width(pci: *mut dw_pcie) -> c_int;
}
extern "C" {
    pub fn dw_pcie_disable_atu(pci: *mut dw_pcie, dir: u32, index: c_int);
}
extern "C" {
    pub fn dw_pcie_hide_unsupported_l1ss(pci: *mut dw_pcie);
}
extern "C" {
    pub fn dw_pcie_program_t_power_on(pci: *mut dw_pcie, t_power_on: u32);
}
extern "C" {
    pub fn dw_pcie_setup(pci: *mut dw_pcie);
}
extern "C" {
    pub fn dw_pcie_iatu_detect(pci: *mut dw_pcie);
}
extern "C" {
    pub fn dw_pcie_edma_detect(pci: *mut dw_pcie) -> c_int;
}
extern "C" {
    pub fn dw_pcie_edma_remove(pci: *mut dw_pcie);
}
extern "C" {
    pub fn dw_pcie_read_dbi(_arg: pci, _arg: reg, _arg: 0x4) -> return;
}
extern "C" {
    pub fn dw_pcie_read_dbi(_arg: pci, _arg: reg, _arg: 0x2) -> return;
}
extern "C" {
    pub fn dw_pcie_read_dbi(_arg: pci, _arg: reg, _arg: 0x1) -> return;
}
// val = dw_pcie_readb_dbi(pci, where);
// val = dw_pcie_readw_dbi(pci, where);
// val = dw_pcie_readl_dbi(pci, where);
extern "C" {
    pub fn dw_pcie_read_dbi(_arg: pci, reg: offset +, _arg: size) -> return;
}
extern "C" {
    pub fn dw_pcie_ep_read_dbi(_arg: ep, _arg: func_no, _arg: reg, _arg: 0x4) -> return;
}
extern "C" {
    pub fn dw_pcie_ep_read_dbi(_arg: ep, _arg: func_no, _arg: reg, _arg: 0x2) -> return;
}
extern "C" {
    pub fn dw_pcie_ep_read_dbi(_arg: ep, _arg: func_no, _arg: reg, _arg: 0x1) -> return;
}
// val = dw_pcie_ep_readb_dbi(ep, func_no, where);
// val = dw_pcie_ep_readw_dbi(ep, func_no, where);
// val = dw_pcie_ep_readl_dbi(ep, func_no, where);

extern "C" {
    pub fn dw_pcie_suspend_noirq(pci: *mut dw_pcie) -> c_int;
}
extern "C" {
    pub fn dw_pcie_resume_noirq(pci: *mut dw_pcie) -> c_int;
}
extern "C" {
    pub fn dw_handle_msi_irq(pp: *mut dw_pcie_rp);
}
extern "C" {
    pub fn dw_pcie_msi_init(pp: *mut dw_pcie_rp);
}
extern "C" {
    pub fn dw_pcie_msi_host_init(pp: *mut dw_pcie_rp) -> c_int;
}
extern "C" {
    pub fn dw_pcie_free_msi(pp: *mut dw_pcie_rp);
}
extern "C" {
    pub fn dw_pcie_setup_rc(pp: *mut dw_pcie_rp) -> c_int;
}
extern "C" {
    pub fn dw_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int;
}
extern "C" {
    pub fn dw_pcie_host_deinit(pp: *mut dw_pcie_rp);
}
extern "C" {
    pub fn dw_pcie_allocate_domains(pp: *mut dw_pcie_rp) -> c_int;
}

extern "C" {
    pub fn dw_pcie_ep_linkup(ep: *mut dw_pcie_ep);
}
extern "C" {
    pub fn dw_pcie_ep_linkdown(ep: *mut dw_pcie_ep);
}
extern "C" {
    pub fn dw_pcie_ep_init(ep: *mut dw_pcie_ep) -> c_int;
}
extern "C" {
    pub fn dw_pcie_ep_init_registers(ep: *mut dw_pcie_ep) -> c_int;
}
extern "C" {
    pub fn dw_pcie_ep_deinit(ep: *mut dw_pcie_ep);
}
extern "C" {
    pub fn dw_pcie_ep_cleanup(ep: *mut dw_pcie_ep);
}
extern "C" {
    pub fn dw_pcie_ep_raise_intx_irq(ep: *mut dw_pcie_ep, func_no: u8) -> c_int;
}
extern "C" {
    pub fn dw_pcie_ep_reset_bar(pci: *mut dw_pcie, bar: pci_barno);
}

extern "C" {
    pub fn dwc_pcie_debugfs_init(pci: *mut dw_pcie, mode: dw_pcie_device_mode);
}
extern "C" {
    pub fn dwc_pcie_debugfs_deinit(pci: *mut dw_pcie);
}

