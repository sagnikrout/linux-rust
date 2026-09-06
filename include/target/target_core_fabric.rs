//! Automatically rewritten from C Header to Rust Module
//! Source: include/target/target_core_fabric.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_core_fabric_ops {
    pub module: *mut module,
//
// XXX: Special case for iscsi/iSCSI...
// If non-null, fabric_alias is used for matching target/$fabric
// ConfigFS paths. If null, fabric_name is used for this (see below).
//
    pub fabric_alias: *const c_char,
//
// fabric_name is used for matching target/$fabric ConfigFS paths
// without a fabric_alias (see above). It's also used for the ALUA state
// path and is stored on disk with PR state.
//
    pub fabric_name: *const c_char,
    pub node_acl_size: usize,
//
// Limits number of scatterlist entries per SCF_SCSI_DATA_CDB payload.
// Setting this value tells target-core to enforce this limit, and
// report as INQUIRY EVPD=b0 MAXIMUM TRANSFER LENGTH.
//
// target-core will currently reset se_cmd->data_length to this
// maximum size, and set UNDERFLOW residual count if length exceeds
// this limit.
//
// XXX: Not all initiator hosts honor this block-limit EVPD
// XXX: Currently assumes single PAGE_SIZE per scatterlist entry
//
    pub max_data_sg_nents: u32,
    pub ): *mut *mut *mut char (tpg_get_wwn)(struct se_portal_group,
    pub ): *mut *mut u16 (tpg_get_tag)(struct se_portal_group,
    pub ): *mut *mut u32 (tpg_get_default_depth)(struct se_portal_group,
    pub ): *mut *mut int (tpg_check_demo_mode)(struct se_portal_group,
    pub ): *mut *mut int (tpg_check_demo_mode_cache)(struct se_portal_group,
    pub ): *mut *mut int (tpg_check_demo_mode_write_protect)(struct se_portal_group,
    pub ): *mut *mut int (tpg_check_prod_mode_write_protect)(struct se_portal_group,
//
// Optionally used by fabrics to allow demo-mode login, but not
// expose any TPG LUNs, and return 'not connected' in standard
// inquiry response
//
    pub ): *mut *mut int (tpg_check_demo_mode_login_only)(struct se_portal_group,
//
// Optionally used as a configfs tunable to determine when
// target-core should signal the PROTECT=1 feature bit for
// backends that don't support T10-PI, so that either fabric
// HW offload or target-core emulation performs the associated
// WRITE_STRIP and READ_INSERT operations.
//
    pub ): *mut *mut int (tpg_check_prot_fabric_only)(struct se_portal_group,
    pub ): *mut *mut u32 (tpg_get_inst_index)(struct se_portal_group,
//
// Optional to release struct se_cmd and fabric dependent allocated
// I/O descriptor after command execution has finished.
//
// Returning 1 will signal a descriptor has been released.
// Returning 0 will signal a descriptor has not been released.
//
    pub ): *mut *mut int (check_stop_free)(struct se_cmd,
    pub ): *mut *mut void (release_cmd)(struct se_cmd,
    pub ): *mut *mut void (close_session)(struct se_session,
    pub ): *mut *mut u32 (sess_get_index)(struct se_session,
//
// Used only for SCSI fabrics that contain multi-value TransportIDs
// (like iSCSI).  All other SCSI fabrics should set this to NULL.
//
    pub u32): *mut *mut unsigned char ,,
    pub ): *mut *mut int (write_pending)(struct se_cmd,
    pub ): *mut *mut void (set_default_node_attributes)(struct se_node_acl,
    pub ): *mut *mut int (get_cmd_state)(struct se_cmd,
    pub ): *mut *mut int (queue_data_in)(struct se_cmd,
    pub ): *mut *mut int (queue_status)(struct se_cmd,
    pub ): *mut *mut void (queue_tm_rsp)(struct se_cmd,
    pub ): *mut *mut void (aborted_task)(struct se_cmd,
//
// fabric module calls for target_core_fabric_configfs.c
//
    pub ): *const *const config_group , char,
    pub ): *mut *mut void (fabric_drop_wwn)(struct se_wwn,
    pub ): *mut *mut void (add_wwn_groups)(struct se_wwn,
    pub ): *const c_char,
    pub enable): *mut *mut *mut int (fabric_enable_tpg)(struct se_portal_group se_tpg, bool,
    pub ): *mut *mut void (fabric_drop_tpg)(struct se_portal_group,
    pub ): *mut se_lun,
    pub ): *mut se_lun,
    pub lun): *mut se_lun,
    pub ): *const *const config_group , char,
    pub ): *mut *mut void (fabric_drop_np)(struct se_tpg_np,
    pub ): *const *const *const int (fabric_init_nodeacl)(struct se_node_acl , char,
    pub tfc_discovery_attrs: *mut configfs_attribute,
    pub tfc_wwn_attrs: *mut configfs_attribute,
    pub tfc_tpg_base_attrs: *mut configfs_attribute,
    pub tfc_tpg_np_base_attrs: *mut configfs_attribute,
    pub tfc_tpg_attrib_attrs: *mut configfs_attribute,
    pub tfc_tpg_auth_attrs: *mut configfs_attribute,
    pub tfc_tpg_param_attrs: *mut configfs_attribute,
    pub tfc_tpg_nacl_base_attrs: *mut configfs_attribute,
    pub tfc_tpg_nacl_attrib_attrs: *mut configfs_attribute,
    pub tfc_tpg_nacl_auth_attrs: *mut configfs_attribute,
    pub tfc_tpg_nacl_param_attrs: *mut configfs_attribute,
//
// Set this member variable if the SCSI transport protocol
// (e.g. iSCSI) requires that the Data-Out buffer is transferred in
// its entirety before a command is aborted.
//
    pub write_pending_must_be_called:1: c_uint,
//
// Set this if the driver does not require calling queue_data_in
// queue_status and check_stop_free from a worker thread when
// completing successful commands.
//
    pub direct_compl_supp:1: c_uint,
//
// Set this if the driver supports submitting commands to the backend
// from target_submit/target_submit_cmd.
//
    pub direct_submit_supp:1: c_uint,
// Set this to a target_submit_type value.
    pub default_submit_type: u8,
// Set this to the target_compl_type value.
    pub default_compl_type: u8,
}

