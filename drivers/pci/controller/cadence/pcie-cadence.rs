//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/cadence/pcie-cadence.h
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
// Copyright (c) 2017 Cadence
// Cadence PCIe controller driver.
// Author: Cyrille Pitchen <cyrille.pitchen@free-electrons.com>

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdns_pcie_rp_bar {
    RP_BAR_UNDEFINED = -1,
    RP_BAR0,
    RP_BAR1,
    RP_NO_BAR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_pcie_rp_ib_bar {
    pub size: u64,
    pub free: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdns_pcie_reg_bank {
    REG_BANK_RP,
    REG_BANK_IP_REG,
    REG_BANK_IP_CFG_CTRL_REG,
    REG_BANK_AXI_MASTER_COMMON,
    REG_BANK_AXI_MASTER,
    REG_BANK_AXI_SLAVE,
    REG_BANK_AXI_HLS,
    REG_BANK_AXI_RAS,
    REG_BANK_AXI_DTI,
    REG_BANKS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdns_pcie_lga_ltssm {
    CDNS_PCIE_LGA_LTSSM_DETECT_QUIET			= 0x00,
    CDNS_PCIE_LGA_LTSSM_DETECT_ACTIVE			= 0x01,
    CDNS_PCIE_LGA_LTSSM_POLLING_ACTIVE			= 0x02,
    CDNS_PCIE_LGA_LTSSM_POLLING_COMPLIANCE			= 0x03,
    CDNS_PCIE_LGA_LTSSM_POLLING_CONFIGURATION		= 0x04,
    CDNS_PCIE_LGA_LTSSM_CONFIGURATION_LINKWIDTH_START	= 0x05,
    CDNS_PCIE_LGA_LTSSM_CONFIGURATION_LINKWIDTH_ACCEPT	= 0x06,
    CDNS_PCIE_LGA_LTSSM_CONFIGURATION_LANENUM_ACCEPT	= 0x07,
    CDNS_PCIE_LGA_LTSSM_CONFIGURATION_LANENUM_WAIT		= 0x08,
    CDNS_PCIE_LGA_LTSSM_CONFIGURATION_COMPLETE		= 0x09,
    CDNS_PCIE_LGA_LTSSM_CONFIGURATION_IDLE			= 0x0A,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_RCVRLOCK			= 0x0B,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_SPEED			= 0x0C,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_RCVRCFG			= 0x0D,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_IDLE			= 0x0E,
    CDNS_PCIE_LGA_LTSSM_L0					= 0x10,
    CDNS_PCIE_LGA_LTSSM_RX_L0S_ENTRY			= 0x11,
    CDNS_PCIE_LGA_LTSSM_RX_L0S_IDLE				= 0x12,
    CDNS_PCIE_LGA_LTSSM_RX_L0S_FTS				= 0x13,
    CDNS_PCIE_LGA_LTSSM_TX_L0S_ENTRY			= 0x14,
    CDNS_PCIE_LGA_LTSSM_TX_L0S_IDLE				= 0x15,
    CDNS_PCIE_LGA_LTSSM_TX_L0S_FTS				= 0x16,
    CDNS_PCIE_LGA_LTSSM_L1_ENTRY				= 0x17,
    CDNS_PCIE_LGA_LTSSM_L1_IDLE				= 0x18,
    CDNS_PCIE_LGA_LTSSM_L2_IDLE				= 0x19,
    CDNS_PCIE_LGA_LTSSM_L2_TRANSMITWAKE			= 0x1A,
    CDNS_PCIE_LGA_LTSSM_DISABLED				= 0x20,
    CDNS_PCIE_LGA_LTSSM_LOOPBACK_ENTRY_MASTER		= 0x21,
    CDNS_PCIE_LGA_LTSSM_LOOPBACK_ACTIVE_MASTER		= 0x22,
    CDNS_PCIE_LGA_LTSSM_LOOPBACK_EXIT_MASTER		= 0x23,
    CDNS_PCIE_LGA_LTSSM_LOOPBACK_ENTRY_SLAVE		= 0x24,
    CDNS_PCIE_LGA_LTSSM_LOOPBACK_ACTIVE_SLAVE		= 0x25,
    CDNS_PCIE_LGA_LTSSM_LOOPBACK_EXIT_SLAVE			= 0x26,
    CDNS_PCIE_LGA_LTSSM_HOT_RESET				= 0x27,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_EQUALIZATION_PHASE_0	= 0x28,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_EQUALIZATION_PHASE_1	= 0x29,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_EQUALIZATION_PHASE_2	= 0x2A,
    CDNS_PCIE_LGA_LTSSM_RECOVERY_EQUALIZATION_PHASE_3	= 0x2B,
    CDNS_PCIE_LGA_LTSSM_UNKNOWN				= 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdns_pcie_hpa_ltssm {
    CDNS_PCIE_HPA_LTSSM_DETECT_QUIET		= 0,
    CDNS_PCIE_HPA_LTSSM_DETECT_QUIET_ENTRY		= 1,
    CDNS_PCIE_HPA_LTSSM_DETECT_ACTIVE		= 2,
    CDNS_PCIE_HPA_LTSSM_DETECT_ACTIVE_1		= 3,
    CDNS_PCIE_HPA_LTSSM_DETECT_ACTIVE_2		= 4,
    CDNS_PCIE_HPA_LTSSM_DETECT_ACTIVE_3		= 5,
    CDNS_PCIE_HPA_LTSSM_RCVR_DETECTED_ST		= 6,
    CDNS_PCIE_HPA_LTSSM_RCVR_DETECTED_1		= 7,
    CDNS_PCIE_HPA_LTSSM_POLLING_ACTIVE		= 8,
    CDNS_PCIE_HPA_LTSSM_POLLING_ACTIVE_1		= 9,
    CDNS_PCIE_HPA_LTSSM_POLLING_ACTIVE_2		= 10,
    CDNS_PCIE_HPA_LTSSM_POLLING_ACTIVE_3		= 11,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE		= 12,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_1	= 13,
    CDNS_PCIE_HPA_LTSSM_POLLING_CONFIG		= 14,
    CDNS_PCIE_HPA_LTSSM_POLLING_CONFIG_1		= 15,
    CDNS_PCIE_HPA_LTSSM_POLLING_CONFIG_2		= 16,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_START_RC		= 17,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_START_RC_1	= 18,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_START_RC_2	= 19,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_ACC_RC		= 20,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LANENUM_WAIT_RC	= 21,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LANENUM_WAIT_RC_1	= 22,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LANENUM_ACC_RC	= 23,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_START_EP		= 24,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_START_EP_1	= 25,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_START_EP_2	= 26,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LW_ACC_EP		= 27,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LANENUM_WAIT_EP	= 28,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LANENUM_WAIT_EP_1	= 29,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LANENUM_ACC_EP	= 30,
    CDNS_PCIE_HPA_LTSSM_CONFIG_LANENUM_ACC_EP_1	= 31,
    CDNS_PCIE_HPA_LTSSM_DUMMY_STATE_1		= 32,
    CDNS_PCIE_HPA_LTSSM_CONFIG_COMPLETE		= 33,
    CDNS_PCIE_HPA_LTSSM_CONFIG_COMPLETE_1		= 34,
    CDNS_PCIE_HPA_LTSSM_CONFIG_COMPLETE_2		= 35,
    CDNS_PCIE_HPA_LTSSM_CONFIG_IDLE			= 36,
    CDNS_PCIE_HPA_LTSSM_CONFIG_IDLE_1		= 37,
    CDNS_PCIE_HPA_LTSSM_DUMMY_STATE_2		= 38,
    CDNS_PCIE_HPA_LTSSM_DUMMY_STATE_3		= 39,
    CDNS_PCIE_HPA_LTSSM_DUMMY_STATE_4		= 40,
    CDNS_PCIE_HPA_LTSSM_L0_STATE			= 41,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_RCVR_LOCK		= 42,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_RCVR_LOCK_1	= 43,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_RCVR_CFG		= 44,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_RCVR_CFG_1		= 45,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_IDLE		= 46,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_IDLE_1		= 47,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK		= 48,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK_1		= 49,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK_2		= 50,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK_3		= 51,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK_4		= 52,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK_5		= 53,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK_6		= 54,
    CDNS_PCIE_HPA_LTSSM_DISABLE_LINK_7		= 55,
    CDNS_PCIE_HPA_LTSSM_HOT_RESET			= 56,
    CDNS_PCIE_HPA_LTSSM_HOT_RESET_1			= 57,
    CDNS_PCIE_HPA_LTSSM_HOT_RESET_2			= 58,
    CDNS_PCIE_HPA_LTSSM_HOT_RESET_3			= 59,
    CDNS_PCIE_HPA_LTSSM_L0S_ENTRY			= 60,
    CDNS_PCIE_HPA_LTSSM_L0S_1			= 61,
    CDNS_PCIE_HPA_LTSSM_L0S_2			= 62,
    CDNS_PCIE_HPA_LTSSM_L0S_3			= 63,
    CDNS_PCIE_HPA_LTSSM_L0S_4			= 64,
    CDNS_PCIE_HPA_LTSSM_L0S_5			= 65,
    CDNS_PCIE_HPA_LTSSM_WAIT_FOR_LINK_TX		= 66,
    CDNS_PCIE_HPA_LTSSM_TX_FTS_ENTRY		= 67,
    CDNS_PCIE_HPA_LTSSM_TX_FTS_1			= 68,
    CDNS_PCIE_HPA_LTSSM_TX_FTS_2			= 69,
    CDNS_PCIE_HPA_LTSSM_TX_ELEC_IDLE_ST		= 70,
    CDNS_PCIE_HPA_LTSSM_TX_ELEC_IDLE_1		= 71,
    CDNS_PCIE_HPA_LTSSM_TX_ELEC_IDLE_2		= 72,
    CDNS_PCIE_HPA_LTSSM_TX_ELEC_IDLE_3		= 73,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_SPEED		= 74,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_SPEED_1		= 75,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_SPEED_2		= 76,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_SPEED_3		= 77,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23	= 78,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_1	= 79,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_2	= 80,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_3	= 81,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_4	= 82,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_5	= 83,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_6	= 84,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_7	= 85,
    CDNS_PCIE_HPA_LTSSM_POLLING_COMPLIANCE_GEN23_8	= 86,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_ENTRY	= 87,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_ENTRY_FROM_RECOVERY = 88,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_EXIT_1	= 89,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_EXIT		= 90,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_GEN2_1	= 91,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_GEN2_2	= 92,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_GEN2_3	= 93,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_GEN2_4	= 94,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_GEN2_5	= 95,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_SLAVE_ACTIVE	= 96,
    CDNS_PCIE_HPA_LTSSM_L1_ENTRY			= 97,
    CDNS_PCIE_HPA_LTSSM_L1_1			= 98,
    CDNS_PCIE_HPA_LTSSM_L1_2			= 99,
    CDNS_PCIE_HPA_LTSSM_L1_3			= 100,
    CDNS_PCIE_HPA_LTSSM_L1_4			= 101,
    CDNS_PCIE_HPA_LTSSM_L1_IDLE			= 102,
    CDNS_PCIE_HPA_LTSSM_L1_EXIT			= 103,
    CDNS_PCIE_HPA_LTSSM_L2_ENTRY			= 104,
    CDNS_PCIE_HPA_LTSSM_L2_1			= 105,
    CDNS_PCIE_HPA_LTSSM_L2_2			= 106,
    CDNS_PCIE_HPA_LTSSM_L2_3			= 107,
    CDNS_PCIE_HPA_LTSSM_L2_4			= 108,
    CDNS_PCIE_HPA_LTSSM_L2_5			= 109,
    CDNS_PCIE_HPA_LTSSM_L2_IDLE			= 110,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ENTRY	= 111,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ENTRY_1	= 112,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ENTRY_2	= 113,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ENTRY_3	= 114,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ENTRY_4	= 115,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ENTRY_5	= 116,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ENTRY_FROM_RECOVERY = 117,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_ACTIVE	= 118,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_EXIT	= 119,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_EXIT_1	= 120,
    CDNS_PCIE_HPA_LTSSM_LOOPBACK_MASTER_EXIT_2	= 121,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_EQUALIZATION_PHASE0 = 122,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_EQUALIZATION_PHASE1 = 123,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_EQUALIZATION_PHASE2_1 = 124,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_EQUALIZATION_PHASE2_2 = 125,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_EQUALIZATION_PHASE3_1 = 126,
    CDNS_PCIE_HPA_LTSSM_RECOVERY_EQUALIZATION_PHASE3_2 = 127,
    CDNS_PCIE_HPA_LTSSM_UNKNOWN			= 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_pcie_ops {
    pub pcie): *mut *mut int (start_link)(struct cdns_pcie,
    pub pcie): *mut *mut void (stop_link)(struct cdns_pcie,
    pub pcie): *mut *mut bool (link_up)(struct cdns_pcie,
    pub cpu_addr): *mut *mut *mut u64 (cpu_addr_fixup)(struct cdns_pcie pcie, u64,
}

//
// struct cdns_plat_pcie_of_data - Register bank offset for a platform
// @is_rc: controller is a RC
// @ip_reg_bank_offset: ip register bank start offset
// @ip_cfg_ctrl_reg_offset: ip config control register start offset
// @axi_mstr_common_offset: AXI master common register start offset
// @axi_slave_offset: AXI slave start offset
// @axi_master_offset: AXI master start offset
// @axi_hls_offset: AXI HLS offset start
// @axi_ras_offset: AXI RAS offset
// @axi_dti_offset: AXI DTI offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_plat_pcie_of_data {
    pub is_rc:1: u32,
    pub ip_reg_bank_offset: u32,
    pub ip_cfg_ctrl_reg_offset: u32,
    pub axi_mstr_common_offset: u32,
    pub axi_slave_offset: u32,
    pub axi_master_offset: u32,
    pub axi_hls_offset: u32,
    pub axi_ras_offset: u32,
    pub axi_dti_offset: u32,
}

//
// struct cdns_pcie - private data for Cadence PCIe controller drivers
// @reg_base: IO mapped register base
// @mem_res: start/end offsets in the physical system memory to map PCI accesses
// @msg_res: Region for send message to map PCI accesses
// @dev: PCIe controller
// @is_rc: tell whether the PCIe controller mode is Root Complex or Endpoint.
// @is_hpa: indicates if the architecture is HPA
// @phy_count: number of supported PHY devices
// @phy: list of pointers to specific PHY control blocks
// @link: list of pointers to corresponding device link representations
// @ops: Platform-specific ops to control various inputs from Cadence PCIe
// wrapper
// @cdns_pcie_reg_offsets: Register bank offsets for different SoC
// @max_link_speed: Maximum supported link speed
// @debug_dir: debugfs node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_pcie {
    pub reg_base: *mut void __iomem,
    pub mem_res: *mut resource,
    pub msg_res: *mut resource,
    pub dev: *mut device,
    pub is_rc: bool,
    pub is_hpa: bool,
    pub phy_count: c_int,
    pub phy: *mut phy,
    pub link: *mut device_link,
    pub ops: *const cdns_pcie_ops,
    pub cdns_pcie_reg_offsets: *const cdns_plat_pcie_of_data,
    pub max_link_speed: c_int,
    pub debug_dir: *mut dentry,
}

//
// struct cdns_pcie_rc - private data for this PCIe Root Complex driver
// @pcie: Cadence PCIe controller
// @cfg_res: start/end offsets in the physical system memory to map PCI
// configuration space accesses
// @cfg_base: IO mapped window to access the PCI configuration space of a
// single function at a time
// @vendor_id: PCI vendor ID
// @device_id: PCI device ID
// @avail_ib_bar: Status of RP_BAR0, RP_BAR1 and RP_NO_BAR if it's free or
// available
// @quirk_retrain_flag: Retrain link as quirk for PCIe Gen2
// @quirk_detect_quiet_flag: LTSSM Detect Quiet min delay set as quirk
// @ecam_supported: Whether the ECAM is supported
// @no_inbound_map: Whether inbound mapping is supported
// @quirk_broken_aspm_l0s: Disable ASPM L0s support as quirk
// @quirk_broken_aspm_l1: Disable ASPM L1 support as quirk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_pcie_rc {
    pub pcie: cdns_pcie,
    pub cfg_res: *mut resource,
    pub cfg_base: *mut void __iomem,
    pub vendor_id: u32,
    pub device_id: u32,
    pub avail_ib_bar: [bool; CDNS_PCIE_RP_MAX_IB],
    pub quirk_retrain_flag:1: c_uint,
    pub quirk_detect_quiet_flag:1: c_uint,
    pub ecam_supported:1: c_uint,
    pub no_inbound_map:1: c_uint,
    pub quirk_broken_aspm_l0s:1: c_uint,
    pub quirk_broken_aspm_l1:1: c_uint,
}

//
// struct cdns_pcie_epf - Structure to hold info about endpoint function
// @epf: Info about virtual functions attached to the physical function
// @epf_bar: reference to the pci_epf_bar for the six Base Address Registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_pcie_epf {
    pub epf: *mut cdns_pcie_epf,
    pub epf_bar: [*mut pci_epf_bar; PCI_STD_NUM_BARS],
}

//
// struct cdns_pcie_ep - private data for this PCIe endpoint controller driver
// @pcie: Cadence PCIe controller
// @max_regions: maximum number of regions supported by hardware
// @ob_region_map: bitmask of mapped outbound regions
// @ob_addr: base addresses in the AXI bus where the outbound regions start
// @irq_phys_addr: base address on the AXI bus where the MSI/INTX IRQ
// dedicated outbound regions is mapped.
// @irq_cpu_addr: base address in the CPU space where a write access triggers
// the sending of a memory write (MSI) / normal message (INTX
// IRQ) TLP through the PCIe bus.
// @irq_pci_addr: used to save the current mapping of the MSI/INTX IRQ
// dedicated outbound region.
// @irq_pci_fn: the latest PCI function that has updated the mapping of
// the MSI/INTX IRQ dedicated outbound region.
// @irq_pending: bitmask of asserted INTX IRQs.
// @lock: spin lock to disable interrupts while modifying PCIe controller
// registers fields (RMW) accessible by both remote RC and EP to
// minimize time between read and write
// @epf: Structure to hold info about endpoint function
// @quirk_detect_quiet_flag: LTSSM Detect Quiet min delay set as quirk
// @quirk_disable_flr: Disable FLR (Function Level Reset) quirk flag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_pcie_ep {
    pub pcie: cdns_pcie,
    pub max_regions: u32,
    pub ob_region_map: c_ulong,
    pub ob_addr: *mut phys_addr_t,
    pub irq_phys_addr: phys_addr_t,
    pub irq_cpu_addr: *mut void __iomem,
    pub irq_pci_addr: u64,
    pub irq_pci_fn: u8,
    pub irq_pending: u8,
// protect writing to PCI_STATUS while raising INTX interrupts
    pub lock: spinlock_t,
    pub epf: *mut cdns_pcie_epf,
    pub quirk_detect_quiet_flag:1: c_uint,
    pub quirk_disable_flr:1: c_uint,
}

