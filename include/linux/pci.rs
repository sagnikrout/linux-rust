//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci.h
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
// pci.h
//
// PCI defines and function prototypes
// Copyright 1994, Drew Eckhardt
// Copyright 1997--1999 Martin Mares <mj@ucw.cz>
//
// PCI Express ASPM defines and function prototypes
// Copyright (c) 2007 Intel Corp.
// Zhang Yanmin (yanmin.zhang@intel.com)
// Shaohua Li (shaohua.li@intel.com)
//
// For more information, please consult the following manuals (look at
// http://www.pcisig.com/ for how to get them):
//
// PCI BIOS Specification
// PCI Local Bus Specification
// PCI to PCI Bridge Specification
// PCI Express Specification
// PCI System Design Guide
//

// Number of reset methods used in pci_reset_fn_methods array in pci.c
pub const PCI_NUM_RESET_METHODS: c_int = 8;

//
// The PCI interface treats multi-function devices as independent
// devices.  The slot/function address of each device is encoded
// in a single byte as follows:
//
// 7:3 = slot
// 2:0 = function
//
// PCI_DEVFN(), PCI_SLOT(), and PCI_FUNC() are defined in uapi/linux/pci.h.
// In the interest of not exposing interfaces to user-space unnecessarily,
// the following kernel-only defines are being added here.
//

// return bus from PCI devid = ((u16)bus_number) << 8) | devfn

//
// PCI_SLOT_ALL_DEVICES indicates a slot that covers all devices on the bus.
// Used for PCIe hotplug where the physical slot is the entire secondary bus,
// and, if ARI Forwarding is enabled, functions may appear to be on multiple
// devices.
//
pub const PCI_SLOT_ALL_DEVICES: c_uint = 0xfeff;
// Used to identify a slot as a placeholder
pub const PCI_SLOT_PLACEHOLDER: c_uint = 0xffff;
// pci_slot represents a physical slot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_slot {
    pub /: *mut *mut *mut pci_bus bus; / Bus this slot is on,
    pub /: *mut *mut list_head list; / Node in list of slots,
    pub /: *mut *mut *mut hotplug_slot hotplug; / Hotplug info (move here),
    pub /: *mut *mut u16 number; / Device nr, or PCI_SLOT_ALL_DEVICES,
    pub /: *mut *mut unsigned int per_func_slot:1; / Allow per function slot,
    pub kobj: kobject,
}

extern "C" {
    pub fn kobject_name(_arg: &slot->kobj) -> return;
}
// File state for mmap()s on /proc/bus/pci/X/Y
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_mmap_state {
    pci_mmap_io,
    pci_mmap_mem
}

// For PCI devices, the region numbers are assigned this way:
// #0-5: standard PCI resources
// #6: expansion ROM resource
// Device-specific resources

// PCI-to-PCI (P2P) bridge windows

// CardBus bridge windows

// Total number of bridge resources for P2P and CardBus
pub const PCI_P2P_BRIDGE_RESOURCE_NUM: c_int = 3;
pub const PCI_BRIDGE_RESOURCE_NUM: c_int = 4;
// Resources assigned to buses behind the bridge
// Total resources associated with a PCI device
// Preserve this for compatibility
//
// enum pci_interrupt_pin - PCI INTx interrupt values
// @PCI_INTERRUPT_UNKNOWN: Unknown or unassigned interrupt
// @PCI_INTERRUPT_INTA: PCI INTA pin
// @PCI_INTERRUPT_INTB: PCI INTB pin
// @PCI_INTERRUPT_INTC: PCI INTC pin
// @PCI_INTERRUPT_INTD: PCI INTD pin
//
// Corresponds to values for legacy PCI INTx interrupts, as can be found in the
// PCI_INTERRUPT_PIN register.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_interrupt_pin {
    PCI_INTERRUPT_UNKNOWN,
    PCI_INTERRUPT_INTA,
    PCI_INTERRUPT_INTB,
    PCI_INTERRUPT_INTC,
    PCI_INTERRUPT_INTD,
}

// The number of legacy PCI INTx interrupts
pub const PCI_NUM_INTX: c_int = 4;
//
// Reading from a device that doesn't respond typically returns ~0.  A
// successful read from a device may also return ~0, so you need additional
// information to reliably identify errors.
//

//
// pci_power_t values must match the bits in the Capabilities PME_Support
// and Control/Status PowerState fields in the Power Management capability.
//
pub type pci_power_t = int ;

// Remember to update this when the list above changes!
//
// typedef pci_channel_state_t
//
// The pci_channel state describes connectivity between the CPU and
// the PCI device.  If some PCI bus between here and the PCI device
// has crashed or locked up, this info is reflected here.
//
pub type pci_channel_state_t = u32;
// I/O channel is in normal state
// I/O to channel is blocked
// PCI card is dead
pub type pcie_reset_state_t = u32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcie_reset_state {
// Reset is NOT asserted (Use to deassert reset)
    pcie_deassert_reset = ( pcie_reset_state_t) 1,

// Use #PERST to reset PCIe device
    pcie_warm_reset = ( pcie_reset_state_t) 2,

// Use PCIe Hot Reset to reset device
    pcie_hot_reset = ( pcie_reset_state_t) 3
}

pub type pci_dev_flags_t = unsigned short ;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_dev_flags {
// INTX_DISABLE in PCI_COMMAND register disables MSI too
    PCI_DEV_FLAGS_MSI_INTX_DISABLE_BUG = ( pci_dev_flags_t) (1 << 0),
// Device configuration is irrevocably lost if disabled into D3
    PCI_DEV_FLAGS_NO_D3 = ( pci_dev_flags_t) (1 << 1),
// Provide indication device is assigned by a Virtual Machine Manager
    PCI_DEV_FLAGS_ASSIGNED = ( pci_dev_flags_t) (1 << 2),
// Flag for quirk use to store if quirk-specific ACS is enabled
    PCI_DEV_FLAGS_ACS_ENABLED_QUIRK = ( pci_dev_flags_t) (1 << 3),
// Use a PCIe-to-PCI bridge alias even if !pci_is_pcie
    PCI_DEV_FLAG_PCIE_BRIDGE_ALIAS = ( pci_dev_flags_t) (1 << 5),
// Do not use bus resets for device
    PCI_DEV_FLAGS_NO_BUS_RESET = ( pci_dev_flags_t) (1 << 6),
// Do not use PM reset even if device advertises NoSoftRst-
    PCI_DEV_FLAGS_NO_PM_RESET = ( pci_dev_flags_t) (1 << 7),
// Get VPD from function 0 VPD
    PCI_DEV_FLAGS_VPD_REF_F0 = ( pci_dev_flags_t) (1 << 8),
// A non-root bridge where translation occurs, stop alias search here
    PCI_DEV_FLAGS_BRIDGE_XLATE_ROOT = ( pci_dev_flags_t) (1 << 9),
// Do not use FLR even if device advertises PCI_AF_CAP
    PCI_DEV_FLAGS_NO_FLR_RESET = ( pci_dev_flags_t) (1 << 10),
// Don't use Relaxed Ordering for TLPs directed at this device
    PCI_DEV_FLAGS_NO_RELAXED_ORDERING = ( pci_dev_flags_t) (1 << 11),
// Device does honor MSI masking despite saying otherwise
    PCI_DEV_FLAGS_HAS_MSI_MASKING = ( pci_dev_flags_t) (1 << 12),
// Device requires write to PCI_MSIX_ENTRY_DATA before any MSIX reads
    PCI_DEV_FLAGS_MSIX_TOUCH_ENTRY_DATA_FIRST = ( pci_dev_flags_t) (1 << 13),
//
// PCIe to PCI bridge does not create RID aliases because the bridge is
// integrated with the downstream devices and doesn't use real PCI.
//
    PCI_DEV_FLAGS_PCI_BRIDGE_NO_ALIAS = ( pci_dev_flags_t) (1 << 14),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_irq_reroute_variant {
    INTEL_IRQ_REROUTE_VARIANT = 1,
    MAX_IRQ_REROUTE_VARIANTS = 3
}

pub type pci_bus_flags_t = unsigned short ;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_bus_flags {
    PCI_BUS_FLAGS_NO_MSI	= ( pci_bus_flags_t) 1,
    PCI_BUS_FLAGS_NO_MMRBC	= ( pci_bus_flags_t) 2,
    PCI_BUS_FLAGS_NO_AERSID	= ( pci_bus_flags_t) 4,
    PCI_BUS_FLAGS_NO_EXTCFG	= ( pci_bus_flags_t) 8,
}

// Values from Link Status register, PCIe r3.1, sec 7.8.8
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcie_link_width {
    PCIE_LNK_WIDTH_RESRV	= 0x00,
    PCIE_LNK_X1		= 0x01,
    PCIE_LNK_X2		= 0x02,
    PCIE_LNK_X4		= 0x04,
    PCIE_LNK_X8		= 0x08,
    PCIE_LNK_X12		= 0x0c,
    PCIE_LNK_X16		= 0x10,
    PCIE_LNK_X32		= 0x20,
    PCIE_LNK_WIDTH_UNKNOWN	= 0xff,
}

// See matching string table in pci_speed_string()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_bus_speed {
    PCI_SPEED_33MHz			= 0x00,
    PCI_SPEED_66MHz			= 0x01,
    PCI_SPEED_66MHz_PCIX		= 0x02,
    PCI_SPEED_100MHz_PCIX		= 0x03,
    PCI_SPEED_133MHz_PCIX		= 0x04,
    PCI_SPEED_66MHz_PCIX_ECC	= 0x05,
    PCI_SPEED_100MHz_PCIX_ECC	= 0x06,
    PCI_SPEED_133MHz_PCIX_ECC	= 0x07,
    PCI_SPEED_66MHz_PCIX_266	= 0x09,
    PCI_SPEED_100MHz_PCIX_266	= 0x0a,
    PCI_SPEED_133MHz_PCIX_266	= 0x0b,
    AGP_UNKNOWN			= 0x0c,
    AGP_1X				= 0x0d,
    AGP_2X				= 0x0e,
    AGP_4X				= 0x0f,
    AGP_8X				= 0x10,
    PCI_SPEED_66MHz_PCIX_533	= 0x11,
    PCI_SPEED_100MHz_PCIX_533	= 0x12,
    PCI_SPEED_133MHz_PCIX_533	= 0x13,
    PCIE_SPEED_2_5GT		= 0x14,
    PCIE_SPEED_5_0GT		= 0x15,
    PCIE_SPEED_8_0GT		= 0x16,
    PCIE_SPEED_16_0GT		= 0x17,
    PCIE_SPEED_32_0GT		= 0x18,
    PCIE_SPEED_64_0GT		= 0x19,
    PCI_SPEED_UNKNOWN		= 0xff,
}

