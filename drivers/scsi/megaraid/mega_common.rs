//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/megaraid/mega_common.h
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
// Linux MegaRAID device driver
//
// Copyright (c) 2003-2004  LSI Logic Corporation.
//
// FILE		: mega_common.h
//
// Libaray of common routine used by all low-level megaraid drivers
//

pub const LSI_MAX_CHANNELS: c_int = 16;

pub const HBA_SIGNATURE_64_BIT: c_uint = 0x299;
pub const PCI_CONF_AMISIG64: c_uint = 0xa4;
pub const MEGA_SCSI_INQ_EVPD: c_int = 1;
pub const MEGA_INVALID_FIELD_IN_CDB: c_uint = 0x24;
//
// scb_t - scsi command control block
// @ccb			: command control block for individual driver
// @list		: list of control blocks
// @gp			: general purpose field for LLDs
// @sno			: all SCBs have a serial number
// @scp			: associated scsi command
// @state		: current state of scb
// @dma_dir		: direction of data transfer
// @dma_type		: transfer with sg list, buffer, or no data transfer
// @dev_channel		: actual channel on the device
// @dev_target		: actual target on the device
// @status		: completion status
//
// This is our central data structure to issue commands the each driver.
// Driver specific data structures are maintained in the ccb field.
// scb provides a field 'gp', which can be used by LLD for its own purposes
//
// dev_channel and dev_target must be initialized with the actual channel and
// target on the controller.
//
// SCB states as it transitions from one state to another
//
pub const SCB_FREE: c_uint = 0x0000	/* on the free list */;
pub const SCB_ACTIVE: c_uint = 0x0001	/* off the free list */;
pub const SCB_PENDQ: c_uint = 0x0002	/* on the pending queue */;
pub const SCB_ISSUED: c_uint = 0x0004	/* issued - owner f/w */;
pub const SCB_ABORT: c_uint = 0x0008	/* Got an abort for this one */;
pub const SCB_RESET: c_uint = 0x0010	/* Got a reset for this one */;
//
// DMA types for scb
//
pub const MRAID_DMA_NONE: c_uint = 0x0000	/* no data transfer for this command */;
pub const MRAID_DMA_WSG: c_uint = 0x0001	/* data transfer using a sg list */;
pub const MRAID_DMA_WBUF: c_uint = 0x0002	/* data transfer using a contiguous buffer */;
//
// struct adapter_t - driver's initialization structure
// @aram dpc_h			: tasklet handle
// @pdev			: pci configuration pointer for kernel
// @host			: pointer to host structure of mid-layer
// @lock			: synchronization lock for mid-layer and driver
// @quiescent			: driver is quiescent for now.
// @outstanding_cmds		: number of commands pending in the driver
// @kscb_list			: pointer to the bulk of SCBs pointers for IO
// @kscb_pool			: pool of free scbs for IO
// @kscb_pool_lock		: lock for pool of free scbs
// @pend_list			: pending commands list
// @pend_list_lock		: exclusion lock for pending commands list
// @completed_list		: list of completed commands
// @completed_list_lock		: exclusion lock for list of completed commands
// @sglen			: max sg elements supported
// @device_ids			: to convert kernel device addr to our devices.
// @raid_device			: raid adapter specific pointer
// @max_channel			: maximum channel number supported - inclusive
// @max_target			: max target supported - inclusive
// @max_lun			: max lun supported - inclusive
// @unique_id			: unique identifier for each adapter
// @irq				: IRQ for this adapter
// @ito				: internal timeout value, (-1) means no timeout
// @ibuf			: buffer to issue internal commands
// @ibuf_dma_h			: dma handle for the above buffer
// @uscb_list			: SCB pointers for user cmds, common mgmt module
// @uscb_pool			: pool of SCBs for user commands
// @uscb_pool_lock		: exclusion lock for these SCBs
// @max_cmds			: max outstanding commands
// @fw_version			: firmware version
// @bios_version		: bios version
// @max_cdb_sz			: biggest CDB size supported.
// @ha				: is high availability present - clustering
// @init_id			: initiator ID, the default value should be 7
// @max_sectors			: max sectors per request
// @cmd_per_lun			: max outstanding commands per LUN
// @being_detached		: set when unloading, no more mgmt calls
//
// mraid_setup_device_map() can be called anytime after the device map is
// available and MRAID_GET_DEVICE_MAP() can be called whenever the mapping is
// required, usually from LLD's queue entry point. The formar API sets up the
// MRAID_IS_LOGICAL(adapter_t *, struct scsi_cmnd *) to find out if the
// device in question is a logical drive.
//
// quiescent flag should be set by the driver if it is not accepting more
// commands
//
// NOTE: The fields of this structures are placed to minimize cache misses
//
// amount of space required to store the bios and firmware version strings
pub const VERSION_SIZE: c_int = 16;

// conversion from scsi command

// generic macro to convert scsi command and host to controller's soft state

//
// MRAID_GET_DEVICE_MAP - device ids
// @adp			: adapter's soft state
// @scp			: mid-layer scsi command pointer
// @p_chan		: physical channel on the controller
// @target		: target id of the device or logical drive number
// @islogical		: set if the command is for the logical drive
//
// Macro to retrieve information about device class, logical or physical and
// the corresponding physical channel and target or logical drive number
//

// \
// Is the request coming for the virtual channel		\
// \
// Get an index into our table of drive ids mapping		\
// \
//
// ### Helper routines ###
//

// mraid_debug_level

// Macro flag: #define ASSERT(expression)

//
// struct mraid_pci_blk - structure holds DMA memory block info
// @vaddr		: virtual address to a memory block
// @dma_addr		: DMA handle to a memory block
//
// This structure is filled up for the caller. It is the responsibilty of the
// caller to allocate this array big enough to store addresses for all
// requested elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mraid_pci_blk {
    pub vaddr: caddr_t,
    pub dma_addr: dma_addr_t,
}
