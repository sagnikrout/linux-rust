//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_nl_gen.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)

// Common nested types
// Ops table for drbd
extern "C" {
    pub fn drbd_adm_dump_devices_done(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn drbd_adm_dump_connections_done(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn drbd_adm_dump_peer_devices_done(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn drbd_nl_get_status_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_get_status_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn drbd_nl_new_minor_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_del_minor_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_new_resource_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_del_resource_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_resource_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_connect_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_disconnect_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_attach_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_resize_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_primary_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_secondary_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_new_c_uuid_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_start_ov_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_detach_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_invalidate_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_inval_peer_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_pause_sync_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_resume_sync_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_suspend_io_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_resume_io_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_outdate_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_get_timeout_type_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_down_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_chg_disk_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_nl_chg_net_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_cfg_reply {
    pub info_text: [c_char; 0],
    pub info_text_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_cfg_context {
    pub ctx_volume: __u32,
    pub ctx_resource_name: [c_char; 128],
    pub ctx_resource_name_len: __u32,
    pub ctx_my_addr: [c_char; 128],
    pub ctx_my_addr_len: __u32,
    pub ctx_peer_addr: [c_char; 128],
    pub ctx_peer_addr_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_conf {
    pub backing_dev: [c_char; 128],
    pub backing_dev_len: __u32,
    pub meta_dev: [c_char; 128],
    pub meta_dev_len: __u32,
    pub meta_dev_idx: __s32,
    pub disk_size: __u64,
    pub max_bio_bvecs: __u32,
    pub on_io_error: __u32,
    pub fencing: __u32,
    pub resync_rate: __u32,
    pub resync_after: __s32,
    pub al_extents: __u32,
    pub c_plan_ahead: __u32,
    pub c_delay_target: __u32,
    pub c_fill_target: __u32,
    pub c_max_rate: __u32,
    pub c_min_rate: __u32,
    pub disk_barrier: c_uchar,
    pub disk_flushes: c_uchar,
    pub disk_drain: c_uchar,
    pub md_flushes: c_uchar,
    pub disk_timeout: __u32,
    pub read_balancing: __u32,
    pub al_updates: c_uchar,
    pub discard_zeroes_if_aligned: c_uchar,
    pub rs_discard_granularity: __u32,
    pub disable_write_same: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct res_opts {
    pub cpu_mask: [c_char; DRBD_CPU_MASK_SIZE],
    pub cpu_mask_len: __u32,
    pub on_no_data: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_conf {
    pub shared_secret: [c_char; SHARED_SECRET_MAX],
    pub shared_secret_len: __u32,
    pub cram_hmac_alg: [c_char; SHARED_SECRET_MAX],
    pub cram_hmac_alg_len: __u32,
    pub integrity_alg: [c_char; SHARED_SECRET_MAX],
    pub integrity_alg_len: __u32,
    pub verify_alg: [c_char; SHARED_SECRET_MAX],
    pub verify_alg_len: __u32,
    pub csums_alg: [c_char; SHARED_SECRET_MAX],
    pub csums_alg_len: __u32,
    pub wire_protocol: __u32,
    pub connect_int: __u32,
    pub timeout: __u32,
    pub ping_int: __u32,
    pub ping_timeo: __u32,
    pub sndbuf_size: __u32,
    pub rcvbuf_size: __u32,
    pub ko_count: __u32,
    pub max_buffers: __u32,
    pub max_epoch_size: __u32,
    pub unplug_watermark: __u32,
    pub after_sb_0p: __u32,
    pub after_sb_1p: __u32,
    pub after_sb_2p: __u32,
    pub rr_conflict: __u32,
    pub on_congestion: __u32,
    pub cong_fill: __u32,
    pub cong_extents: __u32,
    pub two_primaries: c_uchar,
    pub discard_my_data: c_uchar,
    pub tcp_cork: c_uchar,
    pub always_asbp: c_uchar,
    pub tentative: c_uchar,
    pub use_rle: c_uchar,
    pub csums_after_crash_only: c_uchar,
    pub sock_check_timeo: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_role_parms {
    pub assume_uptodate: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resize_parms {
    pub resize_size: __u64,
    pub resize_force: c_uchar,
    pub no_resync: c_uchar,
    pub al_stripes: __u32,
    pub al_stripe_size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct state_info {
    pub sib_reason: __u32,
    pub current_state: __u32,
    pub capacity: __u64,
    pub ed_uuid: __u64,
    pub prev_state: __u32,
    pub new_state: __u32,
    pub uuids: [c_char; DRBD_NL_UUIDS_SIZE],
    pub uuids_len: __u32,
    pub disk_flags: __u32,
    pub bits_total: __u64,
    pub bits_oos: __u64,
    pub bits_rs_total: __u64,
    pub bits_rs_failed: __u64,
    pub helper: [c_char; 32],
    pub helper_len: __u32,
    pub helper_exit_code: __u32,
    pub send_cnt: __u64,
    pub recv_cnt: __u64,
    pub read_cnt: __u64,
    pub writ_cnt: __u64,
    pub al_writ_cnt: __u64,
    pub bm_writ_cnt: __u64,
    pub ap_bio_cnt: __u32,
    pub ap_pending_cnt: __u32,
    pub rs_pending_cnt: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_ov_parms {
    pub ov_start_sector: __u64,
    pub ov_stop_sector: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct new_c_uuid_parms {
    pub clear_bm: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeout_parms {
    pub timeout_type: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disconnect_parms {
    pub force_disconnect: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct detach_parms {
    pub force_detach: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_info {
    pub res_role: __u32,
    pub res_susp: c_uchar,
    pub res_susp_nod: c_uchar,
    pub res_susp_fen: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_info {
    pub dev_disk_state: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct connection_info {
    pub conn_connection_state: __u32,
    pub conn_role: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct peer_device_info {
    pub peer_repl_state: __u32,
    pub peer_disk_state: __u32,
    pub peer_resync_susp_user: __u32,
    pub peer_resync_susp_peer: __u32,
    pub peer_resync_susp_dependency: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_statistics {
    pub res_stat_write_ordering: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_statistics {
    pub dev_size: __u64,
    pub dev_read: __u64,
    pub dev_write: __u64,
    pub dev_al_writes: __u64,
    pub dev_bm_writes: __u64,
    pub dev_upper_pending: __u32,
    pub dev_lower_pending: __u32,
    pub dev_upper_blocked: c_uchar,
    pub dev_lower_blocked: c_uchar,
    pub dev_al_suspended: c_uchar,
    pub dev_exposed_data_uuid: __u64,
    pub dev_current_uuid: __u64,
    pub dev_disk_flags: __u32,
    pub history_uuids: [c_char; DRBD_NL_HISTORY_UUIDS_SIZE],
    pub history_uuids_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct connection_statistics {
    pub conn_congested: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct peer_device_statistics {
    pub peer_dev_received: __u64,
    pub peer_dev_sent: __u64,
    pub peer_dev_pending: __u32,
    pub peer_dev_unacked: __u32,
    pub peer_dev_out_of_sync: __u64,
    pub peer_dev_resync_failed: __u64,
    pub peer_dev_bitmap_uuid: __u64,
    pub peer_dev_flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_notification_header {
    pub nh_type: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_helper_info {
    pub helper_name: [c_char; 32],
    pub helper_name_len: __u32,
    pub helper_status: __u32,
}

extern "C" {
    pub fn drbd_cfg_reply_to_skb(skb: *mut sk_buff, s: *mut drbd_cfg_reply) -> c_int;
}
extern "C" {
    pub fn drbd_cfg_context_from_attrs(s: *mut drbd_cfg_context, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_cfg_context_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn drbd_cfg_context_to_skb(skb: *mut sk_buff, s: *mut drbd_cfg_context) -> c_int;
}
extern "C" {
    pub fn disk_conf_from_attrs(s: *mut disk_conf, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn disk_conf_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn disk_conf_to_skb(skb: *mut sk_buff, s: *mut disk_conf) -> c_int;
}
extern "C" {
    pub fn set_disk_conf_defaults(x: *mut disk_conf);
}
extern "C" {
    pub fn res_opts_from_attrs(s: *mut res_opts, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn res_opts_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn res_opts_to_skb(skb: *mut sk_buff, s: *mut res_opts) -> c_int;
}
extern "C" {
    pub fn set_res_opts_defaults(x: *mut res_opts);
}
extern "C" {
    pub fn net_conf_from_attrs(s: *mut net_conf, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn net_conf_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn net_conf_to_skb(skb: *mut sk_buff, s: *mut net_conf) -> c_int;
}
extern "C" {
    pub fn set_net_conf_defaults(x: *mut net_conf);
}
extern "C" {
    pub fn set_role_parms_from_attrs(s: *mut set_role_parms, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn set_role_parms_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn set_role_parms_to_skb(skb: *mut sk_buff, s: *mut set_role_parms) -> c_int;
}
extern "C" {
    pub fn resize_parms_from_attrs(s: *mut resize_parms, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn resize_parms_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn resize_parms_to_skb(skb: *mut sk_buff, s: *mut resize_parms) -> c_int;
}
extern "C" {
    pub fn set_resize_parms_defaults(x: *mut resize_parms);
}
extern "C" {
    pub fn state_info_to_skb(skb: *mut sk_buff, s: *mut state_info) -> c_int;
}
extern "C" {
    pub fn start_ov_parms_from_attrs(s: *mut start_ov_parms, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn start_ov_parms_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn start_ov_parms_to_skb(skb: *mut sk_buff, s: *mut start_ov_parms) -> c_int;
}
extern "C" {
    pub fn new_c_uuid_parms_from_attrs(s: *mut new_c_uuid_parms, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn new_c_uuid_parms_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn new_c_uuid_parms_to_skb(skb: *mut sk_buff, s: *mut new_c_uuid_parms) -> c_int;
}
extern "C" {
    pub fn timeout_parms_to_skb(skb: *mut sk_buff, s: *mut timeout_parms) -> c_int;
}
extern "C" {
    pub fn disconnect_parms_from_attrs(s: *mut disconnect_parms, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn disconnect_parms_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn disconnect_parms_to_skb(skb: *mut sk_buff, s: *mut disconnect_parms) -> c_int;
}
extern "C" {
    pub fn detach_parms_from_attrs(s: *mut detach_parms, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn detach_parms_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn detach_parms_to_skb(skb: *mut sk_buff, s: *mut detach_parms) -> c_int;
}
extern "C" {
    pub fn resource_info_from_attrs(s: *mut resource_info, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn resource_info_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn resource_info_to_skb(skb: *mut sk_buff, s: *mut resource_info) -> c_int;
}
extern "C" {
    pub fn device_info_from_attrs(s: *mut device_info, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn device_info_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn device_info_to_skb(skb: *mut sk_buff, s: *mut device_info) -> c_int;
}
extern "C" {
    pub fn connection_info_from_attrs(s: *mut connection_info, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn connection_info_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn connection_info_to_skb(skb: *mut sk_buff, s: *mut connection_info) -> c_int;
}
extern "C" {
    pub fn peer_device_info_from_attrs(s: *mut peer_device_info, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn peer_device_info_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn peer_device_info_to_skb(skb: *mut sk_buff, s: *mut peer_device_info) -> c_int;
}
extern "C" {
    pub fn resource_statistics_from_attrs(s: *mut resource_statistics, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn resource_statistics_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn resource_statistics_to_skb(skb: *mut sk_buff, s: *mut resource_statistics) -> c_int;
}
extern "C" {
    pub fn device_statistics_from_attrs(s: *mut device_statistics, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn device_statistics_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn device_statistics_to_skb(skb: *mut sk_buff, s: *mut device_statistics) -> c_int;
}
extern "C" {
    pub fn connection_statistics_from_attrs(s: *mut connection_statistics, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn connection_statistics_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn connection_statistics_to_skb(skb: *mut sk_buff, s: *mut connection_statistics) -> c_int;
}
extern "C" {
    pub fn peer_device_statistics_from_attrs(s: *mut peer_device_statistics, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn peer_device_statistics_ntb_from_attrs(ret_nested_attribute_table: *mut nlattr, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn peer_device_statistics_to_skb(skb: *mut sk_buff, s: *mut peer_device_statistics) -> c_int;
}
extern "C" {
    pub fn drbd_notification_header_to_skb(skb: *mut sk_buff, s: *mut drbd_notification_header) -> c_int;
}
extern "C" {
    pub fn drbd_helper_info_to_skb(skb: *mut sk_buff, s: *mut drbd_helper_info) -> c_int;
}