extern "C" {
    pub fn pcie_get_speed_cap(dev: *mut pci_dev) -> pci_bus_speed;
}
extern "C" {
    pub fn pcie_get_width_cap(dev: *mut pci_dev) -> pcie_link_width;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_vpd {
    pub lock: mutex,
    pub len: c_uint,
    pub cap: u8,
}

// struct pci_dev - describes a PCI device
//
// @supported_speeds:	PCIe Supported Link Speeds Vector (+ reserved 0 at
// LSB). 0 when the supported speeds cannot be
// determined (e.g., for Root Complex Integrated
// Endpoints without the relevant Capability
// Registers).
// @is_hotplug_bridge:	Hotplug bridge of any kind (e.g. PCIe Hot-Plug Capable,
// Conventional PCI Hot-Plug, ACPI slot).
// Such bridges are allocated additional MMIO and bus
// number resources to allow for hierarchy expansion.
// @is_pciehp:		PCIe Hot-Plug Capable bridge.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dev {
    pub /: *mut *mut list_head bus_list; / Node in per-bus list,
    pub /: *mut *mut *mut pci_bus bus; / Bus this device is on,
    pub /: *mut *mut *mut pci_bus subordinate; / Bus this device bridges to,
    pub /: *mut *mut *mut void sysdata; / Hook for sys-specific extension,
    pub /: *mut *mut *mut proc_dir_entry procent; / Device entry in /proc/bus/pci,
    pub /: *mut *mut *mut pci_slot slot; / Physical slot this device is in,
    pub /: *mut *mut unsigned int devfn; / Encoded device & function index,
    pub vendor: c_ushort,
    pub device: c_ushort,
    pub subsystem_vendor: c_ushort,
    pub subsystem_device: c_ushort,
    pub /: *mut *mut unsigned int class; / 3 bytes: (base,sub,prog-if),
    pub /: *mut *mut u8 revision; / PCI revision, low byte of class word,
    pub /: *mut *mut u8 hdr_type; / PCI header type (`multi' flag masked out),

    pub /: *mut *mut u16 aer_cap; / AER capability offset,
    pub /: *mut *mut *mut aer_info aer_info; / AER info for this device,

    pub /: *mut *mut *mut rcec_ea rcec_ea; / RCEC cached endpoint association,
    pub /: *mut *mut *mut pci_dev rcec; / Associated RCEC device,

    pub /: *mut *mut u32 devcap; / PCIe Device Capabilities,
    pub /: *mut *mut u16 rebar_cap; / Resizable BAR capability offset,
    pub /: *mut *mut u8 pcie_cap; / PCIe capability offset,
    pub /: *mut *mut u8 msi_cap; / MSI capability offset,
    pub /: *mut *mut u8 msix_cap; / MSI-X capability offset,
    pub /: *mut *mut u8 pcie_mpss:3; / PCIe Max Payload Size Supported,
    pub /: *mut *mut u8 rom_base_reg; / Config register controlling ROM,
    pub /: *mut *mut u8 pin; / Interrupt pin this device uses,
    pub /: *mut *mut u16 pcie_flags_reg; / Cached PCIe Capabilities Register,
    pub /: *mut *mut *mut unsigned long dma_alias_mask;/ Mask of enabled devfn aliases,
    pub /: *mut *mut *mut pci_driver driver; / Driver bound to this device,
    pub this: *mut *mut u64 dma_mask; / Mask of the bits of bus address,
    pub for: *mut *mut u64 msi_addr_mask; / Mask of the bits of bus address,
    pub dma_parms: device_dma_parameters,
    pub ACPI,: *mut *mut pci_power_t current_state; / Current operating state. In,
    pub /: *mut *mut u8 pm_cap; / PM capability offset,
    pub PME#: *mut *mut unsigned int pme_support:5; / Bitmask of states from which,
    pub /: *mut *mut unsigned int pme_poll:1; / Poll device's PME status bit,
    pub /: *mut *mut unsigned int pinned:1; / Whether this dev is pinned,
    pub /: *mut *mut unsigned int config_rrs_sv:1; / Config RRS software visibility,
    pub /: *mut *mut unsigned int imm_ready:1; / Supports Immediate Readiness,
    pub /: *mut *mut unsigned int d1_support:1; / Low power state D1 is supported,
    pub /: *mut *mut unsigned int d2_support:1; / Low power state D2 is supported,
    pub /: *mut *mut unsigned int no_d1d2:1; / D1 and D2 are forbidden,
    pub /: *mut *mut unsigned int no_d3cold:1; / D3cold is forbidden,
    pub /: *mut *mut unsigned int bridge_d3:1; / Allow D3 for bridge,
    pub /: *mut *mut unsigned int d3cold_allowed:1; / D3cold is allowed by user,
    pub io/mem: *mut *mut unsigned int mmio_always_on:1; / Disallow turning off,
    pub wakeup_prepared:1: c_uint,
    pub /: *mut *mut unsigned int skip_bus_pm:1; / Internal: Skip bus-level PM,
    pub /: *mut *mut unsigned int ignore_hotplug:1; / Ignore hotplug events,
    pub indicators: *mut *mut unsigned int hotplug_user_indicators:1; / SlotCtl,
    pub Link: *mut *mut unsigned int clear_retrain_link:1; / Need to clear Retrain,
    pub /: *mut *mut unsigned int no_bw_notif:1; / BW notifications may cause issues,
    pub /: *mut *mut unsigned int d3hot_delay; / D3hot->D0 transition time in ms,
    pub /: *mut *mut unsigned int d3cold_delay; / D3cold->D0 transition time in ms,
    pub /: *mut *mut u16 l1ss; / L1SS Capability pointer,

    pub /: *mut *mut *mut pcie_link_state link_state; / ASPM link state,
    pub /: *mut *mut unsigned int aspm_l0s_support:1; / ASPM L0s support,
    pub /: *mut *mut unsigned int aspm_l1_support:1; / ASPM L1 support,
    pub Reporting: *mut *mut unsigned int ltr_path:1; / Latency Tolerance,

    pub /: *mut *mut unsigned int pasid_no_tlp:1; / PASID works without TLP Prefix,
    pub /: *mut *mut unsigned int eetlp_prefix_max:3; / Max # of End-End TLP Prefixes, 0=not supported,
    pub /: *mut *mut pci_channel_state_t error_state; / Current connectivity state,
    pub /: *mut *mut device dev; / Generic device interface,
    pub /: *mut *mut int cfg_size; / Size of config space,
//
// Instead of touching interrupt line and base address registers
// directly, use the values stored here. They might be different!
//
    pub irq: c_uint,
    pub /: *mut *mut resource resource[DEVICE_COUNT_RESOURCE]; / I/O and memory regions + expansion ROMs,
    pub /: *mut *mut resource driver_exclusive_resource; / driver exclusive resource ranges,
    pub /: *mut *mut unsigned int transparent:1; / Subtractive decode bridge,
    pub /: *mut *mut unsigned int io_window:1; / Bridge has I/O window,
    pub /: *mut *mut unsigned int pref_window:1; / Bridge has pref mem window,
    pub /: *mut *mut unsigned int pref_64_window:1; / Pref mem window is 64-bit,
    pub /: *mut *mut unsigned int multifunction:1; / Multi-function device,
    pub /: *mut *mut unsigned int is_busmaster:1; / Is busmaster,
    pub /: *mut *mut unsigned int no_msi:1; / May not use MSI,
    pub /: *mut *mut unsigned int block_cfg_access:1; / Config space access blocked,
    pub /: *mut *mut unsigned int broken_parity_status:1; / Generates false positive parity,
    pub /: *mut *mut unsigned int irq_reroute_variant:2; / Needs IRQ rerouting variant,
    pub msi_enabled:1: c_uint,
    pub msix_enabled:1: c_uint,
    pub /: *mut *mut unsigned int ari_enabled:1; / ARI forwarding,
    pub /: *mut *mut unsigned int ats_enabled:1; / Address Translation Svc,
    pub /: *mut *mut unsigned int pasid_enabled:1; / Process Address Space ID,
    pub /: *mut *mut unsigned int pri_enabled:1; / Page Request Interface,
    pub /: *mut *mut unsigned int tph_enabled:1; / TLP Processing Hints,
    pub /: *mut *mut unsigned int fm_enabled:1; / Flit Mode (segment captured),
    pub /: *mut *mut unsigned int is_managed:1; / Managed via devres,
    pub /: *mut *mut unsigned int is_msi_managed:1; / MSI release via devres installed,
    pub /: *mut *mut unsigned int needs_freset:1; / Requires fundamental reset,
    pub state_saved:1: c_uint,
    pub is_physfn:1: c_uint,
    pub is_virtfn:1: c_uint,
    pub is_hotplug_bridge:1: c_uint,
    pub is_pciehp:1: c_uint,
    pub /: *mut *mut unsigned int shpc_managed:1; / SHPC owned by shpchp,
    pub /: *mut *mut unsigned int is_thunderbolt:1; / Thunderbolt controller,
    pub /: *mut *mut unsigned int is_cxl:1; / Compute Express Link (CXL),
//
// Devices marked being untrusted are the ones that can potentially
// execute DMA attacks and similar. They are typically connected
// through external ports such as Thunderbolt but not limited to
// that. When an IOMMU is enabled they should be getting full
// mappings to make sure they cannot access arbitrary memory.
//
    pub untrusted:1: c_uint,
//
// Info from the platform, e.g., ACPI or device tree, may mark a
// device as "external-facing".  An external-facing device is
// itself internal but devices downstream from it are external.
//
    pub external_facing:1: c_uint,
    pub /: *mut *mut unsigned int broken_intx_masking:1; / INTx masking can't be used,
    pub /: *mut *mut unsigned int io_window_1k:1; / Intel bridge 1K I/O windows,
    pub irq_managed:1: c_uint,
    pub /: *mut *mut unsigned int non_compliant_bars:1; / Broken BARs; ignore them,
    pub /: *mut *mut unsigned int is_probed:1; / Device probing in progress,
    pub /: *mut *mut unsigned int link_active_reporting:1;/ Device capable of reporting link active,
    pub /: *mut *mut unsigned int no_vf_scan:1; / Don't scan for VFs after IOV enablement,
    pub /: *mut *mut unsigned int no_command_memory:1; / No PCI_COMMAND_MEMORY,
    pub /: *mut *mut unsigned int rom_bar_overlap:1; / ROM BAR disable broken,
    pub /: *mut *mut unsigned int rom_attr_enabled:1; / Display of ROM attribute enabled?,
    pub /: *mut *mut unsigned int non_mappable_bars:1; / BARs can't be mapped by CPU or peers,
    pub dev_flags: pci_dev_flags_t,
    pub /: *mut *mut atomic_t enable_cnt; / pci_enable_device has been called,
    pub /: *mut *mut spinlock_t pcie_cap_lock; / Protects RMW ops in capability accessors,
    pub /: *mut *mut u32 saved_config_space[16]; / Config space saved at suspend time,
    pub saved_cap_space: hlist_head,

    pub /: *mut *mut unsigned int broken_cmd_compl:1; / No compl for some cmds,

    pub /: *mut *mut u16 ptm_cap; / PTM Capability,
    pub ptm_root:1: c_uint,
    pub ptm_responder:1: c_uint,
    pub ptm_requester:1: c_uint,
    pub ptm_enable_cnt: core::sync::atomic::AtomicI32,
    pub ptm_granularity: u8,

    pub msix_base: *mut void __iomem,
    pub msi_lock: raw_spinlock_t,

    pub vpd: pci_vpd,

    pub dpc_cap: u16,
    pub dpc_rp_extensions:1: c_uint,
    pub dpc_rp_log_size: u8,

    pub link_bwctrl: *mut pcie_bwctrl_data,

    pub /: *mut *mut *mut pci_sriov sriov; / PF: SR-IOV info,
    pub /: *mut *mut *mut pci_dev physfn; / VF: related PF,
}

// These methods index pci_reset_fn_methods[]

//
// Currently in ACPI spec, for each PCI host bridge, PCI Segment
// Group number is limited to a 16-bit value, therefore (int)-1 is
// not a valid PCI domain number, and can be used as a sentinel
// value indicating ->domain_nr is not set by the driver (and
// CONFIG_PCI_DOMAINS_GENERIC=y archs will set it with
// pci_bus_find_domain_nr()).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_host_bridge {
    pub dev: device,
    pub /: *mut *mut *mut pci_bus bus; / Root bus,
    pub ops: *mut pci_ops,
    pub child_ops: *mut pci_ops,
    pub sysdata: *mut c_void,
    pub busnr: c_int,
    pub domain_nr: c_int,
    pub /: *mut *mut list_head windows; / resource_entry,
    pub /: *mut *mut list_head dma_ranges; / dma ranges resource list,
    pub /: *mut *mut list_head ports; / Root Port list (pci_host_port),

    pub /: *mut *mut u16 nr_ide_streams; / Max streams possibly active in @ide_stream_ida,
    pub ide_stream_ida: ida,
    pub /: *mut *mut ida ide_stream_ids_ida; / track unique ids per domain,

    pub /: *mut *mut *mut *mut *mut u8 (swizzle_irq)(struct pci_dev , u8 ); / Platform IRQ swizzler,
    pub u8): *const *const *const int (map_irq)(struct pci_dev , u8,,
    pub ): *mut *mut void (release_fn)(struct pci_host_bridge,
    pub dev): *mut *mut *mut int (enable_device)(struct pci_host_bridge bridge, struct pci_dev,
    pub dev): *mut *mut *mut void (disable_device)(struct pci_host_bridge bridge, struct pci_dev,
    pub dev): *mut *mut *mut int (reset_root_port)(struct pci_host_bridge bridge, struct pci_dev,
    pub release_data: *mut c_void,
    pub /: *mut *mut unsigned int ignore_reset_delay:1; / For entire hierarchy,
    pub /: *mut *mut unsigned int no_ext_tags:1; / No Extended Tags,
    pub /: *mut *mut unsigned int no_inc_mrrs:1; / No Increase MRRS,
    pub /: *mut *mut unsigned int native_aer:1; / OS may use PCIe AER,
    pub /: *mut *mut unsigned int native_pcie_hotplug:1; / OS may use PCIe hotplug,
    pub /: *mut *mut unsigned int native_shpc_hotplug:1; / OS may use SHPC hotplug,
    pub /: *mut *mut unsigned int native_pme:1; / OS may use PCIe PME,
    pub /: *mut *mut unsigned int native_ltr:1; / OS may use PCIe LTR,
    pub /: *mut *mut unsigned int native_dpc:1; / OS may use PCIe DPC,
    pub /: *mut *mut unsigned int native_cxl_error:1; / OS may use CXL RAS/Events,
    pub /: *mut *mut unsigned int preserve_config:1; / Preserve FW resource setup,
    pub /: *mut *mut unsigned int size_windows:1; / Enable root bus sizing,
    pub /: *mut *mut unsigned int msi_domain:1; / Bridge wants MSI domain,
    pub during: *mut *mut unsigned int broken_l1ss_resume:1; / Resuming from L1SS,
// Resource alignment requirements
    pub align): resource_size_t,
    pub ____cacheline_aligned: unsigned long private[],
}

extern "C" {
    pub fn container_of(_arg: priv, pci_host_bridge: struct, _arg: private) -> return;
}
extern "C" {
    pub fn pci_free_host_bridge(bridge: *mut pci_host_bridge);
}
extern "C" {
    pub fn pcibios_root_bridge_prepare(bridge: *mut pci_host_bridge) -> c_int;
}
pub const PCI_REGION_FLAG_MASK: c_uint = 0x0fU	/* These bits of resource flags tell us the PCI region flags */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bus {
    pub /: *mut *mut list_head node; / Node in list of buses,
    pub /: *mut *mut *mut pci_bus parent; / Parent bus this bridge is on,
    pub /: *mut *mut list_head children; / List of child buses,
    pub /: *mut *mut list_head devices; / List of devices on this bus,
    pub /: *mut *mut *mut pci_dev self; / Bridge device as seen by parent,
    pub bus: *mut *mut list_head slots; / List of slots on this,
    pub resource: [*mut resource; PCI_BRIDGE_RESOURCE_NUM],
    pub /: *mut *mut list_head resources; / Address space routed to this bus,
    pub /: *mut *mut resource busn_res; / Bus numbers routed to this bus,
    pub /: *mut *mut *mut pci_ops ops; / Configuration access functions,
    pub /: *mut *mut *mut void sysdata; / Hook for sys-specific extension,
    pub /: *mut *mut *mut proc_dir_entry procdir; / Directory entry in /proc/bus/pci,
    pub /: *mut *mut unsigned char number; / Bus number,
    pub /: *mut *mut unsigned char primary; / Number of primary bridge,
    pub /: *mut *mut unsigned char max_bus_speed; / enum pci_bus_speed,
    pub /: *mut *mut unsigned char cur_bus_speed; / enum pci_bus_speed,

    pub domain_nr: c_int,
    pub name: [c_char; 48],
    pub /: *mut *mut unsigned short bridge_ctl; / Manage NO_ISA/FBB/et al behaviors,
    pub /: *mut *mut pci_bus_flags_t bus_flags; / Inherited by child buses,
    pub bridge: *mut device,
    pub dev: device,
    pub is_added:1: c_uint,
    pub /: *mut *mut unsigned int unsafe_warn:1; / warned about RW1C config write,
    pub /: *mut *mut unsigned int flit_mode:1; / Link in Flit mode,
}

extern "C" {
    pub fn PCI_DEVID(_arg: dev->bus->number, _arg: dev->devfn) -> return;
}
//
// Returns true if the PCI bus is root (behind host-PCI bridge),
// false otherwise
//
// Some code assumes that "bus->self == NULL" means that bus is a root bus.
// This is incorrect because "virtual" buses added for SR-IOV (via
// virtfn_add_bus()) have "bus->self == NULL" but are not root buses.
//
// pci_is_bridge - check if the PCI device is a bridge
// @dev: PCI device
//
// Return true if the PCI device is bridge whether it has subordinate
// or not.
//
// pci_is_vga - check if the PCI device is a VGA device
// @pdev: PCI device
//
// The PCI Code and ID Assignment spec, r1.15, secs 1.4 and 1.1, define
// VGA Base Class and Sub-Classes:
//
// 03 00  PCI_CLASS_DISPLAY_VGA      VGA-compatible or 8514-compatible
// 00 01  PCI_CLASS_NOT_DEFINED_VGA  VGA-compatible (before Class Code)
//
// Return true if the PCI device is a VGA device and uses the legacy VGA
// resources ([mem 0xa0000-0xbffff], [io 0x3b0-0x3bb], [io 0x3c0-0x3df] and
// aliases).
//
// pci_is_display - check if the PCI device is a display controller
// @pdev: PCI device
//
// Determine whether the given PCI device corresponds to a display
// controller. Display controllers are typically used for graphical output
// and are identified based on their class code.
//
// Return: true if the PCI device is a display controller, false otherwise.
//

// Error values that may be returned by PCI functions
pub const PCIBIOS_SUCCESSFUL: c_uint = 0x00;
pub const PCIBIOS_FUNC_NOT_SUPPORTED: c_uint = 0x81;
pub const PCIBIOS_BAD_VENDOR_ID: c_uint = 0x83;
pub const PCIBIOS_DEVICE_NOT_FOUND: c_uint = 0x86;
pub const PCIBIOS_BAD_REGISTER_NUMBER: c_uint = 0x87;
pub const PCIBIOS_SET_FAILED: c_uint = 0x88;
pub const PCIBIOS_BUFFER_TOO_SMALL: c_uint = 0x89;
// Translate above to generic errno for passing back through non-PCI code
// Low-level architecture-dependent routines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_ops {
    pub bus): *mut *mut int (add_bus)(struct pci_bus,
    pub bus): *mut *mut void (remove_bus)(struct pci_bus,
    pub where): *mut *mut *mut *mut void __iomem (map_bus)(struct pci_bus bus, unsigned int devfn, int,
    pub val): *mut *mut *mut int (read)(struct pci_bus bus, unsigned int devfn, int where, int size, u32,
    pub val): *mut *mut *mut int (write)(struct pci_bus bus, unsigned int devfn, int where, int size, u32,
}

//
// ACPI needs to be able to access PCI config space before we've done a
// PCI bus scan and created pci_bus structures.
//

pub type pci_bus_addr_t = u64;

pub type pci_bus_addr_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bus_region {
    pub start: pci_bus_addr_t,
    pub end: pci_bus_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dynids {
    pub /: *mut *mut spinlock_t lock; / Protects list, index,
    pub /: *mut *mut list_head list; / For IDs added at runtime,
}

//
// PCI Error Recovery System (PCI-ERS).  If a PCI device driver provides
// a set of callbacks in struct pci_error_handlers, that device driver
// will be notified of PCI bus errors, and will be driven to recovery
// when an error occurs.
//
pub type pci_ers_result_t = u32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_ers_result {
// No result/none/not supported in device driver
    PCI_ERS_RESULT_NONE = ( pci_ers_result_t) 1,

// Device driver can recover without slot reset
    PCI_ERS_RESULT_CAN_RECOVER = ( pci_ers_result_t) 2,

// Device driver wants slot to be reset
    PCI_ERS_RESULT_NEED_RESET = ( pci_ers_result_t) 3,

// Device has completely failed, is unrecoverable
    PCI_ERS_RESULT_DISCONNECT = ( pci_ers_result_t) 4,

// Device driver is fully recovered and operational
    PCI_ERS_RESULT_RECOVERED = ( pci_ers_result_t) 5,

// No AER capabilities registered for the driver
    PCI_ERS_RESULT_NO_AER_DRIVER = ( pci_ers_result_t) 6,
}

// PCI bus error event callbacks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_error_handlers {
// PCI bus error detected on this device
    pub error): pci_channel_state_t,
// MMIO has been re-enabled, but not DMA
    pub dev): *mut *mut pci_ers_result_t (mmio_enabled)(struct pci_dev,
// PCI slot has been reset
    pub dev): *mut *mut pci_ers_result_t (slot_reset)(struct pci_dev,
// PCI function reset prepare or completed
    pub dev): *mut *mut void (reset_prepare)(struct pci_dev,
    pub dev): *mut *mut void (reset_done)(struct pci_dev,
// Device driver may resume normal operations
    pub dev): *mut *mut void (resume)(struct pci_dev,
// Allow device driver to record more details of a correctable error
    pub dev): *mut *mut void (cor_error_detected)(struct pci_dev,
}

//
// struct pci_driver - PCI driver structure
// @name:	Driver name.
// @id_table:	Pointer to table of device IDs the driver is
// interested in.  Most drivers should export this
// table using MODULE_DEVICE_TABLE(pci,...).
// @probe:	This probing function gets called (during execution
// of pci_register_driver() for already existing
// devices or later if a new device gets inserted) for
// all PCI devices which match the ID table and are not
// "owned" by the other drivers yet. This function gets
// passed a "struct pci_dev \*" for each device whose
// entry in the ID table matches the device. The probe
// function returns zero when the driver chooses to
// take "ownership" of the device or an error code
// (negative number) otherwise.
// The pci_device_id parameter is only valid during probe.
// The probe function always gets called from process
// context, so it can sleep.
// @remove:	The remove() function gets called whenever a device
// being handled by this driver is removed (either during
// deregistration of the driver or when it's manually
// pulled out of a hot-pluggable slot).
// The remove function always gets called from process
// context, so it can sleep.
// @suspend:	Put device into low power state.
// @resume:	Wake device from low power state.
// (Please see Documentation/power/pci.rst for descriptions
// of PCI Power Management and the related functions.)
// @shutdown:	Hook into reboot_notifier_list (kernel/sys.c).
// Intended to stop any idling DMA operations.
// Useful for enabling wake-on-lan (NIC) or changing
// the power state of a device before reboot.
// e.g. drivers/net/e100.c.
// @sriov_configure: Optional driver callback to allow configuration of
// number of VFs to enable via sysfs "sriov_numvfs" file.
// @sriov_set_msix_vec_count: PF Driver callback to change number of MSI-X
// vectors on a VF. Triggered via sysfs "sriov_vf_msix_count".
// This will change MSI-X Table Size in the VF Message Control
// registers.
// @sriov_get_vf_total_msix: PF driver callback to get the total number of
// MSI-X vectors available for distribution to the VFs.
// @err_handler: See Documentation/PCI/pci-error-recovery.rst
// @groups:	Sysfs attribute groups.
// @dev_groups: Attributes attached to the device that will be
// created once it is bound to the driver.
// @driver:	Driver model structure.
// @dynids:	List of dynamically added device IDs.
// @driver_managed_dma: Device driver doesn't use kernel DMA API for DMA.
// For most device drivers, no need to care about this flag
// as long as all DMAs are handled through the kernel DMA API.
// For some special ones, for example VFIO drivers, they know
// how to manage the DMA themselves and set this flag so that
// the IOMMU layer will allow them to setup and manage their
// own I/O address space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_driver {
    pub name: *const c_char,
    pub /: *const *const *const pci_device_id id_table; / Must be non-NULL for probe to be called,
    pub /: *const *const *const *const *const int (probe)(struct pci_dev dev, struct pci_device_id id); / New device inserted,
    pub /: *mut *mut *mut *mut void (remove)(struct pci_dev dev); / Device removed (NULL if not a hot-plug capable driver),
    pub /: *mut *mut *mut *mut int (suspend)(struct pci_dev dev, pm_message_t state); / Device suspended,
    pub /: *mut *mut *mut *mut int (resume)(struct pci_dev dev); / Device woken up,
    pub dev): *mut *mut void (shutdown)(struct pci_dev,
    pub /: *mut *mut *mut *mut int (sriov_configure)(struct pci_dev dev, int num_vfs); / On PF,
    pub /: *mut *mut *mut *mut int (sriov_set_msix_vec_count)(struct pci_dev vf, int msix_vec_count); / On PF,
    pub pf): *mut *mut u32 (sriov_get_vf_total_msix)(struct pci_dev,
    pub err_handler: *const pci_error_handlers,
    pub groups: *const attribute_group,
    pub dev_groups: *const attribute_group,
    pub driver: device_driver,
    pub dynids: pci_dynids,
    pub driver_managed_dma: bool,
}

//
// PCI_DEVICE - macro used to describe a specific PCI device
// @vend: the 16 bit PCI Vendor ID
// @dev: the 16 bit PCI Device ID
//
// This macro is used to create a struct pci_device_id that matches a
// specific device.  The subvendor and subdevice fields will be set to
// PCI_ANY_ID.
//

//
// PCI_DEVICE_DRIVER_OVERRIDE - macro used to describe a PCI device with
// override_only flags.
// @vend: the 16 bit PCI Vendor ID
// @dev: the 16 bit PCI Device ID
// @driver_override: the 32 bit PCI Device override_only
//
// This macro is used to create a struct pci_device_id that matches only a
// driver_override device. The subvendor and subdevice fields will be set to
// PCI_ANY_ID.
//

//
// PCI_DRIVER_OVERRIDE_DEVICE_VFIO - macro used to describe a VFIO
// "driver_override" PCI device.
// @vend: the 16 bit PCI Vendor ID
// @dev: the 16 bit PCI Device ID
//
// This macro is used to create a struct pci_device_id that matches a
// specific device. The subvendor and subdevice fields will be set to
// PCI_ANY_ID and the driver_override will be set to
// PCI_ID_F_VFIO_DRIVER_OVERRIDE.
//

//
// PCI_DEVICE_SUB - macro used to describe a specific PCI device with subsystem
// @vend: the 16 bit PCI Vendor ID
// @dev: the 16 bit PCI Device ID
// @subvend: the 16 bit PCI Subvendor ID
// @subdev: the 16 bit PCI Subdevice ID
//
// This macro is used to create a struct pci_device_id that matches a
// specific device with subsystem information.
//

//
// PCI_DEVICE_CLASS - macro used to describe a specific PCI device class
// @dev_class: the class, subclass, prog-if triple for this device
// @dev_class_mask: the class mask for this device
//
// This macro is used to create a struct pci_device_id that matches a
// specific PCI class.  The vendor, device, subvendor, and subdevice
// fields will be set to PCI_ANY_ID.
//

//
// PCI_VDEVICE - macro used to describe a specific PCI device in short form
// @vend: the vendor name
// @dev: the 16 bit PCI Device ID
//
// This macro is used to create a struct pci_device_id that matches a
// specific PCI device.  The subvendor, and subdevice fields will be set
// to PCI_ANY_ID. The macro allows the next field to follow as the device
// private data.
//

//
// PCI_VDEVICE_SUB - describe a specific PCI device/subdevice in a short form
// @vend: the vendor name
// @dev: the 16 bit PCI Device ID
// @subvend: the 16 bit PCI Subvendor ID
// @subdev: the 16 bit PCI Subdevice ID
//
// Generate the pci_device_id struct layout for the specific PCI
// device/subdevice. Private data may follow the output.
//

//
// PCI_DEVICE_DATA - macro used to describe a specific PCI device in very short form
// @vend: the vendor name (without PCI_VENDOR_ID_ prefix)
// @dev: the device name (without PCI_DEVICE_ID_<vend>_ prefix)
// @data: the driver data to be filled
//
// This macro is used to create a struct pci_device_id that matches a
// specific PCI device.  The subvendor, and subdevice fields will be set
// to PCI_ANY_ID.
//

// These external functions are only available when PCI support is enabled

// PCI legacy I/O port and memory address space sizes.

extern "C" {
    pub fn pcie_bus_configure_settings(bus: *mut pci_bus);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcie_bus_config_types {
    PCIE_BUS_TUNE_OFF,	/* Don't touch MPS at all */
    PCIE_BUS_DEFAULT,	/* Ensure MPS matches upstream bridge */
    PCIE_BUS_SAFE,		/* Use largest MPS boot-time devices support */
    PCIE_BUS_PERFORMANCE,	/* Use MPS and MRRS for best performance */
    PCIE_BUS_PEER2PEER,	/* Set MPS = 128 for all devices */
}

// Do NOT directly access these two variables, unless you are arch-specific PCI
// code, or PCI core code.
extern "C" {
    pub fn pcibios_resource_survey_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_bus_add_device(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pcibios_add_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_remove_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_fixup_bus(: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_enable_device(: *mut pci_dev, mask: c_int) -> int __must_check;
}
// Architecture-specific versions may override this (weak)
// Used only when drivers/pci/setup.c is used
// Generic PCI functions used internally
extern "C" {
    pub fn pcibios_scan_specific_bus(busn: c_int);
}
extern "C" {
    pub fn pci_bus_add_devices(bus: *const pci_bus);
}
extern "C" {
    pub fn pci_host_probe(bridge: *mut pci_host_bridge) -> c_int;
}
extern "C" {
    pub fn pci_probe_flush_workqueue();
}
extern "C" {
    pub fn pci_bus_insert_busn_res(b: *mut pci_bus, bus: c_int, busmax: c_int) -> c_int;
}
extern "C" {
    pub fn pci_bus_update_busn_res_end(b: *mut pci_bus, busmax: c_int) -> c_int;
}
extern "C" {
    pub fn pci_bus_release_busn_res(b: *mut pci_bus);
}
extern "C" {
    pub fn pci_scan_root_bus_bridge(bridge: *mut pci_host_bridge) -> c_int;
}
extern "C" {
    pub fn pci_destroy_slot(slot: *mut pci_slot);
}

extern "C" {
    pub fn pci_dev_assign_slot(dev: *mut pci_dev);
}

extern "C" {
    pub fn pci_scan_slot(bus: *mut pci_bus, devfn: c_int) -> c_int;
}
extern "C" {
    pub fn pci_device_add(dev: *mut pci_dev, bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_scan_child_bus(bus: *mut pci_bus) -> c_uint;
}
extern "C" {
    pub fn pci_bus_add_device(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_read_bridge_bases(child: *mut pci_bus);
}
extern "C" {
    pub fn pci_swizzle_interrupt_pin(dev: *const pci_dev, pin: u8) -> u8;
}
extern "C" {
    pub fn pci_get_interrupt_pin(dev: *mut pci_dev, bridge: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_common_swizzle(dev: *mut pci_dev, pinp: *mut u8) -> u8;
}
extern "C" {
    pub fn pci_dev_put(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_remove_bus(b: *mut pci_bus);
}
extern "C" {
    pub fn pci_stop_and_remove_bus_device(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_stop_and_remove_bus_device_locked(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_stop_root_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_remove_root_bus(bus: *mut pci_bus);
}

extern "C" {
    pub fn pci_setup_cardbus_bridge(bus: *mut pci_bus);
}

extern "C" {
    pub fn pcibios_setup_bridge(bus: *mut pci_bus, type: c_ulong);
}
extern "C" {
    pub fn pci_sort_breadthfirst();
}

// Generic PCI functions exported to card drivers
extern "C" {
    pub fn pci_bus_find_capability(bus: *mut pci_bus, devfn: c_uint, cap: c_int) -> u8;
}
extern "C" {
    pub fn pci_find_capability(dev: *mut pci_dev, cap: c_int) -> u8;
}
extern "C" {
    pub fn pci_find_next_capability(dev: *mut pci_dev, pos: u8, cap: c_int) -> u8;
}
extern "C" {
    pub fn pci_find_ht_capability(dev: *mut pci_dev, ht_cap: c_int) -> u8;
}
extern "C" {
    pub fn pci_find_next_ht_capability(dev: *mut pci_dev, pos: u8, ht_cap: c_int) -> u8;
}
extern "C" {
    pub fn pci_find_ext_capability(dev: *mut pci_dev, cap: c_int) -> u16;
}
extern "C" {
    pub fn pci_find_next_ext_capability(dev: *mut pci_dev, pos: u16, cap: c_int) -> u16;
}
extern "C" {
    pub fn pci_find_vsec_capability(dev: *mut pci_dev, vendor: u16, cap: c_int) -> u16;
}
extern "C" {
    pub fn pci_find_dvsec_capability(dev: *mut pci_dev, vendor: u16, dvsec: u16) -> u16;
}
extern "C" {
    pub fn pci_get_dsn(dev: *mut pci_dev) -> u64;
}
extern "C" {
    pub fn pci_dev_present(ids: *const pci_device_id) -> c_int;
}
extern "C" {
    pub fn pci_read_config_byte(dev: *const pci_dev, where: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn pci_read_config_word(dev: *const pci_dev, where: c_int, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn pci_read_config_dword(dev: *const pci_dev, where: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn pci_write_config_byte(dev: *const pci_dev, where: c_int, val: u8) -> c_int;
}
extern "C" {
    pub fn pci_write_config_word(dev: *const pci_dev, where: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn pci_write_config_dword(dev: *const pci_dev, where: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn pcie_capability_read_word(dev: *mut pci_dev, pos: c_int, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn pcie_capability_read_dword(dev: *mut pci_dev, pos: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn pcie_capability_write_word(dev: *mut pci_dev, pos: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn pcie_capability_write_dword(dev: *mut pci_dev, pos: c_int, val: u32) -> c_int;
}
//
// pcie_capability_clear_and_set_word - RMW accessor for PCI Express Capability Registers
// @dev:	PCI device structure of the PCI Express device
// @pos:	PCI Express Capability Register
// @clear:	Clear bitmask
// @set:	Set bitmask
//
// Perform a Read-Modify-Write (RMW) operation using @clear and @set
// bitmasks on PCI Express Capability Register at @pos. Certain PCI Express
// Capability Registers are accessed concurrently in RMW fashion, hence
// require locking which is handled transparently to the caller.
//
extern "C" {
    pub fn pcie_capability_clear_and_set_word(_arg: dev, _arg: pos, _arg: 0, _arg: set) -> return;
}
extern "C" {
    pub fn pcie_capability_clear_and_set_dword(_arg: dev, _arg: pos, _arg: 0, _arg: set) -> return;
}
extern "C" {
    pub fn pcie_capability_clear_and_set_word(_arg: dev, _arg: pos, _arg: clear, _arg: 0) -> return;
}
extern "C" {
    pub fn pcie_capability_clear_and_set_dword(_arg: dev, _arg: pos, _arg: clear, _arg: 0) -> return;
}
// User-space driven config access
extern "C" {
    pub fn pci_user_read_config_byte(dev: *mut pci_dev, where: c_int, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn pci_user_read_config_word(dev: *mut pci_dev, where: c_int, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn pci_user_read_config_dword(dev: *mut pci_dev, where: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn pci_user_write_config_byte(dev: *mut pci_dev, where: c_int, val: u8) -> c_int;
}
extern "C" {
    pub fn pci_user_write_config_word(dev: *mut pci_dev, where: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn pci_user_write_config_dword(dev: *mut pci_dev, where: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn pci_enable_device(dev: *mut pci_dev) -> int __must_check;
}
extern "C" {
    pub fn pci_enable_device_mem(dev: *mut pci_dev) -> int __must_check;
}
extern "C" {
    pub fn pci_reenable_device(: *mut pci_dev) -> int __must_check;
}
extern "C" {
    pub fn pcim_enable_device(pdev: *mut pci_dev) -> int __must_check;
}
extern "C" {
    pub fn pcim_pin_device(pdev: *mut pci_dev);
}
//
// INTx masking is supported if PCI_COMMAND_INTX_DISABLE is
// writable and no quirk has marked the feature broken.
//
extern "C" {
    pub fn pci_disable_device(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_set_master(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_clear_master(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_set_pcie_reset_state(dev: *mut pci_dev, state: pcie_reset_state) -> c_int;
}
extern "C" {
    pub fn pci_set_cacheline_size(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_set_mwi(dev: *mut pci_dev) -> int __must_check;
}
extern "C" {
    pub fn pcim_set_mwi(dev: *mut pci_dev) -> int __must_check;
}
extern "C" {
    pub fn pci_try_set_mwi(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_clear_mwi(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_disable_parity(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_intx(dev: *mut pci_dev, enable: c_int);
}
extern "C" {
    pub fn pci_check_and_mask_intx(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_check_and_unmask_intx(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_wait_for_pending(dev: *mut pci_dev, pos: c_int, mask: u16) -> c_int;
}
extern "C" {
    pub fn pci_wait_for_pending_transaction(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcix_get_max_mmrbc(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcix_get_mmrbc(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcix_set_mmrbc(dev: *mut pci_dev, mmrbc: c_int) -> c_int;
}
extern "C" {
    pub fn pcie_get_readrq(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcie_set_readrq(dev: *mut pci_dev, rq: c_int) -> c_int;
}
extern "C" {
    pub fn pcie_get_mps(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcie_set_mps(dev: *mut pci_dev, mps: c_int) -> c_int;
}
extern "C" {
    pub fn pcie_link_speed_mbps(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcie_print_link_status(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_reset_flr(dev: *mut pci_dev, probe: bool) -> c_int;
}
extern "C" {
    pub fn pcie_flr(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn __pci_reset_function_locked(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_reset_function(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_reset_function_locked(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_try_reset_function(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_probe_reset_slot(slot: *mut pci_slot) -> c_int;
}
extern "C" {
    pub fn pci_probe_reset_bus(bus: *mut pci_bus) -> c_int;
}
extern "C" {
    pub fn pci_reset_bus(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_reset_secondary_bus(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcibios_reset_secondary_bus(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_update_resource(dev: *mut pci_dev, resno: c_int);
}
extern "C" {
    pub fn pci_assign_resource(dev: *mut pci_dev, i: c_int) -> int __must_check;
}
extern "C" {
    pub fn pci_release_resource(dev: *mut pci_dev, resno: c_int) -> c_int;
}
// Resizable BAR related routines
extern "C" {
    pub fn pci_rebar_bytes_to_size(bytes: u64) -> c_int;
}
extern "C" {
    pub fn pci_rebar_size_to_bytes(size: c_int) -> resource_size_t;
}
extern "C" {
    pub fn pci_rebar_get_possible_sizes(pdev: *mut pci_dev, bar: c_int) -> u64;
}
extern "C" {
    pub fn pci_rebar_size_supported(pdev: *mut pci_dev, bar: c_int, size: c_int) -> bool;
}
extern "C" {
    pub fn pci_rebar_get_max_size(pdev: *mut pci_dev, bar: c_int) -> c_int;
}
extern "C" {
    pub fn pci_select_bars(dev: *mut pci_dev, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn pci_device_is_present(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_ignore_hotplug(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_status_get_and_clear_errors(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_free_irq(dev: *mut pci_dev, nr: c_uint, dev_id: *mut c_void);
}
// ROM control related routines
extern "C" {
    pub fn pci_enable_rom(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_disable_rom(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_unmap_rom(pdev: *mut pci_dev, rom: *mut void __iomem);
}
// Power management related routines
extern "C" {
    pub fn pci_save_state(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_restore_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_platform_power_transition(dev: *mut pci_dev, state: pci_power_t) -> c_int;
}
extern "C" {
    pub fn pci_set_power_state(dev: *mut pci_dev, state: pci_power_t) -> c_int;
}
extern "C" {
    pub fn pci_set_power_state_locked(dev: *mut pci_dev, state: pci_power_t) -> c_int;
}
extern "C" {
    pub fn pci_choose_state(dev: *mut pci_dev, state: pm_message_t) -> pci_power_t;
}
extern "C" {
    pub fn pci_pme_capable(dev: *mut pci_dev, state: pci_power_t) -> bool;
}
extern "C" {
    pub fn pci_pme_active(dev: *mut pci_dev, enable: bool);
}
extern "C" {
    pub fn pci_enable_wake(dev: *mut pci_dev, state: pci_power_t, enable: bool) -> c_int;
}
extern "C" {
    pub fn pci_wake_from_d3(dev: *mut pci_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn pci_prepare_to_sleep(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_back_from_sleep(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_dev_run_wake(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_d3cold_enable(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_d3cold_disable(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_relaxed_ordering_enabled(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_resume_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_bus_set_current_state(bus: *mut pci_bus, state: pci_power_t);
}
// For use by arch with custom probe code
extern "C" {
    pub fn set_pcie_port_type(pdev: *mut pci_dev);
}
extern "C" {
    pub fn set_pcie_hotplug_bridge(pdev: *mut pci_dev);
}
// Functions for PCI Hotplug drivers to use
extern "C" {
    pub fn pci_rescan_bus(bus: *mut pci_bus) -> c_uint;
}
extern "C" {
    pub fn pci_lock_rescan_remove();
}
extern "C" {
    pub fn pci_unlock_rescan_remove();
}
// Vital Product Data routines
extern "C" {
    pub fn pci_read_vpd(dev: *mut pci_dev, pos: loff_t, count: usize, buf: *mut c_void) -> isize;
}
extern "C" {
    pub fn pci_write_vpd(dev: *mut pci_dev, pos: loff_t, count: usize, buf: *const c_void) -> isize;
}
extern "C" {
    pub fn pci_read_vpd_any(dev: *mut pci_dev, pos: loff_t, count: usize, buf: *mut c_void) -> isize;
}
extern "C" {
    pub fn pci_write_vpd_any(dev: *mut pci_dev, pos: loff_t, count: usize, buf: *const c_void) -> isize;
}
// Helper functions for low-level code (drivers/pci/setup-[bus,res].c)
extern "C" {
    pub fn pcibios_retrieve_fw_addr(dev: *mut pci_dev, idx: c_int) -> resource_size_t;
}
extern "C" {
    pub fn pci_bus_assign_resources(bus: *const pci_bus);
}
extern "C" {
    pub fn pci_bus_claim_resources(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_bus_size_bridges(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_claim_resource(: *mut pci_dev, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn pci_claim_bridge_resource(bridge: *mut pci_dev, i: c_int) -> c_int;
}
extern "C" {
    pub fn pci_assign_unassigned_resources();
}
extern "C" {
    pub fn pci_assign_unassigned_bridge_resources(bridge: *mut pci_dev);
}
extern "C" {
    pub fn pci_assign_unassigned_bus_resources(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_assign_unassigned_root_bus_resources(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_enable_resources(: *mut pci_dev, mask: c_int) -> c_int;
}
extern "C" {
    pub fn pci_assign_irq(dev: *mut pci_dev);
}
pub const HAVE_PCI_REQ_REGIONS: c_int = 2;
extern "C" {
    pub fn pci_request_regions(: *mut pci_dev, : *const c_char) -> int __must_check;
}
extern "C" {
    pub fn pci_request_regions_exclusive(: *mut pci_dev, : *const c_char) -> int __must_check;
}
extern "C" {
    pub fn pci_release_regions(: *mut pci_dev);
}
extern "C" {
    pub fn pci_request_region(: *mut pci_dev, _arg: c_int, : *const c_char) -> int __must_check;
}
extern "C" {
    pub fn pci_release_region(: *mut pci_dev, _arg: c_int);
}
extern "C" {
    pub fn pci_request_selected_regions(: *mut pci_dev, _arg: c_int, : *const c_char) -> c_int;
}
extern "C" {
    pub fn pci_request_selected_regions_exclusive(: *mut pci_dev, _arg: c_int, : *const c_char) -> c_int;
}
extern "C" {
    pub fn pci_release_selected_regions(: *mut pci_dev, _arg: c_int);
}
// drivers/pci/bus.c
extern "C" {
    pub fn pci_add_resource(resources: *mut list_head, res: *mut resource);
}
extern "C" {
    pub fn pci_free_resource_list(resources: *mut list_head);
}
extern "C" {
    pub fn pci_bus_add_resource(bus: *mut pci_bus, res: *mut resource);
}
extern "C" {
    pub fn pci_bus_remove_resources(bus: *mut pci_bus);
}
extern "C" {
    pub fn pci_bus_remove_resource(bus: *mut pci_bus, res: *mut resource);
}
// Temporary until new and working PCI SBR API in place
extern "C" {
    pub fn pci_bridge_secondary_bus_reset(dev: *mut pci_dev) -> c_int;
}

//
// pci_bus_for_each_resource - iterate over PCI bus resources
// @bus: the PCI bus
// @res: pointer to the current resource
// @...: optional index of the current resource
//
// Iterate over PCI bus resources. The first part is to go over PCI bus
// resource array, which has at most the %PCI_BRIDGE_RESOURCE_NUM entries.
// After that continue with the separate list of the additional resources,
// if not empty. That's why the Logical OR is being used.
//
// Possible usage:
//
// struct pci_bus *bus = ...;
// struct resource *res;
// unsigned int i;
//
// // With optional index
// pci_bus_for_each_resource(bus, res, i)
// pr_info("PCI bus resource[%u]: %pR\n", i, res);
//
// // Without index
// pci_bus_for_each_resource(bus, res)
// _do_something_(res);
//

extern "C" {
    pub fn pci_address_to_pio(addr: phys_addr_t) -> c_ulong;
}
extern "C" {
    pub fn pci_pio_to_address(pio: c_ulong) -> phys_addr_t;
}
extern "C" {
    pub fn pci_remap_iospace(res: *const resource, phys_addr: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn pci_unmap_iospace(res: *mut resource);
}
// Proper probing supporting hot-pluggable devices
// pci_register_driver() must be a macro so KBUILD_MODNAME can be expanded

extern "C" {
    pub fn pci_unregister_driver(dev: *mut pci_driver);
}
//
// module_pci_driver() - Helper macro for registering a PCI driver
// @__pci_driver: pci_driver struct
//
// Helper macro for PCI drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

//
// builtin_pci_driver() - Helper macro for registering a PCI driver
// @__pci_driver: pci_driver struct
//
// Helper macro for PCI drivers which do not do anything special in their
// init code. This eliminates a lot of boilerplate. Each driver may only
// use this macro once, and calling it replaces device_initcall(...)
//

extern "C" {
    pub fn pci_cfg_space_size(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_bus_max_busnr(bus: *mut pci_bus) -> c_uchar;
}

//
// Virtual interrupts allow for more interrupts to be allocated
// than the device has interrupts for. These are not programmed
// into the device's MSI-X table and must be handled by some
// other driver means.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msix_entry {
    pub /: *mut *mut u32 vector; / Kernel uses to write allocated vector,
    pub /: *mut *mut u16 entry; / Driver uses to specify entry, OS writes,
}

extern "C" {
    pub fn pci_msi_vec_count(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_disable_msi(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_msix_vec_count(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_disable_msix(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_restore_msi_state(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_msi_enabled() -> bool;
}
extern "C" {
    pub fn pci_enable_msi(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_msix_can_alloc_dyn(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_msix_free_irq(pdev: *mut pci_dev, map: msi_map);
}
extern "C" {
    pub fn pci_free_irq_vectors(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_irq_vector(dev: *mut pci_dev, nr: c_uint) -> c_int;
}
//
// pci_irq_type - Get the interrupt type of a PCI device
// @pdev: the PCI device to operate on
//
// Discriminate the interrupt type the PCI core selected for this device
// after a successful pci_alloc_irq_vectors() call.
//
// Return: %PCI_IRQ_MSIX, %PCI_IRQ_MSI, or %PCI_IRQ_INTX.
//

//
// pci_irqd_intx_xlate() - Translate PCI INTx value to an IRQ domain hwirq
// @d: the INTx IRQ domain
// @node: the DT node for the device whose interrupt we're translating
// @intspec: the interrupt specifier data from the DT
// @intsize: the number of entries in @intspec
// @out_hwirq: pointer at which to write the hwirq number
// @out_type: pointer at which to write the interrupt type
//
// Translate a PCI INTx interrupt number from device tree in the range 1-4, as
// stored in the standard PCI_INTERRUPT_PIN register, to a value in the range
// 0-3 suitable for use in a 4 entry IRQ domain. That is, subtract one from the
// INTx value to obtain the hwirq number.
//
// Returns 0 on success, or -EINVAL if the interrupt specifier is out of range.
//
// out_hwirq = intx - PCI_INTERRUPT_INTA;

extern "C" {
    pub fn pci_disable_link_state(pdev: *mut pci_dev, state: c_int) -> c_int;
}
extern "C" {
    pub fn pci_disable_link_state_locked(pdev: *mut pci_dev, state: c_int) -> c_int;
}
extern "C" {
    pub fn pci_enable_link_state(pdev: *mut pci_dev, state: c_int) -> c_int;
}
extern "C" {
    pub fn pci_enable_link_state_locked(pdev: *mut pci_dev, state: c_int) -> c_int;
}
extern "C" {
    pub fn pcie_no_aspm();
}
extern "C" {
    pub fn pcie_aspm_support_enabled() -> bool;
}
extern "C" {
    pub fn pcie_aspm_enabled(pdev: *mut pci_dev) -> bool;
}

extern "C" {
    pub fn pci_hp_ignore_link_change(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_hp_unignore_link_change(pdev: *mut pci_dev);
}

extern "C" {
    pub fn pci_aer_available() -> bool;
}

extern "C" {
    pub fn pci_ats_disabled() -> bool;
}
pub const PCIE_PTM_CONTEXT_UPDATE_AUTO: c_int = 0;
pub const PCIE_PTM_CONTEXT_UPDATE_MANUAL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_ptm_ops {
    pub drvdata): *mut *mut int (check_capability)(void,
    pub mode): *mut *mut *mut int (context_update_write)(void drvdata, u8,
    pub mode): *mut *mut *mut int (context_update_read)(void drvdata, u8,
    pub valid): *mut *mut *mut int (context_valid_write)(void drvdata, bool,
    pub valid): *mut *mut *mut int (context_valid_read)(void drvdata, bool,
    pub clock): *mut *mut *mut int (local_clock_read)(void drvdata, u64,
    pub clock): *mut *mut *mut int (master_clock_read)(void drvdata, u64,
    pub clock): *mut *mut *mut int (t1_read)(void drvdata, u64,
    pub clock): *mut *mut *mut int (t2_read)(void drvdata, u64,
    pub clock): *mut *mut *mut int (t3_read)(void drvdata, u64,
    pub clock): *mut *mut *mut int (t4_read)(void drvdata, u64,
    pub drvdata): *mut *mut bool (context_update_visible)(void,
    pub drvdata): *mut *mut bool (context_valid_visible)(void,
    pub drvdata): *mut *mut bool (local_clock_visible)(void,
    pub drvdata): *mut *mut bool (master_clock_visible)(void,
    pub drvdata): *mut *mut bool (t1_visible)(void,
    pub drvdata): *mut *mut bool (t2_visible)(void,
    pub drvdata): *mut *mut bool (t3_visible)(void,
    pub drvdata): *mut *mut bool (t4_visible)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_ptm_debugfs {
    pub debugfs: *mut dentry,
    pub ops: *const pcie_ptm_ops,
    pub lock: mutex,
    pub pdata: *mut c_void,
}

extern "C" {
    pub fn pci_enable_ptm(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_disable_ptm(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_ptm_enabled(dev: *mut pci_dev) -> bool;
}

extern "C" {
    pub fn pcie_ptm_destroy_debugfs(ptm_debugfs: *mut pci_ptm_debugfs);
}

// pcie_ptm_create_debugfs(struct device *dev, void *pdata,

extern "C" {
    pub fn pci_cfg_access_lock(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_cfg_access_trylock(dev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn pci_cfg_access_unlock(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_dev_lock(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_dev_trylock(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_dev_unlock(dev: *mut pci_dev);
}
//
// PCI domain support.  Sometimes called PCI segment (eg by ACPI),
// a PCI domain is defined to be a set of PCI buses which share
// configuration space.
//

extern "C" {
    pub fn pci_bus_find_emul_domain_nr(hint: u32, min: u32, max: u32) -> c_int;
}
extern "C" {
    pub fn pci_bus_release_emul_domain_nr(domain_nr: c_int);
}

//
// Generic implementation for PCI domain support. If your
// architecture does not need custom management of PCI
// domains then this implementation will be used
//

extern "C" {
    pub fn acpi_pci_bus_find_domain_nr(bus: *mut pci_bus) -> c_int;
}

extern "C" {
    pub fn pci_bus_find_domain_nr(bus: *mut pci_bus, parent: *mut device) -> c_int;
}
extern "C" {
    pub fn pci_bus_release_domain_nr(parent: *mut device, domain_nr: c_int);
}

// Some architectures require additional setup to direct VGA traffic
extern "C" {
    pub fn pci_register_set_vga_state(func: arch_set_vga_state_t);
}
extern "C" {
    pub fn pci_suspend_retains_context(pdev: *mut pci_dev) -> bool;
}

//
// If the system does not have PCI, clearly these return errors.  Define
// these as simple inline functions to avoid hair in drivers.
//

// Power management related routines

// Include architecture-dependent settings and functions

//
// pci_mmap_resource_range() maps a specific BAR, and vm->vm_pgoff
// is expected to be an offset within that region.
//

pub const arch_can_pci_mmap_wc(): c_int = 0;

pub const arch_can_pci_mmap_io(): c_int = 0;

extern "C" {
    pub fn pci_iobar_pfn(pdev: *mut pci_dev, bar: c_int, vma: *mut vm_area_struct) -> c_int;
}

//
// These helpers provide future and backwards compatibility
// for accessing popular PCI BAR info
//

//
// pci_resource_is_io - check if a PCI resource is of I/O port type.
// @dev: PCI device to check.
// @resno: The resource number (BAR index) to check.
//
// Returns true if the resource type is I/O port.
//
// pci_resource_is_mem - check if a PCI resource is of memory type.
// @dev: PCI device to check.
// @resno: The resource number (BAR index) to check.
//
// Returns true if the resource type is memory, including
// prefetchable memory.
//
// Similar to the helpers above, these manipulate per-pci_dev
// driver-specific data.  They are really just a wrapper around
// the generic device structure functions of these calls.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &pdev->dev) -> return;
}
extern "C" {
    pub fn dev_name(_arg: &pdev->dev) -> return;
}
//
// The world is not perfect and supplies us with broken PCI devices.
// For at least a part of these bugs we need a work-around, so both
// generic (drivers/pci/quirks.c) and per-architecture code can define
// fixup hooks to be called for particular buggy devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_fixup {
    pub /: *mut *mut u16 vendor; / Or PCI_ANY_ID,
    pub /: *mut *mut u16 device; / Or PCI_ANY_ID,
    pub /: *mut *mut u32 class; / Or PCI_ANY_ID,
    pub /: *mut *mut unsigned int class_shift; / should be 0, 8, 16,

    pub hook_offset: c_int,

    pub dev): *mut *mut void (hook)(struct pci_dev,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_fixup_pass {
    pci_fixup_early,	/* Before probing BARs */
    pci_fixup_header,	/* After reading configuration header */
    pci_fixup_final,	/* Final phase of device fixups */
    pci_fixup_enable,	/* pci_enable_device() time */
    pci_fixup_resume,	/* pci_device_resume() */
    pci_fixup_suspend,	/* pci_device_suspend() */
    pci_fixup_resume_early, /* pci_device_resume_early() */
    pci_fixup_suspend_late,	/* pci_device_suspend_late() */
}

//
// Clang's LTO may rename static functions in C, but has no way to
// handle such renamings when referenced from inline asm. To work
// around this, create global C stubs for these cases.
//

// Anonymous variables would be nice...

extern "C" {
    pub fn pci_fixup_device(pass: pci_fixup_pass, dev: *mut pci_dev);
}

extern "C" {
    pub fn pcim_intx(pdev: *mut pci_dev, enabled: c_int) -> c_int;
}
extern "C" {
    pub fn pcim_request_all_regions(pdev: *mut pci_dev, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn pcim_iounmap_region(pdev: *mut pci_dev, bar: c_int);
}
extern "C" {
    pub fn pcim_iounmap(pdev: *mut pci_dev, addr: *mut void __iomem);
}
extern "C" {
    pub fn pcim_request_region(pdev: *mut pci_dev, bar: c_int, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn pcim_iomap_regions(pdev: *mut pci_dev, mask: c_int, name: *const c_char) -> c_int;
}

pub const PCIPCI_TRITON: c_int = 2;
pub const PCIPCI_NATOMA: c_int = 4;
pub const PCIPCI_VIAETBF: c_int = 8;
pub const PCIPCI_VSFX: c_int = 16;

// Architecture-specific versions may override these (weak)
extern "C" {
    pub fn pcibios_disable_device(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcibios_set_master(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcibios_device_add(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcibios_release_device(dev: *mut pci_dev);
}

extern "C" {
    pub fn pcibios_penalize_isa_irq(irq: c_int, active: c_int);
}

extern "C" {
    pub fn pcibios_alloc_irq(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcibios_free_irq(dev: *mut pci_dev);
}
extern "C" {
    pub fn pcibios_default_alignment() -> resource_size_t;
}

extern "C" {
    pub fn pci_mmcfg_early_init() -> void __init;
}
extern "C" {
    pub fn pci_mmcfg_late_init() -> void __init;
}

extern "C" {
    pub fn pci_ext_cfg_avail() -> c_int;
}

extern "C" {
    pub fn pci_iov_virtfn_bus(dev: *mut pci_dev, id: c_int) -> c_int;
}
extern "C" {
    pub fn pci_iov_virtfn_devfn(dev: *mut pci_dev, id: c_int) -> c_int;
}
extern "C" {
    pub fn pci_iov_vf_id(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_enable_sriov(dev: *mut pci_dev, nr_virtfn: c_int) -> c_int;
}
extern "C" {
    pub fn pci_disable_sriov(dev: *mut pci_dev);
}
extern "C" {
    pub fn pci_iov_sysfs_link(dev: *mut pci_dev, virtfn: *mut pci_dev, id: c_int) -> c_int;
}
extern "C" {
    pub fn pci_iov_add_virtfn(dev: *mut pci_dev, id: c_int) -> c_int;
}
extern "C" {
    pub fn pci_iov_remove_virtfn(dev: *mut pci_dev, id: c_int);
}
extern "C" {
    pub fn pci_num_vf(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_vfs_assigned(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_sriov_set_totalvfs(dev: *mut pci_dev, numvfs: u16) -> c_int;
}
extern "C" {
    pub fn pci_sriov_get_totalvfs(dev: *mut pci_dev) -> c_uint;
}
extern "C" {
    pub fn pci_sriov_configure_simple(dev: *mut pci_dev, nr_virtfn: c_int) -> c_int;
}
extern "C" {
    pub fn pci_iov_resource_size(dev: *const pci_dev, resno: c_int) -> resource_size_t;
}
extern "C" {
    pub fn pci_iov_vf_bar_set_size(dev: *mut pci_dev, resno: c_int, size: c_int) -> c_int;
}
extern "C" {
    pub fn pci_iov_vf_bar_get_sizes(dev: *mut pci_dev, resno: c_int, num_vfs: c_int) -> u32;
}
extern "C" {
    pub fn pci_vf_drivers_autoprobe(dev: *mut pci_dev, probe: bool);
}
// Arch may override these (weak)
extern "C" {
    pub fn pcibios_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> c_int;
}
extern "C" {
    pub fn pcibios_sriov_disable(pdev: *mut pci_dev) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

//
// pci_pcie_cap - get the saved PCIe capability offset
// @dev: PCI device
//
// PCIe capability offset is calculated at PCI device initialization
// time and saved in the data structure. This function returns saved
// PCIe capability offset. Using this instead of pci_find_capability()
// reduces unnecessary search in the PCI configuration space. If you
// need to calculate PCIe capability offset from raw device for some
// reasons, please use pci_find_capability() instead.
//
// pci_is_pcie - check if the PCI device is PCI Express capable
// @dev: PCI device
//
// Returns: true if the PCI device is PCI Express capable, false otherwise.
//
extern "C" {
    pub fn pci_pcie_cap(_arg: dev) -> return;
}
//
// pcie_caps_reg - get the PCIe Capabilities Register
// @dev: PCI device
//
// pci_pcie_type - get the PCIe device/port type
// @dev: PCI device
//
// pcie_find_root_port - Get the PCIe root port device
// @dev: PCI device
//
// Traverse up the parent chain and return the PCIe Root Port PCI Device
// for a given PCI/PCIe Device.
//
// error_state is set in pci_dev_set_io_state() using xchg/cmpxchg()
// and read w/o common lock. READ_ONCE() ensures compiler cannot cache
// the value (e.g. inside the loop in pci_dev_wait()).
//
extern "C" {
    pub fn pci_request_acs();
}
extern "C" {
    pub fn pci_acs_enabled(pdev: *mut pci_dev, acs_flags: u16) -> bool;
}
extern "C" {
    pub fn pci_enable_atomic_ops_to_root(dev: *mut pci_dev, cap_mask: u32) -> c_int;
}
pub const PCI_VPD_LRDT: c_uint = 0x80	/* Large Resource Data Type */;

// Large Resource Data Type Tag Item Names
pub const PCI_VPD_LTIN_ID_STRING: c_uint = 0x02	/* Identifier String */;
pub const PCI_VPD_LTIN_RO_DATA: c_uint = 0x10	/* Read-Only Data */;
pub const PCI_VPD_LTIN_RW_DATA: c_uint = 0x11	/* Read-Write Data */;

//
// pci_vpd_alloc - Allocate buffer and read VPD into it
// @dev: PCI device
// @size: pointer to field where VPD length is returned
//
// Returns pointer to allocated buffer or an ERR_PTR in case of failure
//
// pci_vpd_find_id_string - Locate id string in VPD
// @buf: Pointer to buffered VPD data
// @len: The length of the buffer area in which to search
// @size: Pointer to field where length of id string is returned
//
// Returns the index of the id string or -ENOENT if not found.
//
extern "C" {
    pub fn pci_vpd_find_id_string(buf: *const u8, len: c_uint, size: *mut c_uint) -> c_int;
}
//
// pci_vpd_find_ro_info_keyword - Locate info field keyword in VPD RO section
// @buf: Pointer to buffered VPD data
// @len: The length of the buffer area in which to search
// @kw: The keyword to search for
// @size: Pointer to field where length of found keyword data is returned
//
// Returns the index of the information field keyword data or -ENOENT if
// not found.
//
// pci_vpd_check_csum - Check VPD checksum
// @buf: Pointer to buffered VPD data
// @len: VPD size
//
// Returns 1 if VPD has no checksum, otherwise 0 or an errno
//
extern "C" {
    pub fn pci_vpd_check_csum(buf: *const c_void, len: c_uint) -> c_int;
}
// PCI <-> OF binding helpers

extern "C" {
    pub fn pci_host_of_has_msi_map(dev: *mut device) -> bool;
}
// Arch may override this (weak)

extern "C" {
    pub fn pci_pr3_present(pdev: *mut pci_dev) -> bool;
}

extern "C" {
    pub fn arch_pci_dev_is_removable(pdev: *mut pci_dev) -> bool;
}

extern "C" {
    pub fn pci_add_dma_alias(dev: *mut pci_dev, devfn_from: u8, nr_devfns: unsigned);
}
extern "C" {
    pub fn pci_devs_are_dma_aliases(dev1: *mut pci_dev, dev2: *mut pci_dev) -> bool;
}
// Helper functions for operation of device flag
//
// pci_ari_enabled - query ARI forwarding status
// @bus: the PCI bus
//
// Returns true if ARI forwarding is enabled.
//
// pci_is_thunderbolt_attached - whether device is on a Thunderbolt daisy chain
// @pdev: PCI device to check
//
// Walk upwards from @pdev and check for each encountered bridge if it's part
// of a Thunderbolt controller.  Reaching the host bridge means @pdev is not
// Thunderbolt-attached.  (But rather soldered to the mainboard usually.)
//

extern "C" {
    pub fn pci_uevent_ers(pdev: *mut pci_dev, err_type: pci_ers_result);
}