// Register access
extern "C" {
    pub fn readl(reg: pcie->reg_base +) -> return;
}
extern "C" {
    pub fn readl(reg: pcie->reg_base +) -> return;
}
// val = cdns_pcie_read_sz(addr, 0x1);
// val = cdns_pcie_read_sz(addr, 0x2);
// val = cdns_pcie_readl(pcie, where);
// Root Port register access
extern "C" {
    pub fn cdns_pcie_read_sz(_arg: addr, _arg: 0x2) -> return;
}
extern "C" {
    pub fn cdns_pcie_read_sz(_arg: addr, _arg: 0x4) -> return;
}
extern "C" {
    pub fn cdns_pcie_read_sz(_arg: addr, _arg: 0x2) -> return;
}
// Endpoint Function register access
extern "C" {
    pub fn cdns_pcie_read_sz(_arg: addr, _arg: 0x2) -> return;
}
extern "C" {
    pub fn readl(reg: pcie->reg_base + CDNS_PCIE_EP_FUNC_BASE(fn) +) -> return;
}

extern "C" {
    pub fn cdns_pcie_host_link_setup(rc: *mut cdns_pcie_rc) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_host_init(rc: *mut cdns_pcie_rc) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_host_setup(rc: *mut cdns_pcie_rc) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_host_disable(rc: *mut cdns_pcie_rc);
}
extern "C" {
    pub fn cdns_pcie_hpa_host_setup(rc: *mut cdns_pcie_rc) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_hpa_host_disable(rc: *mut cdns_pcie_rc);
}

