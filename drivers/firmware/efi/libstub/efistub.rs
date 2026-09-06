//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/efi/libstub/efistub.h
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
// __init annotations should not be used in the EFI stub, since the code is
// either included in the decompressor (x86, ARM) where they have no effect,
// or the whole stub is __init annotated at the section level (arm64), by
// renaming the sections, in which case the __init annotation will be
// redundant, and will result in section names like .init.init.text, and our
// linker script does not expect that.
//

//
// Allow the platform to override the allocation granularity: this allows
// systems that have the capability to run with a larger page size to deal
// with the allocations for initrd and fdt more efficiently.
//

pub type efi_dxe_services_table_t = efi_dxe_services_table;

// Helper macros for the usual case of using simple C variables:

// lo = lower_32_bits(data);
// hi = upper_32_bits(data);
//
// Allocation types for calls to boottime->allocate_pages.
//
pub const EFI_ALLOCATE_ANY_PAGES: c_int = 0;
pub const EFI_ALLOCATE_MAX_ADDRESS: c_int = 1;
pub const EFI_ALLOCATE_ADDRESS: c_int = 2;
pub const EFI_MAX_ALLOCATE_TYPE: c_int = 3;
//
// The type of search to perform when calling boottime->locate_handle
//
pub const EFI_LOCATE_ALL_HANDLES: c_int = 0;
pub const EFI_LOCATE_BY_REGISTER_NOTIFY: c_int = 1;
pub const EFI_LOCATE_BY_PROTOCOL: c_int = 2;
//
// boottime->stall takes the time period in microseconds
//
pub const EFI_USEC_PER_SEC: c_int = 1000000;
//
// boottime->set_timer takes the time in 100ns units
//

//
// An efi_boot_memmap is used by efi_get_memory_map() to return the
// EFI memory map in a dynamically allocated buffer.
//
// The buffer allocated for the EFI memory map includes extra room for
// a minimum of EFI_MMAP_NR_SLACK_SLOTS additional EFI memory descriptors.
// This facilitates the reuse of the EFI memory map buffer when a second
// call to ExitBootServices() is needed because of intervening changes to
// the EFI memory map. Other related structures, e.g. x86 e820ext, need
// to factor in this headroom requirement as well.
//
pub const EFI_MMAP_NR_SLACK_SLOTS: c_int = 32;
pub type efi_device_path_protocol_t = efi_generic_dev_path;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_device_path_to_text_protocol {
    pub bool): bool,,
    pub bool): bool,,
}

pub type efi_device_path_to_text_protocol_t = efi_device_path_to_text_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_device_path_from_text_protocol {
    pub ): *const *const (__efiapi convert_text_to_device_node)(efi_char16_t,
    pub ): *const *const (__efiapi convert_text_to_device_path)(efi_char16_t,
}

