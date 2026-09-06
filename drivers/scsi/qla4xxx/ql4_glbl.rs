//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla4xxx/ql4_glbl.h
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
// QLogic iSCSI HBA Driver
// Copyright (c)  2003-2013 QLogic Corporation
//

// Macro flag: #define	__QLA4x_GBL_H
extern "C" {
    pub fn qla4xxx_hw_reset(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn ql4xxx_lock_drvr_wait(a: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_send_command_to_isp(ha: *mut scsi_qla_host, srb: *mut srb) -> c_int;
}
extern "C" {
    pub fn qla4xxx_initialize_adapter(ha: *mut scsi_qla_host, is_reset: c_int) -> c_int;
}
extern "C" {
    pub fn qla4xxx_soft_reset(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla4xxx_free_ddb(ha: *mut scsi_qla_host, ddb_entry: *mut ddb_entry);
}
extern "C" {
    pub fn qla4xxx_process_aen(ha: *mut scsi_qla_host, process_aen: u8);
}
extern "C" {
    pub fn qla4xxx_get_dhcp_ip_address(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_abort_task(ha: *mut scsi_qla_host, srb: *mut srb) -> c_int;
}
extern "C" {
    pub fn qla4xxx_get_firmware_status(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_get_firmware_state(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_initialize_fw_cb(ha: *mut scsi_qla_host) -> c_int;
}
// FIXME: Goodness!  this really wants a small struct to hold the
// parameters. On x86 the args will get passed on the stack!
extern "C" {
    pub fn qla4xxx_disable_acb(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_mark_device_missing(cls_session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn rd_nvram_word(ha: *mut scsi_qla_host, offset: c_int) -> u16;
}
extern "C" {
    pub fn rd_nvram_byte(ha: *mut scsi_qla_host, offset: c_int) -> u8;
}
extern "C" {
    pub fn qla4xxx_get_crash_record(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_is_nvram_configuration_valid(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_about_firmware(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_init_rings(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_srb_compl(ref: *mut kref);
}
extern "C" {
    pub fn qla4xxx_dump_buffer(b: *mut c_void, size: u32);
}
extern "C" {
    pub fn qla4xxx_queue_iocb(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_complete_iocb(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_get_sys_info(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_iospace_config(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_pci_config(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_start_firmware(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla4xxx_rd_shdw_req_q_out(ha: *mut scsi_qla_host) -> u16;
}
extern "C" {
    pub fn qla4xxx_rd_shdw_rsp_q_in(ha: *mut scsi_qla_host) -> u16;
}
extern "C" {
    pub fn qla4xxx_request_irqs(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_free_irqs(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_process_response_queue(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_wake_dpc(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_get_conn_event_log(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_mailbox_premature_completion(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_dump_registers(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_pci_config(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_iospace_config(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_load_risc(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla4_82xx_queue_iocb(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_82xx_complete_iocb(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_82xx_crb_win_unlock(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_82xx_pci_get_crb_addr_2M(: *mut scsi_qla_host, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_wr_32(: *mut scsi_qla_host, _arg: c_ulong, _arg: u32);
}
extern "C" {
    pub fn qla4_82xx_rd_32(: *mut scsi_qla_host, _arg: c_ulong) -> u32;
}
extern "C" {
    pub fn qla4_82xx_pci_mem_read_2M(: *mut scsi_qla_host, _arg: u64, : *mut c_void, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_pci_mem_write_2M(ha: *mut scsi_qla_host, _arg: u64, : *mut c_void, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_isp_reset(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_rd_shdw_req_q_out(ha: *mut scsi_qla_host) -> u16;
}
extern "C" {
    pub fn qla4_82xx_rd_shdw_rsp_q_in(ha: *mut scsi_qla_host) -> u16;
}
extern "C" {
    pub fn qla4_8xxx_get_sys_info(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_watchdog(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_stop_firmware(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_get_flash_info(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_enable_intrs(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_82xx_disable_intrs(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_enable_msix(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_msi_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla4_8xxx_default_intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla4_8xxx_msix_rsp_q(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla4xxx_mark_all_devices_missing(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_dead_adapter_cleanup(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_82xx_idc_lock(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_idc_unlock(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_device_state_handler(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_need_qsnt_handler(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_clear_drv_active(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_set_drv_active(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_conn_open(ha: *mut scsi_qla_host, fw_ddb_index: u16) -> c_int;
}
extern "C" {
    pub fn qla4xxx_clear_ddb_entry(ha: *mut scsi_qla_host, fw_ddb_index: u32) -> c_int;
}
extern "C" {
    pub fn qla4xxx_send_passthru0(task: *mut iscsi_task) -> c_int;
}
extern "C" {
    pub fn qla4xxx_free_ddb_index(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_get_ddb_index(ha: *mut scsi_qla_host, ddb_index: *mut u16) -> c_int;
}
extern "C" {
    pub fn qla4xxx_login_flash_ddb(cls_session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn qla4xxx_unblock_ddb(cls_session: *mut iscsi_cls_session) -> c_int;
}
extern "C" {
    pub fn qla4xxx_unblock_flash_ddb(cls_session: *mut iscsi_cls_session) -> c_int;
}
extern "C" {
    pub fn qla4xxx_build_ddb_list(ha: *mut scsi_qla_host, is_reset: c_int);
}
// BSG Functions
extern "C" {
    pub fn qla4xxx_bsg_request(bsg_job: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn qla4xxx_process_vendor_specific(bsg_job: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn qla4xxx_arm_relogin_timer(ddb_entry: *mut ddb_entry);
}
extern "C" {
    pub fn qla4xxx_req_template_size(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_alloc_sysfs_attr(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_free_sysfs_attr(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4xxx_alloc_fw_dump(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_82xx_try_start_fw(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_need_reset(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_md_rd_32(ha: *mut scsi_qla_host, off: u32, data: *mut u32) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_md_wr_32(ha: *mut scsi_qla_host, off: u32, data: u32) -> c_int;
}
extern "C" {
    pub fn qla4_82xx_rom_lock_recovery(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_82xx_process_mbox_intr(ha: *mut scsi_qla_host, outcount: c_int);
}
extern "C" {
    pub fn qla4xxx_process_mbox_intr(ha: *mut scsi_qla_host, outcount: c_int);
}
extern "C" {
    pub fn qla4_8xxx_dump_peg_reg(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_disable_intrs(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_enable_intrs(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_start_firmware(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_intr_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla4_83xx_isp_reset(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_queue_iocb(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_complete_iocb(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_rd_reg(ha: *mut scsi_qla_host, addr: c_ulong) -> u32;
}
extern "C" {
    pub fn qla4_83xx_wr_reg(ha: *mut scsi_qla_host, addr: c_ulong, val: u32);
}
extern "C" {
    pub fn qla4_83xx_drv_lock(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_drv_unlock(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_rom_lock_recovery(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_process_mbox_intr(ha: *mut scsi_qla_host, outcount: c_int);
}
extern "C" {
    pub fn qla4_83xx_read_reset_template(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_set_idc_dontreset(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_idc_dontreset(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_clear_idc_dontreset(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_need_reset_handler(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_get_idc_param(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_set_rst_ready(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_clear_rst_ready(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_device_bootstrap(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_get_minidump(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_8xxx_intr_disable(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_intr_enable(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_set_param(ha: *mut scsi_qla_host, param: c_int) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_update_idc_reg(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_post_idc_ack(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_disable_pause(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_enable_mbox_intrs(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla4_83xx_can_perform_reset(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_disable_acb(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_84xx_config_acb(ha: *mut scsi_qla_host, acb_config: c_int) -> c_int;
}
extern "C" {
    pub fn qla4xxx_set_ipaddr_state(fw_ipaddr_state: u8) -> u8;
}
extern "C" {
    pub fn qla4_83xx_get_port_config(ha: *mut scsi_qla_host, config: *mut u32) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_set_port_config(ha: *mut scsi_qla_host, config: *mut u32) -> c_int;
}
extern "C" {
    pub fn qla4_8xxx_check_init_adapter_retry(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4_83xx_is_detached(ha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla4xxx_sysfs_ddb_export(ha: *mut scsi_qla_host) -> c_int;
}
