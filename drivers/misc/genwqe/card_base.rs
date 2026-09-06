//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/genwqe/card_base.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// IBM Accelerator Family 'GenWQE'
//
// (C) Copyright IBM Corp. 2013
//
// Author: Frank Haverkamp <haver@linux.vnet.ibm.com>
// Author: Joerg-Stephan Vogt <jsvogt@de.ibm.com>
// Author: Michael Jung <mijung@gmx.net>
// Author: Michael Ruettger <michael@ibmra.de>
//
// Interfaces within the GenWQE module. Defines genwqe_card and
// ddcb_queue as well as ddcb_requ.
//

// Compile parameters, some of them appear in debugfs for later adjustment

// Sysfs attribute groups used when we create the genwqe device
//
// Config space for Genwqe5 A7:
// 00:[14 10 4b 04]40 00 10 00[00 00 00 12]00 00 00 00
// 10: 0c 00 00 f0 07 3c 00 00 00 00 00 00 00 00 00 00
// 20: 00 00 00 00 00 00 00 00 00 00 00 00[14 10 4b 04]
// 30: 00 00 00 00 50 00 00 00 00 00 00 00 00 00 00 00
//
pub const PCI_DEVICE_GENWQE: c_uint = 0x044b /* Genwqe DeviceID */;
pub const PCI_SUBSYSTEM_ID_GENWQE5: c_uint = 0x035f /* Genwqe A5 Subsystem-ID */;
pub const PCI_SUBSYSTEM_ID_GENWQE5_NEW: c_uint = 0x044b /* Genwqe A5 Subsystem-ID */;
pub const PCI_CLASSCODE_GENWQE5: c_uint = 0x1200 /* UNKNOWN */;
pub const PCI_SUBVENDOR_ID_IBM_SRIOV: c_uint = 0x0000;
pub const PCI_SUBSYSTEM_ID_GENWQE5_SRIOV: c_uint = 0x0000 /* Genwqe A5 Subsystem-ID */;
pub const PCI_CLASSCODE_GENWQE5_SRIOV: c_uint = 0x1200 /* UNKNOWN */;

//
// struct genwqe_reg - Genwqe data dump functionality
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_reg {
    pub addr: u32,
    pub idx: u32,
    pub val: u64,
}

//
// enum genwqe_dbg_type - Specify chip unit to dump/debug
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum genwqe_dbg_type {
    GENWQE_DBG_UNIT0 = 0,  /* captured before prev errs cleared */
    GENWQE_DBG_UNIT1 = 1,
    GENWQE_DBG_UNIT2 = 2,
    GENWQE_DBG_UNIT3 = 3,
    GENWQE_DBG_UNIT4 = 4,
    GENWQE_DBG_UNIT5 = 5,
    GENWQE_DBG_UNIT6 = 6,
    GENWQE_DBG_UNIT7 = 7,
    GENWQE_DBG_REGS  = 8,
    GENWQE_DBG_DMA   = 9,
    GENWQE_DBG_UNITS = 10, /* max number of possible debug units  */
}

// Software error injection to simulate card failures
pub const GENWQE_INJECT_HARDWARE_FAILURE: c_uint = 0x00000001 /* injects -1 reg reads */;
pub const GENWQE_INJECT_BUS_RESET_FAILURE: c_uint = 0x00000002 /* pci_bus_reset fail */;
pub const GENWQE_INJECT_GFIR_FATAL: c_uint = 0x00000004 /* GFIR = 0x0000ffff */;
pub const GENWQE_INJECT_GFIR_INFO: c_uint = 0x00000008 /* GFIR = 0xffff0000 */;
//
// Genwqe card description and management data.
//
// Error-handling in case of card malfunction
// ------------------------------------------
//
// If the card is detected to be defective the outside environment
// will cause the PCI layer to call deinit (the cleanup function for
// probe). This is the same effect like doing a unbind/bind operation
// on the card.
//
// The genwqe card driver implements a health checking thread which
// verifies the card function. If this detects a problem the cards
// device is being shutdown and restarted again, along with a reset of
// the card and queue.
//
// All functions accessing the card device return either -EIO or -ENODEV
// code to indicate the malfunction to the user. The user has to close
// the file descriptor and open a new one, once the card becomes
// available again.
//
// If the open file descriptor is setup to receive SIGIO, the signal is
// genereated for the application which has to provide a handler to
// react on it. If the application does not close the open
// file descriptor a SIGKILL is send to enforce freeing the cards
// resources.
//
// I did not find a different way to prevent kernel problems due to
// reference counters for the cards character devices getting out of
// sync. The character device deallocation does not block, even if
// there is still an open file descriptor pending. If this pending
// descriptor is closed, the data structures used by the character
// device is reinstantiated, which will lead to the reference counter
// dropping below the allowed values.
//
// Card recovery
// -------------
//
// To test the internal driver recovery the following command can be used:
// sudo sh -c 'echo 0xfffff > /sys/class/genwqe/genwqe0_card/err_inject'
//
// struct dma_mapping_type - Mapping type definition
//
// To avoid memcpying data arround we use user memory directly. To do
// this we need to pin/swap-in the memory and request a DMA address
// for it.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_mapping_type {
    GENWQE_MAPPING_RAW = 0,		/* contignous memory buffer */
    GENWQE_MAPPING_SGL_TEMP,	/* sglist dynamically used */
    GENWQE_MAPPING_SGL_PINNED,	/* sglist used with pinning */
}

