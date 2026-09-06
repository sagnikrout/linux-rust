//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/ptt/hisi_ptt.h
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
// Driver for HiSilicon PCIe tune and trace device
//
// Copyright (c) 2022 HiSilicon Technologies Co., Ltd.
// Author: Yicong Yang <yangyicong@hisilicon.com>
//

//
// The definition of the device registers and register fields.
//
pub const HISI_PTT_TUNING_CTRL: c_uint = 0x0000;

pub const HISI_PTT_TUNING_DATA: c_uint = 0x0004;

pub const HISI_PTT_TRACE_ADDR_SIZE: c_uint = 0x0800;
pub const HISI_PTT_TRACE_ADDR_BASE_LO_0: c_uint = 0x0810;
pub const HISI_PTT_TRACE_ADDR_BASE_HI_0: c_uint = 0x0814;
pub const HISI_PTT_TRACE_ADDR_STRIDE: c_uint = 0x8;
pub const HISI_PTT_TRACE_CTRL: c_uint = 0x0850;

pub const HISI_PTT_TRACE_INT_STAT: c_uint = 0x0890;

pub const HISI_PTT_TRACE_INT_MASK: c_uint = 0x0894;

pub const HISI_PTT_TUNING_INT_STAT: c_uint = 0x0898;

pub const HISI_PTT_TRACE_WR_STS: c_uint = 0x08a0;

pub const HISI_PTT_TRACE_STS: c_uint = 0x08b0;

pub const HISI_PTT_DEVICE_RANGE: c_uint = 0x0fe0;

pub const HISI_PTT_LOCATION: c_uint = 0x0fe8;

// Parameters of PTT trace DMA part.
pub const HISI_PTT_TRACE_DMA_IRQ: c_int = 0;
pub const HISI_PTT_TRACE_BUF_CNT: c_int = 4;

// Wait time for hardware DMA to reset

// Poll timeout and interval for waiting hardware work to finish

// FIFO size for dynamically updating the PTT trace filter list.
pub const HISI_PTT_FILTER_UPDATE_FIFO_SIZE: c_int = 16;
// Delay time for filter updating work

// Definition of the PMU configs

//
// struct hisi_ptt_tune_desc - Describe tune event for PTT tune
// @hisi_ptt:   PTT device this tune event belongs to
// @name:       name of this event
// @event_code: code of the event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_ptt_tune_desc {
    pub hisi_ptt: *mut hisi_ptt,
    pub name: *const c_char,
    pub event_code: u32,
}

//
// struct hisi_ptt_dma_buffer - Describe a single trace buffer of PTT trace.
// The detail of the data format is described
// in the documentation of PTT device.
// @dma:   DMA address of this buffer visible to the device
// @addr:  virtual address of this buffer visible to the cpu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_ptt_dma_buffer {
    pub dma: dma_addr_t,
    pub addr: *mut c_void,
}

//
// struct hisi_ptt_trace_ctrl - Control and status of PTT trace
// @trace_buf: array of the trace buffers for holding the trace data.
// the length will be HISI_PTT_TRACE_BUF_CNT.
// @handle:    perf output handle of current trace session
// @buf_index: the index of current using trace buffer
// @on_cpu:    current tracing cpu
// @started:   current trace status, true for started
// @is_port:   whether we're tracing root port or not
// @direction: direction of the TLP headers to trace
// @filter:    filter value for tracing the TLP headers
// @format:    format of the TLP headers to trace
// @type:      type of the TLP headers to trace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_ptt_trace_ctrl {
    pub trace_buf: *mut hisi_ptt_dma_buffer,
    pub handle: perf_output_handle,
    pub buf_index: u32,
    pub on_cpu: c_int,
    pub started: bool,
    pub is_port: bool,
    pub direction:2: u32,
    pub filter:16: u32,
    pub format:1: u32,
    pub type:4: u32,
}

//
// sysfs attribute group name for root port filters and requester filters:
// /sys/devices/hisi_ptt<sicl_id>_<core_id>/root_port_filters
// and
// /sys/devices/hisi_ptt<sicl_id>_<core_id>/requester_filters
//

//
// struct hisi_ptt_filter_desc - Descriptor of the PTT trace filter
// @attr:    sysfs attribute of this filter
// @list:    entry of this descriptor in the filter list
// @is_port: the PCI device of the filter is a Root Port or not
// @name:    name of this filter, same as the name of the related PCI device
// @devid:   the PCI device's devid of the filter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_ptt_filter_desc {
    pub attr: device_attribute,
    pub list: list_head,
    pub is_port: bool,
    pub name: *mut c_char,
    pub devid: u16,
}

//
// struct hisi_ptt_filter_update_info - Information for PTT filter updating
// @is_port:    the PCI device to update is a Root Port or not
// @is_add:     adding to the filter or not
// @devid:      the PCI device's devid of the filter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_ptt_filter_update_info {
    pub is_port: bool,
    pub is_add: bool,
    pub devid: u16,
}

//
// struct hisi_ptt_pmu_buf - Descriptor of the AUX buffer of PTT trace
// @length:   size of the AUX buffer
// @nr_pages: number of pages of the AUX buffer
// @base:     start address of AUX buffer
// @pos:      position in the AUX buffer to commit traced data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_ptt_pmu_buf {
    pub length: usize,
    pub nr_pages: c_int,
    pub base: *mut c_void,
    pub pos: c_long,
}

//
// struct hisi_ptt - Per PTT device data
// @trace_ctrl:   the control information of PTT trace
// @hisi_ptt_nb:  dynamic filter update notifier
// @hotplug_node: node for register cpu hotplug event
// @hisi_ptt_pmu: the pum device of trace
// @iobase:       base IO address of the device
// @pdev:         pci_dev of this PTT device
// @tune_lock:    lock to serialize the tune process
// @pmu_lock:     lock to serialize the perf process
// @trace_irq:    interrupt number used by trace
// @upper_bdf:    the upper BDF range of the PCI devices managed by this PTT device
// @lower_bdf:    the lower BDF range of the PCI devices managed by this PTT device
// @port_filters: the filter list of root ports
// @req_filters:  the filter list of requester ID
// @filter_lock:  lock to protect the filters
// @sysfs_inited: whether the filters' sysfs entries has been initialized
// @port_mask:    port mask of the managed root ports
// @work:         delayed work for filter updating
// @filter_update_lock: spinlock to protect the filter update fifo
// @filter_update_fifo: fifo of the filters waiting to update the filter list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_ptt {
    pub trace_ctrl: hisi_ptt_trace_ctrl,
    pub hisi_ptt_nb: notifier_block,
    pub hotplug_node: hlist_node,
    pub hisi_ptt_pmu: pmu,
    pub iobase: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub tune_lock: mutex,
    pub pmu_lock: spinlock_t,
    pub trace_irq: c_int,
    pub upper_bdf: u32,
    pub lower_bdf: u32,
//
// The trace TLP headers can either be filtered by certain
// root port, or by the requester ID. Organize the filters
// by @port_filters and @req_filters here. The mask of all
// the valid ports is also cached for doing sanity check
// of user input.
//
    pub port_filters: list_head,
    pub req_filters: list_head,
    pub filter_lock: mutex,
    pub sysfs_inited: bool,
    pub port_mask: u16,
//
// We use a delayed work here to avoid indefinitely waiting for
// the hisi_ptt->mutex which protecting the filter list. The
// work will be delayed only if the mutex can not be held,
// otherwise no delay will be applied.
//
    pub work: delayed_work,
    pub filter_update_lock: spinlock_t,
}

