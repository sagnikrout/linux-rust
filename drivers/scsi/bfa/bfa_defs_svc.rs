//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/bfa/bfa_defs_svc.h
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
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014- QLogic Corporation.
// All rights reserved
// www.qlogic.com
//
// Linux driver for QLogic BR-series Fibre Channel Host Bus Adapter.
//

pub const BFA_IOCFC_INTR_DELAY: c_int = 1125;
pub const BFA_IOCFC_INTR_LATENCY: c_int = 225;
pub const BFA_IOCFCOE_INTR_DELAY: c_int = 25;
pub const BFA_IOCFCOE_INTR_LATENCY: c_int = 5;
//
// Interrupt coalescing configuration.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocfc_intr_attr_s {
    pub /: *mut *mut u8 coalesce; / enable/disable coalescing,
    pub rsvd: [u8; 3],
    pub /: *mut *mut __be16 latency; / latency in microseconds,
    pub /: *mut *mut __be16 delay; / delay in microseconds,
}

//
// IOC firmware configuraton
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocfc_fwcfg_s {
    pub /: *mut *mut u16 num_fabrics; / number of fabrics,
    pub /: *mut *mut u16 num_lports; / number of local lports,
    pub /: *mut *mut u16 num_rports; / number of remote ports,
    pub /: *mut *mut u16 num_ioim_reqs; / number of IO reqs,
    pub /: *mut *mut u16 num_tskim_reqs; / task management requests,
    pub /: *mut *mut u16 num_fwtio_reqs; / number of TM IO reqs in FW,
    pub /: *mut *mut u16 num_fcxp_reqs; / unassisted FC exchanges,
    pub /: *mut *mut u16 num_uf_bufs; / unsolicited recv buffers,
    pub num_cqs: u8,
    pub /: *mut *mut u8 fw_tick_res; / FW clock resolution in ms,
    pub rsvd: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocfc_drvcfg_s {
    pub /: *mut *mut u16 num_reqq_elems; / number of req queue elements,
    pub /: *mut *mut u16 num_rspq_elems; / number of rsp queue elements,
    pub /: *mut *mut u16 num_sgpgs; / number of total SG pages,
    pub /: *mut *mut u16 num_sboot_tgts; / number of SAN boot targets,
    pub /: *mut *mut u16 num_sboot_luns; / number of SAN boot luns,
    pub /: *mut *mut u16 ioc_recover; / IOC recovery mode,
    pub /: *mut *mut u16 min_cfg; / minimum configuration,
    pub /: *mut *mut u16 path_tov; / device path timeout,
    pub /: *mut *mut u16 num_tio_reqs; / number of TM IO reqs,
    pub port_mode: u8,
    pub rsvd_a: u8,
    pub failed: *mut *mut bfa_boolean_t delay_comp; / delay completion of,
// inflight IOs
    pub /: *mut *mut u16 num_ttsk_reqs; / TM task management requests,
    pub rsvd: u32,
}

//
// IOC configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocfc_cfg_s {
    pub /: *mut *mut bfa_iocfc_fwcfg_s fwcfg; / firmware side config,
    pub /: *mut *mut bfa_iocfc_drvcfg_s drvcfg; / driver side config,
}

//
// IOC firmware IO stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_ioim_stats_s {
    pub driver*/: *mut *mut u32 host_abort; / IO aborted by host,
    pub /: *mut *mut u32 host_cleanup; / IO clean up by host driver,
    pub /: *mut *mut u32 fw_io_timeout; / IOs timedout,
    pub /: *mut *mut u32 fw_frm_parse; / frame parsed by f/w,
    pub /: *mut *mut u32 fw_frm_data; / fcp_data frame parsed by f/w,
    pub /: *mut *mut u32 fw_frm_rsp; / fcp_rsp frame parsed by f/w,
    pub /: *mut *mut u32 fw_frm_xfer_rdy; / xfer_rdy frame parsed by f/w,
    pub /: *mut *mut u32 fw_frm_bls_acc; / BLS ACC frame parsed by f/w,
    pub /: *mut *mut u32 fw_frm_tgt_abort; / target ABTS parsed by f/w,
    pub /: *mut *mut u32 fw_frm_unknown; / unknown parsed by f/w,
    pub /: *mut *mut u32 fw_data_dma; / f/w DMA'ed the data frame,
    pub /: *mut *mut u32 fw_frm_drop; / f/w drop the frame,
    pub /: *mut *mut u32 rec_timeout; / FW rec timed out,
    pub on: *mut *mut u32 error_rec; / FW sending rec,
// an error condition
    pub /: *mut *mut u32 wait_for_si; / FW wait for SI,
    pub /: *mut *mut u32 rec_rsp_inval; / REC rsp invalid,
    pub /: *mut *mut u32 rec_rsp_xchg_comp; / REC rsp xchg complete,
    pub /: *mut *mut u32 rec_rsp_rd_si_ownd; / REC rsp read si owned,
    pub /: *mut *mut u32 seqr_io_abort; / target does not know cmd so abort,
    pub /: *mut *mut u32 seqr_io_retry; / SEQR failed so retry IO,
    pub /: *mut *mut u32 itn_cisc_upd_rsp; / ITN cisc updated on fcp_rsp,
    pub /: *mut *mut u32 itn_cisc_upd_data; / ITN cisc updated on fcp_data,
    pub /: *mut *mut u32 itn_cisc_upd_xfer_rdy; / ITN cisc updated on fcp_data,
    pub /: *mut *mut u32 fcp_data_lost; / fcp data lost,
    pub /: *mut *mut u32 ro_set_in_xfer_rdy; / Target set RO in Xfer_rdy frame,
    pub /: *mut *mut u32 xfer_rdy_ooo_err; / Out of order Xfer_rdy received,
    pub /: *mut *mut u32 xfer_rdy_unknown_err; / unknown error in xfer_rdy frame,
    pub /: *mut *mut u32 io_abort_timeout; / ABTS timedout,
    pub /: *mut *mut u32 sler_initiated; / SLER initiated,
    pub /: *mut *mut u32 unexp_fcp_rsp; / fcp response in wrong state,
    pub /: *mut *mut u32 fcp_rsp_under_run; / fcp rsp IO underrun,
    pub /: *mut *mut u32 fcp_rsp_under_run_wr; / fcp rsp IO underrun for write,
    pub /: *mut *mut u32 fcp_rsp_under_run_err; / fcp rsp IO underrun error,
    pub /: *mut *mut u32 fcp_rsp_resid_inval; / invalid residue,
    pub /: *mut *mut u32 fcp_rsp_over_run; / fcp rsp IO overrun,
    pub /: *mut *mut u32 fcp_rsp_over_run_err; / fcp rsp IO overrun error,
    pub /: *mut *mut u32 fcp_rsp_proto_err; / protocol error in fcp rsp,
    pub /: *mut *mut u32 fcp_rsp_sense_err; / error in sense info in fcp rsp,
    pub /: *mut *mut u32 fcp_conf_req; / FCP conf requested,
    pub /: *mut *mut u32 tgt_aborted_io; / target initiated abort,
    pub /: *mut *mut u32 ioh_edtov_timeout_event;/ IOH edtov timer popped,
    pub /: *mut *mut u32 ioh_fcp_rsp_excp_event; / IOH FCP_RSP exception,
    pub /: *mut *mut u32 ioh_fcp_conf_event; / IOH FCP_CONF,
    pub /: *mut *mut u32 ioh_mult_frm_rsp_event; / IOH multi_frame FCP_RSP,
    pub /: *mut *mut u32 ioh_hit_class2_event; / IOH hit class2,
    pub /: *mut *mut u32 ioh_miss_other_event; / IOH miss other,
    pub /: *mut *mut u32 ioh_seq_cnt_err_event; / IOH seq cnt error,
    pub !=: *mut *mut u32 ioh_len_err_event; / IOH len error - fcp_dl,
// bytes xfered
    pub /: *mut *mut u32 ioh_seq_len_err_event; / IOH seq len error,
    pub /: *mut *mut u32 ioh_data_oor_event; / Data out of range,
    pub /: *mut *mut u32 ioh_ro_ooo_event; / Relative offset out of range,
    pub /: *mut *mut u32 ioh_cpu_owned_event; / IOH hit -iost owned by f/w,
    pub received: *mut *mut u32 ioh_unexp_frame_event; / unexpected frame,
// count
    pub data-phase: *mut *mut u32 ioh_err_int; / IOH error int during,
