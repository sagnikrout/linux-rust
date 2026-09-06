//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/pcie-rockchip.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Rockchip AXI PCIe controller driver
//
// Copyright (c) 2018 Rockchip, Inc.
//
// Author: Shawn Lin <shawn.lin@rock-chips.com>
//

//
// The upper 16 bits of PCIE_CLIENT_CONFIG are a write mask for the lower 16
// bits.  This allows atomic updates of the register without locking.
//

pub const MAX_LANE_NUM: c_int = 4;
pub const MAX_REGION_LIMIT: c_int = 32;
pub const MIN_EP_APERTURE: c_int = 28;

pub const PCIE_CLIENT_BASE: c_uint = 0x0;

pub const PCIE_CLIENT_DEBUG_LTSSM_L1: c_uint = 0x18;
pub const PCIE_CLIENT_DEBUG_LTSSM_L2: c_uint = 0x19;

pub const PCIE_CLIENT_NEG_LINK_WIDTH_SHIFT: c_int = 6;

pub const PCIE_CLIENT_LINK_STATUS_UP: c_uint = 0x00300000;
pub const PCIE_CLIENT_LINK_STATUS_MASK: c_uint = 0x00300000;

pub const PCIE_CLIENT_INTR_SHIFT: c_int = 5;

pub const PCIE_CORE_CTRL_MGMT_BASE: c_uint = 0x900000;

pub const PCIE_CORE_PL_CONF_LS_MASK: c_uint = 0x00000001;
pub const PCIE_CORE_PL_CONF_LS_READY: c_uint = 0x00000001;
pub const PCIE_CORE_PL_CONF_SPEED_5G: c_uint = 0x00000008;
pub const PCIE_CORE_PL_CONF_SPEED_MASK: c_uint = 0x00000018;
pub const PCIE_CORE_PL_CONF_LANE_MASK: c_uint = 0x00000006;
pub const PCIE_CORE_PL_CONF_LANE_SHIFT: c_int = 1;

pub const PCIE_CORE_CTRL_PLC1_FTS_SHIFT: c_int = 8;
pub const PCIE_CORE_CTRL_PLC1_FTS_CNT: c_uint = 0xffff;

pub const PCIE_CORE_TXCREDIT_CFG1_MUI_MASK: c_uint = 0xFFFF0000;
pub const PCIE_CORE_TXCREDIT_CFG1_MUI_SHIFT: c_int = 16;

pub const PCIE_CORE_LANE_MAP_MASK: c_uint = 0x0000000f;

pub const ROCKCHIP_PCIE_CORE_BAR_CFG_CTRL_DISABLED: c_uint = 0x0;
pub const ROCKCHIP_PCIE_CORE_BAR_CFG_CTRL_IO_32BITS: c_uint = 0x1;
pub const ROCKCHIP_PCIE_CORE_BAR_CFG_CTRL_MEM_32BITS: c_uint = 0x4;
pub const ROCKCHIP_PCIE_CORE_BAR_CFG_CTRL_PREFETCH_MEM_32BITS: c_uint = 0x5;
pub const ROCKCHIP_PCIE_CORE_BAR_CFG_CTRL_MEM_64BITS: c_uint = 0x6;
pub const ROCKCHIP_PCIE_CORE_BAR_CFG_CTRL_PREFETCH_MEM_64BITS: c_uint = 0x7;

pub const PCIE_RC_RP_ATS_BASE: c_uint = 0x400000;
pub const PCIE_RC_CONFIG_NORMAL_BASE: c_uint = 0x800000;
pub const PCIE_EP_PF_CONFIG_REGS_BASE: c_uint = 0x800000;
pub const PCIE_RC_CONFIG_BASE: c_uint = 0xa00000;
pub const PCIE_EP_CONFIG_BASE: c_uint = 0xa00000;

pub const MAX_AXI_IB_ROOTPORT_REGION_NUM: c_int = 3;
pub const MIN_AXI_ADDR_BITS_PASSED: c_int = 8;

pub const PCIE_CORE_AXI_CONF_BASE: c_uint = 0xc00000;

pub const PCIE_CORE_OB_REGION_ADDR0_NUM_BITS: c_uint = 0x3f;

pub const PCIE_CORE_AXI_INBOUND_BASE: c_uint = 0xc00800;

pub const PCIE_CORE_IB_REGION_ADDR0_NUM_BITS: c_uint = 0x3f;

// Size of one AXI Region (not Region 0)

// Size of Region 0, equal to sum of sizes of other regions

