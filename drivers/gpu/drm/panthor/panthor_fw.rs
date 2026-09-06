//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_fw.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2023 Collabora ltd.

pub const MAX_CSGS: c_int = 31;
pub const MAX_CS_PER_CSG: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_ringbuf_input_iface {
    pub insert: u64,
    pub extract: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_ringbuf_output_iface {
    pub extract: u64,
    pub active: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_cs_control_iface {

    pub features: u32,
    pub input_va: u32,
    pub output_va: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_cs_input_iface {

pub const CS_STATE_STOP: c_int = 0;
pub const CS_STATE_START: c_int = 1;

    pub req: u32,

    pub config: u32,
    pub reserved1: u32,
    pub ack_irq_mask: u32,
    pub ringbuf_base: u64,
    pub ringbuf_size: u32,
    pub reserved2: u32,
    pub heap_start: u64,
    pub heap_end: u64,
    pub ringbuf_input: u64,
    pub ringbuf_output: u64,
    pub instr_config: u32,
    pub instrbuf_size: u32,
    pub instrbuf_base: u64,
    pub instrbuf_offset_ptr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_cs_output_iface {
    pub ack: u32,
    pub reserved1: [u32; 15],
    pub status_cmd_ptr: u64,

    pub status_wait: u32,
    pub status_req_resource: u32,
    pub status_wait_sync_ptr: u64,
    pub status_wait_sync_value: u32,
    pub status_scoreboards: u32,
pub const CS_STATUS_BLOCKED_REASON_UNBLOCKED: c_int = 0;
pub const CS_STATUS_BLOCKED_REASON_SB_WAIT: c_int = 1;
pub const CS_STATUS_BLOCKED_REASON_PROGRESS_WAIT: c_int = 2;
pub const CS_STATUS_BLOCKED_REASON_SYNC_WAIT: c_int = 3;
pub const CS_STATUS_BLOCKED_REASON_DEFERRED: c_int = 4;
pub const CS_STATUS_BLOCKED_REASON_RESOURCE: c_int = 5;
pub const CS_STATUS_BLOCKED_REASON_FLUSH: c_int = 6;

    pub status_blocked_reason: u32,
    pub status_wait_sync_value_hi: u32,
    pub reserved2: [u32; 6],
    pub fault: u32,
    pub fatal: u32,
    pub fault_info: u64,
    pub fatal_info: u64,
    pub reserved3: [u32; 10],
    pub heap_vt_start: u32,
    pub heap_vt_end: u32,
    pub reserved4: u32,
    pub heap_frag_end: u32,
    pub heap_address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_csg_control_iface {
    pub features: u32,
    pub input_va: u32,
    pub output_va: u32,
    pub suspend_size: u32,
    pub protm_suspend_size: u32,
    pub stream_num: u32,
    pub stream_stride: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_csg_input_iface {

pub const CSG_STATE_TERMINATE: c_int = 0;
pub const CSG_STATE_START: c_int = 1;
pub const CSG_STATE_SUSPEND: c_int = 2;
pub const CSG_STATE_RESUME: c_int = 3;

    pub req: u32,
    pub ack_irq_mask: u32,
    pub doorbell_req: u32,
    pub cs_irq_ack: u32,
    pub reserved1: [u32; 4],
    pub allow_compute: u64,
    pub allow_fragment: u64,
    pub allow_other: u32,

    pub endpoint_req: u32,
    pub endpoint_req2: u64,
    pub suspend_buf: u64,
    pub protm_suspend_buf: u64,
    pub config: u32,
    pub iter_trace_config: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_csg_output_iface {
    pub ack: u32,
    pub reserved1: u32,
    pub doorbell_ack: u32,
    pub cs_irq_req: u32,
    pub status_endpoint_current: u32,
    pub status_endpoint_req: u32,

    pub status_state: u32,
    pub resource_dep: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_global_control_iface {
    pub version: u32,
    pub features: u32,
    pub input_va: u32,
    pub output_va: u32,
    pub group_num: u32,
    pub group_stride: u32,
    pub perfcnt_size: u32,
    pub instr_features: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_global_input_iface {

pub const GLB_STATE_ACTIVE: c_int = 0;
pub const GLB_STATE_HALT: c_int = 1;
pub const GLB_STATE_SLEEP: c_int = 2;
pub const GLB_STATE_SUSPEND: c_int = 3;

    pub req: u32,
    pub ack_irq_mask: u32,
    pub doorbell_req: u32,
    pub reserved1: u32,
    pub progress_timer: u32,

    pub poweroff_timer: u32,
    pub core_en_mask: u64,
    pub reserved2: u32,
    pub perfcnt_as: u32,
    pub perfcnt_base: u64,
    pub perfcnt_extract: u32,
    pub reserved3: [u32; 3],
    pub perfcnt_config: u32,
    pub perfcnt_csg_select: u32,
    pub perfcnt_fw_enable: u32,
    pub perfcnt_csg_enable: u32,
    pub perfcnt_csf_enable: u32,
    pub perfcnt_shader_enable: u32,
    pub perfcnt_tiler_enable: u32,
    pub perfcnt_mmu_l2_enable: u32,
    pub reserved4: [u32; 8],
    pub idle_timer: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum panthor_fw_halt_status {
    PANTHOR_FW_HALT_OK = 0,
    PANTHOR_FW_HALT_ON_PANIC = 0x4e,
    PANTHOR_FW_HALT_ON_WATCHDOG_EXPIRATION = 0x4f,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_global_output_iface {
    pub ack: u32,
    pub reserved1: u32,
    pub doorbell_ack: u32,
    pub reserved2: u32,
    pub halt_status: u32,
    pub perfcnt_status: u32,
    pub perfcnt_insert: u32,
}

//
// struct panthor_fw_cs_iface - Firmware command stream slot interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_cs_iface {
//
// @lock: Lock protecting access to the panthor_fw_cs_input_iface::req
// field.
//
// Needed so we can update the req field concurrently from the interrupt
// handler and the scheduler logic.
//
// TODO: Ideally we'd want to use a cmpxchg() to update the req, but FW
// interface sections are mapped uncached/write-combined right now, and
// using cmpxchg() on such mappings leads to SError faults. Revisit when
// we have 'SHARED' GPU mappings hooked up.
//
    pub lock: spinlock_t,
//
// @control: Command stream slot control interface.
//
// Used to expose command stream slot properties.
//
// This interface is read-only.
//
    pub control: *mut panthor_fw_cs_control_iface,
//
// @input: Command stream slot input interface.
//
// Used for host updates/events.
//
    pub input: *mut panthor_fw_cs_input_iface,
//
// @output: Command stream slot output interface.
//
// Used for FW updates/events.
//
// This interface is read-only.
//
    pub output: *const panthor_fw_cs_output_iface,
}

//
// struct panthor_fw_csg_iface - Firmware command stream group slot interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_csg_iface {
//
// @lock: Lock protecting access to the panthor_fw_csg_input_iface::req
// field.
//
// Needed so we can update the req field concurrently from the interrupt
// handler and the scheduler logic.
//
// TODO: Ideally we'd want to use a cmpxchg() to update the req, but FW
// interface sections are mapped uncached/write-combined right now, and
// using cmpxchg() on such mappings leads to SError faults. Revisit when
// we have 'SHARED' GPU mappings hooked up.
//
    pub lock: spinlock_t,
//
// @control: Command stream group slot control interface.
//
// Used to expose command stream group slot properties.
//
// This interface is read-only.
//
    pub control: *const panthor_fw_csg_control_iface,
//
// @input: Command stream slot input interface.
//
// Used for host updates/events.
//
    pub input: *mut panthor_fw_csg_input_iface,
//
// @output: Command stream group slot output interface.
//
// Used for FW updates/events.
//
// This interface is read-only.
//
    pub output: *const panthor_fw_csg_output_iface,
}

//
// struct panthor_fw_global_iface - Firmware global interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panthor_fw_global_iface {
//
// @lock: Lock protecting access to the panthor_fw_global_input_iface::req
// field.
//
// Needed so we can update the req field concurrently from the interrupt
// handler and the scheduler/FW management logic.
//
// TODO: Ideally we'd want to use a cmpxchg() to update the req, but FW
// interface sections are mapped uncached/write-combined right now, and
// using cmpxchg() on such mappings leads to SError faults. Revisit when
// we have 'SHARED' GPU mappings hooked up.
//
    pub lock: spinlock_t,
//
// @control: Command stream group slot control interface.
//
// Used to expose global FW properties.
//
// This interface is read-only.
//
    pub control: *const panthor_fw_global_control_iface,
//
// @input: Global input interface.
//
// Used for host updates/events.
//
    pub input: *mut panthor_fw_global_input_iface,
//
// @output: Global output interface.
//
// Used for FW updates/events.
//
// This interface is read-only.
//
    pub output: *const panthor_fw_global_output_iface,
}

//
// panthor_fw_toggle_reqs() - Toggle acknowledge bits to send an event to the FW
// @__iface: The interface to operate on.
// @__in_reg: Name of the register to update in the input section of the interface.
// @__out_reg: Name of the register to take as a reference in the output section of the
// interface.
// @__mask: Mask to apply to the update.
//
// The Host -> FW event/message passing was designed to be lockless, with each side of
// the channel having its writeable section. Events are signaled as a difference between
// the host and FW side in the req/ack registers (when a bit differs, there's an event
// pending, when they are the same, nothing needs attention).
//
// This helper allows one to update the req register based on the current value of the
// ack register managed by the FW. Toggling a specific bit will flag an event. In order
// for events to be re-evaluated, the interface doorbell needs to be rung.
//
// Concurrent accesses to the same req register is covered.
//
// Anything requiring atomic updates to multiple registers requires a dedicated lock.
//

//
// panthor_fw_update_reqs() - Update bits to reflect a configuration change
// @__iface: The interface to operate on.
// @__in_reg: Name of the register to update in the input section of the interface.
// @__val: Value to set.
// @__mask: Mask to apply to the update.
//
// Some configuration get passed through req registers that are also used to
// send events to the FW. Those req registers being updated from the interrupt
// handler, they require special helpers to update the configuration part as well.
//
// Concurrent accesses to the same req register is covered.
//
// Anything requiring atomic updates to multiple registers requires a dedicated lock.
//

extern "C" {
    pub fn panthor_fw_ring_doorbell(ptdev: *mut panthor_device, doorbell_id: u32);
}
extern "C" {
    pub fn panthor_fw_ring_csg_doorbells(ptdev: *mut panthor_device, csg_slot: u32);
}
extern "C" {
    pub fn panthor_fw_pre_reset(ptdev: *mut panthor_device, on_hang: bool);
}
extern "C" {
    pub fn panthor_fw_post_reset(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_fw_post_reset(_arg: ptdev) -> return;
}
extern "C" {
    pub fn panthor_fw_init(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_fw_unplug(ptdev: *mut panthor_device);
}
