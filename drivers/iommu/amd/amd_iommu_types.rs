//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/amd/amd_iommu_types.h
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
//
// Copyright (C) 2007-2010 Advanced Micro Devices, Inc.
// Author: Joerg Roedel <jroedel@suse.de>
// Leo Duran <leo.duran@amd.com>
//

//
// Maximum number of IOMMUs supported
//
pub const MAX_IOMMUS: c_int = 32;
//
// some size calculation constants
//
pub const DEV_TABLE_ENTRY_SIZE: c_int = 32;
// Capability offsets used by the driver
pub const MMIO_CAP_HDR_OFFSET: c_uint = 0x00;
pub const MMIO_RANGE_OFFSET: c_uint = 0x0c;
pub const MMIO_MISC_OFFSET: c_uint = 0x10;
// Masks, shifts and macros to parse the device range capability
pub const MMIO_RANGE_LD_MASK: c_uint = 0xff000000;
pub const MMIO_RANGE_FD_MASK: c_uint = 0x00ff0000;
pub const MMIO_RANGE_BUS_MASK: c_uint = 0x0000ff00;
pub const MMIO_RANGE_LD_SHIFT: c_int = 24;
pub const MMIO_RANGE_FD_SHIFT: c_int = 16;
pub const MMIO_RANGE_BUS_SHIFT: c_int = 8;

// Used offsets into the MMIO space
pub const MMIO_DEV_TABLE_OFFSET: c_uint = 0x0000;
pub const MMIO_CMD_BUF_OFFSET: c_uint = 0x0008;
pub const MMIO_EVT_BUF_OFFSET: c_uint = 0x0010;
pub const MMIO_CONTROL_OFFSET: c_uint = 0x0018;
pub const MMIO_EXCL_BASE_OFFSET: c_uint = 0x0020;
pub const MMIO_EXCL_LIMIT_OFFSET: c_uint = 0x0028;
pub const MMIO_EXT_FEATURES: c_uint = 0x0030;
pub const MMIO_PPR_LOG_OFFSET: c_uint = 0x0038;
pub const MMIO_GA_LOG_BASE_OFFSET: c_uint = 0x00e0;
pub const MMIO_GA_LOG_TAIL_OFFSET: c_uint = 0x00e8;
pub const MMIO_MSI_ADDR_LO_OFFSET: c_uint = 0x015C;
pub const MMIO_MSI_ADDR_HI_OFFSET: c_uint = 0x0160;
pub const MMIO_MSI_DATA_OFFSET: c_uint = 0x0164;
pub const MMIO_INTCAPXT_EVT_OFFSET: c_uint = 0x0170;
pub const MMIO_INTCAPXT_PPR_OFFSET: c_uint = 0x0178;
pub const MMIO_INTCAPXT_GALOG_OFFSET: c_uint = 0x0180;
pub const MMIO_EXT_FEATURES2: c_uint = 0x01A0;
pub const MMIO_CMD_HEAD_OFFSET: c_uint = 0x2000;
pub const MMIO_CMD_TAIL_OFFSET: c_uint = 0x2008;
pub const MMIO_EVT_HEAD_OFFSET: c_uint = 0x2010;
pub const MMIO_EVT_TAIL_OFFSET: c_uint = 0x2018;
pub const MMIO_STATUS_OFFSET: c_uint = 0x2020;
pub const MMIO_PPR_HEAD_OFFSET: c_uint = 0x2030;
pub const MMIO_PPR_TAIL_OFFSET: c_uint = 0x2038;
pub const MMIO_GA_HEAD_OFFSET: c_uint = 0x2040;
pub const MMIO_GA_TAIL_OFFSET: c_uint = 0x2048;
pub const MMIO_CNTR_CONF_OFFSET: c_uint = 0x4000;
pub const MMIO_CNTR_REG_OFFSET: c_uint = 0x40000;
pub const MMIO_REG_END_OFFSET: c_uint = 0x80000;
// Extended Feature Bits

// Extended Feature 2 Bits

// Note:
// The current driver only support 16-bit PASID.
// Currently, hardware only implement upto 16-bit PASID
// even though the spec says it could have upto 20 bits.
//
pub const PASID_MASK: c_uint = 0x0000ffff;
// MMIO status bits

