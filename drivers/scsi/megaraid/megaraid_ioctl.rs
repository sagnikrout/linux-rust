//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/megaraid/megaraid_ioctl.h
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
// FILE		: megaraid_ioctl.h
//
// Definitions to interface with user level applications
//

//
// console messages debug levels
//

//
// con_log() - console log routine
// @level		: indicates the severity of the message.
// @fmt			: format string
//
// con_log displays the error messages on the console based on the current
// debug level. Also it attaches the appropriate kernel severity level with
// the message.
//

//
// Definitions & Declarations needed to use common management module
//

pub const USCSICMD: c_uint = 0x80;
pub const UIOC_RD: c_uint = 0x00001;
pub const UIOC_WR: c_uint = 0x00002;
pub const MBOX_CMD: c_uint = 0x00000;
pub const GET_DRIVER_VER: c_uint = 0x10000;
pub const GET_N_ADAP: c_uint = 0x20000;
pub const GET_ADAP_INFO: c_uint = 0x30000;
pub const GET_CAP: c_uint = 0x40000;
pub const GET_STATS: c_uint = 0x50000;
pub const GET_IOCTL_VERSION: c_uint = 0x01;
pub const EXT_IOCTL_SIGN_SZ: c_int = 16;

pub const MBOX_LEGACY: c_uint = 0x00		/* ioctl has legacy mbox*/;
pub const MBOX_HPE: c_uint = 0x01		/* ioctl has hpe mbox	*/;
pub const APPTYPE_MIMD: c_uint = 0x00		/* old existing apps	*/;
pub const APPTYPE_UIOC: c_uint = 0x01		/* new apps using uioc	*/;
pub const IOCTL_ISSUE: c_uint = 0x00000001	/* Issue ioctl		*/;
pub const IOCTL_ABORT: c_uint = 0x00000002	/* Abort previous ioctl	*/;
pub const DRVRTYPE_MBOX: c_uint = 0x00000001	/* regular mbox driver	*/;
pub const DRVRTYPE_HPE: c_uint = 0x00000002	/* new hpe driver	*/;

//
// struct uioc_t - the common ioctl packet structure
//
// @signature	: Must be "$$_EXTD_IOCTL_$$"
// @mb_type	: Type of the mail box (MB_LEGACY or MB_HPE)
// @app_type	: Type of the issuing application (existing or new)
// @opcode	: Opcode of the command
// @adapno	: Adapter number
// @cmdbuf	: Pointer to buffer - can point to mbox or plain data buffer
// @xferlen	: xferlen for DCMD and non mailbox commands
// @data_dir	: Direction of the data transfer
// @status	: Status from the driver
// @reserved	: reserved bytes for future expansion
//
// @user_data	: user data transfer address is saved in this
// @user_data_len: length of the data buffer sent by user app
// @user_pthru	: user passthru address is saves in this (null if DCMD)
// @pthru32	: kernel address passthru (allocated per kioc)
// @pthru32_h	: physicall address of @pthru32
// @list	: for kioc free pool list maintenance
// @done	: call back routine for llds to call when kioc is completed
// @buf_vaddr	: dma pool buffer attached to kioc for data transfer
// @buf_paddr	: physical address of the dma pool buffer
// @pool_index	: index of the dma pool that @buf_vaddr is taken from
// @free_buf	: indicates if buffer needs to be freed after kioc completes
//
// Note		: All LSI drivers understand only this packet. Any other
// : format sent by applications would be converted to this.
//
// User Apps:
// Driver Data:
// 64bit alignment
// For on-stack uioc timers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uioc_timeout {
    pub timer: timer_list,
    pub uioc: *mut uioc_t,
}

//
// struct mraid_hba_info - information about the controller
//
// @pci_vendor_id		: PCI vendor id
// @pci_device_id		: PCI device id
// @subsystem_vendor_id		: PCI subsystem vendor id
// @subsystem_device_id		: PCI subsystem device id
// @baseport			: base port of hba memory
// @pci_bus			: PCI bus
// @pci_dev_fn			: PCI device/function values
// @irq				: interrupt vector for the device
//
// Extended information of 256 bytes about the controller. Align on the single
// byte boundary so that 32-bit applications can be run on 64-bit platform
// drivers withoug re-compilation.
// NOTE: reduce the number of reserved bytes whenever new field are added, so
// that total size of the structure remains 256 bytes.
//
// mcontroller	: adapter info structure for old mimd_t apps
//
// @base	: base address
// @irq		: irq number
// @numldrv	: number of logical drives
// @pcibus	: pci bus
// @pcidev	: pci device
// @pcifun	: pci function
// @pciid	: pci id
// @pcivendor	: vendor id
// @pcislot	: slot number
// @uid		: unique id
//
// mm_dmapool_t	: Represents one dma pool with just one buffer
//
// @vaddr	: Virtual address
// @paddr	: DMA physicall address
// @bufsize	: In KB - 4 = 4k, 8 = 8k etc.
// @handle	: Handle to the dma pool
// @lock	: lock to synchronize access to the pool
// @in_use	: If pool already in use, attach new block
//
// mraid_mmadp_t: Structure that drivers pass during (un)registration
//
// @unique_id		: Any unique id (usually PCI bus+dev+fn)
// @drvr_type		: megaraid or hpe (DRVRTYPE_MBOX or DRVRTYPE_HPE)
// @drv_data		: Driver specific; not touched by the common module
// @timeout		: timeout for issued kiocs
// @max_kioc		: Maximum ioctl packets acceptable by the lld
// @pdev		: pci dev; used for allocating dma'ble memory
// @issue_uioc		: Driver supplied routine to issue uioc_t commands
// : issue_uioc(drvr_data, kioc, ISSUE/ABORT, uioc_done)
// @quiescent		: flag to indicate if ioctl can be issued to this adp
// @list		: attach with the global list of adapters
// @kioc_list		: block of mem for @max_kioc number of kiocs
// @kioc_pool		: pool of free kiocs
// @kioc_pool_lock	: protection for free pool
// @kioc_semaphore	: so as not to exceed @max_kioc parallel ioctls
// @mbox_list		: block of mem for @max_kioc number of mboxes
// @pthru_dma_pool	: DMA pool to allocate passthru packets
// @dma_pool_list	: array of dma pools
//
// Filled by driver
// Maintained by common module
extern "C" {
    pub fn mraid_mm_register_adp(: *mut mraid_mmadp_t) -> c_int;
}
extern "C" {
    pub fn mraid_mm_unregister_adp(_arg: u32) -> c_int;
}
extern "C" {
    pub fn mraid_mm_adapter_app_handle(_arg: u32) -> u32;
}
