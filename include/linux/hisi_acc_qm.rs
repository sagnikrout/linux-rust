//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hisi_acc_qm.h
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
// Copyright (c) 2019 HiSilicon Limited.

pub const QM_QNUM_V1: c_int = 4096;
pub const QM_QNUM_V2: c_int = 1024;
pub const QM_MAX_VFS_NUM_V2: c_int = 63;
// qm user domain
pub const QM_ARUSER_M_CFG_1: c_uint = 0x100088;

pub const AXUSER_CMD_SMMU_NORMAL: c_int = 1;

pub const QM_ARUSER_M_CFG_ENABLE: c_uint = 0x100090;
pub const ARUSER_M_CFG_ENABLE: c_uint = 0xfffffffe;
pub const QM_AWUSER_M_CFG_1: c_uint = 0x100098;
pub const QM_AWUSER_M_CFG_ENABLE: c_uint = 0x1000a0;
pub const AWUSER_M_CFG_ENABLE: c_uint = 0xfffffffe;
pub const QM_WUSER_M_CFG_ENABLE: c_uint = 0x1000a8;
pub const WUSER_M_CFG_ENABLE: c_uint = 0xffffffff;
// mailbox
pub const QM_MB_CMD_SQC: c_uint = 0x0;
pub const QM_MB_CMD_CQC: c_uint = 0x1;
pub const QM_MB_CMD_EQC: c_uint = 0x2;
pub const QM_MB_CMD_AEQC: c_uint = 0x3;
pub const QM_MB_CMD_SQC_BT: c_uint = 0x4;
pub const QM_MB_CMD_CQC_BT: c_uint = 0x5;
pub const QM_MB_CMD_SQC_VFT_V2: c_uint = 0x6;
pub const QM_MB_CMD_STOP_QP: c_uint = 0x8;
pub const QM_MB_CMD_FLUSH_QM: c_uint = 0x9;
pub const QM_MB_CMD_SRC: c_uint = 0xc;
pub const QM_MB_CMD_DST: c_uint = 0xd;
pub const QM_MB_CMD_SEND_BASE: c_uint = 0x300;
pub const QM_MB_EVENT_SHIFT: c_int = 8;
pub const QM_MB_BUSY_SHIFT: c_int = 13;
pub const QM_MB_OP_SHIFT: c_int = 14;
pub const QM_MB_CMD_DATA_ADDR_L: c_uint = 0x304;
pub const QM_MB_CMD_DATA_ADDR_H: c_uint = 0x308;
pub const QM_MB_MAX_WAIT_CNT: c_int = 6000;
// doorbell
pub const QM_DOORBELL_CMD_SQ: c_int = 0;
pub const QM_DOORBELL_CMD_CQ: c_int = 1;
pub const QM_DOORBELL_CMD_EQ: c_int = 2;
pub const QM_DOORBELL_CMD_AEQ: c_int = 3;
pub const QM_DOORBELL_SQ_CQ_BASE_V2: c_uint = 0x1000;
pub const QM_DOORBELL_EQ_AEQ_BASE_V2: c_uint = 0x2000;
pub const QM_QP_MAX_NUM_SHIFT: c_int = 11;
pub const QM_DB_CMD_SHIFT_V2: c_int = 12;
pub const QM_DB_RAND_SHIFT_V2: c_int = 16;
pub const QM_DB_INDEX_SHIFT_V2: c_int = 32;
pub const QM_DB_PRIORITY_SHIFT_V2: c_int = 48;
pub const QM_VF_STATE: c_uint = 0x60;
// qm cache
pub const QM_CACHE_CTL: c_uint = 0x100050;

