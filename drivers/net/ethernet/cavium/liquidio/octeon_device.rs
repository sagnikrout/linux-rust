//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/octeon_device.h
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


//
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// ! \file octeon_device.h
// \brief Host Driver: This file defines the octeon device structure.
//

// PCI VendorId Device Id
pub const OCTEON_CN68XX_PCIID: c_uint = 0x91177d;
pub const OCTEON_CN66XX_PCIID: c_uint = 0x92177d;
pub const OCTEON_CN23XX_PCIID_PF: c_uint = 0x9702177d;
// Driver identifies chips by these Ids, created by clubbing together
// DeviceId+RevisionId; Where Revision Id is not used to distinguish
// between chips, a value of 0 is used for revision id.
//
pub const OCTEON_CN68XX: c_uint = 0x0091;
pub const OCTEON_CN66XX: c_uint = 0x0092;
pub const OCTEON_CN23XX_PF_VID: c_uint = 0x9702;
pub const OCTEON_CN23XX_VF_VID: c_uint = 0x9712;
// RevisionId for the chips
pub const OCTEON_CN23XX_REV_1_0: c_uint = 0x00;
pub const OCTEON_CN23XX_REV_1_1: c_uint = 0x01;
pub const OCTEON_CN23XX_REV_2_0: c_uint = 0x80;
// SubsystemId for the chips

// Endian-swap modes supported by Octeon.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octeon_pci_swap_mode {
    OCTEON_PCI_PASSTHROUGH = 0,
    OCTEON_PCI_64BIT_SWAP = 1,
    OCTEON_PCI_32BIT_BYTE_SWAP = 2,
    OCTEON_PCI_32BIT_LW_SWAP = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lio_fw_state {
    FW_IS_PRELOADED = 0,
    FW_NEEDS_TO_BE_LOADED = 1,
    FW_IS_BEING_LOADED = 2,
    FW_HAS_BEEN_LOADED = 3,
}

pub const OCTEON_ALL_INTR: c_uint = 0xff;
// ---------------   PCI BAR1 index registers -------------
// BAR1 Mask
pub const PCI_BAR1_ENABLE_CA: c_int = 1;

pub const PCI_BAR1_ENTRY_VALID: c_int = 1;

// Octeon Device state.
// Each octeon device goes through each of these states
// as it is initialized.
//
pub const OCT_DEV_BEGIN_STATE: c_uint = 0x0;
pub const OCT_DEV_PCI_ENABLE_DONE: c_uint = 0x1;
pub const OCT_DEV_PCI_MAP_DONE: c_uint = 0x2;
pub const OCT_DEV_DISPATCH_INIT_DONE: c_uint = 0x3;
pub const OCT_DEV_INSTR_QUEUE_INIT_DONE: c_uint = 0x4;
pub const OCT_DEV_SC_BUFF_POOL_INIT_DONE: c_uint = 0x5;
pub const OCT_DEV_RESP_LIST_INIT_DONE: c_uint = 0x6;
pub const OCT_DEV_DROQ_INIT_DONE: c_uint = 0x7;
pub const OCT_DEV_MBOX_SETUP_DONE: c_uint = 0x8;
pub const OCT_DEV_MSIX_ALLOC_VECTOR_DONE: c_uint = 0x9;
pub const OCT_DEV_INTR_SET_DONE: c_uint = 0xa;
pub const OCT_DEV_IO_QUEUES_DONE: c_uint = 0xb;
pub const OCT_DEV_CONSOLE_INIT_DONE: c_uint = 0xc;
pub const OCT_DEV_HOST_OK: c_uint = 0xd;
pub const OCT_DEV_CORE_OK: c_uint = 0xe;
pub const OCT_DEV_RUNNING: c_uint = 0xf;
pub const OCT_DEV_IN_RESET: c_uint = 0x10;
pub const OCT_DEV_STATE_INVALID: c_uint = 0x11;

// Octeon Device interrupts
// These interrupt bits are set in int_status filed of
// octeon_device structure
//
pub const OCT_DEV_INTR_DMA0_FORCE: c_uint = 0x01;
pub const OCT_DEV_INTR_DMA1_FORCE: c_uint = 0x02;
pub const OCT_DEV_INTR_PKT_DATA: c_uint = 0x04;