// for scsi write
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_tio_stats_s {
    pub /: *mut *mut u32 tio_conf_proc; / TIO CONF processed,
    pub /: *mut *mut u32 tio_conf_drop; / TIO CONF dropped,
    pub /: *mut *mut u32 tio_cleanup_req; / TIO cleanup requested,
    pub /: *mut *mut u32 tio_cleanup_comp; / TIO cleanup completed,
    pub /: *mut *mut u32 tio_abort_rsp; / TIO abort response,
    pub /: *mut *mut u32 tio_abort_rsp_comp; / TIO abort rsp completed,
    pub /: *mut *mut u32 tio_abts_req; / TIO ABTS requested,
    pub /: *mut *mut u32 tio_abts_ack; / TIO ABTS ack-ed,
    pub /: *mut *mut u32 tio_abts_ack_nocomp;/ TIO ABTS ack-ed but not completed,
    pub /: *mut *mut u32 tio_abts_tmo; / TIO ABTS timeout,
    pub /: *mut *mut u32 tio_snsdata_dma; / TIO sense data DMA,
    pub /: *mut *mut u32 tio_rxwchan_wait; / TIO waiting for RX wait channel,
    pub /: *mut *mut u32 tio_rxwchan_avail; / TIO RX wait channel available,
    pub /: *mut *mut u32 tio_hit_bls; / TIO IOH BLS event,
    pub /: *mut *mut u32 tio_uf_recv; / TIO received UF,
    pub /: *mut *mut u32 tio_rd_invalid_sm; / TIO read reqst in wrong state machine,
    pub /: *mut *mut u32 tio_wr_invalid_sm; / TIO write reqst in wrong state machine,
    pub /: *mut *mut u32 ds_rxwchan_wait; / DS waiting for RX wait channel,
    pub /: *mut *mut u32 ds_rxwchan_avail; / DS RX wait channel available,
    pub /: *mut *mut u32 ds_unaligned_rd; / DS unaligned read,
    pub state: *mut *mut u32 ds_rdcomp_invalid_sm; / DS read completed in wrong,
// machine
    pub state: *mut *mut u32 ds_wrcomp_invalid_sm; / DS write completed in wrong,
// machine
    pub /: *mut *mut u32 ds_flush_req; / DS flush requested,
    pub /: *mut *mut u32 ds_flush_comp; / DS flush completed,
    pub /: *mut *mut u32 ds_xfrdy_exp; / DS XFER_RDY expired,
    pub /: *mut *mut u32 ds_seq_cnt_err; / DS seq cnt error,
    pub /: *mut *mut u32 ds_seq_len_err; / DS seq len error,
    pub /: *mut *mut u32 ds_data_oor; / DS data out of order,
    pub /: *mut *mut u32 ds_hit_bls; / DS hit BLS,
    pub /: *mut *mut u32 ds_edtov_timer_exp; / DS edtov expired,
    pub /: *mut *mut u32 ds_cpu_owned; / DS cpu owned,
    pub /: *mut *mut u32 ds_hit_class2; / DS hit class2,
    pub /: *mut *mut u32 ds_length_err; / DS length error,
    pub /: *mut *mut u32 ds_ro_ooo_err; / DS relative offset out-of-order error,
    pub /: *mut *mut u32 ds_rectov_timer_exp;/ DS rectov expired,
    pub /: *mut *mut u32 ds_unexp_fr_err; / DS unexp frame error,
}

//
// IOC firmware IO stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_io_stats_s {
    pub ioim_stats: bfa_fw_ioim_stats_s,
    pub tio_stats: bfa_fw_tio_stats_s,
}

//
// IOC port firmware stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_port_fpg_stats_s {
    pub intr_evt: u32,
    pub intr: u32,
    pub intr_excess: u32,
    pub intr_cause0: u32,
    pub intr_other: u32,
    pub intr_other_ign: u32,
    pub sig_lost: u32,
    pub sig_regained: u32,
    pub sync_lost: u32,
    pub sync_to: u32,
    pub sync_regained: u32,
    pub div2_overflow: u32,
    pub div2_underflow: u32,
    pub efifo_overflow: u32,
    pub efifo_underflow: u32,
    pub idle_rx: u32,
    pub lrr_rx: u32,
    pub lr_rx: u32,
    pub ols_rx: u32,
    pub nos_rx: u32,
    pub lip_rx: u32,
    pub arbf0_rx: u32,
    pub arb_rx: u32,
    pub mrk_rx: u32,
    pub const_mrk_rx: u32,
    pub prim_unknown: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_port_lksm_stats_s {
    pub /: *mut *mut u32 hwsm_success; / hwsm state machine success,
    pub /: *mut *mut u32 hwsm_fails; / hwsm fails,
    pub /: *mut *mut u32 hwsm_wdtov; / hwsm timed out,
    pub /: *mut *mut u32 swsm_success; / swsm success,
    pub /: *mut *mut u32 swsm_fails; / swsm fails,
    pub /: *mut *mut u32 swsm_wdtov; / swsm timed out,
    pub /: *mut *mut u32 busybufs; / link init failed due to busybuf,
    pub /: *mut *mut u32 buf_waits; / bufwait state entries,
    pub /: *mut *mut u32 link_fails; / link failures,
    pub /: *mut *mut u32 psp_errors; / primitive sequence protocol errors,
    pub /: *mut *mut u32 lr_unexp; / No. of times LR rx-ed unexpectedly,
    pub /: *mut *mut u32 lrr_unexp; / No. of times LRR rx-ed unexpectedly,
    pub /: *mut *mut u32 lr_tx; / No. of times LR tx started,
    pub /: *mut *mut u32 lrr_tx; / No. of times LRR tx started,
    pub /: *mut *mut u32 ols_tx; / No. of times OLS tx started,
    pub /: *mut *mut u32 nos_tx; / No. of times NOS tx started,
    pub /: *mut *mut u32 hwsm_lrr_rx; / No. of times LRR rx-ed by HWSM,
    pub /: *mut *mut u32 hwsm_lr_rx; / No. of times LR rx-ed by HWSM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_port_snsm_stats_s {
    pub /: *mut *mut u32 hwsm_success; / Successful hwsm terminations,
    pub /: *mut *mut u32 hwsm_fails; / hwsm fail count,
    pub /: *mut *mut u32 hwsm_wdtov; / hwsm timed out,
    pub /: *mut *mut u32 swsm_success; / swsm success,
    pub /: *mut *mut u32 swsm_wdtov; / swsm timed out,
    pub /: *mut *mut u32 error_resets; / error resets initiated by upsm,
    pub /: *mut *mut u32 sync_lost; / Sync loss count,
    pub /: *mut *mut u32 sig_lost; / Signal loss count,
    pub /: *mut *mut u32 asn8g_attempts; / SNSM HWSM at 8Gbps attempts,
    pub /: *mut *mut u32 adapt_success; / SNSM adaptation success,
    pub /: *mut *mut u32 adapt_fails; / SNSM adaptation failures,
    pub /: *mut *mut u32 adapt_ign_fails; / SNSM adaptation failures ignored,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_port_physm_stats_s {
    pub /: *mut *mut u32 module_inserts; / Module insert count,
    pub /: *mut *mut u32 module_xtracts; / Module extracts count,
    pub /: *mut *mut u32 module_invalids; / Invalid module inserted count,
    pub /: *mut *mut u32 module_read_ign; / Module validation status ignored,
    pub /: *mut *mut u32 laser_faults; / Laser fault count,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_fip_stats_s {
    pub /: *mut *mut u32 vlan_req; / vlan discovery requests,
    pub /: *mut *mut u32 vlan_notify; / vlan notifications,
    pub /: *mut *mut u32 vlan_err; / vlan response error,
    pub /: *mut *mut u32 vlan_timeouts; / vlan disvoery timeouts,
    pub /: *mut *mut u32 vlan_invalids; / invalid vlan in discovery advert.,
    pub /: *mut *mut u32 disc_req; / Discovery solicit requests,
    pub /: *mut *mut u32 disc_rsp; / Discovery solicit response,
    pub /: *mut *mut u32 disc_err; / Discovery advt. parse errors,
    pub /: *mut *mut u32 disc_unsol; / Discovery unsolicited,
    pub /: *mut *mut u32 disc_timeouts; / Discovery timeouts,
    pub /: *mut *mut u32 disc_fcf_unavail; / Discovery FCF Not Avail.,
    pub /: *mut *mut u32 linksvc_unsupp; / Unsupported link service req,
    pub /: *mut *mut u32 linksvc_err; / Parse error in link service req,
    pub /: *mut *mut u32 logo_req; / FIP logos received,
    pub /: *mut *mut u32 clrvlink_req; / Clear virtual link req,
    pub /: *mut *mut u32 op_unsupp; / Unsupported FIP operation,
    pub /: *mut *mut u32 untagged; / Untagged frames (ignored),
    pub /: *mut *mut u32 invalid_version; / Invalid FIP version,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_lps_stats_s {
    pub /: *mut *mut u32 mac_invalids; / Invalid mac assigned,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_fcoe_stats_s {
    pub /: *mut *mut u32 cee_linkups; / CEE link up count,
    pub /: *mut *mut u32 cee_linkdns; / CEE link down count,
    pub /: *mut *mut u32 fip_linkups; / FIP link up count,
    pub /: *mut *mut u32 fip_linkdns; / FIP link up count,
    pub /: *mut *mut u32 fip_fails; / FIP fail count,
    pub /: *mut *mut u32 mac_invalids; / Invalid mac assigned,
}

//
// IOC firmware FCoE port stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_fcoe_port_stats_s {
    pub fcoe_stats: bfa_fw_fcoe_stats_s,
    pub fip_stats: bfa_fw_fip_stats_s,
}

//
// @brief LPSM statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_lpsm_stats_s {
    pub /: *mut *mut u32 cls_rx; / LPSM cls_rx,
    pub /: *mut *mut u32 cls_tx; / LPSM cls_tx,
    pub /: *mut *mut u32 arbf0_rx; / LPSM abrf0 rcvd,
    pub /: *mut *mut u32 arbf0_tx; / LPSM abrf0 xmit,
    pub /: *mut *mut u32 init_rx; / LPSM loop init start,
    pub /: *mut *mut u32 unexp_hwst; / LPSM unknown hw state,
    pub /: *mut *mut u32 unexp_frame; / LPSM unknown_frame,
    pub /: *mut *mut u32 unexp_prim; / LPSM unexpected primitive,
    pub /: *mut *mut u32 prev_alpa_unavail; / LPSM prev alpa unavailable,
    pub /: *mut *mut u32 alpa_unavail; / LPSM alpa not available,
    pub /: *mut *mut u32 lip_rx; / LPSM lip rcvd,
    pub /: *mut *mut u32 lip_f7f7_rx; / LPSM lip f7f7 rcvd,
    pub /: *mut *mut u32 lip_f8_rx; / LPSM lip f8 rcvd,
    pub /: *mut *mut u32 lip_f8f7_rx; / LPSM lip f8f7 rcvd,
    pub /: *mut *mut u32 lip_other_rx; / LPSM lip other rcvd,
    pub /: *mut *mut u32 lip_tx; / LPSM lip xmit,
    pub /: *mut *mut u32 retry_tov; / LPSM retry TOV,
    pub /: *mut *mut u32 lip_tov; / LPSM LIP wait TOV,
    pub /: *mut *mut u32 idle_tov; / LPSM idle wait TOV,
    pub /: *mut *mut u32 arbf0_tov; / LPSM arbfo wait TOV,
    pub /: *mut *mut u32 stop_loop_tov; / LPSM stop loop wait TOV,
    pub /: *mut *mut u32 lixa_tov; / LPSM lisa wait TOV,
    pub /: *mut *mut u32 lixx_tov; / LPSM lilp/lirp wait TOV,
    pub /: *mut *mut u32 cls_tov; / LPSM cls wait TOV,
    pub /: *mut *mut u32 sler; / LPSM SLER recvd,
    pub /: *mut *mut u32 failed; / LPSM failed,
    pub /: *mut *mut u32 success; / LPSM online,
}

//
// IOC firmware FC uport stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_fc_uport_stats_s {
    pub snsm_stats: bfa_fw_port_snsm_stats_s,
    pub lksm_stats: bfa_fw_port_lksm_stats_s,
    pub lpsm_stats: bfa_fw_lpsm_stats_s,
}

//
// IOC firmware FC port stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfa_fw_fc_port_stats_s {
    pub fc_stats: bfa_fw_fc_uport_stats_s,
    pub fcoe_stats: bfa_fw_fcoe_port_stats_s,
}

//
// IOC firmware port stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_port_stats_s {
    pub fpg_stats: bfa_fw_port_fpg_stats_s,
    pub physm_stats: bfa_fw_port_physm_stats_s,
    pub fc_port: bfa_fw_fc_port_stats_s,
}