pub const QM_AXI_M_CFG: c_uint = 0x1000ac;
pub const AXI_M_CFG: c_uint = 0xffff;
pub const QM_AXI_M_CFG_ENABLE: c_uint = 0x1000b0;
pub const AM_CFG_SINGLE_PORT_MAX_TRANS: c_uint = 0x300014;
pub const AXI_M_CFG_ENABLE: c_uint = 0xffffffff;
pub const QM_PEH_AXUSER_CFG: c_uint = 0x1000cc;
pub const QM_PEH_AXUSER_CFG_ENABLE: c_uint = 0x1000d0;
pub const PEH_AXUSER_CFG: c_uint = 0x401001;
pub const PEH_AXUSER_CFG_ENABLE: c_uint = 0xffffffff;
pub const QM_MIN_QNUM: c_int = 2;
pub const HISI_ACC_SGL_SGE_NR_MAX: c_int = 255;
pub const QM_SHAPER_CFG: c_uint = 0x100164;

pub const QM_SHAPER_TYPE1_OFFSET: c_int = 10;
// page number for queue file region
pub const QM_DOORBELL_PAGE_NR: c_int = 1;
pub const QM_DEV_ALG_MAX_LEN: c_int = 256;
pub const QM_MIG_REGION_SEL: c_uint = 0x100198;

pub const QM_MAX_CHANNEL_NUM: c_int = 8;
pub const QM_CHANNEL_USAGE_OFFSET: c_uint = 0x1100;
pub const QM_MAX_DEV_USAGE: c_int = 100;
pub const QM_DEV_USAGE_RATE: c_int = 100;
pub const QM_CHANNEL_ADDR_INTRVL: c_uint = 0x4;
// uacce mode of the driver

