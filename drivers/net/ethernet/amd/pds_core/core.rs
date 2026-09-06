//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amd/pds_core/core.h
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
// Copyright(c) 2023 Advanced Micro Devices, Inc

pub const PDSC_WATCHDOG_SECS: c_int = 5;
pub const PDSC_QUEUE_NAME_MAX_SZ: c_int = 16;

// Use fixed 4MB instead of PAGE_SIZE << MAX_PAGE_ORDER to avoid
// cpu_to_le32() truncation on large-page configs
//

pub const PDSC_HOST_MEM_MAX_COUNT: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_deferred_dma {
    pub list: list_head,
    pub dma_addr: dma_addr_t,
    pub va: *mut c_void,
    pub size: usize,
    pub dir: dma_data_direction,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_dev_bar {
    pub vaddr: *mut void __iomem,
    pub bus_addr: phys_addr_t,
    pub len: c_ulong,
    pub res_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_vf {
    pub padev: *mut pds_auxiliary_dev,
    pub vf: *mut pdsc,
    pub index: u16,
    pub vif_types: [__le16; PDS_DEV_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_devinfo {
    pub asic_type: u8,
    pub asic_rev: u8,
    pub 1]: char fw_version[PDS_CORE_DEVINFO_FWVERS_BUFLEN +,
    pub 1]: char serial_num[PDS_CORE_DEVINFO_SERIAL_BUFLEN +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_queue {
    pub info: *mut pdsc_q_info,
    pub dbval: u64,
    pub head_idx: u16,
    pub tail_idx: u16,
    pub hw_type: u8,
    pub index: c_uint,
    pub num_descs: c_uint,
    pub dbell_count: u64,
    pub features: u64,
    pub type: c_uint,
    pub hw_index: c_uint,
    pub base: *mut c_void,
    pub adminq: *mut pds_core_admin_cmd,
}

pub const PDSC_INTR_NAME_MAX_SZ: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_intr_info {
    pub name: [c_char; PDSC_INTR_NAME_MAX_SZ],
    pub index: c_uint,
    pub vector: c_uint,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_cq_info {
    pub comp: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_buf_info {
    pub page: *mut page,
    pub dma_addr: dma_addr_t,
    pub page_offset: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_q_info {
    pub desc: *mut c_void,
    pub adminq_desc: *mut pdsc_admin_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_cq {
    pub info: *mut pdsc_cq_info,
    pub bound_q: *mut pdsc_queue,
    pub bound_intr: *mut pdsc_intr_info,
    pub tail_idx: u16,
    pub done_color: bool,
    pub num_descs: c_uint,
    pub desc_size: c_uint,
    pub base: *mut c_void,
    pub /: *mut *mut dma_addr_t base_pa; / must be page aligned,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_qcq {
    pub pdsc: *mut pdsc,
    pub q_base: *mut c_void,
    pub /: *mut *mut dma_addr_t q_base_pa; / might not be page aligned,
    pub cq_base: *mut c_void,
    pub /: *mut *mut dma_addr_t cq_base_pa; / might not be page aligned,
    pub q_size: u32,
    pub cq_size: u32,
    pub armed: bool,
    pub flags: c_uint,
    pub work: work_struct,
    pub q: pdsc_queue,
    pub cq: pdsc_cq,
    pub intx: c_int,
    pub accum_work: u32,
    pub dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_viftype {
    pub name: *mut c_char,
    pub supported: bool,
    pub enabled: bool,
    pub dl_id: c_int,
    pub vif_id: c_int,
    pub padev: *mut pds_auxiliary_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc_host_mem {
    pub size: u32,
    pub tag: u16,
    pub order: u8,
    pub pg: *mut page,
    pub pa: dma_addr_t,
}

// No state flags set means we are in a steady running state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pdsc_state_flags {
    PDSC_S_FW_DEAD,		    /* stopped, wait on startup or recovery */
    PDSC_S_INITING_DRIVER,	    /* initial startup from probe */
    PDSC_S_STOPPING_DRIVER,	    /* driver remove */

// leave this as last
    PDSC_S_STATE_SIZE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdsc {
    pub pdev: *mut pci_dev,
    pub dentry: *mut dentry,
    pub dev: *mut device,
    pub bars: [pdsc_dev_bar; PDS_CORE_BARS_MAX],
    pub padev: *mut pds_auxiliary_dev,
    pub vfs: *mut pdsc_vf,
    pub num_vfs: c_int,
    pub vf_id: c_int,
    pub hw_index: c_int,
    pub uid: c_int,
    pub state: c_ulong,
    pub fw_status: u8,
    pub fw_generation: u8,
    pub last_fw_time: c_ulong,
    pub last_hb: u32,
    pub wdtimer: timer_list,
    pub wdtimer_period: c_uint,
    pub health_work: work_struct,
    pub health_stopped: bool,
    pub fw_reporter: *mut devlink_health_reporter,
    pub fw_recoveries: u32,
    pub dev_info: pdsc_devinfo,
    pub dev_ident: pds_core_dev_identity,
    pub nintrs: c_uint,
    pub /: *mut *mut *mut pdsc_intr_info intr_info; / array of nintrs elements,
    pub wq: *mut workqueue_struct,
    pub devcmd_timeout: c_uint,
    pub /: *mut *mut mutex devcmd_lock; / lock for dev_cmd operations,
    pub /: *mut *mut mutex config_lock; / lock for configuration operations,
    pub /: *mut *mut spinlock_t adminq_lock; / lock for adminq operations,
    pub deferred_dma_list: list_head,
    pub /: *mut *mut spinlock_t deferred_dma_lock; / lock for deferred DMA list,
    pub adminq_refcnt: refcount_t,
    pub info_regs: *mut pds_core_dev_info_regs __iomem,
    pub cmd_regs: *mut pds_core_dev_cmd_regs __iomem,
    pub intr_ctrl: *mut pds_core_intr __iomem,
    pub intr_status: *mut u64 __iomem,
    pub db_pages: *mut u64 __iomem,
    pub phy_db_pages: dma_addr_t,
    pub kern_dbpage: *mut u64 __iomem,
    pub adminqcq: pdsc_qcq,
    pub notifyqcq: pdsc_qcq,
    pub last_eid: u64,
    pub viftype_status: *mut pdsc_viftype,
    pub pci_reset_work: work_struct,
    pub host_mem_reqs: *mut pdsc_host_mem,
    pub num_host_mem_reqs: u16,
    pub fw_components: pds_core_component_list_info,
}

// enum pds_core_dbell_bits - bitwise composition of dbell values.
//
// @PDS_CORE_DBELL_QID_MASK:	unshifted mask of valid queue id bits.
// @PDS_CORE_DBELL_QID_SHIFT:	queue id shift amount in dbell value.
// @PDS_CORE_DBELL_QID:		macro to build QID component of dbell value.
//
// @PDS_CORE_DBELL_RING_MASK:	unshifted mask of valid ring bits.
// @PDS_CORE_DBELL_RING_SHIFT:	ring shift amount in dbell value.
// @PDS_CORE_DBELL_RING:	macro to build ring component of dbell value.
//
// @PDS_CORE_DBELL_RING_0:	ring zero dbell component value.
// @PDS_CORE_DBELL_RING_1:	ring one dbell component value.
// @PDS_CORE_DBELL_RING_2:	ring two dbell component value.
// @PDS_CORE_DBELL_RING_3:	ring three dbell component value.
//
// @PDS_CORE_DBELL_INDEX_MASK:	bit mask of valid index bits, no shift needed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_dbell_bits {
    PDS_CORE_DBELL_QID_MASK		= 0xffffff,
    PDS_CORE_DBELL_QID_SHIFT		= 24,

    (((u64)(n) & PDS_CORE_DBELL_QID_MASK) << PDS_CORE_DBELL_QID_SHIFT)

    PDS_CORE_DBELL_RING_MASK		= 0x7,
    PDS_CORE_DBELL_RING_SHIFT		= 16,

    (((u64)(n) & PDS_CORE_DBELL_RING_MASK) << PDS_CORE_DBELL_RING_SHIFT)

    PDS_CORE_DBELL_RING_0		= 0,
    PDS_CORE_DBELL_RING_1		= PDS_CORE_DBELL_RING(1),
    PDS_CORE_DBELL_RING_2		= PDS_CORE_DBELL_RING(2),
    PDS_CORE_DBELL_RING_3		= PDS_CORE_DBELL_RING(3),

    PDS_CORE_DBELL_INDEX_MASK		= 0xffff,
}

extern "C" {
    pub fn pdsc_debugfs_create();
}
extern "C" {
    pub fn pdsc_debugfs_destroy();
}
extern "C" {
    pub fn pdsc_debugfs_add_dev(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_debugfs_del_dev(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_debugfs_add_ident(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_debugfs_add_viftype(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_debugfs_add_irqs(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_debugfs_add_qcq(pdsc: *mut pdsc, qcq: *mut pdsc_qcq);
}
extern "C" {
    pub fn pdsc_debugfs_del_qcq(qcq: *mut pdsc_qcq);
}
extern "C" {
    pub fn pdsc_debugfs_add_host_mem(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_debugfs_del_host_mem(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_err_to_errno(code: pds_core_status_code) -> c_int;
}
extern "C" {
    pub fn pdsc_is_fw_running(pdsc: *mut pdsc) -> bool;
}
extern "C" {
    pub fn pdsc_is_fw_good(pdsc: *mut pdsc) -> bool;
}
extern "C" {
    pub fn pdsc_devcmd_init(pdsc: *mut pdsc) -> c_int;
}
extern "C" {
    pub fn pdsc_devcmd_reset(pdsc: *mut pdsc) -> c_int;
}
extern "C" {
    pub fn pdsc_dev_init(pdsc: *mut pdsc) -> c_int;
}
extern "C" {
    pub fn pdsc_dev_uninit(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_intr_free(pdsc: *mut pdsc, index: c_int);
}
extern "C" {
    pub fn pdsc_qcq_free(pdsc: *mut pdsc, qcq: *mut pdsc_qcq);
}
extern "C" {
    pub fn pdsc_setup(pdsc: *mut pdsc, init: bool) -> c_int;
}
extern "C" {
    pub fn pdsc_teardown(pdsc: *mut pdsc, removing: bool);
}
extern "C" {
    pub fn pdsc_start(pdsc: *mut pdsc) -> c_int;
}
extern "C" {
    pub fn pdsc_stop(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_health_thread(work: *mut work_struct);
}
extern "C" {
    pub fn pdsc_register_notify(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn pdsc_unregister_notify(nb: *mut notifier_block);
}
extern "C" {
    pub fn pdsc_notify(event: c_ulong, data: *mut c_void);
}
extern "C" {
    pub fn pdsc_process_adminq(qcq: *mut pdsc_qcq);
}
extern "C" {
    pub fn pdsc_work_thread(work: *mut work_struct);
}
extern "C" {
    pub fn pdsc_adminq_isr(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn pdsc_get_component_info(pdsc: *mut pdsc) -> c_int;
}
extern "C" {
    pub fn pdsc_fw_components_invalidate(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_fw_down(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_fw_up(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_pci_reset_thread(work: *mut work_struct);
}
extern "C" {
    pub fn pdsc_host_mem_add(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_host_mem_free(pdsc: *mut pdsc);
}
extern "C" {
    pub fn pdsc_deferred_dma_free(pdsc: *mut pdsc);
}
