//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/platform.h
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
// platform.h
//
// Hardware platform operations. Intended for use by domain-0 kernel.
//
// Copyright (c) 2002-2006, K Fraser
//

pub const XENPF_INTERFACE_VERSION: c_uint = 0x03000001;
//
// Set clock such that it would read <secs,nsecs> after 00:00:00 UTC,
// 1 January, 1970 if the current system time was <system_time>.
//
pub const XENPF_settime32: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_settime32 {
// IN variables.
    pub secs: u32,
    pub nsecs: u32,
    pub system_time: u64,
}

pub const XENPF_settime64: c_int = 62;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_settime64 {
// IN variables.
    pub secs: u64,
    pub nsecs: u32,
    pub mbz: u32,
    pub system_time: u64,
}

//
// Request memory range (@mfn, @mfn+@nr_mfns-1) to have type @type.
// On x86, @type is an architecture-defined MTRR memory type.
// On success, returns the MTRR that was used (@reg) and a handle that can
// be passed to XENPF_DEL_MEMTYPE to accurately tear down the new setting.
// (x86-specific).
//
pub const XENPF_add_memtype: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_add_memtype {
// IN variables.
    pub mfn: xen_pfn_t,
    pub nr_mfns: u64,
    pub type: u32,
// OUT variables.
    pub handle: u32,
    pub reg: u32,
}

//
// Tear down an existing memory-range type. If @handle is remembered then it
// should be passed in to accurately tear down the correct setting (in case
// of overlapping memory regions with differing types). If it is not known
// then @handle should be set to zero. In all cases @reg must be set.
// (x86-specific).
//
pub const XENPF_del_memtype: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_del_memtype {
// IN variables.
    pub handle: u32,
    pub reg: u32,
}

// Read current type of an MTRR (x86-specific).
pub const XENPF_read_memtype: c_int = 33;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_read_memtype {
// IN variables.
    pub reg: u32,
// OUT variables.
    pub mfn: xen_pfn_t,
    pub nr_mfns: u64,
    pub type: u32,
}

pub const XENPF_microcode_update: c_int = 35;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_microcode_update {
// IN variables.
    pub /: *mut *mut GUEST_HANDLE(void) data; / Pointer to microcode data,
    pub /: *mut *mut uint32_t length; / Length of microcode data.,
}

pub const XENPF_platform_quirk: c_int = 39;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_platform_quirk {
// IN variables.
    pub quirk_id: u32,
}

pub const XENPF_efi_runtime_call: c_int = 49;
pub const XEN_EFI_get_time: c_int = 1;
pub const XEN_EFI_set_time: c_int = 2;
pub const XEN_EFI_get_wakeup_time: c_int = 3;
pub const XEN_EFI_set_wakeup_time: c_int = 4;
pub const XEN_EFI_get_next_high_monotonic_count: c_int = 5;
pub const XEN_EFI_get_variable: c_int = 6;
pub const XEN_EFI_set_variable: c_int = 7;
pub const XEN_EFI_get_next_variable_name: c_int = 8;
pub const XEN_EFI_query_variable_info: c_int = 9;
pub const XEN_EFI_query_capsule_capabilities: c_int = 10;
pub const XEN_EFI_update_capsule: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_efi_runtime_call {
    pub function: u32,
//
// This field is generally used for per sub-function flags (defined
// below), except for the XEN_EFI_get_next_high_monotonic_count case,
// where it holds the single returned value.
//
    pub misc: u32,
    pub status: xen_ulong_t,
pub const XEN_EFI_GET_TIME_SET_CLEARS_NS: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_efi_time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub ns: u32,
    pub tz: i16,
    pub daylight: u8,
    pub time: },
    pub resolution: u32,
    pub accuracy: u32,
    pub get_time: },
    pub set_time: xenpf_efi_time,
pub const XEN_EFI_GET_WAKEUP_TIME_ENABLED: c_uint = 0x00000001;
pub const XEN_EFI_GET_WAKEUP_TIME_PENDING: c_uint = 0x00000002;
    pub get_wakeup_time: xenpf_efi_time,
pub const XEN_EFI_SET_WAKEUP_TIME_ENABLE: c_uint = 0x00000001;
pub const XEN_EFI_SET_WAKEUP_TIME_ENABLE_ONLY: c_uint = 0x00000002;
    pub set_wakeup_time: xenpf_efi_time,
