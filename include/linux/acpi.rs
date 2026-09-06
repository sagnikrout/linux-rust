//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/acpi.h
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
// acpi.h - ACPI Interface
//
// Copyright (C) 2001 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
//

// Macro flag: #define __init_or_acpilib
// Macro flag: #define __initdata_or_acpilib

// Macro flag: #define EXPORT_SYMBOL_ACPI_LIB(x)

extern "C" {
    pub fn is_acpi_device_node(_arg: dev->fwnode) -> return;
}
extern "C" {
    pub fn dev_name(_arg: &adev->dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_irq_model_id {
    ACPI_IRQ_MODEL_PIC = 0,
    ACPI_IRQ_MODEL_IOAPIC,
    ACPI_IRQ_MODEL_IOSAPIC,
    ACPI_IRQ_MODEL_PLATFORM,
    ACPI_IRQ_MODEL_GIC,
    ACPI_IRQ_MODEL_GIC_V5,
    ACPI_IRQ_MODEL_LPIC,
    ACPI_IRQ_MODEL_RINTC,
    ACPI_IRQ_MODEL_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_interrupt_id {
    ACPI_INTERRUPT_PMI	= 1,
    ACPI_INTERRUPT_INIT,
    ACPI_INTERRUPT_CPEI,
    ACPI_INTERRUPT_COUNT
}

pub const ACPI_SPACE_MEM: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_address_range_id {
    ACPI_ADDRESS_RANGE_MEMORY = 1,
    ACPI_ADDRESS_RANGE_RESERVED = 2,
    ACPI_ADDRESS_RANGE_ACPI = 3,
    ACPI_ADDRESS_RANGE_NVS	= 4,
    ACPI_ADDRESS_RANGE_COUNT
}

// Table Handlers
extern "C" {
    pub fn int(table: *mut *mut acpi_tbl_table_handler)(struct acpi_table_header) -> typedef;
}
// Debugger support
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_debugger_ops {
    pub context): *mut *mut int (create_thread)(acpi_osd_exec_callback function, void,
    pub msg): *const *const ssize_t (write_log)(char,
    pub length): *mut *mut *mut ssize_t (read_cmd)(char buffer, size_t,
    pub length): *mut *mut *mut int (wait_command_ready)(bool single_step, char buffer, size_t,
    pub (*notify_command_complete)(void): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_debugger {
    pub ops: *const acpi_debugger_ops,
    pub owner: *mut module,
    pub lock: mutex,
}

extern "C" {
    pub fn acpi_debugger_init() -> int __init;
}
extern "C" {
    pub fn acpi_unregister_debugger(ops: *const acpi_debugger_ops);
}
extern "C" {
    pub fn acpi_debugger_create_thread(function: acpi_osd_exec_callback, context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn acpi_debugger_write_log(msg: *const c_char) -> isize;
}
extern "C" {
    pub fn acpi_debugger_read_cmd(buffer: *mut c_char, buffer_length: usize) -> isize;
}
extern "C" {
    pub fn acpi_debugger_wait_command_ready() -> c_int;
}
extern "C" {
    pub fn acpi_debugger_notify_command_complete() -> c_int;
}

extern "C" {
    pub fn __acpi_unmap_table(map: *mut void __iomem, size: c_ulong);
}
extern "C" {
    pub fn early_acpi_boot_init() -> c_int;
}
extern "C" {
    pub fn acpi_boot_init() -> c_int;
}
extern "C" {
    pub fn acpi_boot_table_prepare();
}
extern "C" {
    pub fn acpi_boot_table_init();
}
extern "C" {
    pub fn acpi_mps_check() -> c_int;
}
extern "C" {
    pub fn acpi_numa_init() -> c_int;
}
extern "C" {
    pub fn acpi_locate_initial_tables() -> c_int;
}
extern "C" {
    pub fn acpi_reserve_initial_tables();
}
extern "C" {
    pub fn acpi_table_init_complete();
}
extern "C" {
    pub fn acpi_table_init() -> c_int;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn acpi_table_parse(id: *mut c_char, handler: acpi_tbl_table_handler) -> c_int;
}
extern "C" {
    pub fn acpi_parse_mcfg(header: *mut acpi_table_header) -> c_int;
}
extern "C" {
    pub fn acpi_table_print_madt_entry(madt: *mut acpi_subtable_header);
}

extern "C" {
    pub fn acpi_numa_processor_affinity_init(pa: *mut acpi_srat_cpu_affinity);
}

extern "C" {
    pub fn acpi_numa_x2apic_affinity_init(pa: *mut acpi_srat_x2apic_cpu_affinity);
}

extern "C" {
    pub fn acpi_arch_dma_setup(dev: *mut device);
}

extern "C" {
    pub fn acpi_numa_gicc_affinity_init(pa: *mut acpi_srat_gicc_affinity);
}

extern "C" {
    pub fn acpi_numa_rintc_affinity_init(pa: *mut acpi_srat_rintc_affinity);
}

pub type phys_cpuid_t = u32;

extern "C" {
    pub fn acpi_get_madt_revision() -> int __init;
}
// Validate the processor object's proc_id
extern "C" {
    pub fn acpi_duplicate_processor_id(proc_id: c_int) -> bool;
}
// Processor _CTS control

extern "C" {
    pub fn acpi_processor_claim_cst_control() -> bool;
}

// Arch dependent functions for cpu hotplug support
extern "C" {
    pub fn acpi_unmap_cpu(cpu: c_int) -> c_int;
}

extern "C" {
    pub fn acpi_get_processor_handle(cpu: c_int) -> acpi_handle;
}
//
// acpi_get_cpu_uid() - Get ACPI Processor UID of from MADT table
// @cpu: Logical CPU number (0-based)
// @uid: Pointer to store ACPI Processor UID
//
// Return: 0 on success (ACPI Processor ID stored in *uid);
// -EINVAL if CPU number is invalid or out of range;
// -ENODEV if ACPI Processor UID for the CPU is not found.
//
extern "C" {
    pub fn acpi_get_cpu_uid(cpu: c_uint, uid: *mut u32) -> c_int;
}

extern "C" {
    pub fn acpi_get_ioapic_id(handle: acpi_handle, gsi_base: u32, phys_addr: *mut u64) -> c_int;
}

extern "C" {
    pub fn acpi_register_ioapic(handle: acpi_handle, phys_addr: u64, gsi_base: u32) -> c_int;
}
extern "C" {
    pub fn acpi_unregister_ioapic(handle: acpi_handle, gsi_base: u32) -> c_int;
}
extern "C" {
    pub fn acpi_ioapic_registered(handle: acpi_handle, gsi_base: u32) -> c_int;
}
extern "C" {
    pub fn acpi_irq_stats_init();
}

extern "C" {
    pub fn acpi_register_gsi(dev: *mut device, gsi: u32, triggering: c_int, polarity: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_gsi_to_irq(gsi: u32, irq: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn acpi_isa_irq_to_gsi(isa_irq: unsigned, gsi: *mut u32) -> c_int;
}
extern "C" {
    pub fn acpi_handle(_arg: *mut acpi_gsi_handle_disp_fn)(u32) -> typedef;
}
extern "C" {
    pub fn acpi_get_gsi_dispatcher() -> acpi_gsi_domain_disp_fn;
}
extern "C" {
    pub fn acpi_set_gsi_to_irq_fallback((*)(u32): *mut u32);
}
extern "C" {
    pub fn acpi_irq_add_auto_dep(handle: acpi_handle) -> u32;
}

extern "C" {
    pub fn acpi_get_override_irq(gsi: u32, trigger: *mut c_int, polarity: *mut c_int) -> c_int;
}

//
// This function undoes the effect of one call to acpi_register_gsi().
// If this matches the last registration, any IRQ resources for gsi
// are freed.
//
extern "C" {
    pub fn acpi_unregister_gsi(gsi: u32);
}
extern "C" {
    pub fn acpi_pci_irq_enable(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn acpi_penalize_isa_irq(irq: c_int, active: c_int);
}
extern "C" {
    pub fn acpi_isa_irq_available(irq: c_int) -> bool;
}

extern "C" {
    pub fn acpi_penalize_sci_irq(irq: c_int, trigger: c_int, polarity: c_int);
}

extern "C" {
    pub fn acpi_pci_irq_disable(dev: *mut pci_dev);
}
extern "C" {
    pub fn ec_read(addr: u8, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn ec_write(addr: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn ec_get_handle() -> acpi_handle;
}
extern "C" {
    pub fn acpi_is_pnp_device(: *mut acpi_device) -> bool;
}

extern "C" {
    pub fn void(data: *mut *mut wmi_notify_handler) (union acpi_object, context: *mut c_void) -> typedef;
}
extern "C" {
    pub fn wmi_instance_count(guid: *const c_char) -> c_int;
}
extern "C" {
    pub fn wmi_remove_notify_handler(guid: *const c_char) -> acpi_status;
}
extern "C" {
    pub fn wmi_has_guid(guid: *const c_char) -> bool;
}

pub const ACPI_VIDEO_OUTPUT_SWITCHING: c_uint = 0x0001;
pub const ACPI_VIDEO_DEVICE_POSTING: c_uint = 0x0002;
pub const ACPI_VIDEO_ROM_AVAILABLE: c_uint = 0x0004;
pub const ACPI_VIDEO_BACKLIGHT: c_uint = 0x0008;
pub const ACPI_VIDEO_BACKLIGHT_FORCE_VENDOR: c_uint = 0x0010;
pub const ACPI_VIDEO_BACKLIGHT_FORCE_VIDEO: c_uint = 0x0020;
pub const ACPI_VIDEO_OUTPUT_SWITCHING_FORCE_VENDOR: c_uint = 0x0040;
pub const ACPI_VIDEO_OUTPUT_SWITCHING_FORCE_VIDEO: c_uint = 0x0080;
pub const ACPI_VIDEO_BACKLIGHT_DMI_VENDOR: c_uint = 0x0100;
pub const ACPI_VIDEO_BACKLIGHT_DMI_VIDEO: c_uint = 0x0200;
pub const ACPI_VIDEO_OUTPUT_SWITCHING_DMI_VENDOR: c_uint = 0x0400;
pub const ACPI_VIDEO_OUTPUT_SWITCHING_DMI_VIDEO: c_uint = 0x0800;
extern "C" {
    pub fn acpi_dev_is_video_device(adev: *mut acpi_device) -> bool;
}
extern "C" {
    pub fn acpi_is_video_device(handle: acpi_handle) -> c_long;
}
extern "C" {
    pub fn acpi_osi_setup(str: *mut c_char);
}
extern "C" {
    pub fn acpi_osi_is_win8() -> bool;
}

extern "C" {
    pub fn thermal_acpi_active_trip_temp(adev: *mut acpi_device, id: c_int, ret_temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn thermal_acpi_passive_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn thermal_acpi_hot_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn thermal_acpi_critical_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int;
}

extern "C" {
    pub fn acpi_get_genport_coordinates(uid: u32, coord: *mut access_coordinate) -> c_int;
}

extern "C" {
    pub fn acpi_map_pxm_to_node(pxm: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_get_node(handle: acpi_handle) -> c_int;
}
//
// pxm_to_online_node - Map proximity ID to online node
// @pxm: ACPI proximity ID
//
// This is similar to pxm_to_node(), but always returns an online
// node.  When the mapped node from a given proximity ID is offline, it
// looks up the node distance table and returns the nearest online node.
//
// ACPI device drivers, which are called after the NUMA initialization has
// completed in the kernel, can call this interface to obtain their device
// NUMA topology from ACPI tables.  Such drivers do not have to deal with
// offline nodes.  A node may be offline when SRAT memory entry does not exist,
// or NUMA is disabled, ex. "numa=off" on x86.
//
extern "C" {
    pub fn numa_map_to_online_node(_arg: node) -> return;
}

extern "C" {
    pub fn acpi_dev_resource_memory(ares: *mut acpi_resource, res: *mut resource) -> bool;
}
extern "C" {
    pub fn acpi_dev_resource_io(ares: *mut acpi_resource, res: *mut resource) -> bool;
}
extern "C" {
    pub fn acpi_dev_irq_flags(triggering: u8, polarity: u8, shareable: u8, wake_capable: u8) -> c_ulong;
}
extern "C" {
    pub fn acpi_dev_get_irq_type(triggering: c_int, polarity: c_int) -> c_uint;
}
extern "C" {
    pub fn acpi_dev_free_resource_list(list: *mut list_head);
}
extern "C" {
    pub fn acpi_dev_get_memory_resources(adev: *mut acpi_device, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn acpi_dev_filter_resource_type(_arg: ares, long)arg: (unsigned) -> return;
}
extern "C" {
    pub fn acpi_check_resource_conflict(res: *const resource) -> c_int;
}
extern "C" {
    pub fn acpi_resources_are_enforced() -> c_int;
}

extern "C" {
    pub fn acpi_old_suspend_ordering() -> void __init;
}
extern "C" {
    pub fn acpi_nvs_nosave() -> void __init;
}
extern "C" {
    pub fn acpi_nvs_nosave_s3() -> void __init;
}
extern "C" {
    pub fn acpi_sleep_no_blacklist() -> void __init;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_osc_context {
    pub /: *mut *mut *mut char uuid_str; / UUID string,
    pub rev: c_int,
    pub /: *mut *mut acpi_buffer cap; / list of DWORD capabilities,
    pub /: *mut *mut acpi_buffer ret; / free by caller if success,
}

extern "C" {
    pub fn acpi_run_osc(handle: acpi_handle, context: *mut acpi_osc_context) -> acpi_status;
}
// Number of _OSC capability DWORDS depends on bridge type
pub const OSC_PCI_CAPABILITY_DWORDS: c_int = 3;
pub const OSC_CXL_CAPABILITY_DWORDS: c_int = 5;
// Indexes into _OSC Capabilities Buffer (DWORDs 2 to 5 are device-specific)

// _OSC Capabilities DWORD 1: Query/Control and Error Returns (generic)
pub const OSC_QUERY_ENABLE: c_uint = 0x00000001  /* input */;
pub const OSC_REQUEST_ERROR: c_uint = 0x00000002  /* return */;
pub const OSC_INVALID_UUID_ERROR: c_uint = 0x00000004  /* return */;
pub const OSC_INVALID_REVISION_ERROR: c_uint = 0x00000008  /* return */;
pub const OSC_CAPABILITIES_MASK_ERROR: c_uint = 0x00000010  /* return */;
// Platform-Wide Capabilities _OSC: Capabilities DWORD 2: Support Field
pub const OSC_SB_PAD_SUPPORT: c_uint = 0x00000001;
pub const OSC_SB_PPC_OST_SUPPORT: c_uint = 0x00000002;
pub const OSC_SB_PR3_SUPPORT: c_uint = 0x00000004;
pub const OSC_SB_HOTPLUG_OST_SUPPORT: c_uint = 0x00000008;
pub const OSC_SB_APEI_SUPPORT: c_uint = 0x00000010;
pub const OSC_SB_CPC_SUPPORT: c_uint = 0x00000020;
pub const OSC_SB_CPCV2_SUPPORT: c_uint = 0x00000040;
pub const OSC_SB_PCLPI_SUPPORT: c_uint = 0x00000080;
pub const OSC_SB_OSLPI_SUPPORT: c_uint = 0x00000100;
pub const OSC_SB_FAST_THERMAL_SAMPLING_SUPPORT: c_uint = 0x00000200;
pub const OSC_SB_OVER_16_PSTATES_SUPPORT: c_uint = 0x00000400;
pub const OSC_SB_GED_SUPPORT: c_uint = 0x00000800;
pub const OSC_SB_CPC_DIVERSE_HIGH_SUPPORT: c_uint = 0x00001000;
pub const OSC_SB_IRQ_RESOURCE_SOURCE_SUPPORT: c_uint = 0x00002000;
pub const OSC_SB_CPC_FLEXIBLE_ADR_SPACE: c_uint = 0x00004000;
pub const OSC_SB_GENERIC_INITIATOR_SUPPORT: c_uint = 0x00020000;
pub const OSC_SB_NATIVE_USB4_SUPPORT: c_uint = 0x00040000;
pub const OSC_SB_BATTERY_CHARGE_LIMITING_SUPPORT: c_uint = 0x00080000;
pub const OSC_SB_PRM_SUPPORT: c_uint = 0x00200000;
pub const OSC_SB_FFH_OPR_SUPPORT: c_uint = 0x00400000;
// USB4 Capabilities
pub const OSC_USB_USB3_TUNNELING: c_uint = 0x00000001;
pub const OSC_USB_DP_TUNNELING: c_uint = 0x00000002;
pub const OSC_USB_PCIE_TUNNELING: c_uint = 0x00000004;
pub const OSC_USB_XDOMAIN: c_uint = 0x00000008;
// PCI Host Bridge _OSC: Capabilities DWORD 2: Support Field
pub const OSC_PCI_EXT_CONFIG_SUPPORT: c_uint = 0x00000001;
pub const OSC_PCI_ASPM_SUPPORT: c_uint = 0x00000002;
pub const OSC_PCI_CLOCK_PM_SUPPORT: c_uint = 0x00000004;
pub const OSC_PCI_SEGMENT_GROUPS_SUPPORT: c_uint = 0x00000008;
pub const OSC_PCI_MSI_SUPPORT: c_uint = 0x00000010;
pub const OSC_PCI_EDR_SUPPORT: c_uint = 0x00000080;
pub const OSC_PCI_HPX_TYPE_3_SUPPORT: c_uint = 0x00000100;
// PCI Host Bridge _OSC: Capabilities DWORD 3: Control Field
pub const OSC_PCI_EXPRESS_NATIVE_HP_CONTROL: c_uint = 0x00000001;
pub const OSC_PCI_SHPC_NATIVE_HP_CONTROL: c_uint = 0x00000002;
pub const OSC_PCI_EXPRESS_PME_CONTROL: c_uint = 0x00000004;
pub const OSC_PCI_EXPRESS_AER_CONTROL: c_uint = 0x00000008;
pub const OSC_PCI_EXPRESS_CAPABILITY_CONTROL: c_uint = 0x00000010;
pub const OSC_PCI_EXPRESS_LTR_CONTROL: c_uint = 0x00000020;
pub const OSC_PCI_EXPRESS_DPC_CONTROL: c_uint = 0x00000080;
// CXL _OSC: Capabilities DWORD 4: Support Field
pub const OSC_CXL_1_1_PORT_REG_ACCESS_SUPPORT: c_uint = 0x00000001;
pub const OSC_CXL_2_0_PORT_DEV_REG_ACCESS_SUPPORT: c_uint = 0x00000002;
pub const OSC_CXL_PROTOCOL_ERR_REPORTING_SUPPORT: c_uint = 0x00000004;
pub const OSC_CXL_NATIVE_HP_SUPPORT: c_uint = 0x00000008;
// CXL _OSC: Capabilities DWORD 5: Control Field
pub const OSC_CXL_ERROR_REPORTING_CONTROL: c_uint = 0x00000001;
pub const ACPI_GSB_ACCESS_ATTRIB_QUICK: c_uint = 0x00000002;
pub const ACPI_GSB_ACCESS_ATTRIB_SEND_RCV: c_uint = 0x00000004;
pub const ACPI_GSB_ACCESS_ATTRIB_BYTE: c_uint = 0x00000006;
pub const ACPI_GSB_ACCESS_ATTRIB_WORD: c_uint = 0x00000008;
pub const ACPI_GSB_ACCESS_ATTRIB_BLOCK: c_uint = 0x0000000A;
pub const ACPI_GSB_ACCESS_ATTRIB_MULTIBYTE: c_uint = 0x0000000B;
pub const ACPI_GSB_ACCESS_ATTRIB_WORD_CALL: c_uint = 0x0000000C;
pub const ACPI_GSB_ACCESS_ATTRIB_BLOCK_CALL: c_uint = 0x0000000D;
pub const ACPI_GSB_ACCESS_ATTRIB_RAW_BYTES: c_uint = 0x0000000E;
pub const ACPI_GSB_ACCESS_ATTRIB_RAW_PROCESS: c_uint = 0x0000000F;
// Enable _OST when all relevant hotplug operations are enabled

// Macro flag: #define ACPI_HOTPLUG_OST

// _OST Source Event Code (OSPM Action)
pub const ACPI_OST_EC_OSPM_SHUTDOWN: c_uint = 0x100;
pub const ACPI_OST_EC_OSPM_EJECT: c_uint = 0x103;
pub const ACPI_OST_EC_OSPM_INSERTION: c_uint = 0x200;
// _OST General Processing Status Code
pub const ACPI_OST_SC_SUCCESS: c_uint = 0x0;
pub const ACPI_OST_SC_NON_SPECIFIC_FAILURE: c_uint = 0x1;
pub const ACPI_OST_SC_UNRECOGNIZED_NOTIFY: c_uint = 0x2;
// _OST OS Shutdown Processing (0x100) Status Code
pub const ACPI_OST_SC_OS_SHUTDOWN_DENIED: c_uint = 0x80;
pub const ACPI_OST_SC_OS_SHUTDOWN_IN_PROGRESS: c_uint = 0x81;
pub const ACPI_OST_SC_OS_SHUTDOWN_COMPLETED: c_uint = 0x82;
pub const ACPI_OST_SC_OS_SHUTDOWN_NOT_SUPPORTED: c_uint = 0x83;
// _OST Ejection Request (0x3, 0x103) Status Code
pub const ACPI_OST_SC_EJECT_NOT_SUPPORTED: c_uint = 0x80;
pub const ACPI_OST_SC_DEVICE_IN_USE: c_uint = 0x81;
pub const ACPI_OST_SC_DEVICE_BUSY: c_uint = 0x82;
pub const ACPI_OST_SC_EJECT_DEPENDENCY_BUSY: c_uint = 0x83;
pub const ACPI_OST_SC_EJECT_IN_PROGRESS: c_uint = 0x84;
// _OST Insertion Request (0x200) Status Code
pub const ACPI_OST_SC_INSERT_IN_PROGRESS: c_uint = 0x80;
pub const ACPI_OST_SC_DRIVER_LOAD_FAILURE: c_uint = 0x81;
pub const ACPI_OST_SC_INSERT_NOT_SUPPORTED: c_uint = 0x82;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_predicate {
    all_versions,
    less_than_or_equal,
    equal,
    greater_than_or_equal,
}

// Table must be terminted by a NULL entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_platform_list {
    pub oem_id: [c_char; ACPI_OEM_ID_SIZE+1],
    pub oem_table_id: [c_char; ACPI_OEM_TABLE_ID_SIZE+1],
    pub oem_revision: u32,
    pub table: *mut c_char,
    pub pred: acpi_predicate,
    pub reason: *mut c_char,
    pub data: u32,
}

extern "C" {
    pub fn acpi_match_platform_list(plat: *const acpi_platform_list) -> c_int;
}
extern "C" {
    pub fn acpi_early_init();
}
extern "C" {
    pub fn acpi_subsystem_init();
}
extern "C" {
    pub fn acpi_nvs_register(start: __u64, size: __u64) -> c_int;
}
extern "C" {
    pub fn acpi_device_uevent_modalias(: *const device, : *mut kobj_uevent_env) -> c_int;
}
extern "C" {
    pub fn acpi_device_modalias(: *mut device, : *mut c_char, _arg: c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_reconfig_event {
    ACPI_RECONFIG_DEVICE_ADD = 0,
    ACPI_RECONFIG_DEVICE_REMOVE,
}

extern "C" {
    pub fn acpi_reconfig_notifier_register(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn acpi_reconfig_notifier_unregister(nb: *mut notifier_block) -> c_int;
}

extern "C" {
    pub fn acpi_gtdt_init(table: *mut acpi_table_header, platform_timer_count: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_gtdt_map_ppi(type: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_gtdt_c3stop(type: c_int) -> bool;
}

extern "C" {
    pub fn acpi_get_local_u64_address(handle: acpi_handle, addr: *mut u64) -> c_int;
}
extern "C" {
    pub fn acpi_get_local_address(handle: acpi_handle, addr: *mut u32) -> c_int;
}

extern "C" {
    pub fn acpi_mrrm_max_mem_region() -> c_int;
}

pub const acpi_disabled: c_int = 1;

// Get rid of the -Wunused-variable for adev

// uid = cpu;

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn arch_post_acpi_subsys_init();
}

extern "C" {
    pub fn acpi_ioapic_add(root: acpi_handle) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_s2idle_dev_ops {
    pub list_node: list_head,
    pub (*prepare)(void): *mut c_void,
    pub (*check)(void): *mut c_void,
    pub (*restore)(void): *mut c_void,
}

extern "C" {
    pub fn acpi_register_lps0_dev(arg: *mut acpi_s2idle_dev_ops) -> c_int;
}
extern "C" {
    pub fn acpi_unregister_lps0_dev(arg: *mut acpi_s2idle_dev_ops);
}

extern "C" {
    pub fn arch_reserve_mem_area(addr: acpi_physical_address, size: usize);
}

extern "C" {
    pub fn acpi_dev_suspend(dev: *mut device, wakeup: bool) -> c_int;
}
extern "C" {
    pub fn acpi_dev_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_dev_pm_attach(dev: *mut device, power_on: bool) -> c_int;
}
extern "C" {
    pub fn acpi_storage_d3(dev: *mut device) -> bool;
}
extern "C" {
    pub fn acpi_dev_state_d0(dev: *mut device) -> bool;
}

extern "C" {
    pub fn acpi_subsys_prepare(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_complete(dev: *mut device);
}
extern "C" {
    pub fn acpi_subsys_suspend_late(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_suspend_noirq(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_freeze(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_poweroff(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn acpi_subsys_restore_early(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn acpi_ec_mark_gpe_for_wake();
}
extern "C" {
    pub fn acpi_ec_set_gpe_wake_mask(action: u8);
}

extern "C" {
    pub fn __acpi_handle_debug(descriptor: *mut _ddebug, handle: acpi_handle, fmt: *const c_char, ...);
}

//
// acpi_handle_<level>: Print message with ACPI prefix and object path
//
// These interfaces acquire the global namespace mutex to obtain an object
// path.  In interrupt context, it shows the object path as <n/a>.
//

extern "C" {
    pub fn acpi_dev_gpio_irq_wake_get_by(_arg: adev, _arg: NULL, _arg: index, _arg: wake_capable) -> return;
}
extern "C" {
    pub fn acpi_dev_gpio_irq_wake_get_by(_arg: adev, _arg: con_id, _arg: index, _arg: NULL) -> return;
}
extern "C" {
    pub fn acpi_dev_gpio_irq_wake_get_by(_arg: adev, _arg: NULL, _arg: index, _arg: NULL) -> return;
}
// Device properties

pub const ACPI_TABLE_ID_LEN: c_int = 5;
//
// struct acpi_probe_entry - boot-time probing entry
// @id:			ACPI table name
// @type:		Optional subtable type to match
// (if @id contains subtables)
// @subtable_valid:	Optional callback to check the validity of
// the subtable
// @probe_table:	Callback to the driver being probed when table
// match is successful
// @probe_subtbl:	Callback to the driver being probed when table and
// subtable match (and optional callback is successful)
// @driver_data:	Sideband data provided back to the driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_probe_entry {
    pub id: [__u8; ACPI_TABLE_ID_LEN],
    pub type: __u8,
    pub subtable_valid: acpi_probe_entry_validate_subtbl,
    pub probe_table: acpi_tbl_table_handler,
    pub probe_subtbl: acpi_tbl_entry_handler,
}

extern "C" {
    pub fn arch_sort_irqchip_probe(ap_head: *mut acpi_probe_entry, nr: c_int);
}

extern "C" {
    pub fn __acpi_probe_device_table(start: *mut acpi_probe_entry, nr: c_int) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENXIO) -> return;
}

extern "C" {
    pub fn acpi_table_upgrade();
}

extern "C" {
    pub fn acpi_has_watchdog() -> bool;
}

extern "C" {
    pub fn acpi_parse_spcr(enable_earlycon: bool, enable_console: bool) -> c_int;
}

extern "C" {
    pub fn acpi_irq_get(handle: acpi_handle, index: c_uint, res: *mut resource) -> c_int;
}

extern "C" {
    pub fn lpit_read_residency_count_address(address: *mut u64) -> c_int;
}

extern "C" {
    pub fn acpi_pptt_cpu_is_thread(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn find_acpi_cpu_topology(cpu: c_uint, level: c_int) -> c_int;
}
extern "C" {
    pub fn find_acpi_cpu_topology_cluster(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn find_acpi_cpu_topology_package(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn find_acpi_cpu_topology_hetero_id(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn acpi_pptt_get_cpus_from_container(acpi_cpu_id: u32, cpus: *mut cpumask_t);
}
extern "C" {
    pub fn find_acpi_cache_level_from_id(cache_id: u32) -> c_int;
}
extern "C" {
    pub fn acpi_pptt_get_cpumask_from_cache_id(cache_id: u32, cpus: *mut cpumask_t) -> c_int;
}

extern "C" {
    pub fn acpi_arch_init();
}

extern "C" {
    pub fn acpi_init_pcc();
}

extern "C" {
    pub fn acpi_init_ffh();
}

extern "C" {
    pub fn acpi_device_notify(dev: *mut device);
}
extern "C" {
    pub fn acpi_device_notify_remove(dev: *mut device);
}

extern "C" {
    pub fn acpi_node_backed_by_real_pxm(nid: c_int) -> bool;
}

