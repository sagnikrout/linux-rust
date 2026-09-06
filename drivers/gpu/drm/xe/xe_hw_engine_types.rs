//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_hw_engine_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

// See "Engine ID Definition" struct in the Icelake PRM
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_engine_class {
    XE_ENGINE_CLASS_RENDER = 0,
    XE_ENGINE_CLASS_VIDEO_DECODE = 1,
    XE_ENGINE_CLASS_VIDEO_ENHANCE = 2,
    XE_ENGINE_CLASS_COPY = 3,
    XE_ENGINE_CLASS_OTHER = 4,
    XE_ENGINE_CLASS_COMPUTE = 5,
    XE_ENGINE_CLASS_MAX = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_hw_engine_id {
    XE_HW_ENGINE_RCS0,

    XE_HW_ENGINE_BCS0,
    XE_HW_ENGINE_BCS1,
    XE_HW_ENGINE_BCS2,
    XE_HW_ENGINE_BCS3,
    XE_HW_ENGINE_BCS4,
    XE_HW_ENGINE_BCS5,
    XE_HW_ENGINE_BCS6,
    XE_HW_ENGINE_BCS7,
    XE_HW_ENGINE_BCS8,

    XE_HW_ENGINE_VCS0,
    XE_HW_ENGINE_VCS1,
    XE_HW_ENGINE_VCS2,
    XE_HW_ENGINE_VCS3,
    XE_HW_ENGINE_VCS4,
    XE_HW_ENGINE_VCS5,
    XE_HW_ENGINE_VCS6,
    XE_HW_ENGINE_VCS7,

    XE_HW_ENGINE_VECS0,
    XE_HW_ENGINE_VECS1,
    XE_HW_ENGINE_VECS2,
    XE_HW_ENGINE_VECS3,

    XE_HW_ENGINE_CCS0,
    XE_HW_ENGINE_CCS1,
    XE_HW_ENGINE_CCS2,
    XE_HW_ENGINE_CCS3,

    XE_HW_ENGINE_GSCCS0,

    XE_NUM_HW_ENGINES,
}

// FIXME: s/XE_HW_ENGINE_MAX_INSTANCE/XE_HW_ENGINE_MAX_COUNT
pub const XE_HW_ENGINE_MAX_INSTANCE: c_int = 9;
//
// struct xe_hw_engine_class_intf - per hw engine class struct interface
//
// Contains all the hw engine properties per engine class.
//
// @sched_props: scheduling properties
// @defaults: default scheduling properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_hw_engine_class_intf {
//
// @sched_props: scheduling properties
// @defaults: default scheduling properties
//
// @sched_props.job_timeout_ms: Set job timeout in ms for engine
    pub job_timeout_ms: u32,
// @sched_props.job_timeout_min: Min job timeout in ms for engine
    pub job_timeout_min: u32,
// @sched_props.job_timeout_max: Max job timeout in ms for engine
    pub job_timeout_max: u32,
// @sched_props.timeslice_us: timeslice period in micro-seconds
    pub timeslice_us: u32,
// @sched_props.timeslice_min: min timeslice period in micro-seconds
    pub timeslice_min: u32,
// @sched_props.timeslice_max: max timeslice period in micro-seconds
    pub timeslice_max: u32,
// @sched_props.preempt_timeout_us: preemption timeout in micro-seconds
    pub preempt_timeout_us: u32,
// @sched_props.preempt_timeout_min: min preemption timeout in micro-seconds
    pub preempt_timeout_min: u32,
// @sched_props.preempt_timeout_max: max preemption timeout in micro-seconds
    pub preempt_timeout_max: u32,
    pub defaults: } sched_props,,
}

//
// struct xe_hw_engine - Hardware engine
//
// Contains all the hardware engine state for physical instances.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_hw_engine {
// @gt: GT structure this hw engine belongs to
    pub gt: *mut xe_gt,
// @name: name of this hw engine
    pub name: *const c_char,
// @class: class of this hw engine
    pub class: xe_engine_class,
// @instance: physical instance of this hw engine
    pub instance: u16,
//
// @logical_instance: logical instance of this hw engine.
//
// Note: For GuC usage, always use xe_hwe_guc_logical_instance().
// For GuC usage, we should no longer use the raw logical instance.
//
    pub logical_instance: u16,
// @irq_offset: IRQ offset of this hw engine
    pub irq_offset: u16,
// @irq_page: MEMIRQ page used by this HW engine
    pub irq_page: u16,
// @mmio_base: MMIO base address of this hw engine
    pub mmio_base: u32,
//
// @reg_sr: table with registers to be restored on GT init/resume/reset
//
    pub reg_sr: xe_reg_sr,
//
// @reg_whitelist: table with registers to be whitelisted
//
    pub reg_whitelist: xe_reg_sr,
//
// @oa_whitelist: oa registers to be whitelisted
//
    pub oa_whitelist: xe_reg_sr,
//
// @oa_sr: oa nonpriv whitelist registers, changed on oa stream open/close
//
    pub oa_sr: xe_reg_sr,
//
// @reg_lrc: LRC workaround registers
//
    pub reg_lrc: xe_reg_sr,
// @domain: force wake domain of this hw engine
    pub domain: xe_force_wake_domains,
// @hwsp: hardware status page buffer object
    pub hwsp: *mut xe_bo,
// @exl_port: execlists port
    pub exl_port: *mut xe_execlist_port,
// @fence_irq: fence IRQ to run when a hw engine IRQ is received
    pub fence_irq: *mut xe_hw_fence_irq,
// @irq_handler: IRQ handler to run when hw engine IRQ is received
    pub intr_vec): *mut *mut *mut void (irq_handler)(struct xe_hw_engine hwe, u16,
// @engine_id: id  for this hw engine
    pub engine_id: xe_hw_engine_id,
// @eclass: pointer to per hw engine class interface
    pub eclass: *mut xe_hw_engine_class_intf,
// @oa_unit: oa unit for this hw engine
    pub oa_unit: *mut xe_oa_unit,
// @hw_engine_group: the group of hw engines this one belongs to
    pub hw_engine_group: *mut xe_hw_engine_group,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_hw_engine_snapshot_source_id {
    XE_ENGINE_CAPTURE_SOURCE_MANUAL,
    XE_ENGINE_CAPTURE_SOURCE_GUC
}

//
// struct xe_hw_engine_snapshot - Hardware engine snapshot
//
// Contains the snapshot of useful hardware engine info and registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_hw_engine_snapshot {
// @name: name of the hw engine
    pub name: *mut c_char,
// @hwe: hw engine
    pub hwe: *mut xe_hw_engine,
// @logical_instance: logical instance of this hw engine
    pub logical_instance: u16,
// @forcewake: Force Wake information snapshot
// @forcewake.domain: force wake domain of this hw engine
    pub domain: xe_force_wake_domains,
// @forcewake.ref: Forcewake ref for the above domain
    pub ref: c_int,
    pub forcewake: },
// @mmio_base: MMIO base address of this hw engine
    pub mmio_base: u32,
// @kernel_reserved: Engine reserved, can't be used by userspace
    pub kernel_reserved: bool,
}