pub const XEN_EFI_VARIABLE_NON_VOLATILE: c_uint = 0x00000001;
pub const XEN_EFI_VARIABLE_BOOTSERVICE_ACCESS: c_uint = 0x00000002;
pub const XEN_EFI_VARIABLE_RUNTIME_ACCESS: c_uint = 0x00000004;
    pub /: *mut *mut GUEST_HANDLE(void) name; / UCS-2/UTF-16 string,
    pub size: xen_ulong_t,
    pub data: GUEST_HANDLE(void),
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_efi_guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
    pub vendor_guid: },
    pub set_variable: } get_variable,,
    pub size: xen_ulong_t,
    pub /: *mut *mut GUEST_HANDLE(void) name; / UCS-2/UTF-16 string,
    pub vendor_guid: xenpf_efi_guid,
    pub get_next_variable_name: },
    pub attr: u32,
    pub max_store_size: u64,
    pub remain_store_size: u64,
    pub max_size: u64,
    pub query_variable_info: },
    pub capsule_header_array: GUEST_HANDLE(void),
    pub capsule_count: xen_ulong_t,
    pub max_capsule_size: u64,
    pub reset_type: u32,
    pub query_capsule_capabilities: },
    pub capsule_header_array: GUEST_HANDLE(void),
    pub capsule_count: xen_ulong_t,
    pub /: *mut *mut uint64_t sg_list; / machine address,
    pub update_capsule: },
    pub u: },
}

pub const XEN_FW_EFI_VERSION: c_int = 0;
pub const XEN_FW_EFI_CONFIG_TABLE: c_int = 1;
pub const XEN_FW_EFI_VENDOR: c_int = 2;
pub const XEN_FW_EFI_MEM_INFO: c_int = 3;
pub const XEN_FW_EFI_RT_VERSION: c_int = 4;
pub const XENPF_firmware_info: c_int = 50;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_firmware_info {
// IN variables.
    pub type: u32,
    pub index: u32,
// OUT variables.
// Int13, Fn48: Check Extensions Present.
    pub /: *mut *mut uint8_t device; / %dl: bios device number,
    pub /: *mut *mut uint8_t version; / %ah: major version,
    pub /: *mut *mut uint16_t interface_support; / %cx: support bitmap,
// Int13, Fn08: Legacy Get Device Parameters.
    pub /: *mut *mut uint16_t legacy_max_cylinder; / %cl[7:6]:%ch: max cyl #,
    pub /: *mut *mut uint8_t legacy_max_head; / %dh: max head #,
    pub /: *mut *mut uint8_t legacy_sectors_per_track; / %cl[5:0]: max sector #,
// Int13, Fn41: Get Device Parameters (as filled into %ds:%esi).
// NB. First uint16_t of buffer must be set to buffer size.
    pub edd_params: GUEST_HANDLE(void),
    pub /: *mut *mut } disk_info; / XEN_FW_DISK_INFO,
    pub /: *mut *mut uint8_t device; / bios device number,
    pub /: *mut *mut uint32_t mbr_signature; / offset 0x1b8 in mbr,
    pub /: *mut *mut } disk_mbr_signature; / XEN_FW_DISK_MBR_SIGNATURE,
// Int10, AX=4F15: Get EDID info.
    pub capabilities: u8,
    pub edid_transfer_time: u8,
// must refer to 128-byte buffer
    pub edid: GUEST_HANDLE(uchar),
    pub /: *mut *mut } vbeddc_info; / XEN_FW_VBEDDC_INFO,
#[repr(C)]
#[derive(Copy, Clone)]
pub union xenpf_efi_info {
    pub version: u32,
    pub /: *mut *mut uint64_t addr; / EFI_CONFIGURATION_TABLE,
    pub nent: u32,
    pub cfg: },
    pub revision: u32,
    pub /: *mut *mut uint32_t bufsz; / input, in bytes,
    pub name: GUEST_HANDLE(void),
// UCS-2/UTF-16 string
    pub vendor: },
    pub addr: u64,
    pub size: u64,
    pub attr: u64,
    pub type: u32,
    pub mem: },
    pub /: *mut *mut } efi_info; / XEN_FW_EFI_INFO,
    pub /: *mut *mut uint8_t kbd_shift_flags; / XEN_FW_KBD_SHIFT_FLAGS,
    pub u: },
}

