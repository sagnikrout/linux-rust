//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/mtip32xx/mtip32xx.h
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
// mtip32xx.h - Header file for the P320 SSD Block Driver
// Copyright (C) 2011 Micron Technology, Inc.
//
// Portions of this code were derived from works subjected to the
// following copyright:
// Copyright (C) 2009 Integrated Device Technology, Inc.
//

// Offset of Subsystem Device ID in pci confoguration space
pub const PCI_SUBSYSTEM_DEVICEID: c_uint = 0x2E;
// offset of Device Control register in PCIe extended capabilites space
pub const PCIE_CONFIG_EXT_DEVICE_CONTROL_OFFSET: c_uint = 0x48;
// check for erase mode support during secure erase
pub const MTIP_SEC_ERASE_MODE: c_uint = 0x2;
// # of times to retry timed out/failed IOs
pub const MTIP_MAX_RETRIES: c_int = 2;
// Various timeout values in ms
pub const MTIP_NCQ_CMD_TIMEOUT_MS: c_int = 15000;
pub const MTIP_IOCTL_CMD_TIMEOUT_MS: c_int = 5000;
pub const MTIP_INT_CMD_TIMEOUT_MS: c_int = 5000;

// check for timeouts every 500ms
pub const MTIP_TIMEOUT_CHECK_PERIOD: c_int = 500;
// ftl rebuild
pub const MTIP_FTL_REBUILD_OFFSET: c_int = 142;
pub const MTIP_FTL_REBUILD_MAGIC: c_uint = 0xED51;
pub const MTIP_FTL_REBUILD_TIMEOUT_MS: c_int = 2400000;
// unaligned IO handling
pub const MTIP_MAX_UNALIGNED_SLOTS: c_int = 2;
// Macro to extract the tag bit number from a tag value.

//
// Macro to extract the tag index from a tag value. The index
// is used to access the correct s_active/Command Issue register based
// on the tag value.
//

//
// Maximum number of scatter gather entries
// a single command may have.
//
pub const MTIP_MAX_SG: c_int = 504;
//
// Maximum number of slot groups (Command Issue & s_active registers)
// NOTE: This is the driver maximum; check dd->slot_groups for actual value.
//
pub const MTIP_MAX_SLOT_GROUPS: c_int = 8;
// Internal command tag.
pub const MTIP_TAG_INTERNAL: c_int = 0;
// Micron Vendor ID & P320x SSD Device ID
pub const PCI_VENDOR_ID_MICRON: c_uint = 0x1344;
pub const P320H_DEVICE_ID: c_uint = 0x5150;
pub const P320M_DEVICE_ID: c_uint = 0x5151;
pub const P320S_DEVICE_ID: c_uint = 0x5152;
pub const P325M_DEVICE_ID: c_uint = 0x5153;
pub const P420H_DEVICE_ID: c_uint = 0x5160;
pub const P420M_DEVICE_ID: c_uint = 0x5161;
pub const P425M_DEVICE_ID: c_uint = 0x5163;
// Driver name and version strings

// Maximum number of minor device numbers per device.
pub const MTIP_MAX_MINORS: c_int = 16;
// Maximum number of supported command slots.

//
// Per-tag bitfield size in longs.
// Linux bit manipulation functions
// (i.e. test_and_set_bit, find_next_zero_bit)
// manipulate memory in longs, so we try to make the math work.
// take the slot groups and find the number of longs, rounding up.
// Careful! i386 and x86_64 use different size longs!
//

// BAR number used to access the HBA registers.
pub const MTIP_ABAR: c_int = 5;

pub const MTIP_DFS_MAX_BUF_SIZE: c_int = 1024;
// below are bit numbers in 'flags' defined in mtip_port
// below are bit numbers in 'dd_flag' defined in driver_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smart_attr {
    pub attr_id: u8,
    pub flags: __le16,
    pub cur: u8,
    pub worst: u8,
    pub data: __le32,
    pub res: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtip_work {
    pub work: work_struct,
    pub port: *mut c_void,
    pub cpu_binding: c_int,
    pub completed: u32,
    pub ____cacheline_aligned_in_smp: },

    pub \: *mut *mut *mut mtip_work w = (mtip_work ) work;,
    pub \: mtip_workq_sdbfx(w->port, group, w->completed);,
