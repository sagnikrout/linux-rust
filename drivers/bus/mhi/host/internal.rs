//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bus/mhi/host/internal.h
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
// Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
//

// Host request register
pub const MHI_SOC_RESET_REQ_OFFSET: c_uint = 0xb0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ctxt {
    pub er_ctxt: *mut mhi_event_ctxt,
    pub chan_ctxt: *mut mhi_chan_ctxt,
    pub cmd_ctxt: *mut mhi_cmd_ctxt,
    pub er_ctxt_addr: dma_addr_t,
    pub chan_ctxt_addr: dma_addr_t,
    pub cmd_ctxt_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bhi_vec_entry {
    pub dma_addr: __le64,
    pub size: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_fw_load_type {
    MHI_FW_LOAD_BHI,	/* BHI only in PBL */
    MHI_FW_LOAD_BHIE,	/* BHIe only in PBL */
    MHI_FW_LOAD_FBC,	/* BHI in PBL followed by BHIe in SBL */
    MHI_FW_LOAD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_ch_state_type {
    MHI_CH_STATE_TYPE_RESET,
    MHI_CH_STATE_TYPE_STOP,
    MHI_CH_STATE_TYPE_START,
    MHI_CH_STATE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_st_transition {
    DEV_ST_TRANSITION_PBL,
    DEV_ST_TRANSITION_READY,
    DEV_ST_TRANSITION_SBL,
    DEV_ST_TRANSITION_MISSION_MODE,
    DEV_ST_TRANSITION_FP,
    DEV_ST_TRANSITION_SYS_ERR,
    DEV_ST_TRANSITION_DISABLE,
    DEV_ST_TRANSITION_DISABLE_DESTROY_DEVICE,
    DEV_ST_TRANSITION_MAX,
}

// internal power states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_pm_state {
    MHI_PM_STATE_DISABLE,
    MHI_PM_STATE_POR,
    MHI_PM_STATE_M0,
    MHI_PM_STATE_M2,
    MHI_PM_STATE_M3_ENTER,
    MHI_PM_STATE_M3,
    MHI_PM_STATE_M3_EXIT,
    MHI_PM_STATE_FW_DL_ERR,
    MHI_PM_STATE_SYS_ERR_DETECT,
    MHI_PM_STATE_SYS_ERR_PROCESS,
    MHI_PM_STATE_SYS_ERR_FAIL,
    MHI_PM_STATE_SHUTDOWN_PROCESS,
    MHI_PM_STATE_LD_ERR_FATAL_DETECT,
    MHI_PM_STATE_MAX
}

// firmware download failure state

// link not accessible

pub const NR_OF_CMD_RINGS: c_int = 1;
pub const CMD_EL_PER_RING: c_int = 128;
pub const PRIMARY_CMD_RING: c_int = 0;
pub const MHI_DEV_WAKE_DB: c_int = 127;
pub const MHI_MAX_MTU: c_uint = 0xffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mhi_er_type {
    MHI_ER_TYPE_INVALID = 0x0,
    MHI_ER_TYPE_VALID = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct db_cfg {
    pub reset_req: bool,
    pub db_mode: bool,
    pub pollcfg: u32,
    pub brstmode: mhi_db_brst_mode,
    pub db_val: dma_addr_t,
    pub db_val): dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_pm_transitions {
    pub from_state: mhi_pm_state,
    pub to_states: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct state_transition {
    pub node: list_head,
    pub state: dev_st_transition,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_ring {
    pub dma_handle: dma_addr_t,
    pub iommu_base: dma_addr_t,
    pub /: *mut *mut *mut __le64 ctxt_wp; / point to ctxt wp,
    pub pre_aligned: *mut c_void,
    pub base: *mut c_void,
    pub rp: *mut c_void,
    pub wp: *mut c_void,
    pub el_size: usize,
    pub len: usize,
    pub elements: usize,
    pub alloc_size: usize,
    pub db_addr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_cmd {
    pub ring: mhi_ring,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_buf_info {
    pub v_addr: *mut c_void,
    pub bb_addr: *mut c_void,
    pub wp: *mut c_void,
    pub cb_buf: *mut c_void,
    pub p_addr: dma_addr_t,
    pub len: usize,
    pub dir: dma_data_direction,
    pub /: *mut *mut bool used; / Indicates whether the buffer is used or not,
    pub /: *mut *mut bool pre_mapped; / Already pre-mapped by client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_event {
    pub mhi_cntrl: *mut mhi_controller,
    pub /: *mut *mut *mut mhi_chan mhi_chan; / dedicated to channel,
    pub er_index: u32,
    pub intmod: u32,
    pub irq: u32,
    pub /: *mut *mut int chan; / this event ring is dedicated to a channel (optional),
    pub priority: u32,
    pub data_type: mhi_er_data_type,
    pub ring: mhi_ring,
    pub db_cfg: db_cfg,
    pub task: tasklet_struct,
    pub lock: spinlock_t,
    pub event_quota): u32,
    pub hw_ring: bool,
    pub cl_manage: bool,
    pub /: *mut *mut bool offload_ev; / managed by a device driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_chan {
    pub name: *const c_char,
//
// Important: When consuming, increment tre_ring first and when
// releasing, decrement buf_ring first. If tre_ring has space, buf_ring
// is guaranteed to have space so we do not need to check both rings.
//
    pub buf_ring: mhi_ring,
    pub tre_ring: mhi_ring,
    pub chan: u32,
    pub er_index: u32,
    pub intmod: u32,
    pub type: mhi_ch_type,
    pub dir: dma_data_direction,
    pub db_cfg: db_cfg,
    pub ee_mask: mhi_ch_ee_mask,
    pub ch_state: mhi_ch_state,
    pub ccs: mhi_ev_ccs,
    pub mhi_dev: *mut mhi_device,
    pub result): *mut *mut *mut void (xfer_cb)(struct mhi_device mhi_dev, struct mhi_result,
    pub mutex: mutex,
    pub completion: completion,
    pub lock: rwlock_t,
    pub node: list_head,
    pub lpm_notify: bool,
    pub configured: bool,
    pub offload_ch: bool,
    pub wake_capable: bool,
}

// Default MHI timeout

// debugfs related functions

extern "C" {
    pub fn mhi_create_debugfs(mhi_cntrl: *mut mhi_controller);
}
extern "C" {
    pub fn mhi_destroy_debugfs(mhi_cntrl: *mut mhi_controller);
}
extern "C" {
    pub fn mhi_debugfs_init();
}
extern "C" {
    pub fn mhi_debugfs_exit();
}

extern "C" {
    pub fn mhi_destroy_device(dev: *mut device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mhi_create_devices(mhi_cntrl: *mut mhi_controller);
}
// Power management APIs
extern "C" {
    pub fn mhi_pm_st_worker(work: *mut work_struct);
}
extern "C" {
    pub fn mhi_pm_sys_err_handler(mhi_cntrl: *mut mhi_controller);
}
extern "C" {
    pub fn mhi_ready_state_transition(mhi_cntrl: *mut mhi_controller) -> c_int;
}
extern "C" {
    pub fn mhi_pm_m0_transition(mhi_cntrl: *mut mhi_controller) -> c_int;
}
extern "C" {
    pub fn mhi_pm_m1_transition(mhi_cntrl: *mut mhi_controller);
}
extern "C" {
    pub fn mhi_pm_m3_transition(mhi_cntrl: *mut mhi_controller) -> c_int;
}
extern "C" {
    pub fn __mhi_device_get_sync(mhi_cntrl: *mut mhi_controller) -> c_int;
}
extern "C" {
    pub fn mhi_download_amss_image(mhi_cntrl: *mut mhi_controller) -> c_int;
}
// Register access methods
extern "C" {
    pub fn mhi_ring_er_db(mhi_event: *mut mhi_event);
}
extern "C" {
    pub fn mhi_ring_cmd_db(mhi_cntrl: *mut mhi_controller, mhi_cmd: *mut mhi_cmd);
}
// Initialization methods
extern "C" {
    pub fn mhi_init_mmio(mhi_cntrl: *mut mhi_controller) -> c_int;
}
extern "C" {
    pub fn mhi_fw_load_handler(mhi_cntrl: *mut mhi_controller);
}
// Event processing methods
extern "C" {
    pub fn mhi_ctrl_ev_task(data: c_ulong);
}
extern "C" {
    pub fn mhi_ev_task(data: c_ulong);
}
extern "C" {
    pub fn mhi_uevent_notify(mhi_cntrl: *mut mhi_controller, ee: mhi_ee_type);
}
// ISR handlers
extern "C" {
    pub fn mhi_irq_handler(irq_number: c_int, dev: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mhi_intvec_threaded_handler(irq_number: c_int, dev: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mhi_intvec_handler(irq_number: c_int, dev: *mut c_void) -> irqreturn_t;
}