//
// fcxchg module statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_fcxchg_stats_s {
    pub ua_tag_inv: u32,
    pub ua_state_inv: u32,
}

//
// Trunk statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_trunk_stats_s {
    pub /: *mut *mut u32 emt_recvd; / Trunk EMT received,
    pub /: *mut *mut u32 emt_accepted; / Trunk EMT Accepted,
    pub /: *mut *mut u32 emt_rejected; / Trunk EMT rejected,
    pub /: *mut *mut u32 etp_recvd; / Trunk ETP received,
    pub /: *mut *mut u32 etp_accepted; / Trunk ETP Accepted,
    pub /: *mut *mut u32 etp_rejected; / Trunk ETP rejected,
    pub /: *mut *mut u32 lr_recvd; / Trunk LR received,
    pub /: *mut *mut u32 rsvd; / padding for 64 bit alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_aport_stats_s {
    pub /: *mut *mut u32 flogi_sent; / Flogi sent,
    pub /: *mut *mut u32 flogi_acc_recvd; / Flogi Acc received,
    pub /: *mut *mut u32 flogi_rjt_recvd; / Flogi rejects received,
    pub /: *mut *mut u32 flogi_retries; / Flogi retries,
    pub /: *mut *mut u32 elp_recvd; / ELP received,
    pub /: *mut *mut u32 elp_accepted; / ELP Accepted,
    pub /: *mut *mut u32 elp_rejected; / ELP rejected,
    pub /: *mut *mut u32 elp_dropped; / ELP dropped,
    pub /: *mut *mut u32 bbcr_lr_count; /!< BBCR Link Resets,
    pub /: *mut *mut u32 frame_lost_intrs; /!< BBCR Frame loss intrs,
    pub /: *mut *mut u32 rrdy_lost_intrs; /!< BBCR Rrdy loss intrs,
    pub rsvd: u32,
}

//
// IOCFC firmware stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_iocfc_stats_s {
    pub /: *mut *mut u32 cfg_reqs; / cfg request,
    pub /: *mut *mut u32 updq_reqs; / update queue request,
    pub /: *mut *mut u32 ic_reqs; / interrupt coalesce reqs,
    pub unknown_reqs: u32,
    pub /: *mut *mut u32 set_intr_reqs; / set interrupt reqs,
}

//
// IOC attributes returned in queries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_iocfc_attr_s {
    pub /: *mut *mut bfa_iocfc_cfg_s config; / IOCFC config,
    pub /: *mut *mut bfa_iocfc_intr_attr_s intr_attr; / interrupt attr,
}

//
// Eth_sndrcv mod stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_eth_sndrcv_stats_s {
    pub crc_err: u32,
    pub /: *mut *mut u32 rsvd; / 64bit align,
}

//
// CT MAC mod stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_mac_mod_stats_s {
    pub /: *mut *mut u32 mac_on; / MAC got turned-on,
    pub /: *mut *mut u32 link_up; / link-up,
    pub /: *mut *mut u32 signal_off; / lost signal,
    pub /: *mut *mut u32 dfe_on; / DFE on,
    pub /: *mut *mut u32 mac_reset; / # of MAC reset to bring lnk up,
    pub /: *mut *mut u32 pcs_reset; / # of PCS reset to bring lnk up,
    pub /: *mut *mut u32 loopback; / MAC got into serdes loopback,
    pub lb_mac_reset: u32,
// # of MAC reset to bring link up in loopback
    pub lb_pcs_reset: u32,
// # of PCS reset to bring link up in loopback
    pub /: *mut *mut u32 rsvd; / 64bit align,
}

//
// CT MOD stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_ct_mod_stats_s {
    pub /: *mut *mut u32 rxa_rds_undrun; / RxA RDS underrun,
    pub /: *mut *mut u32 rad_bpc_ovfl; / RAD BPC overflow,
    pub /: *mut *mut u32 rad_rlb_bpc_ovfl; / RAD RLB BPC overflow,
    pub /: *mut *mut u32 bpc_fcs_err; / BPC FCS_ERR,
    pub /: *mut *mut u32 txa_tso_hdr; / TxA TSO header too long,
    pub /: *mut *mut u32 rsvd; / 64bit align,
}

//
// RDS mod stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_rds_stats_s {
    pub /: *mut *mut u32 no_fid_drop_err; / RDS no fid drop error,
    pub /: *mut *mut u32 rsvd; / 64bit align,
}

//
// IOC firmware stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fw_stats_s {
    pub ioc_stats: bfa_fw_ioc_stats_s,
    pub iocfc_stats: bfa_fw_iocfc_stats_s,
    pub io_stats: bfa_fw_io_stats_s,
    pub port_stats: bfa_fw_port_stats_s,
    pub fcxchg_stats: bfa_fw_fcxchg_stats_s,
    pub lps_stats: bfa_fw_lps_stats_s,
    pub trunk_stats: bfa_fw_trunk_stats_s,
    pub aport_stats: bfa_fw_aport_stats_s,
    pub macmod_stats: bfa_fw_mac_mod_stats_s,
    pub ctmod_stats: bfa_fw_ct_mod_stats_s,
    pub ethsndrcv_stats: bfa_fw_eth_sndrcv_stats_s,
    pub rds_stats: bfa_fw_rds_stats_s,
}

pub const BFA_IOCFC_PATHTOV_MAX: c_int = 60;
pub const BFA_IOCFC_QDEPTH_MAX: c_int = 2000;
//
// QoS states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_qos_state {
    BFA_QOS_DISABLED = 0,		/* QoS is disabled */
    BFA_QOS_ONLINE = 1,		/*  QoS is online */
    BFA_QOS_OFFLINE = 2,		/*  QoS is offline */
}

//
// QoS  Priority levels.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_qos_priority {
    BFA_QOS_UNKNOWN = 0,
    BFA_QOS_HIGH  = 1,	/*  QoS Priority Level High */
    BFA_QOS_MED  =  2,	/*  QoS Priority Level Medium */
    BFA_QOS_LOW  =  3,	/*  QoS Priority Level Low */
}