// Register Frame Information Structure (FIS), host to device.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_to_dev_fis {
//
// FIS type.
// - 27h Register FIS, host to device.
// - 34h Register FIS, device to host.
// - 39h DMA Activate FIS, device to host.
// - 41h DMA Setup FIS, bi-directional.
// - 46h Data FIS, bi-directional.
// - 58h BIST Activate FIS, bi-directional.
// - 5Fh PIO Setup FIS, device to host.
// - A1h Set Device Bits FIS, device to host.
//
    pub type: c_uchar,
    pub opts: c_uchar,
    pub command: c_uchar,
    pub features: c_uchar,
    pub lba_low: c_uchar,
    pub sector: c_uchar,
}

// Command header structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtip_cmd_hdr {
//
// Command options.
// - Bits 31:16 Number of PRD entries.
// - Bits 15:8 Unused in this implementation.
// - Bit 7 Prefetch bit, informs the drive to prefetch PRD entries.
// - Bit 6 Write bit, should be set when writing data to the device.
// - Bit 5 Unused in this implementation.
// - Bits 4:0 Length of the command FIS in DWords (DWord = 4 bytes).
//
    pub opts: __le32,
// This field is unsed when using NCQ.
    pub byte_count: __le32,
    pub status: __le32,
}

//
// Lower 32 bits of the command table address associated with this
// header. The command table addresses must be 128 byte aligned.
//
// If 64 bit addressing is used this field is the upper 32 bits
// of the command table address associated with this command.
//
// Reserved and unused.
// Command scatter gather structure (PRD).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtip_cmd_sg {
//
// Low 32 bits of the data buffer address. For P320 this
// address must be 8 byte aligned signified by bits 2:0 being
// set to 0.
//
    pub dba: __le32,
//
// When 64 bit addressing is used this field is the upper
// 32 bits of the data buffer address.
//
    pub dba_upper: __le32,
// Unused.
    pub reserved: __le32,
//
// Bit 31: interrupt when this data block has been transferred.
// Bits 30..22: reserved
// Bits 21..0: byte count (minus 1).  For P320 the byte count must be
// 8 byte aligned signified by bits 2:0 being set to 1.
//
    pub info: __le32,
}

// Structure used to describe a command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtip_cmd {
    pub /: *mut *mut *mut void command; / ptr to command table entry,
    pub /: *mut *mut dma_addr_t command_dma; / corresponding physical address,
    pub /: *mut *mut int scatter_ents; / Number of scatter list entries used,
    pub /: *mut *mut int unaligned; / command is unaligned on 4k boundary,
    pub /: *mut *mut scatterlist sg[MTIP_MAX_SG]; / Scatter list entries,
    pub icmd: *mut mtip_int_cmd,
}

// Structure used to describe a port.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtip_port {
// Pointer back to the driver data for this port.
    pub dd: *mut driver_data,
//
// Used to determine if the data pointed to by the
// identify field is valid.
//
    pub identify_valid: c_ulong,
// Base address of the memory mapped IO for the port.
    pub mmio: *mut void __iomem,
// Array of pointers to the memory mapped s_active registers.
    pub s_active: [*mut void __iomem; MTIP_MAX_SLOT_GROUPS],
// Array of pointers to the memory mapped completed registers.
    pub completed: [*mut void __iomem; MTIP_MAX_SLOT_GROUPS],
// Array of pointers to the memory mapped Command Issue registers.
    pub cmd_issue: [*mut void __iomem; MTIP_MAX_SLOT_GROUPS],
//
// Pointer to the beginning of the command header memory as used
// by the driver.
//
    pub command_list: *mut c_void,