pub type efi_device_path_from_text_protocol_t = efi_device_path_from_text_protocol;
// Note that notifications won't work in mixed mode
extern "C" {
    pub fn void(efi_event_notify_t)(efi_event_t: *mut __efiapi, : *mut c_void) -> typedef;
}
pub const EFI_EVT_TIMER: c_uint = 0x80000000U;
pub const EFI_EVT_RUNTIME: c_uint = 0x40000000U;
pub const EFI_EVT_NOTIFY_WAIT: c_uint = 0x00000100U;
pub const EFI_EVT_NOTIFY_SIGNAL: c_uint = 0x00000200U;
//
// efi_set_event_at() - add event to events array
//
// @events:	array of UEFI events
// @ids:	index where to put the event in the array
// @event:	event to add to the aray
//
// boottime->wait_for_event() takes an array of events as input.
// Provide a helper to set it up correctly for mixed mode.
//
pub const EFI_TPL_APPLICATION: c_int = 4;
pub const EFI_TPL_CALLBACK: c_int = 8;
pub const EFI_TPL_NOTIFY: c_int = 16;
pub const EFI_TPL_HIGH_LEVEL: c_int = 31;
//
// EFI Boot Services table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_boot_services {
    pub hdr: efi_table_hdr_t,
    pub raise_tpl: *mut c_void,
    pub restore_tpl: *mut c_void,
    pub ): *mut efi_physical_addr_t,
    pub long): unsigned,
    pub ): *mut *mut unsigned long , u32,
    pub ): *mut c_void,
    pub ): *mut *mut efi_status_t (__efiapi free_pool)(void,
    pub ): *mut efi_event_t,
    pub u64): EFI_TIMER_DELAY,,
    pub ): *mut c_ulong,
    pub signal_event: *mut c_void,
    pub close_event)(efi_event_t): *mut efi_status_t (__efiapi,
    pub check_event: *mut c_void,
    pub install_protocol_interface: *mut c_void,
    pub reinstall_protocol_interface: *mut c_void,
    pub uninstall_protocol_interface: *mut c_void,
    pub ): *mut *mut efi_guid_t , void,
    pub __reserved: *mut c_void,
    pub register_protocol_notify: *mut c_void,
    pub ): *mut efi_handle_t,
    pub ): *mut efi_handle_t,
    pub ): *mut c_void,
    pub ): *mut efi_handle_t,
    pub ): *mut efi_char16_t,
    pub ): *mut efi_char16_t,
    pub unload_image)(efi_handle_t): *mut efi_status_t (__efiapi,
    pub long): unsigned,
    pub get_next_monotonic_count: *mut c_void,
    pub long): *mut *mut efi_status_t (__efiapi stall)(unsigned,
    pub set_watchdog_timer: *mut c_void,
    pub connect_controller: *mut c_void,
    pub open_protocol: *mut c_void,
    pub close_protocol: *mut c_void,
    pub open_protocol_information: *mut c_void,
    pub protocols_per_handle: *mut c_void,
    pub ): *mut efi_handle_t,
    pub ): *mut c_void,
    pub ...): *mut *mut *mut efi_status_t (__efiapi install_multiple_protocol_interfaces)(efi_handle_t ,,
    pub ...): *mut *mut efi_status_t (__efiapi uninstall_multiple_protocol_interfaces)(efi_handle_t,,
    pub calculate_crc32: *mut c_void,
    pub long): *const *const *const *const void (__efiapi copy_mem)(void , void , unsigned,
    pub char): *mut *mut *mut void (__efiapi set_mem)(void , unsigned long, unsigned,
    pub create_event_ex: *mut c_void,
}

//
// EFI DXE Services table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_dxe_services_table {
    pub hdr: efi_table_hdr_t,
    pub add_memory_space: *mut c_void,
    pub allocate_memory_space: *mut c_void,
    pub free_memory_space: *mut c_void,
    pub remove_memory_space: *mut c_void,
    pub ): *mut efi_gcd_memory_space_desc_t,
    pub u64): u64,,
    pub get_memory_space_map: *mut c_void,
    pub add_io_space: *mut c_void,
    pub allocate_io_space: *mut c_void,
    pub free_io_space: *mut c_void,
    pub remove_io_space: *mut c_void,
    pub get_io_space_descriptor: *mut c_void,
    pub get_io_space_map: *mut c_void,
    pub dispatch: *mut c_void,
    pub schedule: *mut c_void,
    pub trust: *mut c_void,
    pub process_firmware_volume: *mut c_void,
    pub set_memory_space_capabilities: *mut c_void,
}

pub type efi_memory_attribute_protocol_t = efi_memory_attribute_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_memory_attribute_protocol {
    pub ): *mut *mut efi_memory_attribute_protocol_t , efi_physical_addr_t, u64, u64,
    pub u64): *mut *mut efi_memory_attribute_protocol_t , efi_physical_addr_t, u64,,
    pub u64): *mut *mut efi_memory_attribute_protocol_t , efi_physical_addr_t, u64,,
}

pub type efi_uga_draw_protocol_t = efi_uga_draw_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_uga_draw_protocol {
    pub u32*): *mut *mut *mut *mut u32, u32, u32,,
    pub set_mode: *mut c_void,
    pub blt: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_simple_text_input_protocol {
    pub reset: *mut c_void,
    pub ): *mut efi_input_key_t,
    pub wait_for_key: efi_event_t,
}

extern "C" {
    pub fn efi_wait_for_key(usec: c_ulong, key: *mut efi_input_key_t) -> efi_status_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_simple_text_output_protocol {
    pub reset: *mut c_void,
    pub ): *mut efi_char16_t,
    pub test_string: *mut c_void,
}

