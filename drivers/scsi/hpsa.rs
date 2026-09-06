//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/hpsa.h
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
// Disk Array driver for HP Smart Array SAS controllers
// Copyright (c) 2019-2020 Microchip Technology Inc. and its subsidiaries
// Copyright 2016 Microsemi Corporation
// Copyright 2014-2015 PMC-Sierra, Inc.
// Copyright 2000,2009-2015 Hewlett-Packard Development Company, L.P.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more details.
//
// Questions/Comments/Bugfixes to esc.storagedev@microsemi.com
//

pub const IO_OK: c_int = 0;
pub const IO_ERROR: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_method {
    pub c): *mut CommandList,
    pub val): *mut *mut *mut void (set_intr_mask)(struct ctlr_info h, unsigned long,
    pub h): *mut *mut bool (intr_pending)(struct ctlr_info,
    pub q): *mut *mut *mut unsigned long (command_completed)(struct ctlr_info h, u8,
}

// for SAS hosts and SAS expanders
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpsa_sas_node {
    pub parent_dev: *mut device,
    pub port_list_head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpsa_sas_port {
    pub port_list_entry: list_head,
    pub sas_address: u64,
    pub port: *mut sas_port,
    pub next_phy_index: c_int,
    pub phy_list_head: list_head,
    pub parent_node: *mut hpsa_sas_node,
    pub rphy: *mut sas_rphy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpsa_sas_phy {
    pub phy_list_entry: list_head,
    pub phy: *mut sas_phy,
    pub parent_port: *mut hpsa_sas_port,
    pub added_to_port: bool,
}

pub const EXTERNAL_QD: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpsa_scsi_dev_t {
    pub devtype: c_uint,
    pub /: *mut *mut int bus, target, lun; / as presented to the OS,
    pub /: *mut *mut unsigned char scsi3addr[8]; / as presented to the HW,
    pub 1: u8 physical_device :,
    pub expose_device: u8,
    pub /: *mut *mut u8 removed : 1; / device is marked for death,
    pub /: *mut *mut u8 was_removed : 1; / device actually removed,

    pub /: *mut *mut unsigned char device_id[16]; / from inquiry pg. 0x83,
    pub sas_address: u64,
    pub /: *mut *mut u64 eli; / from report diags.,
    pub /: *mut *mut unsigned char vendor[8]; / bytes 8-15 of inquiry data,
    pub /: *mut *mut unsigned char model[16]; / bytes 16-31 of inquiry data,
    pub /: *mut *mut unsigned char rev; / byte 2 of inquiry data,
    pub /: *mut *mut unsigned char raid_level; / from inquiry page 0xC1,
    pub /: *mut *mut unsigned char volume_offline; / discovered via TUR or VPD,
    pub /: *mut *mut u16 queue_depth; / max queue_depth for this device,
    pub /: *mut *mut atomic_t commands_outstanding; / track commands sent to device,
    pub devices: *mut *mut atomic_t ioaccel_cmds_out; / Only used for physical,
// counts commands sent to physical
// device via "ioaccel" path.
//
    pub in_reset: bool,
    pub ioaccel_handle: u32,
    pub active_path_index: u8,
    pub path_map: u8,
    pub bay: u8,
    pub box: [u8; 8],
    pub phys_connector: [u16; 8],
    pub /: *mut *mut int offload_config; / I/O accel RAID offload configured,
    pub /: *mut *mut int offload_enabled; / I/O accel RAID offload enabled,
    pub offload_to_be_enabled: c_int,
    pub hba_ioaccel_enabled: c_int,
    pub RAID: *mut *mut int offload_to_mirror; / Send next I/O accelerator,
// offload request to mirror drive
//
    pub /: *mut *mut raid_map_data raid_map; / I/O accelerator RAID map,
//
// Pointers from logical drive map indices to the phys drives that
// make those logical drives.  Note, multiple logical drives may
// share physical drives.  You can have for instance 5 physical
// drives with 3 logical drives each using those same 5 physical
// disks. We need these pointers for counting i/o's out to physical
// devices in order to honor physical device queue depth limits.
//
    pub phys_disk: [*mut hpsa_scsi_dev_t; RAID_MAP_MAX_ENTRIES],
    pub nphysical_disks: c_int,
    pub supports_aborts: c_int,
    pub sas_port: *mut hpsa_sas_port,
    pub /: *mut *mut int external; / 1-from external array 0-not <0-unknown,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reply_queue_buffer {
    pub head: *mut u64,
    pub size: usize,
    pub wraparound: u8,
    pub current_entry: u32,
    pub busaddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmic_controller_parameters {
    pub led_flags: u8,
    pub enable_command_list_verification: u8,
    pub backed_out_write_drives: u8,
    pub stripes_for_parity: u16,
    pub parity_distribution_mode_flags: u8,
    pub max_driver_requests: u16,
    pub elevator_trend_count: u16,
    pub disable_elevator: u8,
    pub force_scan_complete: u8,
    pub scsi_transfer_mode: u8,
    pub force_narrow: u8,
    pub rebuild_priority: u8,
    pub expand_priority: u8,
    pub host_sdb_asic_fix: u8,
    pub pdpi_burst_from_host_disabled: u8,
    pub software_name: [c_char; 64],
    pub hardware_name: [c_char; 32],
    pub bridge_revision: u8,
    pub snapshot_priority: u8,
    pub os_specific: u32,
    pub post_prompt_timeout: u8,
    pub automatic_drive_slamming: u8,
    pub reserved1: u8,
    pub nvram_flags: u8,
    pub cache_nvram_flags: u8,
    pub drive_config_flags: u8,
    pub reserved2: u16,
    pub temp_warning_level: u8,
    pub temp_shutdown_level: u8,
    pub temp_condition_reset: u8,
    pub max_coalesce_commands: u8,
    pub max_coalesce_delay: u32,
    pub orca_password: [u8; 4],
    pub access_id: [u8; 16],
    pub reserved: [u8; 356],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctlr_info {
    pub reply_map: *mut c_uint,
    pub ctlr: c_int,
    pub devname: [c_char; 16],
    pub product_name: *mut c_char,
    pub pdev: *mut pci_dev,
    pub board_id: u32,
    pub sas_address: u64,
    pub vaddr: *mut void __iomem,
    pub paddr: c_ulong,
    pub /: *mut *mut int nr_cmds; / Number of commands allowed on this controller,
pub const HPSA_CMDS_RESERVED_FOR_ABORTS: c_int = 2;
pub const HPSA_CMDS_RESERVED_FOR_DRIVER: c_int = 1;
    pub cfgtable: *mut CfgTable __iomem,
    pub interrupts_enabled: c_int,
    pub max_commands: c_int,
    pub /: *mut *mut int last_collision_tag; / tags are global,
    pub commands_outstanding: core::sync::atomic::AtomicI32,

    pub msix_vectors: c_uint,
    pub /: *mut *mut int intr_mode; / either PERF_MODE_INT or SIMPLE_MODE_INT,
    pub access: access_method,
// queue and queue Info
    pub Qdepth: c_uint,
    pub maxSG: c_uint,
    pub lock: spinlock_t,
    pub maxsgentries: c_int,
    pub max_cmd_sg_entries: u8,
    pub chainsize: c_int,
    pub cmd_sg_list: *mut SGDescriptor,
    pub ioaccel2_cmd_sg_list: *mut ioaccel2_sg_element,
// pointers to command and error info pool
    pub cmd_pool: *mut CommandList,
    pub cmd_pool_dhandle: dma_addr_t,
    pub ioaccel_cmd_pool: *mut io_accel1_cmd,
    pub ioaccel_cmd_pool_dhandle: dma_addr_t,
    pub ioaccel2_cmd_pool: *mut io_accel2_cmd,
    pub ioaccel2_cmd_pool_dhandle: dma_addr_t,
    pub errinfo_pool: *mut ErrorInfo,
    pub errinfo_pool_dhandle: dma_addr_t,
    pub cmd_pool_bits: *mut c_ulong,
    pub scan_finished: c_int,
    pub 1: u8 scan_waiting :,
    pub scan_lock: spinlock_t,
    pub scan_wait_queue: wait_queue_head_t,
    pub scsi_host: *mut Scsi_Host,
    pub /: *mut *mut spinlock_t devlock; / to protect hba[ctlr]->dev[];,
    pub /: *mut *mut int ndevices; / number of used elements in .dev[] array.,
    pub dev: [*mut hpsa_scsi_dev_t; HPSA_MAX_DEVICES],
//
// Performant mode tables.
//
    pub trans_support: u32,
    pub trans_offset: u32,
    pub transtable: *mut TransTable___iomem,
    pub transMethod: c_ulong,
// cap concurrent passthrus at some reasonable maximum

    pub passthru_cmds_avail: core::sync::atomic::AtomicI32,
//
// Performant mode completion buffers
//
    pub reply_queue_size: usize,
    pub reply_queue: [reply_queue_buffer; MAX_REPLY_QUEUES],
    pub nreply_queues: u8,
    pub blockFetchTable: *mut u32,
    pub ioaccel1_blockFetchTable: *mut u32,
    pub ioaccel2_blockFetchTable: *mut u32,
    pub ioaccel2_bft2_regs: *mut u32 __iomem,
    pub hba_inquiry_data: *mut c_uchar,
    pub driver_support: u32,
    pub fw_support: u32,
    pub ioaccel_support: c_int,
    pub ioaccel_maxsg: c_int,
    pub last_intr_timestamp: u64,
    pub last_heartbeat: u32,
    pub last_heartbeat_timestamp: u64,
    pub heartbeat_sample_interval: u32,
    pub firmware_flash_in_progress: core::sync::atomic::AtomicI32,
    pub lockup_detected: *mut u32 __percpu,
    pub monitor_ctlr_work: delayed_work,
    pub rescan_ctlr_work: delayed_work,
    pub event_monitor_work: delayed_work,
    pub remove_in_progress: c_int,
// Address of h->q[x] is passed to intr handler to know which queue
    pub q: [u8; MAX_REPLY_QUEUES],
    pub /: *mut *mut char intrname[MAX_REPLY_QUEUES][32]; / controller and IRQ names,
    pub /: *mut *mut u32 TMFSupportFlags; / cache what task mgmt funcs are supported.,

    pub events: u32,

    pub offline_device_lock: spinlock_t,
    pub offline_device_list: list_head,
    pub acciopath_status: c_int,
    pub drv_req_rescan: c_int,
    pub raid_offload_debug: c_int,
    pub discovery_polling: c_int,
    pub legacy_board: c_int,
    pub lastlogicals: *mut ReportLUNdata,
    pub needs_abort_tags_swizzled: c_int,
    pub resubmit_wq: *mut workqueue_struct,
    pub rescan_ctlr_wq: *mut workqueue_struct,
    pub monitor_ctlr_wq: *mut workqueue_struct,
    pub abort_cmds_available: core::sync::atomic::AtomicI32,
    pub event_sync_wait_queue: wait_queue_head_t,
    pub reset_mutex: mutex,
    pub reset_in_progress: u8,
    pub sas_host: *mut hpsa_sas_node,
    pub reset_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct offline_device_entry {
    pub scsi3addr: [c_uchar; 8],
    pub offline_list: list_head,
}

pub const HPSA_ABORT_MSG: c_int = 0;
pub const HPSA_DEVICE_RESET_MSG: c_int = 1;
pub const HPSA_RESET_TYPE_CONTROLLER: c_uint = 0x00;
pub const HPSA_RESET_TYPE_BUS: c_uint = 0x01;
pub const HPSA_RESET_TYPE_LUN: c_uint = 0x04;
pub const HPSA_PHYS_TARGET_RESET: c_uint = 0x99 /* not defined by cciss spec */;
pub const HPSA_MSG_SEND_RETRY_LIMIT: c_int = 10;

// Maximum time in seconds driver will wait for command completions
// when polling before giving up.
//

// During SCSI error recovery, HPSA_TUR_RETRY_LIMIT defines
// how many times to retry TEST UNIT READY on a device
// while waiting for it to become ready before giving up.
// HPSA_MAX_WAIT_INTERVAL_SECS is the max wait interval
// between sending TURs while waiting for a device
// to become ready.
//

// HPSA_BOARD_READY_WAIT_SECS is how long to wait for a board
// to become ready, in seconds, before giving up on it.
// HPSA_BOARD_READY_POLL_INTERVAL_MSECS * is how long to wait
// between polling the board to see if it is ready, in
// milliseconds.  HPSA_BOARD_READY_POLL_INTERVAL and
// HPSA_BOARD_READY_ITERATIONS are derived from those.
//

// Defining the diffent access_menthods
//
// Memory mapped FIFO interface (SMART 53xx cards)
//
pub const SA5_DOORBELL: c_uint = 0x20;
pub const SA5_REQUEST_PORT_OFFSET: c_uint = 0x40;
pub const SA5_REQUEST_PORT64_LO_OFFSET: c_uint = 0xC0;
pub const SA5_REQUEST_PORT64_HI_OFFSET: c_uint = 0xC4;
pub const SA5_REPLY_INTR_MASK_OFFSET: c_uint = 0x34;
pub const SA5_REPLY_PORT_OFFSET: c_uint = 0x44;
pub const SA5_INTR_STATUS: c_uint = 0x30;
pub const SA5_SCRATCHPAD_OFFSET: c_uint = 0xB0;
pub const SA5_CTCFG_OFFSET: c_uint = 0xB4;
pub const SA5_CTMEM_OFFSET: c_uint = 0xB8;
pub const SA5_INTR_OFF: c_uint = 0x08;
pub const SA5B_INTR_OFF: c_uint = 0x04;
pub const SA5_INTR_PENDING: c_uint = 0x08;
pub const SA5B_INTR_PENDING: c_uint = 0x04;
pub const FIFO_EMPTY: c_uint = 0xffffffff;
pub const HPSA_FIRMWARE_READY: c_uint = 0xffff0000 /* value in scratchpad register */;
pub const HPSA_ERROR_BIT: c_uint = 0x02;
// Performant mode flags
pub const SA5_PERF_INTR_PENDING: c_uint = 0x04;
pub const SA5_PERF_INTR_OFF: c_uint = 0x05;
pub const SA5_OUTDB_STATUS_PERF_BIT: c_uint = 0x01;
pub const SA5_OUTDB_CLEAR_PERF_BIT: c_uint = 0x01;
pub const SA5_OUTDB_CLEAR: c_uint = 0xA0;
pub const SA5_OUTDB_CLEAR_PERF_BIT: c_uint = 0x01;
pub const SA5_OUTDB_STATUS: c_uint = 0x9C;
pub const HPSA_INTR_ON: c_int = 1;
pub const HPSA_INTR_OFF: c_int = 0;
//
// Inbound Post Queue offsets for IO Accelerator Mode 2
//
pub const IOACCEL2_INBOUND_POSTQ_32: c_uint = 0x48;
pub const IOACCEL2_INBOUND_POSTQ_64_LOW: c_uint = 0xd0;
pub const IOACCEL2_INBOUND_POSTQ_64_HI: c_uint = 0xd4;
pub const HPSA_PHYSICAL_DEVICE_BUS: c_int = 0;
pub const HPSA_RAID_VOLUME_BUS: c_int = 1;
pub const HPSA_EXTERNAL_RAID_VOLUME_BUS: c_int = 2;
pub const HPSA_HBA_BUS: c_int = 0;
pub const HPSA_LEGACY_HBA_BUS: c_int = 3;
//
// This card is the opposite of the other cards.
// 0 turns interrupts on...
// 0x08 turns them off...
//
// Variant of the above; 0x04 turns interrupts off...
//
// msi auto clears the interrupt pending bit.
// flush the controller write of the reply queue by reading
// outbound doorbell status register.
//
// Do a read in order to flush the write to the controller
// (as per spec.)
//
// Check for wraparound
//
// returns value read from hardware.
// returns FIFO_EMPTY if there is nothing to read
//

//
// Returns true if an interrupt is pending..
//
// Read outbound doorbell to flush
pub const SA5_IOACCEL_MODE1_INTR_STATUS_CMP_BIT: c_uint = 0x100;
//
// Returns true if an interrupt is pending..
//
pub const IOACCEL_MODE1_REPLY_QUEUE_INDEX: c_uint = 0x1A0;
pub const IOACCEL_MODE1_PRODUCER_INDEX: c_uint = 0x1B8;
pub const IOACCEL_MODE1_CONSUMER_INDEX: c_uint = 0x1BC;
pub const IOACCEL_MODE1_REPLY_UNUSED: c_uint = 0xFFFFFFFFFFFFFFFFULL;
//
// @todo
//
// Don't really need to write the new index after each command,
// but with current driver design this is easiest.
//
// Duplicate entry of the above to mark unsupported boards
#[repr(C)]
#[derive(Copy, Clone)]
pub struct board_type {
    pub board_id: u32,
    pub product_name: *mut c_char,
    pub access: *mut access_method,
}