//
// struct dma_mapping - Information about memory mappings done by the driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_mapping {
    pub type: dma_mapping_type,
    pub /: *mut *mut *mut void u_vaddr; / user-space vaddr/non-aligned,
    pub /: *mut *mut *mut void k_vaddr; / kernel-space vaddr/non-aligned,
    pub /: *mut *mut dma_addr_t dma_addr; / physical DMA address,
    pub /: *mut *mut *mut *mut page page_list; / list of pages used by user buff,
    pub /: *mut *mut *mut dma_addr_t dma_list; / list of dma addresses per page,
    pub /: *mut *mut unsigned int nr_pages; / number of pages,
    pub /: *mut *mut unsigned int size; / size in bytes,
    pub /: *mut *mut list_head card_list; / list of usr_maps for card,
    pub /: *mut *mut list_head pin_list; / list of pinned memory for dev,
    pub /: *mut *mut int write; / writable map? useful in unmapping,
}

//
// struct ddcb_queue - DDCB queue data
// @ddcb_max:          Number of DDCBs on the queue
// @ddcb_next:         Next free DDCB
// @ddcb_act:          Next DDCB supposed to finish
// @ddcb_seq:          Sequence number of last DDCB
// @ddcbs_in_flight:   Currently enqueued DDCBs
// @ddcbs_completed:   Number of already completed DDCBs
// @return_on_busy:    Number of -EBUSY returns on full queue
// @wait_on_busy:      Number of waits on full queue
// @ddcb_daddr:        DMA address of first DDCB in the queue
// @ddcb_vaddr:        Kernel virtual address of first DDCB in the queue
// @ddcb_req:          Associated requests (one per DDCB)
// @ddcb_waitqs:       Associated wait queues (one per DDCB)
// @ddcb_lock:         Lock to protect queuing operations
// @ddcb_waitq:        Wait on next DDCB finishing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddcb_queue {
    pub /: *mut *mut int ddcb_max; / amount of DDCBs,
    pub /: *mut *mut int ddcb_next; / next available DDCB num,
    pub /: *mut *mut int ddcb_act; / DDCB to be processed,
    pub /: *mut *mut u16 ddcb_seq; / slc seq num,
    pub /: *mut *mut unsigned int ddcbs_in_flight; / number of ddcbs in processing,
    pub ddcbs_completed: c_uint,
    pub ddcbs_max_in_flight: c_uint,
    pub /: *mut *mut unsigned int return_on_busy; / how many times -EBUSY?,
    pub wait_on_busy: c_uint,
    pub /: *mut *mut dma_addr_t ddcb_daddr; / DMA address,
    pub /: *mut *mut *mut ddcb ddcb_vaddr; / kernel virtual addr for DDCBs,
    pub /: *mut *mut *mut *mut ddcb_requ ddcb_req; / ddcb processing parameter,
    pub /: *mut *mut *mut wait_queue_head_t ddcb_waitqs; / waitqueue per ddcb,
    pub /: *mut *mut spinlock_t ddcb_lock; / exclusive access to queue,
    pub /: *mut *mut wait_queue_head_t busy_waitq; / wait for ddcb processing,
