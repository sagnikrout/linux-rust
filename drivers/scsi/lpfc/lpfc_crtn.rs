//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/lpfc/lpfc_crtn.h
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


//
// This file is part of the Emulex Linux Device Driver for
// Fibre Channel Host Bus Adapters.
// Copyright (C) 2017-2026 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
// Copyright (C) 2004-2016 Emulex.  All rights reserved.
// EMULEX and SLI are trademarks of Emulex.
// www.broadcom.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General
// Public License as published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful.
// ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
// WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT, ARE
// DISCLAIMED, EXCEPT TO THE EXTENT THAT SUCH DISCLAIMERS ARE HELD
// TO BE LEGALLY INVALID.  See the GNU General Public License for
// more details, a copy of which can be found in the file COPYING
// included with this package.
//
extern "C" {
    pub fn int(: *mut *mut node_filter)(struct lpfc_nodelist, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn lpfc_down_link(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_sli_read_link_ste(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_dump_mem(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: u16, _arg: u16);
}
extern "C" {
    pub fn lpfc_dump_wakeup_param(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_dump_static_vport(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_dump_cfg_rg23(: *mut lpfc_hba, : *mut lpfcMboxq) -> c_int;
}
extern "C" {
    pub fn lpfc_read_nv(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_config_async(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: u32);
}
extern "C" {
    pub fn lpfc_mbox_rsrc_prep(phba: *mut lpfc_hba, mbox: *mut LPFC_MBOXQ_t) -> c_int;
}
extern "C" {
    pub fn lpfc_heart_beat(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_read_topology(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, : *mut lpfc_dmabuf) -> c_int;
}
extern "C" {
    pub fn lpfc_clear_la(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_issue_clear_la(: *mut lpfc_hba, : *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_config_link(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_config_msi(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t) -> c_int;
}
extern "C" {
    pub fn lpfc_read_sparam(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_read_config(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_read_lnk_stat(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_set_var(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn lpfc_unreg_login(: *mut lpfc_hba, _arg: u16, _arg: u32, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_unreg_did(: *mut lpfc_hba, _arg: u16, _arg: u32, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_sli4_unreg_all_rpis(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_reg_vpi(: *mut lpfc_vport, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_unreg_vpi(: *mut lpfc_hba, _arg: u16, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_init_link(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: u32, _arg: u32);
}
extern "C" {
    pub fn lpfc_request_features(: *mut lpfc_hba, : *mut lpfcMboxq);
}
extern "C" {
    pub fn lpfc_get_sli4_parameters(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t) -> c_int;
}
extern "C" {
    pub fn lpfc_reg_congestion_buf(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_unreg_congestion_buf(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_cleanup_rcv_buffers(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_rcv_seq_check_edtov(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_cleanup_rpis(: *mut lpfc_vport, _arg: c_int);
}
extern "C" {
    pub fn lpfc_cleanup_pending_mbox(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_linkdown(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_linkdown_port(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_port_link_failure(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_read_topology(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_init_vpi_cmpl(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_cancel_all_vport_retry_delay_timer(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_retry_pport_discovery(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_init_iocb_list(phba: *mut lpfc_hba, cnt: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_free_iocb_list(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_read_lds_params(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_calc_cmf_latency(phba: *mut lpfc_hba) -> u32;
}
extern "C" {
    pub fn lpfc_cmf_signal_init(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_cmf_start(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_cmf_stop(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_init_congestion_stat(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_init_congestion_buf(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_cgn_params_read(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_cgn_calc_crc32(data: *const c_void, size: usize) -> u32;
}
extern "C" {
    pub fn lpfc_config_cgn_signal(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_cmf_sync_wqe(phba: *mut lpfc_hba, ms: u32, total: u64) -> c_int;
}
extern "C" {
    pub fn lpfc_cgn_dump_rxmonitor(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_cgn_update_stat(phba: *mut lpfc_hba, dtag: u32);
}
extern "C" {
    pub fn lpfc_unblock_requests(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_block_requests(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_rx_monitor_destroy_ring(rx_monitor: *mut lpfc_rx_info_monitor);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_local_config_link(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_reg_login(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_dflt_rpi(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_fabric_reg_login(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_ns_reg_login(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_fc_reg_login(phba: *mut lpfc_hba, pmb: *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_fdmi_reg_login(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbx_cmpl_reg_vfi(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_unregister_vfi_cmpl(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_enqueue_node(: *mut lpfc_vport, : *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_dequeue_node(: *mut lpfc_vport, : *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_nlp_set_state(: *mut lpfc_vport, : *mut lpfc_nodelist, _arg: c_int);
}
extern "C" {
    pub fn lpfc_nlp_reg_node(vport: *mut lpfc_vport, ndlp: *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_nlp_unreg_node(vport: *mut lpfc_vport, ndlp: *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_drop_node(: *mut lpfc_vport, : *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_set_disctmo(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_can_disctmo(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_unreg_rpi(: *mut lpfc_vport, : *mut lpfc_nodelist) -> c_int;
}
extern "C" {
    pub fn lpfc_unreg_all_rpis(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_unreg_hba_rpis(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_unreg_default_rpis(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_issue_reg_vpi(: *mut lpfc_hba, : *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_nlp_put(: *mut lpfc_nodelist) -> c_int;
}
extern "C" {
    pub fn lpfc_disc_list_loopmap(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_disc_start(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_cleanup_discovery_resources(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_cleanup(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_prep_embed_io(phba: *mut lpfc_hba, lpfc_ncmd: *mut lpfc_io_buf);
}
extern "C" {
    pub fn lpfc_disc_timeout(: *mut timer_list);
}
extern "C" {
    pub fn lpfc_unregister_fcf_prep(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_worker_wake_up(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_workq_post_event(: *mut lpfc_hba, : *mut c_void, : *mut c_void, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_do_work(: *mut c_void) -> c_int;
}
extern "C" {
    pub fn lpfc_do_scr_ns_plogi(: *mut lpfc_hba, : *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_els_abort(: *mut lpfc_hba, : *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_more_plogi(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_more_adisc(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_end_rscn(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_els_chk_latt(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_els_abort_flogi(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_initial_flogi(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_init_vfi(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_initial_fdisc(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_plogi(: *mut lpfc_vport, _arg: u32, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_prli(: *mut lpfc_vport, : *mut lpfc_nodelist, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_adisc(: *mut lpfc_vport, : *mut lpfc_nodelist, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_logo(: *mut lpfc_vport, : *mut lpfc_nodelist, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_npiv_logo(: *mut lpfc_vport, : *mut lpfc_nodelist) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_scr(vport: *mut lpfc_vport, retry: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_rscn(vport: *mut lpfc_vport, retry: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_fabric_reglogin(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_rdf(vport: *mut lpfc_vport, retry: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_els_edc(vport: *mut lpfc_vport, retry: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_els_rcv_fpin(vport: *mut lpfc_vport, p: *mut c_void, fpin_length: u32);
}
extern "C" {
    pub fn lpfc_els_free_iocb(: *mut lpfc_hba, : *mut lpfc_iocbq) -> c_int;
}
extern "C" {
    pub fn lpfc_ct_free_iocb(: *mut lpfc_hba, : *mut lpfc_iocbq) -> c_int;
}
extern "C" {
    pub fn lpfc_cancel_retry_delay_tmo(: *mut lpfc_vport, : *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_els_retry_delay(: *mut timer_list);
}
extern "C" {
    pub fn lpfc_els_retry_delay_handler(: *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_els_handle_rscn(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_els_flush_rscn(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_rscn_payload_check(: *mut lpfc_vport, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_els_flush_all_cmd(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_els_flush_cmd(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_els_disc_adisc(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_els_disc_plogi(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_els_timeout(: *mut timer_list);
}
extern "C" {
    pub fn lpfc_els_timeout_handler(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_sli_prep_wqe(phba: *mut lpfc_hba, job: *mut lpfc_iocbq);
}
extern "C" {
    pub fn lpfc_hb_timeout_handler(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_ct_handle_unsol_abort(: *mut lpfc_hba, : *mut hbq_dmabuf) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_gidpt(vport: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_gidft(vport: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_get_gidft_type(vport: *mut lpfc_vport, iocbq: *mut lpfc_iocbq) -> c_int;
}
extern "C" {
    pub fn lpfc_ns_cmd(: *mut lpfc_vport, _arg: c_int, _arg: u8, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_fdmi_cmd(: *mut lpfc_vport, : *mut lpfc_nodelist, _arg: c_int, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_fdmi_change_check(vport: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_delayed_disc_tmo(: *mut timer_list);
}
extern "C" {
    pub fn lpfc_delayed_disc_timeout_handler(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_config_port_prep(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_update_vport_wwn(vport: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_config_port_post(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_refresh_params(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_hba_down_prep(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_hba_down_post(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_hba_init(: *mut lpfc_hba, : *mut u32);
}
extern "C" {
    pub fn lpfc_sli3_post_buffer(phba: *mut lpfc_hba, pring: *mut lpfc_sli_ring, cnt: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_decode_firmware_rev(: *mut lpfc_hba, : *mut c_char, _arg: c_int);
}
extern "C" {
    pub fn lpfc_online(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_unblock_mgmt_io(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_offline_prep(: *mut lpfc_hba, _arg: c_int);
}
extern "C" {
    pub fn lpfc_offline(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_reset_hba(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_setup(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_setup(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_queue_init(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_queue_init(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_handle_eratt(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_handle_latt(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lpfc_sli_sp_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lpfc_sli_fp_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lpfc_sli4_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lpfc_sli4_hba_intr_handler(_arg: c_int, : *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lpfc_sli4_hba_intr_handler_th(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lpfc_sli4_cleanup_poll_list(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_poll_hbtimer(t: *mut timer_list);
}
extern "C" {
    pub fn lpfc_sli4_start_polling(q: *mut lpfc_queue);
}
extern "C" {
    pub fn lpfc_sli4_stop_polling(q: *mut lpfc_queue);
}
extern "C" {
    pub fn lpfc_read_rev(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_sli4_swap_str(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_config_ring(: *mut lpfc_hba, _arg: c_int, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_config_port(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_kill_board(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbox_put(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn __lpfc_mbox_cmpl_put(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbox_cmpl_put(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_mbox_cmd_check(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t) -> c_int;
}
extern "C" {
    pub fn lpfc_mbox_dev_check(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_mbox_tmo_val(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t) -> c_int;
}
extern "C" {
    pub fn lpfc_init_vfi(: *mut lpfcMboxq, : *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_reg_vfi(: *mut lpfcMboxq, : *mut lpfc_vport, _arg: dma_addr_t);
}
extern "C" {
    pub fn lpfc_init_vpi(: *mut lpfc_hba, : *mut lpfcMboxq, _arg: u16);
}
extern "C" {
    pub fn lpfc_unreg_vfi(: *mut lpfcMboxq, : *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_reg_fcfi(: *mut lpfc_hba, : *mut lpfcMboxq);
}
extern "C" {
    pub fn lpfc_reg_fcfi_mrq(phba: *mut lpfc_hba, mbox: *mut lpfcMboxq, mode: c_int);
}
extern "C" {
    pub fn lpfc_unreg_fcfi(: *mut lpfcMboxq, _arg: u16);
}
extern "C" {
    pub fn lpfc_resume_rpi(: *mut lpfcMboxq, : *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_check_pending_fcoe_event(: *mut lpfc_hba, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_init_vpi(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_els_hbq_free(: *mut lpfc_hba, : *mut hbq_dmabuf);
}
extern "C" {
    pub fn lpfc_sli4_rb_free(: *mut lpfc_hba, : *mut hbq_dmabuf);
}
extern "C" {
    pub fn lpfc_sli4_nvmet_free(phba: *mut lpfc_hba, dmab: *mut rqb_dmabuf);
}
extern "C" {
    pub fn lpfc_nvmet_wqfull_process(phba: *mut lpfc_hba, wq: *mut lpfc_queue);
}
extern "C" {
    pub fn lpfc_nvme_wait_for_io_drain(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_free_rq_buffer(phba: *mut lpfc_hba, hq: *mut lpfc_queue) -> c_int;
}
extern "C" {
    pub fn lpfc_unregister_fcf(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_unregister_fcf_rescan(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_unregister_unused_fcf(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_redisc_fcf_table(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_fcf_redisc_wait_start_timer(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_fcf_dead_failthrough(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_fcf_rr_next_index_get(: *mut lpfc_hba) -> u16;
}
extern "C" {
    pub fn lpfc_sli4_set_fcf_flogi_fail(: *mut lpfc_hba, _arg: u16);
}
extern "C" {
    pub fn lpfc_sli4_fcf_rr_index_set(: *mut lpfc_hba, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_fcf_rr_index_clear(: *mut lpfc_hba, _arg: u16);
}
extern "C" {
    pub fn lpfc_sli4_fcf_rr_next_proc(: *mut lpfc_vport, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_clear_fcf_rr_bmask(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_mem_alloc(: *mut lpfc_hba, align: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_nvmet_mem_alloc(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_mem_alloc_active_rrq_pool_s4(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_mem_free(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_mem_free_all(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_stop_vport_timers(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_poll_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn lpfc_poll_start_timer(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_poll_eratt(: *mut timer_list);
}
extern "C" {
    pub fn lpfc_sli_get_iocbq(: *mut lpfc_hba) -> *mut lpfc_iocbq;
}
extern "C" {
    pub fn lpfc_sli_release_iocbq(: *mut lpfc_hba, : *mut lpfc_iocbq);
}
extern "C" {
    pub fn lpfc_sli_next_iotag(: *mut lpfc_hba, : *mut lpfc_iocbq) -> u16;
}
extern "C" {
    pub fn lpfc_sli_wake_mbox_wait(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_selective_reset(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_reset_barrier(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_brdready(: *mut lpfc_hba, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_brdkill(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_chipset_init(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_brdreset(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_brdrestart(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_hba_setup(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_config_port(: *mut lpfc_hba, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_host_down(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_hba_down(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_issue_mbox(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_handle_mb_event(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_mbox_sys_shutdown(: *mut lpfc_hba, _arg: c_int);
}
extern "C" {
    pub fn lpfc_sli_check_eratt(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_handle_received_buffer(: *mut lpfc_hba, : *mut hbq_dmabuf);
}
extern "C" {
    pub fn lpfc_sli_def_mbox_cmpl(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_sli4_unreg_rpi_cmpl_clr(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t);
}
extern "C" {
    pub fn lpfc_sli_pcimem_bcopy(: *mut c_void, : *mut c_void, _arg: u32);
}
extern "C" {
    pub fn lpfc_sli_bemem_bcopy(: *mut c_void, : *mut c_void, _arg: u32);
}
extern "C" {
    pub fn lpfc_sli_abort_iocb_ring(: *mut lpfc_hba, : *mut lpfc_sli_ring);
}
extern "C" {
    pub fn lpfc_sli_abort_fcp_rings(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_hba_iocb_abort(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_flush_io_rings(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_get_buffer_tag(: *mut lpfc_hba) -> u32;
}
extern "C" {
    pub fn lpfc_sli_hbq_count() -> c_int;
}
extern "C" {
    pub fn lpfc_sli_hbqbuf_add_hbqs(: *mut lpfc_hba, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_hbqbuf_free_all(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_hbq_size() -> c_int;
}
extern "C" {
    pub fn lpfc_sli_sum_iocb(: *mut lpfc_vport, _arg: u16, _arg: u64, _arg: lpfc_ctx_cmd) -> c_int;
}
extern "C" {
    pub fn lpfc_mbox_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn lpfc_mbox_timeout_handler(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_issue_hb_mbox(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_hb_tmo(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli_issue_mbox_wait(: *mut lpfc_hba, : *mut LPFC_MBOXQ_t, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_free_hbq(: *mut lpfc_hba, : *mut hbq_dmabuf);
}
extern "C" {
    pub fn __lpfc_mbuf_free(: *mut lpfc_hba, : *mut c_void, _arg: dma_addr_t);
}
extern "C" {
    pub fn lpfc_mbuf_free(: *mut lpfc_hba, : *mut c_void, _arg: dma_addr_t);
}
extern "C" {
    pub fn lpfc_nvmet_buf_free(phba: *mut lpfc_hba, virtp: *mut c_void, dma: dma_addr_t);
}
extern "C" {
    pub fn lpfc_in_buf_free(: *mut lpfc_hba, : *mut lpfc_dmabuf);
}
extern "C" {
    pub fn lpfc_rq_buf_free(phba: *mut lpfc_hba, mp: *mut lpfc_dmabuf);
}
extern "C" {
    pub fn lpfc_setup_fdmi_mask(vport: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_link_reset(vport: *mut lpfc_vport) -> c_int;
}
// Function prototypes.
extern "C" {
    pub fn lpfc_check_pci_resettable(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_info(: *mut Scsi_Host) -> *const c_char;
}
extern "C" {
    pub fn lpfc_scan_finished(: *mut Scsi_Host, long: unsigned) -> c_int;
}
extern "C" {
    pub fn lpfc_init_api_table_setup(: *mut lpfc_hba, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_api_table_setup(: *mut lpfc_hba, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_scsi_api_table_setup(: *mut lpfc_hba, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_mbox_api_table_setup(: *mut lpfc_hba, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_api_table_setup(: *mut lpfc_hba, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_get_cfgparam(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_get_vport_cfgparam(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_alloc_sysfs_attr(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_free_sysfs_attr(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_vport_symbolic_node_name(: *mut lpfc_vport, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn lpfc_vport_symbolic_port_name(: *mut lpfc_vport, : *mut c_char, _arg: usize) -> c_int;
}
extern "C" {
    pub fn lpfc_terminate_rport_io(: *mut fc_rport);
}
extern "C" {
    pub fn lpfc_dev_loss_tmo_callbk(rport: *mut fc_rport);
}
extern "C" {
    pub fn lpfc_vport_disable(fc_vport: *mut fc_vport, disable: bool) -> c_int;
}
extern "C" {
    pub fn lpfc_mbx_unreg_vpi(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn destroy_port(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_get_instance() -> c_int;
}
extern "C" {
    pub fn lpfc_host_attrib_init(: *mut Scsi_Host);
}
extern "C" {
    pub fn lpfc_debugfs_initialize(: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_debugfs_terminate(: *mut lpfc_vport);
}
// SLI4 if_type 2 externs.
extern "C" {
    pub fn lpfc_sli4_alloc_resource_identifiers(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_dealloc_resource_identifiers(: *mut lpfc_hba) -> c_int;
}
// Interface exported by fabric iocb scheduler
extern "C" {
    pub fn lpfc_fabric_abort_nport(: *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_fabric_abort_hba(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_fabric_block_timeout(: *mut timer_list);
}
extern "C" {
    pub fn lpfc_unblock_fabric_iocbs(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_rampdown_queue_depth(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_ramp_down_queue_handler(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_scsi_dev_block(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_free_fast_evt(: *mut lpfc_hba, : *mut lpfc_fast_path_event);
}
extern "C" {
    pub fn lpfc_create_static_vport(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_stop_hba_timers(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_stop_port(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_update_cmf_cmd(phba: *mut lpfc_hba, sz: u32) -> c_int;
}
extern "C" {
    pub fn __lpfc_sli4_stop_fcf_redisc_wait_timer(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_stop_fcf_redisc_wait_timer(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_parse_fcoe_conf(: *mut lpfc_hba, : *mut u8, _arg: u32);
}
extern "C" {
    pub fn lpfc_parse_vpd(: *mut lpfc_hba, : *mut u8, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_start_fdiscs(phba: *mut lpfc_hba);
}
pub const HBA_EVENT_RSCN: c_int = 5;
pub const HBA_EVENT_LINK_UP: c_int = 2;
pub const HBA_EVENT_LINK_DOWN: c_int = 3;
// functions to support SGIOv4/bsg interface
extern "C" {
    pub fn lpfc_bsg_request(: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn lpfc_bsg_timeout(: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn lpfc_bsg_ct_unsol_abort(: *mut lpfc_hba, : *mut hbq_dmabuf) -> c_int;
}
extern "C" {
    pub fn lpfc_drain_txq(: *mut lpfc_hba) -> u32;
}
extern "C" {
    pub fn lpfc_clr_rrq_active(: *mut lpfc_hba, _arg: u16, : *mut lpfc_node_rrq);
}
extern "C" {
    pub fn lpfc_test_rrq_active(: *mut lpfc_hba, : *mut lpfc_nodelist, _arg: u16) -> c_int;
}
extern "C" {
    pub fn lpfc_handle_rrq_active(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_send_rrq(: *mut lpfc_hba, : *mut lpfc_node_rrq) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_xri_inrange(: *mut lpfc_hba, _arg: u16) -> u16;
}
extern "C" {
    pub fn lpfc_cleanup_vports_rrqs(: *mut lpfc_vport, : *mut lpfc_nodelist);
}
extern "C" {
    pub fn lpfc_idiag_mbxacc_dump_issue_mbox(: *mut lpfc_hba, : *mut MAILBOX_t);
}
extern "C" {
    pub fn lpfc_wr_object(: *mut lpfc_hba, : *mut list_head, _arg: u32, : *mut u32) -> c_int;
}
// functions to support SR-IOV
extern "C" {
    pub fn lpfc_sli_probe_sriov_nr_virtfn(: *mut lpfc_hba, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_sli_sriov_nr_virtfn_get(: *mut lpfc_hba) -> u16;
}
extern "C" {
    pub fn lpfc_sli4_queue_create(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_queue_destroy(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_hba_init_link_fc_topology(: *mut lpfc_hba, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_reg_vfi(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_issue_unreg_vfi(: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_selective_reset(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_read_config(: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_node_rpi_restore(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_els_sgl_update(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_nvmet_sgl_update(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_io_buf_flush(phba: *mut lpfc_hba, sglist: *mut list_head) -> c_int;
}
extern "C" {
    pub fn lpfc_io_buf_replenish(phba: *mut lpfc_hba, cbuf: *mut list_head) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_io_sgl_update(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_new_io_buf(phba: *mut lpfc_hba, num_to_alloc: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_io_free(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_free_sgl_list(: *mut lpfc_hba, : *mut list_head);
}
extern "C" {
    pub fn lpfc_sli_port_speed_get(: *mut lpfc_hba) -> u32;
}
extern "C" {
    pub fn lpfc_sli4_request_firmware_update(: *mut lpfc_hba, _arg: u8) -> c_int;
}
extern "C" {
    pub fn lpfc_sli4_offline_eratt(: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_delete_device_data(: *mut lpfc_hba, lpfc_device_data*: *mut struct);
}
extern "C" {
    pub fn lpfc_sli4_dump_page_a0(phba: *mut lpfc_hba, mbox: *mut lpfcMboxq) -> c_int;
}
extern "C" {
    pub fn lpfc_mbx_cmpl_rdp_page_a0(phba: *mut lpfc_hba, pmb: *mut LPFC_MBOXQ_t);
}
// RAS Interface
extern "C" {
    pub fn lpfc_sli4_ras_init(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_sli4_ras_setup(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_ras_stop_fwlog(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_check_fwlog_support(phba: *mut lpfc_hba) -> c_int;
}
// NVME interfaces.
extern "C" {
    pub fn lpfc_nvme_create_localport(vport: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_nvme_destroy_localport(vport: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_nvme_update_localport(vport: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_nvmet_create_targetport(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_nvmet_update_targetport(phba: *mut lpfc_hba) -> c_int;
}
extern "C" {
    pub fn lpfc_nvmet_destroy_targetport(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_nvme_mod_param_dep(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_create_multixri_pools(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_create_destroy_pools(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_move_xri_pvt_to_pbl(phba: *mut lpfc_hba, hwqid: u32);
}
extern "C" {
    pub fn lpfc_move_xri_pbl_to_pvt(phba: *mut lpfc_hba, hwqid: u32, cnt: u32);
}
extern "C" {
    pub fn lpfc_adjust_high_watermark(phba: *mut lpfc_hba, hwqid: u32);
}
extern "C" {
    pub fn lpfc_keep_pvt_pool_above_lowwm(phba: *mut lpfc_hba, hwqid: u32);
}
extern "C" {
    pub fn lpfc_adjust_pvt_pool_count(phba: *mut lpfc_hba, hwqid: u32);
}

extern "C" {
    pub fn lpfc_snapshot_mxp(: *mut lpfc_hba, _arg: u32);
}

extern "C" {
    pub fn lpfc_io_ktime(phba: *mut lpfc_hba, ncmd: *mut lpfc_io_buf);
}
extern "C" {
    pub fn lpfc_wqe_cmd_template();
}
extern "C" {
    pub fn lpfc_nvmet_cmd_template();
}
extern "C" {
    pub fn lpfc_nvme_flush_abts_list(phba: *mut lpfc_hba);
}
extern "C" {
    pub fn lpfc_nvmels_flush_cmd(phba: *mut lpfc_hba);
}
// vmid interface
extern "C" {
    pub fn lpfc_vmid_uvem(vport: *mut lpfc_vport, vmid: *mut lpfc_vmid, ins: bool) -> c_int;
}
extern "C" {
    pub fn lpfc_vmid_get_cs_ctl(vport: *mut lpfc_vport) -> u32;
}
extern "C" {
    pub fn lpfc_vmid_hash_fn(vmid: *const c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn lpfc_vmid_vport_cleanup(vport: *mut lpfc_vport);
}
extern "C" {
    pub fn lpfc_issue_els_qfpa(vport: *mut lpfc_vport) -> c_int;
}
extern "C" {
    pub fn lpfc_reinit_vmid(vport: *mut lpfc_vport);
}
