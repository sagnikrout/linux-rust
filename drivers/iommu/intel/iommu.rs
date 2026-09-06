//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/intel/iommu.h
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
// Copyright © 2006-2015, Intel Corporation.
//
// Authors: Ashok Raj <ashok.raj@intel.com>
// Anil S Keshavamurthy <anil.s.keshavamurthy@intel.com>
// David Woodhouse <David.Woodhouse@intel.com>
//

//
// VT-d hardware uses 4KiB page size regardless of host page size.
//

pub const DMA_SL_PTE_DIRTY_BIT: c_int = 9;

pub const CONTEXT_TT_MULTI_LEVEL: c_int = 0;
pub const CONTEXT_TT_DEV_IOTLB: c_int = 1;
pub const CONTEXT_TT_PASS_THROUGH: c_int = 2;

//
// Intel IOMMU register specification per version 1.0 public spec.
//
pub const DMAR_VER_REG: c_uint = 0x0	/* Arch version supported by this IOMMU */;
pub const DMAR_CAP_REG: c_uint = 0x8	/* Hardware supported capabilities */;
pub const DMAR_ECAP_REG: c_uint = 0x10	/* Extended capabilities supported */;
pub const DMAR_GCMD_REG: c_uint = 0x18	/* Global command register */;
pub const DMAR_GSTS_REG: c_uint = 0x1c	/* Global status register */;
pub const DMAR_RTADDR_REG: c_uint = 0x20	/* Root entry table */;
pub const DMAR_CCMD_REG: c_uint = 0x28	/* Context command reg */;
pub const DMAR_FSTS_REG: c_uint = 0x34	/* Fault Status register */;
pub const DMAR_FECTL_REG: c_uint = 0x38	/* Fault control register */;
pub const DMAR_FEDATA_REG: c_uint = 0x3c	/* Fault event interrupt data register */;
pub const DMAR_FEADDR_REG: c_uint = 0x40	/* Fault event interrupt addr register */;
pub const DMAR_FEUADDR_REG: c_uint = 0x44	/* Upper address register */;
pub const DMAR_PMEN_REG: c_uint = 0x64	/* Enable Protected Memory Region */;
pub const DMAR_PLMBASE_REG: c_uint = 0x68	/* PMRR Low addr */;
pub const DMAR_PLMLIMIT_REG: c_uint = 0x6c	/* PMRR low limit */;
pub const DMAR_PHMBASE_REG: c_uint = 0x70	/* pmrr high base addr */;
pub const DMAR_PHMLIMIT_REG: c_uint = 0x78	/* pmrr high limit */;
pub const DMAR_IQH_REG: c_uint = 0x80	/* Invalidation queue head register */;
pub const DMAR_IQT_REG: c_uint = 0x88	/* Invalidation queue tail register */;