// registers or the respective queue to be used
    pub IO_QUEUE_CONFIG: u32,
    pub IO_QUEUE_STATUS: u32,
    pub IO_QUEUE_SEGMENT: u32,
    pub IO_QUEUE_INITSQN: u32,
    pub IO_QUEUE_WRAP: u32,
    pub IO_QUEUE_OFFSET: u32,
    pub IO_QUEUE_WTIME: u32,
    pub IO_QUEUE_ERRCNTS: u32,
    pub IO_QUEUE_LRW: u32,
}

//
// GFIR, SLU_UNITCFG, APP_UNITCFG
// 8 Units with FIR/FEC + 64 * 2ndary FIRS/FEC.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_ffdc {
    pub entries: c_uint,
    pub regs: *mut genwqe_reg,
}

//
// struct genwqe_dev - GenWQE device information
// @card_state:       Card operation state, see above
// @ffdc:             First Failure Data Capture buffers for each unit
// @card_thread:      Working thread to operate the DDCB queue
// @card_waitq:       Wait queue used in card_thread
// @queue:            DDCB queue
// @health_thread:    Card monitoring thread (only for PFs)
// @health_waitq:     Wait queue used in health_thread
// @pci_dev:          Associated PCI device (function)
// @mmio:             Base address of 64-bit register space
// @mmio_len:         Length of register area
// @file_lock:        Lock to protect access to file_list
// @file_list:        List of all processes with open GenWQE file descriptors
//
// This struct contains all information needed to communicate with a
// GenWQE card. It is initialized when a GenWQE device is found and
// destroyed when it goes away. It holds data to maintain the queue as
// well as data needed to feed the user interfaces.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_dev {
    pub card_state: genwqe_card_state,
    pub print_lock: spinlock_t,
    pub /: *mut *mut int card_idx; / card index 0..CARD_NO_MAX-1,
    pub /: *mut *mut u64 flags; / general flags,
// FFDC data gathering
    pub ffdc: [genwqe_ffdc; GENWQE_DBG_UNITS],
// DDCB workqueue
    pub card_thread: *mut task_struct,
    pub queue_waitq: wait_queue_head_t,
    pub /: *mut *mut ddcb_queue queue; / genwqe DDCB queue,
    pub irqs_processed: c_uint,
// Card health checking thread
    pub health_thread: *mut task_struct,
    pub health_waitq: wait_queue_head_t,
    pub /: *mut *mut int use_platform_recovery; / use platform recovery mechanisms,
// char device
    pub /: *mut *mut dev_t devnum_genwqe; / major/minor num card,
    pub /: *const *const *const class class_genwqe; / reference to class object,
    pub /: *mut *mut *mut device dev; / for device creation,
    pub /: *mut *mut cdev cdev_genwqe; / char device for card,
    pub /: *mut *mut *mut dentry debugfs_root; / debugfs card root directory,
    pub /: *mut *mut *mut dentry debugfs_genwqe; / debugfs driver root directory,
// pci resources
    pub /: *mut *mut *mut pci_dev pci_dev; / PCI device,
    pub /: *mut *mut *mut void __iomem mmio; / BAR-0 MMIO start,
    pub mmio_len: c_ulong,
    pub num_vfs: c_int,
    pub vf_jobtimeout_msec: [u32; GENWQE_MAX_VFS],
    pub /: *mut *mut int is_privileged; / access to all regs possible,
// config regs which we need often
    pub slu_unitcfg: u64,
    pub app_unitcfg: u64,
    pub softreset: u64,
    pub err_inject: u64,
    pub last_gfir: u64,
    pub app_name: [c_char; 5],
    pub /: *mut *mut spinlock_t file_lock; / lock for open files,
    pub /: *mut *mut list_head file_list; / list of open files,
// debugfs parameters
    pub /: *mut *mut int ddcb_software_timeout; / wait until DDCB times out,
    pub /: *mut *mut int skip_recovery; / circumvention if recovery fails,
    pub /: *mut *mut int kill_timeout; / wait after sending SIGKILL,
}

//
// enum genwqe_requ_state - State of a DDCB execution request
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum genwqe_requ_state {
    GENWQE_REQU_NEW      = 0,
    GENWQE_REQU_ENQUEUED = 1,
    GENWQE_REQU_TAPPED   = 2,
    GENWQE_REQU_FINISHED = 3,
    GENWQE_REQU_STATE_MAX,
}

