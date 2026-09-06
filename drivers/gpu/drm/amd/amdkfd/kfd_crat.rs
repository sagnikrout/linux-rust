//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_crat.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2014-2022 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define KFD_CRAT_H_INCLUDED

//
// 4CC signature value for the CRAT ACPI table
//

//
// Component Resource Association Table (CRAT)
//
pub const CRAT_OEMID_LENGTH: c_int = 6;
pub const CRAT_OEMTABLEID_LENGTH: c_int = 8;
pub const CRAT_RESERVED_LENGTH: c_int = 6;
// Compute Unit flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_header {
    pub signature: u32,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; CRAT_OEMID_LENGTH],
    pub oem_table_id: [u8; CRAT_OEMTABLEID_LENGTH],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
    pub total_entries: u32,
    pub num_domains: u16,
    pub reserved: [u8; CRAT_RESERVED_LENGTH],
}

//
// The header structure is immediately followed by total_entries of the
// data definitions
//
// The currently defined subtype entries in the CRAT
//
pub const CRAT_SUBTYPE_COMPUTEUNIT_AFFINITY: c_int = 0;
pub const CRAT_SUBTYPE_MEMORY_AFFINITY: c_int = 1;
pub const CRAT_SUBTYPE_CACHE_AFFINITY: c_int = 2;
pub const CRAT_SUBTYPE_TLB_AFFINITY: c_int = 3;
pub const CRAT_SUBTYPE_CCOMPUTE_AFFINITY: c_int = 4;
pub const CRAT_SUBTYPE_IOLINK_AFFINITY: c_int = 5;
pub const CRAT_SUBTYPE_MAX: c_int = 6;
//
// Do not change the value of CRAT_SIBLINGMAP_SIZE from 32
// as it breaks the ABI.
//
pub const CRAT_SIBLINGMAP_SIZE: c_int = 32;
//
// ComputeUnit Affinity structure and definitions
//
pub const CRAT_CU_FLAGS_ENABLED: c_uint = 0x00000001;
pub const CRAT_CU_FLAGS_HOT_PLUGGABLE: c_uint = 0x00000002;
pub const CRAT_CU_FLAGS_CPU_PRESENT: c_uint = 0x00000004;
pub const CRAT_CU_FLAGS_GPU_PRESENT: c_uint = 0x00000008;
pub const CRAT_CU_FLAGS_IOMMU_PRESENT: c_uint = 0x00000010;
pub const CRAT_CU_FLAGS_RESERVED: c_uint = 0xffffffe0;
pub const CRAT_COMPUTEUNIT_RESERVED_LENGTH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_subtype_computeunit {
    pub type: u8,
    pub length: u8,
    pub reserved: u16,
    pub flags: u32,
    pub proximity_domain: u32,
    pub processor_id_low: u32,
    pub num_cpu_cores: u16,
    pub num_simd_cores: u16,
    pub max_waves_simd: u16,
    pub io_count: u16,
    pub hsa_capability: u16,
    pub lds_size_in_kb: u16,
    pub wave_front_size: u8,
    pub num_banks: u8,
    pub micro_engine_id: u16,
    pub array_count: u8,
    pub num_cu_per_array: u8,
    pub num_simd_per_cu: u8,
    pub max_slots_scatch_cu: u8,
    pub reserved2: [u8; CRAT_COMPUTEUNIT_RESERVED_LENGTH],
}

//
// HSA Memory Affinity structure and definitions
//
pub const CRAT_MEM_FLAGS_ENABLED: c_uint = 0x00000001;
pub const CRAT_MEM_FLAGS_HOT_PLUGGABLE: c_uint = 0x00000002;
pub const CRAT_MEM_FLAGS_NON_VOLATILE: c_uint = 0x00000004;
pub const CRAT_MEM_FLAGS_RESERVED: c_uint = 0xfffffff8;
pub const CRAT_MEMORY_RESERVED_LENGTH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_subtype_memory {
    pub type: u8,
    pub length: u8,
    pub reserved: u16,
    pub flags: u32,
    pub proximity_domain: u32,
    pub base_addr_low: u32,
    pub base_addr_high: u32,
    pub length_low: u32,
    pub length_high: u32,
    pub width: u32,
    pub /: *mut *mut uint8_t visibility_type; / for virtual (dGPU) CRAT,
    pub 1]: uint8_t reserved2[CRAT_MEMORY_RESERVED_LENGTH -,
}

//
// HSA Cache Affinity structure and definitions
//
pub const CRAT_CACHE_FLAGS_ENABLED: c_uint = 0x00000001;
pub const CRAT_CACHE_FLAGS_DATA_CACHE: c_uint = 0x00000002;
pub const CRAT_CACHE_FLAGS_INST_CACHE: c_uint = 0x00000004;
pub const CRAT_CACHE_FLAGS_CPU_CACHE: c_uint = 0x00000008;
pub const CRAT_CACHE_FLAGS_SIMD_CACHE: c_uint = 0x00000010;
pub const CRAT_CACHE_FLAGS_RESERVED: c_uint = 0xffffffe0;
pub const CRAT_CACHE_RESERVED_LENGTH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_subtype_cache {
    pub type: u8,
    pub length: u8,
    pub reserved: u16,
    pub flags: u32,
    pub processor_id_low: u32,
    pub sibling_map: [u8; CRAT_SIBLINGMAP_SIZE],
    pub cache_size: u32,
    pub cache_level: u8,
    pub lines_per_tag: u8,
    pub cache_line_size: u16,
    pub associativity: u8,
    pub cache_properties: u8,
    pub cache_latency: u16,
    pub reserved2: [u8; CRAT_CACHE_RESERVED_LENGTH],
}