// ---------------------------DISPATCH LIST-------------------------------
// The dispatch list entry.
// The driver keeps a record of functions registered for each
// response header opcode in this structure. Since the opcode is
// hashed to index into the driver's list, more than one opcode
// can hash to the same entry, in which case the list field points
// to a linked list with the other entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_dispatch {
// List head for this entry
    pub list: list_head,
// The opcode for which the dispatch function & arg should be used
    pub opcode: u16,
// The function to be called for a packet received by the driver
    pub dispatch_fn: octeon_dispatch_fn_t,
// The application specified argument to be passed to the above
// function along with the received packet
//
    pub arg: *mut c_void,
}

// The dispatch list structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_dispatch_list {
// access to dispatch list must be atomic
    pub lock: spinlock_t,
// Count of dispatch functions currently registered
    pub count: u32,
// The list of dispatch functions
    pub dlist: *mut octeon_dispatch,
}

// -----------------------  THE OCTEON DEVICE  ---------------------------
pub const OCT_MEM_REGIONS: c_int = 3;
// PCI address space mapping information.
// Each of the 3 address spaces given by BAR0, BAR2 and BAR4 of
// Octeon gets mapped to different physical address spaces in
// the kernel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_mmio {
// PCI address to which the BAR is mapped.
    pub start: u64,
// Length of this PCI address space.
    pub len: u32,
// Length that has been mapped to phys. address space.
    pub mapped_len: u32,
// The physical address to which the PCI address space is mapped.
    pub hw_addr: *mut u8 __iomem,
// Flag indicating the mapping was successful.
    pub done: u32,
}