//
// Pointer to the beginning of the command header memory as used
// by the DMA.
//
    pub command_list_dma: dma_addr_t,
//
// Pointer to the beginning of the RX FIS memory as used
// by the driver.
//
    pub rxfis: *mut c_void,
//
// Pointer to the beginning of the RX FIS memory as used
// by the DMA.
//
    pub rxfis_dma: dma_addr_t,
//
// Pointer to the DMA region for RX Fis, Identify, RLE10, and SMART
//
    pub block1: *mut c_void,
//
// DMA address of region for RX Fis, Identify, RLE10, and SMART
//
    pub block1_dma: dma_addr_t,
//
// Pointer to the beginning of the identify data memory as used
// by the driver.
//
    pub identify: *mut u16,
//
// Pointer to the beginning of the identify data memory as used
// by the DMA.
//
    pub identify_dma: dma_addr_t,
//
// Pointer to the beginning of a sector buffer that is used
// by the driver when issuing internal commands.
//
    pub sector_buffer: *mut u16,
//
// Pointer to the beginning of a sector buffer that is used
// by the DMA when the driver issues internal commands.
//
    pub sector_buffer_dma: dma_addr_t,
    pub log_buf: *mut u16,
    pub log_buf_dma: dma_addr_t,
    pub smart_buf: *mut u8,
    pub smart_buf_dma: dma_addr_t,
//
// used to queue commands when an internal command is in progress
// or error handling is active
//
    pub cmds_to_issue: [c_ulong; SLOTBITS_IN_LONGS],
// Used by mtip_service_thread to wait for an event
    pub svc_wait: wait_queue_head_t,
//
// indicates the state of the port. Also, helps the service thread
// to determine its action on wake up.
//
    pub flags: c_ulong,
//
// Timer used to complete commands that have been active for too long.
//
    pub ic_pause_timer: c_ulong,
// Counter to control queue depth of unaligned IOs
    pub cmd_slot_unal: core::sync::atomic::AtomicI32,
// Spinlock for working around command-issue bug.
    pub cmd_issue_lock: [spinlock_t; MTIP_MAX_SLOT_GROUPS],
}

//
// Driver private data structure.
//
// One structure is allocated per probed device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct driver_data {
    pub /: *mut *mut *mut void __iomem mmio; / Base address of the HBA registers.,
    pub /: *mut *mut int major; / Major device number.,
    pub /: *mut *mut int instance; / Instance number. First device probed is 0, ...,
    pub /: *mut *mut *mut gendisk disk; / Pointer to our gendisk structure.,
    pub /: *mut *mut *mut pci_dev pdev; / Pointer to the PCI device structure.,
    pub /: *mut *mut *mut request_queue queue; / Our request queue.,
    pub /: *mut *mut blk_mq_tag_set tags; / blk_mq tags,
    pub ioctl_mutex: mutex,
    pub /: *mut *mut *mut mtip_port port; / Pointer to the port data structure.,
    pub /: *mut *mut unsigned product_type; / magic value declaring the product type,
    pub /: *mut *mut unsigned slot_groups; / number of slot groups the product supports,
    pub /: *mut *mut unsigned long index; / Index to determine the disk name,
    pub /: *mut *mut unsigned long dd_flag; / NOTE: use atomic bit operations on this,
    pub /: *mut *mut *mut task_mtip_svc_handler; / task_of svc thd,
    pub dfs_node: *mut dentry,
    pub sr: bool,
    pub /: *mut *mut int numa_node; / NUMA support,
    pub workq_name: [c_char; 32],
    pub isr_workq: *mut workqueue_struct,
    pub irq_workers_active: core::sync::atomic::AtomicI32,
    pub work: [mtip_work; MTIP_MAX_SLOT_GROUPS],
    pub isr_binding: c_int,
    pub /: *mut *mut int unal_qdepth; / qdepth of unaligned IO queue,
}
