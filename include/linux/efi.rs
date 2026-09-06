//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/efi.h
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
// Extensible Firmware Interface
// Based on 'Extensible Firmware Interface Specification' version 0.9, April 30, 1999
//
// Copyright (C) 1999 VA Linux Systems
// Copyright (C) 1999 Walt Drummond <drummond@valinux.com>
// Copyright (C) 1999, 2002-2003 Hewlett-Packard Co.
// David Mosberger-Tang <davidm@hpl.hp.com>
// Stephane Eranian <eranian@hpl.hp.com>
//

pub const EFI_SUCCESS: c_int = 0;

pub type efi_status_t = c_ulong;
pub type efi_bool_t = u8;
pub type efi_physical_addr_t = u64;

// Macro flag: #define __efiapi

//
// The UEFI spec and EDK2 reference implementation both define EFI_GUID as
// struct { u32 a; u16 b; u16 c; u8 d[8]; }; and so the implied alignment
// is 32 bits not 8 bits like our guid_t. In some cases (i.e., on 32-bit ARM),
// this means that firmware services invoked by the kernel may assume that
// efi_guid_t* arguments are 32-bit aligned, and use memory accessors that
// do not tolerate misalignment. So let's set the minimum alignment to 32 bits.
//
// Note that the UEFI spec as well as some comments in the EDK2 code base
// suggest that EFI_GUID should be 64-bit aligned, but this appears to be
// a mistake, given that no code seems to exist that actually enforces that
// or relies on it.
//
extern "C" {
    pub fn __aligned(_arg: __alignof__(u32)) -> typedef guid_t efi_guid_t;
}

//
// Generic EFI table header
//
// Memory map descriptor:
//
// Memory types:
pub const EFI_RESERVED_TYPE: c_int = 0;
pub const EFI_LOADER_CODE: c_int = 1;
pub const EFI_LOADER_DATA: c_int = 2;
pub const EFI_BOOT_SERVICES_CODE: c_int = 3;
pub const EFI_BOOT_SERVICES_DATA: c_int = 4;
pub const EFI_RUNTIME_SERVICES_CODE: c_int = 5;
pub const EFI_RUNTIME_SERVICES_DATA: c_int = 6;
pub const EFI_CONVENTIONAL_MEMORY: c_int = 7;
pub const EFI_UNUSABLE_MEMORY: c_int = 8;
pub const EFI_ACPI_RECLAIM_MEMORY: c_int = 9;
pub const EFI_ACPI_MEMORY_NVS: c_int = 10;
pub const EFI_MEMORY_MAPPED_IO: c_int = 11;
pub const EFI_MEMORY_MAPPED_IO_PORT_SPACE: c_int = 12;
pub const EFI_PAL_CODE: c_int = 13;
pub const EFI_PERSISTENT_MEMORY: c_int = 14;
pub const EFI_UNACCEPTED_MEMORY: c_int = 15;
pub const EFI_MAX_MEMORY_TYPE: c_int = 16;
// Attribute values:

pub const EFI_MEMORY_DESCRIPTOR_VERSION: c_int = 1;
pub const EFI_PAGE_SHIFT: c_int = 12;

// EFI_FIRMWARE_MANAGEMENT_CAPSULE_HEADER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_manage_capsule_header {
    pub ver: u32,
    pub emb_drv_cnt: u16,
    pub payload_cnt: u16,
//
// Variable-size array of the size given by the sum of
// emb_drv_cnt and payload_cnt.
//
    pub offset_list: [u64; ],
    pub __packed: },
// EFI_FIRMWARE_MANAGEMENT_CAPSULE_IMAGE_HEADER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_manage_capsule_image_header {
    pub ver: u32,
    pub image_type_id: efi_guid_t,
    pub image_index: u8,
    pub reserved_bytes: [u8; 3],
    pub image_size: u32,
    pub vendor_code_size: u32,
// hw_ins was introduced in version 2
    pub hw_ins: u64,
// capsule_support was introduced in version 3
    pub capsule_support: u64,
    pub __packed: },
