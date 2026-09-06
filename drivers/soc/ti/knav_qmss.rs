//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/ti/knav_qmss.h
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
// Keystone Navigator QMSS driver internal header
//
// Copyright (C) 2014 Texas Instruments Incorporated - http://www.ti.com
// Author:	Sandeep Nair <sandeep_n@ti.com>
// Cyril Chemparathy <cyril@ti.com>
// Santosh Shilimkar <santosh.shilimkar@ti.com>
//

pub const THRESH_LT: c_int = 0;
pub const PDSP_CTRL_PC_MASK: c_uint = 0xffff0000;

pub const ACC_MAX_CHANNEL: c_int = 48;

pub const ACC_CHANNEL_INT_BASE: c_int = 2;
pub const ACC_LIST_ENTRY_TYPE: c_int = 1;

pub const ACC_LIST_ENTRY_QUEUE_IDX: c_int = 0;

pub const ACC_CMD_DISABLE_CHANNEL: c_uint = 0x80;
pub const ACC_CMD_ENABLE_CHANNEL: c_uint = 0x81;

pub const RANGE_MAX_IRQS: c_int = 64;

pub const DESC_SIZE_MASK: c_uint = 0xful;

pub const KNAV_NAME_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum knav_acc_result {
    ACC_RET_IDLE,
    ACC_RET_SUCCESS,
    ACC_RET_INVALID_COMMAND,
    ACC_RET_INVALID_CHANNEL,
    ACC_RET_INACTIVE_CHANNEL,
    ACC_RET_ACTIVE_CHANNEL,
    ACC_RET_INVALID_QUEUE,
    ACC_RET_INVALID_RET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_reg_config {
    pub revision: u32,
    pub __pad1: u32,
    pub divert: u32,
    pub link_ram_base0: u32,
    pub link_ram_size0: u32,
    pub link_ram_base1: u32,
    pub __pad2: [u32; 2],
    pub starvation: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_reg_region {
    pub base: u32,
    pub start_index: u32,
    pub size_count: u32,
    pub __pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_reg_pdsp_regs {
    pub control: u32,
    pub status: u32,
    pub cycle_count: u32,
    pub stall_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_reg_acc_command {
    pub command: u32,
    pub queue_mask: u32,
    pub list_dma: u32,
    pub queue_num: u32,
    pub timer_config: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_link_ram_block {
    pub dma: dma_addr_t,
    pub virt: *mut c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_acc_info {
    pub pdsp_id: u32,
    pub start_channel: u32,
    pub list_entries: u32,
    pub pacing_mode: u32,
    pub timer_count: u32,
    pub mem_size: c_int,
    pub list_size: c_int,
    pub pdsp: *mut knav_pdsp_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_acc_channel {
    pub channel: u32,
    pub list_index: u32,
    pub open_mask: u32,
    pub list_cpu: [*mut u32; 2],
    pub list_dma: [dma_addr_t; 2],
    pub name: [c_char; KNAV_NAME_SIZE],
    pub retrigger_count: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_pdsp_info {
    pub name: *const c_char,
    pub regs: *mut knav_reg_pdsp_regs __iomem,
    pub command: *mut u32 __iomem,
    pub acc_command: *mut knav_reg_acc_command __iomem,
    pub qos_command: *mut u32 __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_qmgr_info {
    pub start_queue: unsigned,
    pub num_queues: unsigned,
    pub reg_config: *mut knav_reg_config __iomem,
    pub reg_region: *mut knav_reg_region __iomem,
    pub reg_peek: *mut *mut *mut knav_reg_queue __iomem reg_push, reg_pop,,
    pub reg_status: *mut void __iomem,
    pub list: list_head,
}

pub const KNAV_NUM_LINKRAM: c_int = 2;
//
// struct knav_queue_stats:	queue statistics
// pushes:			number of push operations
// pops:			number of pop operations
// push_errors:			number of push errors
// pop_errors:			number of pop errors
// notifies:			notifier counts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_queue_stats {
    pub pushes: c_uint,
    pub pops: c_uint,
    pub push_errors: c_uint,
    pub pop_errors: c_uint,
    pub notifies: c_uint,
}

//
// struct knav_reg_queue:	queue registers
// @entry_count:		valid entries in the queue
// @byte_count:			total byte count in thhe queue
// @packet_size:		packet size for the queue
// @ptr_size_thresh:		packet pointer size threshold
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_reg_queue {
    pub entry_count: u32,
    pub byte_count: u32,
    pub packet_size: u32,
    pub ptr_size_thresh: u32,
}

//
// struct knav_region:		qmss region info
// @dma_start, dma_end:		start and end dma address
// @virt_start, virt_end:	start and end virtual address
// @desc_size:			descriptor size
// @used_desc:			consumed descriptors
// @id:				region number
// @num_desc:			total descriptors
// @link_index:			index of the first descriptor
// @name:			region name
// @list:			instance in the device's region list
// @pools:			list of descriptor pools in the region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_region {
    pub dma_end: dma_addr_t dma_start,,
    pub virt_end: *mut *mut void virt_start,,
    pub desc_size: unsigned,
    pub used_desc: unsigned,
    pub id: unsigned,
    pub num_desc: unsigned,
    pub link_index: unsigned,
    pub name: *const c_char,
    pub list: list_head,
    pub pools: list_head,
}

//
// struct knav_pool:		qmss pools
// @dev:			device pointer
// @region:			qmss region info
// @queue:			queue registers
// @kdev:			qmss device pointer
// @region_offset:		offset from the base
// @num_desc:			total descriptors
// @desc_size:			descriptor size
// @region_id:			region number
// @name:			pool name
// @list:			list head
// @region_inst:		instance in the region's pool list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_pool {
    pub dev: *mut device,
    pub region: *mut knav_region,
    pub queue: *mut knav_queue,
    pub kdev: *mut knav_device,
    pub region_offset: c_int,
    pub num_desc: c_int,
    pub desc_size: c_int,
    pub region_id: c_int,
    pub name: *const c_char,
    pub list: list_head,
    pub region_inst: list_head,
}

//
// struct knav_queue_inst:		qmss queue instance properties
// @descs:				descriptor pointer
// @desc_head, desc_tail, desc_count:	descriptor counters
// @acc:				accumulator channel pointer
// @kdev:				qmss device pointer
// @range:				range info
// @qmgr:				queue manager info
// @id:					queue instance id
// @irq_num:				irq line number
// @notify_needed:			notifier needed based on queue type
// @num_notifiers:			total notifiers
// @handles:				list head
// @name:				queue instance name
// @irq_name:				irq line name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_queue_inst {
    pub descs: *mut u32,
    pub desc_count: atomic_t desc_head, desc_tail,,
    pub acc: *mut knav_acc_channel,
    pub kdev: *mut knav_device,
    pub range: *mut knav_range_info,
    pub qmgr: *mut knav_qmgr_info,
    pub id: u32,
    pub irq_num: c_int,
    pub notify_needed: c_int,
    pub num_notifiers: core::sync::atomic::AtomicI32,
    pub handles: list_head,
    pub name: *const c_char,
    pub irq_name: *const c_char,
}

//
// struct knav_queue:			qmss queue properties
// @reg_push, reg_pop, reg_peek:	push, pop queue registers
// @inst:				qmss queue instance properties
// @notifier_fn:			notifier function
// @notifier_fn_arg:			notifier function argument
// @notifier_enabled:			notier enabled for a give queue
// @rcu:				rcu head
// @flags:				queue flags
// @list:				list head
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_queue {
    pub reg_peek: *mut *mut *mut knav_reg_queue __iomem reg_push, reg_pop,,
    pub inst: *mut knav_queue_inst,
    pub stats: *mut knav_queue_stats __percpu,
    pub notifier_fn: knav_queue_notify_fn,
    pub notifier_fn_arg: *mut c_void,
    pub notifier_enabled: core::sync::atomic::AtomicI32,
    pub rcu: rcu_head,
    pub flags: unsigned,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmss_version {
    QMSS,
    QMSS_66AK2G,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_device {
    pub dev: *mut device,
    pub base_id: unsigned,
    pub num_queues: unsigned,
    pub num_queues_in_use: unsigned,
    pub inst_shift: unsigned,
    pub link_rams: [knav_link_ram_block; KNAV_NUM_LINKRAM],
    pub instances: *mut c_void,
    pub regions: list_head,
    pub queue_ranges: list_head,
    pub pools: list_head,
    pub pdsps: list_head,
    pub qmgrs: list_head,
    pub debugfs_file: *mut dentry,
    pub version: qmss_version,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_range_ops {
    pub range): *mut *mut int (init_range)(struct knav_range_info,
    pub range): *mut *mut int (free_range)(struct knav_range_info,
    pub inst): *mut knav_queue_inst,
    pub flags): *mut *mut knav_queue_inst inst, unsigned,
    pub inst): *mut knav_queue_inst,
    pub enabled): *mut *mut knav_queue_inst inst, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_irq_info {
    pub irq: c_int,
    pub cpu_mask: *mut cpumask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_range_info {
    pub name: *const c_char,
    pub kdev: *mut knav_device,
    pub queue_base: unsigned,
    pub num_queues: unsigned,
    pub queue_base_inst: *mut c_void,
    pub flags: unsigned,
    pub list: list_head,
    pub ops: *const knav_range_ops,
    pub acc_info: knav_acc_info,
    pub acc: *mut knav_acc_channel,
    pub num_irqs: unsigned,
    pub irqs: [knav_irq_info; RANGE_MAX_IRQS],
}

extern "C" {
    pub fn knav_queue_notify(inst: *mut knav_queue_inst);
}