pub const MAX_OCTEON_MAPS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_io_enable {
    pub iq: u64,
    pub oq: u64,
    pub iq64B: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_reg_list {
    pub pci_win_wr_addr_hi: *mut u32 __iomem,
    pub pci_win_wr_addr_lo: *mut u32 __iomem,
    pub pci_win_wr_addr: *mut u64 __iomem,
    pub pci_win_rd_addr_hi: *mut u32 __iomem,
    pub pci_win_rd_addr_lo: *mut u32 __iomem,
    pub pci_win_rd_addr: *mut u64 __iomem,
    pub pci_win_wr_data_hi: *mut u32 __iomem,
    pub pci_win_wr_data_lo: *mut u32 __iomem,
    pub pci_win_wr_data: *mut u64 __iomem,
    pub pci_win_rd_data_hi: *mut u32 __iomem,
    pub pci_win_rd_data_lo: *mut u32 __iomem,
    pub pci_win_rd_data: *mut u64 __iomem,
}

pub const OCTEON_CONSOLE_MAX_READ_BYTES: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_console {
    pub active: u32,
    pub waiting: u32,
    pub addr: u64,
    pub buffer_size: u32,
    pub input_base_addr: u64,
    pub output_base_addr: u64,
    pub print: octeon_console_print_fn,
    pub leftover: [c_char; OCTEON_CONSOLE_MAX_READ_BYTES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_board_info {
    pub name: [c_char; OCT_BOARD_NAME],
    pub serial_number: [c_char; OCT_SERIAL_LEN],
    pub major: u64,
    pub minor: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_fn_list {
    pub u32): *mut *mut *mut void (setup_iq_regs)(struct octeon_device ,,
    pub u32): *mut *mut *mut void (setup_oq_regs)(struct octeon_device ,,
    pub ): *mut *mut irqreturn_t (process_interrupt_regs)(void,
    pub ): *mut *mut u64 (msix_interrupt_handler)(void,
    pub ): *mut *mut int (setup_mbox)(struct octeon_device,
    pub ): *mut *mut int (free_mbox)(struct octeon_device,
    pub ): *mut *mut int (soft_reset)(struct octeon_device,
    pub ): *mut *mut int (setup_device_regs)(struct octeon_device,
    pub int): *mut *mut *mut void (bar1_idx_setup)(struct octeon_device , u64, u32,,
    pub u32): *mut *mut *mut void (bar1_idx_write)(struct octeon_device , u32,,
    pub u32): *mut *mut *mut u32 (bar1_idx_read)(struct octeon_device ,,
    pub ): *mut *mut u32 (update_iq_read_idx)(struct octeon_instr_queue,
    pub u32): *mut *mut *mut void (enable_oq_pkt_time_intr)(struct octeon_device ,,
    pub u32): *mut *mut *mut void (disable_oq_pkt_time_intr)(struct octeon_device ,,
    pub u8): *mut *mut *mut void (enable_interrupt)(struct octeon_device ,,
    pub u8): *mut *mut *mut void (disable_interrupt)(struct octeon_device ,,
    pub ): *mut *mut int (enable_io_queues)(struct octeon_device,
    pub ): *mut *mut void (disable_io_queues)(struct octeon_device,
}

// Must be multiple of 8, changing breaks ABI
pub const CVMX_BOOTMEM_NAME_LEN: c_int = 128;
// Structure for named memory blocks
// Number of descriptors
// available can be changed without affecting compatibility,
// but name length changes require a bump in the bootmem
// descriptor version
// Note: This structure must be naturally 64 bit aligned, as a single
// memory image will be used by both 32 and 64 bit programs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_bootmem_named_block_desc {
// Base address of named block
    pub base_addr: u64,
// Size actually allocated for named block
    pub size: u64,
// name of named block
    pub name: [c_char; CVMX_BOOTMEM_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_fw_info {
    pub /: *mut *mut *mut u32 max_nic_ports; / max nic ports for the device,
    pub /: *mut *mut *mut u32 num_gmx_ports; / num gmx ports,
    pub /: *mut *mut *mut u64 app_cap_flags; / firmware cap flags,
// The core application is running in this mode.
// See octeon-drv-opcodes.h for values.
//
    pub app_mode: u32,
    pub liquidio_firmware_version: [c_char; 32],
// Fields extracted from legacy string 'liquidio_firmware_version'
    pub maj: u8,
    pub min: u8,
    pub rev: u8,
    pub ver: },
}

// wrappers around work structs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cavium_wk {
    pub work: delayed_work,
    pub ctxptr: *mut c_void,
    pub ctxul: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cavium_wq {
    pub wq: *mut workqueue_struct,
    pub wk: cavium_wk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octdev_props {
// Each interface in the Octeon device has a network
// device pointer (used for OS specific calls).
//
    pub rx_on: c_int,
    pub fec: c_int,
    pub fec_boot: c_int,
    pub napi_enabled: c_int,
    pub gmxport: c_int,
    pub netdev: *mut net_device,
}

pub const LIO_FLAG_MSIX_ENABLED: c_uint = 0x1;
pub const MSIX_PO_INT: c_uint = 0x1;
pub const MSIX_PI_INT: c_uint = 0x2;
pub const MSIX_MBOX_INT: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_pf_vf_hs_word {

// PKIND value assigned for the DPI interface
    pub 8: u64 pkind :,
// OCTEON core clock multiplier
    pub 16: u64 core_tics_per_us :,
// OCTEON coprocessor clock multiplier
    pub 16: u64 coproc_tics_per_us :,
// app that currently running on OCTEON
    pub 8: u64 app_mode :,
// RESERVED
    pub 16: u64 reserved :,

// RESERVED
    pub 16: u64 reserved :,
// app that currently running on OCTEON
    pub 8: u64 app_mode :,
// OCTEON coprocessor clock multiplier
    pub 16: u64 coproc_tics_per_us :,
// OCTEON core clock multiplier
    pub 16: u64 core_tics_per_us :,
// PKIND value assigned for the DPI interface
    pub 8: u64 pkind :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_sriov_info {
// Number of rings assigned to VF
    pub rings_per_vf: u32,
// Max Number of VF devices that can be enabled. This variable can
// specified during load time or it will be derived after allocating
// PF queues. When max_vfs is derived then each VF will get one queue
//
    pub max_vfs: u32,
// Number of VF devices enabled using sysfs.
    pub num_vfs_alloced: u32,
// Actual rings left for PF device
    pub num_pf_rings: u32,
// SRN of PF usable IO queues
    pub pf_srn: u32,
// total pf rings
    pub trs: u32,
    pub sriov_enabled: u32,
    pub trusted_vf: lio_trusted_vf,
    pub vf_macaddr: [u64; MAX_POSSIBLE_VFS],
    pub vf_vlantci: [u16; MAX_POSSIBLE_VFS],
    pub vf_linkstate: [c_int; MAX_POSSIBLE_VFS],
    pub vf_spoofchk: [bool; MAX_POSSIBLE_VFS],
    pub vf_drv_loaded_mask: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_ioq_vector {
    pub oct_dev: *mut octeon_device,
    pub iq_index: c_int,
    pub droq_index: c_int,
    pub vector: c_int,
    pub mbox: *mut octeon_mbox,
    pub affinity_mask: cpumask,
    pub ioq_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_list {
    pub num_vfs: c_int,
    pub ndev: [*mut net_device; CN23XX_MAX_VFS_PER_PF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_devlink_priv {
    pub oct: *mut octeon_device,
}

// The Octeon device.
// Each Octeon device has this structure to represent all its
// components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_device {
// Lock for PCI window configuration accesses
    pub pci_win_lock: spinlock_t,
// Lock for memory accesses
    pub mem_access_lock: spinlock_t,
// PCI device pointer
    pub pci_dev: *mut pci_dev,
// Chip specific information.
    pub chip: *mut c_void,
// Number of interfaces detected in this octeon device.
    pub ifcount: u32,
    pub props: [octdev_props; MAX_OCTEON_LINKS],
// Octeon Chip type.
    pub chip_id: u16,
    pub rev_id: u16,
    pub subsystem_id: u32,
    pub pf_num: u16,
    pub vf_num: u16,
// This device's id - set by the driver.
    pub octeon_id: u32,
// This device's PCIe port used for traffic.
    pub pcie_port: u16,
    pub flags: u16,

// The state of this device
    pub status: core::sync::atomic::AtomicI32,
// memory mapped io range
    pub mmio: [octeon_mmio; OCT_MEM_REGIONS],
    pub reg_list: octeon_reg_list,
    pub fn_list: octeon_fn_list,
    pub boardinfo: octeon_board_info,
    pub num_iqs: u32,
// The pool containing pre allocated buffers used for soft commands
    pub sc_buf_pool: octeon_sc_buffer_pool,
// The input instruction queues
// The doubly-linked list of instruction response
    pub response_list: [octeon_response_list; MAX_RESPONSE_LISTS],
    pub num_oqs: u32,
// The DROQ output queues
    pub droq: [*mut octeon_droq; MAX_POSSIBLE_OCTEON_OUTPUT_QUEUES],
    pub io_qmask: octeon_io_enable,
// List of dispatch functions
    pub dispatch: octeon_dispatch_list,
    pub int_status: u32,
    pub droq_intr: u64,
// Physical location of the cvmx_bootmem_desc_t in octeon memory
    pub bootmem_desc_addr: u64,
// Placeholder memory for named blocks.
// Assumes single-threaded access
//
    pub bootmem_named_block_desc: cvmx_bootmem_named_block_desc,
// Address of consoles descriptor
    pub console_desc_addr: u64,
// Number of consoles available. 0 means they are inaccessible
    pub num_consoles: u32,
// Console caches
    pub console: [octeon_console; MAX_OCTEON_MAPS],
// Console named block info
    pub dram_region_base: u64,
    pub bar1_index: c_int,
    pub console_nb_info: },
// Coprocessor clock rate.
    pub coproc_clock_rate: u64,
// The core application is running in this mode. See liquidio_common.h
// for values.
//
    pub app_mode: u32,
    pub fw_info: oct_fw_info,
// The name given to this device.
    pub device_name: [c_char; 32],
// Application Context
    pub app_ctx: *mut c_void,
    pub dma_comp_wq: cavium_wq,
// Lock for dma response list
    pub cmd_resp_wqlock: spinlock_t,
    pub cmd_resp_state: u32,
    pub check_db_wq: [cavium_wq; MAX_POSSIBLE_OCTEON_INSTR_QUEUES],
    pub nic_poll_work: cavium_wk,
    pub console_poll_work: [cavium_wk; MAX_OCTEON_MAPS],
    pub priv: *mut c_void,
    pub num_msix_irqs: c_int,
    pub msix_entries: *mut c_void,
// when requesting IRQs, the names are stored here
    pub irq_name_storage: *mut c_void,
    pub sriov_info: octeon_sriov_info,
    pub pfvf_hsword: octeon_pf_vf_hs_word,
    pub msix_on: c_int,
// Mail Box details of each octeon queue.
    pub mbox: [*mut octeon_mbox; MAX_POSSIBLE_VFS],
// IOq information of it's corresponding MSI-X interrupt.
    pub ioq_vector: *mut octeon_ioq_vector,
    pub rx_pause: c_int,
    pub tx_pause: c_int,
    pub firmware*/: *mut *mut oct_link_stats link_stats; /stastics from,
// private flags to control driver-specific features through ethtool
    pub priv_flags: u32,
    pub watchdog_task: *mut c_void,
    pub rx_coalesce_usecs: u32,
    pub rx_max_coalesced_frames: u32,
    pub tx_max_coalesced_frames: u32,
    pub cores_crashed: bool,
    pub bus: c_int,
    pub dev: c_int,
    pub func: c_int,
    pub loc: },
    pub /: *mut *mut *mut atomic_t adapter_refcount; / reference count of adapter,
    pub /: *mut *mut *mut atomic_t adapter_fw_state; / per-adapter, lio_fw_state,
    pub ptp_enable: bool,
    pub vf_rep_list: lio_vf_rep_list,
    pub devlink: *mut devlink,
    pub eswitch_mode: devlink_eswitch_mode,
// for 25G NIC speed change
    pub speed_boot: u8,
    pub speed_setting: u8,
    pub no_speed_setting: u8,
    pub vfstats_poll: u32,
pub const LIO_VFSTATS_POLL: c_int = 10;
}

pub const OCT_DRV_ONLINE: c_int = 1;
pub const OCT_DRV_OFFLINE: c_int = 2;

pub const MAX_IO_PENDING_PKT_COUNT: c_int = 100;
// ------------------ Function Prototypes ----------------------
// Initialize device list memory
extern "C" {
    pub fn octeon_init_device_list(conf_type: c_int);
}
// Free memory for Input and Output queue structures for a octeon device
extern "C" {
    pub fn octeon_free_device_mem(oct: *mut octeon_device);
}
// Look up a free entry in the octeon_device table and allocate resources
// for the octeon_device structure for an octeon device. Called at init
// time.
//
// Register a device's bus location at initialization time.
// @param octeon_dev - pointer to the octeon device structure.
// @param bus        - PCIe bus #
// @param dev        - PCIe device #
// @param func       - PCIe function #
// @param is_pf      - TRUE for PF, FALSE for VF
// @return reference count of device's adapter
//
// Deregister a device at de-initialization time.
// @param octeon_dev - pointer to the octeon device structure.
// @return reference count of device's adapter
//
extern "C" {
    pub fn octeon_deregister_device(oct: *mut octeon_device) -> c_int;
}
// Initialize the driver's dispatch list which is a mix of a hash table
// and a linked list. This is done at driver load time.
// @param octeon_dev - pointer to the octeon device structure.
// @return 0 on success, else -ve error value
//
extern "C" {
    pub fn octeon_init_dispatch_list(octeon_dev: *mut octeon_device) -> c_int;
}
// Delete the driver's dispatch list and all registered entries.
// This is done at driver unload time.
// @param octeon_dev - pointer to the octeon device structure.
//
extern "C" {
    pub fn octeon_delete_dispatch_list(octeon_dev: *mut octeon_device);
}
// Initialize the core device fields with the info returned by the FW.
// @param recv_info - Receive info structure
// @param buf       - Receive buffer
//
extern "C" {
    pub fn octeon_core_drv_init(recv_info: *mut octeon_recv_info, buf: *mut c_void) -> c_int;
}
// Gets the dispatch function registered to receive packets with a
// given opcode/subcode.
// @param  octeon_dev  - the octeon device pointer.
// @param  opcode      - the opcode for which the dispatch function
// is to checked.
// @param  subcode     - the subcode for which the dispatch function
// is to checked.
//
// @return Success: octeon_dispatch_fn_t (dispatch function pointer)
// @return Failure: NULL
//
// Looks up the dispatch list to get the dispatch function for a
// given opcode.
//
// Get the octeon device pointer.
// @param octeon_id  - The id for which the octeon device pointer is required.
// @return Success: Octeon device pointer.
// @return Failure: NULL.
//
// Read windowed register.
// @param  oct   -  pointer to the Octeon device.
// @param  addr  -  Address of the register to read.
//
// This routine is called to read from the indirectly accessed
// Octeon registers that are visible through a PCI BAR0 mapped window
// register.
// @return  - 64 bit value read from the register.
//
extern "C" {
    pub fn lio_pci_readq(oct: *mut octeon_device, addr: u64) -> u64;
}
// Write windowed register.
// @param  oct  -  pointer to the Octeon device.
// @param  val  -  Value to write
// @param  addr -  Address of the register to write
//
// This routine is called to write to the indirectly accessed
// Octeon registers that are visible through a PCI BAR0 mapped window
// register.
// @return   Nothing.
//
extern "C" {
    pub fn lio_pci_writeq(oct: *mut octeon_device, val: u64, addr: u64);
}
// Routines for reading and writing CSRs

//
// Checks if memory access is okay
//
// @param oct which octeon to send to
// @return Zero on success, negative on failure.
//
extern "C" {
    pub fn octeon_mem_access_ok(oct: *mut octeon_device) -> c_int;
}
//
// Waits for DDR initialization.
//
// @param oct which octeon to send to
// @param timeout_in_ms pointer to how long to wait until DDR is initialized
// in ms.
// If contents are 0, it waits until contents are non-zero
// before starting to check.
// @return Zero on success, negative on failure.
//
// Wait for u-boot to boot and be waiting for a command.
//
// @param wait_time_hundredths
// Maximum time to wait
//
// @return Zero on success, negative on failure.
//
// Initialize console access
//
// @param oct which octeon initialize
// @return Zero on success, negative on failure.
//
extern "C" {
    pub fn octeon_init_consoles(oct: *mut octeon_device) -> c_int;
}
//
// Adds access to a console to the device.
//
// @param oct:          which octeon to add to
// @param console_num:  which console
// @param dbg_enb:      ptr to debug enablement string, one of:
// * NULL for no debug output (i.e. disabled)
// * empty string enables debug output (via default method)
// * specific string to enable debug console output
//
// @return Zero on success, negative on failure.
//
// Removes all attached consoles.
extern "C" {
    pub fn octeon_remove_consoles(oct: *mut octeon_device);
}
//
// Send a string to u-boot on console 0 as a command.
//
// @param oct which octeon to send to
// @param cmd_str String to send
// @param wait_hundredths Time to wait for u-boot to accept the command.
//
// @return Zero on success, negative on failure.
//
// Parses, validates, and downloads firmware, then boots associated cores.
// @param oct which octeon to download firmware to
// @param data  - The complete firmware file image
// @param size  - The size of the data
//
// @return 0 if success.
// -EINVAL if file is incompatible or badly formatted.
// -ENODEV if no handler was found for the application type or an
// invalid octeon id was passed.
//
// Sets up instruction queues for the device
// @param oct which octeon to setup
//
// @return 0 if success. 1 if fails
//
extern "C" {
    pub fn octeon_setup_instr_queues(oct: *mut octeon_device) -> c_int;
}
// Sets up output queues for the device
// @param oct which octeon to setup
//
// @return 0 if success. 1 if fails
//
extern "C" {
    pub fn octeon_setup_output_queues(oct: *mut octeon_device) -> c_int;
}
extern "C" {
    pub fn octeon_get_tx_qsize(oct: *mut octeon_device, q_no: u32) -> c_int;
}
extern "C" {
    pub fn octeon_get_rx_qsize(oct: *mut octeon_device, q_no: u32) -> c_int;
}
// Turns off the input and output queues for the device
// @param oct which octeon to disable
//
extern "C" {
    pub fn octeon_set_io_queues_off(oct: *mut octeon_device) -> c_int;
}
// Turns on or off the given output queue for the device
// @param oct which octeon to change
// @param q_no which queue
// @param enable 1 to enable, 0 to disable
//
extern "C" {
    pub fn octeon_set_droq_pkt_op(oct: *mut octeon_device, q_no: u32, enable: u32);
}
// Retrieve the config for the device
// @param oct which octeon
// @param card_type type of card
//
// @returns pointer to configuration
//
// Gets the octeon device configuration
// @return - pointer to the octeon configuration struture
//
extern "C" {
    pub fn octeon_free_ioq_vector(oct: *mut octeon_device);
}
extern "C" {
    pub fn octeon_allocate_ioq_vector(oct: *mut octeon_device, num_ioqs: u32) -> c_int;
}
extern "C" {
    pub fn lio_enable_irq(droq: *mut octeon_droq, iq: *mut octeon_instr_queue);
}
// LiquidIO driver pivate flags
pub const OCT_PRIV_FLAG_DEFAULT: c_uint = 0x0;
