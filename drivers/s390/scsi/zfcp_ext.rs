//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/scsi/zfcp_ext.h
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
// zfcp device driver
//
// External function declarations.
//
// Copyright IBM Corp. 2002, 2026
//

// zfcp_aux.c
extern "C" {
    pub fn zfcp_adapter_release(: *mut kref);
}
extern "C" {
    pub fn zfcp_adapter_unregister(: *mut zfcp_adapter);
}
// zfcp_ccw.c
extern "C" {
    pub fn zfcp_ccw_adapter_put(: *mut zfcp_adapter);
}
// zfcp_dbf.c
extern "C" {
    pub fn zfcp_dbf_adapter_register(: *mut zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_dbf_adapter_unregister(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_dbf_rec_run(: *mut c_char, : *mut zfcp_erp_action);
}
extern "C" {
    pub fn zfcp_dbf_rec_run_wka(: *mut c_char, : *mut zfcp_fc_wka_port, _arg: u64);
}
extern "C" {
    pub fn zfcp_dbf_hba_fsf_uss(: *mut c_char, : *mut zfcp_fsf_req);
}
extern "C" {
    pub fn zfcp_dbf_hba_fsf_res(: *mut c_char, _arg: c_int, : *mut zfcp_fsf_req);
}
extern "C" {
    pub fn zfcp_dbf_hba_bit_err(: *mut c_char, : *mut zfcp_fsf_req);
}
extern "C" {
    pub fn zfcp_dbf_hba_def_err(: *mut zfcp_adapter, _arg: u64, _arg: u16, : *mut c_void);
}
extern "C" {
    pub fn zfcp_dbf_san_req(: *mut c_char, : *mut zfcp_fsf_req, _arg: u32);
}
extern "C" {
    pub fn zfcp_dbf_san_res(: *mut c_char, : *mut zfcp_fsf_req);
}
extern "C" {
    pub fn zfcp_dbf_san_in_els(: *mut c_char, : *mut zfcp_fsf_req);
}
// zfcp_erp.c
extern "C" {
    pub fn zfcp_erp_set_adapter_status(: *mut zfcp_adapter, _arg: u32);
}
extern "C" {
    pub fn zfcp_erp_clear_adapter_status(: *mut zfcp_adapter, _arg: u32);
}
extern "C" {
    pub fn zfcp_erp_adapter_reopen(: *mut zfcp_adapter, _arg: c_int, : *mut c_char);
}
extern "C" {
    pub fn zfcp_erp_adapter_shutdown(: *mut zfcp_adapter, _arg: c_int, : *mut c_char);
}
extern "C" {
    pub fn zfcp_erp_set_port_status(: *mut zfcp_port, _arg: u32);
}
extern "C" {
    pub fn zfcp_erp_clear_port_status(: *mut zfcp_port, _arg: u32);
}
extern "C" {
    pub fn zfcp_erp_port_shutdown(: *mut zfcp_port, _arg: c_int, : *mut c_char);
}
extern "C" {
    pub fn zfcp_erp_port_forced_reopen(: *mut zfcp_port, _arg: c_int, : *mut c_char);
}
extern "C" {
    pub fn zfcp_erp_set_lun_status(: *mut scsi_device, _arg: u32);
}
extern "C" {
    pub fn zfcp_erp_clear_lun_status(: *mut scsi_device, _arg: u32);
}
extern "C" {
    pub fn zfcp_erp_lun_reopen(: *mut scsi_device, _arg: c_int, : *mut c_char);
}
extern "C" {
    pub fn zfcp_erp_lun_shutdown(: *mut scsi_device, _arg: c_int, : *mut c_char);
}
extern "C" {
    pub fn zfcp_erp_lun_shutdown_wait(: *mut scsi_device, : *mut c_char);
}
extern "C" {
    pub fn zfcp_erp_thread_setup(: *mut zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_erp_thread_kill(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_erp_wait(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_erp_notify(: *mut zfcp_erp_action, long: unsigned);
}
extern "C" {
    pub fn zfcp_erp_timeout_handler(t: *mut timer_list);
}
// zfcp_fc.c
extern "C" {
    pub fn zfcp_fc_post_event(: *mut work_struct);
}
extern "C" {
    pub fn zfcp_fc_scan_ports(: *mut work_struct);
}
extern "C" {
    pub fn zfcp_fc_incoming_els(: *mut zfcp_fsf_req);
}
extern "C" {
    pub fn zfcp_fc_port_did_lookup(: *mut work_struct);
}
extern "C" {
    pub fn zfcp_fc_trigger_did_lookup(: *mut zfcp_port);
}
extern "C" {
    pub fn zfcp_fc_plogi_evaluate(: *mut zfcp_port, : *mut fc_els_flogi);
}
extern "C" {
    pub fn zfcp_fc_test_link(: *mut zfcp_port);
}
extern "C" {
    pub fn zfcp_fc_link_test_work(: *mut work_struct);
}
extern "C" {
    pub fn zfcp_fc_wka_ports_force_offline(: *mut zfcp_fc_wka_ports);
}
extern "C" {
    pub fn zfcp_fc_gs_setup(: *mut zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_fc_gs_destroy(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_fc_exec_bsg_job(: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn zfcp_fc_timeout_bsg_job(: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn zfcp_fc_sym_name_update(: *mut work_struct);
}
extern "C" {
    pub fn zfcp_fc_port_scan_backoff() -> c_uint;
}
extern "C" {
    pub fn zfcp_fc_conditional_port_scan(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_fc_inverse_conditional_port_scan(: *mut zfcp_adapter);
}
// zfcp_fsf.c
extern "C" {
    pub fn zfcp_fsf_open_port(: *mut zfcp_erp_action) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_open_wka_port(: *mut zfcp_fc_wka_port) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_close_wka_port(: *mut zfcp_fc_wka_port) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_close_port(: *mut zfcp_erp_action) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_close_physical_port(: *mut zfcp_erp_action) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_open_lun(: *mut zfcp_erp_action) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_close_lun(: *mut zfcp_erp_action) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_exchange_config_data(: *mut zfcp_erp_action) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_exchange_port_data(: *mut zfcp_erp_action) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_convert_portspeed(fsf_speed: u32) -> u32;
}
extern "C" {
    pub fn zfcp_fsf_req_dismiss_all(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_fsf_status_read(: *mut zfcp_qdio) -> c_int;
}
extern "C" {
    pub fn zfcp_status_read_refill(adapter: *mut zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_fcp_cmnd(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn zfcp_fsf_req_free(: *mut zfcp_fsf_req);
}
extern "C" {
    pub fn zfcp_fsf_fc_host_link_down(adapter: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_fsf_reqid_check(: *mut zfcp_qdio, _arg: c_int);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zfcp_fsf_print_fmt {
    ZFCP_FSF_PRINT_FMT_LIST,
    ZFCP_FSF_PRINT_FMT_SINGLEITEM,
}

// zfcp_qdio.c
extern "C" {
    pub fn zfcp_qdio_setup(: *mut zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_qdio_destroy(: *mut zfcp_qdio);
}
extern "C" {
    pub fn zfcp_qdio_send(: *mut zfcp_qdio, : *mut zfcp_qdio_req) -> c_int;
}
extern "C" {
    pub fn zfcp_qdio_open(: *mut zfcp_qdio) -> c_int;
}
extern "C" {
    pub fn zfcp_qdio_close(: *mut zfcp_qdio);
}
extern "C" {
    pub fn zfcp_qdio_siosl(: *mut zfcp_adapter);
}
// zfcp_scsi.c
extern "C" {
    pub fn zfcp_scsi_adapter_register(: *mut zfcp_adapter) -> c_int;
}
extern "C" {
    pub fn zfcp_scsi_adapter_unregister(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_scsi_rport_work(: *mut work_struct);
}
extern "C" {
    pub fn zfcp_scsi_schedule_rport_register(: *mut zfcp_port);
}
extern "C" {
    pub fn zfcp_scsi_schedule_rport_block(: *mut zfcp_port);
}
extern "C" {
    pub fn zfcp_scsi_schedule_rports_block(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_scsi_set_prot(: *mut zfcp_adapter);
}
extern "C" {
    pub fn zfcp_scsi_dif_sense_error(: *mut scsi_cmnd, _arg: c_int);
}
// zfcp_sysfs.c
extern "C" {
    pub fn zfcp_sysfs_port_is_removing(port: *const *const zfcp_port) -> bool;
}
// zfcp_unit.c
extern "C" {
    pub fn zfcp_unit_add(: *mut zfcp_port, _arg: u64) -> c_int;
}
extern "C" {
    pub fn zfcp_unit_remove(: *mut zfcp_port, _arg: u64) -> c_int;
}
extern "C" {
    pub fn zfcp_unit_scsi_scan(: *mut zfcp_unit);
}
extern "C" {
    pub fn zfcp_unit_queue_scsi_scan(: *mut zfcp_port);
}
extern "C" {
    pub fn zfcp_unit_sdev_status(: *mut zfcp_unit) -> c_uint;
}
