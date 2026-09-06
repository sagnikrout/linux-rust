//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpi3mr/mpi/mpi30_image.h
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
// Copyright 2018-2026 Broadcom Inc. All rights reserved.
//
pub const MPI30_IMAGE_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_comp_image_version {
    pub build_num: __le16,
    pub customer_id: __le16,
    pub phase_minor: u8,
    pub phase_major: u8,
    pub gen_minor: u8,
    pub gen_major: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_hash_exclusion_format {
    pub offset: __le32,
    pub size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_component_image_header {
    pub signature0: __le32,
    pub load_address: __le32,
    pub data_size: __le32,
    pub start_offset: __le32,
    pub signature1: __le32,
    pub flash_offset: __le32,
    pub image_size: __le32,
    pub version_string_offset: __le32,
    pub build_date_string_offset: __le32,
    pub build_time_string_offset: __le32,
    pub environment_variable_offset: __le32,
    pub application_specific: __le32,
    pub signature2: __le32,
    pub header_size: __le32,
    pub crc: __le32,
    pub flags: __le32,
    pub secondary_flash_offset: __le32,
    pub etp_offset: __le32,
    pub etp_size: __le32,
    pub rmc_interface_version: mpi3_version_union,
    pub etp_interface_version: mpi3_version_union,
    pub component_image_version: mpi3_comp_image_version,
    pub hash_exclusion: [mpi3_hash_exclusion_format; MPI3_IMAGE_HASH_EXCUSION_NUM],
    pub next_image_header_offset: __le32,
    pub security_version: mpi3_version_union,
    pub reserved84: [__le32; 31],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ci_manifest_mpi_comp_image_ref {
    pub signature1: __le32,
    pub reserved04: [__le32; 3],
    pub component_image_version: mpi3_comp_image_version,
    pub component_image_version_string_offset: __le32,
    pub crc: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_ci_manifest_mpi {
    pub manifest_type: u8,
    pub reserved01: [u8; 3],
    pub reserved04: [__le32; 3],
    pub num_image_references: u8,
    pub release_level: u8,
    pub reserved12: __le16,
    pub reserved14: __le16,
    pub flags: __le16,
    pub reserved18: [__le32; 2],
    pub vendor_id: __le16,
    pub device_id: __le16,
    pub subsystem_vendor_id: __le16,
    pub subsystem_id: __le16,
    pub reserved28: [__le32; 2],
    pub package_security_version: mpi3_version_union,
    pub reserved34: __le32,
    pub package_version: mpi3_comp_image_version,
    pub package_version_string_offset: __le32,
    pub package_build_date_string_offset: __le32,
    pub package_build_time_string_offset: __le32,
    pub diag_authorization_key_offset: __le32,
    pub diag_authorization_identifier: [__le32; 16],
    pub component_image_ref: [mpi3_ci_manifest_mpi_comp_image_ref; MPI3_CI_MANIFEST_MPI_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sb_manifest_ci_digest {
    pub signature1: __le32,
    pub reserved04: [__le32; 2],
    pub hash_algorithm: u8,
    pub reserved09: [u8; 3],
    pub component_image_version: mpi3_comp_image_version,
    pub component_image_version_string_offset: __le32,
    pub digest: [__le32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sb_manifest_ci_ref_element {
    pub num_ci_digests: u8,
    pub reserved01: [u8; 3],
    pub ci_digest: [mpi3_sb_manifest_ci_digest; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sb_manifest_embedded_key_element {
    pub reserved00: [__le32; 3],
    pub key_algorithm: u8,
    pub flags: u8,
    pub public_key_size: __le16,
    pub start_tag: __le32,
    pub public_key: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sb_manifest_diag_key_element {
    pub reserved00: [__le32; 3],
    pub key_algorithm: u8,
    pub flags: u8,
    pub public_key_size: __le16,
    pub public_key: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_sb_manifest_element_data {
    pub ci_ref: mpi3_sb_manifest_ci_ref_element,
    pub embed_key: mpi3_sb_manifest_embedded_key_element,
    pub diag_key: mpi3_sb_manifest_diag_key_element,
    pub dword: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sb_manifest_element {
    pub manifest_element_form: u8,
    pub reserved01: [u8; 3],
    pub form_specific: [mpi3_sb_manifest_element_data; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_sb_manifest_mpi {
    pub manifest_type: u8,
    pub reserved01: [u8; 3],
    pub reserved04: [__le32; 3],
    pub reserved10: u8,
    pub release_level: u8,
    pub reserved12: __le16,
    pub reserved14: __le16,
    pub flags: __le16,
    pub reserved18: [__le32; 2],
    pub vendor_id: __le16,
    pub device_id: __le16,
    pub subsystem_vendor_id: __le16,
    pub subsystem_id: __le16,
    pub reserved28: [__le32; 2],
    pub package_security_version: mpi3_version_union,
    pub reserved34: __le32,
    pub package_version: mpi3_comp_image_version,
    pub package_version_string_offset: __le32,
    pub package_build_date_string_offset: __le32,
    pub package_build_time_string_offset: __le32,
    pub component_image_references_offset: __le32,
    pub embedded_key0offset: __le32,
    pub embedded_key1offset: __le32,
    pub diag_authorization_key_offset: __le32,
    pub reserved5c: [__le32; 9],
    pub manifest_elements: [mpi3_sb_manifest_element; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mpi3_ci_manifest {
    pub mpi: mpi3_ci_manifest_mpi,
    pub sb_mpi: mpi3_sb_manifest_mpi,
    pub dword: [__le32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_extended_image_header {
    pub image_type: u8,
    pub reserved01: [u8; 3],
    pub checksum: __le32,
    pub image_size: __le32,
    pub next_image_header_offset: __le32,
    pub reserved10: [__le32; 4],
    pub identify_string: [__le32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_supported_device {
    pub device_id: __le16,
    pub vendor_id: __le16,
    pub device_id_mask: __le16,
    pub reserved06: __le16,
    pub low_pci_rev: u8,
    pub high_pci_rev: u8,
    pub reserved0a: __le16,
    pub reserved0c: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_supported_devices_data {
    pub image_version: u8,
    pub reserved01: u8,
    pub num_devices: u8,
    pub reserved03: u8,
    pub reserved04: __le32,
    pub supported_device: [mpi3_supported_device; MPI3_SUPPORTED_DEVICE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_encrypted_hash_entry {
    pub hash_image_type: u8,
    pub hash_algorithm: u8,
    pub encryption_algorithm: u8,
    pub flags: u8,
    pub public_key_size: __le16,
    pub signature_size: __le16,
    pub public_key: [__le32; MPI3_PUBLIC_KEY_MAX],
}

// hierarchical signature system (hss)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_encrypted_hash_data {
    pub image_version: u8,
    pub num_hash: u8,
    pub reserved02: __le16,
    pub reserved04: __le32,
    pub encrypted_hash_entry: [mpi3_encrypted_hash_entry; MPI3_ENCRYPTED_HASH_ENTRY_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi3_aux_processor_data {
    pub boot_method: u8,
    pub num_load_addr: u8,
    pub reserved02: u8,
    pub type: u8,
    pub version: __le32,
    pub load_address: [__le32; 8],
    pub reserved28: [__le32; 22],
    pub aux_processor_data: [__le32; MPI3_AUX_PROC_DATA_MAX],
}