pub const XENPF_enter_acpi_sleep: c_int = 51;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_enter_acpi_sleep {
// IN variables
    pub /: *mut *mut uint16_t val_a; / PM1a control / sleep type A.,
    pub /: *mut *mut uint16_t val_b; / PM1b control / sleep type B.,
    pub /: *mut *mut uint32_t sleep_state; / Which state to enter (Sn).,
pub const XENPF_ACPI_SLEEP_EXTENDED: c_uint = 0x00000001;
    pub /: *mut *mut *mut uint32_t flags; / XENPF_ACPI_SLEEP_.,
}

pub const XENPF_change_freq: c_int = 52;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_change_freq {
// IN variables
    pub /: *mut *mut uint32_t flags; / Must be zero.,
    pub /: *mut *mut uint32_t cpu; / Physical cpu.,
    pub /: *mut *mut uint64_t freq; / New frequency (Hz).,
}

//
// Get idle times (nanoseconds since boot) for physical CPUs specified in the
// @cpumap_bitmap with range [0..@cpumap_nr_cpus-1]. The @idletime array is
// indexed by CPU number; only entries with the corresponding @cpumap_bitmap
// bit set are written to. On return, @cpumap_bitmap is modified so that any
// non-existent CPUs are cleared. Such CPUs have their @idletime array entry
// cleared.
//
pub const XENPF_getidletime: c_int = 53;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_getidletime {
// IN/OUT variables
// IN: CPUs to interrogate; OUT: subset of IN which are present
    pub cpumap_bitmap: GUEST_HANDLE(uchar),
// IN variables
// Size of cpumap bitmap.
    pub cpumap_nr_cpus: u32,
// Must be indexable for every cpu in cpumap_bitmap.
    pub idletime: GUEST_HANDLE(uint64_t),
// OUT variables
// System time when the idletime snapshots were taken.
    pub now: u64,
}

