//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/prom.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// Definitions for talking to the Open Firmware PROM on
// Power Macintosh computers.
//
// Copyright (C) 1996-2005 Paul Mackerras.
//
// Updates for PPC64 by Peter Bergner & David Engebretsen, IBM Corp.
//

// Minimum RMA in bytes for CAS negotiation

pub const OF_DT_BEGIN_NODE: c_uint = 0x1		/* Start of node, full name */;
pub const OF_DT_END_NODE: c_uint = 0x2		/* End node */;
pub const OF_DT_PROP: c_uint = 0x3		/* Property: name off, size,;
// content
pub const OF_DT_NOP: c_uint = 0x4		/* nop */;
pub const OF_DT_END: c_uint = 0x9;
pub const OF_DT_VERSION: c_uint = 0x10;
//
// This is what gets passed to the kernel by prom_init or kexec
//
// The dt struct contains the device tree structure, full pathes and
// property contents. The dt strings contain a separate block with just
// the strings for the property names, and is fully page aligned and
// self contained in a page, so that it can be kept around by the kernel,
// each property name appears only once in this page (cheap compression)
//
// the mem_rsvmap contains a map of reserved ranges of physical memory,
// passing it here instead of in the device-tree itself greatly simplifies
// the job of everybody. It's just a list of u64 pairs (base/size) that
// ends when size is 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_param_header {
    pub /: *mut *mut __be32 magic; / magic word OF_DT_HEADER,
    pub /: *mut *mut __be32 totalsize; / total size of DT block,
    pub /: *mut *mut __be32 off_dt_struct; / offset to structure,
    pub /: *mut *mut __be32 off_dt_strings; / offset to strings,
    pub /: *mut *mut __be32 off_mem_rsvmap; / offset to memory reserve map,
    pub /: *mut *mut __be32 version; / format version,
    pub /: *mut *mut __be32 last_comp_version; / last compatible version,
// version 2 fields below
    pub /: *mut *mut __be32 boot_cpuid_phys; / Physical CPU id we're booting on,
// version 3 fields below
    pub /: *mut *mut __be32 dt_strings_size; / size of the DT strings block,
// version 17 fields below
    pub /: *mut *mut __be32 dt_struct_size; / size of the DT structure block,
}