// event logging constants
pub const EVENT_TYPE_SHIFT: c_int = 28;
pub const EVENT_TYPE_MASK: c_uint = 0xf;
pub const EVENT_TYPE_ILL_DEV: c_uint = 0x1;
pub const EVENT_TYPE_IO_FAULT: c_uint = 0x2;
pub const EVENT_TYPE_DEV_TAB_ERR: c_uint = 0x3;
pub const EVENT_TYPE_PAGE_TAB_ERR: c_uint = 0x4;
pub const EVENT_TYPE_ILL_CMD: c_uint = 0x5;
pub const EVENT_TYPE_CMD_HARD_ERR: c_uint = 0x6;
pub const EVENT_TYPE_IOTLB_INV_TO: c_uint = 0x7;
pub const EVENT_TYPE_INV_DEV_REQ: c_uint = 0x8;
pub const EVENT_TYPE_INV_PPR_REQ: c_uint = 0x9;
pub const EVENT_TYPE_RMP_FAULT: c_uint = 0xd;
pub const EVENT_TYPE_RMP_HW_ERR: c_uint = 0xe;
pub const EVENT_DEVID_MASK: c_uint = 0xffff;
pub const EVENT_DEVID_SHIFT: c_int = 0;
pub const EVENT_DOMID_MASK_LO: c_uint = 0xffff;
pub const EVENT_DOMID_MASK_HI: c_uint = 0xf0000;
pub const EVENT_FLAGS_MASK: c_uint = 0xfff;
pub const EVENT_FLAGS_SHIFT: c_uint = 0x10;
pub const EVENT_FLAG_RW: c_uint = 0x020;
pub const EVENT_FLAG_I: c_uint = 0x008;
pub const EVENT_FLAG_PPR_RX: c_uint = 0x001;
pub const EVENT_FLAG_PPR_GN: c_uint = 0x200;
// feature control bits
pub const CONTROL_IOMMU_EN: c_int = 0;
pub const CONTROL_HT_TUN_EN: c_int = 1;
pub const CONTROL_EVT_LOG_EN: c_int = 2;
pub const CONTROL_EVT_INT_EN: c_int = 3;
pub const CONTROL_COMWAIT_EN: c_int = 4;
pub const CONTROL_INV_TIMEOUT: c_int = 5;
pub const CONTROL_PASSPW_EN: c_int = 8;
pub const CONTROL_RESPASSPW_EN: c_int = 9;
pub const CONTROL_COHERENT_EN: c_int = 10;
pub const CONTROL_ISOC_EN: c_int = 11;
pub const CONTROL_CMDBUF_EN: c_int = 12;
pub const CONTROL_PPRLOG_EN: c_int = 13;
pub const CONTROL_PPRINT_EN: c_int = 14;
pub const CONTROL_PPR_EN: c_int = 15;
pub const CONTROL_GT_EN: c_int = 16;
pub const CONTROL_GA_EN: c_int = 17;
pub const CONTROL_GAM_EN: c_int = 25;
pub const CONTROL_GALOG_EN: c_int = 28;
pub const CONTROL_GAINT_EN: c_int = 29;
pub const CONTROL_NUM_INT_REMAP_MODE: c_int = 43;
pub const CONTROL_NUM_INT_REMAP_MODE_MASK: c_uint = 0x03;
pub const CONTROL_NUM_INT_REMAP_MODE_2K: c_uint = 0x01;
pub const CONTROL_EPH_EN: c_int = 45;
pub const CONTROL_XT_EN: c_int = 50;
pub const CONTROL_INTCAPXT_EN: c_int = 51;
pub const CONTROL_GCR3TRPMODE: c_int = 58;
pub const CONTROL_IRTCACHEDIS: c_int = 59;
pub const CONTROL_SNPAVIC_EN: c_int = 61;
pub const CTRL_INV_TO_MASK: c_int = 7;
pub const CTRL_INV_TO_NONE: c_int = 0;
pub const CTRL_INV_TO_1MS: c_int = 1;
pub const CTRL_INV_TO_10MS: c_int = 2;
pub const CTRL_INV_TO_100MS: c_int = 3;
pub const CTRL_INV_TO_1S: c_int = 4;
pub const CTRL_INV_TO_10S: c_int = 5;
pub const CTRL_INV_TO_100S: c_int = 6;
// command specific defines
pub const CMD_COMPL_WAIT: c_uint = 0x01;
pub const CMD_INV_DEV_ENTRY: c_uint = 0x02;
pub const CMD_INV_IOMMU_PAGES: c_uint = 0x03;
pub const CMD_INV_IOTLB_PAGES: c_uint = 0x04;
pub const CMD_INV_IRT: c_uint = 0x05;
pub const CMD_COMPLETE_PPR: c_uint = 0x07;
pub const CMD_INV_ALL: c_uint = 0x08;
pub const CMD_COMPL_WAIT_STORE_MASK: c_uint = 0x01;
pub const CMD_COMPL_WAIT_INT_MASK: c_uint = 0x02;
pub const CMD_INV_IOMMU_PAGES_SIZE_MASK: c_uint = 0x01;
pub const CMD_INV_IOMMU_PAGES_PDE_MASK: c_uint = 0x02;
pub const CMD_INV_IOMMU_PAGES_GN_MASK: c_uint = 0x04;
pub const PPR_STATUS_MASK: c_uint = 0xf;
pub const PPR_STATUS_SHIFT: c_int = 12;
pub const CMD_INV_IOMMU_ALL_PAGES_ADDRESS: c_uint = 0x7ffffffffffff000ULL;
// macros and definitions for device table entries
pub const DEV_ENTRY_VALID: c_uint = 0x00;
pub const DEV_ENTRY_TRANSLATION: c_uint = 0x01;
pub const DEV_ENTRY_HAD: c_uint = 0x07;
pub const DEV_ENTRY_PPR: c_uint = 0x34;
pub const DEV_ENTRY_IR: c_uint = 0x3d;
pub const DEV_ENTRY_IW: c_uint = 0x3e;
pub const DEV_ENTRY_NO_PAGE_FAULT: c_uint = 0x62;
pub const DEV_ENTRY_SYSMGT1: c_uint = 0x68;
pub const DEV_ENTRY_SYSMGT2: c_uint = 0x69;