pub const XENPF_set_processor_pminfo: c_int = 54;
// ability bits
pub const XEN_PROCESSOR_PM_CX: c_int = 1;
pub const XEN_PROCESSOR_PM_PX: c_int = 2;
pub const XEN_PROCESSOR_PM_TX: c_int = 4;
// cmd type
pub const XEN_PM_CX: c_int = 0;
pub const XEN_PM_PX: c_int = 1;
pub const XEN_PM_TX: c_int = 2;
pub const XEN_PM_PDC: c_int = 3;
// Px sub info type
pub const XEN_PX_PCT: c_int = 1;
pub const XEN_PX_PSS: c_int = 2;
pub const XEN_PX_PPC: c_int = 4;
pub const XEN_PX_PSD: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_power_register {
    pub space_id: u32,
    pub bit_width: u32,
    pub bit_offset: u32,
    pub access_size: u32,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_processor_csd {
    pub /: *mut *mut uint32_t domain; / domain number of one dependent group,
    pub /: *mut *mut uint32_t coord_type; / coordination type,
    pub /: *mut *mut uint32_t num; / number of processors in same domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_processor_cx {
    pub /: *mut *mut xen_power_register reg; / GAS for Cx trigger register,
    pub /: *mut *mut uint8_t type; / cstate value, c0: 0, c1: 1, ...,
    pub /: *mut *mut uint32_t latency; / worst latency (ms) to enter/exit this cstate,
    pub /: *mut *mut uint32_t power; / average power consumption(mW),
    pub /: *mut *mut uint32_t dpcnt; / number of dependency entries,
    pub /: *mut *mut GUEST_HANDLE(xen_processor_csd) dp; / NULL if no dependency,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_processor_flags {
    pub bm_control:1: u32,
    pub bm_check:1: u32,
    pub has_cst:1: u32,
    pub power_setup_done:1: u32,
    pub bm_rld_set:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_processor_power {
    pub /: *mut *mut uint32_t count; / number of C state entries in array below,
    pub /: *mut *mut xen_processor_flags flags; / global flags of this processor,
    pub /: *mut *mut GUEST_HANDLE(xen_processor_cx) states; / supported c states,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pct_register {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub reserved: u8,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_processor_px {
    pub /: *mut *mut uint64_t core_frequency; / megahertz,
    pub /: *mut *mut uint64_t power; / milliWatts,
    pub /: *mut *mut uint64_t transition_latency; / microseconds,
    pub /: *mut *mut uint64_t bus_master_latency; / microseconds,
    pub /: *mut *mut uint64_t control; / control value,
    pub /: *mut *mut uint64_t status; / success indicator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_psd_package {
    pub num_entries: u64,
    pub revision: u64,
    pub domain: u64,
    pub coord_type: u64,
    pub num_processors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_processor_performance {
    pub /: *mut *mut uint32_t flags; / flag for Px sub info type,
    pub /: *mut *mut uint32_t platform_limit; / Platform limitation on freq usage,
    pub control_register: xen_pct_register,
    pub status_register: xen_pct_register,
    pub /: *mut *mut uint32_t state_count; / total available performance states,
    pub states: GUEST_HANDLE(xen_processor_px),
    pub domain_info: xen_psd_package,
    pub /: *mut *mut uint32_t shared_type; / coordination type of this processor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_set_processor_pminfo {
// IN variables
    pub /: *mut *mut uint32_t id; / ACPI CPU ID,
    pub /: *mut *mut uint32_t type; / {XEN_PM_CX, XEN_PM_PX},
    pub /: *mut *mut xen_processor_power power;/ Cx: _CST/_CSD,
    pub /: *mut *mut xen_processor_performance perf; / Px: _PPC/_PCT/_PSS/_PSD,
    pub pdc: GUEST_HANDLE(uint32_t),
}

pub const XENPF_get_cpuinfo: c_int = 55;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_pcpuinfo {
// IN
    pub xen_cpuid: u32,
// OUT
// The maxium cpu_id that is present
    pub max_present: u32,
pub const XEN_PCPU_FLAGS_ONLINE: c_int = 1;
// Correponding xen_cpuid is not present
pub const XEN_PCPU_FLAGS_INVALID: c_int = 2;
    pub flags: u32,
    pub apic_id: u32,
    pub acpi_id: u32,
}

pub const XENPF_cpu_online: c_int = 56;
pub const XENPF_cpu_offline: c_int = 57;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_cpu_ol {
    pub cpuid: u32,
}

pub const XENPF_cpu_hotadd: c_int = 58;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_cpu_hotadd {
    pub apic_id: u32,
    pub acpi_id: u32,
    pub pxm: u32,
}

pub const XENPF_mem_hotadd: c_int = 59;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_mem_hotadd {
    pub spfn: u64,
    pub epfn: u64,
    pub pxm: u32,
    pub flags: u32,
}

pub const XENPF_core_parking: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_core_parking {
// IN variables
pub const XEN_CORE_PARKING_SET: c_int = 1;
pub const XEN_CORE_PARKING_GET: c_int = 2;
    pub type: u32,
// IN variables:  set cpu nums expected to be idled
// OUT variables: get cpu nums actually be idled
    pub idle_nums: u32,
}

pub const XENPF_get_symbol: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenpf_symdata {
// IN/OUT variables
    pub /: *mut *mut uint32_t namelen; / size of 'name' buffer,
// IN/OUT variables
    pub /: *mut *mut uint32_t symnum; / IN: Symbol to read,
// OUT: Next available symbol. If same as IN
// then  we reached the end
// OUT variables
    pub name: GUEST_HANDLE(char),
    pub address: u64,
    pub type: c_char,
}

pub const XENPF_get_dom0_console: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_platform_op {
    pub cmd: u32,
    pub /: *mut *mut uint32_t interface_version; / XENPF_INTERFACE_VERSION,
    pub settime32: xenpf_settime32,
    pub settime64: xenpf_settime64,
    pub add_memtype: xenpf_add_memtype,
    pub del_memtype: xenpf_del_memtype,
    pub read_memtype: xenpf_read_memtype,
    pub microcode: xenpf_microcode_update,
    pub platform_quirk: xenpf_platform_quirk,
    pub efi_runtime_call: xenpf_efi_runtime_call,
    pub firmware_info: xenpf_firmware_info,
    pub enter_acpi_sleep: xenpf_enter_acpi_sleep,
    pub change_freq: xenpf_change_freq,
    pub getidletime: xenpf_getidletime,
    pub set_pminfo: xenpf_set_processor_pminfo,
    pub pcpu_info: xenpf_pcpuinfo,
    pub cpu_ol: xenpf_cpu_ol,
    pub cpu_add: xenpf_cpu_hotadd,
    pub mem_add: xenpf_mem_hotadd,
    pub core_parking: xenpf_core_parking,
    pub symdata: xenpf_symdata,
    pub dom0_console: dom0_vga_console_info,
    pub pad: [u8; 128],
    pub u: },
}