//
// struct genwqe_sgl - Scatter gather list describing user-space memory
// @sgl:            scatter gather list needs to be 128 byte aligned
// @sgl_dma_addr:   dma address of sgl
// @sgl_size:       size of area used for sgl
// @user_addr:      user-space address of memory area
// @user_size:      size of user-space memory area
// @page:           buffer for partial pages if needed
// @page_dma_addr:  dma address partial pages
// @write:          should we write it back to userspace?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_sgl {
    pub sgl_dma_addr: dma_addr_t,
    pub sgl: *mut sg_entry,
    pub /: *mut *mut size_t sgl_size; / size of sgl,
    pub /: *mut *mut *mut void __user user_addr; / user-space base-address,
    pub /: *mut *mut size_t user_size; / size of memory area,
    pub write: c_int,
    pub nr_pages: c_ulong,
    pub fpage_offs: c_ulong,
    pub fpage_size: usize,
    pub lpage_size: usize,
    pub fpage: *mut c_void,
    pub fpage_dma_addr: dma_addr_t,
    pub lpage: *mut c_void,
    pub lpage_dma_addr: dma_addr_t,
}

extern "C" {
    pub fn genwqe_free_sync_sgl(cd: *mut genwqe_dev, sgl: *mut genwqe_sgl) -> c_int;
}
//
// struct ddcb_requ - Kernel internal representation of the DDCB request
// @cmd:          User space representation of the DDCB execution request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddcb_requ {
// kernel specific content
    pub /: *mut *mut genwqe_requ_state req_state; / request status,
    pub /: *mut *mut int num; / ddcb_no for this request,
    pub /: *mut *mut *mut ddcb_queue queue; / associated queue,
    pub dma_mappings: [dma_mapping; DDCB_FIXUPS],
    pub sgls: [genwqe_sgl; DDCB_FIXUPS],
// kernel/user shared content
    pub /: *mut *mut genwqe_ddcb_cmd cmd; / ddcb_no for this request,
    pub debug_data: genwqe_debug_data,
}

//
// struct genwqe_file - Information for open GenWQE devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genwqe_file {
    pub cd: *mut genwqe_dev,
    pub client: *mut genwqe_driver,
    pub filp: *mut file,
    pub async_queue: *mut fasync_struct,
    pub opener: *mut pid,
    pub /: *mut *mut list_head list; / entry in list of open files,
    pub /: *mut *mut spinlock_t map_lock; / lock for dma_mappings,
    pub /: *mut *mut list_head map_list; / list of dma_mappings,
    pub /: *mut *mut spinlock_t pin_lock; / lock for pinned memory,
    pub /: *mut *mut list_head pin_list; / list of pinned memory,
}

