//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/pci.h
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

// Number of possible devfns: 0.0 to 1f.7 inclusive
pub const MAX_NR_DEVFNS: c_int = 256;
pub const PCI_MAX_NR_DEVS: c_int = 32;
pub const MAX_NR_LANES: c_int = 16;
pub const PCI_FIND_CAP_TTL: c_int = 48;
pub const PCI_VSEC_ID_INTEL_TBT: c_uint = 0x1234	/* Thunderbolt */;
pub const PCIE_LINK_RETRAIN_TIMEOUT_MS: c_int = 1000;
//
// Power stable to PERST# inactive.
//
// See the "Power Sequencing and Reset Signal Timings" table of the PCI Express
// Card Electromechanical Specification, Revision 5.1, Section 2.9.2, Symbol
// "T_PVPERL".
//
pub const PCIE_T_PVPERL_MS: c_int = 100;
//
// REFCLK stable before PERST# inactive.
//
// See the "Power Sequencing and Reset Signal Timings" table of the PCI Express
// Card Electromechanical Specification, Revision 5.1, Section 2.9.2, Symbol
// "T_PERST-CLK".
//
pub const PCIE_T_PERST_CLK_US: c_int = 100;
//
// PCIe r6.0, sec 5.3.3.2.1 <PME Synchronization>
// Recommends 1ms to 10ms timeout to check L2 ready.
//
pub const PCIE_PME_TO_L2_TIMEOUT_US: c_int = 10000;
//
// PCIe r6.0, sec 6.6.1 <Conventional Reset>
//
// - "With a Downstream Port that does not support Link speeds greater
// than 5.0 GT/s, software must wait a minimum of 100 ms following exit
// from a Conventional Reset before sending a Configuration Request to
// the device immediately below that Port."
//
// - "With a Downstream Port that supports Link speeds greater than
// 5.0 GT/s, software must wait a minimum of 100 ms after Link training
// completes before sending a Configuration Request to the device
// immediately below that Port."
//
pub const PCIE_RESET_CONFIG_WAIT_MS: c_int = 100;
// Parameters for the waiting for link up routine
pub const PCIE_LINK_WAIT_MAX_RETRIES: c_int = 10;
pub const PCIE_LINK_WAIT_SLEEP_MS: c_int = 90;
// Format of TLP; PCIe r7.0, sec 2.2.1
pub const PCIE_TLP_FMT_3DW_NO_DATA: c_uint = 0x00 /* 3DW header, no data */;
pub const PCIE_TLP_FMT_4DW_NO_DATA: c_uint = 0x01 /* 4DW header, no data */;
pub const PCIE_TLP_FMT_3DW_DATA: c_uint = 0x02 /* 3DW header, with data */;
pub const PCIE_TLP_FMT_4DW_DATA: c_uint = 0x03 /* 4DW header, with data */;
// Type of TLP; PCIe r7.0, sec 2.2.1
pub const PCIE_TLP_TYPE_MEM_RDWR: c_uint = 0x00 /* Memory Read/Write Request */;
pub const PCIE_TLP_TYPE_IO_RDWR: c_uint = 0x02 /* I/O Read/Write Request */;
pub const PCIE_TLP_TYPE_CFG0_RDWR: c_uint = 0x04 /* Config Type 0 Read/Write Request */;
pub const PCIE_TLP_TYPE_CFG1_RDWR: c_uint = 0x05 /* Config Type 1 Read/Write Request */;
pub const PCIE_TLP_TYPE_MSG: c_uint = 0x10 /* Message With/Without data Request */;
// Message Routing (r[2:0]); PCIe r6.0, sec 2.2.8
pub const PCIE_MSG_TYPE_R_RC: c_int = 0;
pub const PCIE_MSG_TYPE_R_ADDR: c_int = 1;
pub const PCIE_MSG_TYPE_R_ID: c_int = 2;
pub const PCIE_MSG_TYPE_R_BC: c_int = 3;
pub const PCIE_MSG_TYPE_R_LOCAL: c_int = 4;
pub const PCIE_MSG_TYPE_R_GATHER: c_int = 5;
// Power Management Messages; PCIe r6.0, sec 2.2.8.2
pub const PCIE_MSG_CODE_PME_TURN_OFF: c_uint = 0x19;
// INTx Mechanism Messages; PCIe r6.0, sec 2.2.8.1
pub const PCIE_MSG_CODE_ASSERT_INTA: c_uint = 0x20;
pub const PCIE_MSG_CODE_ASSERT_INTB: c_uint = 0x21;
pub const PCIE_MSG_CODE_ASSERT_INTC: c_uint = 0x22;
pub const PCIE_MSG_CODE_ASSERT_INTD: c_uint = 0x23;
pub const PCIE_MSG_CODE_DEASSERT_INTA: c_uint = 0x24;
pub const PCIE_MSG_CODE_DEASSERT_INTB: c_uint = 0x25;
pub const PCIE_MSG_CODE_DEASSERT_INTC: c_uint = 0x26;
pub const PCIE_MSG_CODE_DEASSERT_INTD: c_uint = 0x27;
// Cpl. status of Complete; PCIe r7.0, sec 2.2.9.1
pub const PCIE_CPL_STS_SUCCESS: c_uint = 0x00 /* Successful Completion */;
pub const PCI_BUS_BRIDGE_IO_WINDOW: c_int = 0;
pub const PCI_BUS_BRIDGE_MEM_WINDOW: c_int = 1;
pub const PCI_BUS_BRIDGE_PREF_MEM_WINDOW: c_int = 2;

