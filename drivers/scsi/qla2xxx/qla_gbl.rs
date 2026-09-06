//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qla2xxx/qla_gbl.h
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
// QLogic Fibre Channel HBA Driver
// Copyright (c)  2003-2014 QLogic Corporation
//

//
// Global Function Prototypes in qla_init.c source file.
//
extern "C" {
    pub fn qla2x00_alloc_fce_trace(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_free_fce_trace(ha: *mut qla_hw_data);
}
extern "C" {
    pub fn qla_enable_fce_trace(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_initialize_adapter(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_post_prli_work(vha: *mut scsi_qla_host, fcport: *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla2100_pci_config(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2300_pci_config(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla24xx_pci_config(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla25xx_pci_config(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_reset_chip(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla24xx_reset_chip(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_chip_diag(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla24xx_chip_diag(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_config_rings(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla24xx_config_rings(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_reset_adapter(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla24xx_reset_adapter(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_nvram_config(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla24xx_nvram_config(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla81xx_nvram_config(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_update_fw_options(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla24xx_update_fw_options(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_load_risc(: *mut scsi_qla_host, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qla24xx_load_risc(: *mut scsi_qla_host_t, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qla81xx_load_risc(: *mut scsi_qla_host_t, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qla29xx_load_risc(vha: *mut scsi_qla_host_t, srisc_addr: *mut u32) -> c_int;
}
extern "C" {
    pub fn qla2x00_perform_loop_resync(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_loop_resync(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_clear_loop_id(fcport: *mut fc_port_t);
}
extern "C" {
    pub fn qla2x00_fabric_login(: *mut scsi_qla_host_t, : *mut fc_port_t, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla2x00_local_device_login(: *mut scsi_qla_host_t, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_els_dcmd_iocb(: *mut scsi_qla_host_t, _arg: c_int, _arg: port_id_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_els_dcmd2_iocb(: *mut scsi_qla_host_t, _arg: c_int, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_abort_isp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_abort_isp_cleanup(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_quiesce_io(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_update_fcport(: *mut scsi_qla_host_t, : *mut fc_port_t);
}
extern "C" {
    pub fn qla_register_fcport_fn(: *mut work_struct);
}
extern "C" {
    pub fn qla2x00_alloc_fw_dump(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_try_to_stop_firmware(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_get_thermal_temp(: *mut scsi_qla_host_t, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla84xx_put_chip(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_async_logout(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_async_prlo(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_async_tm_cmd(: *mut fc_port_t, _arg: u32, _arg: u64, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qla24xx_async_gnl(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_post_work(vha: *mut scsi_qla_host, e: *mut qla_work_evt) -> c_int;
}
extern "C" {
    pub fn qla24xx_update_fcport_fcp_prio(: *mut scsi_qla_host_t, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_async_abort_cmd(: *mut srb_t, _arg: bool) -> c_int;
}
extern "C" {
    pub fn qla2x00_set_fcport_state(fcport: *mut fc_port_t, state: c_int);
}
extern "C" {
    pub fn __qla83xx_set_idc_control(: *mut scsi_qla_host_t, _arg: u32) -> c_int;
}
extern "C" {
    pub fn __qla83xx_get_idc_control(: *mut scsi_qla_host_t, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qla83xx_idc_audit(: *mut scsi_qla_host_t, _arg: c_int);
}
extern "C" {
    pub fn qla83xx_nic_core_reset(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla83xx_reset_ownership(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2xxx_mctp_dump(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_init_rings(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2xxx_delete_qpair(: *mut scsi_qla_host, : *mut qla_qpair) -> c_int;
}
extern "C" {
    pub fn qla2x00_handle_rscn(vha: *mut scsi_qla_host_t, ea: *mut event_arg);
}
extern "C" {
    pub fn qla24xx_async_gpdb(: *mut scsi_qla_host, : *mut fc_port_t, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qla24xx_async_prli(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_fcport_handle_login(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_detect_sfp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_post_gpdb_work(: *mut scsi_qla_host, : *mut fc_port_t, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qla_post_iidma_work(vha: *mut scsi_qla_host, fcport: *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla_do_iidma_work(vha: *mut scsi_qla_host, fcport: *mut fc_port_t);
}
extern "C" {
    pub fn qla2x00_reserve_mgmt_server_loop_id(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla_rscn_replay(fcport: *mut fc_port_t);
}
extern "C" {
    pub fn qla24xx_free_purex_item(item: *mut purex_item);
}
extern "C" {
    pub fn qla24xx_risc_firmware_invalid(: *mut u32) -> bool;
}
extern "C" {
    pub fn qla_init_iocb_limit(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla_edif_list_del(fcport: *mut fc_port_t);
}
extern "C" {
    pub fn qla_edif_sadb_release(ha: *mut qla_hw_data);
}
extern "C" {
    pub fn qla_edif_sadb_build_free_pool(ha: *mut qla_hw_data) -> c_int;
}
extern "C" {
    pub fn qla_edif_sadb_release_free_pool(ha: *mut qla_hw_data);
}
extern "C" {
    pub fn qla2x00_release_all_sadb(vha: *mut scsi_qla_host, fcport: *mut fc_port);
}
extern "C" {
    pub fn qla_edif_process_els(vha: *mut scsi_qla_host_t, bsgjob: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn qla_edif_sess_down(vha: *mut scsi_qla_host, sess: *mut fc_port);
}
extern "C" {
    pub fn qla_adjust_iocb_limit(vha: *mut scsi_qla_host_t);
}
//
// Global Data in qla_os.c source file.
//
extern "C" {
    pub fn qla2x00_loop_reset(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_abort_all_cmds(: *mut scsi_qla_host_t, _arg: c_int);
}
extern "C" {
    pub fn qla2x00_post_idc_ack_work(: *mut scsi_qla_host, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla2x00_set_exlogins_buffer(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_free_exlogin_buffer(: *mut qla_hw_data);
}
extern "C" {
    pub fn qla2x00_set_exchoffld_buffer(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_free_exchoffld_buffer(: *mut qla_hw_data);
}
extern "C" {
    pub fn qla81xx_restart_mpi_firmware(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_relogin(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_do_work(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_free_fcports(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_free_fcport(: *mut fc_port_t);
}
extern "C" {
    pub fn qla83xx_schedule_work(: *mut scsi_qla_host_t, _arg: c_int);
}
extern "C" {
    pub fn qla83xx_service_idc_aen(: *mut work_struct);
}
extern "C" {
    pub fn qla83xx_nic_core_unrecoverable_work(: *mut work_struct);
}
extern "C" {
    pub fn qla83xx_idc_state_handler_work(: *mut work_struct);
}
extern "C" {
    pub fn qla83xx_nic_core_reset_work(: *mut work_struct);
}
extern "C" {
    pub fn qla83xx_idc_lock(: *mut scsi_qla_host_t, _arg: u16);
}
extern "C" {
    pub fn qla83xx_idc_unlock(: *mut scsi_qla_host_t, _arg: u16);
}
extern "C" {
    pub fn qla83xx_idc_state_handler(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla83xx_set_drv_presence(vha: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn __qla83xx_set_drv_presence(vha: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla83xx_clear_drv_presence(vha: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn __qla83xx_clear_drv_presence(vha: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_post_uevent_work(: *mut scsi_qla_host, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qla2x00_disable_board_on_pci_error(: *mut work_struct);
}
extern "C" {
    pub fn qla2x00_sp_compl(sp: *mut srb_t, _arg: c_int);
}
extern "C" {
    pub fn qla2xxx_qpair_sp_free_dma(sp: *mut srb_t);
}
extern "C" {
    pub fn qla2xxx_qpair_sp_compl(sp: *mut srb_t, _arg: c_int);
}
extern "C" {
    pub fn qla24xx_sched_upd_fcport(: *mut fc_port_t);
}
extern "C" {
    pub fn qla24xx_post_gnl_work(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_post_relogin_work(vha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_wait_for_sess_deletion(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla_pci_set_eeh_busy(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla_schedule_eeh_work(: *mut scsi_qla_host);
}
//
// Global Functions in qla_mid.c source file.
//
extern "C" {
    pub fn qla_update_vp_map(: *mut scsi_qla_host, _arg: c_int);
}
extern "C" {
    pub fn qla2x00_timer(: *mut timer_list);
}
extern "C" {
    pub fn qla2x00_start_timer(: *mut scsi_qla_host_t, long: unsigned);
}
extern "C" {
    pub fn qla24xx_deallocate_vp_id(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla24xx_disable_vp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_enable_vp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_control_vp(: *mut scsi_qla_host_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla24xx_modify_vp_config(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_send_change_request(: *mut scsi_qla_host_t, _arg: u16, _arg: u16) -> c_int;
}
extern "C" {
    pub fn qla2x00_vp_stop_timer(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla24xx_configure_vhba(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_report_id_acquisition(vha: *mut scsi_qla_host_t, pkt: *mut c_void);
}
extern "C" {
    pub fn qla2x00_do_dpc_all_vps(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla24xx_vport_create_req_sanity_check(: *mut fc_vport) -> c_int;
}
extern "C" {
    pub fn qla2x00_sp_free_dma(sp: *mut srb_t);
}
extern "C" {
    pub fn qla2x00_mark_device_lost(: *mut scsi_qla_host_t, : *mut fc_port_t, _arg: c_int);
}
extern "C" {
    pub fn qla2x00_mark_all_devices_lost(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla24xx_async_abort_cmd(: *mut srb_t, _arg: bool) -> c_int;
}
extern "C" {
    pub fn qla2x00_wait_for_hba_online(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_wait_for_chip_reset(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_wait_for_fcoe_ctx_reset(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2xxx_wake_dpc(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_alert_all_vps(: *mut rsp_que, : *mut u16);
}
extern "C" {
    pub fn qla2x00_vp_abort_isp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla_adjust_buf(: *mut scsi_qla_host);
}
//
// Global Function Prototypes in qla_iocb.c source file.
//
extern "C" {
    pub fn qla2x00_calc_iocbs_32(_arg: u16) -> u16;
}
extern "C" {
    pub fn qla2x00_calc_iocbs_64(_arg: u16) -> u16;
}
extern "C" {
    pub fn qla2x00_build_scsi_iocbs_32(: *mut srb_t, : *mut cmd_entry_t, _arg: u16);
}
extern "C" {
    pub fn qla2x00_build_scsi_iocbs_64(: *mut srb_t, : *mut cmd_entry_t, _arg: u16);
}
extern "C" {
    pub fn qla2xxx_get_next_handle(req: *mut req_que) -> u32;
}
extern "C" {
    pub fn qla2x00_start_scsi(sp: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_start_scsi(sp: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_start_sp(: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_dif_start_scsi(: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_start_bidir(: *mut srb_t, : *mut scsi_qla_host, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qla2xxx_dif_start_scsi_mq(: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_get_async_timeout(: *mut scsi_qla_host) -> c_ulong;
}
extern "C" {
    pub fn qla2x00_issue_marker(: *mut scsi_qla_host_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla24xx_get_one_block_sg(_arg: u32, : *mut qla2_sgx, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qla24xx_configure_prot_mode(: *mut srb_t, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla2x00_sp_release(kref: *mut kref);
}
extern "C" {
    pub fn qla2x00_els_dcmd2_iocb_timeout(data: *mut c_void);
}
//
// Global Function Prototypes in qla_mbx.c source file.
//
extern "C" {
    pub fn qla24xx_abort_command(: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_async_abort_command(: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla84xx_verify_chip(: *mut scsi_qla_host, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla81xx_idc_ack(: *mut scsi_qla_host_t, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla81xx_fac_semaphore_access(: *mut scsi_qla_host_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla2x00_get_data_rate(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_send_mb_cmd(: *mut scsi_qla_host, : *mut mbx_cmd_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_gpdb_wait(: *mut scsi_qla_host, : *mut fc_port_t, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qla24xx_print_fc_port_id(: *mut scsi_qla_host, : *mut seq_file, _arg: u16) -> c_int;
}
extern "C" {
    pub fn qla27xx_get_zio_threshold(: *mut scsi_qla_host_t, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla27xx_set_zio_threshold(: *mut scsi_qla_host_t, _arg: u16) -> c_int;
}
extern "C" {
    pub fn qla24xx_res_count_wait(: *mut scsi_qla_host, : *mut u16, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla_no_op_mb(vha: *mut scsi_qla_host);
}
//
// Global Function Prototypes in qla_isr.c source file.
//
extern "C" {
    pub fn qla2100_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla2300_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla24xx_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla2x00_process_response_queue(: *mut rsp_que);
}
extern "C" {
    pub fn qla2x00_request_irqs(: *mut qla_hw_data, : *mut rsp_que) -> c_int;
}
extern "C" {
    pub fn qla2x00_free_irqs(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_get_data_rate(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn __qla_consume_iocb(: *mut scsi_qla_host, : *mut c_void, : *mut rsp_que);
}
extern "C" {
    pub fn qla2xxx_process_purls_iocb(pkt: *mut c_void, rsp: *mut rsp_que);
}
//
// Global Function Prototypes in qla_sup.c source file.
//
extern "C" {
    pub fn qla2x00_is_a_vp_did(: *mut scsi_qla_host_t, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qla2x00_check_reg32_for_disconnect(: *mut scsi_qla_host_t, _arg: u32) -> bool;
}
extern "C" {
    pub fn qla2x00_check_reg16_for_disconnect(: *mut scsi_qla_host_t, _arg: u16) -> bool;
}
extern "C" {
    pub fn qla2x00_beacon_on(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_beacon_off(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla2x00_beacon_blink(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla24xx_beacon_on(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla24xx_beacon_off(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla24xx_beacon_blink(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla83xx_beacon_blink(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla82xx_beacon_on(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla82xx_beacon_off(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla83xx_wr_reg(: *mut scsi_qla_host_t, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qla83xx_rd_reg(: *mut scsi_qla_host_t, _arg: u32, : *mut u32) -> c_int;
}
extern "C" {
    pub fn qla83xx_restart_nic_firmware(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla8044_watchdog(vha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_get_flash_version(: *mut scsi_qla_host_t, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn qla24xx_get_flash_version(: *mut scsi_qla_host_t, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn qla82xx_get_flash_version(: *mut scsi_qla_host_t, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn qla2xxx_get_flash_info(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2xxx_get_vpd_field(: *mut scsi_qla_host_t, : *mut c_char, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn qla2xxx_flash_npiv_conf(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla24xx_read_fcp_prio_cfg(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_mailbox_passthru(bsg_job: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn qla2x00_sys_ld_info(bsg_job: *mut bsg_job) -> c_int;
}
//
// Global Function Prototypes in qla_dbg.c source file.
//
extern "C" {
    pub fn qla2xxx_dump_fw(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2100_fw_dump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2300_fw_dump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla24xx_fw_dump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla25xx_fw_dump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla81xx_fw_dump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_fw_dump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla8044_fw_dump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla27xx_fwdump(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla27xx_mpi_fwdump(: *mut scsi_qla_host_t, _arg: c_int);
}
extern "C" {
    pub fn qla27xx_fwdt_calculate_dump_size(: *mut scsi_qla_host, : *mut c_void) -> c_ulong;
}
extern "C" {
    pub fn qla27xx_fwdt_template_valid(: *mut c_void) -> c_int;
}
extern "C" {
    pub fn qla27xx_fwdt_template_size(: *mut c_void) -> c_ulong;
}
extern "C" {
    pub fn qla2xxx_dump_post_process(: *mut scsi_qla_host_t, _arg: c_int);
}
extern "C" {
    pub fn ql_dump_regs(_arg: c_uint, : *mut scsi_qla_host_t, _arg: c_uint);
}
extern "C" {
    pub fn ql_dump_buffer(_arg: c_uint, : *mut scsi_qla_host_t, _arg: c_uint, : *const c_void, _arg: c_uint);
}
//
// Global Function Prototypes in qla_gs.c source file.
//
extern "C" {
    pub fn qla2x00_ga_nxt(: *mut scsi_qla_host_t, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_gid_pt(: *mut scsi_qla_host_t, : *mut sw_info_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_gpn_id(: *mut scsi_qla_host_t, : *mut sw_info_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_gnn_id(: *mut scsi_qla_host_t, : *mut sw_info_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_gff_id(: *mut scsi_qla_host_t, : *mut sw_info_t);
}
extern "C" {
    pub fn qla2x00_rft_id(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_rff_id(: *mut scsi_qla_host_t, _arg: u8) -> c_int;
}
extern "C" {
    pub fn qla2x00_rnn_id(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_rsnn_nn(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_fdmi_register(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_gfpn_id(: *mut scsi_qla_host_t, : *mut sw_info_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_get_sym_node_name(: *mut scsi_qla_host_t, : *mut u8, _arg: usize) -> usize;
}
extern "C" {
    pub fn qla2x00_async_iocb_timeout(data: *mut c_void);
}
extern "C" {
    pub fn qla24xx_post_gpsc_work(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_async_gpsc(: *mut scsi_qla_host_t, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_handle_gpsc_event(: *mut scsi_qla_host_t, : *mut event_arg);
}
extern "C" {
    pub fn qla2x00_mgmt_svr_login(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_async_gffid(vha: *mut scsi_qla_host_t, fcport: *mut fc_port_t, _arg: bool) -> c_int;
}
extern "C" {
    pub fn qla_fab_async_scan(: *mut scsi_qla_host_t, : *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla_fab_scan_start(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla_fab_scan_finish(: *mut scsi_qla_host_t, : *mut srb_t);
}
extern "C" {
    pub fn qla24xx_post_gfpnid_work(: *mut scsi_qla_host, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_async_gfpnid(: *mut scsi_qla_host_t, : *mut fc_port_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_handle_gfpnid_event(: *mut scsi_qla_host_t, : *mut event_arg);
}
extern "C" {
    pub fn qla24xx_sp_unmap(: *mut scsi_qla_host_t, : *mut srb_t);
}
extern "C" {
    pub fn qla_scan_work_fn(: *mut work_struct);
}
extern "C" {
    pub fn qla25xx_fdmi_port_speed_capability(: *mut qla_hw_data) -> c_uint;
}
extern "C" {
    pub fn qla25xx_fdmi_port_speed_currently(: *mut qla_hw_data) -> c_uint;
}
//
// Global Function Prototypes in qla_attr.c source file.
//
extern "C" {
    pub fn qla2x00_alloc_sysfs_attr(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_free_sysfs_attr(: *mut scsi_qla_host_t, _arg: bool);
}
extern "C" {
    pub fn qla2x00_init_host_attr(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_alloc_sysfs_attr(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla2x00_loopback_test(: *mut scsi_qla_host_t, : *mut msg_echo_lb, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla24xx_update_all_fcp_prio(: *mut scsi_qla_host_t) -> c_int;
}
//
// Global Function Prototypes in qla_dfs.c source file.
//
extern "C" {
    pub fn qla2x00_dfs_setup(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_dfs_remove(: *mut scsi_qla_host_t) -> c_int;
}
// Globa function prototypes for multi-q
extern "C" {
    pub fn qla25xx_init_req_que(: *mut scsi_qla_host, : *mut req_que) -> c_int;
}
extern "C" {
    pub fn qla25xx_init_rsp_que(: *mut scsi_qla_host, : *mut rsp_que) -> c_int;
}
extern "C" {
    pub fn qla2x00_init_response_q_entries(: *mut rsp_que);
}
extern "C" {
    pub fn qla25xx_delete_req_que(: *mut scsi_qla_host, : *mut req_que) -> c_int;
}
extern "C" {
    pub fn qla25xx_delete_rsp_que(: *mut scsi_qla_host, : *mut rsp_que) -> c_int;
}
extern "C" {
    pub fn qla25xx_delete_queues(: *mut scsi_qla_host) -> c_int;
}
// qlafx00 related functions
extern "C" {
    pub fn qlafx00_pci_config(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qlafx00_initialize_adapter(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qlafx00_soft_reset(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qlafx00_chip_diag(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qlafx00_config_rings(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qlafx00_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qlafx00_enable_intrs(: *mut qla_hw_data);
}
extern "C" {
    pub fn qlafx00_disable_intrs(: *mut qla_hw_data);
}
extern "C" {
    pub fn qlafx00_abort_target(: *mut fc_port_t, _arg: u64, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlafx00_lun_reset(: *mut fc_port_t, _arg: u64, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlafx00_start_scsi(: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qlafx00_abort_isp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qlafx00_iospace_config(: *mut qla_hw_data) -> c_int;
}
extern "C" {
    pub fn qlafx00_init_firmware(: *mut scsi_qla_host_t, _arg: u16) -> c_int;
}
extern "C" {
    pub fn qlafx00_driver_shutdown(: *mut scsi_qla_host_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qlafx00_fw_ready(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qlafx00_configure_devices(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qlafx00_reset_initialize(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qlafx00_fx_disc(: *mut scsi_qla_host_t, : *mut fc_port_t, _arg: u16) -> c_int;
}
extern "C" {
    pub fn qlafx00_process_aen(: *mut scsi_qla_host, : *mut qla_work_evt);
}
extern "C" {
    pub fn qlafx00_get_host_speed(: *mut Scsi_Host);
}
extern "C" {
    pub fn qlafx00_init_response_q_entries(: *mut rsp_que);
}
extern "C" {
    pub fn qlafx00_tm_iocb(: *mut srb_t, : *mut tsk_mgmt_entry_fx00);
}
extern "C" {
    pub fn qlafx00_abort_iocb(: *mut srb_t, : *mut abort_iocb_entry_fx00);
}
extern "C" {
    pub fn qlafx00_fxdisc_iocb(: *mut srb_t, : *mut fxdisc_entry_fx00);
}
extern "C" {
    pub fn qlafx00_timer_routine(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qlafx00_rescan_isp(: *mut scsi_qla_host_t) -> c_int;
}
// qla82xx related functions
// PCI related functions
extern "C" {
    pub fn qla82xx_pci_config(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla82xx_pci_mem_read_2M(: *mut qla_hw_data, _arg: u64, : *mut c_void, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla82xx_iospace_config(: *mut qla_hw_data) -> c_int;
}
// Initialization related functions
extern "C" {
    pub fn qla82xx_reset_chip(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla82xx_config_rings(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla82xx_watchdog(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_start_firmware(: *mut scsi_qla_host_t) -> c_int;
}
// Firmware and flash related functions
extern "C" {
    pub fn qla82xx_load_risc(: *mut scsi_qla_host_t, : *mut u32) -> c_int;
}
// Mailbox related functions
extern "C" {
    pub fn qla82xx_abort_isp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_restart_isp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla_mpipt_validate_fw(vha: *mut scsi_qla_host_t, img_idx: u16, state: *mut u16) -> c_int;
}
// IOCB related functions
extern "C" {
    pub fn qla82xx_start_scsi(: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_sp_free(sp: *mut srb_t);
}
extern "C" {
    pub fn qla2x00_sp_timeout(: *mut timer_list);
}
extern "C" {
    pub fn qla2x00_bsg_job_done(sp: *mut srb_t, _arg: c_int);
}
extern "C" {
    pub fn qla2x00_bsg_sp_free(sp: *mut srb_t);
}
extern "C" {
    pub fn qla2x00_start_iocbs(: *mut scsi_qla_host, : *mut req_que);
}
// Interrupt related
extern "C" {
    pub fn qla82xx_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla82xx_msix_default(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla82xx_msix_rsp_q(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla82xx_enable_intrs(: *mut qla_hw_data);
}
extern "C" {
    pub fn qla82xx_disable_intrs(: *mut qla_hw_data);
}
extern "C" {
    pub fn qla82xx_poll(_arg: c_int, : *mut c_void);
}
extern "C" {
    pub fn qla82xx_init_flags(: *mut qla_hw_data);
}
// ISP 8021 hardware related
extern "C" {
    pub fn qla82xx_set_drv_active(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_wr_32(: *mut qla_hw_data, _arg: c_ulong, _arg: u32) -> c_int;
}
extern "C" {
    pub fn qla82xx_rd_32(: *mut qla_hw_data, _arg: c_ulong) -> c_int;
}
// ISP 8021 IDC
extern "C" {
    pub fn qla82xx_clear_drv_active(: *mut qla_hw_data);
}
extern "C" {
    pub fn qla82xx_idc_lock(: *mut qla_hw_data) -> c_int;
}
extern "C" {
    pub fn qla82xx_idc_unlock(: *mut qla_hw_data);
}
extern "C" {
    pub fn qla82xx_device_state_handler(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla8xxx_dev_failed_handler(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_clear_qsnt_ready(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_mbx_intr_enable(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_mbx_intr_disable(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_start_iocbs(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_fcoe_ctx_reset(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_check_md_needed(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_chip_reset_cleanup(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla81xx_set_led_config(: *mut scsi_qla_host_t, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla81xx_get_led_config(: *mut scsi_qla_host_t, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla82xx_mbx_beacon_ctl(: *mut scsi_qla_host_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn qla82xx_clear_pending_mbx(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_read_temperature(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla8044_read_temperature(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla2x00_read_sfp_dev(: *mut scsi_qla_host, : *mut c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ql26xx_led_config(: *mut scsi_qla_host_t, _arg: u16, : *mut u16) -> c_int;
}
// BSG related functions
extern "C" {
    pub fn qla24xx_bsg_request(: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn qla24xx_bsg_timeout(: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn qla84xx_reset_chip(: *mut scsi_qla_host_t, _arg: u16) -> c_int;
}
extern "C" {
    pub fn qla24xx_sadb_update(bsg_job: *mut bsg_job) -> c_int;
}
// 83xx related functions
extern "C" {
    pub fn qla83xx_fw_dump(vha: *mut scsi_qla_host_t);
}
// Minidump related functions
extern "C" {
    pub fn qla82xx_md_get_template_size(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_md_get_template(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_md_alloc(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_md_free(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_md_collect(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla82xx_md_prep(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_set_reset_owner(: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla82xx_validate_template_chksum(vha: *mut scsi_qla_host_t) -> c_int;
}
// Function declarations for ISP8044
extern "C" {
    pub fn qla8044_idc_lock(ha: *mut qla_hw_data) -> c_int;
}
extern "C" {
    pub fn qla8044_idc_unlock(ha: *mut qla_hw_data);
}
extern "C" {
    pub fn qla8044_rd_reg(ha: *mut qla_hw_data, addr: c_ulong) -> u32;
}
extern "C" {
    pub fn qla8044_wr_reg(ha: *mut qla_hw_data, addr: c_ulong, val: u32);
}
extern "C" {
    pub fn qla8044_read_reset_template(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla8044_set_idc_dontreset(ha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla8044_rd_direct(vha: *mut scsi_qla_host, crb_reg: u32) -> c_int;
}
extern "C" {
    pub fn qla8044_device_state_handler(vha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla8044_clear_qsnt_ready(vha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla8044_clear_drv_active(: *mut qla_hw_data);
}
extern "C" {
    pub fn qla8044_get_minidump(vha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla8044_collect_md_data(vha: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla8044_md_get_template(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla8044_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn qla82xx_mbx_completion(: *mut scsi_qla_host_t, _arg: u16);
}
extern "C" {
    pub fn qla8044_abort_isp(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla8044_check_fw_alive(: *mut scsi_qla_host) -> c_int;
}
extern "C" {
    pub fn qla_set_exlogin_mem_cfg(vha: *mut scsi_qla_host_t, phys_addr: dma_addr_t) -> c_int;
}
extern "C" {
    pub fn qla_get_exchoffld_status(: *mut scsi_qla_host_t, : *mut u16, : *mut u16) -> c_int;
}
extern "C" {
    pub fn qla_set_exchoffld_mem_cfg(: *mut scsi_qla_host_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_do_nack_work(: *mut scsi_qla_host, : *mut qla_work_evt);
}
extern "C" {
    pub fn qlt_plogi_ack_unref(: *mut scsi_qla_host, : *mut qlt_plogi_ack_t);
}
extern "C" {
    pub fn qlt_schedule_sess_for_deletion(: *mut fc_port);
}
extern "C" {
    pub fn qla24xx_delete_sess_fn(: *mut work_struct);
}
extern "C" {
    pub fn qlt_unknown_atio_work_fn(: *mut work_struct);
}
extern "C" {
    pub fn qla_update_host_map(: *mut scsi_qla_host, _arg: port_id_t);
}
extern "C" {
    pub fn qla_remove_hostmap(ha: *mut qla_hw_data);
}
extern "C" {
    pub fn qlt_clr_qp_table(vha: *mut scsi_qla_host);
}
extern "C" {
    pub fn qlt_set_mode(: *mut scsi_qla_host);
}
extern "C" {
    pub fn qla2x00_set_data_rate(vha: *mut scsi_qla_host_t, mode: u16) -> c_int;
}
extern "C" {
    pub fn qla24xx_process_purex_list(: *mut purex_list);
}
extern "C" {
    pub fn qla2x00_dfs_create_rport(vha: *mut scsi_qla_host_t, fp: *mut fc_port);
}
extern "C" {
    pub fn qla2x00_dfs_remove_rport(vha: *mut scsi_qla_host_t, fp: *mut fc_port);
}
extern "C" {
    pub fn qla_wait_nvme_release_cmd_kref(sp: *mut srb_t);
}
extern "C" {
    pub fn qla_nvme_abort_set_option(pkt: *mut c_void, sp: *mut srb_t);
}
extern "C" {
    pub fn qla_nvme_abort_process_comp_status(pkt: *mut c_void, sp: *mut srb_t);
}
// nvme.c
extern "C" {
    pub fn qla_nvme_unregister_remote_port(fcport: *mut fc_port);
}
// qla_edif.c
extern "C" {
    pub fn qla_edb_stop(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla_edif_app_mgmt(bsg_job: *mut bsg_job) -> i32;
}
extern "C" {
    pub fn qla_enode_init(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla_enode_stop(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla_edif_flush_sa_ctl_lists(fcport: *mut fc_port_t);
}
extern "C" {
    pub fn qla_edb_init(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla_edif_timer(vha: *mut scsi_qla_host_t);
}
extern "C" {
    pub fn qla28xx_start_scsi_edif(sp: *mut srb_t) -> c_int;
}
extern "C" {
    pub fn qla24xx_sa_update_iocb(sp: *mut srb_t, sa_update_iocb: *mut sa_update_28xx);
}
extern "C" {
    pub fn qla24xx_sa_replace_iocb(sp: *mut srb_t, sa_update_iocb: *mut sa_update_28xx);
}
extern "C" {
    pub fn qla24xx_auth_els(vha: *mut scsi_qla_host_t, pkt: *mut c_void, rsp: *mut rsp_que);
}
extern "C" {
    pub fn qla_handle_els_plogi_done(vha: *mut scsi_qla_host_t, ea: *mut event_arg);
}

pub const QLA2XX_MAX_LINK_DOWN_TIME: c_int = 100;
extern "C" {
    pub fn qla2xxx_start_stats(shost: *mut Scsi_Host, flags: u32) -> c_int;
}
extern "C" {
    pub fn qla2xxx_stop_stats(shost: *mut Scsi_Host, flags: u32) -> c_int;
}
extern "C" {
    pub fn qla2xxx_reset_stats(shost: *mut Scsi_Host, flags: u32) -> c_int;
}
extern "C" {
    pub fn qla2xxx_get_ini_stats(shost: *mut Scsi_Host, flags: u32, data: *mut c_void, size: u64) -> c_int;
}
extern "C" {
    pub fn qla2xxx_disable_port(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn qla2xxx_enable_port(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn qla2x00_get_num_tgts(vha: *mut scsi_qla_host_t) -> u64;
}
extern "C" {
    pub fn qla2x00_count_set_bits(num: u32) -> u64;
}
extern "C" {
    pub fn qla_create_buf_pool(: *mut scsi_qla_host, : *mut qla_qpair) -> c_int;
}
extern "C" {
    pub fn qla_free_buf_pool(: *mut qla_qpair);
}
extern "C" {
    pub fn qla_get_buf(: *mut scsi_qla_host, : *mut qla_qpair, : *mut qla_buf_dsc) -> c_int;
}
extern "C" {
    pub fn qla_put_buf(: *mut qla_qpair, : *mut qla_buf_dsc);
}
