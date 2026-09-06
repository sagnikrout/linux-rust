//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvme/target/nvmet.h
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
// Copyright (c) 2015-2016 HGST, a Western Digital Company.
//

pub const NVMET_ASYNC_EVENTS: c_int = 4;
pub const NVMET_ERROR_LOG_SLOTS: c_int = 128;

pub const NVMET_MN_MAX_SIZE: c_int = 40;
pub const NVMET_SN_MAX_SIZE: c_int = 20;
pub const NVMET_FR_MAX_SIZE: c_int = 8;
pub const NVMET_PR_LOG_QUEUE_SIZE: c_int = 64;

//
// Supported optional AENs:
//

//
// Plus mandatory SMART AENs (we'll never send them, but allow enabling them):
//

// Helper Macros when NVMe error is NVME_SC_CONNECT_INVALID_PARAM
// The 16 bit shift is to set IATTR bit to 1, which means offending
// offset starts in the data section of connect()
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_pr_registrant {
    pub rkey: u64,
    pub hostid: uuid_t,
    pub rtype: nvme_pr_type,
    pub entry: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_pr {
    pub enable: bool,
    pub notify_mask: c_ulong,
    pub generation: core::sync::atomic::AtomicI32,
    pub holder: *mut nvmet_pr_registrant __rcu,
//
// During the execution of the reservation command, mutual
// exclusion is required throughout the process. However,
// while waiting asynchronously for the 'per controller
// percpu_ref' to complete before the 'preempt and abort'
// command finishes, a semaphore is needed to ensure mutual
// exclusion instead of a mutex.
//
    pub pr_sem: semaphore,
    pub registrant_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_pr_per_ctrl_ref {
    pub ref: percpu_ref,
    pub free_done: completion,
    pub confirm_done: completion,
    pub hostid: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_ns {
    pub ref: percpu_ref,
    pub bdev_file: *mut file,
    pub bdev: *mut block_device,
    pub file: *mut file,
    pub readonly: bool,
    pub nsid: u32,
    pub blksize_shift: u32,
    pub size: loff_t,
    pub nguid: [u8; 16],
    pub uuid: uuid_t,
    pub anagrpid: u32,
    pub buffered_io: bool,
    pub enabled: bool,
    pub subsys: *mut nvmet_subsys,
    pub device_path: *const c_char,
    pub device_group: config_group,
    pub group: config_group,
    pub disable_done: completion,
    pub bvec_pool: *mut mempool_t,
    pub p2p_dev: *mut pci_dev,
    pub use_p2pmem: c_int,
    pub pi_type: c_int,
    pub metadata_size: c_int,
    pub csi: u8,
    pub pr: nvmet_pr,
    pub pr_per_ctrl_refs: xarray,

    pub debugfs_dir: *mut dentry,

}

extern "C" {
    pub fn container_of(_arg: to_config_group(item), nvmet_ns: struct, _arg: group) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_cq {
    pub ctrl: *mut nvmet_ctrl,
    pub qid: u16,
    pub size: u16,
    pub ref: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_sq {
    pub ctrl: *mut nvmet_ctrl,
    pub ref: percpu_ref,
    pub cq: *mut nvmet_cq,
    pub qid: u16,
    pub size: u16,
    pub sqhd: u32,
    pub sqhd_disabled: bool,

    pub authenticated: bool,
    pub auth_expired_work: delayed_work,
    pub dhchap_tid: u16,
    pub sc_c: u8,
    pub dhchap_status: u8,
    pub dhchap_step: u8,
    pub dhchap_c1: *mut u8,
    pub dhchap_c2: *mut u8,
    pub dhchap_s1: u32,
    pub dhchap_s2: u32,
    pub dhchap_skey: *mut u8,
    pub dhchap_skey_len: c_int,

    pub tls_key: *mut key,

    pub free_done: completion,
    pub confirm_done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_ana_group {
    pub group: config_group,
    pub port: *mut nvmet_port,
    pub grpid: u32,
}

//
// struct nvmet_port -	Common structure to keep port
// information for the target.
// @entry:		Entry into referrals or transport list.
// @disc_addr:		Address information is stored in a format defined
// for a discovery log page entry.
// @group:		ConfigFS group for this element's folder.
// @priv:		Private data for the transport.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_port {
    pub entry: list_head,
    pub disc_addr: nvmf_disc_rsp_page_entry,
    pub group: config_group,
    pub subsys_group: config_group,
    pub subsystems: list_head,
    pub referrals_group: config_group,
    pub referrals: list_head,
    pub global_entry: list_head,
    pub ana_groups_group: config_group,
    pub ana_default_group: nvmet_ana_group,
    pub keyring: *mut key,
    pub priv: *mut c_void,
    pub enabled: bool,
    pub inline_data_size: c_int,
    pub max_queue_size: c_int,
    pub mdts: c_int,
    pub tr_ops: *const nvmet_fabrics_ops,
    pub pi_enable: bool,
    pub ana_state: [nvme_ana_state; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_pr_log_mgr {
    pub lock: mutex,
    pub lost_count: u64,
    pub counter: u64,
    pub NVMET_PR_LOG_QUEUE_SIZE): DECLARE_KFIFO(log_queue, struct nvme_pr_log,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_ctrl {
    pub subsys: *mut nvmet_subsys,
    pub sqs: *mut nvmet_sq,
    pub cqs: *mut nvmet_cq,
    pub drvdata: *mut c_void,
    pub reset_tbkas: bool,
    pub lock: mutex,
    pub cap: u64,
    pub cc: u32,
    pub csts: u32,
    pub hostid: uuid_t,
    pub cntlid: u16,
    pub max_qid: u16,
    pub kato: u32,
    pub port: *mut nvmet_port,
    pub aen_enabled: u32,
    pub aen_masked: c_ulong,
    pub async_event_cmds: [*mut nvmet_req; NVMET_ASYNC_EVENTS],
    pub nr_async_event_cmds: c_uint,
    pub async_events: list_head,
    pub async_event_work: work_struct,
    pub subsys_entry: list_head,
    pub ref: kref,
    pub ka_work: delayed_work,
    pub fatal_err_work: work_struct,
    pub ops: *const nvmet_fabrics_ops,
    pub changed_ns_list: *mut __le32,
    pub nr_changed_ns: u32,
    pub hostnqn: [c_char; NVMF_NQN_FIELD_LEN],
    pub p2p_client: *mut device,
    pub p2p_ns_map: radix_tree_root,

    pub debugfs_dir: *mut dentry,

    pub error_lock: spinlock_t,
    pub err_counter: u64,
    pub slots: [nvme_error_slot; NVMET_ERROR_LOG_SLOTS],
    pub pi_support: bool,
    pub concat: bool,

    pub host_key: *mut nvme_dhchap_key,
    pub ctrl_key: *mut nvme_dhchap_key,
    pub shash_id: u8,
    pub dh_tfm: *mut crypto_kpp,
    pub dh_gid: u8,
    pub dh_key: *mut u8,
    pub dh_keysize: usize,

    pub tls_key: *mut key,

    pub pr_log_mgr: nvmet_pr_log_mgr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_subsys {
    pub type: nvme_subsys_type,
    pub lock: mutex,
    pub ref: kref,
    pub namespaces: xarray,
    pub nr_namespaces: c_uint,
    pub max_nsid: u32,
    pub cntlid_min: u16,
    pub cntlid_max: u16,
    pub ctrls: list_head,
    pub hosts: list_head,
    pub allow_any_host: bool,

    pub debugfs_dir: *mut dentry,

    pub max_qid: u16,
    pub ver: u64,
    pub serial: [c_char; NVMET_SN_MAX_SIZE],
    pub subsys_discovered: bool,
    pub subsysnqn: *mut c_char,
    pub pi_support: bool,
    pub group: config_group,
    pub namespaces_group: config_group,
    pub allowed_hosts_group: config_group,
    pub vendor_id: u16,
    pub subsys_vendor_id: u16,
    pub model_number: *mut c_char,
    pub ieee_oui: u32,
    pub firmware_rev: *mut c_char,

    pub passthru_ctrl: *mut nvme_ctrl,
    pub passthru_ctrl_path: *mut c_char,
    pub passthru_group: config_group,
    pub admin_timeout: c_uint,
    pub io_timeout: c_uint,
    pub clear_ids: c_uint,

    pub zasl: u8,

}

extern "C" {
    pub fn container_of(_arg: to_config_group(item), nvmet_subsys: struct, _arg: group) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_host {
    pub group: config_group,
    pub dhchap_secret: *mut u8,
    pub dhchap_ctrl_secret: *mut u8,
    pub dhchap_key_hash: u8,
    pub dhchap_ctrl_key_hash: u8,
    pub dhchap_hash_id: u8,
    pub dhchap_dhgroup_id: u8,
}

extern "C" {
    pub fn container_of(_arg: to_config_group(item), nvmet_host: struct, _arg: group) -> return;
}
extern "C" {
    pub fn config_item_name(_arg: &host->group.cg_item) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_host_link {
    pub entry: list_head,
    pub host: *mut nvmet_host,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_subsys_link {
    pub entry: list_head,
    pub subsys: *mut nvmet_subsys,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_fabrics_ops {
    pub owner: *mut module,
    pub type: c_uint,
    pub msdbd: c_uint,
    pub flags: c_uint,

    pub req): *mut *mut void (queue_response)(struct nvmet_req,
    pub port): *mut *mut int (add_port)(struct nvmet_port,
    pub port): *mut *mut void (remove_port)(struct nvmet_port,
    pub ctrl): *mut *mut void (delete_ctrl)(struct nvmet_ctrl,
    pub traddr): *mut *mut nvmet_port port, char,
    pub traddr_len): *mut *mut char traddr, size_t,
    pub nvme_sq): *mut *mut u16 (install_queue)(struct nvmet_sq,
    pub port): *mut *mut void (discovery_chg)(struct nvmet_port,
    pub ctrl): *const *const u8 (get_mdts)(struct nvmet_ctrl,
    pub ctrl): *const *const u16 (get_max_queue_size)(struct nvmet_ctrl,
// Operations mandatory for PCI target controllers
    pub prp1): u16 qsize, u64,
    pub sqid): *mut *mut *mut u16 (delete_sq)(struct nvmet_ctrl ctrl, u16,
    pub irq_vector): u16 qsize, u64 prp1, u16,
    pub cqid): *mut *mut *mut u16 (delete_cq)(struct nvmet_ctrl ctrl, u16,
    pub feat_data): *mut c_void,
    pub feat_data): *mut c_void,
}

pub const NVMET_MAX_INLINE_BIOVEC: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_req {
    pub cmd: *mut nvme_command,
    pub cqe: *mut nvme_completion,
    pub sq: *mut nvmet_sq,
    pub cq: *mut nvmet_cq,
    pub ns: *mut nvmet_ns,
    pub sg: *mut scatterlist,
    pub metadata_sg: *mut scatterlist,
    pub inline_bvec: [bio_vec; NVMET_MAX_INLINE_BIOVEC],
    pub inline_bio: bio,
    pub b: },
    pub mpool_alloc: bool,
    pub iocb: kiocb,
    pub bvec: *mut bio_vec,
    pub work: work_struct,
    pub f: },
    pub inline_bio: bio,
    pub rq: *mut request,
    pub work: work_struct,
    pub use_workqueue: bool,
    pub p: },

    pub inline_bio: bio,
    pub zmgmt_work: work_struct,
    pub z: },

    pub abort_work: work_struct,
    pub r: },
}

// data length as parsed from the SGL descriptor:
pub const NVMET_MAX_MPOOL_BVEC: c_int = 16;
//
// NVMe command writes actually are DMA reads for us on the target side.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_async_event {
    pub entry: list_head,
    pub event_type: u8,
    pub event_info: u8,
    pub log_page: u8,
}

extern "C" {
    pub fn test_and_set_bit(_arg: bn, _arg: &ctrl->aen_masked) -> return;
}
extern "C" {
    pub fn nvmet_get_feat_kato(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_get_feat_async_event(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_set_feat_kato(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_set_feat_async_event(req: *mut nvmet_req, mask: u32) -> u16;
}
extern "C" {
    pub fn nvmet_execute_async_event(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_start_keep_alive_timer(ctrl: *mut nvmet_ctrl);
}
extern "C" {
    pub fn nvmet_stop_keep_alive_timer(ctrl: *mut nvmet_ctrl);
}
extern "C" {
    pub fn nvmet_parse_connect_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_connect_cmd_data_len(req: *mut nvmet_req) -> u32;
}
extern "C" {
    pub fn nvmet_bdev_set_limits(bdev: *mut block_device, id: *mut nvme_id_ns);
}
extern "C" {
    pub fn nvmet_bdev_parse_io_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_file_parse_io_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_bdev_zns_parse_io_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_admin_cmd_data_len(req: *mut nvmet_req) -> u32;
}
extern "C" {
    pub fn nvmet_parse_admin_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_discovery_cmd_data_len(req: *mut nvmet_req) -> u32;
}
extern "C" {
    pub fn nvmet_parse_discovery_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_parse_fabrics_admin_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_fabrics_admin_cmd_data_len(req: *mut nvmet_req) -> u32;
}
extern "C" {
    pub fn nvmet_parse_fabrics_io_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_fabrics_io_cmd_data_len(req: *mut nvmet_req) -> u32;
}
extern "C" {
    pub fn nvmet_req_uninit(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_req_transfer_len(req: *mut nvmet_req) -> usize;
}
extern "C" {
    pub fn nvmet_check_transfer_len(req: *mut nvmet_req, len: usize) -> bool;
}
extern "C" {
    pub fn nvmet_check_data_len_lte(req: *mut nvmet_req, data_len: usize) -> bool;
}
extern "C" {
    pub fn nvmet_req_complete(req: *mut nvmet_req, status: u16);
}
extern "C" {
    pub fn nvmet_req_alloc_sgls(req: *mut nvmet_req) -> c_int;
}
extern "C" {
    pub fn nvmet_req_free_sgls(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_execute_set_features(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_execute_get_features(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_execute_keep_alive(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_check_cqid(ctrl: *mut nvmet_ctrl, cqid: u16, create: bool) -> u16;
}
extern "C" {
    pub fn nvmet_check_io_cqid(ctrl: *mut nvmet_ctrl, cqid: u16, create: bool) -> u16;
}
extern "C" {
    pub fn nvmet_cq_init(cq: *mut nvmet_cq);
}
extern "C" {
    pub fn nvmet_cq_destroy(cq: *mut nvmet_cq);
}
extern "C" {
    pub fn nvmet_cq_get(cq: *mut nvmet_cq) -> bool;
}
extern "C" {
    pub fn nvmet_cq_put(cq: *mut nvmet_cq);
}
extern "C" {
    pub fn nvmet_cq_in_use(cq: *mut nvmet_cq) -> bool;
}
extern "C" {
    pub fn nvmet_check_sqid(ctrl: *mut nvmet_ctrl, sqid: u16, create: bool) -> u16;
}
extern "C" {
    pub fn nvmet_sq_destroy(sq: *mut nvmet_sq);
}
extern "C" {
    pub fn nvmet_sq_init(sq: *mut nvmet_sq, cq: *mut nvmet_cq) -> c_int;
}
extern "C" {
    pub fn nvmet_ctrl_fatal_error(ctrl: *mut nvmet_ctrl);
}
extern "C" {
    pub fn nvmet_update_cc(ctrl: *mut nvmet_ctrl, new: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_alloc_ctrl_args {
    pub port: *mut nvmet_port,
    pub sq: *mut nvmet_sq,
    pub subsysnqn: *mut c_char,
    pub hostnqn: *mut c_char,
    pub hostid: *mut uuid_t,
    pub ops: *const nvmet_fabrics_ops,
    pub p2p_client: *mut device,
    pub kato: u32,
    pub result: __le32,
    pub error_loc: u16,
    pub status: u16,
}

extern "C" {
    pub fn nvmet_ctrl_put(ctrl: *mut nvmet_ctrl);
}
extern "C" {
    pub fn nvmet_check_ctrl_status(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_subsys_put(subsys: *mut nvmet_subsys);
}
extern "C" {
    pub fn nvmet_subsys_del_ctrls(subsys: *mut nvmet_subsys);
}
extern "C" {
    pub fn nvmet_req_find_ns(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_put_namespace(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_ns_enable(ns: *mut nvmet_ns) -> c_int;
}
extern "C" {
    pub fn nvmet_ns_disable(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_ns_free(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_port_send_ana_event(port: *mut nvmet_port);
}
extern "C" {
    pub fn nvmet_register_transport(ops: *const nvmet_fabrics_ops) -> c_int;
}
extern "C" {
    pub fn nvmet_unregister_transport(ops: *const nvmet_fabrics_ops);
}
extern "C" {
    pub fn nvmet_enable_port(port: *mut nvmet_port) -> c_int;
}
extern "C" {
    pub fn nvmet_disable_port(port: *mut nvmet_port);
}
extern "C" {
    pub fn nvmet_referral_enable(parent: *mut nvmet_port, port: *mut nvmet_port);
}
extern "C" {
    pub fn nvmet_referral_disable(parent: *mut nvmet_port, port: *mut nvmet_port);
}
extern "C" {
    pub fn nvmet_zero_sgl(req: *mut nvmet_req, off: off_t, len: usize) -> u16;
}
extern "C" {
    pub fn nvmet_get_log_page_len(cmd: *mut nvme_command) -> u32;
}
extern "C" {
    pub fn nvmet_get_log_page_offset(cmd: *mut nvme_command) -> u64;
}
pub const NVMET_MIN_QUEUE_SIZE: c_int = 16;
pub const NVMET_MAX_QUEUE_SIZE: c_int = 1024;
pub const NVMET_NR_QUEUES: c_int = 128;

pub const NVMET_MAX_MDTS: c_int = 255;
//
// Nice round number that makes a list of nsids fit into a page.
// Should become tunable at some point in the future.
//
pub const NVMET_MAX_NAMESPACES: c_int = 1024;
//
// 0 is not a valid ANA group ID, so we start numbering at 1.
//
// ANA Group 1 exists without manual intervention, has namespaces assigned to it
// by default, and is available in an optimized state through all ports.
//
pub const NVMET_MAX_ANAGRPS: c_int = 128;
pub const NVMET_DEFAULT_ANA_GRPID: c_int = 1;
pub const NVMET_KAS: c_int = 10;
pub const NVMET_DISC_KATO_MS: c_int = 120000;
extern "C" {
    pub fn nvmet_init_configfs() -> int __init;
}
extern "C" {
    pub fn nvmet_exit_configfs() -> void __exit;
}
extern "C" {
    pub fn nvmet_init_discovery() -> int __init;
}
extern "C" {
    pub fn nvmet_exit_discovery();
}
extern "C" {
    pub fn nvmet_host_allowed(subsys: *mut nvmet_subsys, hostnqn: *const c_char) -> bool;
}
extern "C" {
    pub fn nvmet_bdev_ns_enable(ns: *mut nvmet_ns) -> c_int;
}
extern "C" {
    pub fn nvmet_file_ns_enable(ns: *mut nvmet_ns) -> c_int;
}
extern "C" {
    pub fn nvmet_bdev_ns_disable(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_file_ns_disable(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_bdev_flush(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_file_flush(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_ns_changed(subsys: *mut nvmet_subsys, nsid: u32);
}
extern "C" {
    pub fn nvmet_bdev_ns_revalidate(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_file_ns_revalidate(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_ns_revalidate(ns: *mut nvmet_ns) -> bool;
}
extern "C" {
    pub fn blk_to_nvme_status(req: *mut nvmet_req, blk_sts: blk_status_t) -> u16;
}
extern "C" {
    pub fn nvmet_bdev_zns_enable(ns: *mut nvmet_ns) -> bool;
}
extern "C" {
    pub fn nvmet_execute_identify_ctrl_zns(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_execute_identify_ns_zns(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_bdev_execute_zone_mgmt_recv(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_bdev_execute_zone_mgmt_send(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_bdev_execute_zone_append(req: *mut nvmet_req);
}
// Limit MDTS according to port config or transport capability
extern "C" {
    pub fn min_not_zero(_arg: ctrl->ops->get_mdts(ctrl), _arg: mdts) -> return;
}

extern "C" {
    pub fn nvmet_passthru_subsys_free(subsys: *mut nvmet_subsys);
}
extern "C" {
    pub fn nvmet_passthru_ctrl_enable(subsys: *mut nvmet_subsys) -> c_int;
}
extern "C" {
    pub fn nvmet_passthru_ctrl_disable(subsys: *mut nvmet_subsys);
}
extern "C" {
    pub fn nvmet_parse_passthru_admin_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_parse_passthru_io_cmd(req: *mut nvmet_req) -> u16;
}

extern "C" {
    pub fn nvmet_is_passthru_subsys(_arg: nvmet_req_subsys(req)) -> return;
}
extern "C" {
    pub fn nvmet_passthrough_override_cap(ctrl: *mut nvmet_ctrl);
}
extern "C" {
    pub fn errno_to_nvme_status(req: *mut nvmet_req, errno: c_int) -> u16;
}
extern "C" {
    pub fn nvmet_report_invalid_opcode(req: *mut nvmet_req) -> u16;
}
// Convert a 32-bit number to a 16-bit 0's based number
extern "C" {
    pub fn cpu_to_le16(_arg: clamp(a, _arg: 1U, 1: 1U << 16) -) -> return;
}
extern "C" {
    pub fn cpu_to_le64(SECTOR_SHIFT): sect >> (ns->blksize_shift -) -> return;
}
extern "C" {
    pub fn le64_to_cpu(SECTOR_SHIFT: lba) << (ns->blksize_shift -) -> return;
}

extern "C" {
    pub fn nvmet_auth_send_data_len(req: *mut nvmet_req) -> u32;
}
extern "C" {
    pub fn nvmet_execute_auth_send(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_auth_receive_data_len(req: *mut nvmet_req) -> u32;
}
extern "C" {
    pub fn nvmet_execute_auth_receive(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_auth_set_host_hash(host: *mut nvmet_host, hash: *const c_char) -> c_int;
}
extern "C" {
    pub fn nvmet_setup_auth(ctrl: *mut nvmet_ctrl, sq: *mut nvmet_sq, reset: bool) -> u8;
}
extern "C" {
    pub fn nvmet_auth_sq_init(sq: *mut nvmet_sq);
}
extern "C" {
    pub fn nvmet_destroy_auth(ctrl: *mut nvmet_ctrl);
}
extern "C" {
    pub fn nvmet_auth_sq_free(sq: *mut nvmet_sq);
}
extern "C" {
    pub fn nvmet_auth_sq_destroy(sq: *mut nvmet_sq);
}
extern "C" {
    pub fn nvmet_setup_dhgroup(ctrl: *mut nvmet_ctrl, dhgroup_id: u8) -> c_int;
}
extern "C" {
    pub fn nvmet_check_auth_status(req: *mut nvmet_req) -> bool;
}
extern "C" {
    pub fn nvmet_auth_insert_psk(sq: *mut nvmet_sq);
}

extern "C" {
    pub fn nvmet_pr_init_ns(ns: *mut nvmet_ns) -> c_int;
}
extern "C" {
    pub fn nvmet_parse_pr_cmd(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_pr_check_cmd_access(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_ctrl_init_pr(ctrl: *mut nvmet_ctrl) -> c_int;
}
extern "C" {
    pub fn nvmet_ctrl_destroy_pr(ctrl: *mut nvmet_ctrl);
}
extern "C" {
    pub fn nvmet_pr_exit_ns(ns: *mut nvmet_ns);
}
extern "C" {
    pub fn nvmet_execute_get_log_page_resv(req: *mut nvmet_req);
}
extern "C" {
    pub fn nvmet_set_feat_resv_notif_mask(req: *mut nvmet_req, mask: u32) -> u16;
}
extern "C" {
    pub fn nvmet_get_feat_resv_notif_mask(req: *mut nvmet_req) -> u16;
}
extern "C" {
    pub fn nvmet_pr_get_ns_pc_ref(req: *mut nvmet_req) -> u16;
}
//
// Data for the get_feature() and set_feature() operations of PCI target
// controllers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_feat_irq_coalesce {
    pub thr: u8,
    pub time: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_feat_irq_config {
    pub iv: u16,
    pub cd: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmet_feat_arbitration {
    pub hpw: u8,
    pub mpw: u8,
    pub lpw: u8,
    pub ab: u8,
}