//
// QoS  bandwidth allocation for each priority level
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_qos_bw_alloc {
    BFA_QOS_BW_HIGH  = 60,	/*  bandwidth allocation for High */
    BFA_QOS_BW_MED  =  30,	/*  bandwidth allocation for Medium */
    BFA_QOS_BW_LOW  =  10,	/*  bandwidth allocation for Low */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_qos_bw_s {
    pub qos_bw_set: u8,
    pub high: u8,
    pub med: u8,
    pub low: u8,
}

//
// QoS attribute returned in QoS Query
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_qos_attr_s {
    pub /: *mut *mut u8 state; / QoS current state,
    pub rsvd1: [u8; 3],
    pub /: *mut *mut u32 total_bb_cr; / Total BB Credits,
    pub /: *mut *mut bfa_qos_bw_s qos_bw; / QOS bw cfg,
    pub /: *mut *mut bfa_qos_bw_s qos_bw_op; / QOS bw operational,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_bbcr_state {
    BFA_BBCR_DISABLED,	/*!< BBCR is disable */
    BFA_BBCR_ONLINE,	/*!< BBCR is online  */
    BFA_BBCR_OFFLINE,	/*!< BBCR is offline */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_bbcr_err_reason {
    BFA_BBCR_ERR_REASON_NONE, /*!< Unknown */
    BFA_BBCR_ERR_REASON_SPEED_UNSUP, /*!< Port speed < max sup_speed */
    BFA_BBCR_ERR_REASON_PEER_UNSUP,	/*!< BBCR is disable on peer port */
    BFA_BBCR_ERR_REASON_NON_BRCD_SW, /*!< Connected to non BRCD switch */
    BFA_BBCR_ERR_REASON_FLOGI_RJT, /*!< Login rejected by the switch */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_bbcr_attr_s {
    pub state: u8,
    pub peer_bb_scn: u8,
    pub reason: u8,
    pub rsvd: u8,
}

//
// These fields should be displayed only from the CLI.
// There will be a separate BFAL API (get_qos_vc_attr ?)
// to retrieve this.
//
pub const BFA_QOS_MAX_VC: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_qos_vc_info_s {
    pub vc_credit: u8,
    pub borrow_credit: u8,
    pub priority: u8,
    pub resvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_qos_vc_attr_s {
    pub /: *mut *mut u16 total_vc_count; / Total VC Count,
    pub shared_credit: u16,
    pub elp_opmode_flags: u32,
    pub as: *mut *mut bfa_qos_vc_info_s vc_info[BFA_QOS_MAX_VC]; / as many,
// total_vc_count
}

//
// QoS statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_qos_stats_s {
    pub /: *mut *mut u32 flogi_sent; / QoS Flogi sent,
    pub /: *mut *mut u32 flogi_acc_recvd; / QoS Flogi Acc received,
    pub /: *mut *mut u32 flogi_rjt_recvd; / QoS Flogi rejects received,
    pub /: *mut *mut u32 flogi_retries; / QoS Flogi retries,
    pub /: *mut *mut u32 elp_recvd; / QoS ELP received,
    pub /: *mut *mut u32 elp_accepted; / QoS ELP Accepted,
    pub /: *mut *mut u32 elp_rejected; / QoS ELP rejected,
    pub /: *mut *mut u32 elp_dropped; / QoS ELP dropped,
    pub /: *mut *mut u32 qos_rscn_recvd; / QoS RSCN received,
    pub /: *mut *mut u32 rsvd; / padding for 64 bit alignment,
}

//
// FCoE statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcoe_stats_s {
    pub /: *mut *mut u64 secs_reset; / Seconds since stats reset,
    pub /: *mut *mut u64 cee_linkups; / CEE link up,
    pub /: *mut *mut u64 cee_linkdns; / CEE link down,
    pub /: *mut *mut u64 fip_linkups; / FIP link up,
    pub /: *mut *mut u64 fip_linkdns; / FIP link down,
    pub /: *mut *mut u64 fip_fails; / FIP failures,
    pub /: *mut *mut u64 mac_invalids; / Invalid mac assignments,
    pub /: *mut *mut u64 vlan_req; / Vlan requests,
    pub /: *mut *mut u64 vlan_notify; / Vlan notifications,
    pub /: *mut *mut u64 vlan_err; / Vlan notification errors,
    pub /: *mut *mut u64 vlan_timeouts; / Vlan request timeouts,
    pub /: *mut *mut u64 vlan_invalids; / Vlan invalids,
    pub /: *mut *mut u64 disc_req; / Discovery requests,
    pub /: *mut *mut u64 disc_rsp; / Discovery responses,
    pub /: *mut *mut u64 disc_err; / Discovery error frames,
    pub /: *mut *mut u64 disc_unsol; / Discovery unsolicited,
    pub /: *mut *mut u64 disc_timeouts; / Discovery timeouts,
    pub /: *mut *mut u64 disc_fcf_unavail; / Discovery FCF not avail,
    pub /: *mut *mut u64 linksvc_unsupp; / FIP link service req unsupp,
    pub /: *mut *mut u64 linksvc_err; / FIP link service req errors,
    pub /: *mut *mut u64 logo_req; / FIP logos received,
    pub /: *mut *mut u64 clrvlink_req; / Clear virtual link requests,
    pub /: *mut *mut u64 op_unsupp; / FIP operation unsupp.,
    pub /: *mut *mut u64 untagged; / FIP untagged frames,
    pub /: *mut *mut u64 txf_ucast; / Tx FCoE unicast frames,
    pub /: *mut *mut u64 txf_ucast_vlan; / Tx FCoE unicast vlan frames,
    pub /: *mut *mut u64 txf_ucast_octets; / Tx FCoE unicast octets,
    pub /: *mut *mut u64 txf_mcast; / Tx FCoE multicast frames,
    pub /: *mut *mut u64 txf_mcast_vlan; / Tx FCoE multicast vlan frames,
    pub /: *mut *mut u64 txf_mcast_octets; / Tx FCoE multicast octets,
    pub /: *mut *mut u64 txf_bcast; / Tx FCoE broadcast frames,
    pub /: *mut *mut u64 txf_bcast_vlan; / Tx FCoE broadcast vlan frames,
    pub /: *mut *mut u64 txf_bcast_octets; / Tx FCoE broadcast octets,
    pub /: *mut *mut u64 txf_timeout; / Tx timeouts,
    pub /: *mut *mut u64 txf_parity_errors; / Transmit parity err,
    pub /: *mut *mut u64 txf_fid_parity_errors; / Transmit FID parity err,
    pub /: *mut *mut u64 rxf_ucast_octets; / Rx FCoE unicast octets,
    pub /: *mut *mut u64 rxf_ucast; / Rx FCoE unicast frames,
    pub /: *mut *mut u64 rxf_ucast_vlan; / Rx FCoE unicast vlan frames,
    pub /: *mut *mut u64 rxf_mcast_octets; / Rx FCoE multicast octets,
    pub /: *mut *mut u64 rxf_mcast; / Rx FCoE multicast frames,
    pub /: *mut *mut u64 rxf_mcast_vlan; / Rx FCoE multicast vlan frames,
    pub /: *mut *mut u64 rxf_bcast_octets; / Rx FCoE broadcast octets,
    pub /: *mut *mut u64 rxf_bcast; / Rx FCoE broadcast frames,
    pub /: *mut *mut u64 rxf_bcast_vlan; / Rx FCoE broadcast vlan frames,
}

//
// QoS or FCoE stats (fcport stats excluding physical FC port stats)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfa_fcport_stats_u {
    pub fcqos: bfa_qos_stats_s,
    pub fcoe: bfa_fcoe_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcpim_del_itn_stats_s {
    pub /: *mut *mut u32 del_itn_iocomp_aborted; / Aborted IO requests,
    pub /: *mut *mut u32 del_itn_iocomp_timedout; / IO timeouts,
    pub /: *mut *mut u32 del_itn_iocom_sqer_needed; / IO retry for SQ error recovery,
    pub /: *mut *mut u32 del_itn_iocom_res_free; / Delayed freeing of IO resources,
    pub /: *mut *mut u32 del_itn_iocom_hostabrts; / Host IO abort requests,
    pub /: *mut *mut u32 del_itn_total_ios; / Total IO count,
    pub /: *mut *mut u32 del_io_iocdowns; / IO cleaned-up due to IOC down,
    pub /: *mut *mut u32 del_tm_iocdowns; / TM cleaned-up due to IOC down,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itnim_iostats_s {
    pub /: *mut *mut u32 total_ios; / Total IO Requests,
    pub /: *mut *mut u32 input_reqs; / Data in-bound requests,
    pub /: *mut *mut u32 output_reqs; / Data out-bound requests,
    pub /: *mut *mut u32 io_comps; / Total IO Completions,
    pub /: *mut *mut u32 wr_throughput; / Write data transferred in bytes,
    pub /: *mut *mut u32 rd_throughput; / Read data transferred in bytes,
    pub /: *mut *mut u32 iocomp_ok; / Slowpath IO completions,
    pub /: *mut *mut u32 iocomp_underrun; / IO underrun,
    pub /: *mut *mut u32 iocomp_overrun; / IO overrun,
    pub /: *mut *mut u32 qwait; / IO Request-Q wait,
    pub /: *mut *mut u32 qresumes; / IO Request-Q wait done,
    pub /: *mut *mut u32 no_iotags; / No free IO tag,
    pub /: *mut *mut u32 iocomp_timedout; / IO timeouts,
    pub /: *mut *mut u32 iocom_nexus_abort; / IO failure due to target offline,
    pub /: *mut *mut u32 iocom_proto_err; / IO protocol errors,
    pub /: *mut *mut u32 iocom_dif_err; / IO SBC-3 protection errors,
    pub /: *mut *mut u32 iocom_sqer_needed; / fcp-2 error recovery failed,
    pub /: *mut *mut u32 iocom_res_free; / Delayed freeing of IO tag,
    pub /: *mut *mut u32 io_aborts; / Host IO abort requests,
    pub /: *mut *mut u32 iocom_hostabrts; / Host IO abort completions,
    pub /: *mut *mut u32 io_cleanups; / IO clean-up requests,
    pub /: *mut *mut u32 path_tov_expired; / IO path tov expired,
    pub /: *mut *mut u32 iocomp_aborted; / IO abort completions,
    pub /: *mut *mut u32 io_iocdowns; / IO cleaned-up due to IOC down,
    pub /: *mut *mut u32 iocom_utags; / IO comp with unknown tags,
    pub /: *mut *mut u32 io_tmaborts; / Abort request due to TM command,
    pub /: *mut *mut u32 tm_io_comps; / Abort completion due to TM command,
    pub /: *mut *mut u32 creates; / IT Nexus create requests,
    pub /: *mut *mut u32 fw_create; / IT Nexus FW create requests,
    pub /: *mut *mut u32 create_comps; / IT Nexus FW create completions,
    pub /: *mut *mut u32 onlines; / IT Nexus onlines,
    pub /: *mut *mut u32 offlines; / IT Nexus offlines,
    pub /: *mut *mut u32 fw_delete; / IT Nexus FW delete requests,
    pub /: *mut *mut u32 delete_comps; / IT Nexus FW delete completions,
    pub /: *mut *mut u32 deletes; / IT Nexus delete requests,
    pub /: *mut *mut u32 sler_events; / SLER events,
    pub /: *mut *mut u32 ioc_disabled; / Num IOC disables,
    pub /: *mut *mut u32 cleanup_comps; / IT Nexus cleanup completions,
    pub /: *mut *mut u32 tm_cmnds; / TM Requests,
    pub /: *mut *mut u32 tm_fw_rsps; / TM Completions,
    pub /: *mut *mut u32 tm_success; / TM initiated IO cleanup success,
    pub /: *mut *mut u32 tm_failures; / TM initiated IO cleanup failure,
    pub /: *mut *mut u32 no_tskims; / No free TM tag,
    pub /: *mut *mut u32 tm_qwait; / TM Request-Q wait,
    pub /: *mut *mut u32 tm_qresumes; / TM Request-Q wait done,
    pub /: *mut *mut u32 tm_iocdowns; / TM cleaned-up due to IOC down,
    pub /: *mut *mut u32 tm_cleanups; / TM cleanup requests,
    pub /: *mut *mut u32 tm_cleanup_comps; / TM cleanup completions,
    pub rsvd: [u32; 6],
}

// Modify char* port_stt[] in bfal_port.c if a new state was added
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_states {
    BFA_PORT_ST_UNINIT		= 1,
    BFA_PORT_ST_ENABLING_QWAIT	= 2,
    BFA_PORT_ST_ENABLING		= 3,
    BFA_PORT_ST_LINKDOWN		= 4,
    BFA_PORT_ST_LINKUP		= 5,
    BFA_PORT_ST_DISABLING_QWAIT	= 6,
    BFA_PORT_ST_DISABLING		= 7,
    BFA_PORT_ST_DISABLED		= 8,
    BFA_PORT_ST_STOPPED		= 9,
    BFA_PORT_ST_IOCDOWN		= 10,
    BFA_PORT_ST_IOCDIS		= 11,
    BFA_PORT_ST_FWMISMATCH		= 12,
    BFA_PORT_ST_PREBOOT_DISABLED	= 13,
    BFA_PORT_ST_TOGGLING_QWAIT	= 14,
    BFA_PORT_ST_FAA_MISCONFIG	= 15,
    BFA_PORT_ST_DPORT		= 16,
    BFA_PORT_ST_DDPORT		= 17,
    BFA_PORT_ST_MAX_STATE,
}

//
// Port operational type (in sync with SNIA port type).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_type {
    BFA_PORT_TYPE_UNKNOWN	= 1,	/*  port type is unknown */
    BFA_PORT_TYPE_NPORT	= 5,	/*  P2P with switched fabric */
    BFA_PORT_TYPE_NLPORT	= 6,	/*  public loop */
    BFA_PORT_TYPE_LPORT	= 20,	/*  private loop */
    BFA_PORT_TYPE_P2P	= 21,	/*  P2P with no switched fabric */
    BFA_PORT_TYPE_VPORT	= 22,	/*  NPIV - virtual port */
}

//
// Port topology setting. A port's topology and fabric login status
// determine its operational type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_topology {
    BFA_PORT_TOPOLOGY_NONE = 0,	/*  No valid topology */
    BFA_PORT_TOPOLOGY_P2P_OLD_VER = 1, /* P2P def for older ver */
    BFA_PORT_TOPOLOGY_LOOP = 2,	/* LOOP topology */
    BFA_PORT_TOPOLOGY_AUTO_OLD_VER = 3, /* auto def for older ver */
    BFA_PORT_TOPOLOGY_AUTO = 4,	/* auto topology selection */
    BFA_PORT_TOPOLOGY_P2P = 5,	/* P2P only */
}

//
// Physical port loopback types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_opmode {
    BFA_PORT_OPMODE_NORMAL   = 0x00, /*  normal non-loopback mode */
    BFA_PORT_OPMODE_LB_INT   = 0x01, /*  internal loop back */
    BFA_PORT_OPMODE_LB_SLW   = 0x02, /*  serial link wrapback (serdes) */
    BFA_PORT_OPMODE_LB_EXT   = 0x04, /*  external loop back (serdes) */
    BFA_PORT_OPMODE_LB_CBL   = 0x08, /*  cabled loop back */
    BFA_PORT_OPMODE_LB_NLINT = 0x20, /*  NL_Port internal loopback */
}

//
// Port link state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_linkstate {
    BFA_PORT_LINKUP		= 1,	/*  Physical port/Trunk link up */
    BFA_PORT_LINKDOWN	= 2,	/*  Physical port/Trunk link down */
}