extern "C" {
    pub fn pcie_get_link_speed(speed: c_uint) -> c_uchar;
}
extern "C" {
    pub fn pcie_cap_has_lnkctl(dev: *const pci_dev) -> bool;
}
extern "C" {
    pub fn pcie_cap_has_lnkctl2(dev: *const pci_dev) -> bool;
}
extern "C" {
    pub fn pcie_cap_has_rtctl(dev: *const pci_dev) -> bool;
}
// Standard Capability finder
//
// PCI_FIND_NEXT_CAP - Find a PCI standard capability
// @read_cfg: Function pointer for reading PCI config space
// @start: Starting position to begin search
// @cap: Capability ID to find
// @prev_ptr: Pointer to store position of preceding capability (optional)
// @args: Arguments to pass to read_cfg function
//
// Search the capability list in PCI config space to find @cap. If
// found, update *prev_ptr with the position of the preceding capability
// (if prev_ptr != NULL)
// Implements TTL (time-to-live) protection against infinite loops.
//
// Return: Position of the capability if found, 0 otherwise.
//

// (u8 *)prev_ptr = __prev_pos;		\
// Extended Capability finder
//
// PCI_FIND_NEXT_EXT_CAP - Find a PCI extended capability
// @read_cfg: Function pointer for reading PCI config space
// @start: Starting position to begin search (0 for initial search)
// @cap: Extended capability ID to find
// @prev_ptr: Pointer to store position of preceding capability (optional)
// @args: Arguments to pass to read_cfg function
//
// Search the extended capability list in PCI config space to find @cap.
// If found, update *prev_ptr with the position of the preceding capability
// (if prev_ptr != NULL)
// Implements TTL protection against infinite loops using a calculated
// maximum search count.
//
// Return: Position of the capability if found, 0 otherwise.
//

// (u16 *)prev_ptr = __prev_pos;		\
// Functions internal to the PCI core code

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_mmap_api {
    PCI_MMAP_SYSFS,	/* mmap on /sys/bus/pci/devices/<BDF>/resource<N> */
    PCI_MMAP_PROCFS	/* mmap on /proc/bus/pci/<BDF> */
}

