//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/eeh.h
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
// Copyright (C) 2001  Dave Engebretsen & Todd Inglett IBM Corporation.
// Copyright 2001-2012 IBM Corporation.
//

// EEH subsystem flags
pub const EEH_ENABLED: c_uint = 0x01	/* EEH enabled			     */;
pub const EEH_FORCE_DISABLED: c_uint = 0x02	/* EEH disabled			     */;
pub const EEH_PROBE_MODE_DEV: c_uint = 0x04	/* From PCI device		     */;
pub const EEH_PROBE_MODE_DEVTREE: c_uint = 0x08	/* From device tree		     */;
pub const EEH_ENABLE_IO_FOR_LOG: c_uint = 0x20	/* Enable IO for log		     */;
pub const EEH_EARLY_DUMP_LOG: c_uint = 0x40	/* Dump log immediately		     */;
//
// Delay for PE reset, all in ms
//
// PCI specification has reset hold time of 100 milliseconds.
// We have 250 milliseconds here. The PCI bus settlement time
// is specified as 1.5 seconds and we have 1.8 seconds.
//
pub const EEH_PE_RST_HOLD_TIME: c_int = 250;
pub const EEH_PE_RST_SETTLE_TIME: c_int = 1800;
//
// The struct is used to trace PE related EEH functionality.
// In theory, there will have one instance of the struct to
// be created against particular PE. In nature, PEs correlate
// to each other. the struct has to reflect that hierarchy in
// order to easily pick up those affected PEs when one particular
// PE has EEH errors.
//
// Also, one particular PE might be composed of PCI device, PCI
// bus and its subordinate components. The struct also need ship
// the information. Further more, one particular PE is only meaingful
// in the corresponding PHB. Therefore, the root PEs should be created
// against existing PHBs in on-to-one fashion.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeh_pe {
    pub /: *mut *mut int type; / PE type: PHB/Bus/Device,
    pub /: *mut *mut int state; / PE EEH dependent mode,
    pub /: *mut *mut int addr; / PE configuration address,
    pub /: *mut *mut *mut pci_controller phb; / Associated PHB,
    pub /: *mut *mut *mut pci_bus bus; / Top PCI bus for bus PE,
    pub /: *mut *mut int check_count; / Times of ignored error,
    pub /: *mut *mut int freeze_count; / Times of froze up,
    pub /: *mut *mut time64_t tstamp; / Time on first-time freeze,
    pub /: *mut *mut int false_positives; / Times of reported #ff's,
    pub /: *mut *mut atomic_t pass_dev_cnt; / Count of passed through devs,
    pub /: *mut *mut *mut eeh_pe parent; / Parent PE,
    pub /: *mut *mut *mut void data; / PE auxiliary data,
    pub /: *mut *mut list_head child_list; / List of PEs below this PE,
    pub /: *mut *mut list_head child; / Memb. child_list/eeh_phb_pe,
    pub /: *mut *mut list_head edevs; / List of eeh_dev in this PE,

//
// Saved stack trace. When we find a PE freeze in eeh_dev_check_failure
// the stack trace is saved here so we can print it in the recovery
// thread if it turns out to due to a real problem rather than
// a hot-remove.
//
// A max of 64 entries might be overkill, but it also might not be.
//
    pub stack_trace: [c_ulong; 64],
    pub trace_entries: c_int,

}

//
// The struct is used to trace EEH state for the associated
// PCI device node or PCI device. In future, it might
// represent PE as well so that the EEH device to form
// another tree except the currently existing tree of PCI
// buses and PCI devices
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeh_dev {
    pub /: *mut *mut int mode; / EEH mode,
    pub /: *mut *mut int bdfn; / bdfn of device (for cfg ops),
    pub controller: *mut pci_controller,
    pub /: *mut *mut int pe_config_addr; / PE config address,
    pub /: *mut *mut u32 config_space[16]; / Saved PCI config space,
    pub /: *mut *mut int pcix_cap; / Saved PCIx capability,
    pub /: *mut *mut int pcie_cap; / Saved PCIe capability,
    pub /: *mut *mut int aer_cap; / Saved AER capability,
    pub /: *mut *mut int af_cap; / Saved AF capability,
    pub /: *mut *mut *mut eeh_pe pe; / Associated PE,
    pub /: *mut *mut list_head entry; / Membership in eeh_pe.edevs,
    pub /: *mut *mut list_head rmv_entry; / Membership in rmv_list,
    pub /: *mut *mut *mut pci_dn pdn; / Associated PCI device node,
    pub /: *mut *mut *mut pci_dev pdev; / Associated PCI device,
    pub /: *mut *mut bool in_error; / Error flag for edev,
// VF specific properties
    pub /: *mut *mut *mut pci_dev physfn; / Associated SRIOV PF,
    pub /: *mut *mut int vf_index; / Index of this VF,
}

// "fmt" must be a simple literal string