// WIN_CERTIFICATE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct win_cert {
    pub len: u32,
    pub rev: u16,
    pub cert_type: u16,
}

// WIN_CERTIFICATE_UEFI_GUID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct win_cert_uefi_guid {
    pub hdr: win_cert,
    pub cert_type: efi_guid_t,
    pub cert_data: [u8; ],
}

// EFI_FIRMWARE_IMAGE_AUTHENTICATION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_image_auth {
    pub mon_count: u64,
    pub auth_info: win_cert_uefi_guid,
}

//
// EFI capsule flags
//
pub const EFI_CAPSULE_PERSIST_ACROSS_RESET: c_uint = 0x00010000;
pub const EFI_CAPSULE_POPULATE_SYSTEM_TABLE: c_uint = 0x00020000;
pub const EFI_CAPSULE_INITIATE_RESET: c_uint = 0x00040000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct capsule_info {
    pub header: efi_capsule_header_t,
    pub capsule: *mut efi_capsule_header_t,
    pub reset_type: c_int,
    pub index: c_long,
    pub count: usize,
    pub total_size: usize,
    pub pages: *mut page,
    pub phys: *mut phys_addr_t,
    pub page_bytes_remain: usize,
}

extern "C" {
    pub fn __efi_capsule_setup_info(cap_info: *mut capsule_info) -> c_int;
}
//
// Types and defines for Time Services
//
pub const EFI_TIME_ADJUST_DAYLIGHT: c_uint = 0x1;
pub const EFI_TIME_IN_DAYLIGHT: c_uint = 0x2;
pub const EFI_UNSPECIFIED_TIMEZONE: c_uint = 0x07ff;
pub type efi_boot_services_t = efi_boot_services;
//
// Types and defines for EFI ResetSystem
//
pub const EFI_RESET_COLD: c_int = 0;
pub const EFI_RESET_WARM: c_int = 1;
pub const EFI_RESET_SHUTDOWN: c_int = 2;
//
// EFI Runtime Services table
//

pub const EFI_RUNTIME_SERVICES_REVISION: c_uint = 0x00010000;
extern "C" {
    pub fn efi_get_time_t(tm: *mut efi_time_t, tc: *mut efi_time_cap_t) -> typedef efi_status_t;
}
extern "C" {
    pub fn efi_set_time_t(tm: *mut efi_time_t) -> typedef efi_status_t;
}
extern "C" {
    pub fn efi_set_wakeup_time_t(enabled: efi_bool_t, tm: *mut efi_time_t) -> typedef efi_status_t;
}
extern "C" {
    pub fn efi_get_next_high_mono_count_t(count: *mut u32) -> typedef efi_status_t;
}
extern "C" {
    pub fn efi_native_runtime_setup();
}
//
// EFI Configuration Table and GUID definitions
//
// These are all defined in a single line to make them easier to
// grep for and to see them at a glance - while still having a
// similar structure to the definitions in the spec.
//
// Here's how they are structured:
//
// GUID: 12345678-1234-1234-1234-123456789012
// Spec:
// #define EFI_SOME_PROTOCOL_GUID \
// {0x12345678,0x1234,0x1234,\
// {0x12,0x34,0x12,0x34,0x56,0x78,0x90,0x12}}
// Here:
// #define SOME_PROTOCOL_GUID		EFI_GUID(0x12345678, 0x1234, 0x1234,  0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12)
// ^ tabs					    ^extra space
//
// Note that the 'extra space' separates the values at the same place
// where the UEFI SPEC breaks the line.
//

//
// This GUIDs are used to pass to the kernel proper the primary
// display that has been populated by the stub based on the GOP
// instance associated with ConOut.
//

//
// This GUID may be installed onto the kernel image's handle as a NULL protocol
// to signal to the stub that the placement of the image should be respected,
// and moving the image in physical memory is undesirable. To ensure
// compatibility with 64k pages kernels with virtually mapped stacks, and to
// avoid defeating physical randomization, this protocol should only be
// installed if the image was placed at a randomized 128k aligned address in
// memory.
//