extern "C" {
    pub fn target_register_template(fo: *const target_core_fabric_ops) -> c_int;
}
extern "C" {
    pub fn target_unregister_template(fo: *const target_core_fabric_ops);
}
extern "C" {
    pub fn target_depend_item(item: *mut config_item) -> c_int;
}
extern "C" {
    pub fn target_undepend_item(item: *mut config_item);
}
extern "C" {
    pub fn target_remove_session(: *mut se_session);
}
extern "C" {
    pub fn target_stop_cmd_counter(cmd_cnt: *mut target_cmd_counter);
}
extern "C" {
    pub fn target_wait_for_cmds(cmd_cnt: *mut target_cmd_counter);
}
extern "C" {
    pub fn target_free_cmd_counter(cmd_cnt: *mut target_cmd_counter);
}
extern "C" {
    pub fn transport_init_session(se_sess: *mut se_session);
}
extern "C" {
    pub fn target_show_dynamic_sessions(: *mut se_portal_group, : *mut c_char) -> isize;
}
extern "C" {
    pub fn transport_free_session(: *mut se_session);
}
extern "C" {
    pub fn target_spc2_release(nacl: *mut se_node_acl);
}
extern "C" {
    pub fn target_put_nacl(: *mut se_node_acl);
}
extern "C" {
    pub fn transport_deregister_session_configfs(: *mut se_session);
}
extern "C" {
    pub fn transport_deregister_session(: *mut se_session);
}
extern "C" {
    pub fn target_submit(se_cmd: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn transport_lookup_cmd_lun(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_cmd_parse_cdb(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn transport_generic_new_cmd(: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn target_put_cmd_and_wait(cmd: *mut se_cmd);
}
extern "C" {
    pub fn target_execute_cmd(cmd: *mut se_cmd);
}
extern "C" {
    pub fn transport_generic_free_cmd(: *mut se_cmd, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn transport_wait_for_tasks(: *mut se_cmd) -> bool;
}
extern "C" {
    pub fn target_send_busy(cmd: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn target_get_sess_cmd(: *mut se_cmd, _arg: bool) -> c_int;
}
extern "C" {
    pub fn target_put_sess_cmd(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn target_stop_session(se_sess: *mut se_session);
}
extern "C" {
    pub fn target_wait_for_sess_cmds(: *mut se_session);
}
extern "C" {
    pub fn target_show_cmd(pfx: *const c_char, cmd: *mut se_cmd);
}
extern "C" {
    pub fn core_tmr_alloc_req(: *mut se_cmd, : *mut c_void, _arg: u8, _arg: gfp_t) -> c_int;
}
extern "C" {
    pub fn core_tmr_release_req(: *mut se_tmr_req);
}
extern "C" {
    pub fn transport_generic_handle_tmr(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn transport_generic_request_failure(: *mut se_cmd, _arg: sense_reason_t);
}
extern "C" {
    pub fn transport_lookup_tmr_lun(: *mut se_cmd) -> c_int;
}
extern "C" {
    pub fn core_allocate_nexus_loss_ua(acl: *mut se_node_acl);
}
extern "C" {
    pub fn core_tpg_set_initiator_node_queue_depth(: *mut se_node_acl, _arg: u32) -> c_int;
}
extern "C" {
    pub fn core_tpg_register(: *mut se_wwn, : *mut se_portal_group, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn core_tpg_deregister(: *mut se_portal_group) -> c_int;
}
extern "C" {
    pub fn target_free_sgl(sgl: *mut scatterlist, nents: c_int);
}
//
// The LIO target core uses DMA_TO_DEVICE to mean that data is going
// to the target (eg handling a WRITE) and DMA_FROM_DEVICE to mean
// that data is coming from the target (eg handling a READ).  However,
// this is just the opposite of what we have to tell the DMA mapping
// layer -- eg when handling a READ, the HBA will have to DMA the data
// out of memory so it can send it to the initiator, which means we
// need to use DMA_TO_DEVICE when we map the data.
//