// Return values from eeh_ops::next_error
//
// The struct is used to trace the registered EEH operation
// callback functions. Actually, those operation callback
// functions are heavily platform dependent. That means the
// platform should register its own EEH operation callback
// functions before any EEH further operations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeh_ops {
    pub name: *mut c_char,
    pub pdev): *mut *mut *mut eeh_dev (probe)(pci_dev,
    pub option): *mut *mut *mut int (set_option)(struct eeh_pe pe, int,
    pub delay): *mut *mut *mut int (get_state)(struct eeh_pe pe, int,
    pub option): *mut *mut *mut int (reset)(struct eeh_pe pe, int,
    pub len): *mut *mut *mut *mut int (get_log)(struct eeh_pe pe, int severity, char drv_log, unsigned long,
    pub pe): *mut *mut int (configure_bridge)(struct eeh_pe,
    pub mask): unsigned long addr, unsigned long,
    pub val): *mut *mut *mut int (read_config)(struct eeh_dev edev, int where, int size, u32,
    pub val): *mut *mut *mut int (write_config)(struct eeh_dev edev, int where, int size, u32,
    pub pe): *mut *mut int (next_error)(struct eeh_pe,
    pub edev): *mut *mut int (restore_config)(struct eeh_dev,
    pub edev): *mut *mut int (notify_resume)(struct eeh_dev,
}

extern "C" {
    pub fn eeh_has_flag(!eeh_has_flag(EEH_FORCE_DISABLED: EEH_ENABLED) &&) -> return;
}
extern "C" {
    pub fn void(edev: *mut *mut eeh_edev_traverse_func)(struct eeh_dev, flag: *mut c_void) -> typedef;
}
extern "C" {
    pub fn eeh_set_pe_aux_size(size: c_int);
}
extern "C" {
    pub fn eeh_phb_pe_create(phb: *mut pci_controller) -> c_int;
}
extern "C" {
    pub fn eeh_wait_state(pe: *mut eeh_pe, max_wait: c_int) -> c_int;
}
extern "C" {
    pub fn eeh_pe_tree_insert(edev: *mut eeh_dev, new_pe_parent: *mut eeh_pe) -> c_int;
}
extern "C" {
    pub fn eeh_pe_tree_remove(edev: *mut eeh_dev) -> c_int;
}
extern "C" {
    pub fn eeh_pe_update_time_stamp(pe: *mut eeh_pe);
}
extern "C" {
    pub fn eeh_pe_restore_bars(pe: *mut eeh_pe);
}
extern "C" {
    pub fn eeh_show_enabled();
}
extern "C" {
    pub fn eeh_init(ops: *mut eeh_ops) -> int __init;
}
extern "C" {
    pub fn eeh_check_failure(token: *const volatile void __iomem) -> c_int;
}
extern "C" {
    pub fn eeh_dev_check_failure(edev: *mut eeh_dev) -> c_int;
}
extern "C" {
    pub fn eeh_addr_cache_init();
}
extern "C" {
    pub fn eeh_probe_device(pdev: *mut pci_dev);
}
extern "C" {
    pub fn eeh_remove_device(: *mut pci_dev);
}
extern "C" {
    pub fn eeh_unfreeze_pe(pe: *mut eeh_pe) -> c_int;
}
extern "C" {
    pub fn eeh_pe_reset_and_recover(pe: *mut eeh_pe) -> c_int;
}
extern "C" {
    pub fn eeh_dev_open(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn eeh_dev_release(pdev: *mut pci_dev);
}
extern "C" {
    pub fn eeh_pe_set_option(pe: *mut eeh_pe, option: c_int) -> c_int;
}
extern "C" {
    pub fn eeh_pe_get_state(pe: *mut eeh_pe) -> c_int;
}
extern "C" {
    pub fn eeh_pe_reset(pe: *mut eeh_pe, option: c_int, include_passed: bool) -> c_int;
}
extern "C" {
    pub fn eeh_pe_configure(pe: *mut eeh_pe) -> c_int;
}
extern "C" {
    pub fn eeh_pe_inject_mmio_error(pdev: *mut pci_dev) -> c_int;
}
//
// EEH_POSSIBLE_ERROR() -- test for possible MMIO failure.
//
// If this macro yields TRUE, the caller relays to eeh_check_failure()
// which does further tests out of line.
//

//
// Reads from a device which has been isolated by EEH will return
// all 1s.  This macro gives an all-1s value of the given size (in
// bytes: 1, 2, or 4) for comparing with the result of a read.
//

extern "C" {
    pub fn pseries_eeh_init_edev_recursive(pdn: *mut pci_dn);
}

//
// MMIO read/write operations with EEH support.
//
// Look for ffff's here at dest[n].  Assume that at least 4 bytes
// were copied. Check all four bytes.
//
// in-string eeh macros
extern "C" {
    pub fn eeh_cache_debugfs_init() -> void __init;
}