pub const PIXEL_RGB_RESERVED_8BIT_PER_COLOR: c_int = 0;
pub const PIXEL_BGR_RESERVED_8BIT_PER_COLOR: c_int = 1;
pub const PIXEL_BIT_MASK: c_int = 2;
pub const PIXEL_BLT_ONLY: c_int = 3;
pub const PIXEL_FORMAT_MAX: c_int = 4;
pub type efi_graphics_output_protocol_mode_t = efi_graphics_output_protocol_mode;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_graphics_output_protocol_mode {
    pub max_mode: u32,
    pub mode: u32,
    pub info: *mut efi_graphics_output_mode_info_t,
    pub size_of_info: c_ulong,
    pub frame_buffer_base: efi_physical_addr_t,
    pub frame_buffer_size: c_ulong,
}

pub type efi_graphics_output_protocol_t = efi_graphics_output_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_graphics_output_protocol {
    pub ): *mut efi_graphics_output_mode_info_t,
    pub u32): *mut *mut *mut efi_status_t (__efiapi set_mode) (efi_graphics_output_protocol_t ,,
    pub blt: *mut c_void,
    pub mode: *mut efi_graphics_output_protocol_mode_t,
}

pub type efi_edid_discovered_protocol_t = efi_edid_discovered_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_edid_discovered_protocol {
    pub size_of_edid: u32,
    pub edid: *mut u8,
}

pub type efi_edid_active_protocol_t = efi_edid_active_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_edid_active_protocol {
    pub size_of_edid: u32,
    pub edid: *mut u8,
}

pub type efi_file_protocol_t = efi_file_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_file_protocol {
    pub revision: u64,
    pub ): *mut *mut efi_status_t (__efiapi close) (efi_file_protocol_t,
    pub ): *mut *mut efi_status_t (__efiapi delete) (efi_file_protocol_t,
    pub ): *mut c_void,
    pub ): *mut unsigned long, void,
    pub ): *mut u64,
    pub ): *mut c_void,
    pub ): *mut c_void,
    pub ): *mut *mut efi_status_t (__efiapi flush) (efi_file_protocol_t,
}

pub type efi_simple_file_system_protocol_t = efi_simple_file_system_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_simple_file_system_protocol {
    pub revision: u64,
    pub ): *mut efi_file_protocol_t,
}

pub const EFI_FILE_MODE_READ: c_uint = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: c_uint = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: c_uint = 0x8000000000000000;
pub type efi_pci_io_protocol_t = efi_pci_io_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_pci_io_protocol {
    pub poll_mem: *mut c_void,
    pub poll_io: *mut c_void,
    pub mem: efi_pci_io_protocol_access_t,
    pub io: efi_pci_io_protocol_access_t,
    pub pci: efi_pci_io_protocol_config_access_t,
    pub copy_mem: *mut c_void,
    pub map: *mut c_void,
    pub unmap: *mut c_void,
    pub allocate_buffer: *mut c_void,
    pub free_buffer: *mut c_void,
    pub flush: *mut c_void,
    pub func_nr): *mut c_ulong,
    pub attributes: *mut c_void,
    pub get_bar_attributes: *mut c_void,
    pub set_bar_attributes: *mut c_void,
    pub romsize: u64,
    pub romimage: *mut c_void,
}

pub const EFI_PCI_IO_ATTRIBUTE_ISA_MOTHERBOARD_IO: c_uint = 0x0001;
pub const EFI_PCI_IO_ATTRIBUTE_ISA_IO: c_uint = 0x0002;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_PALETTE_IO: c_uint = 0x0004;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_MEMORY: c_uint = 0x0008;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_IO: c_uint = 0x0010;
pub const EFI_PCI_IO_ATTRIBUTE_IDE_PRIMARY_IO: c_uint = 0x0020;
pub const EFI_PCI_IO_ATTRIBUTE_IDE_SECONDARY_IO: c_uint = 0x0040;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_WRITE_COMBINE: c_uint = 0x0080;
pub const EFI_PCI_IO_ATTRIBUTE_IO: c_uint = 0x0100;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY: c_uint = 0x0200;
pub const EFI_PCI_IO_ATTRIBUTE_BUS_MASTER: c_uint = 0x0400;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_CACHED: c_uint = 0x0800;
pub const EFI_PCI_IO_ATTRIBUTE_MEMORY_DISABLE: c_uint = 0x1000;
pub const EFI_PCI_IO_ATTRIBUTE_EMBEDDED_DEVICE: c_uint = 0x2000;
pub const EFI_PCI_IO_ATTRIBUTE_EMBEDDED_ROM: c_uint = 0x4000;
pub const EFI_PCI_IO_ATTRIBUTE_DUAL_ADDRESS_CYCLE: c_uint = 0x8000;
pub const EFI_PCI_IO_ATTRIBUTE_ISA_IO_16: c_uint = 0x10000;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_PALETTE_IO_16: c_uint = 0x20000;
pub const EFI_PCI_IO_ATTRIBUTE_VGA_IO_16: c_uint = 0x40000;
pub type apple_properties_protocol_t = apple_properties_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union apple_properties_protocol {
    pub version: c_ulong,
    pub ): *mut *mut *mut efi_char16_t , void , u32,
    pub u32): *mut *mut *mut efi_char16_t , void ,,
    pub ): *mut efi_char16_t,
    pub ): *mut *mut void buffer, u32,
}