extern "C" {
    pub fn cdns_pcie_ep_setup(ep: *mut cdns_pcie_ep) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_ep_disable(ep: *mut cdns_pcie_ep);
}
extern "C" {
    pub fn cdns_pcie_hpa_ep_setup(ep: *mut cdns_pcie_ep) -> c_int;
}

extern "C" {
    pub fn cdns_pcie_find_capability(pcie: *mut cdns_pcie, cap: u8) -> u8;
}
extern "C" {
    pub fn cdns_pcie_find_ext_capability(pcie: *mut cdns_pcie, cap: u8) -> u16;
}
extern "C" {
    pub fn cdns_pcie_linkup(pcie: *mut cdns_pcie) -> bool;
}
extern "C" {
    pub fn cdns_pcie_detect_quiet_min_delay_set(pcie: *mut cdns_pcie);
}
extern "C" {
    pub fn cdns_pcie_reset_outbound_region(pcie: *mut cdns_pcie, r: u32);
}
extern "C" {
    pub fn cdns_pcie_disable_phy(pcie: *mut cdns_pcie);
}
extern "C" {
    pub fn cdns_pcie_enable_phy(pcie: *mut cdns_pcie) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_init_phy(dev: *mut device, pcie: *mut cdns_pcie) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_hpa_detect_quiet_min_delay_set(pcie: *mut cdns_pcie);
}
extern "C" {
    pub fn cdns_pcie_hpa_host_link_setup(rc: *mut cdns_pcie_rc) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_hpa_host_start_link(rc: *mut cdns_pcie_rc) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_hpa_start_link(pcie: *mut cdns_pcie) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_hpa_stop_link(pcie: *mut cdns_pcie);
}
extern "C" {
    pub fn cdns_pcie_hpa_link_up(pcie: *mut cdns_pcie) -> bool;
}

extern "C" {
    pub fn cdns_pcie_debugfs_deinit(pci: *mut cdns_pcie);
}
extern "C" {
    pub fn cdns_pcie_debugfs_init(pci: *mut cdns_pcie);
}