extern "C" {
    pub fn genwqe_finish_queue(cd: *mut genwqe_dev) -> c_int;
}
extern "C" {
    pub fn genwqe_release_service_layer(cd: *mut genwqe_dev) -> c_int;
}
//
// genwqe_get_slu_id() - Read Service Layer Unit Id
// Return: 0x00: Development code
// 0x01: SLC1 (old)
// 0x02: SLC2 (sept2012)
// 0x03: SLC2 (feb2013, generic driver)
//
extern "C" {
    pub fn genwqe_ddcbs_in_flight(cd: *mut genwqe_dev) -> c_int;
}
extern "C" {
    pub fn genwqe_card_type(cd: *mut genwqe_dev) -> u8;
}
extern "C" {
    pub fn genwqe_card_reset(cd: *mut genwqe_dev) -> c_int;
}
extern "C" {
    pub fn genwqe_set_interrupt_capability(cd: *mut genwqe_dev, count: c_int) -> c_int;
}
extern "C" {
    pub fn genwqe_reset_interrupt_capability(cd: *mut genwqe_dev);
}
extern "C" {
    pub fn genwqe_device_create(cd: *mut genwqe_dev) -> c_int;
}
extern "C" {
    pub fn genwqe_device_remove(cd: *mut genwqe_dev) -> c_int;
}
// debugfs
extern "C" {
    pub fn genwqe_init_debugfs(cd: *mut genwqe_dev);
}
extern "C" {
    pub fn genqwe_exit_debugfs(cd: *mut genwqe_dev);
}
extern "C" {
    pub fn genwqe_read_softreset(cd: *mut genwqe_dev) -> c_int;
}
// Hardware Circumventions
extern "C" {
    pub fn genwqe_recovery_on_fatal_gfir_required(cd: *mut genwqe_dev) -> c_int;
}
extern "C" {
    pub fn genwqe_flash_readback_fails(cd: *mut genwqe_dev) -> c_int;
}
//
// genwqe_write_vreg() - Write register in VF window
// @cd:    genwqe device
// @reg:   register address
// @val:   value to write
// @func:  0: PF, 1: VF0, ..., 15: VF14
//
extern "C" {
    pub fn genwqe_write_vreg(cd: *mut genwqe_dev, reg: u32, val: u64, func: c_int) -> c_int;
}
//
// genwqe_read_vreg() - Read register in VF window
// @cd:    genwqe device
// @reg:   register address
// @func:  0: PF, 1: VF0, ..., 15: VF14
//
// Return: content of the register
//
extern "C" {
    pub fn genwqe_read_vreg(cd: *mut genwqe_dev, reg: u32, func: c_int) -> u64;
}
// FFDC Buffer Management
extern "C" {
    pub fn genwqe_ffdc_buff_size(cd: *mut genwqe_dev, unit_id: c_int) -> c_int;
}
extern "C" {
    pub fn genwqe_init_crc32();
}
extern "C" {
    pub fn genwqe_read_app_id(cd: *mut genwqe_dev, app_name: *mut c_char, len: c_int) -> c_int;
}
// Memory allocation/deallocation; dma address handling
extern "C" {
    pub fn genwqe_user_vunmap(cd: *mut genwqe_dev, m: *mut dma_mapping) -> c_int;
}
//
// __genwqe_execute_ddcb() - Execute DDCB request with addr translation
//
// This function will do the address translation changes to the DDCBs
// according to the definitions required by the ATS field. It looks up
// the memory allocation buffer or does vmap/vunmap for the respective
// user-space buffers, inclusive page pinning and scatter gather list
// buildup and teardown.
//
// __genwqe_execute_raw_ddcb() - Execute DDCB request without addr translation
//
// This version will not do address translation or any modification of
// the DDCB data. It is used e.g. for the MoveFlash DDCB which is
// entirely prepared by the driver itself. That means the appropriate
// DMA addresses are already in the DDCB and do not need any
// modification.
//
extern "C" {
    pub fn __genwqe_wait_ddcb(cd: *mut genwqe_dev, req: *mut ddcb_requ) -> c_int;
}
extern "C" {
    pub fn __genwqe_purge_ddcb(cd: *mut genwqe_dev, req: *mut ddcb_requ) -> c_int;
}
// register access
extern "C" {
    pub fn __genwqe_writeq(cd: *mut genwqe_dev, byte_offs: u64, val: u64) -> c_int;
}
extern "C" {
    pub fn __genwqe_readq(cd: *mut genwqe_dev, byte_offs: u64) -> u64;
}
extern "C" {
    pub fn __genwqe_writel(cd: *mut genwqe_dev, byte_offs: u64, val: u32) -> c_int;
}
extern "C" {
    pub fn __genwqe_readl(cd: *mut genwqe_dev, byte_offs: u64) -> u32;
}
// Base clock frequency in MHz
extern "C" {
    pub fn genwqe_base_clock_frequency(cd: *mut genwqe_dev) -> c_int;
}
// Before FFDC is captured the traps should be stopped.
extern "C" {
    pub fn genwqe_stop_traps(cd: *mut genwqe_dev);
}
extern "C" {
    pub fn genwqe_start_traps(cd: *mut genwqe_dev);
}
// Hardware circumvention
extern "C" {
    pub fn genwqe_need_err_masking(cd: *mut genwqe_dev) -> bool;
}
//
// genwqe_is_privileged() - Determine operation mode for PCI function
//
// On Intel with SRIOV support we see:
// PF: is_physfn = 1 is_virtfn = 0
// VF: is_physfn = 0 is_virtfn = 1
//
// On Systems with no SRIOV support _and_ virtualized systems we get:
// is_physfn = 0 is_virtfn = 0
//
// Other vendors have individual pci device ids to distinguish between
// virtual function drivers and physical function drivers. GenWQE
// unfortunately has just on pci device id for both, VFs and PF.
//
// The following code is used to distinguish if the card is running in
// privileged mode, either as true PF or in a virtualized system with
// full register access e.g. currently on PowerPC.
//
// if (pci_dev->is_virtfn)
// cd->is_privileged = 0;
// else
// cd->is_privileged = (__genwqe_readq(cd, IO_SLU_BITSTREAM)
// != IO_ILLEGAL_VALUE);
//
