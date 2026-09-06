//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/idxd/idxd.h
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
// Copyright(c) 2019 Intel Corporation. All rights rsvd.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_dev_type {
    IDXD_DEV_NONE = -1,
    IDXD_DEV_DSA = 0,
    IDXD_DEV_IAX,
    IDXD_DEV_WQ,
    IDXD_DEV_GROUP,
    IDXD_DEV_ENGINE,
    IDXD_DEV_CDEV,
    IDXD_DEV_CDEV_FILE,
    IDXD_DEV_MAX_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_dev {
    pub conf_dev: device,
    pub type: idxd_dev_type,
}

pub const IDXD_REG_TIMEOUT: c_int = 50;
pub const IDXD_DRAIN_TIMEOUT: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_type {
    IDXD_TYPE_UNKNOWN = -1,
    IDXD_TYPE_DSA = 0,
    IDXD_TYPE_IAX,
    IDXD_TYPE_MAX,
}

pub const IDXD_NAME_SIZE: c_int = 128;
pub const IDXD_PMU_EVENT_MAX: c_int = 64;
pub const IDXD_ENQCMDS_RETRIES: c_int = 32;
pub const IDXD_ENQCMDS_MAX_RETRIES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_complete_type {
    IDXD_COMPLETE_NORMAL = 0,
    IDXD_COMPLETE_ABORT,
    IDXD_COMPLETE_DEV_FAIL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_device_driver {
    pub name: *const c_char,
    pub type: *mut idxd_dev_type,
    pub idxd_dev): *mut *mut int (probe)(struct idxd_dev,
    pub idxd_dev): *mut *mut void (remove)(struct idxd_dev,
    pub status): *mut *mut void ctx, u32,
    pub drv: device_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_irq_entry {
    pub id: c_int,
    pub vector: c_int,
    pub pending_llist: llist_head,
    pub work_list: list_head,
//
// Lock to protect access between irq thread process descriptor
// and irq thread processing error descriptor.
//
    pub list_lock: spinlock_t,
    pub int_handle: c_int,
    pub pasid: ioasid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_group {
    pub idxd_dev: idxd_dev,
    pub idxd: *mut idxd_device,
    pub grpcfg: grpcfg,
    pub id: c_int,
    pub num_engines: c_int,
    pub num_wqs: c_int,
    pub use_rdbuf_limit: bool,
    pub rdbufs_allowed: u8,
    pub rdbufs_reserved: u8,
    pub tc_a: c_int,
    pub tc_b: c_int,
    pub desc_progress_limit: c_int,
    pub batch_progress_limit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_pmu {
    pub idxd: *mut idxd_device,
    pub event_list: [*mut perf_event; IDXD_PMU_EVENT_MAX],
    pub n_events: c_int,
    pub IDXD_PMU_EVENT_MAX): DECLARE_BITMAP(used_mask,,
    pub pmu: pmu,
    pub name: [c_char; IDXD_NAME_SIZE],
    pub n_counters: c_int,
    pub counter_width: c_int,
    pub n_event_categories: c_int,
    pub per_counter_caps_supported: bool,
    pub supported_event_categories: c_ulong,
    pub supported_filters: c_ulong,
    pub n_filters: c_int,
}

pub const IDXD_MAX_PRIORITY: c_uint = 0xf;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_wq_state {
    IDXD_WQ_DISABLED = 0,
    IDXD_WQ_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_wq_flag {
    WQ_FLAG_DEDICATED = 0,
    WQ_FLAG_BLOCK_ON_FAULT,
    WQ_FLAG_ATS_DISABLE,
    WQ_FLAG_PRS_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_wq_type {
    IDXD_WQT_NONE = 0,
    IDXD_WQT_KERNEL,
    IDXD_WQT_USER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_cdev {
    pub wq: *mut idxd_wq,
    pub cdev: cdev,
    pub idxd_dev: idxd_dev,
    pub minor: c_int,
}

pub const DRIVER_NAME_SIZE: c_int = 128;
pub const WQ_NAME_SIZE: c_int = 1024;
pub const WQ_TYPE_SIZE: c_int = 10;
pub const WQ_DEFAULT_QUEUE_DEPTH: c_int = 16;

pub const WQ_DEFAULT_MAX_BATCH: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_op_type {
    IDXD_OP_BLOCK = 0,
    IDXD_OP_NONBLOCK = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_dma_chan {
    pub chan: dma_chan,
    pub wq: *mut idxd_wq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_wq {
    pub portal: *mut void __iomem,
    pub portal_offset: u32,
    pub enqcmds_retries: c_uint,
    pub wq_active: percpu_ref,
    pub wq_dead: completion,
    pub wq_resurrect: completion,
    pub idxd_dev: idxd_dev,
    pub idxd_cdev: *mut idxd_cdev,
    pub err_queue: wait_queue_head,
    pub wq: *mut workqueue_struct,
    pub idxd: *mut idxd_device,
    pub id: c_int,
    pub ie: idxd_irq_entry,
    pub type: idxd_wq_type,
    pub group: *mut idxd_group,
    pub client_count: c_int,
    pub /: *mut *mut mutex wq_lock; / mutex for workqueue,
    pub size: u32,
    pub threshold: u32,
    pub priority: u32,
    pub state: idxd_wq_state,
    pub flags: c_ulong,
    pub wqcfg: *mut wqcfg,
    pub opcap_bmap: *mut c_ulong,
    pub hw_descs: *mut dsa_hw_desc,
    pub num_descs: c_int,
    pub compls: *mut dsa_completion_record,
    pub iax_compls: *mut iax_completion_record,
}

// Lock to protect upasid_xa access.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_engine {
    pub idxd_dev: idxd_dev,
    pub id: c_int,
    pub group: *mut idxd_group,
    pub idxd: *mut idxd_device,
}

// shadow registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_hw {
    pub version: u32,
    pub gen_cap: gen_cap_reg,
    pub wq_cap: wq_cap_reg,
    pub group_cap: group_cap_reg,
    pub engine_cap: engine_cap_reg,
    pub opcap: opcap,
    pub cmd_cap: u32,
    pub iaa_cap: iaa_cap_reg,
    pub dsacap0: dsacap0_reg,
    pub dsacap1: dsacap1_reg,
    pub dsacap2: dsacap2_reg,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_device_state {
    IDXD_DEV_HALTED = -1,
    IDXD_DEV_DISABLED = 0,
    IDXD_DEV_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_device_flag {
    IDXD_FLAG_CONFIGURABLE = 0,
    IDXD_FLAG_CMD_RUNNING,
    IDXD_FLAG_PASID_ENABLED,
    IDXD_FLAG_USER_PASID_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_dma_dev {
    pub idxd: *mut idxd_device,
    pub dma: dma_device,
}

extern "C" {
    pub fn int(idxd: *mut *mut load_device_defaults_fn_t) (struct idxd_device) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_driver_data {
    pub name_prefix: *const c_char,
    pub type: idxd_type,
    pub dev_type: *const device_type,
    pub compl_size: c_int,
    pub align: c_int,
    pub evl_cr_off: c_int,
    pub cr_status_off: c_int,
    pub cr_result_off: c_int,
    pub user_submission_safe: bool,
    pub load_device_defaults: load_device_defaults_fn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_evl {
// Lock to protect event log access.
    pub lock: mutex,
    pub log: *mut c_void,
    pub dma: dma_addr_t,
// Total size of event log = number of entries * entry size.
    pub log_size: c_uint,
// The number of entries in the event log.
    pub size: u16,
    pub bmap: *mut c_ulong,
    pub batch_fail: [bool; IDXD_MAX_BATCH_IDENT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_evl_fault {
    pub work: work_struct,
    pub wq: *mut idxd_wq,
    pub status: u8,
// make this last member always
    pub entry: [__evl_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_device {
    pub idxd_dev: idxd_dev,
    pub data: *mut idxd_driver_data,
    pub list: list_head,
    pub hw: idxd_hw,
    pub state: idxd_device_state,
    pub flags: c_ulong,
    pub id: c_int,
    pub major: c_int,
    pub cmd_status: u32,
    pub /: *mut *mut idxd_irq_entry ie; / misc irq, msix 0,
    pub pdev: *mut pci_dev,
    pub reg_base: *mut void __iomem,
    pub /: *mut *mut spinlock_t dev_lock; / spinlock for device,
    pub /: *mut *mut spinlock_t cmd_lock; / spinlock for device commands,
    pub cmd_done: *mut completion,
    pub groups: *mut idxd_group,
    pub wqs: *mut idxd_wq,
    pub engines: *mut idxd_engine,
    pub sva: *mut iommu_sva,
    pub pasid: c_uint,
    pub num_groups: c_int,
    pub irq_cnt: c_int,
    pub request_int_handles: bool,
    pub msix_perm_offset: u32,
    pub wqcfg_offset: u32,
    pub grpcfg_offset: u32,
    pub perfmon_offset: u32,
    pub max_xfer_bytes: u64,
    pub max_batch_size: u32,
    pub max_sgl_size: u32,
    pub max_groups: c_int,
    pub max_engines: c_int,
    pub max_rdbufs: c_int,
    pub max_wqs: c_int,
    pub max_wq_size: c_int,
    pub rdbuf_limit: c_int,
    pub /: *mut *mut int nr_rdbufs; / non-reserved read buffers,
    pub wqcfg_size: c_uint,
    pub wq_enable_map: *mut c_ulong,
    pub sw_err: sw_err_reg,
    pub cmd_waitq: wait_queue_head_t,
    pub idxd_dma: *mut idxd_dma_dev,
    pub wq: *mut workqueue_struct,
    pub work: work_struct,
    pub idxd_pmu: *mut idxd_pmu,
    pub opcap_bmap: *mut c_ulong,
    pub evl: *mut idxd_evl,
    pub evl_cache: *mut kmem_cache,
    pub dbgfs_dir: *mut dentry,
    pub dbgfs_evl_file: *mut dentry,
    pub user_submission_safe: bool,
    pub idxd_saved: *mut idxd_saved_states,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_saved_states {
    pub saved_idxd: idxd_device,
    pub saved_evl: idxd_evl,
    pub saved_engines: *mut idxd_engine,
    pub saved_wqs: *mut idxd_wq,
    pub saved_groups: *mut idxd_group,
    pub saved_wq_enable_map: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_ctx {
    pub req: *mut acomp_req,
    pub tfm: *mut crypto_tfm,
    pub src_addr: dma_addr_t,
    pub dst_addr: dma_addr_t,
    pub compress: bool,
}

// IDXD software descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idxd_desc {
    pub hw: *mut dsa_hw_desc,
    pub iax_hw: *mut iax_hw_desc,
}

//
// This is software defined error for the completion status. We overload the error code
// that will never appear in completion status and only SWERR register.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_completion_status {
    IDXD_COMP_DESC_ABORT = 0xff,
}

extern "C" {
    pub fn idxd_dev_to_idxd(_arg: idxd_dev) -> return;
}
extern "C" {
    pub fn idxd_dev_to_wq(_arg: idxd_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: idxd_dev, idxd_engine: struct, _arg: idxd_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: idxd_dev, idxd_group: struct, _arg: idxd_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: idxd_dev, idxd_cdev: struct, _arg: idxd_dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: ie, idxd_wq: struct, _arg: ie) -> return;
}
extern "C" {
    pub fn container_of(_arg: ie, idxd_device: struct, _arg: ie) -> return;
}
extern "C" {
    pub fn is_dsa_dev(is_iax_dev(idxd_dev: idxd_dev) ||) -> return;
}
extern "C" {
    pub fn test_bit(_arg: WQ_FLAG_DEDICATED, _arg: &wq->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: IDXD_FLAG_PASID_ENABLED, _arg: &idxd->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: IDXD_FLAG_USER_PASID_ENABLED, _arg: &idxd->flags) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_portal_prot {
    IDXD_PORTAL_UNLIMITED = 0,
    IDXD_PORTAL_LIMITED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idxd_interrupt_type {
    IDXD_IRQ_MSIX = 0,
    IDXD_IRQ_IMS,
}

//
// Even though this function can be accessed by multiple threads, it is safe to use.
// At worst the address gets used more than once before it gets incremented. We don't
// hit a threshold until iops becomes many million times a second. So the occasional
// reuse of the same address is tolerable compare to using an atomic variable. This is
// safe on a system that has atomic load/store for 32bit integers. Given that this is an
// Intel iEP device, that should not be a problem.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: wq_confdev(wq)) -> return;
}
//
// Intel IAA does not support batch processing.
// The max batch size of device, max batch size of wq and
// max batch shift of wqcfg should be always 0 on IAA.
//

extern "C" {
    pub fn idxd_driver_unregister(idxd_drv: *mut idxd_device_driver);
}

extern "C" {
    pub fn idxd_free_desc(wq: *mut idxd_wq, desc: *mut idxd_desc);
}
extern "C" {
    pub fn idxd_register_devices(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_unregister_devices(idxd: *mut idxd_device);
}
extern "C" {
    pub fn idxd_wqs_quiesce(idxd: *mut idxd_device);
}
extern "C" {
    pub fn idxd_queue_int_handle_resubmit(desc: *mut idxd_desc) -> bool;
}
extern "C" {
    pub fn multi_u64_to_bmap(bmap: *mut c_ulong, val: *mut u64, count: c_int);
}
extern "C" {
    pub fn idxd_load_iaa_device_defaults(idxd: *mut idxd_device) -> c_int;
}
// device interrupt control
extern "C" {
    pub fn idxd_misc_thread(vec: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn idxd_wq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn idxd_mask_error_interrupts(idxd: *mut idxd_device);
}
extern "C" {
    pub fn idxd_unmask_error_interrupts(idxd: *mut idxd_device);
}
// device control
extern "C" {
    pub fn idxd_device_drv_probe(idxd_dev: *mut idxd_dev) -> c_int;
}
extern "C" {
    pub fn idxd_device_drv_remove(idxd_dev: *mut idxd_dev);
}
extern "C" {
    pub fn idxd_drv_enable_wq(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn idxd_drv_disable_wq(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_device_init_reset(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_device_enable(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_device_disable(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_device_reset(idxd: *mut idxd_device);
}
extern "C" {
    pub fn idxd_device_clear_state(idxd: *mut idxd_device);
}
extern "C" {
    pub fn idxd_device_config(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_device_drain_pasid(idxd: *mut idxd_device, pasid: c_int);
}
extern "C" {
    pub fn idxd_device_load_config(idxd: *mut idxd_device) -> c_int;
}
// work queue control
extern "C" {
    pub fn idxd_wqs_unmap_portal(idxd: *mut idxd_device);
}
extern "C" {
    pub fn idxd_wq_alloc_resources(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn idxd_wq_free_resources(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_wq_enable(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn idxd_wq_disable(wq: *mut idxd_wq, reset_config: bool) -> c_int;
}
extern "C" {
    pub fn idxd_wq_drain(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_wq_reset(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_wq_map_portal(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn idxd_wq_unmap_portal(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_wq_set_pasid(wq: *mut idxd_wq, pasid: c_int) -> c_int;
}
extern "C" {
    pub fn idxd_wq_disable_pasid(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn __idxd_wq_quiesce(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_wq_quiesce(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_wq_init_percpu_ref(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn idxd_wq_free_irq(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_wq_request_irq(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn idxd_wq_flush_descs(wq: *mut idxd_wq);
}
// submission
extern "C" {
    pub fn idxd_submit_desc(wq: *mut idxd_wq, desc: *mut idxd_desc) -> c_int;
}
extern "C" {
    pub fn idxd_enqcmds(wq: *mut idxd_wq, portal: *mut void __iomem, desc: *const c_void) -> c_int;
}
// dmaengine
extern "C" {
    pub fn idxd_register_dma_device(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_unregister_dma_device(idxd: *mut idxd_device);
}
// cdev
extern "C" {
    pub fn idxd_cdev_register() -> c_int;
}
extern "C" {
    pub fn idxd_cdev_remove();
}
extern "C" {
    pub fn idxd_cdev_get_major(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_wq_add_cdev(wq: *mut idxd_wq) -> c_int;
}
extern "C" {
    pub fn idxd_wq_del_cdev(wq: *mut idxd_wq);
}
extern "C" {
    pub fn idxd_user_counter_increment(wq: *mut idxd_wq, pasid: u32, index: c_int);
}
// perfmon

extern "C" {
    pub fn perfmon_pmu_init(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn perfmon_pmu_remove(idxd: *mut idxd_device);
}
extern "C" {
    pub fn perfmon_counter_overflow(idxd: *mut idxd_device);
}

// debugfs
extern "C" {
    pub fn idxd_device_init_debugfs(idxd: *mut idxd_device) -> c_int;
}
extern "C" {
    pub fn idxd_device_remove_debugfs(idxd: *mut idxd_device);
}
extern "C" {
    pub fn idxd_init_debugfs() -> c_int;
}
extern "C" {
    pub fn idxd_remove_debugfs();
}