pub const DEV_ENTRY_IRQ_TBL_EN: c_uint = 0x80;
pub const DEV_ENTRY_INIT_PASS: c_uint = 0xb8;
pub const DEV_ENTRY_EINT_PASS: c_uint = 0xb9;
pub const DEV_ENTRY_NMI_PASS: c_uint = 0xba;
pub const DEV_ENTRY_LINT0_PASS: c_uint = 0xbe;
pub const DEV_ENTRY_LINT1_PASS: c_uint = 0xbf;
pub const DEV_ENTRY_MODE_MASK: c_uint = 0x07;
pub const DEV_ENTRY_MODE_SHIFT: c_uint = 0x09;
pub const MAX_DEV_TABLE_ENTRIES: c_uint = 0xffff;
// constants to configure the command buffer
pub const CMD_BUFFER_SIZE: c_int = 8192;
pub const CMD_BUFFER_UNINITIALIZED: c_int = 1;
pub const CMD_BUFFER_ENTRIES: c_int = 512;
pub const MMIO_CMD_SIZE_SHIFT: c_int = 56;

// constants for event buffer handling
pub const EVTLOG_ENTRY_SIZE: c_uint = 0x10;
pub const EVTLOG_SIZE_SHIFT: c_int = 56;

// Constants for PPR Log handling
pub const PPRLOG_ENTRY_SIZE: c_uint = 0x10;
pub const PPRLOG_SIZE_SHIFT: c_int = 56;

// PAGE_SERVICE_REQUEST PPR Log Buffer Entry flags
pub const PPR_FLAG_EXEC: c_uint = 0x002	/* Execute permission requested */;
pub const PPR_FLAG_READ: c_uint = 0x004	/* Read permission requested */;
pub const PPR_FLAG_WRITE: c_uint = 0x020	/* Write permission requested */;
pub const PPR_FLAG_US: c_uint = 0x040	/* 1: User, 0: Supervisor */;
pub const PPR_FLAG_RVSD: c_uint = 0x080	/* Reserved bit not zero */;
pub const PPR_FLAG_GN: c_uint = 0x100	/* GVA and PASID is valid */;

pub const PPR_REQ_FAULT: c_uint = 0x01;
// Constants for GA Log handling
pub const GA_LOG_ENTRIES: c_int = 512;
pub const GA_LOG_SIZE_SHIFT: c_int = 56;

pub const GA_ENTRY_SIZE: c_int = 8;

pub const GA_GUEST_NR: c_uint = 0x1;
//
// This bitmap is used to advertise the page sizes our hardware support
// to the IOMMU core, which will then use this information to split
// physically contiguous memory regions it is mapping into page sizes
// that we support.
//
// 512GB Pages are not supported due to a hardware bug
// Page sizes >= the 52 bit max physical address of the CPU are not supported.
//