//
// HSA TLB Affinity structure and definitions
//
pub const CRAT_TLB_FLAGS_ENABLED: c_uint = 0x00000001;
pub const CRAT_TLB_FLAGS_DATA_TLB: c_uint = 0x00000002;
pub const CRAT_TLB_FLAGS_INST_TLB: c_uint = 0x00000004;
pub const CRAT_TLB_FLAGS_CPU_TLB: c_uint = 0x00000008;
pub const CRAT_TLB_FLAGS_SIMD_TLB: c_uint = 0x00000010;
pub const CRAT_TLB_FLAGS_RESERVED: c_uint = 0xffffffe0;
pub const CRAT_TLB_RESERVED_LENGTH: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_subtype_tlb {
    pub type: u8,
    pub length: u8,
    pub reserved: u16,
    pub flags: u32,
    pub processor_id_low: u32,
    pub sibling_map: [u8; CRAT_SIBLINGMAP_SIZE],
    pub tlb_level: u32,
    pub data_tlb_associativity_2mb: u8,
    pub data_tlb_size_2mb: u8,
    pub instruction_tlb_associativity_2mb: u8,
    pub instruction_tlb_size_2mb: u8,
    pub data_tlb_associativity_4k: u8,
    pub data_tlb_size_4k: u8,
    pub instruction_tlb_associativity_4k: u8,
    pub instruction_tlb_size_4k: u8,
    pub data_tlb_associativity_1gb: u8,
    pub data_tlb_size_1gb: u8,
    pub instruction_tlb_associativity_1gb: u8,
    pub instruction_tlb_size_1gb: u8,
    pub reserved2: [u8; CRAT_TLB_RESERVED_LENGTH],
}

//
// HSA CCompute/APU Affinity structure and definitions
//
pub const CRAT_CCOMPUTE_FLAGS_ENABLED: c_uint = 0x00000001;
pub const CRAT_CCOMPUTE_FLAGS_RESERVED: c_uint = 0xfffffffe;
pub const CRAT_CCOMPUTE_RESERVED_LENGTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_subtype_ccompute {
    pub type: u8,
    pub length: u8,
    pub reserved: u16,
    pub flags: u32,
    pub processor_id_low: u32,
    pub sibling_map: [u8; CRAT_SIBLINGMAP_SIZE],
    pub apu_size: u32,
    pub reserved2: [u8; CRAT_CCOMPUTE_RESERVED_LENGTH],
}

//
// HSA IO Link Affinity structure and definitions
//

pub const CRAT_IOLINK_FLAGS_RESERVED_MASK: c_uint = 0x7fffffe0;
//
// IO interface types
//
pub const CRAT_IOLINK_TYPE_UNDEFINED: c_int = 0;
pub const CRAT_IOLINK_TYPE_HYPERTRANSPORT: c_int = 1;
pub const CRAT_IOLINK_TYPE_PCIEXPRESS: c_int = 2;
pub const CRAT_IOLINK_TYPE_AMBA: c_int = 3;
pub const CRAT_IOLINK_TYPE_MIPI: c_int = 4;
pub const CRAT_IOLINK_TYPE_QPI_1_1: c_int = 5;
pub const CRAT_IOLINK_TYPE_RESERVED1: c_int = 6;
pub const CRAT_IOLINK_TYPE_RESERVED2: c_int = 7;
pub const CRAT_IOLINK_TYPE_RAPID_IO: c_int = 8;
pub const CRAT_IOLINK_TYPE_INFINIBAND: c_int = 9;
pub const CRAT_IOLINK_TYPE_RESERVED3: c_int = 10;
pub const CRAT_IOLINK_TYPE_XGMI: c_int = 11;
pub const CRAT_IOLINK_TYPE_XGOP: c_int = 12;
pub const CRAT_IOLINK_TYPE_GZ: c_int = 13;
pub const CRAT_IOLINK_TYPE_ETHERNET_RDMA: c_int = 14;
pub const CRAT_IOLINK_TYPE_RDMA_OTHER: c_int = 15;
pub const CRAT_IOLINK_TYPE_OTHER: c_int = 16;
pub const CRAT_IOLINK_TYPE_MAX: c_int = 255;
pub const CRAT_IOLINK_RESERVED_LENGTH: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_subtype_iolink {
    pub type: u8,
    pub length: u8,
    pub reserved: u16,
    pub flags: u32,
    pub proximity_domain_from: u32,
    pub proximity_domain_to: u32,
    pub io_interface_type: u8,
    pub version_major: u8,
    pub version_minor: u16,
    pub minimum_latency: u32,
    pub maximum_latency: u32,
    pub minimum_bandwidth_mbs: u32,
    pub maximum_bandwidth_mbs: u32,
    pub recommended_transfer_size: u32,
    pub 1]: uint8_t reserved2[CRAT_IOLINK_RESERVED_LENGTH -,
    pub weight_xgmi: u8,
}

//
// HSA generic sub-type header
//
pub const CRAT_SUBTYPE_FLAGS_ENABLED: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crat_subtype_generic {
    pub type: u8,
    pub length: u8,
    pub reserved: u16,
    pub flags: u32,
}

// Static table to describe GPU Cache information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_gpu_cache_info {
    pub cache_size: u32,
    pub cache_level: u32,
    pub cache_line_size: u32,
    pub flags: u32,
// Indicates how many Compute Units share this cache
// within a SA. Value = 1 indicates the cache is not shared
//
    pub num_cu_shared: u32,
}

extern "C" {
    pub fn kfd_get_gpu_cache_info(kdev: *mut kfd_node, pcache_info: *mut kfd_gpu_cache_info) -> c_int;
}
extern "C" {
    pub fn kfd_destroy_crat_image(crat_image: *mut c_void);
}