//
// Port link state reason code
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_port_linkstate_rsn {
    BFA_PORT_LINKSTATE_RSN_NONE		= 0,
    BFA_PORT_LINKSTATE_RSN_DISABLED		= 1,
    BFA_PORT_LINKSTATE_RSN_RX_NOS		= 2,
    BFA_PORT_LINKSTATE_RSN_RX_OLS		= 3,
    BFA_PORT_LINKSTATE_RSN_RX_LIP		= 4,
    BFA_PORT_LINKSTATE_RSN_RX_LIPF7		= 5,
    BFA_PORT_LINKSTATE_RSN_SFP_REMOVED	= 6,
    BFA_PORT_LINKSTATE_RSN_PORT_FAULT	= 7,
    BFA_PORT_LINKSTATE_RSN_RX_LOS		= 8,
    BFA_PORT_LINKSTATE_RSN_LOCAL_FAULT	= 9,
    BFA_PORT_LINKSTATE_RSN_REMOTE_FAULT	= 10,
    BFA_PORT_LINKSTATE_RSN_TIMEOUT		= 11,
    BFA_PORT_LINKSTATE_RSN_FAA_MISCONFIG	= 12,



// CEE related reason codes/errors
    CEE_LLDP_INFO_AGED_OUT			= 20,
    CEE_LLDP_SHUTDOWN_TLV_RCVD		= 21,
    CEE_PEER_NOT_ADVERTISE_DCBX		= 22,
    CEE_PEER_NOT_ADVERTISE_PG		= 23,
    CEE_PEER_NOT_ADVERTISE_PFC		= 24,
    CEE_PEER_NOT_ADVERTISE_FCOE		= 25,
    CEE_PG_NOT_COMPATIBLE			= 26,
    CEE_PFC_NOT_COMPATIBLE			= 27,
    CEE_FCOE_NOT_COMPATIBLE			= 28,
    CEE_BAD_PG_RCVD				= 29,
    CEE_BAD_BW_RCVD				= 30,
    CEE_BAD_PFC_RCVD			= 31,
    CEE_BAD_APP_PRI_RCVD			= 32,
    CEE_FCOE_PRI_PFC_OFF			= 33,
    CEE_DUP_CONTROL_TLV_RCVD		= 34,
    CEE_DUP_FEAT_TLV_RCVD			= 35,
    CEE_APPLY_NEW_CFG			= 36, /* reason, not error */
    CEE_PROTOCOL_INIT			= 37, /* reason, not error */
    CEE_PHY_LINK_DOWN			= 38,
    CEE_LLS_FCOE_ABSENT			= 39,
    CEE_LLS_FCOE_DOWN			= 40,
    CEE_ISCSI_NOT_COMPATIBLE		= 41,
    CEE_ISCSI_PRI_PFC_OFF			= 42,
    CEE_ISCSI_PRI_OVERLAP_FCOE_PRI		= 43
}