//
// enum qm_stop_reason - Queue manager stop reasons
// @QM_NORMAL:      Graceful stop. Used for device unbind, driver removal,
// or runtime power management (runtime_suspend).
// @QM_SOFT_RESET:  Error recovery reset. Triggered by unrecoverable hardware
// errors (e.g., PCIe AER, timeout) to recover device state.
// @QM_DOWN:        Function Level Reset. Used when the device needs to
// be reset at the function level without resetting the link.
// @QM_SHUTDOWN:    System shutdown. Used during system poweroff, reboot, or
// kexec to ensure hardware is in a safe state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_stop_reason {
    QM_NORMAL,
    QM_SOFT_RESET,
    QM_DOWN,
    QM_SHUTDOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_state {
    QM_WORK = 0,
    QM_STOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qp_state {
    QP_START = 1,
    QP_STOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_hw_ver {
    QM_HW_V1 = 0x20,
    QM_HW_V2 = 0x21,
    QM_HW_V3 = 0x30,
    QM_HW_V4 = 0x50,
    QM_HW_V5 = 0x51,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_fun_type {
    QM_HW_PF,
    QM_HW_VF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_debug_file {
    CURRENT_QM,
    CURRENT_Q,
    CLEAR_ENABLE,
    DEBUG_FILE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_vf_state {
    QM_READY = 0,
    QM_NOT_READY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_misc_ctl_bits {
    QM_DRIVER_REMOVING = 0x0,
    QM_RESETTING,
    QM_MODULE_PARAM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qm_cap_bits {
    QM_SUPPORT_DB_ISOLATION = 0x0,
    QM_SUPPORT_FUNC_QOS,
    QM_SUPPORT_STOP_QP,
    QM_SUPPORT_STOP_FUNC,
    QM_SUPPORT_MB_COMMAND,
    QM_SUPPORT_SVA_PREFETCH,
    QM_SUPPORT_RPM,
    QM_SUPPORT_DAE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_dev_alg {
    pub alg_msk: u64,
    pub alg: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_dev_dfx {
    pub dev_state: u32,
    pub dev_timeout: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfx_diff_registers {
    pub regs: *mut u32,
    pub reg_offset: u32,
    pub reg_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_dfx {
    pub err_irq_cnt: core::sync::atomic::AtomicI64,
    pub aeq_irq_cnt: core::sync::atomic::AtomicI64,
    pub abnormal_irq_cnt: core::sync::atomic::AtomicI64,
    pub create_qp_err_cnt: core::sync::atomic::AtomicI64,
    pub mb_err_cnt: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_file {
    pub index: qm_debug_file,
    pub lock: mutex,
    pub debug: *mut qm_debug,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_debug {
    pub curr_qm_qp_num: u32,
    pub sqe_mask_offset: u32,
    pub sqe_mask_len: u32,
    pub dfx: qm_dfx,
    pub debug_root: *mut dentry,
    pub qm_d: *mut dentry,
    pub files: [debugfs_file; DEBUG_FILE_NUM],
    pub dev_dfx: qm_dev_dfx,
    pub qm_last_words: *mut c_uint,
// ACC engines recoreding last regs
    pub last_words: *mut c_uint,
    pub qm_diff_regs: *mut dfx_diff_registers,
    pub acc_diff_regs: *mut dfx_diff_registers,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_shaper_factor {
    pub func_qos: u32,
    pub cir_b: u64,
    pub cir_u: u64,
    pub cir_s: u64,
    pub cbs_s: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_dma {
    pub va: *mut c_void,
    pub dma: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_status {
    pub eq_head: u32,
    pub eqc_phase: bool,
    pub aeq_head: u32,
    pub aeqc_phase: bool,
    pub flags: core::sync::atomic::AtomicI32,
    pub stop_reason: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acc_err_result {
    ACC_ERR_NONE,
    ACC_ERR_NEED_RESET,
    ACC_ERR_RECOVERED,
    ACC_ERR_NEED_FUNC_RESET,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_err_mask {
    pub ecc_2bits_mask: u32,
    pub shutdown_mask: u32,
    pub reset_mask: u32,
    pub ce: u32,
    pub nfe: u32,
    pub fe: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_err_info {
    pub acpi_rst: *mut c_char,
    pub msi_wr_port: u32,
    pub qm_err: hisi_qm_err_mask,
    pub dev_err: hisi_qm_err_mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_err_status {
    pub is_qm_ecc_mbit: u32,
    pub is_dev_ecc_mbit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_err_ini {
    pub qm): *mut *mut int (hw_init)(struct hisi_qm,
    pub qm): *mut *mut void (hw_err_enable)(struct hisi_qm,
    pub qm): *mut *mut void (hw_err_disable)(struct hisi_qm,
    pub qm): *mut *mut u32 (get_dev_hw_err_status)(struct hisi_qm,
    pub err_sts): *mut *mut *mut void (clear_dev_hw_err_status)(struct hisi_qm qm, u32,
    pub qm): *mut *mut void (open_axi_master_ooo)(struct hisi_qm,
    pub qm): *mut *mut void (close_axi_master_ooo)(struct hisi_qm,
    pub qm): *mut *mut void (open_sva_prefetch)(struct hisi_qm,
    pub qm): *mut *mut void (close_sva_prefetch)(struct hisi_qm,
    pub qm): *mut *mut void (show_last_dfx_regs)(struct hisi_qm,
    pub qm): *mut *mut void (err_info_init)(struct hisi_qm,
    pub qm): *mut *mut acc_err_result (get_err_result)(struct hisi_qm,
    pub qm): *mut *mut bool (dev_is_abnormal)(struct hisi_qm,
    pub qm): *mut *mut int (set_priv_status)(struct hisi_qm,
    pub qm): *mut *mut void (disable_axi_error)(struct hisi_qm,
    pub qm): *mut *mut void (enable_axi_error)(struct hisi_qm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_cap_info {
    pub type: u32,
// Register offset
    pub offset: u32,
// Bit offset in register
    pub shift: u32,
    pub mask: u32,
    pub v1_val: u32,
    pub v2_val: u32,
    pub v3_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_cap_query_info {
    pub type: u32,
    pub name: *const c_char,
    pub offset: u32,
    pub v1_val: u32,
    pub v2_val: u32,
    pub v3_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_cap_record {
    pub type: u32,
    pub name: *const c_char,
    pub cap_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_cap_tables {
    pub qm_cap_size: u32,
    pub qm_cap_table: *mut hisi_qm_cap_record,
    pub dev_cap_size: u32,
    pub dev_cap_table: *mut hisi_qm_cap_record,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_list {
    pub lock: mutex,
    pub list: list_head,
    pub qm): *mut *mut int (register_to_crypto)(struct hisi_qm,
    pub qm): *mut *mut void (unregister_from_crypto)(struct hisi_qm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm_poll_data {
    pub qm: *mut hisi_qm,
    pub work: work_struct,
    pub qp_finish_id: *mut u16,
    pub eqe_num: u16,
}

//
// struct qm_err_isolate
// @isolate_lock: protects device error log
// @err_threshold: user config error threshold which triggers isolation
// @is_isolate: device isolation state
// @uacce_hw_errs: index into qm device error list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_err_isolate {
    pub isolate_lock: mutex,
    pub err_threshold: u32,
    pub is_isolate: bool,
    pub qm_hw_errs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_rsv_buf {
    pub sqc: *mut qm_sqc,
    pub cqc: *mut qm_cqc,
    pub eqc: *mut qm_eqc,
    pub aeqc: *mut qm_aeqc,
    pub sqc_dma: dma_addr_t,
    pub cqc_dma: dma_addr_t,
    pub eqc_dma: dma_addr_t,
    pub aeqc_dma: dma_addr_t,
    pub qcdma: qm_dma,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm_channel {
    pub channel_num: c_int,
    pub channel_name: [*const c_char; QM_MAX_CHANNEL_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qm {
    pub ver: qm_hw_ver,
    pub fun_type: qm_fun_type,
    pub dev_name: *const c_char,
    pub pdev: *mut pci_dev,
    pub io_base: *mut void __iomem,
    pub db_io_base: *mut void __iomem,
// Capbility version, 0: not supports
    pub cap_ver: u32,
    pub sqe_size: u32,
    pub qp_base: u32,
    pub qp_num: u32,
    pub qp_in_used: u32,
    pub ctrl_qp_num: u32,
    pub max_qp_num: u32,
    pub vfs_num: u32,
    pub db_interval: u32,
    pub eq_depth: u16,
    pub aeq_depth: u16,
    pub list: list_head,
    pub qm_list: *mut hisi_qm_list,
    pub qdma: qm_dma,
    pub sqc: *mut qm_sqc,
    pub cqc: *mut qm_cqc,
    pub eqe: *mut qm_eqe,
    pub aeqe: *mut qm_aeqe,
    pub sqc_dma: dma_addr_t,
    pub cqc_dma: dma_addr_t,
    pub eqe_dma: dma_addr_t,
    pub aeqe_dma: dma_addr_t,
    pub xqc_buf: qm_rsv_buf,
    pub status: hisi_qm_status,
    pub err_ini: *const hisi_qm_err_ini,
    pub err_info: hisi_qm_err_info,
    pub err_status: hisi_qm_err_status,
// driver removing and reset sched
    pub misc_ctl: c_ulong,
// Device capability bit
    pub caps: c_ulong,
    pub qps_lock: rw_semaphore,
    pub qp_idr: idr,
    pub qp_array: *mut hisi_qp,
    pub poll_data: *mut hisi_qm_poll_data,
    pub mailbox_lock: mutex,
    pub ifc_lock: mutex,
    pub ops: *const hisi_qm_hw_ops,
    pub debug: qm_debug,
    pub error_mask: u32,
    pub wq: *mut workqueue_struct,
    pub rst_work: work_struct,
    pub cmd_process: work_struct,
    pub use_sva: bool,
    pub phys_base: resource_size_t,
    pub db_phys_base: resource_size_t,
    pub uacce: *mut uacce_device,
    pub mode: c_int,
    pub factor: *mut qm_shaper_factor,
    pub mb_qos: u32,
    pub type_rate: u32,
    pub isolate_data: qm_err_isolate,
    pub cap_tables: hisi_qm_cap_tables,
    pub channel_data: qm_channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qp_status {
    pub used: core::sync::atomic::AtomicI32,
    pub sq_tail: u16,
    pub cq_head: u16,
    pub cqc_phase: bool,
    pub flags: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qp_ops {
    pub d_parm): *mut *mut *mut *mut int (fill_sqe)(void sqe, void q_parm, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct instance_backlog {
    pub list: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_qp {
    pub qp_id: u32,
    pub sq_depth: u16,
    pub cq_depth: u16,
    pub alg_type: u8,
    pub qdma: qm_dma,
    pub sqe: *mut c_void,
    pub cqe: *mut qm_cqe,
    pub sqe_dma: dma_addr_t,
    pub cqe_dma: dma_addr_t,
    pub qp_status: hisi_qp_status,
    pub hw_ops: *mut hisi_qp_ops,
    pub data): *mut *mut *mut void (req_cb)(struct hisi_qp qp, void,
    pub qp): *mut *mut void (event_cb)(struct hisi_qp,
    pub qm: *mut hisi_qm,
    pub is_resetting: bool,
    pub is_in_kernel: bool,
    pub pasid: u16,
    pub uacce_q: *mut uacce_queue,
    pub ref_count: u32,
    pub qp_lock: spinlock_t,
    pub backlog: instance_backlog,
    pub msg: *const c_void,
}

extern "C" {
    pub fn param_set_int(_arg: val, _arg: kp) -> return;
}
extern "C" {
    pub fn param_set_int(_arg: val, _arg: kp) -> return;
}
extern "C" {
    pub fn mode_set(_arg: val, _arg: kp) -> return;
}
extern "C" {
    pub fn hisi_qm_register_uacce(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_qm_init(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_qm_uninit(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_start(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_qm_stop(qm: *mut hisi_qm, r: qm_stop_reason) -> c_int;
}
extern "C" {
    pub fn hisi_qp_send(qp: *mut hisi_qp, msg: *const c_void) -> c_int;
}
extern "C" {
    pub fn hisi_qm_debug_init(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_debug_regs_clear(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_sriov_enable(pdev: *mut pci_dev, max_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn hisi_qm_sriov_disable(pdev: *mut pci_dev, is_frozen: bool) -> c_int;
}
extern "C" {
    pub fn hisi_qm_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn hisi_qm_dev_err_init(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_dev_err_uninit(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_regs_debugfs_uninit(qm: *mut hisi_qm, reg_len: u32);
}
extern "C" {
    pub fn hisi_qm_dev_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t;
}
extern "C" {
    pub fn hisi_qm_reset_prepare(pdev: *mut pci_dev);
}
extern "C" {
    pub fn hisi_qm_reset_done(pdev: *mut pci_dev);
}
extern "C" {
    pub fn hisi_qm_wait_mb_ready(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_qm_mb_read(qm: *mut hisi_qm, base: *mut u64, cmd: u8, queue: u16) -> c_int;
}
extern "C" {
    pub fn hisi_qm_free_qps(qps: *mut hisi_qp, qp_num: c_int);
}
extern "C" {
    pub fn hisi_qm_dev_shutdown(pdev: *mut pci_dev);
}
extern "C" {
    pub fn hisi_qm_wait_task_finish(qm: *mut hisi_qm, qm_list: *mut hisi_qm_list);
}
extern "C" {
    pub fn hisi_qm_alg_register(qm: *mut hisi_qm, qm_list: *mut hisi_qm_list, guard: c_int) -> c_int;
}
extern "C" {
    pub fn hisi_qm_alg_unregister(qm: *mut hisi_qm, qm_list: *mut hisi_qm_list, guard: c_int);
}
extern "C" {
    pub fn hisi_qm_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn hisi_qm_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn hisi_qm_pm_uninit(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_pm_init(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_get_dfx_access(qm: *mut hisi_qm) -> c_int;
}
extern "C" {
    pub fn hisi_qm_put_dfx_access(qm: *mut hisi_qm);
}
extern "C" {
    pub fn hisi_qm_regs_dump(s: *mut seq_file, regset: *mut debugfs_regset32);
}
// Used by VFIO ACC live migration driver