// Special mode where page-sizes are limited to 4 KiB

// 4K, 2MB, 1G page sizes are supported

// Bit value definition for dte irq remapping fields

pub const DTE_INTTAB_ALIGNMENT: c_int = 128;

pub const PAGE_MODE_NONE: c_uint = 0x00;
pub const PAGE_MODE_1_LEVEL: c_uint = 0x01;
pub const PAGE_MODE_2_LEVEL: c_uint = 0x02;
pub const PAGE_MODE_3_LEVEL: c_uint = 0x03;
pub const PAGE_MODE_4_LEVEL: c_uint = 0x04;
pub const PAGE_MODE_5_LEVEL: c_uint = 0x05;
pub const PAGE_MODE_6_LEVEL: c_uint = 0x06;
pub const PAGE_MODE_7_LEVEL: c_uint = 0x07;
pub const GUEST_PGTABLE_4_LEVEL: c_uint = 0x00;
pub const GUEST_PGTABLE_5_LEVEL: c_uint = 0x01;
pub const PM_ADDR_MASK: c_uint = 0x000ffffffffff000ULL;
//
// Bit value definition for DTE fields
//

pub const DTE_GPT_LEVEL_SHIFT: c_int = 54;

pub const GCR3_VALID: c_uint = 0x01ULL;
// DTE[128:179] | DTE[184:191]

pub const IOMMU_PROT_MASK: c_uint = 0x03;
pub const IOMMU_PROT_IR: c_uint = 0x01;
pub const IOMMU_PROT_IW: c_uint = 0x02;
// IOMMU capabilities
pub const IOMMU_CAP_IOTLB: c_int = 24;
pub const IOMMU_CAP_NPCACHE: c_int = 26;
pub const IOMMU_CAP_EFR: c_int = 27;
// IOMMU IVINFO
pub const IOMMU_IVINFO_OFFSET: c_int = 36;

// IOMMU Feature Reporting Field (for IVHD type 10h
pub const IOMMU_FEAT_GASUP_SHIFT: c_int = 6;
// IOMMU HATDIS for IVHD type 11h and 40h
pub const IOMMU_IVHD_ATTR_HATDIS_SHIFT: c_int = 0;
// IOMMU Extended Feature Register (EFR)
pub const IOMMU_EFR_XTSUP_SHIFT: c_int = 2;
pub const IOMMU_EFR_GASUP_SHIFT: c_int = 7;
pub const IOMMU_EFR_MSICAPMMIOSUP_SHIFT: c_int = 46;
pub const MAX_DOMAIN_ID: c_int = 65536;
// Timeout stuff
pub const LOOP_TIMEOUT: c_int = 100000;
pub const MMIO_STATUS_TIMEOUT: c_int = 2000000;

// SNP page mode 0 support
// global flag if IOMMUs cache non-present entries
// Only true if all IOMMUs support device IOTLBs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_remap_table {
    pub lock: raw_spinlock_t,
    pub min_index: unsigned,
    pub table: *mut u32,
}

// Interrupt remapping feature used?
// IVRS indicates that pre-boot remapping was enabled

// Make iterating over all pci segment easier

//
// Make iterating over all IOMMUs easier
//

// Making iterating over protection_domain->dev_data_list easier

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcr3_tbl_info {
    pub /: *mut *mut *mut u64 gcr3_tbl; / Guest CR3 table,
    pub /: *mut *mut int glx; / Number of levels for GCR3 table,
    pub /: *mut *mut u32 pasid_cnt; / Track attached PASIDs,
    pub /: *mut *mut u16 domid; / Per device domain ID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum protection_domain_mode {
    PD_MODE_NONE,
    PD_MODE_V1,
    PD_MODE_V2,
}

// Track dev_data/PASID list for the protection domain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdom_dev_data {
// Points to attached device data
    pub dev_data: *mut iommu_dev_data,
// PASID attached to the protection domain
    pub pasid: ioasid_t,
// For protection_domain->dev_data_list
    pub list: list_head,
}

