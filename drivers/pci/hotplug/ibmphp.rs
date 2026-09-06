//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/ibmphp.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// IBM Hot Plug Controller Driver
//
// Written By: Jyoti Shah, Tong Yu, Irene Zubarev, IBM Corporation
//
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001-2003 IBM Corp.
//
// All rights reserved.
//
// Send feedback to <gregkh@us.ibm.com>
//

// EBDA stuff
//
// SLOT CAPABILITY
//
pub const EBDA_SLOT_133_MAX: c_uint = 0x20;
pub const EBDA_SLOT_100_MAX: c_uint = 0x10;
pub const EBDA_SLOT_66_MAX: c_uint = 0x02;
pub const EBDA_SLOT_PCIX_CAP: c_uint = 0x08;
//
// RESOURCE TYPE
//
pub const EBDA_RSRC_TYPE_MASK: c_uint = 0x03;
pub const EBDA_IO_RSRC_TYPE: c_uint = 0x00;
pub const EBDA_MEM_RSRC_TYPE: c_uint = 0x01;
pub const EBDA_PFM_RSRC_TYPE: c_uint = 0x03;
pub const EBDA_RES_RSRC_TYPE: c_uint = 0x02;
//
// IO RESTRICTION TYPE
//
pub const EBDA_IO_RESTRI_MASK: c_uint = 0x0c;
pub const EBDA_NO_RESTRI: c_uint = 0x00;
pub const EBDA_AVO_VGA_ADDR: c_uint = 0x04;
pub const EBDA_AVO_VGA_ADDR_AND_ALIA: c_uint = 0x08;
pub const EBDA_AVO_ISA_ADDR: c_uint = 0x0c;
//
// DEVICE TYPE DEF
//
pub const EBDA_DEV_TYPE_MASK: c_uint = 0x10;
pub const EBDA_PCI_DEV: c_uint = 0x10;
pub const EBDA_NON_PCI_DEV: c_uint = 0x00;
//
// PRIMARY DEF DEFINITION
//
pub const EBDA_PRI_DEF_MASK: c_uint = 0x20;
pub const EBDA_PRI_PCI_BUS_INFO: c_uint = 0x20;
pub const EBDA_NORM_DEV_RSRC_INFO: c_uint = 0x00;
// --------------------------------------------------------------
// RIO TABLE DATA STRUCTURE
// --------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_table_hdr {
    pub ver_num: u8,
    pub scal_count: u8,
    pub riodev_count: u8,
    pub offset: u16,
}

// -------------------------------------------------------------
// SCALABILITY DETAIL
// -------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scal_detail {
    pub node_id: u8,
    pub cbar: u32,
    pub port0_node_connect: u8,
    pub port0_port_connect: u8,
    pub port1_node_connect: u8,
    pub port1_port_connect: u8,
    pub port2_node_connect: u8,
    pub port2_port_connect: u8,
    pub chassis_num: u8,
// struct list_head scal_detail_list;
}

// --------------------------------------------------------------
// RIO DETAIL
// --------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_detail {
    pub rio_node_id: u8,
    pub bbar: u32,
    pub rio_type: u8,
    pub owner_id: u8,
    pub port0_node_connect: u8,
    pub port0_port_connect: u8,
    pub port1_node_connect: u8,
    pub port1_port_connect: u8,
    pub first_slot_num: u8,
    pub status: u8,
    pub wpindex: u8,
    pub chassis_num: u8,
    pub rio_detail_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opt_rio {
    pub rio_type: u8,
    pub chassis_num: u8,
    pub first_slot_num: u8,
    pub middle_num: u8,
    pub opt_rio_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opt_rio_lo {
    pub rio_type: u8,
    pub chassis_num: u8,
    pub first_slot_num: u8,
    pub middle_num: u8,
    pub pack_count: u8,
    pub opt_rio_lo_list: list_head,
}

//
// HPC DESCRIPTOR NODE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebda_hpc_list {
    pub format: u8,
    pub num_ctlrs: u16,
    pub phys_addr: c_short,
// struct list_head ebda_hpc_list;
}