pub type efi_tcg2_event_log_format = u32;
pub const INITRD_EVENT_TAG_ID: c_uint = 0x8F3B22ECU;
pub const LOAD_OPTIONS_EVENT_TAG_ID: c_uint = 0x8F3B22EDU;
pub const EV_EVENT_TAG: c_uint = 0x00000006U;
pub const EFI_TCG2_EVENT_HEADER_VERSION: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_tcg2_event {
    pub event_size: u32,
    pub header_size: u32,
    pub header_version: u16,
    pub pcr_index: u32,
    pub event_type: u32,
    pub event_header: } __packed,
// u8[] event follows here
    pub __packed: },
// from TCG PC Client Platform Firmware Profile Specification
    pub tagged_event_id: u32,
    pub tagged_event_data_size: u32,
    pub tagged_event_data: [u8; ],
    pub TCG_PCClientTaggedEvent: },
pub type efi_tcg2_event_t = efi_tcg2_event;
pub type efi_tcg2_protocol_t = efi_tcg2_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_tcg2_protocol {
    pub get_capability: *mut c_void,
    pub ): *mut efi_bool_t,
    pub ): *const efi_tcg2_event_t,
    pub submit_command: *mut c_void,
    pub get_active_pcr_banks: *mut c_void,
    pub set_active_pcr_banks: *mut c_void,
    pub get_result_of_set_active_pcr_banks: *mut c_void,
}

// EFI CC type/subtype defines
pub const EFI_CC_TYPE_NONE: c_int = 0;
pub const EFI_CC_TYPE_AMD_SEV: c_int = 1;
pub const EFI_CC_TYPE_INTEL_TDX: c_int = 2;
pub type efi_cc_mr_index_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_cc_event {
    pub event_size: u32,
    pub header_size: u32,
    pub header_version: u16,
    pub mr_index: u32,
    pub event_type: u32,
    pub event_header: } __packed,
// u8[] event follows here
    pub __packed: },
pub type efi_cc_event_t = efi_cc_event;
pub type efi_cc_event_log_bitmap_t = u32;
pub type efi_cc_event_log_format_t = u32;
pub type efi_cc_event_algorithm_bitmap_t = u32;
    pub size: u8,
    pub structure_version: efi_cc_version_t,
    pub protocol_version: efi_cc_version_t,
    pub hash_algorithm_bitmap: efi_cc_event_algorithm_bitmap_t,
    pub supported_event_logs: efi_cc_event_log_bitmap_t,
    pub cc_type: efi_cc_type_t,
    pub efi_cc_boot_service_cap_t: },
pub const EFI_CC_EVENT_HEADER_VERSION: c_int = 1;
pub const EFI_CC_BOOT_HASH_ALG_SHA384: c_uint = 0x00000004;
pub const EFI_CC_EVENT_LOG_FORMAT_TCG_2: c_uint = 0x00000002;
pub type efi_cc_protocol_t = efi_cc_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_cc_protocol {
    pub ): *mut efi_cc_boot_service_cap_t,
    pub ): *mut efi_bool_t,
    pub ): *const efi_cc_event_t,
    pub ): *mut efi_cc_mr_index_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_efi_boot_protocol {
    pub revision: u64,
    pub boot_hartid): *mut c_ulong,
}

pub type efi_load_file_protocol_t = efi_load_file_protocol;
pub type efi_load_file2_protocol_t = efi_load_file_protocol;
#[repr(C)]
#[derive(Copy, Clone)]
pub union efi_load_file_protocol {
    pub ): *mut *mut bool, unsigned long , void,
}