//
// OF address retreival & translation
//
// Parse the ibm,dma-window property of an OF node into the busno, phys and
// size parameters.
//
extern "C" {
    pub fn of_instantiate_rtc();
}
extern "C" {
    pub fn of_get_ibm_chip_id(np: *mut device_node) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_drc_info {
    pub drc_type: *mut c_char,
    pub drc_name_prefix: *mut c_char,
    pub drc_index_start: u32,
    pub drc_name_suffix_start: u32,
    pub num_sequential_elems: u32,
    pub sequential_inc: u32,
    pub drc_power_domain: u32,
    pub last_drc_index: u32,
}

//
// There are two methods for telling firmware what our capabilities are.
// Newer machines have an "ibm,client-architecture-support" method on the
// root node.  For older machines, we have to call the "process-elf-header"
// method in the /packages/elf-loader node, passing it a fake 32-bit
// ELF header containing a couple of PT_NOTE sections that contain
// structures that contain various information.
//
// New method - extensible architecture description vector.
// Option vector bits - generic bits in byte 1
pub const OV_IGNORE: c_uint = 0x80	/* ignore this vector */;
pub const OV_CESSATION_POLICY: c_uint = 0x40	/* halt if unsupported option present*/;
// Option vector 1: processor architectures supported
pub const OV1_PPC_2_00: c_uint = 0x80	/* set if we support PowerPC 2.00 */;
pub const OV1_PPC_2_01: c_uint = 0x40	/* set if we support PowerPC 2.01 */;
pub const OV1_PPC_2_02: c_uint = 0x20	/* set if we support PowerPC 2.02 */;
pub const OV1_PPC_2_03: c_uint = 0x10	/* set if we support PowerPC 2.03 */;
pub const OV1_PPC_2_04: c_uint = 0x08	/* set if we support PowerPC 2.04 */;
pub const OV1_PPC_2_05: c_uint = 0x04	/* set if we support PowerPC 2.05 */;
pub const OV1_PPC_2_06: c_uint = 0x02	/* set if we support PowerPC 2.06 */;
pub const OV1_PPC_2_07: c_uint = 0x01	/* set if we support PowerPC 2.07 */;
pub const OV1_PPC_3_00: c_uint = 0x80	/* set if we support PowerPC 3.00 */;
pub const OV1_PPC_3_1: c_uint = 0x40	/* set if we support PowerPC 3.1 */;
pub const OV1_PPC_3_2: c_uint = 0x20	/* set if we support PowerPC 3.2 */;
// Option vector 2: Open Firmware options supported
pub const OV2_REAL_MODE: c_uint = 0x20	/* set if we want OF in real mode */;
// Option vector 3: processor options supported
pub const OV3_FP: c_uint = 0x80	/* floating point */;
pub const OV3_VMX: c_uint = 0x40	/* VMX/Altivec */;
pub const OV3_DFP: c_uint = 0x20	/* decimal FP */;
// Option vector 4: IBM PAPR implementation
pub const OV4_MIN_ENT_CAP: c_uint = 0x01	/* minimum VP entitled capacity */;
// Option vector 5: PAPR/OF options supported
// These bits are also used in firmware_has_feature() to validate
// the capabilities reported for vector 5 in the device tree so we
// encode the vector index in the define and use the OV5_FEAT()
// and OV5_INDX() macros to extract the desired information.
//

pub const OV5_LPAR: c_uint = 0x0280	/* logical partitioning supported */;
pub const OV5_SPLPAR: c_uint = 0x0240	/* shared-processor LPAR supported */;
// ibm,dynamic-reconfiguration-memory property supported
pub const OV5_DRCONF_MEMORY: c_uint = 0x0220;
pub const OV5_LARGE_PAGES: c_uint = 0x0210	/* large pages supported */;
pub const OV5_DONATE_DEDICATE_CPU: c_uint = 0x0202	/* donate dedicated CPU support */;
pub const OV5_MSI: c_uint = 0x0201	/* PCIe/MSI support */;
pub const OV5_CMO: c_uint = 0x0480	/* Cooperative Memory Overcommitment */;
pub const OV5_XCMO: c_uint = 0x0440	/* Page Coalescing */;
pub const OV5_FORM1_AFFINITY: c_uint = 0x0580	/* FORM1 NUMA affinity */;
pub const OV5_PRRN: c_uint = 0x0540	/* Platform Resource Reassignment */;
pub const OV5_FORM2_AFFINITY: c_uint = 0x0520	/* Form2 NUMA affinity */;
pub const OV5_HP_EVT: c_uint = 0x0604	/* Hot Plug Event support */;
pub const OV5_RESIZE_HPT: c_uint = 0x0601	/* Hash Page Table resizing */;
pub const OV5_PFO_HW_RNG: c_uint = 0x1180	/* PFO Random Number Generator */;
pub const OV5_PFO_HW_842: c_uint = 0x1140	/* PFO Compression Accelerator */;
pub const OV5_PFO_HW_ENCR: c_uint = 0x1120	/* PFO Encryption Accelerator */;
pub const OV5_SUB_PROCESSORS: c_uint = 0x1501	/* 1,2,or 4 Sub-Processors supported */;
pub const OV5_DRMEM_V2: c_uint = 0x1680	/* ibm,dynamic-reconfiguration-v2 */;
pub const OV5_XIVE_SUPPORT: c_uint = 0x17C0	/* XIVE Exploitation Support Mask */;
pub const OV5_XIVE_LEGACY: c_uint = 0x1700	/* XIVE legacy mode Only */;
pub const OV5_XIVE_EXPLOIT: c_uint = 0x1740	/* XIVE exploitation mode Only */;
pub const OV5_XIVE_EITHER: c_uint = 0x1780	/* XIVE legacy or exploitation mode */;
// MMU Base Architecture
pub const OV5_MMU_SUPPORT: c_uint = 0x18C0	/* MMU Mode Support Mask */;
pub const OV5_MMU_HASH: c_uint = 0x1800	/* Hash MMU Only */;
pub const OV5_MMU_RADIX: c_uint = 0x1840	/* Radix MMU Only */;
pub const OV5_MMU_EITHER: c_uint = 0x1880	/* Hash or Radix Supported */;
pub const OV5_MMU_DYNAMIC: c_uint = 0x18C0	/* Hash or Radix Can Switch Later */;
pub const OV5_NMMU: c_uint = 0x1820	/* Nest MMU Available */;
// Hash Table Extensions
pub const OV5_HASH_SEG_TBL: c_uint = 0x1980	/* In Memory Segment Tables Available */;
pub const OV5_HASH_GTSE: c_uint = 0x1940	/* Guest Translation Shoot Down Avail */;
// Radix Table Extensions
pub const OV5_RADIX_GTSE: c_uint = 0x1A40	/* Guest Translation Shoot Down Avail */;
pub const OV5_DRC_INFO: c_uint = 0x1640	/* Redef Prop Structures: drc-info   */;
// Option Vector 6: IBM PAPR hints
pub const OV6_LINUX: c_uint = 0x02	/* Linux is our OS */;