//
// IN HPC DATA STRUCTURE, THE ASSOCIATED SLOT AND BUS
// STRUCTURE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebda_hpc_slot {
    pub slot_num: u8,
    pub slot_bus_num: u32,
    pub ctl_index: u8,
    pub slot_cap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebda_hpc_bus {
    pub bus_num: u32,
    pub slots_at_33_conv: u8,
    pub slots_at_66_conv: u8,
    pub slots_at_66_pcix: u8,
    pub slots_at_100_pcix: u8,
    pub slots_at_133_pcix: u8,
}

//
// THREE TYPE OF HOT PLUG CONTROLLER
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isa_ctlr_access {
    pub io_start: u16,
    pub io_end: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_ctlr_access {
    pub bus: u8,
    pub dev_fun: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpeg_i2c_ctlr_access {
    pub wpegbbar: c_ulong,
    pub i2c_addr: u8,
}

pub const HPC_DEVICE_ID: c_uint = 0x0246;
pub const HPC_SUBSYSTEM_ID: c_uint = 0x0247;
pub const HPC_PCI_OFFSET: c_uint = 0x40;
//
// RSTC DESCRIPTOR NODE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebda_rsrc_list {
    pub format: u8,
    pub num_entries: u16,
    pub phys_addr: u16,
    pub next: *mut ebda_rsrc_list,
}

//
// PCI RSRC NODE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebda_pci_rsrc {
    pub rsrc_type: u8,
    pub bus_num: u8,
    pub dev_fun: u8,
    pub start_addr: u32,
    pub end_addr: u32,
    pub /: *mut *mut u8 marked; / for NVRAM,
    pub ebda_pci_rsrc_list: list_head,
}

//
// BUS_INFO DATE STRUCTURE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_info {
    pub slot_min: u8,
    pub slot_max: u8,
    pub slot_count: u8,
    pub busno: u8,
    pub controller_id: u8,
    pub current_speed: u8,
    pub current_bus_mode: u8,
    pub index: u8,
    pub slots_at_33_conv: u8,
    pub slots_at_66_conv: u8,
    pub slots_at_66_pcix: u8,
    pub slots_at_100_pcix: u8,
    pub slots_at_133_pcix: u8,
    pub bus_info_list: list_head,
}

//
// GLOBAL VARIABLES
//
// FUNCTION PROTOTYPES
//
extern "C" {
    pub fn ibmphp_free_ebda_hpc_queue();
}
extern "C" {
    pub fn ibmphp_access_ebda() -> c_int;
}
extern "C" {
    pub fn ibmphp_free_bus_info_queue();
}
extern "C" {
    pub fn ibmphp_free_ebda_pci_rsrc_queue();
}
extern "C" {
    pub fn ibmphp_get_bus_index(_arg: u8) -> c_int;
}
extern "C" {
    pub fn ibmphp_get_total_controllers() -> u16;
}
extern "C" {
    pub fn ibmphp_register_pci() -> c_int;
}
// passed parameters
pub const MEM: c_int = 0;
pub const IO: c_int = 1;
pub const PFMEM: c_int = 2;
// bit masks
pub const RESTYPE: c_uint = 0x03;
pub const IOMASK: c_uint = 0x00	/* will need to take its complement */;
pub const MMASK: c_uint = 0x01;
pub const PFMASK: c_uint = 0x03;
pub const PCIDEVMASK: c_uint = 0x10	/* we should always have PCI devices */;
pub const PRIMARYBUSMASK: c_uint = 0x20;
// pci specific defines
pub const PCI_VENDOR_ID_NOTVALID: c_uint = 0xFFFF;

