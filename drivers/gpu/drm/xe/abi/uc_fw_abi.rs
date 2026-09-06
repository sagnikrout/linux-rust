//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/abi/uc_fw_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

//
// DOC: CSS-based Firmware Layout
//
// The CSS-based firmware structure is used for GuC releases on all platforms
// and for HuC releases up to DG1. Starting from DG2/MTL the HuC uses the GSC
// layout instead.
// The CSS firmware layout looks like this::
//
// +======================================================================+
// |  Firmware blob                                                       |
// +===============+===============+============+============+============+
// |  CSS header   |     uCode     |  RSA key   |  modulus   |  exponent  |
// +===============+===============+============+============+============+
// <-header size->                 <---header size continued ----------->
// <--- size ----------------------------------------------------------->
// <-key size->
// <-mod size->
// <-exp size->
//
// The firmware may or may not have modulus key and exponent data. The header,
// uCode and RSA signature are must-have components that will be used by driver.
// Length of each components, which is all in dwords, can be found in header.
// In the case that modulus and exponent are not present in fw, a.k.a truncated
// image, the length value still appears in header.
//
// Driver will do some basic fw size validation based on the following rules:
//
// 1. Header, uCode and RSA are must-have components.
// 2. All firmware components, if they present, are in the sequence illustrated
// in the layout table above.
// 3. Length info of each component can be found in header, in dwords.
// 4. Modulus and exponent key are not required by driver. They may not appear
// in fw. So driver will load a truncated firmware in this case.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uc_css_rsa_info {
    pub key_size_dw: u32,
    pub modulus_size_dw: u32,
    pub exponent_size_dw: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uc_css_guc_info {
    pub time: u32,
    pub reserved0: [u32; 5],
    pub sw_version: u32,

    pub submission_version: u32,
    pub reserved1: [u32; 11],
    pub header_info: u32,

    pub private_data_size: u32,
    pub ukernel_info: u32,

pub const CSS_UKERNEL_INFO_BUILDTYPE_PROD: c_int = 0;
pub const CSS_UKERNEL_INFO_BUILDTYPE_PREPROD: c_int = 1;
pub const CSS_UKERNEL_INFO_BUILDTYPE_DEBUG: c_int = 2;

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uc_css_header {
    pub module_type: u32,
//
// header_size includes all non-uCode bits, including css_header, rsa
// key, modulus key and exponent data.
//
    pub header_size_dw: u32,
    pub header_version: u32,
    pub reserved0: u32,
    pub module_vendor: u32,
    pub date: u32,

    pub /: *mut *mut u32 size_dw; / uCode plus header_size_dw,
    pub reserved1: [u32; 3],
    pub rsa_info: uc_css_rsa_info,
}

//
// DOC: GSC-based Firmware Layout
//
// The GSC-based firmware structure is used for GSC releases on all platforms
// and for HuC releases starting from DG2/MTL. Older HuC releases use the
// CSS-based layout instead. Differently from the CSS headers, the GSC headers
// uses a directory + entries structure (i.e., there is array of addresses
// pointing to specific header extensions identified by a name). Although the
// header structures are the same, some of the entries are specific to GSC while
// others are specific to HuC. The manifest header entry, which includes basic
// information about the binary (like the version) is always present, but it is
// named differently based on the binary type.
//
// The HuC binary starts with a Code Partition Directory (CPD) header. The
// entries we're interested in for use in the driver are:
//
// 1. "HUCP.man": points to the manifest header for the HuC.
// 2. "huc_fw": points to the FW code. On platforms that support load via DMA
// and 2-step HuC authentication (i.e. MTL+) this is a full CSS-based binary,
// while if the GSC is the one doing the load (which only happens on DG2)
// this section only contains the uCode.
//
// The GSC-based HuC firmware layout looks like this::
//
// +================================================+
// |  CPD Header                                    |
// +================================================+
// |  CPD entries[]                                 |
// |      entry1                                    |
// |      ...                                       |
// |      entryX                                    |
// |          "HUCP.man"                            |
// |           ...                                  |
// |           offset  >----------------------------|------o
// |      ...                                       |      |
// |      entryY                                    |      |
// |          "huc_fw"                              |      |
// |           ...                                  |      |
// |           offset  >----------------------------|----------o
// +================================================+      |   |
// |   |
// +================================================+      |   |
// |  Manifest Header                               |<-----o   |
// |      ...                                       |          |
// |      FW version                                |          |
// |      ...                                       |          |
// +================================================+          |
// |
// +================================================+          |
// |  FW binary                                     |<---------o
// |      CSS (MTL+ only)                           |
// |      uCode                                     |
// |      RSA Key (MTL+ only)                       |
// |      ...                                       |
// +================================================+
//
// The GSC binary starts instead with a layout header, which contains the
// locations of the various partitions of the binary. The one we're interested
// in is the boot1 partition, where we can find a BPDT header followed by
// entries, one of which points to the RBE sub-section of the partition, which
// contains the CPD. The GSC blob does not contain a CSS-based binary, so we
// only need to look for the manifest, which is under the "RBEP.man" CPD entry.
// Note that we have no need to find where the actual FW code is inside the
// image because the GSC ROM will itself parse the headers to find it and load
// it.
// The GSC firmware header layout looks like this::
//
// +================================================+
// |  Layout Pointers                               |
// |      ...                                       |
// |      Boot1 offset  >---------------------------|------o
// |      ...                                       |      |
// +================================================+      |
// |
// +================================================+      |
// |  BPDT header                                   |<-----o
// +================================================+
// |  BPDT entries[]                                |
// |      entry1                                    |
// |      ...                                       |
// |      entryX                                    |
// |          type == GSC_RBE                       |
// |          offset  >-----------------------------|------o
// |      ...                                       |      |
// +================================================+      |
// |
// +================================================+      |
// |  CPD Header                                    |<-----o
// +================================================+
// |  CPD entries[]                                 |
// |      entry1                                    |
// |      ...                                       |
// |      entryX                                    |
// |          "RBEP.man"                            |
// |           ...                                  |
// |           offset  >----------------------------|------o
// |      ...                                       |      |
// +================================================+      |
// |
// +================================================+      |
// | Manifest Header                                |<-----o
// |  ...                                           |
// |  FW version                                    |
// |  ...                                           |
// |  Security version                              |
// |  ...                                           |
// +================================================+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_version {
    pub major: u16,
    pub minor: u16,
    pub hotfix: u16,
    pub build: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_partition {
    pub offset: u32,
    pub size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_layout_pointers {
    pub rom_bypass_vector: [u8; 16],
// size of this header section, not including ROM bypass vector
    pub size: u16,
//
// bit0: Backup copy of layout pointers exists
// bits1-15: reserved
//
    pub flags: u8,
    pub reserved: u8,
    pub crc32: u32,
    pub datap: gsc_partition,
    pub boot1: gsc_partition,
    pub boot2: gsc_partition,
    pub boot3: gsc_partition,
    pub boot4: gsc_partition,
    pub boot5: gsc_partition,
    pub temp_pages: gsc_partition,
    pub __packed: },
// Boot partition structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_bpdt_header {
    pub signature: u32,
pub const GSC_BPDT_HEADER_SIGNATURE: c_uint = 0x000055AA;
    pub /: *mut *mut u16 descriptor_count; / num of entries after the header,
    pub version: u8,
    pub configuration: u8,
    pub crc32: u32,
    pub build_version: u32,
    pub tool_version: gsc_version,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_bpdt_entry {
//
// Bits 0-15: BPDT entry type
// Bits 16-17: reserved
// Bit 18: code sub-partition
// Bits 19-31: reserved
//
    pub type: u32,

pub const GSC_BPDT_ENTRY_TYPE_GSC_RBE: c_uint = 0x1;
    pub /: *mut *mut u32 sub_partition_offset; / from the base of the BPDT header,
    pub sub_partition_size: u32,
    pub __packed: },
// Code partition directory (CPD) structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_cpd_header_v2 {
    pub header_marker: u32,
pub const GSC_CPD_HEADER_MARKER: c_uint = 0x44504324;
    pub num_of_entries: u32,
    pub header_version: u8,
    pub entry_version: u8,
    pub /: *mut *mut u8 header_length; / in bytes,
    pub flags: u8,
    pub partition_name: u32,
    pub crc32: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_cpd_entry {
    pub name: [u8; 12],
//
// Bits 0-24: offset from the beginning of the code partition
// Bit 25: huffman compressed
// Bits 26-31: reserved
//
    pub offset: u32,

//
// Module/Item length, in bytes. For Huffman-compressed modules, this
// refers to the uncompressed size. For software-compressed modules,
// this refers to the compressed size.
//
    pub length: u32,
    pub reserved: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsc_manifest_header {
    pub /: *mut *mut u32 header_type; / 0x4 for manifest type,
    pub /: *mut *mut u32 header_length; / in dwords,
    pub header_version: u32,
    pub flags: u32,
    pub vendor: u32,
    pub date: u32,
    pub /: *mut *mut u32 size; / In dwords, size of entire manifest (header + extensions),
    pub header_id: u32,
    pub internal_data: u32,
    pub fw_version: gsc_version,
    pub security_version: u32,
    pub meu_kit_version: gsc_version,
    pub meu_manifest_version: u32,
    pub general_data: [u8; 4],
    pub reserved3: [u8; 56],
    pub /: *mut *mut u32 modulus_size; / in dwords,
    pub /: *mut *mut u32 exponent_size; / in dwords,
    pub __packed: },
//
// DOC: Late binding Firmware Layout
//
// The Late binding binary starts with FPT header, which contains locations
// of various partitions of the binary. Here we're interested in finding out
// manifest version. To the manifest version, we need to locate CPD header
// one of the entry in CPD header points to manifest header. Manifest header
// contains the version.
//
// +================================================+
// |  FPT Header                                    |
// +================================================+
// |  FPT entries[]                                 |
// |      entry1                                    |
// |      ...                                       |
// |      entryX                                    |
// |          "LTES"                                |
// |          ...                                   |
// |          offset  >-----------------------------|------o
// +================================================+      |
// |
// +================================================+      |
// |  CPD Header                                    |<-----o
// +================================================+
// |  CPD entries[]                                 |
// |      entry1                                    |
// |      ...                                       |
// |      entryX                                    |
// |          "LTES.man"                            |
// |           ...                                  |
// |           offset  >----------------------------|------o
// +================================================+      |
// |
// +================================================+      |
// |  Manifest Header                               |<-----o
// |      ...                                       |
// |      FW version                                |
// |      ...                                       |
// +================================================+
//
// FPT Headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csc_fpt_header {
    pub header_marker: u32,
pub const CSC_FPT_HEADER_MARKER: c_uint = 0x54504624;
    pub num_of_entries: u32,
    pub header_version: u8,
    pub entry_version: u8,
    pub /: *mut *mut u8 header_length; / in bytes,
    pub flags: u8,
    pub ticks_to_add: u16,
    pub tokens_to_add: u16,
    pub uma_size: u32,
    pub crc32: u32,
    pub fitc_version: gsc_version,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csc_fpt_entry {
    pub /: *mut *mut u8 name[4]; / partition name,
    pub reserved1: u32,
    pub /: *mut *mut u32 offset; / offset from beginning of CSE region,
    pub /: *mut *mut u32 length; / partition length in bytes,
    pub reserved2: [u32; 3],
    pub partition_flags: u32,
    pub __packed: },