// Keeps track of the IOMMUs attached to protection domain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdom_iommu_info {
    pub /: *mut *mut *mut amd_iommu iommu; / IOMMUs attach to protection domain,
    pub /: *mut *mut u32 refcnt; / Count of attached dev/pasid per domain/IOMMU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_iommu_viommu {
    pub core: iommufd_viommu,
    pub /: *mut *mut *mut protection_domain parent; / nest parent domain for this viommu,
    pub /: *mut *mut list_head pdom_list; / For protection_domain->viommu_list,
//
// Per-vIOMMU guest domain ID to host domain ID mapping.
// Indexed by guest domain ID.
//
    pub gdomid_array: xarray,
}

//
// Contains guest domain ID mapping info,
// which is stored in the struct xarray gdomid_array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guest_domain_mapping_info {
    pub users: refcount_t,
    pub /: *mut *mut u32 hdom_id; / Host domain ID,
}

//
// Nested domain is specifically used for nested translation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nested_domain {
    pub /: *mut *mut iommu_domain domain; / generic domain handle used by iommu core code,
    pub /: *mut *mut u16 gdom_id; / domain ID from gDTE,
    pub gdom_info: *mut guest_domain_mapping_info,
    pub /: *mut *mut iommu_hwpt_amd_guest gdte; / Guest vIOMMU DTE,
    pub /: *mut *mut *mut amd_iommu_viommu viommu; / AMD hw-viommu this nested domain belong to,
}

//
// This structure contains generic data for  IOMMU protection domains
// independent of their use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protection_domain {
    pub domain: iommu_domain,
    pub iommu: pt_iommu,
    pub amdv1: pt_iommu_amdv1,
    pub amdv2: pt_iommu_x86_64,
}

//
// Store reference to list of vIOMMUs, which use this protection domain.
// This will be used to look up host domain ID when flushing this domain.
//
// This structure contains information about one PCI segment in the system.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_iommu_pci_seg {
// List with all PCI segments in the system
    pub list: list_head,
// List of all available dev_data structures
    pub dev_data_list: llist_head,
// PCI segment number
    pub id: u16,
// Largest PCI device id we expect translation requests for
    pub last_bdf: u16,
// Size of the device table
    pub dev_table_size: u32,
//
// device table virtual address
//
// Pointer to the per PCI segment device table.
// It is indexed by the PCI device id or the HT unit id and contains
// information about the domain the device belongs to as well as the
// page table root pointer.
//
    pub dev_table: *mut dev_table_entry,
//
// The rlookup iommu table is used to find the IOMMU which is
// responsible for a specific device. It is indexed by the PCI
// device id.
//
    pub rlookup_table: *mut amd_iommu,
//
// This table is used to find the irq remapping table for a given
// device id quickly.
//
    pub irq_lookup_table: *mut irq_remap_table,
//
// Pointer to a device table which the content of old device table
// will be copied to. It's only be used in kdump kernel.
//
    pub old_dev_tbl_cpy: *mut dev_table_entry,
//
// The alias table is a driver specific data structure which contains the
// mappings of the PCI device ids to the actual requestor ids on the IOMMU.
// More than one device can share the same requestor id.
//
    pub alias_table: *mut u16,
//
// A list of required unity mappings we find in ACPI. It is not locked
// because as runtime it is only read. It is created at ACPI table
// parsing time.
//
    pub unity_map: list_head,
}

//
// Structure where we save information about one hardware AMD IOMMU in the
// system.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_iommu {
    pub list: list_head,
// Index within the IOMMU array
    pub index: c_int,
// locks the accesses to the hardware
    pub lock: raw_spinlock_t,
// Pointer to PCI device of this IOMMU
    pub dev: *mut pci_dev,
// Cache pdev to root device for resume quirks
    pub root_pdev: *mut pci_dev,
// physical address of MMIO space
    pub mmio_phys: u64,
// physical end address of MMIO space
    pub mmio_phys_end: u64,
// virtual address of MMIO space
    pub mmio_base: *mut u8 __iomem,
// capabilities of that IOMMU read from ACPI
    pub cap: u32,
// flags read from acpi table
    pub acpi_flags: u8,
// Extended features
    pub features: u64,
// Extended features 2
    pub features2: u64,
// PCI device id of the IOMMU device
    pub devid: u16,
//
// Capability pointer. There could be more than one IOMMU per PCI
// device function if there are more than one AMD IOMMU capability
// pointers.
//
    pub cap_ptr: u16,
// pci domain of this IOMMU
    pub pci_seg: *mut amd_iommu_pci_seg,