// OEM GUIDs

// OVMF protocol GUIDs

pub type efi_simple_text_input_protocol_t = efi_simple_text_input_protocol;
pub type efi_simple_text_output_protocol_t = efi_simple_text_output_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_boot_memmap {
    pub map_size: c_ulong,
    pub desc_size: c_ulong,
    pub desc_ver: u32,
    pub map_key: c_ulong,
    pub buff_size: c_ulong,
    pub map: [efi_memory_desc_t; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_unaccepted_memory {
    pub version: u32,
    pub unit_size: u32,
    pub phys_base: u64,
    pub size: u64,
    pub bitmap: [c_ulong; ],
}

//
// Architecture independent structure for describing a memory map for the
// benefit of efi_memmap_init_early(), and for passing context between
// efi_memmap_alloc() and efi_memmap_install().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_memory_map_data {
    pub phys_map: phys_addr_t,
    pub size: c_ulong,
    pub desc_version: c_ulong,
    pub desc_size: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_memory_map {
    pub phys_map: phys_addr_t,
    pub map: *mut c_void,
    pub map_end: *mut c_void,
    pub nr_map: c_int,
    pub desc_version: c_ulong,
    pub desc_size: c_ulong,

    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_mem_range {
    pub range: range,
    pub attribute: u64,
}

pub const EFI_RT_PROPERTIES_TABLE_VERSION: c_uint = 0x1;

// BIT0 implies that Runtime code includes the forward control flow guard
// instruction, such as X86 CET-IBT or ARM BTI.
pub const EFI_MEMORY_ATTRIBUTES_FLAGS_RT_FORWARD_CONTROL_FLOW_GUARD: c_uint = 0x1;
//
// There are @num_entries following, each of size @desc_size bytes,
// including an efi_memory_desc_t header. See efi_memdesc_ptr().
//
// efi_signature_data_t signatures[][]
//
// All runtime access to EFI goes through this structure:
//
pub const EFI_RT_SUPPORTED_GET_TIME: c_uint = 0x0001;
pub const EFI_RT_SUPPORTED_SET_TIME: c_uint = 0x0002;
pub const EFI_RT_SUPPORTED_GET_WAKEUP_TIME: c_uint = 0x0004;
pub const EFI_RT_SUPPORTED_SET_WAKEUP_TIME: c_uint = 0x0008;
pub const EFI_RT_SUPPORTED_GET_VARIABLE: c_uint = 0x0010;
pub const EFI_RT_SUPPORTED_GET_NEXT_VARIABLE_NAME: c_uint = 0x0020;
pub const EFI_RT_SUPPORTED_SET_VARIABLE: c_uint = 0x0040;
pub const EFI_RT_SUPPORTED_SET_VIRTUAL_ADDRESS_MAP: c_uint = 0x0080;
pub const EFI_RT_SUPPORTED_CONVERT_POINTER: c_uint = 0x0100;
pub const EFI_RT_SUPPORTED_GET_NEXT_HIGH_MONOTONIC_COUNT: c_uint = 0x0200;
pub const EFI_RT_SUPPORTED_RESET_SYSTEM: c_uint = 0x0400;
pub const EFI_RT_SUPPORTED_UPDATE_CAPSULE: c_uint = 0x0800;
pub const EFI_RT_SUPPORTED_QUERY_CAPSULE_CAPABILITIES: c_uint = 0x1000;
pub const EFI_RT_SUPPORTED_QUERY_VARIABLE_INFO: c_uint = 0x2000;
pub const EFI_RT_SUPPORTED_ALL: c_uint = 0x3fff;
pub const EFI_RT_SUPPORTED_TIME_SERVICES: c_uint = 0x0003;
pub const EFI_RT_SUPPORTED_WAKEUP_SERVICES: c_uint = 0x000c;
pub const EFI_RT_SUPPORTED_VARIABLE_SERVICES: c_uint = 0x0070;
extern "C" {
    pub fn memcmp(_arg: &left, _arg: &right, (efi_guid_t): sizeof) -> return;
}
extern "C" {
    pub fn efi_init();
}
extern "C" {
    pub fn efi_earlycon_reprobe();
}

extern "C" {
    pub fn __efi_memmap_init(data: *mut efi_memory_map_data) -> int __init;
}
extern "C" {
    pub fn efi_memmap_init_early(data: *mut efi_memory_map_data) -> int __init;
}
extern "C" {
    pub fn efi_memmap_init_late(addr: phys_addr_t, size: c_ulong) -> int __init;
}
extern "C" {
    pub fn efi_memmap_unmap() -> void __init;
}

extern "C" {
    pub fn efi_esrt_init() -> void __init;
}

extern "C" {
    pub fn efi_systab_check_header(systab_hdr: *const efi_table_hdr_t) -> c_int;
}
extern "C" {
    pub fn efi_get_iobase() -> u64;
}
extern "C" {
    pub fn efi_mem_type(phys_addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn efi_mem_attributes(phys_addr: c_ulong) -> u64;
}
extern "C" {
    pub fn efi_mem_attribute(phys_addr: c_ulong, size: c_ulong) -> u64;
}
extern "C" {
    pub fn efi_uart_console_only() -> int __init;
}
extern "C" {
    pub fn efi_mem_desc_end(md: *mut efi_memory_desc_t) -> u64;
}
extern "C" {
    pub fn efi_mem_desc_lookup(phys_addr: u64, out_md: *mut efi_memory_desc_t) -> c_int;
}
extern "C" {
    pub fn __efi_mem_desc_lookup(phys_addr: u64, out_md: *mut efi_memory_desc_t) -> c_int;
}
extern "C" {
    pub fn efi_mem_reserve(addr: phys_addr_t, size: u64);
}
extern "C" {
    pub fn efi_mem_reserve_persistent(addr: phys_addr_t, size: u64) -> c_int;
}
extern "C" {
    pub fn efi_get_fdt_params(data: *mut efi_memory_map_data) -> u64;
}
extern "C" {
    pub fn efi_poweroff_required() -> bool;
}
//
// efi_memattr_perm_setter - arch specific callback function passed into
// efi_memattr_apply_permissions() that updates the
// mapping permissions described by the second
// argument in the page tables referred to by the
// first argument.
//
extern "C" {
    pub fn int(: *mut *mut efi_memattr_perm_setter)(struct mm_struct, : *mut efi_memory_desc_t, _arg: bool) -> typedef;
}
extern "C" {
    pub fn efi_memattr_init();
}
//
// efi_memdesc_ptr - get the n-th EFI memmap descriptor
// @map: the start of efi memmap
// @desc_size: the size of space for each EFI memmap descriptor
// @n: the index of efi memmap descriptor
//
// EFI boot service provides the GetMemoryMap() function to get a copy of the
// current memory map which is an array of memory descriptors, each of
// which describes a contiguous block of memory. It also gets the size of the
// map, and the size of each descriptor, etc.
//
// Note that per section 6.2 of UEFI Spec 2.6 Errata A, the returned size of
// each descriptor might not be equal to sizeof(efi_memory_memdesc_t),
// since efi_memory_memdesc_t may be extended in the future. Thus the OS
// MUST use the returned size of the descriptor to find the start of each
// efi_memory_memdesc_t in the memory map array. This should only be used
// during bootup since for_each_efi_memory_desc_xxx() is available after the
// kernel initializes the EFI subsystem to set up struct efi_memory_map.
//

// Iterate through an efi_memory_map

//
// for_each_efi_memory_desc - iterate over descriptors in efi.memmap
// @md: the efi_memory_desc_t * iterator
//
// Once the loop finishes @md must not be accessed.
//

//
// Format an EFI memory descriptor's type and attributes to a user-provided
// character buffer, as per snprintf(), and return the buffer.
//
// We play games with efi_enabled so that the compiler will, if
// possible, remove EFI-related code altogether.
//

//
// Test whether the above EFI_* bits are enabled.
//
extern "C" {
    pub fn efi_reboot(reboot_mode: reboot_mode, __unused: *const c_char);
}
extern "C" {
    pub fn __efi_soft_reserve_enabled() -> bool __pure;
}
extern "C" {
    pub fn efi_find_mirror();
}

extern "C" {
    pub fn efi_status_to_err(status: efi_status_t) -> c_int;
}
//
// Variable Attributes
//
pub const EFI_VARIABLE_NON_VOLATILE: c_uint = 0x0000000000000001;
pub const EFI_VARIABLE_BOOTSERVICE_ACCESS: c_uint = 0x0000000000000002;
pub const EFI_VARIABLE_RUNTIME_ACCESS: c_uint = 0x0000000000000004;
pub const EFI_VARIABLE_HARDWARE_ERROR_RECORD: c_uint = 0x0000000000000008;
pub const EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS: c_uint = 0x0000000000000010;
pub const EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS: c_uint = 0x0000000000000020;
pub const EFI_VARIABLE_APPEND_WRITE: c_uint = 0x0000000000000040;

//
// Length of a GUID string (strlen("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"))
// not including trailing NUL
//

//
// EFI Device Path information
//
pub const EFI_DEV_HW: c_uint = 0x01;
pub const EFI_DEV_PCI: c_int = 1;
pub const EFI_DEV_PCCARD: c_int = 2;
pub const EFI_DEV_MEM_MAPPED: c_int = 3;
pub const EFI_DEV_VENDOR: c_int = 4;
pub const EFI_DEV_CONTROLLER: c_int = 5;
pub const EFI_DEV_ACPI: c_uint = 0x02;
pub const EFI_DEV_BASIC_ACPI: c_int = 1;
pub const EFI_DEV_EXPANDED_ACPI: c_int = 2;
pub const EFI_DEV_MSG: c_uint = 0x03;
pub const EFI_DEV_MSG_ATAPI: c_int = 1;
pub const EFI_DEV_MSG_SCSI: c_int = 2;
pub const EFI_DEV_MSG_FC: c_int = 3;
pub const EFI_DEV_MSG_1394: c_int = 4;
pub const EFI_DEV_MSG_USB: c_int = 5;
pub const EFI_DEV_MSG_USB_CLASS: c_int = 15;
pub const EFI_DEV_MSG_I20: c_int = 6;
pub const EFI_DEV_MSG_MAC: c_int = 11;
pub const EFI_DEV_MSG_IPV4: c_int = 12;
pub const EFI_DEV_MSG_IPV6: c_int = 13;
pub const EFI_DEV_MSG_INFINIBAND: c_int = 9;
pub const EFI_DEV_MSG_UART: c_int = 14;
pub const EFI_DEV_MSG_VENDOR: c_int = 10;
pub const EFI_DEV_MEDIA: c_uint = 0x04;
pub const EFI_DEV_MEDIA_HARD_DRIVE: c_int = 1;
pub const EFI_DEV_MEDIA_CDROM: c_int = 2;
pub const EFI_DEV_MEDIA_VENDOR: c_int = 3;
pub const EFI_DEV_MEDIA_FILE: c_int = 4;
pub const EFI_DEV_MEDIA_PROTOCOL: c_int = 5;
pub const EFI_DEV_MEDIA_REL_OFFSET: c_int = 8;
pub const EFI_DEV_BIOS_BOOT: c_uint = 0x05;
pub const EFI_DEV_END_PATH: c_uint = 0x7F;
pub const EFI_DEV_END_PATH2: c_uint = 0xFF;
pub const EFI_DEV_END_INSTANCE: c_uint = 0x01;
pub const EFI_DEV_END_ENTIRE: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_generic_dev_path {
    pub type: u8,
    pub sub_type: u8,
    pub length: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_acpi_dev_path {
    pub header: efi_generic_dev_path,
    pub hid: u32,
    pub uid: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_pci_dev_path {
    pub header: efi_generic_dev_path,
    pub fn: u8,
    pub dev: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_vendor_dev_path {
    pub header: efi_generic_dev_path,
    pub vendorguid: efi_guid_t,
    pub vendordata: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_rel_offset_dev_path {
    pub header: efi_generic_dev_path,
    pub reserved: u32,
    pub starting_offset: u64,
    pub ending_offset: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_mem_mapped_dev_path {
    pub header: efi_generic_dev_path,
    pub memory_type: u32,
    pub starting_addr: u64,
    pub ending_addr: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_file_path_dev_path {
    pub header: efi_generic_dev_path,
    pub filename: [efi_char16_t; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_dev_path {
    pub header: efi_generic_dev_path,
    pub acpi: efi_acpi_dev_path,
    pub pci: efi_pci_dev_path,
    pub vendor: efi_vendor_dev_path,
    pub rel_offset: efi_rel_offset_dev_path,
}

// npages = PFN_UP(*addr + (*npages<<EFI_PAGE_SHIFT)) - PFN_DOWN(*addr);
// addr &= PAGE_MASK;
//
// EFI Variable support.
//
// Different firmware drivers can expose their EFI-like variables using
// the following.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efivar_operations {
    pub get_variable: *mut efi_get_variable_t,
    pub get_next_variable: *mut efi_get_next_variable_t,
    pub set_variable: *mut efi_set_variable_t,
    pub set_variable_nonblocking: *mut efi_set_variable_t,
    pub query_variable_store: *mut efi_query_variable_store_t,
    pub query_variable_info: *mut efi_query_variable_info_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efivars {
    pub kset: *mut kset,
    pub ops: *const efivar_operations,
}

extern "C" {
    pub fn efivar_reserved_space() -> u64 __attribute_const__;
}

//
// There is no actual upper limit specified for the variable name size.
//
// This limit exists only for practical purposes, since name conversions
// are bounds-checked and name data is occasionally stored in-line.
//
pub const EFI_VAR_NAME_LEN: c_int = 1024;
extern "C" {
    pub fn efivars_unregister(efivars: *mut efivars) -> c_int;
}

extern "C" {
    pub fn efivar_is_available() -> bool;
}

extern "C" {
    pub fn efivar_supports_writes() -> bool;
}
extern "C" {
    pub fn efivar_lock() -> c_int;
}
extern "C" {
    pub fn efivar_trylock() -> c_int;
}
extern "C" {
    pub fn efivar_unlock();
}

extern "C" {
    pub fn efi_capsule_pending(reset_type: *mut c_int) -> bool;
}

extern "C" {
    pub fn efi_runtime_disabled() -> bool;
}

extern "C" {
    pub fn efi_call_virt_check_flags(flags: c_ulong, caller: *const c_void);
}
extern "C" {
    pub fn efi_call_virt_save_flags() -> c_ulong;
}
extern "C" {
    pub fn efi_runtime_assert_lock_held();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efi_secureboot_mode {
    efi_secureboot_mode_unset,
    efi_secureboot_mode_unknown,
    efi_secureboot_mode_disabled,
    efi_secureboot_mode_enabled,
}

extern "C" {
    pub fn efi_check_for_embedded_firmwares();
}

//
// Arch code must implement the following three routines:
//
// * arch_efi_call_virt_setup()
//
// Sets up the environment for the call (e.g. switching page tables,
// allowing kernel-mode use of floating point, if required).
//
// * arch_efi_call_virt()
//
// Performs the call. This routine takes a variable number of arguments so
// it must be implemented as a variadic preprocessor macro.
//
// * arch_efi_call_virt_teardown()
//
// Restores the usual kernel environment once the call has returned.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_efi_random_seed {
    pub size: u32,
    pub bits: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_efi_tpm_eventlog {
    pub size: u32,
    pub final_events_preboot_size: u32,
    pub version: u8,
    pub log: [u8; ],
}

extern "C" {
    pub fn efi_tpm_eventlog_init() -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_tcg2_final_events_table {
    pub version: u64,
    pub nr_events: u64,
    pub events: [u8; ],
}

//
// efi_runtime_service() function identifiers.
// "NONE" is used by efi_crash_gracefully_on_page_fault() to check if the
// page fault happened while executing an efi runtime service.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efi_rts_ids {
    EFI_NONE,
    EFI_GET_TIME,
    EFI_SET_TIME,
    EFI_GET_WAKEUP_TIME,
    EFI_SET_WAKEUP_TIME,
    EFI_GET_VARIABLE,
    EFI_GET_NEXT_VARIABLE,
    EFI_SET_VARIABLE,
    EFI_QUERY_VARIABLE_INFO,
    EFI_GET_NEXT_HIGH_MONO_COUNT,
    EFI_RESET_SYSTEM,
    EFI_UPDATE_CAPSULE,
    EFI_QUERY_CAPSULE_CAPS,
    EFI_ACPI_PRM_HANDLER,
}

//
// efi_runtime_work:	Details of EFI Runtime Service work
// @args:		Pointer to union describing the arguments
// @status:		Status of executing EFI Runtime Service
// @efi_rts_id:		EFI Runtime Service function identifier
// @efi_rts_comp:	Struct used for handling completions
// @caller:		The caller of the runtime service
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_runtime_work {
    pub args: *mut efi_rts_args,
    pub status: efi_status_t,
    pub work: work_struct,
    pub efi_rts_id: efi_rts_ids,
    pub efi_rts_comp: completion,
    pub caller: *const c_void,
}

// Workqueue to queue EFI Runtime Services
extern "C" {
    pub fn efi_rts_park_worker() -> void __noreturn;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_efi_memreserve {
    pub array: int size; // allocated size of the,
    pub used: atomic_t count; // number of entries,
    pub instance: phys_addr_t next; // pa of next struct,
    pub base: phys_addr_t,
    pub size: phys_addr_t,
    pub entry: [}; ],
}

extern "C" {
    pub fn efi_arch_mem_reserve(addr: phys_addr_t, size: u64) -> void __init;
}
//
// The LINUX_EFI_MOK_VARIABLE_TABLE_GUID config table can be provided
// to the kernel by an EFI boot loader. The table contains a packed
// sequence of these entries, one for each named MOK variable.
// The sequence is terminated by an entry with a completely NULL
// name and 0 data size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_mokvar_table_entry {
    pub name: [c_char; 256],
    pub data_size: u64,
    pub data: [u8; ],
    pub __attribute((packed)): },

    pub efi_mokvar_table_init(void): extern void __init,
    pub mokvar_entry): *mut efi_mokvar_table_entry,
    pub name): *const *const extern struct efi_mokvar_table_entry efi_mokvar_entry_find(char,

    pub NULL: return,
    pub NULL: return,

    pub opt): *const *const extern void efifb_setup_from_dmi(struct screen_info si, char,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_efi_coco_secret_area {
    pub base_pa: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linux_efi_initrd {
    pub base: c_ulong,
    pub size: c_ulong,
}

// Header of a populated EFI secret area

extern "C" {
    pub fn xen_efi_config_table_is_usable(guid: *const efi_guid_t, table: c_ulong) -> bool;
}
extern "C" {
    pub fn xen_efi_config_table_is_usable(_arg: guid, _arg: table) -> return;
}
extern "C" {
    pub fn efi_attr_is_visible(kobj: *mut kobject, attr: *mut attribute, n: c_int) -> umode_t;
}
extern "C" {
    pub fn ovmf_log_probe(ovmf_debug_log_table: c_ulong) -> c_int;
}
//
// efivar ops event type
//
pub const EFIVAR_OPS_RDONLY: c_int = 0;
pub const EFIVAR_OPS_RDWR: c_int = 1;
extern "C" {
    pub fn efivars_generic_ops_register();
}
extern "C" {
    pub fn efivars_generic_ops_unregister();
}