pub const LATENCY: c_uint = 0x64;
pub const CACHE: c_int = 64;
pub const DEVICEENABLE: c_uint = 0x015F		/* CPQ has 0x0157 */;
pub const IOBRIDGE: c_uint = 0x1000		/* 4k */;
pub const MEMBRIDGE: c_uint = 0x100000	/* 1M */;
// irqs
pub const SCSI_IRQ: c_uint = 0x09;
pub const LAN_IRQ: c_uint = 0x0A;
pub const OTHER_IRQ: c_uint = 0x0B;
// Data Structures
// type is of the form x x xx xx
// | |  |  |_ 00 - I/O, 01 - Memory, 11 - PFMemory
// | |  - 00 - No Restrictions, 01 - Avoid VGA, 10 - Avoid
// | |    VGA and their aliases, 11 - Avoid ISA
// | - 1 - PCI device, 0 - non pci device
// - 1 - Primary PCI Bus Information (0 if Normal device)
// the IO restrictions [2:3] are only for primary buses
//
// we need this struct because there could be several resource blocks
// allocated per primary bus in the EBDA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct range_node {
    pub rangeno: c_int,
    pub start: u32,
    pub end: u32,
    pub next: *mut range_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_node {
    pub busno: u8,
    pub noIORanges: c_int,
    pub rangeIO: *mut range_node,
    pub noMemRanges: c_int,
    pub rangeMem: *mut range_node,
    pub noPFMemRanges: c_int,
    pub rangePFMem: *mut range_node,
    pub needIOUpdate: c_int,
    pub needMemUpdate: c_int,
    pub needPFMemUpdate: c_int,
    pub /: *mut *mut *mut resource_node firstIO; / first IO resource on the Bus,
    pub /: *mut *mut *mut resource_node firstMem; / first memory resource on the Bus,
    pub /: *mut *mut *mut resource_node firstPFMem; / first prefetchable memory resource on the Bus,
    pub /: *mut *mut *mut resource_node firstPFMemFromMem; / when run out of pfmem available, taking from Mem,
    pub bus_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_node {
    pub rangeno: c_int,
    pub busno: u8,
    pub devfunc: u8,
    pub start: u32,
    pub end: u32,
    pub len: u32,
    pub /: *mut *mut int type; / MEM, IO, PFMEM,
    pub from: *mut *mut u8 fromMem; / this is to indicate that the range is,
// the Memory bucket rather than from PFMem
    pub next: *mut resource_node,
    pub /: *mut *mut *mut resource_node nextRange; / for the other mem range on bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct res_needed {
    pub mem: u32,
    pub pfmem: u32,
    pub io: u32,
    pub /: *mut *mut u8 not_correct; / needed for return,
    pub /: *mut *mut int devices[32]; / for device numbers behind this bridge,
}

// functions
extern "C" {
    pub fn ibmphp_rsrc_init() -> c_int;
}
extern "C" {
    pub fn ibmphp_add_resource(: *mut resource_node) -> c_int;
}
extern "C" {
    pub fn ibmphp_remove_resource(: *mut resource_node) -> c_int;
}
extern "C" {
    pub fn ibmphp_find_resource(: *mut bus_node, _arg: u32, : *mut resource_node, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ibmphp_check_resource(: *mut resource_node, _arg: u8) -> c_int;
}
extern "C" {
    pub fn ibmphp_remove_bus(: *mut bus_node, _arg: u8) -> c_int;
}
extern "C" {
    pub fn ibmphp_free_resources();
}
extern "C" {
    pub fn ibmphp_add_pfmem_from_mem(: *mut resource_node) -> c_int;
}
extern "C" {
    pub fn ibmphp_hpc_readslot(: *mut slot, _arg: u8, : *mut u8) -> c_int;
}
extern "C" {
    pub fn ibmphp_hpc_writeslot(: *mut slot, _arg: u8) -> c_int;
}
extern "C" {
    pub fn ibmphp_lock_operations();
}
extern "C" {
    pub fn ibmphp_unlock_operations();
}
extern "C" {
    pub fn ibmphp_hpc_start_poll_thread() -> c_int;
}
extern "C" {
    pub fn ibmphp_hpc_stop_poll_thread();
}
// ----------------------------------------------------------------------------
// HPC return codes
// ----------------------------------------------------------------------------
pub const HPC_ERROR: c_uint = 0xFF;
// -----------------------------------------------------------------------------
// BUS INFO
// -----------------------------------------------------------------------------
pub const BUS_SPEED: c_uint = 0x30;
pub const BUS_MODE: c_uint = 0x40;
pub const BUS_MODE_PCIX: c_uint = 0x01;
pub const BUS_MODE_PCI: c_uint = 0x00;
pub const BUS_SPEED_2: c_uint = 0x20;
pub const BUS_SPEED_1: c_uint = 0x10;
pub const BUS_SPEED_33: c_uint = 0x00;
pub const BUS_SPEED_66: c_uint = 0x01;
pub const BUS_SPEED_100: c_uint = 0x02;
pub const BUS_SPEED_133: c_uint = 0x03;
pub const BUS_SPEED_66PCIX: c_uint = 0x04;
pub const BUS_SPEED_66UNKNOWN: c_uint = 0x05;
pub const BUS_STATUS_AVAILABLE: c_uint = 0x01;
pub const BUS_CONTROL_AVAILABLE: c_uint = 0x02;
pub const SLOT_LATCH_REGS_SUPPORTED: c_uint = 0x10;
pub const PRGM_MODEL_REV_LEVEL: c_uint = 0xF0;
pub const MAX_ADAPTER_NONE: c_uint = 0x09;
// ----------------------------------------------------------------------------
// HPC 'write' operations/commands
// ----------------------------------------------------------------------------
// Command			Code	State	Write to reg
// Machine	at index
// -------------------------	----	-------	------------
pub const HPC_CTLR_ENABLEIRQ: c_uint = 0x00	// N	15;
pub const HPC_CTLR_DISABLEIRQ: c_uint = 0x01	// N	15;
pub const HPC_SLOT_OFF: c_uint = 0x02	// Y	0-14;
pub const HPC_SLOT_ON: c_uint = 0x03	// Y	0-14;
pub const HPC_SLOT_ATTNOFF: c_uint = 0x04	// N	0-14;
pub const HPC_SLOT_ATTNON: c_uint = 0x05	// N	0-14;
pub const HPC_CTLR_CLEARIRQ: c_uint = 0x06	// N	15;
pub const HPC_CTLR_RESET: c_uint = 0x07	// Y	15;
pub const HPC_CTLR_IRQSTEER: c_uint = 0x08	// N	15;
pub const HPC_BUS_33CONVMODE: c_uint = 0x09	// Y	31-34;
pub const HPC_BUS_66CONVMODE: c_uint = 0x0A	// Y	31-34;
pub const HPC_BUS_66PCIXMODE: c_uint = 0x0B	// Y	31-34;
pub const HPC_BUS_100PCIXMODE: c_uint = 0x0C	// Y	31-34;
pub const HPC_BUS_133PCIXMODE: c_uint = 0x0D	// Y	31-34;
pub const HPC_ALLSLOT_OFF: c_uint = 0x11	// Y	15;
pub const HPC_ALLSLOT_ON: c_uint = 0x12	// Y	15;
pub const HPC_SLOT_BLINKLED: c_uint = 0x13	// N	0-14;
// ----------------------------------------------------------------------------
// read commands
// ----------------------------------------------------------------------------
pub const READ_SLOTSTATUS: c_uint = 0x01;
pub const READ_EXTSLOTSTATUS: c_uint = 0x02;
pub const READ_BUSSTATUS: c_uint = 0x03;
pub const READ_CTLRSTATUS: c_uint = 0x04;
pub const READ_ALLSTAT: c_uint = 0x05;
pub const READ_ALLSLOT: c_uint = 0x06;
pub const READ_SLOTLATCHLOWREG: c_uint = 0x07;
pub const READ_REVLEVEL: c_uint = 0x08;
pub const READ_HPCOPTIONS: c_uint = 0x09;
// ----------------------------------------------------------------------------
// slot status
// ----------------------------------------------------------------------------
pub const HPC_SLOT_POWER: c_uint = 0x01;
pub const HPC_SLOT_CONNECT: c_uint = 0x02;
pub const HPC_SLOT_ATTN: c_uint = 0x04;
pub const HPC_SLOT_PRSNT2: c_uint = 0x08;
pub const HPC_SLOT_PRSNT1: c_uint = 0x10;
pub const HPC_SLOT_PWRGD: c_uint = 0x20;
pub const HPC_SLOT_BUS_SPEED: c_uint = 0x40;
pub const HPC_SLOT_LATCH: c_uint = 0x80;
// ----------------------------------------------------------------------------
// HPC_SLOT_POWER status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_POWER_OFF: c_uint = 0x00;
pub const HPC_SLOT_POWER_ON: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_SLOT_CONNECT status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_CONNECTED: c_uint = 0x00;
pub const HPC_SLOT_DISCONNECTED: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_SLOT_ATTN status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_ATTN_OFF: c_uint = 0x00;
pub const HPC_SLOT_ATTN_ON: c_uint = 0x01;
pub const HPC_SLOT_ATTN_BLINK: c_uint = 0x02;
// ----------------------------------------------------------------------------
// HPC_SLOT_PRSNT status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_EMPTY: c_uint = 0x00;
pub const HPC_SLOT_PRSNT_7: c_uint = 0x01;
pub const HPC_SLOT_PRSNT_15: c_uint = 0x02;
pub const HPC_SLOT_PRSNT_25: c_uint = 0x03;
// ----------------------------------------------------------------------------
// HPC_SLOT_PWRGD status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_PWRGD_FAULT_NONE: c_uint = 0x00;
pub const HPC_SLOT_PWRGD_GOOD: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_SLOT_BUS_SPEED status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_BUS_SPEED_OK: c_uint = 0x00;
pub const HPC_SLOT_BUS_SPEED_MISM: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_SLOT_LATCH status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_LATCH_OPEN: c_uint = 0x01	// NOTE : in PCI spec bit off = open;
pub const HPC_SLOT_LATCH_CLOSED: c_uint = 0x00	// NOTE : in PCI spec bit on  = closed;
// ----------------------------------------------------------------------------
// extended slot status
// ----------------------------------------------------------------------------
pub const HPC_SLOT_PCIX: c_uint = 0x01;
pub const HPC_SLOT_SPEED1: c_uint = 0x02;
pub const HPC_SLOT_SPEED2: c_uint = 0x04;
pub const HPC_SLOT_BLINK_ATTN: c_uint = 0x08;
pub const HPC_SLOT_RSRVD1: c_uint = 0x10;
pub const HPC_SLOT_RSRVD2: c_uint = 0x20;
pub const HPC_SLOT_BUS_MODE: c_uint = 0x40;
pub const HPC_SLOT_RSRVD3: c_uint = 0x80;
// ----------------------------------------------------------------------------
// HPC_XSLOT_PCIX_CAP status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_PCIX_NO: c_uint = 0x00;
pub const HPC_SLOT_PCIX_YES: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_XSLOT_SPEED status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_SPEED_33: c_uint = 0x00;
pub const HPC_SLOT_SPEED_66: c_uint = 0x01;
pub const HPC_SLOT_SPEED_133: c_uint = 0x02;
// ----------------------------------------------------------------------------
// HPC_XSLOT_ATTN_BLINK status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_ATTN_BLINK_OFF: c_uint = 0x00;
pub const HPC_SLOT_ATTN_BLINK_ON: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_XSLOT_BUS_MODE status return codes
// ----------------------------------------------------------------------------
pub const HPC_SLOT_BUS_MODE_OK: c_uint = 0x00;
pub const HPC_SLOT_BUS_MODE_MISM: c_uint = 0x01;
// ----------------------------------------------------------------------------
// Controller status
// ----------------------------------------------------------------------------
pub const HPC_CTLR_WORKING: c_uint = 0x01;
pub const HPC_CTLR_FINISHED: c_uint = 0x02;
pub const HPC_CTLR_RESULT0: c_uint = 0x04;
pub const HPC_CTLR_RESULT1: c_uint = 0x08;
pub const HPC_CTLR_RESULE2: c_uint = 0x10;
pub const HPC_CTLR_RESULT3: c_uint = 0x20;
pub const HPC_CTLR_IRQ_ROUTG: c_uint = 0x40;
pub const HPC_CTLR_IRQ_PENDG: c_uint = 0x80;
// ----------------------------------------------------------------------------
// HPC_CTLR_WORKING status return codes
// ----------------------------------------------------------------------------
pub const HPC_CTLR_WORKING_NO: c_uint = 0x00;
pub const HPC_CTLR_WORKING_YES: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_CTLR_FINISHED status return codes
// ----------------------------------------------------------------------------
pub const HPC_CTLR_FINISHED_NO: c_uint = 0x00;
pub const HPC_CTLR_FINISHED_YES: c_uint = 0x01;
// ----------------------------------------------------------------------------
// HPC_CTLR_RESULT status return codes
// ----------------------------------------------------------------------------
pub const HPC_CTLR_RESULT_SUCCESS: c_uint = 0x00;
pub const HPC_CTLR_RESULT_FAILED: c_uint = 0x01;
pub const HPC_CTLR_RESULT_RSVD: c_uint = 0x02;
pub const HPC_CTLR_RESULT_NORESP: c_uint = 0x03;
// ----------------------------------------------------------------------------
// macro for slot info
// ----------------------------------------------------------------------------

// --------------------------------------------------------------------------
// macro for bus info
// ---------------------------------------------------------------------------

// ----------------------------------------------------------------------------
// macro for controller info
// ----------------------------------------------------------------------------

// command that affect the state machine of HPC

// Core part of the driver
pub const ENABLE: c_int = 1;
pub const DISABLE: c_int = 0;
pub const CARD_INFO: c_uint = 0x07;
pub const PCIX133: c_uint = 0x07;
pub const PCIX66: c_uint = 0x05;
pub const PCI66: c_uint = 0x04;
// Variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_func {
    pub /: *mut *mut *mut pci_dev dev; / from the OS,
    pub busno: u8,
    pub device: u8,
    pub function: u8,
    pub io: [*mut resource_node; 6],
    pub mem: [*mut resource_node; 6],
    pub pfmem: [*mut resource_node; 6],
    pub next: *mut pci_func,
    pub /: *mut *mut int devices[32]; / for bridge config,
    pub /: *mut *mut u8 irq[4]; / for interrupt config,
    pub /: *mut *mut u8 bus; / flag for unconfiguring, to say if PPB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot {
    pub bus: u8,
    pub device: u8,
    pub number: u8,
    pub real_physical_slot_num: u8,
    pub capabilities: u32,
    pub supported_speed: u8,
    pub supported_bus_mode: u8,
    pub /: *mut *mut u8 flag; / this is for disable slot and polling,
    pub ctlr_index: u8,
    pub hotplug_slot: hotplug_slot,
    pub ctrl: *mut controller,
    pub func: *mut pci_func,
    pub irq: [u8; 4],
    pub /: *mut *mut int bit_mode; / 0 = 32, 1 = 64,
    pub bus_on: *mut bus_info,
    pub ibm_slot_list: list_head,
    pub status: u8,
    pub ext_status: u8,
    pub busstatus: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct controller {
    pub slots: *mut ebda_hpc_slot,
    pub buses: *mut ebda_hpc_bus,
    pub /: *mut *mut *mut pci_dev ctrl_dev; / in case where controller is PCI,
    pub controls*/: *mut *mut u8 starting_slot_num; / starting and ending slot #'s this ctrl,
    pub ending_slot_num: u8,
    pub revision: u8,
    pub /: *mut *mut u8 options; / which options HPC supports,
    pub status: u8,
    pub ctlr_id: u8,
    pub slot_count: u8,
    pub bus_count: u8,
    pub ctlr_relative_id: u8,
    pub irq: u32,
    pub isa_ctlr: isa_ctlr_access,
    pub pci_ctlr: pci_ctlr_access,
    pub wpeg_ctlr: wpeg_i2c_ctlr_access,
    pub u: },
    pub ctlr_type: u8,
    pub ebda_hpc_list: list_head,
}

// Functions
extern "C" {
    pub fn ibmphp_do_disable_slot(slot_cur: *mut slot) -> c_int;
}
extern "C" {
    pub fn ibmphp_configure_card(: *mut pci_func, _arg: u8) -> c_int;
}
extern "C" {
    pub fn ibmphp_unconfigure_card(: *mut slot, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: hotplug_slot, slot: struct, _arg: hotplug_slot) -> return;
}