// command buffer virtual address
    pub cmd_buf: *mut u8,
    pub cmd_buf_head: u32,
    pub cmd_buf_tail: u32,
// event buffer virtual address
    pub evt_buf: *mut u8,
// Name for event log interrupt
    pub evt_irq_name: [c_uchar; 16],
// Base of the PPR log, if present
    pub ppr_log: *mut u8,
// Name for PPR log interrupt
    pub ppr_irq_name: [c_uchar; 16],
// Base of the GA log, if present
    pub ga_log: *mut u8,
// Name for GA log interrupt
    pub ga_irq_name: [c_uchar; 16],
// Tail of the GA log, if present
    pub ga_log_tail: *mut u8,
// true if interrupts for this IOMMU are already enabled
    pub int_enabled: bool,
// if one, we need to send a completion wait command
    pub need_sync: bool,
// true if disable irte caching
    pub irtcachedis_enabled: bool,
// Handle for IOMMU core code
    pub iommu: iommu_device,
//
// We can't rely on the BIOS to restore all values on reinit, so we
// need to stash them
//
// The iommu BAR
    pub stored_addr_lo: u32,
    pub stored_addr_hi: u32,
//
// Each iommu has 6 l1s, each of which is documented as having 0x12
// registers
//
    pub stored_l1: [u32; 6][0x12],
// The l2 indirect registers
    pub stored_l2: [u32; 0x83],
// The maximum PC banks and counters/bank (PCSup=1)
    pub max_banks: u8,
    pub max_counters: u8,

    pub ir_domain: *mut irq_domain,
    pub irte_ops: *mut amd_irte_ops,

    pub flags: u32,
    pub cmd_sem: *mut volatile u64,
    pub cmd_sem_val: u64,
//
// Track physical address to directly use it in build_completion_wait()
// and avoid adding any special checks and handling for kdump.
//
    pub cmd_sem_paddr: u64,

// DebugFS Info
    pub debugfs: *mut dentry,
    pub dbg_mmio_offset: c_int,
    pub dbg_cap_offset: c_int,

// IOPF support
    pub iopf_queue: *mut iopf_queue,
    pub iopfq_name: [c_uchar; 32],
}

extern "C" {
    pub fn container_of(_arg: iommu, amd_iommu: struct, _arg: iommu) -> return;
}
pub const ACPIHID_UID_LEN: c_int = 256;
pub const ACPIHID_HID_LEN: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpihid_map_entry {
    pub list: list_head,
    pub uid: [u8; ACPIHID_UID_LEN],
    pub hid: [u8; ACPIHID_HID_LEN],
    pub devid: u32,
    pub root_devid: u32,
    pub cmd_line: bool,
    pub group: *mut iommu_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devid_map {
    pub list: list_head,
    pub id: u8,
    pub devid: u32,
    pub cmd_line: bool,
}

pub const AMD_IOMMU_DEVICE_FLAG_ATS_SUP: c_uint = 0x1    /* ATS feature supported */;
pub const AMD_IOMMU_DEVICE_FLAG_PRI_SUP: c_uint = 0x2    /* PRI feature supported */;
pub const AMD_IOMMU_DEVICE_FLAG_PASID_SUP: c_uint = 0x4    /* PASID context supported */;
// Device may request execution on memory pages
pub const AMD_IOMMU_DEVICE_FLAG_EXEC_SUP: c_uint = 0x8;
// Device may request super-user privileges
pub const AMD_IOMMU_DEVICE_FLAG_PRIV_SUP: c_uint = 0x10;
//
// This struct contains device specific data for the IOMMU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_dev_data {
// Protect against attach/detach races
    pub mutex: mutex,
    pub /: *mut *mut spinlock_t dte_lock; / DTE lock for 256-bit access,
    pub /: *mut *mut list_head list; / For domain->dev_list,
    pub /: *mut *mut llist_node dev_data_list; / For global dev_data_list,
    pub /: *mut *mut *mut protection_domain domain; / Domain the device is bound to,
    pub /: *mut *mut gcr3_tbl_info gcr3_info; / Per-device GCR3 table,
    pub dev: *mut device,
    pub /: *mut *mut u16 devid; / PCI Device ID,
    pub /: *mut *mut unsigned int max_irqs; / Maximum IRQs supported by device,
    pub /: *mut *mut u32 max_pasids; / Max supported PASIDs,
    pub /: *mut *mut *mut u32 flags; / Holds AMD_IOMMU_DEVICE_FLAG_<>,
    pub ats_qdep: c_int,
    pub /: *mut *mut u8 ats_enabled :1; / ATS state,
    pub /: *mut *mut u8 pri_enabled :1; / PRI state,
    pub /: *mut *mut u8 pasid_enabled:1; / PASID state,
    pub for: *mut *mut u8 pri_tlp :1; / PASID TLB required,
    pub /: *mut *mut u8 ppr :1; / Enable device PPR support,
    pub /: *mut *mut bool use_vapic; / Enable device to use vapic mode,
    pub defer_attach: bool,
    pub /: *mut *mut ratelimit_state rs; / Ratelimit IOPF messages,
}