pub const DMAR_IQA_REG: c_uint = 0x90	/* Invalidation queue addr register */;
pub const DMAR_ICS_REG: c_uint = 0x9c	/* Invalidation complete status register */;
pub const DMAR_IQER_REG: c_uint = 0xb0	/* Invalidation queue error record register */;
pub const DMAR_IRTA_REG: c_uint = 0xb8    /* Interrupt remapping table addr register */;
pub const DMAR_PQH_REG: c_uint = 0xc0	/* Page request queue head register */;
pub const DMAR_PQT_REG: c_uint = 0xc8	/* Page request queue tail register */;
pub const DMAR_PQA_REG: c_uint = 0xd0	/* Page request queue address register */;
pub const DMAR_PRS_REG: c_uint = 0xdc	/* Page request status register */;
pub const DMAR_PECTL_REG: c_uint = 0xe0	/* Page request event control register */;
pub const DMAR_PEDATA_REG: c_uint = 0xe4	/* Page request event interrupt data register */;
pub const DMAR_PEADDR_REG: c_uint = 0xe8	/* Page request event interrupt addr register */;
pub const DMAR_PEUADDR_REG: c_uint = 0xec	/* Page request event Upper address register */;
pub const DMAR_MTRRCAP_REG: c_uint = 0x100	/* MTRR capability register */;
pub const DMAR_MTRRDEF_REG: c_uint = 0x108	/* MTRR default type register */;
pub const DMAR_MTRR_FIX64K_00000_REG: c_uint = 0x120 /* MTRR Fixed range registers */;
pub const DMAR_MTRR_FIX16K_80000_REG: c_uint = 0x128;
pub const DMAR_MTRR_FIX16K_A0000_REG: c_uint = 0x130;
pub const DMAR_MTRR_FIX4K_C0000_REG: c_uint = 0x138;
pub const DMAR_MTRR_FIX4K_C8000_REG: c_uint = 0x140;
pub const DMAR_MTRR_FIX4K_D0000_REG: c_uint = 0x148;
pub const DMAR_MTRR_FIX4K_D8000_REG: c_uint = 0x150;
pub const DMAR_MTRR_FIX4K_E0000_REG: c_uint = 0x158;
pub const DMAR_MTRR_FIX4K_E8000_REG: c_uint = 0x160;
pub const DMAR_MTRR_FIX4K_F0000_REG: c_uint = 0x168;
pub const DMAR_MTRR_FIX4K_F8000_REG: c_uint = 0x170;
pub const DMAR_MTRR_PHYSBASE0_REG: c_uint = 0x180 /* MTRR Variable range registers */;
pub const DMAR_MTRR_PHYSMASK0_REG: c_uint = 0x188;
pub const DMAR_MTRR_PHYSBASE1_REG: c_uint = 0x190;
pub const DMAR_MTRR_PHYSMASK1_REG: c_uint = 0x198;
pub const DMAR_MTRR_PHYSBASE2_REG: c_uint = 0x1a0;
pub const DMAR_MTRR_PHYSMASK2_REG: c_uint = 0x1a8;
pub const DMAR_MTRR_PHYSBASE3_REG: c_uint = 0x1b0;
pub const DMAR_MTRR_PHYSMASK3_REG: c_uint = 0x1b8;
pub const DMAR_MTRR_PHYSBASE4_REG: c_uint = 0x1c0;
pub const DMAR_MTRR_PHYSMASK4_REG: c_uint = 0x1c8;
pub const DMAR_MTRR_PHYSBASE5_REG: c_uint = 0x1d0;
pub const DMAR_MTRR_PHYSMASK5_REG: c_uint = 0x1d8;
pub const DMAR_MTRR_PHYSBASE6_REG: c_uint = 0x1e0;
pub const DMAR_MTRR_PHYSMASK6_REG: c_uint = 0x1e8;
pub const DMAR_MTRR_PHYSBASE7_REG: c_uint = 0x1f0;
pub const DMAR_MTRR_PHYSMASK7_REG: c_uint = 0x1f8;
pub const DMAR_MTRR_PHYSBASE8_REG: c_uint = 0x200;
pub const DMAR_MTRR_PHYSMASK8_REG: c_uint = 0x208;
pub const DMAR_MTRR_PHYSBASE9_REG: c_uint = 0x210;
pub const DMAR_MTRR_PHYSMASK9_REG: c_uint = 0x218;
pub const DMAR_PERFCAP_REG: c_uint = 0x300;
pub const DMAR_PERFCFGOFF_REG: c_uint = 0x310;
pub const DMAR_PERFOVFOFF_REG: c_uint = 0x318;
pub const DMAR_PERFCNTROFF_REG: c_uint = 0x31c;
pub const DMAR_PERFINTRSTS_REG: c_uint = 0x324;
pub const DMAR_PERFINTRCTL_REG: c_uint = 0x328;
pub const DMAR_PERFEVNTCAP_REG: c_uint = 0x380;
pub const DMAR_ECMD_REG: c_uint = 0x400;
pub const DMAR_ECEO_REG: c_uint = 0x408;
pub const DMAR_ECRSP_REG: c_uint = 0x410;
pub const DMAR_ECCAP_REG: c_uint = 0x430;

//
// Decoding Capability Register
//

