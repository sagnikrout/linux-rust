//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/target/target_core_internal.h
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

pub const TARGET_CORE_NAME_MAX_LEN: c_int = 64;
pub const TARGET_FABRIC_NAME_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_backend {
    pub list: list_head,
    pub ops: *const target_backend_ops,
    pub tb_dev_cit: config_item_type,
    pub tb_dev_attrib_cit: config_item_type,
    pub tb_dev_action_cit: config_item_type,
    pub tb_dev_pr_cit: config_item_type,
    pub tb_dev_wwn_cit: config_item_type,
    pub tb_dev_alua_tg_pt_gps_cit: config_item_type,
    pub tb_dev_stat_cit: config_item_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_fabric_configfs {
    pub tf_access_cnt: core::sync::atomic::AtomicI32,
    pub tf_list: list_head,
    pub tf_group: config_group,
    pub tf_disc_group: config_group,
    pub tf_ops: *const target_core_fabric_ops,
    pub tf_discovery_cit: config_item_type,
    pub tf_wwn_cit: config_item_type,
    pub tf_wwn_fabric_stats_cit: config_item_type,
    pub tf_wwn_param_cit: config_item_type,
    pub tf_tpg_cit: config_item_type,
    pub tf_tpg_base_cit: config_item_type,
    pub tf_tpg_lun_cit: config_item_type,
    pub tf_tpg_port_cit: config_item_type,
    pub tf_tpg_port_stat_cit: config_item_type,
    pub tf_tpg_np_cit: config_item_type,
    pub tf_tpg_np_base_cit: config_item_type,
    pub tf_tpg_attrib_cit: config_item_type,
    pub tf_tpg_auth_cit: config_item_type,
    pub tf_tpg_param_cit: config_item_type,
    pub tf_tpg_nacl_cit: config_item_type,
    pub tf_tpg_nacl_base_cit: config_item_type,
    pub tf_tpg_nacl_attrib_cit: config_item_type,
    pub tf_tpg_nacl_auth_cit: config_item_type,
    pub tf_tpg_nacl_param_cit: config_item_type,
    pub tf_tpg_nacl_stat_cit: config_item_type,
    pub tf_tpg_mappedlun_cit: config_item_type,
    pub tf_tpg_mappedlun_stat_cit: config_item_type,
}

// target_core_alua.c
// target_core_device.c
extern "C" {
    pub fn target_pr_kref_release(: *mut kref);
}
extern "C" {
    pub fn core_update_device_list_access(_arg: u64, _arg: bool, : *mut se_node_acl);
}
extern "C" {
    pub fn core_clear_lun_from_tpg(: *mut se_lun, : *mut se_portal_group);
}
extern "C" {
    pub fn core_dev_del_lun(: *mut se_portal_group, : *mut se_lun);
}
extern "C" {
    pub fn core_dev_setup_virtual_lun0() -> c_int;
}
extern "C" {
    pub fn core_dev_release_virtual_lun0();
}
extern "C" {
    pub fn target_configure_device(dev: *mut se_device) -> c_int;
}
extern "C" {
    pub fn target_free_device(: *mut se_device);
}
extern "C" {
    pub fn target_dev_ua_allocate(dev: *mut se_device, asc: u8, ascq: u8);
}
// target_core_configfs.c
extern "C" {
    pub fn target_setup_backend_cits(: *mut target_backend);
}
// target_core_fabric_configfs.c
extern "C" {
    pub fn target_fabric_setup_cits(: *mut target_fabric_configfs) -> c_int;
}
// target_core_fabric_lib.c
// target_core_hba.c
extern "C" {
    pub fn core_delete_hba(: *mut se_hba) -> c_int;
}
// target_core_tmr.c
// target_core_tpg.c
extern "C" {
    pub fn core_tpg_wait_for_nacl_pr_ref(: *mut se_node_acl);
}
extern "C" {
    pub fn target_tpg_free_lun(head: *mut rcu_head);
}
extern "C" {
    pub fn core_tpg_remove_lun(: *mut se_portal_group, : *mut se_lun);
}
extern "C" {
    pub fn core_tpg_del_initiator_node_acl(acl: *mut se_node_acl);
}
extern "C" {
    pub fn target_tpg_enable(se_tpg: *mut se_portal_group) -> c_int;
}
extern "C" {
    pub fn target_tpg_disable(se_tpg: *mut se_portal_group) -> c_int;
}
// target_core_transport.c
extern "C" {
    pub fn init_se_kmem_caches() -> c_int;
}
extern "C" {
    pub fn release_se_kmem_caches();
}
extern "C" {
    pub fn scsi_get_new_index(_arg: scsi_index_t) -> u32;
}
extern "C" {
    pub fn transport_subsystem_check_init();
}
extern "C" {
    pub fn transport_dump_dev_state(: *mut se_device, : *mut c_char, : *mut c_int);
}
extern "C" {
    pub fn transport_dump_vpd_proto_id(: *mut t10_vpd, : *mut c_uchar, _arg: c_int);
}
extern "C" {
    pub fn transport_dump_vpd_assoc(: *mut t10_vpd, : *mut c_uchar, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn transport_dump_vpd_ident_type(: *mut t10_vpd, : *mut c_uchar, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn transport_dump_vpd_ident(: *mut t10_vpd, : *mut c_uchar, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn transport_clear_lun_ref(: *mut se_lun);
}
extern "C" {
    pub fn target_cmd_size_check(cmd: *mut se_cmd, size: c_uint) -> sense_reason_t;
}
extern "C" {
    pub fn target_qf_do_work(work: *mut work_struct);
}
extern "C" {
    pub fn target_do_delayed_work(work: *mut work_struct);
}
extern "C" {
    pub fn target_check_wce(dev: *mut se_device) -> bool;
}
extern "C" {
    pub fn target_check_fua(dev: *mut se_device) -> bool;
}
extern "C" {
    pub fn __target_execute_cmd(: *mut se_cmd, _arg: bool);
}
extern "C" {
    pub fn target_queued_submit_work(work: *mut work_struct);
}
// target_core_stat.c
extern "C" {
    pub fn target_stat_setup_dev_default_groups(: *mut se_device);
}
extern "C" {
    pub fn target_stat_setup_port_default_groups(: *mut se_lun);
}
extern "C" {
    pub fn target_stat_setup_mappedlun_default_groups(: *mut se_lun_acl);
}
// target_core_xcopy.c
// target_core_configfs.c
pub const DB_ROOT_LEN: c_int = 4096;