// Map HPET and IOAPIC ids to the devid used by the IOMMU
//
// List with all PCI segments in the system. This list is not locked because
// it is only written at driver initialization time
//
// List with all IOMMUs in the system. This list is not locked because it is
// only written and read at driver initialization or suspend time
//
// Structure defining one entry in the device table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_table_entry {
    pub data: [u64; 4],
    pub data128: [u128; 2],
}

//
// Structure defining one entry in the command buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_cmd {
    pub data: [u32; 4],
}

//
// Structure to sture persistent DTE flags from IVHD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivhd_dte_flags {
    pub list: list_head,
    pub segid: u16,
    pub devid_first: u16,
    pub devid_last: u16,
    pub dte: dev_table_entry,
}

//
// One entry for unity mappings parsed out of the ACPI table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unity_map_entry {
    pub list: list_head,
// starting device id this entry is used for (including)
    pub devid_start: u16,
// end device id this entry is used for (including)
    pub devid_end: u16,
// start address to unity map (including)
    pub address_start: u64,
// end address to unity map (including)
    pub address_end: u64,
// required protection
    pub prot: c_int,
}

//
// Data structures for device handling
//
// Max levels of glxval supported
// IDA to track protection domain IDs
// Global EFR and EFR2 registers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_iommu_intr_mode_type {
//
// The legacy format mode is not visible to users to prevent the user
// from crashing x2APIC systems, which for all intents and purposes
// require 128-bit IRTEs.   The legacy format will be forced as needed
// when hardware doesn't support 128-bit IRTEs.
//
    AMD_IOMMU_GUEST_IR_LEGACY,
    AMD_IOMMU_GUEST_IR_LEGACY_GA,
    AMD_IOMMU_GUEST_IR_VAPIC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union irte {
    pub val: u32,
    pub 8: rsvd_2 :,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union irte_ga_lo {
    pub val: u64,
// For int remapping
// ------
    pub 32: ga_tag :,
    pub fields_remap: },
// For guest vAPIC
// ------
    pub 32: ga_tag :,
    pub fields_vapic: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union irte_ga_hi {
    pub val: u64,
    pub 8: destination :,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irte_ga {
    pub lo: irte_ga_lo,
    pub hi: irte_ga_hi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_2_irte {
    pub /: *mut *mut u16 devid; / Device ID for IRTE table,
    pub table*/: *mut *mut u16 index; / Index into IRTE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_ir_data {
    pub iommu: *mut amd_iommu,
    pub irq_2_irte: irq_2_irte,
    pub msi_entry: msi_msg,
    pub /: *mut *mut *mut void entry; / Pointer to union irte or struct irte_ga,
//
// Store information for activate/de-activate
// Guest virtual APIC mode during runtime.
//
    pub cfg: *mut irq_cfg,
    pub ga_vector: c_int,
    pub ga_root_ptr: u64,
    pub ga_tag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_irte_ops {
    pub int): *mut *mut *mut void (prepare)(void , u32, bool, u8, u32,,
    pub u16): *mut *mut *mut *mut void (activate)(struct amd_iommu iommu, void , u16,,
    pub u16): *mut *mut *mut *mut void (deactivate)(struct amd_iommu iommu, void , u16,,
    pub u32): *mut *mut *mut *mut void (set_affinity)(struct amd_iommu iommu, void , u16, u16, u8,,
    pub int): *mut *mut *mut *mut void (get)(struct irq_remap_table ,,
    pub int): *mut *mut *mut void (set_allocated)(struct irq_remap_table ,,
    pub int): *mut *mut *mut bool (is_allocated)(struct irq_remap_table ,,
    pub int): *mut *mut *mut void (clear_allocated)(struct irq_remap_table ,,
}