//
// Extended Capability Register
//

//
// Decoding Perf Capability Register
//

// The counter stride is calculated as 2 ^ (x+10) bytes

//
// Decoding Perf Event Capability Register
//

// Virtual command interface capability

// IOTLB_REG
pub const DMA_TLB_FLUSH_GRANU_OFFSET: c_int = 60;

// INVALID_DESC
pub const DMA_CCMD_INVL_GRANU_OFFSET: c_int = 61;

// PMEN_REG

// GCMD_REG

// GSTS_REG

// DMA_RTADDR_REG

// CCMD_REG

pub const DMA_CCMD_MASK_NOBIT: c_int = 0;
pub const DMA_CCMD_MASK_1BIT: c_int = 1;
pub const DMA_CCMD_MASK_2BIT: c_int = 2;
pub const DMA_CCMD_MASK_3BIT: c_int = 3;

// ECMD_REG
pub const DMA_MAX_NUM_ECMD: c_int = 256;

pub const DMA_ECMD_REG_STEP: c_int = 8;
pub const DMA_ECMD_ENABLE: c_uint = 0xf0;
pub const DMA_ECMD_DISABLE: c_uint = 0xf1;
pub const DMA_ECMD_FREEZE: c_uint = 0xf4;
pub const DMA_ECMD_UNFREEZE: c_uint = 0xf5;
pub const DMA_ECMD_OA_SHIFT: c_int = 16;
pub const DMA_ECMD_ECRSP_IP: c_uint = 0x1;
pub const DMA_ECMD_ECCAP3: c_int = 3;

// FECTL_REG

// FSTS_REG

// FRCD_REG, 32 bits access

// low 64 bit

// PRS_REG

// PERFINTRSTS_REG

pub const QI_CC_TYPE: c_uint = 0x1;
pub const QI_IOTLB_TYPE: c_uint = 0x2;
pub const QI_DIOTLB_TYPE: c_uint = 0x3;
pub const QI_IEC_TYPE: c_uint = 0x4;
pub const QI_IWD_TYPE: c_uint = 0x5;
pub const QI_EIOTLB_TYPE: c_uint = 0x6;
pub const QI_PC_TYPE: c_uint = 0x7;
pub const QI_DEIOTLB_TYPE: c_uint = 0x8;
pub const QI_PGRP_RESP_TYPE: c_uint = 0x9;
pub const QI_PSTRM_RESP_TYPE: c_uint = 0xa;

pub const QI_DEV_IOTLB_SIZE: c_int = 1;
pub const QI_DEV_IOTLB_MAX_INVS: c_int = 32;

// PASID cache invalidation granu
pub const QI_PC_ALL_PASIDS: c_int = 0;
pub const QI_PC_PASID_SEL: c_int = 1;
pub const QI_PC_GLOBAL: c_int = 3;

// QI Dev-IOTLB inv granu
pub const QI_DEV_IOTLB_GRAN_ALL: c_int = 1;
pub const QI_DEV_IOTLB_GRAN_PASID_SEL: c_int = 0;

pub const QI_DEV_EIOTLB_MAX_INVS: c_int = 32;
// Page group response descriptor QW0

// Page group response descriptor QW1

pub const QI_RESP_SUCCESS: c_uint = 0x0;
pub const QI_RESP_INVALID: c_uint = 0x1;
pub const QI_RESP_FAILURE: c_uint = 0xf;
pub const QI_GRAN_NONG_PASID: c_int = 2;
pub const QI_GRAN_PSI_PASID: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qi_desc {
    pub qw0: u64,
    pub qw1: u64,
    pub qw2: u64,
    pub qw3: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q_inval {
    pub q_lock: raw_spinlock_t,
    pub /: *mut *mut *mut void desc; / invalidation queue,
    pub /: *mut *mut *mut int desc_status; / desc status,
    pub /: *mut *mut int free_head; / first free entry,
    pub /: *mut *mut int free_tail; / last free entry,
    pub free_cnt: c_int,
}

// Page Request Queue depth
pub const PRQ_ORDER: c_int = 4;

pub const INTR_REMAP_TABLE_REG_SIZE: c_uint = 0xf;
pub const INTR_REMAP_TABLE_REG_SIZE_MASK: c_uint = 0xf;
pub const INTR_REMAP_TABLE_ENTRIES: c_int = 65536;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_table {
    pub base: *mut irte,
    pub bitmap: *mut c_ulong,
}

extern "C" {
    pub fn intel_irq_remap_add_device(info: *mut dmar_pci_notify_info);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_flush {
    pub type): u8 fm, u64,
    pub type): unsigned int size_order, u64,
}

