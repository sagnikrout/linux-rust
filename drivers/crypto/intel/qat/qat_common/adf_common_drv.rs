//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_common_drv.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2021 Intel Corporation

pub const ADF_STATUS_RESTARTING: c_int = 0;
pub const ADF_STATUS_STARTING: c_int = 1;
pub const ADF_STATUS_CONFIGURED: c_int = 2;
pub const ADF_STATUS_STARTED: c_int = 3;
pub const ADF_STATUS_AE_INITIALISED: c_int = 4;
pub const ADF_STATUS_AE_UCODE_LOADED: c_int = 5;
pub const ADF_STATUS_AE_STARTED: c_int = 6;
pub const ADF_STATUS_PF_RUNNING: c_int = 7;
pub const ADF_STATUS_IRQ_ALLOCATED: c_int = 8;
pub const ADF_STATUS_CRYPTO_ALGS_REGISTERED: c_int = 9;
pub const ADF_STATUS_COMP_ALGS_REGISTERED: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_dev_reset_mode {
    ADF_DEV_RESET_ASYNC = 0,
    ADF_DEV_RESET_SYNC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_event {
    ADF_EVENT_INIT = 0,
    ADF_EVENT_START,
    ADF_EVENT_STOP,
    ADF_EVENT_SHUTDOWN,
    ADF_EVENT_RESTARTING,
    ADF_EVENT_RESTARTED,
    ADF_EVENT_FATAL_ERROR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct service_hndl {
    pub event): adf_event,
    pub init_status: [c_ulong; ADF_DEVS_ARRAY_SIZE],
    pub start_status: [c_ulong; ADF_DEVS_ARRAY_SIZE],
    pub name: *mut c_char,
    pub list: list_head,
}

extern "C" {
    pub fn adf_service_register(service: *mut service_hndl) -> c_int;
}
extern "C" {
    pub fn adf_service_unregister(service: *mut service_hndl) -> c_int;
}
extern "C" {
    pub fn adf_dev_up(accel_dev: *mut adf_accel_dev, init_config: bool) -> c_int;
}
extern "C" {
    pub fn adf_dev_down(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_dev_restart(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_devmgr_update_class_index(hw_data: *mut adf_hw_device_data);
}
extern "C" {
    pub fn adf_clean_vf_map(_arg: bool);
}
extern "C" {
    pub fn adf_notify_fatal_error(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_error_notifier(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_devmgr_in_reset(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_dev_started(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_dev_restarting_notify(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_dev_restarted_notify(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_ae_init(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_ae_shutdown(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_ae_fw_load(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_ae_fw_release(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_ae_start(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_ae_stop(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_reset_sbr(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_reset_flr(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_dev_restore(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_set_bme(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_init_aer() -> c_int;
}
extern "C" {
    pub fn adf_exit_aer();
}
extern "C" {
    pub fn adf_init_arb(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_exit_arb(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_update_ring_arb(ring: *mut adf_etr_ring_data);
}
extern "C" {
    pub fn adf_dev_get(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_dev_put(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_dev_in_use(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_init_etr_data(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_cleanup_etr_data(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn qat_crypto_register() -> c_int;
}
extern "C" {
    pub fn qat_crypto_unregister() -> c_int;
}
extern "C" {
    pub fn qat_crypto_vf_dev_config(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn qat_crypto_put_instance(inst: *mut qat_crypto_instance);
}
extern "C" {
    pub fn qat_alg_callback(resp: *mut c_void);
}
extern "C" {
    pub fn qat_alg_asym_callback(resp: *mut c_void);
}
extern "C" {
    pub fn qat_algs_register() -> c_int;
}
extern "C" {
    pub fn qat_algs_unregister();
}
extern "C" {
    pub fn qat_asym_algs_register() -> c_int;
}
extern "C" {
    pub fn qat_asym_algs_unregister();
}
extern "C" {
    pub fn qat_compression_put_instance(inst: *mut qat_compression_instance);
}
extern "C" {
    pub fn qat_compression_register() -> c_int;
}
extern "C" {
    pub fn qat_compression_unregister() -> c_int;
}
extern "C" {
    pub fn qat_comp_algs_register(caps: u32) -> c_int;
}
extern "C" {
    pub fn qat_comp_algs_unregister(caps: u32);
}
extern "C" {
    pub fn qat_comp_alg_callback(resp: *mut c_void);
}
extern "C" {
    pub fn adf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_isr_resource_free(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_isr_sync_ae_cluster(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_vf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_vf_isr_resource_free(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_pfvf_comms_disabled(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_sysfs_init(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn qat_hal_init(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn qat_hal_deinit(handle: *mut icp_qat_fw_loader_handle);
}
extern "C" {
    pub fn qat_hal_start(handle: *mut icp_qat_fw_loader_handle) -> c_int;
}
extern "C" {
    pub fn qat_hal_reset(handle: *mut icp_qat_fw_loader_handle);
}
extern "C" {
    pub fn qat_hal_clr_reset(handle: *mut icp_qat_fw_loader_handle) -> c_int;
}
extern "C" {
    pub fn qat_hal_get_ins_num() -> c_int;
}
extern "C" {
    pub fn qat_uclo_wr_all_uimage(handle: *mut icp_qat_fw_loader_handle) -> c_int;
}
extern "C" {
    pub fn qat_uclo_del_obj(handle: *mut icp_qat_fw_loader_handle);
}
extern "C" {
    pub fn adf_init_misc_wq() -> c_int;
}
extern "C" {
    pub fn adf_exit_misc_wq();
}
extern "C" {
    pub fn adf_misc_wq_queue_work(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn adf_misc_wq_flush();
}

extern "C" {
    pub fn adf_sriov_configure(pdev: *mut pci_dev, numvfs: c_int) -> c_int;
}
extern "C" {
    pub fn adf_disable_sriov(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_reenable_sriov(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_enable_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, vf_mask: u32);
}
extern "C" {
    pub fn adf_enable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, num_vfs: u32);
}
extern "C" {
    pub fn adf_disable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_recv_and_handle_pf2vf_msg(accel_dev: *mut adf_accel_dev) -> bool;
}
extern "C" {
    pub fn adf_recv_and_handle_vf2pf_msg(accel_dev: *mut adf_accel_dev, vf_nr: u32) -> bool;
}
extern "C" {
    pub fn adf_pf2vf_handle_pf_restarting(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_enable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_disable_pf2vf_interrupts(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_schedule_vf2pf_handler(vf_info: *mut adf_accel_vf_info);
}
extern "C" {
    pub fn adf_init_pf_wq() -> c_int;
}
extern "C" {
    pub fn adf_exit_pf_wq();
}
extern "C" {
    pub fn adf_init_vf_wq() -> c_int;
}
extern "C" {
    pub fn adf_exit_vf_wq();
}
extern "C" {
    pub fn adf_flush_vf_wq(accel_dev: *mut adf_accel_dev);
}