extern "C" {
    pub fn pci_reset_supported(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_init_reset_methods(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_bridge_secondary_bus_reset(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_bus_error_reset(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_try_reset_bridge(bridge: *mut pci_dev) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_cap_saved_data {
    pub cap_nr: u16,
    pub cap_extended: bool,
    pub size: c_uint,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_cap_saved_state {
    pub next: hlist_node,
    pub cap: pci_cap_saved_data,
}

extern "C" {
    pub fn pci_allocate_cap_save_buffers(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_free_cap_save_buffers(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_add_cap_save_buffer(dev: *mut pci_dev, cap: c_char, size: c_uint) -> c_int;
}

extern "C" {
    pub fn pci_update_current_state(dev: *mut pci_dev, state: pci_power_t);
}
extern "C" {
    pub fn pci_refresh_power_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_power_up(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_disable_enabled_device(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_finish_runtime_suspend(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcie_clear_device_status(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_clear_root_pme_status(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_check_pme_status(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_pme_wakeup_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_pme_restore(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_dev_need_resume(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_dev_adjust_pme(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_dev_complete_resume(pci_dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_config_pm_runtime_get(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_config_pm_runtime_put(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_pm_power_up_and_verify_state(pci_dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_pm_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_ea_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_ea_fixed_busnrs(dev: *mut pci_dev, sec: *mut u8, sub: *mut u8) -> bool;
}
extern "C" {
    pub fn pci_msi_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_msix_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_bridge_d3_possible(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_bridge_d3_update(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_bridge_wait_for_secondary_bus(dev: *mut pci_dev, reset_type: *mut c_char) -> c_int;
}
extern "C" {
    pub fn platform_pci_configure_wake(dev: *mut pci_dev);
}
extern "C" {
    pub fn platform_pci_remove_wake(dev: *mut pci_dev);
}
// Wait 100 ms before the system can be put into a sleep state.
//
// pci_bar_index_is_valid - Check whether a BAR index is within valid range
// @bar: BAR index
//
// Protects against overflowing &struct pci_dev.resource array.
//
// Return: true for valid index, false otherwise.
//
// Currently we allow normal PCI devices and PCI bridges transition
// into D3 if their bridge_d3 is set.
//
extern "C" {
    pub fn pci_vpd_init(dev: *mut pci_dev);
}
// PCI Virtual Channel
extern "C" {
    pub fn pci_save_vc_state(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_restore_vc_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_allocate_vc_save_buffers(dev: *mut pci_dev);
}
// PCI /proc functions

extern "C" {
    pub fn pci_proc_attach_device(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_proc_detach_device(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_proc_detach_bus(bus: *mut pci_bus) -> c_int;
}

// Functions for PCI Hotplug drivers to use
extern "C" {
    pub fn pci_hp_add_bridge(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_hp_spurious_link_change(pdev: *mut pci_dev) -> bool;
}
// Lock for read/write access to pci device and bus lists

extern "C" {
    pub fn pci_no_msi();
}

extern "C" {
    pub fn pci_realloc_get_opt(: *mut c_char);
}

extern "C" {
    pub fn pci_legacy_has_sparse(bus: *mut pci_bus, type: pci_mmap_state) -> bool;
}

extern "C" {
    pub fn pci_cardbus_resource_alignment(res: *const resource) -> c_ulong;
}
extern "C" {
    pub fn pci_setup_cardbus(str: *mut c_char) -> c_int;
}

//
// pci_id_from_device - Obtain a pci_device_id from a PCI device
// @dev: the PCI device
//
// Return: a pci_device_id filled.
//
// pci_match_one_id - Tell if a PCI device ID matches a needle PCI device ID
// @id: single PCI device id structure to match against (needle)
// @dev_id: the actual ID from the PCI device
//
// ID can be retrieved from device using pci_id_from_device().
//
// Return: the matching pci_device_id structure or %NULL if there is no match.
//
// PCI slot sysfs helper code

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_slot_attribute {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct pci_slot , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct pci_slot , char ,,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_bar_type {
    pci_bar_unknown,	/* Standard PCI BAR probe */
    pci_bar_io,		/* An I/O port BAR */
    pci_bar_mem32,		/* A 32-bit memory BAR */
    pci_bar_mem64,		/* A 64-bit memory BAR */
}

extern "C" {
    pub fn pci_put_host_bridge_device(dev: *mut device);
}
extern "C" {
    pub fn pci_resize_resource_set_size(dev: *mut pci_dev, resno: c_int, size: c_int);
}
extern "C" {
    pub fn pci_rescan_bus_bridge_resize(bridge: *mut pci_dev) -> c_uint;
}
extern "C" {
    pub fn pci_reassign_resource(dev: *mut pci_dev, i: c_int, add_size: resource_size_t, align: resource_size_t) -> int __must_check;
}
extern "C" {
    pub fn pci_configure_extended_tags(dev: *mut pci_dev, ign: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pci_setup_device(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_configure_ari(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_bus_clip_resource(dev: *mut pci_dev, idx: c_int) -> bool;
}
extern "C" {
    pub fn pci_resource_is_optional(dev: *const pci_dev, resno: c_int) -> bool;
}
//
// pci_resource_num - Reverse lookup resource number from device resources
// @dev: PCI device
// @res: Resource to lookup index for (MUST be a @dev's resource)
//
// Perform reverse lookup to determine the resource number for @res within
// @dev resource array. NOTE: The caller is responsible for ensuring @res is
// among @dev's resources!
//
// Returns: resource number.
//
// Passing a resource that is not among dev's resources?
extern "C" {
    pub fn pbus_validate_busn(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_reassigndev_resource_alignment(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_disable_bridge_window(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_bus_put(bus: *mut pci_bus);
}

// PCIe link information from Link Capabilities 2

// PCIe speed to Mb/s reduced by encoding overhead

extern "C" {
    pub fn pcie_get_supported_speeds(dev: *mut pci_dev) -> u8;
}
extern "C" {
    pub fn __pcie_print_link_status(dev: *mut pci_dev, verbose: bool);
}
extern "C" {
    pub fn pcie_report_downtraining(dev: *mut pci_dev);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcie_link_change_reason {
    PCIE_LINK_RETRAIN,
    PCIE_ADD_BUS,
    PCIE_BWCTRL_ENABLE,
    PCIE_BWCTRL_IRQ,
    PCIE_HOTPLUG,
}

extern "C" {
    pub fn pcie_update_link_speed(bus: *mut pci_bus, reason: pcie_link_change_reason);
}
// Single Root I/O Virtualization
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_sriov {
    pub /: *mut *mut int pos; / Capability position,
    pub /: *mut *mut int nres; / Number of resources,
    pub /: *mut *mut u32 cap; / SR-IOV Capabilities,
    pub /: *mut *mut u16 ctrl; / SR-IOV Control,
    pub /: *mut *mut u16 total_VFs; / Total VFs associated with the PF,
    pub /: *mut *mut u16 initial_VFs; / Initial VFs associated with the PF,
    pub /: *mut *mut u16 num_VFs; / Number of VFs available,
    pub /: *mut *mut u16 offset; / First VF Routing ID offset,
    pub /: *mut *mut u16 stride; / Following VF stride,
    pub /: *mut *mut u16 vf_device; / VF device ID,
    pub /: *mut *mut u32 pgsz; / Page size for BAR alignment,
    pub /: *mut *mut u8 link; / Function Dependency Link,
    pub /: *mut *mut u8 max_VF_buses; / Max buses consumed by VFs,
    pub /: *mut *mut u16 driver_max_VFs; / Max num VFs driver supports,
    pub /: *mut *mut *mut pci_dev dev; / Lowest numbered PF,
    pub /: *mut *mut *mut pci_dev self; / This PF,
    pub /: *mut *mut u32 class; / VF device,
    pub /: *mut *mut u8 hdr_type; / VF header type,
    pub /: *mut *mut u16 subsystem_vendor; / VF subsystem vendor,
    pub /: *mut *mut u16 subsystem_device; / VF subsystem device,
    pub /: *mut *mut resource_size_t barsz[PCI_SRIOV_NUM_BARS]; / VF BAR size,
    pub /: *mut *mut u16 vf_rebar_cap; / VF Resizable BAR capability offset,
    pub /: *mut *mut bool drivers_autoprobe; / Auto probing of VFs by driver,
}

extern "C" {
    pub fn pci_doe_init(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_doe_destroy(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_doe_disconnected(pdev: *mut pci_dev);
}

extern "C" {
    pub fn pci_npem_create(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_npem_remove(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_doe_sysfs_init(pci_dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_doe_sysfs_teardown(pdev: *mut pci_dev);
}

extern "C" {
    pub fn pci_ide_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_ide_init_host_bridge(hb: *mut pci_host_bridge);
}
extern "C" {
    pub fn pci_ide_destroy(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_tsm_init(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_tsm_destroy(pdev: *mut pci_dev);
}

//
// pci_dev_set_io_state - Set the new error state if possible.
//
// @dev: PCI device to set new error_state
// @new: the state we want dev to be in
//
// If the device is experiencing perm_failure, it has to remain in that state.
// Any other transition is allowed.
//
// Returns true if state has been changed to the requested state.
//
// pci_dev priv_flags
pub const PCI_DEV_ADDED: c_int = 0;
pub const PCI_DPC_RECOVERED: c_int = 1;
pub const PCI_DPC_RECOVERING: c_int = 2;
pub const PCI_DEV_REMOVED: c_int = 3;
pub const PCI_LINK_CHANGED: c_int = 4;
pub const PCI_LINK_CHANGING: c_int = 5;
pub const PCI_LINK_LBMS_SEEN: c_int = 6;
pub const PCI_DEV_ALLOW_BINDING: c_int = 7;
extern "C" {
    pub fn test_and_clear_bit(_arg: PCI_DEV_ADDED, _arg: &dev->priv_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: PCI_DEV_ADDED, _arg: &dev->priv_flags) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: PCI_DEV_REMOVED, _arg: &dev->priv_flags) -> return;
}

//
// struct aer_err_info - AER Error Information
// @dev: Devices reporting error
// @ratelimit_print: Flag to log or not log the devices' error. 0=NotLog/1=Log
// @__pad1: Padding for alignment
// @error_dev_num: Number of devices reporting an error
// @level: printk level to use in logging
// @id: Value from register PCI_ERR_ROOT_ERR_SRC
// @severity: AER severity, 0-UNCOR Non-fatal, 1-UNCOR fatal, 2-COR
// @root_ratelimit_print: Flag to log or not log the root's error. 0=NotLog/1=Log
// @multi_error_valid: If multiple errors are reported
// @first_error: First reported error
// @__pad2: Padding for alignment
// @is_cxl: Bus type error: 0-PCI Bus error, 1-CXL Bus error
// @tlp_header_valid: Indicates if TLP field contains error information
// @status: COR/UNCOR error status
// @mask: COR/UNCOR mask
// @anfe_status: Advisory Non-Fatal Errors, i.e. Uncorrectable Errors signaled
// as Correctable Errors (PCIe r7.0 sec 6.2.4.3).  Only used if @severity
// is AER_CORRECTABLE and @status has Advisory Non-Fatal Error Status set.
// @tlp: Transaction packet information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aer_err_info {
    pub dev: [*mut pci_dev; AER_MAX_MULTI_ERR_DEVICES],
    pub ratelimit_print: [c_int; AER_MAX_MULTI_ERR_DEVICES],
    pub error_dev_num: c_int,
    pub level: *const c_char,
    pub id:16: c_uint,
    pub severity:2: c_uint,
    pub root_ratelimit_print:1: c_uint,
    pub __pad1:4: c_uint,
    pub multi_error_valid:1: c_uint,
    pub first_error:5: c_uint,
    pub __pad2:1: c_uint,
    pub is_cxl:1: c_uint,
    pub tlp_header_valid:1: c_uint,
    pub status: c_uint,
    pub mask: c_uint,
    pub anfe_status: u32,
    pub tlp: pcie_tlp_log,
}

extern "C" {
    pub fn aer_get_device_error_info(info: *mut aer_err_info, i: c_int) -> c_int;
}
extern "C" {
    pub fn aer_print_error(info: *mut aer_err_info, i: c_int);
}
extern "C" {
    pub fn aer_tlp_log_len(dev: *mut pci_dev, aercc: u32) -> c_uint;
}

// Cached RCEC Endpoint Association
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcec_ea {
    pub nextbusn: u8,
    pub lastbusn: u8,
    pub bitmap: u32,
}

extern "C" {
    pub fn pci_save_dpc_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_dpc_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_dpc_init(pdev: *mut pci_dev);
}
extern "C" {
    pub fn dpc_process_error(pdev: *mut pci_dev);
}
extern "C" {
    pub fn dpc_reset_link(pdev: *mut pci_dev) -> pci_ers_result_t;
}
extern "C" {
    pub fn pci_dpc_recovered(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn dpc_tlp_log_len(dev: *mut pci_dev) -> c_uint;
}

extern "C" {
    pub fn pci_rcec_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_rcec_exit(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_link_rcec(rcec: *mut pci_dev);
}

// Address Translation Service
extern "C" {
    pub fn pci_ats_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_ats_state(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_pri_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_pri_state(pdev: *mut pci_dev);
}

extern "C" {
    pub fn pci_pasid_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_pasid_state(pdev: *mut pci_dev);
}

extern "C" {
    pub fn pci_iov_init(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_iov_release(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_iov_remove(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_iov_update_resource(dev: *mut pci_dev, resno: c_int);
}
extern "C" {
    pub fn pci_restore_iov_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_iov_bus_range(bus: *mut pci_bus) -> c_int;
}
extern "C" {
    pub fn pci_iov_resource_set_size(dev: *mut pci_dev, resno: c_int, size: c_int);
}
extern "C" {
    pub fn pci_iov_is_memory_decoding_enabled(dev: *mut pci_dev) -> bool;
}

extern "C" {
    pub fn pci_restore_tph_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_save_tph_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_no_tph();
}
extern "C" {
    pub fn pci_tph_init(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_ptm_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_save_ptm_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_ptm_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_suspend_ptm(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_resume_ptm(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_acs_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_enable_acs(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_dev_specific_acs_enabled(dev: *mut pci_dev, acs_flags: u16) -> c_int;
}
extern "C" {
    pub fn pci_dev_specific_enable_acs(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_dev_specific_disable_acs_redir(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_disable_broken_acs_cap(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_failed_link_retrain(dev: *mut pci_dev) -> c_int;
}

// PCI error reporting and recovery
extern "C" {
    pub fn pcie_wait_for_link(pdev: *mut pci_dev, active: bool) -> bool;
}
extern "C" {
    pub fn pcie_retrain_link(pdev: *mut pci_dev, use_lt: bool) -> c_int;
}
// ASPM-related functionality we need even without CONFIG_PCIEASPM
extern "C" {
    pub fn pci_save_ltr_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_ltr_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_configure_aspm_l1ss(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_save_aspm_l1ss_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_aspm_l1ss_state(dev: *mut pci_dev);
}

extern "C" {
    pub fn pcie_aspm_remove_cap(pdev: *mut pci_dev, lnkcap: u32);
}
extern "C" {
    pub fn pcie_aspm_init_link_state(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_aspm_exit_link_state(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_aspm_pm_state_change(pdev: *mut pci_dev, locked: bool);
}
extern "C" {
    pub fn pcie_aspm_powersave_config_link(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_configure_ltr(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_bridge_reconfigure_ltr(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_encode_t_power_on(t_power_on_us: u32, scale: *mut u8, value: *mut u8);
}

// scale = 0;
// value = 0;

extern "C" {
    pub fn pcie_set_ecrc_checking(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_ecrc_get_policy(str: *mut c_char);
}

extern "C" {
    pub fn pcie_reset_lbms(port: *mut pci_dev);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dev_reset_methods {
    pub vendor: u16,
    pub device: u16,
    pub probe): *mut *mut *mut int (reset)(struct pci_dev dev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_reset_fn_method {
    pub probe): *mut *mut *mut int (reset_fn)(struct pci_dev pdev, bool,
    pub name: *mut c_char,
}

extern "C" {
    pub fn pci_dev_specific_reset(dev: *mut pci_dev, probe: bool) -> c_int;
}

extern "C" {
    pub fn pci_dev_specific_ats_required(dev: *mut pci_dev) -> bool;
}

extern "C" {
    pub fn pci_rebar_init(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_rebar_state(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_rebar_get_current_size(pdev: *mut pci_dev, bar: c_int) -> c_int;
}
extern "C" {
    pub fn pci_rebar_set_size(pdev: *mut pci_dev, bar: c_int, size: c_int) -> c_int;
}
pub const PCI_EQ_RESV: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum equalization_preset_type {
    EQ_PRESET_TYPE_8GTS,
    EQ_PRESET_TYPE_16GTS,
    EQ_PRESET_TYPE_32GTS,
    EQ_PRESET_TYPE_64GTS,
    EQ_PRESET_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_eq_presets {
    pub eq_presets_8gts: [u16; MAX_NR_LANES],
    pub 1][MAX_NR_LANES]: u8 eq_presets_Ngts[EQ_PRESET_TYPE_MAX -,
}

extern "C" {
    pub fn of_get_pci_domain_nr(node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_pci_get_max_link_speed(node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn of_pci_preserve_config(node: *mut device_node) -> bool;
}
extern "C" {
    pub fn pci_set_of_node(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_release_of_node(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_set_bus_of_node(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_release_bus_of_node(bus: *mut pci_bus);
}
extern "C" {
    pub fn devm_of_pci_bridge_init(dev: *mut device, bridge: *mut pci_host_bridge) -> c_int;
}
extern "C" {
    pub fn of_pci_supply_present(np: *mut device_node) -> bool;
}

// slot_power_limit_value = 0;
// slot_power_limit_scale = 0;

extern "C" {
    pub fn of_pci_make_dev_node(pdev: *mut pci_dev);
}
extern "C" {
    pub fn of_pci_remove_node(pdev: *mut pci_dev);
}
extern "C" {
    pub fn of_pci_make_host_bridge_node(bridge: *mut pci_host_bridge);
}
extern "C" {
    pub fn of_pci_remove_host_bridge_node(bridge: *mut pci_host_bridge);
}

extern "C" {
    pub fn pci_no_aer();
}
extern "C" {
    pub fn pci_aer_init(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_aer_exit(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_aer_clear_fatal_status(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_aer_clear_status(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_aer_raw_clear_status(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_save_aer_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_aer_state(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_acpi_preserve_config(bridge: *mut pci_host_bridge) -> bool;
}
extern "C" {
    pub fn pci_acpi_program_hp_params(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_set_acpi_fwnode(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_dev_acpi_reset(dev: *mut pci_dev, probe: bool) -> c_int;
}
extern "C" {
    pub fn acpi_pci_power_manageable(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn acpi_pci_bridge_d3(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn acpi_pci_set_power_state(dev: *mut pci_dev, state: pci_power_t) -> c_int;
}
extern "C" {
    pub fn acpi_pci_get_power_state(dev: *mut pci_dev) -> pci_power_t;
}
extern "C" {
    pub fn acpi_pci_refresh_power_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn acpi_pci_wakeup(dev: *mut pci_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn acpi_pci_need_resume(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn acpi_pci_choose_state(pdev: *mut pci_dev) -> pci_power_t;
}

extern "C" {
    pub fn pci_use_mid_pm() -> bool;
}
extern "C" {
    pub fn mid_pci_set_power_state(pdev: *mut pci_dev, state: pci_power_t) -> c_int;
}
extern "C" {
    pub fn mid_pci_get_power_state(pdev: *mut pci_dev) -> pci_power_t;
}

extern "C" {
    pub fn pci_msix_write_tph_tag(pdev: *mut pci_dev, index: c_uint, tag: u16) -> c_int;
}

//
// Config Address for PCI Configuration Mechanism #1
//
// See PCI Local Bus Specification, Revision 3.0,
// Section 3.2.2.3.2, Figure 3-2, p. 50.
//

pub const PCI_CONF1_BUS_MASK: c_uint = 0xff;
pub const PCI_CONF1_DEV_MASK: c_uint = 0x1f;
pub const PCI_CONF1_FUNC_MASK: c_uint = 0x7;
pub const PCI_CONF1_REG_MASK: c_uint = 0xfc /* Limit aligned offset to a maximum of 256B */;

//
// Extension of PCI Config Address for accessing extended PCIe registers
//
// No standardized specification, but used on lot of non-ECAM-compliant ARM SoCs
// or on AMD Barcelona and new CPUs. Reserved bits [27:24] of PCI Config Address
// are used for specifying additional 4 high bits of PCI Express register.
//
pub const PCI_CONF1_EXT_REG_SHIFT: c_int = 16;
pub const PCI_CONF1_EXT_REG_MASK: c_uint = 0xf00;