pub const MAX_LUN_MASK_CFG: c_int = 16;
//
// Initially flash content may be fff. On making LUN mask enable and disable
// state change.  when report lun command is being processed it goes from
// BFA_LUN_MASK_ACTIVE to BFA_LUN_MASK_FETCH and comes back to
// BFA_LUN_MASK_ACTIVE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_ioim_lun_mask_state_s {
    BFA_IOIM_LUN_MASK_INACTIVE = 0,
    BFA_IOIM_LUN_MASK_ACTIVE = 1,
    BFA_IOIM_LUN_MASK_FETCHED = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_lunmask_state_s {
    BFA_LUNMASK_DISABLED = 0x00,
    BFA_LUNMASK_ENABLED = 0x01,
    BFA_LUNMASK_MINCFG = 0x02,
    BFA_LUNMASK_UNINITIALIZED = 0xff,
}

//
// FEC states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_fec_state_s {
    BFA_FEC_ONLINE = 1,		/*!< FEC is online */
    BFA_FEC_OFFLINE = 2,		/*!< FEC is offline */
    BFA_FEC_OFFLINE_NOT_16G = 3,	/*!< FEC is offline (speed not 16Gig) */
}

//
// LUN mask configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lun_mask_s {
    pub lp_wwn: wwn_t,
    pub rp_wwn: wwn_t,
    pub lun: scsi_lun,
    pub ua: u8,
    pub rsvd: [u8; 3],
    pub rp_tag: u16,
    pub lp_tag: u8,
    pub state: u8,
}

pub const MAX_LUN_MASK_CFG: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_lunmask_cfg_s {
    pub status: u32,
    pub rsvd: u32,
    pub lun_list: [bfa_lun_mask_s; MAX_LUN_MASK_CFG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_throttle_cfg_s {
    pub is_valid: u16,
    pub value: u16,
    pub rsvd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_defs_fcpim_throttle_s {
    pub max_value: u16,
    pub cur_value: u16,
    pub cfg_value: u16,
    pub rsvd: u16,
}

pub const BFA_BB_SCN_DEF: c_int = 3;
pub const BFA_BB_SCN_MAX: c_uint = 0x0F;
//
// Physical port configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_cfg_s {
    pub /: *mut *mut u8 topology; / bfa_port_topology,
    pub /: *mut *mut u8 speed; / enum bfa_port_speed,
    pub /: *mut *mut u8 trunked; / trunked or not,
    pub /: *mut *mut u8 qos_enabled; / qos enabled or not,
    pub /: *mut *mut u8 cfg_hardalpa; / is hard alpa configured,
    pub /: *mut *mut u8 hardalpa; / configured hard alpa,
    pub /: *mut *mut __be16 maxfrsize; / maximum frame size,
    pub /: *mut *mut u8 rx_bbcredit; / receive buffer credits,
    pub /: *mut *mut u8 tx_bbcredit; / transmit buffer credits,
    pub /: *mut *mut u8 ratelimit; / ratelimit enabled or not,
    pub /: *mut *mut u8 trl_def_speed; / ratelimit default speed,
    pub /: *mut *mut u8 bb_cr_enabled; /!< Config state of BB_SCN,
    pub /: *mut *mut u8 bb_scn; /!< BB_SCN value for FLOGI Exchg,
    pub /: *mut *mut u8 faa_state; / FAA enabled/disabled,
    pub rsvd1: u8,
    pub /: *mut *mut u16 path_tov; / device path timeout,
    pub /: *mut *mut u16 q_depth; / SCSI Queue depth,
    pub /: *mut *mut bfa_qos_bw_s qos_bw; / QOS bandwidth,
}

//
// Port attribute values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_attr_s {
//
// Static fields
//
    pub /: *mut *mut wwn_t nwwn; / node wwn,
    pub /: *mut *mut wwn_t pwwn; / port wwn,
    pub /: *mut *mut wwn_t factorynwwn; / factory node wwn,
    pub /: *mut *mut wwn_t factorypwwn; / factory port wwn,
    pub of: *mut *mut fc_cos cos_supported; / supported class,
// services
    pub rsvd: u32,
    pub /: *mut *mut fc_symname_s port_symname; / port symbolic name,
    pub /: *mut *mut bfa_port_speed speed_supported; / supported speeds,
    pub pbind_enabled: bfa_boolean_t,
//
// Configured values
//
    pub /: *mut *mut bfa_port_cfg_s pport_cfg; / pport cfg,
//
// Dynamic field - info from BFA
//
    pub /: *mut *mut bfa_port_states port_state; / current port state,
    pub /: *mut *mut bfa_port_speed speed; / current speed,
    pub /: *mut *mut bfa_port_topology topology; / current topology,
    pub /: *mut *mut bfa_boolean_t beacon; / current beacon status,
    pub /: *mut *mut bfa_boolean_t link_e2e_beacon; / link beacon is on,
    pub oper: *mut *mut bfa_boolean_t bbsc_op_status; / fc credit recovery,
// state
    pub /: *mut *mut bfa_fec_state_s fec_state; /!< current FEC state,
//
// Dynamic field - info from FCS
//
    pub /: *mut *mut u32 pid; / port ID,
    pub /: *mut *mut bfa_port_type port_type; / current topology,
    pub /: *mut *mut u32 loopback; / external loopback,
    pub /: *mut *mut u32 authfail; / auth fail state,
// FCoE specific
    pub fcoe_vlan: u16,
    pub rsvd1: [u8; 2],
}

//
// Port FCP mappings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_fcpmap_s {
    pub osdevname: [c_char; 256],
    pub bus: u32,
    pub target: u32,
    pub oslun: u32,
    pub fcid: u32,
    pub nwwn: wwn_t,
    pub pwwn: wwn_t,
    pub fcplun: u64,
    pub luid: [c_char; 256],
}

//
// Port RNID info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_rnid_s {
    pub wwn: wwn_t,
    pub unittype: u32,
    pub portid: u32,
    pub attached_nodes_num: u32,
    pub ip_version: u16,
    pub udp_port: u16,
    pub ipaddr: [u8; 16],
    pub rsvd: u16,
    pub topologydiscoveryflags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcport_fcf_s {
    pub /: *mut *mut wwn_t name; / FCF name,
    pub /: *mut *mut wwn_t fabric_name; / Fabric Name,
    pub /: *mut *mut u8 fipenabled; / FIP enabled or not,
    pub /: *mut *mut u8 fipfailed; / FIP failed or not,
    pub resv: [u8; 2],
    pub /: *mut *mut u8 pri; / FCF priority,
    pub /: *mut *mut u8 version; / FIP version used,
    pub /: *mut *mut u8 available; / Available for login,
    pub /: *mut *mut u8 fka_disabled; / FKA is disabled,
    pub /: *mut *mut u8 maxsz_verified; / FCoE max size verified,
    pub /: *mut *mut u8 fc_map[3]; / FC map,
    pub /: *mut *mut __be16 vlan; / FCoE vlan tag/priority,
    pub /: *mut *mut u32 fka_adv_per; / FIP ka advert. period,
    pub /: *mut *mut mac_t mac; / FCF mac,
}

//
// Trunk states for BCU/BFAL
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_trunk_state {
    BFA_TRUNK_DISABLED	= 0,	/*  Trunk is not configured	*/
    BFA_TRUNK_ONLINE	= 1,	/*  Trunk is online		*/
    BFA_TRUNK_OFFLINE	= 2,	/*  Trunk is offline		*/
}