pub const OB_REG_SIZE_SHIFT: c_int = 5;
pub const IB_ROOT_PORT_REG_SIZE_SHIFT: c_int = 3;
pub const AXI_WRAPPER_IO_WRITE: c_uint = 0x6;
pub const AXI_WRAPPER_MEM_WRITE: c_uint = 0x2;
pub const AXI_WRAPPER_TYPE0_CFG: c_uint = 0xa;
pub const AXI_WRAPPER_TYPE1_CFG: c_uint = 0xb;
pub const AXI_WRAPPER_NOR_MSG: c_uint = 0xc;
pub const PCIE_RC_SEND_PME_OFF: c_uint = 0x11960;

pub const RC_REGION_0_ADDR_TRANS_H: c_uint = 0x00000000;
pub const RC_REGION_0_ADDR_TRANS_L: c_uint = 0x00000000;

pub const MAX_AXI_WRAPPER_REGION_NUM: c_int = 33;

pub const ROCKCHIP_PCIE_EP_CMD_STATUS: c_uint = 0x4;

pub const ROCKCHIP_PCIE_EP_MSI_CTRL_REG: c_uint = 0x90;
pub const ROCKCHIP_PCIE_EP_MSI_CP1_OFFSET: c_int = 8;

pub const ROCKCHIP_PCIE_EP_MSI_FLAGS_OFFSET: c_int = 16;
pub const ROCKCHIP_PCIE_EP_MSI_CTRL_MMC_OFFSET: c_int = 17;

pub const ROCKCHIP_PCIE_EP_MSI_CTRL_MME_OFFSET: c_int = 20;

pub const ROCKCHIP_PCIE_EP_MSIX_CAP_REG: c_uint = 0xb0;
pub const ROCKCHIP_PCIE_EP_MSIX_CAP_CP_OFFSET: c_int = 8;

pub const ROCKCHIP_PCIE_EP_DUMMY_IRQ_ADDR: c_uint = 0x1;
pub const ROCKCHIP_PCIE_EP_PCI_LEGACY_IRQ_ADDR: c_uint = 0x3;
pub const ROCKCHIP_PCIE_AT_MIN_NUM_BITS: c_int = 8;
pub const ROCKCHIP_PCIE_AT_MAX_NUM_BITS: c_int = 20;

pub const ROCKCHIP_PCIE_AT_MIN_NUM_BITS: c_int = 8;
pub const ROCKCHIP_PCIE_AT_MAX_NUM_BITS: c_int = 20;

// NOTE: Do not reorder the deassert sequence of the following reset pins
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_pcie {
    pub /: *mut *mut *mut void __iomem reg_base; / DT axi-base,
    pub /: *mut *mut *mut void __iomem apb_base; / DT apb-base,
    pub legacy_phy: bool,
    pub phys: [*mut phy; MAX_LANE_NUM],
    pub pm_rsts: [reset_control_bulk_data; ROCKCHIP_NUM_PM_RSTS],
    pub core_rsts: [reset_control_bulk_data; ROCKCHIP_NUM_CORE_RSTS],
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub /: *mut *mut *mut regulator vpcie12v; / 12V power supply,
    pub /: *mut *mut *mut regulator vpcie3v3; / 3.3V power supply,
    pub /: *mut *mut *mut regulator vpcie1v8; / 1.8V power supply,
    pub /: *mut *mut *mut regulator vpcie0v9; / 0.9V power supply,
    pub perst_gpio: *mut gpio_desc,
    pub lanes: u32,
    pub lanes_map: u8,
    pub link_gen: c_int,
    pub dev: *mut device,
    pub irq_domain: *mut irq_domain,
    pub offset: c_int,
    pub msg_region: *mut void __iomem,
    pub msg_bus_addr: phys_addr_t,
    pub is_rc: bool,
    pub mem_res: *mut resource,
}

extern "C" {
    pub fn readl(reg: rockchip->apb_base +) -> return;
}
extern "C" {
    pub fn rockchip_pcie_parse_dt(rockchip: *mut rockchip_pcie) -> c_int;
}
extern "C" {
    pub fn rockchip_pcie_init_port(rockchip: *mut rockchip_pcie) -> c_int;
}
extern "C" {
    pub fn rockchip_pcie_get_phys(rockchip: *mut rockchip_pcie) -> c_int;
}
extern "C" {
    pub fn rockchip_pcie_deinit_phys(rockchip: *mut rockchip_pcie);
}
extern "C" {
    pub fn rockchip_pcie_enable_clocks(rockchip: *mut rockchip_pcie) -> c_int;
}
extern "C" {
    pub fn rockchip_pcie_disable_clocks(rockchip: *mut rockchip_pcie);
}
