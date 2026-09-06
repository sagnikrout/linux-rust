//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/megaraid/megaraid_mbox.h
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
// FILE		: megaraid_mbox.h
//

//
// Define some PCI values here until they are put in the kernel
//
pub const PCI_DEVICE_ID_PERC4_DI_DISCOVERY: c_uint = 0x000E;
pub const PCI_SUBSYS_ID_PERC4_DI_DISCOVERY: c_uint = 0x0123;
pub const PCI_DEVICE_ID_PERC4_SC: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_PERC4_SC: c_uint = 0x0520;
pub const PCI_DEVICE_ID_PERC4_DC: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_PERC4_DC: c_uint = 0x0518;
pub const PCI_DEVICE_ID_VERDE: c_uint = 0x0407;
pub const PCI_DEVICE_ID_PERC4_DI_EVERGLADES: c_uint = 0x000F;
pub const PCI_SUBSYS_ID_PERC4_DI_EVERGLADES: c_uint = 0x014A;
pub const PCI_DEVICE_ID_PERC4E_SI_BIGBEND: c_uint = 0x0013;
pub const PCI_SUBSYS_ID_PERC4E_SI_BIGBEND: c_uint = 0x016c;
pub const PCI_DEVICE_ID_PERC4E_DI_KOBUK: c_uint = 0x0013;
pub const PCI_SUBSYS_ID_PERC4E_DI_KOBUK: c_uint = 0x016d;
pub const PCI_DEVICE_ID_PERC4E_DI_CORVETTE: c_uint = 0x0013;
pub const PCI_SUBSYS_ID_PERC4E_DI_CORVETTE: c_uint = 0x016e;
pub const PCI_DEVICE_ID_PERC4E_DI_EXPEDITION: c_uint = 0x0013;
pub const PCI_SUBSYS_ID_PERC4E_DI_EXPEDITION: c_uint = 0x016f;
pub const PCI_DEVICE_ID_PERC4E_DI_GUADALUPE: c_uint = 0x0013;
pub const PCI_SUBSYS_ID_PERC4E_DI_GUADALUPE: c_uint = 0x0170;
pub const PCI_DEVICE_ID_DOBSON: c_uint = 0x0408;
pub const PCI_DEVICE_ID_MEGARAID_SCSI_320_0: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_MEGARAID_SCSI_320_0: c_uint = 0xA520;
pub const PCI_DEVICE_ID_MEGARAID_SCSI_320_1: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_MEGARAID_SCSI_320_1: c_uint = 0x0520;
pub const PCI_DEVICE_ID_MEGARAID_SCSI_320_2: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_MEGARAID_SCSI_320_2: c_uint = 0x0518;
pub const PCI_DEVICE_ID_MEGARAID_I4_133_RAID: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_MEGARAID_I4_133_RAID: c_uint = 0x0522;
pub const PCI_DEVICE_ID_MEGARAID_SATA_150_4: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_MEGARAID_SATA_150_4: c_uint = 0x4523;
pub const PCI_DEVICE_ID_MEGARAID_SATA_150_6: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_MEGARAID_SATA_150_6: c_uint = 0x0523;
pub const PCI_DEVICE_ID_LINDSAY: c_uint = 0x0409;
pub const PCI_DEVICE_ID_INTEL_RAID_SRCS16: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_INTEL_RAID_SRCS16: c_uint = 0x0523;
pub const PCI_DEVICE_ID_INTEL_RAID_SRCU41L_LAKE_SHETEK: c_uint = 0x1960;
pub const PCI_SUBSYS_ID_INTEL_RAID_SRCU41L_LAKE_SHETEK: c_uint = 0x0520;
pub const PCI_SUBSYS_ID_PERC3_QC: c_uint = 0x0471;
pub const PCI_SUBSYS_ID_PERC3_DC: c_uint = 0x0493;
pub const PCI_SUBSYS_ID_PERC3_SC: c_uint = 0x0475;
pub const PCI_SUBSYS_ID_CERC_ATA100_4CH: c_uint = 0x0511;

pub const MBOX_SYNC_WAIT_CNT: c_uint = 0xFFFF	// wait loop index for synchronous mode;

//
// maximum transfer that can happen through the firmware commands issued
// internnaly from the driver.
//
pub const MBOX_IBUF_SIZE: c_int = 4096;
//
// mbox_ccb_t - command control block specific to mailbox based controllers
// @raw_mbox		: raw mailbox pointer
// @mbox		: mailbox
// @mbox64		: extended mailbox
// @mbox_dma_h		: mailbox dma address
// @sgl64		: 64-bit scatter-gather list
// @sgl32		: 32-bit scatter-gather list
// @sgl_dma_h		: dma handle for the scatter-gather list
// @pthru		: passthru structure
// @pthru_dma_h		: dma handle for the passthru structure
// @epthru		: extended passthru structure
// @epthru_dma_h	: dma handle for extended passthru structure
// @buf_dma_h		: dma handle for buffers w/o sg list
//
// command control block specific to the mailbox based controllers
//
// mraid_device_t - adapter soft state structure for mailbox controllers
// @una_mbox64			: 64-bit mbox - unaligned
// @una_mbox64_dma		: mbox dma addr - unaligned
// @mbox			: 32-bit mbox - aligned
// @mbox64			: 64-bit mbox - aligned
// @mbox_dma			: mbox dma addr - aligned
// @mailbox_lock		: exclusion lock for the mailbox
// @baseport			: base port of hba memory
// @baseaddr			: mapped addr of hba memory
// @mbox_pool			: pool of mailboxes
// @mbox_pool_handle		: handle for the mailbox pool memory
// @epthru_pool			: a pool for extended passthru commands
// @epthru_pool_handle		: handle to the pool above
// @sg_pool			: pool of scatter-gather lists for this driver
// @sg_pool_handle		: handle to the pool above
// @ccb_list			: list of our command control blocks
// @uccb_list			: list of cmd control blocks for mgmt module
// @umbox64			: array of mailbox for user commands (cmm)
// @pdrv_state			: array for state of each physical drive.
// @last_disp			: flag used to show device scanning
// @hw_error			: set if FW not responding
// @fast_load			: If set, skip physical device scanning
// @channel_class		: channel class, RAID or SCSI
// @sysfs_mtx			: mutex to serialize access to sysfs res.
// @sysfs_uioc			: management packet to issue FW calls from sysfs
// @sysfs_mbox64		: mailbox packet to issue FW calls from sysfs
// @sysfs_buffer		: data buffer for FW commands issued from sysfs
// @sysfs_buffer_dma		: DMA buffer for FW commands issued from sysfs
// @sysfs_wait_q		: wait queue for sysfs operations
// @random_del_supported	: set if the random deletion is supported
// @curr_ldmap			: current LDID map
//
// Initialization structure for mailbox controllers: memory based and IO based
// All the fields in this structure are LLD specific and may be discovered at
// init() or start() time.
//
// NOTE: The fields of this structures are placed to minimize cache misses
//
pub const MAX_LD_EXTENDED64: c_int = 64;
// route to raid device from adapter

// Find out if this channel is a RAID or SCSI