//
// VC attributes for trunked link
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_trunk_vc_attr_s {
    pub bb_credit: u32,
    pub elp_opmode_flags: u32,
    pub req_credit: u32,
    pub vc_credits: [u16; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_fcport_loop_info_s {
    pub /: *mut *mut u8 myalpa; / alpa claimed,
    pub /: *mut *mut u8 alpabm_val; / alpa bitmap valid or not (1 or 0),
    pub resvd: [u8; 6],
    pub /: *mut *mut fc_alpabm_s alpabm; / alpa bitmap,
}

//
// Link state information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_link_s {
    pub /: *mut *mut u8 linkstate; / Link state bfa_port_linkstate,
    pub /: *mut *mut u8 linkstate_rsn; / bfa_port_linkstate_rsn_t,
    pub /: *mut *mut u8 topology; / P2P/LOOP bfa_port_topology,
    pub /: *mut *mut u8 speed; / Link speed (1/2/4/8 G),
    pub /: *mut *mut u32 linkstate_opt; / Linkstate optional data (debug),
    pub /: *mut *mut u8 trunked; / Trunked or not (1 or 0),
    pub /: *mut *mut u8 fec_state; /!< State of FEC,
    pub resvd: [u8; 6],
    pub /: *mut *mut bfa_qos_attr_s qos_attr; / QoS Attributes,
    pub loop_info: bfa_fcport_loop_info_s,
    pub bbcr_attr: bfa_bbcr_attr_s,
    pub qos_vc_attr: bfa_qos_vc_attr_s,
// VC info from ELP
    pub trunk_vc_attr: bfa_trunk_vc_attr_s,
    pub fcf: bfa_fcport_fcf_s,
// FCF information (for FCoE)
    pub vc_fcf: },
    pub attr: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_trunk_link_fctl {
    BFA_TRUNK_LINK_FCTL_NORMAL,
    BFA_TRUNK_LINK_FCTL_VC,
    BFA_TRUNK_LINK_FCTL_VC_QOS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_trunk_link_state {
    BFA_TRUNK_LINK_STATE_UP = 1,		/* link part of trunk */
    BFA_TRUNK_LINK_STATE_DN_LINKDN = 2,	/* physical link down */
    BFA_TRUNK_LINK_STATE_DN_GRP_MIS = 3,	/* trunk group different */
    BFA_TRUNK_LINK_STATE_DN_SPD_MIS = 4,	/* speed mismatch */
    BFA_TRUNK_LINK_STATE_DN_MODE_MIS = 5,	/* remote port not trunked */
}

pub const BFA_TRUNK_MAX_PORTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_trunk_link_attr_s {
    pub trunk_wwn: wwn_t,
    pub fctl: bfa_trunk_link_fctl,
    pub link_state: bfa_trunk_link_state,
    pub speed: bfa_port_speed,
    pub deskew: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_trunk_attr_s {
    pub state: bfa_trunk_state,
    pub speed: bfa_port_speed,
    pub port_id: u32,
    pub rsvd: u32,
    pub link_attr: [bfa_trunk_link_attr_s; BFA_TRUNK_MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_hal_stats_s {
    pub /: *mut *mut u32 sm_un_cr; / uninit: create events,
    pub /: *mut *mut u32 sm_un_unexp; / uninit: exception events,
    pub /: *mut *mut u32 sm_cr_on; / created: online events,
    pub /: *mut *mut u32 sm_cr_del; / created: delete events,
    pub /: *mut *mut u32 sm_cr_hwf; / created: IOC down,
    pub /: *mut *mut u32 sm_cr_unexp; / created: exception events,
    pub /: *mut *mut u32 sm_fwc_rsp; / fw create: f/w responses,
    pub /: *mut *mut u32 sm_fwc_del; / fw create: delete events,
    pub /: *mut *mut u32 sm_fwc_off; / fw create: offline events,
    pub /: *mut *mut u32 sm_fwc_hwf; / fw create: IOC down,
    pub events*/: *mut *mut u32 sm_fwc_unexp; / fw create: exception,
    pub /: *mut *mut u32 sm_on_off; / online: offline events,
    pub /: *mut *mut u32 sm_on_del; / online: delete events,
    pub /: *mut *mut u32 sm_on_hwf; / online: IOC down events,
    pub /: *mut *mut u32 sm_on_unexp; / online: exception events,
    pub /: *mut *mut u32 sm_fwd_rsp; / fw delete: fw responses,
    pub /: *mut *mut u32 sm_fwd_del; / fw delete: delete events,
    pub /: *mut *mut u32 sm_fwd_hwf; / fw delete: IOC down events,
    pub events*/: *mut *mut u32 sm_fwd_unexp; / fw delete: exception,
    pub /: *mut *mut u32 sm_off_del; / offline: delete events,
    pub /: *mut *mut u32 sm_off_on; / offline: online events,
    pub /: *mut *mut u32 sm_off_hwf; / offline: IOC down events,
    pub /: *mut *mut u32 sm_off_unexp; / offline: exception events,
    pub /: *mut *mut u32 sm_del_fwrsp; / delete: fw responses,
    pub /: *mut *mut u32 sm_del_hwf; / delete: IOC down events,
    pub /: *mut *mut u32 sm_del_unexp; / delete: exception events,
    pub /: *mut *mut u32 sm_delp_fwrsp; / delete pend: fw responses,
    pub /: *mut *mut u32 sm_delp_hwf; / delete pend: IOC downs,
    pub /: *mut *mut u32 sm_delp_unexp; / delete pend: exceptions,
    pub /: *mut *mut u32 sm_offp_fwrsp; / off-pending: fw responses,
    pub /: *mut *mut u32 sm_offp_del; / off-pending: deletes,
    pub /: *mut *mut u32 sm_offp_hwf; / off-pending: IOC downs,
    pub /: *mut *mut u32 sm_offp_unexp; / off-pending: exceptions,
    pub /: *mut *mut u32 sm_iocd_off; / IOC down: offline events,
    pub /: *mut *mut u32 sm_iocd_del; / IOC down: delete events,
    pub /: *mut *mut u32 sm_iocd_on; / IOC down: online events,
    pub /: *mut *mut u32 sm_iocd_unexp; / IOC down: exceptions,
    pub rsvd: u32,
}

//
// Rport's QoS attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_qos_attr_s {
    pub /: *mut *mut u8 qos_priority; / rport's QoS priority,
    pub rsvd: [u8; 3],
    pub /: *mut *mut u32 qos_flow_id; / QoS flow Id,
}

pub const BFA_IOBUCKET_MAX: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itnim_latency_s {
    pub min: [u32; BFA_IOBUCKET_MAX],
    pub max: [u32; BFA_IOBUCKET_MAX],
    pub count: [u32; BFA_IOBUCKET_MAX],
    pub avg: [u32; BFA_IOBUCKET_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_itnim_ioprofile_s {
    pub clock_res_mul: u32,
    pub clock_res_div: u32,
    pub index: u32,
    pub /: *mut *mut u32 io_profile_start_time; / IO profile start time,
    pub /: *mut *mut u32 iocomps[BFA_IOBUCKET_MAX]; / IO completed,
    pub io_latency: bfa_itnim_latency_s,
}

//
// vHBA port attribute values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_vhba_attr_s {
    pub /: *mut *mut wwn_t nwwn; / node wwn,
    pub /: *mut *mut wwn_t pwwn; / port wwn,
    pub /: *mut *mut u32 pid; / port ID,
    pub /: *mut *mut bfa_boolean_t io_profile; / get it from fcpim mod,
    pub /: *mut *mut bfa_boolean_t plog_enabled; / portlog is enabled,
    pub path_tov: u16,
    pub rsvd: [u8; 2],
}

//
// FC physical port statistics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_fc_stats_s {
    pub /: *mut *mut u64 secs_reset; / Seconds since stats is reset,
    pub /: *mut *mut u64 tx_frames; / Tx frames,
    pub /: *mut *mut u64 tx_words; / Tx words,
    pub /: *mut *mut u64 tx_lip; / Tx LIP,
    pub /: *mut *mut u64 tx_lip_f7f7; / Tx LIP_F7F7,
    pub /: *mut *mut u64 tx_lip_f8f7; / Tx LIP_F8F7,
    pub /: *mut *mut u64 tx_arbf0; / Tx ARB F0,
    pub /: *mut *mut u64 tx_nos; / Tx NOS,
    pub /: *mut *mut u64 tx_ols; / Tx OLS,
    pub /: *mut *mut u64 tx_lr; / Tx LR,
    pub /: *mut *mut u64 tx_lrr; / Tx LRR,
    pub /: *mut *mut u64 rx_frames; / Rx frames,
    pub /: *mut *mut u64 rx_words; / Rx words,
    pub /: *mut *mut u64 lip_count; / Rx LIP,
    pub /: *mut *mut u64 rx_lip_f7f7; / Rx LIP_F7F7,
    pub /: *mut *mut u64 rx_lip_f8f7; / Rx LIP_F8F7,
    pub /: *mut *mut u64 rx_arbf0; / Rx ARB F0,
    pub /: *mut *mut u64 nos_count; / Rx NOS,
    pub /: *mut *mut u64 ols_count; / Rx OLS,
    pub /: *mut *mut u64 lr_count; / Rx LR,
    pub /: *mut *mut u64 lrr_count; / Rx LRR,
    pub /: *mut *mut u64 invalid_crcs; / Rx CRC err frames,
    pub /: *mut *mut u64 invalid_crc_gd_eof; / Rx CRC err good EOF frames,
    pub /: *mut *mut u64 undersized_frm; / Rx undersized frames,
    pub /: *mut *mut u64 oversized_frm; / Rx oversized frames,
    pub /: *mut *mut u64 bad_eof_frm; / Rx frames with bad EOF,
    pub /: *mut *mut u64 error_frames; / Errored frames,
    pub /: *mut *mut u64 dropped_frames; / Dropped frames,
    pub /: *mut *mut u64 link_failures; / Link Failure (LF) count,
    pub /: *mut *mut u64 loss_of_syncs; / Loss of sync count,
    pub /: *mut *mut u64 loss_of_signals; / Loss of signal count,
    pub /: *mut *mut u64 primseq_errs; / Primitive sequence protocol err.,
    pub /: *mut *mut u64 bad_os_count; / Invalid ordered sets,
    pub /: *mut *mut u64 err_enc_out; / Encoding err nonframe_8b10b,
    pub /: *mut *mut u64 err_enc; / Encoding err frame_8b10b,
    pub /: *mut *mut u64 bbcr_frames_lost; /!< BBCR Frames Lost,
    pub /: *mut *mut u64 bbcr_rrdys_lost; /!< BBCR RRDYs Lost,
    pub /: *mut *mut u64 bbcr_link_resets; /!< BBCR Link Resets,
    pub /: *mut *mut u64 bbcr_frame_lost_intrs; /!< BBCR Frame loss intrs,
    pub /: *mut *mut u64 bbcr_rrdy_lost_intrs; /!< BBCR Rrdy loss intrs,
    pub /: *mut *mut u64 loop_timeouts; / Loop timeouts,
}

//
// Eth Physical Port statistics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_eth_stats_s {
    pub /: *mut *mut u64 secs_reset; / Seconds since stats is reset,
    pub /: *mut *mut u64 frame_64; / Frames 64 bytes,
    pub /: *mut *mut u64 frame_65_127; / Frames 65-127 bytes,
    pub /: *mut *mut u64 frame_128_255; / Frames 128-255 bytes,
    pub /: *mut *mut u64 frame_256_511; / Frames 256-511 bytes,
    pub /: *mut *mut u64 frame_512_1023; / Frames 512-1023 bytes,
    pub /: *mut *mut u64 frame_1024_1518; / Frames 1024-1518 bytes,
    pub /: *mut *mut u64 frame_1519_1522; / Frames 1519-1522 bytes,
    pub /: *mut *mut u64 tx_bytes; / Tx bytes,
    pub /: *mut *mut u64 tx_packets; / Tx packets,
    pub /: *mut *mut u64 tx_mcast_packets; / Tx multicast packets,
    pub /: *mut *mut u64 tx_bcast_packets; / Tx broadcast packets,
    pub /: *mut *mut u64 tx_control_frame; / Tx control frame,
    pub /: *mut *mut u64 tx_drop; / Tx drops,
    pub /: *mut *mut u64 tx_jabber; / Tx jabber,
    pub /: *mut *mut u64 tx_fcs_error; / Tx FCS errors,
    pub /: *mut *mut u64 tx_fragments; / Tx fragments,
    pub /: *mut *mut u64 rx_bytes; / Rx bytes,
    pub /: *mut *mut u64 rx_packets; / Rx packets,
    pub /: *mut *mut u64 rx_mcast_packets; / Rx multicast packets,
    pub /: *mut *mut u64 rx_bcast_packets; / Rx broadcast packets,
    pub /: *mut *mut u64 rx_control_frames; / Rx control frames,
    pub /: *mut *mut u64 rx_unknown_opcode; / Rx unknown opcode,
    pub /: *mut *mut u64 rx_drop; / Rx drops,
    pub /: *mut *mut u64 rx_jabber; / Rx jabber,
    pub /: *mut *mut u64 rx_fcs_error; / Rx FCS errors,
    pub /: *mut *mut u64 rx_alignment_error; / Rx alignment errors,
    pub /: *mut *mut u64 rx_frame_length_error; / Rx frame len errors,
    pub /: *mut *mut u64 rx_code_error; / Rx code errors,
    pub /: *mut *mut u64 rx_fragments; / Rx fragments,
    pub /: *mut *mut u64 rx_pause; / Rx pause,
    pub /: *mut *mut u64 rx_zero_pause; / Rx zero pause,
    pub /: *mut *mut u64 tx_pause; / Tx pause,
    pub /: *mut *mut u64 tx_zero_pause; / Tx zero pause,
    pub /: *mut *mut u64 rx_fcoe_pause; / Rx FCoE pause,
    pub /: *mut *mut u64 rx_fcoe_zero_pause; / Rx FCoE zero pause,
    pub /: *mut *mut u64 tx_fcoe_pause; / Tx FCoE pause,
    pub /: *mut *mut u64 tx_fcoe_zero_pause; / Tx FCoE zero pause,
    pub /: *mut *mut u64 rx_iscsi_pause; / Rx iSCSI pause,
    pub /: *mut *mut u64 rx_iscsi_zero_pause; / Rx iSCSI zero pause,
    pub /: *mut *mut u64 tx_iscsi_pause; / Tx iSCSI pause,
    pub /: *mut *mut u64 tx_iscsi_zero_pause; / Tx iSCSI zero pause,
}

//
// Port statistics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bfa_port_stats_u {
    pub fc: bfa_port_fc_stats_s,
    pub eth: bfa_port_eth_stats_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_port_cfg_mode_s {
    pub max_pf: u16,
    pub max_vf: u16,
    pub mode: bfa_mode_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_lldp_str_s {
    pub sub_type: u8,
    pub len: u8,
    pub rsvd: [u8; 2],
    pub value: [u8; BFA_CEE_LLDP_MAX_STRING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_lldp_cfg_s {
    pub chassis_id: bfa_cee_lldp_str_s,
    pub port_id: bfa_cee_lldp_str_s,
    pub port_desc: bfa_cee_lldp_str_s,
    pub sys_name: bfa_cee_lldp_str_s,
    pub sys_desc: bfa_cee_lldp_str_s,
    pub mgmt_addr: bfa_cee_lldp_str_s,
    pub time_to_live: u16,
    pub enabled_system_cap: u16,
}

// CEE/DCBX parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_dcbx_cfg_s {
    pub pgid: [u8; BFA_CEE_DCBX_MAX_PRIORITY],
    pub pg_percentage: [u8; BFA_CEE_DCBX_MAX_PGID],
    pub /: *mut *mut u8 pfc_primap; / bitmap of priorties with PFC enabled,
    pub /: *mut *mut u8 fcoe_primap; / bitmap of priorities used for FcoE traffic,
    pub /: *mut *mut u8 iscsi_primap; / bitmap of priorities used for iSCSI traffic,
    pub /: *mut *mut u8 dcbx_version; / operating version:CEE or preCEE,
    pub /: *mut *mut u8 lls_fcoe; / FCoE Logical Link Status,
    pub /: *mut *mut u8 lls_lan; / LAN Logical Link Status,
    pub rsvd: [u8; 2],
}

// CEE Query
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_attr_s {
    pub cee_status: u8,
    pub error_reason: u8,
    pub lldp_remote: bfa_cee_lldp_cfg_s,
    pub dcbx_remote: bfa_cee_dcbx_cfg_s,
    pub src_mac: mac_t,
    pub link_speed: u8,
    pub nw_priority: u8,
    pub filler: [u8; 2],
}

// LLDP/DCBX/CEE Statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_cee_stats_s {
    pub /: *mut *mut u32 lldp_tx_frames; / LLDP Tx Frames,
    pub /: *mut *mut u32 lldp_rx_frames; / LLDP Rx Frames,
    pub /: *mut *mut u32 lldp_rx_frames_invalid; / LLDP Rx Frames invalid,
    pub /: *mut *mut u32 lldp_rx_frames_new; / LLDP Rx Frames new,
    pub /: *mut *mut u32 lldp_tlvs_unrecognized; / LLDP Rx unrecog. TLVs,
    pub /: *mut *mut u32 lldp_rx_shutdown_tlvs; / LLDP Rx shutdown TLVs,
    pub /: *mut *mut u32 lldp_info_aged_out; / LLDP remote info aged,
    pub /: *mut *mut u32 dcbx_phylink_ups; / DCBX phy link ups,
    pub /: *mut *mut u32 dcbx_phylink_downs; / DCBX phy link downs,
    pub /: *mut *mut u32 dcbx_rx_tlvs; / DCBX Rx TLVs,
    pub /: *mut *mut u32 dcbx_rx_tlvs_invalid; / DCBX Rx TLVs invalid,
    pub /: *mut *mut u32 dcbx_control_tlv_error; / DCBX control TLV errors,
    pub /: *mut *mut u32 dcbx_feature_tlv_error; / DCBX feature TLV errors,
    pub /: *mut *mut u32 dcbx_cee_cfg_new; / DCBX new CEE cfg rcvd,
    pub /: *mut *mut u32 cee_status_down; / DCB status down,
    pub /: *mut *mut u32 cee_status_up; / DCB status up,
    pub /: *mut *mut u32 cee_hw_cfg_changed; / DCB hw cfg changed,
    pub /: *mut *mut u32 cee_rx_invalid_cfg; / DCB invalid cfg,
}

//
// AEN related definitions
//

// BFA remote port events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfa_rport_aen_event {
    BFA_RPORT_AEN_ONLINE     = 1,   /* RPort online event */
    BFA_RPORT_AEN_OFFLINE    = 2,   /* RPort offline event */
    BFA_RPORT_AEN_DISCONNECT = 3,   /* RPort disconnect event */
    BFA_RPORT_AEN_QOS_PRIO   = 4,   /* QOS priority change event */
    BFA_RPORT_AEN_QOS_FLOWID = 5,   /* QOS flow Id change event */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_rport_aen_data_s {
    pub /: *mut *mut u16 vf_id; / vf_id of this logical port,
    pub rsvd: [u16; 3],
    pub /: *mut *mut wwn_t ppwwn; / WWN of its physical port,
    pub /: *mut *mut wwn_t lpwwn; / WWN of this logical port,
    pub /: *mut *mut wwn_t rpwwn; / WWN of this remote port,
    pub qos: bfa_rport_qos_attr_s,
    pub priv: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bfa_aen_data_u {
    pub adapter: bfa_adapter_aen_data_s,
    pub port: bfa_port_aen_data_s,
    pub lport: bfa_lport_aen_data_s,
    pub rport: bfa_rport_aen_data_s,
    pub itnim: bfa_itnim_aen_data_s,
    pub audit: bfa_audit_aen_data_s,
    pub ioc: bfa_ioc_aen_data_s,
}

pub const BFA_AEN_MAX_ENTRY: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfa_aen_entry_s {
    pub qe: list_head,
    pub aen_category: bfa_aen_category,
    pub aen_type: c_int,
    pub aen_data: bfa_aen_data_u,
    pub aen_tv_sec: u64,
    pub aen_tv_usec: u64,
    pub seq_num: u32,
    pub bfad_num: u32,
}