// efi_char16_t description[];
// efi_device_path_protocol_t file_path_list[];
// u8 optional_data[];
pub const EFI_LOAD_OPTION_ACTIVE: c_uint = 0x0001U;
pub const EFI_LOAD_OPTION_FORCE_RECONNECT: c_uint = 0x0002U;
pub const EFI_LOAD_OPTION_HIDDEN: c_uint = 0x0008U;
pub const EFI_LOAD_OPTION_CATEGORY: c_uint = 0x1f00U;
pub const EFI_LOAD_OPTION_CATEGORY_BOOT: c_uint = 0x0000U;
pub const EFI_LOAD_OPTION_CATEGORY_APP: c_uint = 0x0100U;

extern "C" {
    pub fn efi_pci_disable_bridge_busmaster();
}
extern "C" {
    pub fn efi_get_random_bytes(size: c_ulong, out: *mut u8) -> efi_status_t;
}
extern "C" {
    pub fn efi_random_get_seed() -> efi_status_t;
}
extern "C" {
    pub fn check_platform_features() -> efi_status_t;
}
// NOTE: These functions do not print a trailing newline after the string
extern "C" {
    pub fn efi_char16_puts(: *mut efi_char16_t);
}
extern "C" {
    pub fn efi_puts(str: *const c_char);
}
extern "C" {
    pub fn efi_free(size: c_ulong, addr: c_ulong);
}
extern "C" {
    pub fn efi_apply_loadoptions_quirk(load_options: *const c_void, load_options_size: *mut u32);
}
extern "C" {
    pub fn efi_parse_options(cmdline: *const c_char) -> efi_status_t;
}
extern "C" {
    pub fn efi_parse_option_graphics(option: *mut c_char);
}
extern "C" {
    pub fn efi_setup_graphics(si: *mut screen_info, edid: *mut edid_info) -> efi_status_t;
}
//
// This function handles the architcture specific differences between arm and
// arm64 regarding where the kernel image must be loaded and any memory that
// must be reserved. On failure it is required to free all
// all allocations it has made.
//
// shared entrypoint between the normal stub and the zboot stub
extern "C" {
    pub fn efi_handle_cmdline(image: *mut efi_loaded_image_t, cmdline_ptr: *mut c_char) -> efi_status_t;
}
extern "C" {
    pub fn efi_handle_post_ebs_state();
}
extern "C" {
    pub fn efi_get_secureboot() -> efi_secureboot_mode;
}

extern "C" {
    pub fn efi_enable_reset_attack_mitigation();
}

extern "C" {
    pub fn efi_retrieve_eventlog();
}
extern "C" {
    pub fn free_primary_display(dpy: *mut sysfb_display_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_smbios_record {
    pub type: u8,
    pub length: u8,
    pub handle: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_smbios_type1_record {
    pub header: efi_smbios_record,
    pub manufacturer: u8,
    pub product_name: u8,
    pub version: u8,
    pub serial_number: u8,
    pub uuid: efi_guid_t,
    pub wakeup_type: u8,
    pub sku_number: u8,
    pub family: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_smbios_type4_record {
    pub header: efi_smbios_record,
    pub socket: u8,
    pub processor_type: u8,
    pub processor_family: u8,
    pub processor_manufacturer: u8,
    pub processor_id: [u8; 8],
    pub processor_version: u8,
    pub voltage: u8,
    pub external_clock: u16,
    pub max_speed: u16,
    pub current_speed: u16,
    pub status: u8,
    pub processor_upgrade: u8,
    pub l1_cache_handle: u16,
    pub l2_cache_handle: u16,
    pub l3_cache_handle: u16,
    pub serial_number: u8,
    pub asset_tag: u8,
    pub part_number: u8,
    pub core_count: u8,
    pub enabled_core_count: u8,
    pub thread_count: u8,
    pub processor_characteristics: u16,
    pub processor_family2: u16,
    pub core_count2: u16,
    pub enabled_core_count2: u16,
    pub thread_count2: u16,
    pub thread_enabled: u16,
}

extern "C" {
    pub fn efi_kaslr_get_phys_seed(image_handle: efi_handle_t) -> u32;
}
extern "C" {
    pub fn process_unaccepted_memory(start: u64, end: u64);
}
extern "C" {
    pub fn accept_memory(start: phys_addr_t, size: c_ulong);
}
extern "C" {
    pub fn arch_accept_memory(start: phys_addr_t, end: phys_addr_t);
}
extern "C" {
    pub fn efi_zboot_decompress_init(alloc_size: *mut c_ulong) -> efi_status_t;
}
extern "C" {
    pub fn efi_zboot_decompress(out: *mut u8, outlen: c_ulong) -> efi_status_t;
}