//
// 0: Present
// 1-11: Reserved
// 12-63: Context Ptr (12 - (haw-1))
// 64-127: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_entry {
    pub lo: u64,
    pub hi: u64,
}

//
// low 64 bits:
// 0: present
// 1: fault processing disable
// 2-3: translation type
// 12-63: address space root
// high 64 bits:
// 0-2: address width
// 3-6: aval
// 8-23: domain id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct context_entry {
    pub lo: u64,
    pub hi: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_domain_info {
    pub iommu: *mut intel_iommu,
    pub /: *mut *mut unsigned int refcnt; / Refcount of devices per iommu,
    pub since: *mut *mut u16 did; / Domain ids per IOMMU. Use u16,
// domain ids are 16 bit wide according
// to VT-d spec, section 9.3
}

//
// We start simply by using a fixed size for the batched descriptors. This
// size is currently sufficient for our needs. Future improvements could
// involve dynamically allocating the batch buffer based on actual demand,
// allowing us to adjust the batch size for optimal performance in different
// scenarios.
//
pub const QI_MAX_BATCHED_DESC_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qi_batch {
    pub descs: [qi_desc; QI_MAX_BATCHED_DESC_COUNT],
    pub index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmar_domain {
    pub domain: iommu_domain,
    pub iommu: pt_iommu,
// First stage page table
    pub fspt: pt_iommu_x86_64,
// Second stage page table
    pub sspt: pt_iommu_vtdss,
}

// buffer when creating mappings.
//
// DMA remapping domain
// Protect the s1_domains list
// Track s1_domains nested on this domain
// Nested user domain
// parent page table which the user domain is nested on
// page table attributes
// link to parent domain siblings
// SVA domain
//
// In theory, the VT-d 4.0 spec can support up to 2 ^ 16 counters.
// But in practice, there are only 14 counters for the existing
// platform. Setting the max number of counters to 64 should be good
// enough for a long time. Also, supporting more than 64 counters
// requires more extras, e.g., extra freeze and overflow registers,
// which is not necessary for now.
//
pub const IOMMU_PMU_IDX_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_pmu {
    pub iommu: *mut intel_iommu,
    pub /: *mut *mut u32 num_cntr; / Number of counters,
    pub /: *mut *mut u32 num_eg; / Number of event group,
    pub /: *mut *mut u32 cntr_width; / Counter width,
    pub /: *mut *mut u32 cntr_stride; / Counter Stride,
    pub /: *mut *mut u32 filter; / Bitmask of filter support,
    pub /: *mut *mut *mut void __iomem base; / the PerfMon base address,
    pub /: *mut *mut *mut void __iomem cfg_reg; / counter configuration base address,
    pub address*/: *mut *mut *mut void __iomem cntr_reg; / counter 0,
    pub /: *mut *mut *mut void __iomem overflow; / overflow status register,
    pub /: *mut *mut *mut u64 evcap; / Indicates all supported events,
    pub /: *mut *mut *mut *mut u32 cntr_evcap; / Supported events of each counter.,
    pub pmu: pmu,
    pub IOMMU_PMU_IDX_MAX): DECLARE_BITMAP(used_mask,,
    pub event_list: [*mut perf_event; IOMMU_PMU_IDX_MAX],
    pub irq_name: [c_uchar; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_iommu {
    pub /: *mut *mut *mut void __iomem reg; / Pointer to hardware regs, virtual addr,
    pub /: *mut *mut u64 reg_phys; / physical address of hw register set,
    pub /: *mut *mut u64 reg_size; / size of hw register set,
    pub cap: u64,
    pub ecap: u64,
    pub vccap: u64,
    pub ecmdcap: [u64; DMA_MAX_NUM_ECMDCAP],
    pub /: *mut *mut u32 gcmd; / Holds TE, EAFL. Don't need SRTP, SFL, WBF,
    pub /: *mut *mut raw_spinlock_t register_lock; / protect register handling,
    pub /: *mut *mut int seq_id; / sequence id of the iommu,
    pub /: *mut *mut int agaw; / agaw of this iommu,
    pub /: *mut *mut int msagaw; / max sagaw of this iommu,
    pub perf_irq: unsigned int irq, pr_irq,,
    pub /: *mut *mut u16 segment; / PCI segment#,
    pub /: *mut *mut unsigned char name[16]; / Device Name,

// mutex to protect domain_ida
    pub did_lock: mutex,
    pub /: *mut *mut ida domain_ida; / domain id allocator,
    pub max_domain_id: c_ulong,
    pub /: *mut *mut *mut unsigned long copied_tables; / bitmap of copied tables,
    pub /: *mut *mut spinlock_t lock; / protect context, domain ids,
    pub /: *mut *mut *mut root_entry root_entry; / virtual address,
    pub flush: iommu_flush,

    pub prq: *mut page_req_dsc,
    pub /: *mut *mut unsigned char prq_name[16]; / Name for PRQ interrupt,
    pub prq_seq_number: c_ulong,
    pub prq_complete: completion,
    pub iopf_queue: *mut iopf_queue,
    pub iopfq_name: [c_uchar; 16],
// Synchronization between fault report and iommu device release.
    pub iopf_lock: mutex,
    pub /: *mut *mut *mut q_inval qi; / Queued invalidation info,
    pub resume.*/: *mut *mut u32 iommu_state[MAX_SR_DMAR_REGS]; / Store iommu states between suspend and,
// rb tree for all probed devices
    pub device_rbtree: rb_root,
// protect the device_rbtree
    pub device_rbtree_lock: spinlock_t,

    pub /: *mut *mut *mut ir_table ir_table; / Interrupt remapping info,
    pub ir_domain: *mut irq_domain,

    pub /: *mut *mut iommu_device iommu; / IOMMU core code handle,
    pub node: c_int,
    pub /: *mut *mut u32 flags; / Software defined flags,
    pub drhd: *mut dmar_drhd_unit,
    pub perf_statistic: *mut c_void,
    pub pmu: *mut iommu_pmu,
}

// PCI domain-device relationship
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_domain_info {
    pub /: *mut *mut list_head link; / link to domain siblings,
    pub /: *mut *mut u32 segment; / PCI segment number,
    pub /: *mut *mut u8 bus; / PCI bus number,
    pub /: *mut *mut u8 devfn; / PCI devfn number,
    pub /: *mut *mut u16 pfsid; / SRIOV physical function source ID,
    pub pasid_supported:3: u8,
    pub pasid_enabled:1: u8,
    pub pri_supported:1: u8,
    pub pri_enabled:1: u8,
    pub ats_supported:1: u8,
    pub ats_enabled:1: u8,
    pub /: *mut *mut u8 dtlb_extra_inval:1; / Quirk for devices need extra flush,
    pub /: *mut *mut u8 domain_attached:1; / Device has domain attached,
    pub ats_qdep: u8,
    pub iopf_refcount: c_uint,
    pub /: *mut *mut *mut device dev; / it's NULL for PCIe-to-PCI bridge,
    pub /: *mut *mut *mut intel_iommu iommu; / IOMMU used by this device,
    pub /: *mut *mut *mut dmar_domain domain; / pointer to domain,
    pub /: *mut *mut *mut pasid_table pasid_table; / pasid table,
// device tracking node(lookup by PCI RID)
    pub node: rb_node,

    pub /: *mut *mut *mut dentry debugfs_dentry; / pointer to device directory dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pasid_info {
    pub /: *mut *mut list_head link_domain; / link to domain siblings,
    pub dev: *mut device,
    pub pasid: ioasid_t,

    pub /: *mut *mut *mut dentry debugfs_dentry; / pointer to pasid directory dentry,

}

// Convert generic struct iommu_domain to private struct dmar_domain
extern "C" {
    pub fn container_of(_arg: dom, dmar_domain: struct, _arg: domain) -> return;
}
//
// Domain ID 0 and 1 are reserved:
//
// If Caching mode is set, then invalid translations are tagged
// with domain-id 0, hence we need to pre-allocate it. We also
// use domain-id 0 as a marker for non-allocated domain-id, so
// make sure it is not used for a real domain.
//
// Vt-d spec rev3.0 (section 6.2.3.1) requires that each pasid
// entry for first-level or pass-through translation modes should
// be programmed with a domain id different from those used for
// second-level or nested translation. We reserve a domain id for
// this purpose. This domain id is also used for identity domain
// in legacy mode.
//
pub const FLPT_DEFAULT_DID: c_int = 1;
pub const IDA_START_DID: c_int = 2;
// Retrieve the domain ID which has allocated to the domain
extern "C" {
    pub fn domain_id_iommu(_arg: to_dmar_domain(domain), _arg: iommu) -> return;
}
//
// 0: readable
// 1: writable
// 2-6: reserved
// 7: super page
// 8-10: available
// 11: snoop behavior
// 12-63: Host physical address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_pte {
    pub val: u64,
}

// Must have a full atomic 64-bit read

extern "C" {
    pub fn DIV_ROUND_UP(30: width -, _arg: LEVEL_STRIDE) -> return;
}
//
// Clear the Present (P) bit (bit 0) of a context table entry. This initiates
// the transition of the entry's ownership from hardware to software. The
// caller is responsible for fulfilling the invalidation handshake recommended
// by the VT-d spec, Section 6.5.3.3 (Guidance to Software for Invalidations).
//

extern "C" {
    pub fn test_bit(devfn: ((long)bus << 8) |, _arg: iommu->copied_tables) -> return;
}

//
// Set the RID_PASID field of a scalable mode context entry. The
// IOMMU hardware will use the PASID value set in this field for
// DMA translations of DMA requests without PASID.
//
// Set the DTE(Device-TLB Enable) field of a scalable mode context
// entry.
//
// Set the PRE(Page Request Enable) field of a scalable mode context
// entry.
//
// Clear the PRE(Page Request Enable) field of a scalable mode context
// entry.
//
// Returns a number of VTD pages, but aligned to MM page size
// Return a size from number of VTD pages.
// PASID-selective IOTLB invalidation
// Page-selective-within-PASID IOTLB invalidation
//
// calculate_psi_aligned_address() must be used for addr and size_order
//
// If S bit is 0, we only flush a single page. If S bit is set,
// The least significant zero bit indicates the invalidation address
// range. VT-d spec 6.5.2.6.
// e.g. address bit 12[0] indicates 8KB, 13[0] indicates 16KB.
// size order = 0 is PAGE_SIZE 4KB
// Max Invs Pending (MIP) is set to 0 for now until we have DIT in
// ECAP.
//
// Take page address
//
// Existing 0s in address below size_order may be the least
// significant bit, we must set them to 1s to avoid having
// smaller size than desired.
//
// Clear size_order bit to indicate size
// Set the S bit to indicate flushing more than 1 page
// Convert value to context PASID directory size field coding.

extern "C" {
    pub fn dmar_enable_qi(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn dmar_disable_qi(iommu: *mut intel_iommu);
}
extern "C" {
    pub fn dmar_reenable_qi(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn qi_global_iec(iommu: *mut intel_iommu);
}
extern "C" {
    pub fn qi_flush_piotlb_all(iommu: *mut intel_iommu, did: u16, pasid: u32);
}
//
// Options used in qi_submit_sync:
// QI_OPT_WAIT_DRAIN - Wait for PRQ drain completion, spec 6.5.2.8.
//

extern "C" {
    pub fn domain_attach_iommu(domain: *mut dmar_domain, iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn domain_detach_iommu(domain: *mut dmar_domain, iommu: *mut intel_iommu);
}
extern "C" {
    pub fn device_block_translation(dev: *mut device);
}
extern "C" {
    pub fn paging_domain_compatible(domain: *mut iommu_domain, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dmar_ir_support() -> c_int;
}
extern "C" {
    pub fn iommu_flush_write_buffer(iommu: *mut intel_iommu);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cache_tag_type {
    CACHE_TAG_IOTLB,
    CACHE_TAG_DEVTLB,
    CACHE_TAG_NESTING_IOTLB,
    CACHE_TAG_NESTING_DEVTLB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_tag {
    pub node: list_head,
    pub type: cache_tag_type,
    pub iommu: *mut intel_iommu,
//
// The @dev field represents the location of the cache. For IOTLB, it
// resides on the IOMMU hardware. @dev stores the device pointer to
// the IOMMU hardware. For DevTLB, it locates in the PCIe endpoint.
// @dev stores the device pointer to that endpoint.
//
    pub dev: *mut device,
    pub domain_id: u16,
    pub pasid: ioasid_t,
    pub users: c_uint,
}

extern "C" {
    pub fn cache_tag_flush_all(domain: *mut dmar_domain);
}
extern "C" {
    pub fn intel_iommu_enable_prq(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn intel_iommu_finish_prq(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn intel_iommu_drain_pasid_prq(dev: *mut device, pasid: u32);
}
extern "C" {
    pub fn intel_iommu_enable_iopf(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn intel_iommu_disable_iopf(dev: *mut device);
}
// SVA with non-IOMMU/PRI IOPF handling is allowed.
extern "C" {
    pub fn intel_iommu_enable_iopf(_arg: dev) -> return;
}

extern "C" {
    pub fn intel_svm_check(iommu: *mut intel_iommu);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn intel_iommu_debugfs_init();
}
extern "C" {
    pub fn intel_iommu_debugfs_create_dev(info: *mut device_domain_info);
}
extern "C" {
    pub fn intel_iommu_debugfs_remove_dev(info: *mut device_domain_info);
}
extern "C" {
    pub fn intel_iommu_debugfs_create_dev_pasid(dev_pasid: *mut dev_pasid_info);
}
extern "C" {
    pub fn intel_iommu_debugfs_remove_dev_pasid(dev_pasid: *mut dev_pasid_info);
}

extern "C" {
    pub fn iommu_calculate_agaw(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn iommu_calculate_max_sagaw(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn ecmd_submit_sync(iommu: *mut intel_iommu, ecmd: u8, oa: u64, ob: u64) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmar_force_on {
    DMAR_FORCEON_PLATFORM,
    DMAR_FORCEON_TBOOT
}

//
// On policies are positive, with more positive value being stronger.
// Off policies are negative, with more negative value being stronger.
//
// 'dmar' here refers to DMA remapping instead of the dmar/iommu unit.
//
// - DMAR_FORCE_ON:
// force to turn on (e.g. by tboot or platform opt-in).
//
// - DMAR_ON:
// turn on by build configuration (CONFIG_INTEL_IOMMU_DEFAULT_ON=on)
// or user opts ("intel_iommu=on").
//
// - DMAR_DEFAULT_OFF
// turn off by build configuration (CONFIG_INTEL_IOMMU_DEFAULT_ON=off).
//
// - DMAR_USER_OFF
// turn off by user opts ("intel_iommu=off" or "iommu=off").
//
// - DMAR_FW_OFF
// turn off due to firmware opt-out (DMAR_REMAP_OPT_OUT)
//
// - '0' is invalid, compared to decide the on/off policy
//
pub const DMAR_FORCE_ON: c_int = 2;
pub const DMAR_ON: c_int = 1;

extern "C" {
    pub fn dmar_can_force_on(force_on: dmar_force_on) -> bool;
}
// Private Data
