//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/hci_core.h
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

// HCI priority
pub const HCI_PRIO_MAX: c_int = 7;
// HCI maximum id value
pub const HCI_MAX_ID: c_int = 10000;
// HCI Core structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inquiry_data {
    pub bdaddr: bdaddr_t,
    pub pscan_rep_mode: __u8,
    pub pscan_period_mode: __u8,
    pub pscan_mode: __u8,
    pub dev_class: [__u8; 3],
    pub clock_offset: __le16,
    pub rssi: __s8,
    pub ssp_mode: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inquiry_entry {
    pub /: *mut *mut list_head all; / inq_cache.all,
    pub /: *mut *mut list_head list; / unknown or resolve,
    pub name_state: },
    pub timestamp: __u32,
    pub data: inquiry_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct discovery_state {
    pub type: c_int,
    pub state: },
    pub /: *mut *mut list_head all; / All devices found during inquiry,
    pub /: *mut *mut list_head unknown; / Name state not known,
    pub /: *mut *mut list_head resolve; / Name needs to be resolved,
    pub timestamp: __u32,
    pub last_adv_addr: bdaddr_t,
    pub last_adv_addr_type: u8,
    pub last_adv_rssi: i8,
    pub last_adv_flags: u32,
    pub last_adv_data: [u8; HCI_MAX_EXT_AD_LENGTH],
    pub last_adv_data_len: u8,
    pub report_invalid_rssi: bool,
    pub result_filtering: bool,
    pub limited: bool,
    pub rssi: i8,
    pub uuid_count: u16,
    pub (*uuids)[16]: *mut u8,
    pub name_resolve_timeout: c_ulong,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum suspend_tasks {
    SUSPEND_PAUSE_DISCOVERY,
    SUSPEND_UNPAUSE_DISCOVERY,

    SUSPEND_PAUSE_ADVERTISING,
    SUSPEND_UNPAUSE_ADVERTISING,

    SUSPEND_SCAN_DISABLE,
    SUSPEND_SCAN_ENABLE,
    SUSPEND_DISCONNECTING,

    SUSPEND_POWERING_DOWN,

    SUSPEND_PREPARE_NOTIFIER,

    SUSPEND_SET_ADV_FILTER,
    __SUSPEND_NUM_TASKS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum suspended_state {
    BT_RUNNING = 0,
    BT_SUSPEND_DISCONNECT,
    BT_SUSPEND_CONFIGURE_WAKE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_conn_hash {
    pub list: list_head,
    pub acl_num: c_uint,
    pub sco_num: c_uint,
    pub cis_num: c_uint,
    pub bis_num: c_uint,
    pub pa_num: c_uint,
    pub le_num: c_uint,
    pub le_num_peripheral: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdaddr_list {
    pub list: list_head,
    pub bdaddr: bdaddr_t,
    pub bdaddr_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codec_list {
    pub list: list_head,
    pub id: u8,
    pub cid: __u16,
    pub vid: __u16,
    pub transport: u8,
    pub num_caps: u8,
    pub len: u32,
    pub caps: [hci_codec_caps; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdaddr_list_with_irk {
    pub list: list_head,
    pub bdaddr: bdaddr_t,
    pub bdaddr_type: u8,
    pub peer_irk: [u8; 16],
    pub local_irk: [u8; 16],
}

// Bitmask of connection flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hci_conn_flags {
    HCI_CONN_FLAG_REMOTE_WAKEUP = BIT(0),
    HCI_CONN_FLAG_DEVICE_PRIVACY = BIT(1),
    HCI_CONN_FLAG_ADDRESS_RESOLUTION = BIT(2),
    HCI_CONN_FLAG_PAST = BIT(3),
}

pub type hci_conn_flags_t = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdaddr_list_with_flags {
    pub list: list_head,
    pub bdaddr: bdaddr_t,
    pub bdaddr_type: u8,
    pub flags: hci_conn_flags_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_uuid {
    pub list: list_head,
    pub uuid: [u8; 16],
    pub size: u8,
    pub svc_hint: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blocked_key {
    pub list: list_head,
    pub rcu: rcu_head,
    pub type: u8,
    pub val: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_csrk {
    pub bdaddr: bdaddr_t,
    pub bdaddr_type: u8,
    pub type: u8,
    pub val: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_ltk {
    pub list: list_head,
    pub rcu: rcu_head,
    pub bdaddr: bdaddr_t,
    pub bdaddr_type: u8,
    pub authenticated: u8,
    pub type: u8,
    pub enc_size: u8,
    pub ediv: __le16,
    pub rand: __le64,
    pub val: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_irk {
    pub list: list_head,
    pub rcu: rcu_head,
    pub rpa: bdaddr_t,
    pub bdaddr: bdaddr_t,
    pub addr_type: u8,
    pub val: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_key {
    pub list: list_head,
    pub rcu: rcu_head,
    pub bdaddr: bdaddr_t,
    pub type: u8,
    pub val: [u8; HCI_LINK_KEY_SIZE],
    pub pin_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oob_data {
    pub list: list_head,
    pub bdaddr: bdaddr_t,
    pub bdaddr_type: u8,
    pub present: u8,
    pub hash192: [u8; 16],
    pub rand192: [u8; 16],
    pub hash256: [u8; 16],
    pub rand256: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv_info {
    pub list: list_head,
    pub enabled: bool,
    pub pending: bool,
    pub periodic: bool,
    pub periodic_enabled: bool,
    pub mesh: __u8,
    pub instance: __u8,
    pub handle: __u8,
    pub sid: __u8,
    pub flags: __u32,
    pub timeout: __u16,
    pub remaining_time: __u16,
    pub duration: __u16,
    pub adv_data_len: __u16,
    pub adv_data: [__u8; HCI_MAX_EXT_AD_LENGTH],
    pub adv_data_changed: bool,
    pub scan_rsp_len: __u16,
    pub scan_rsp_data: [__u8; HCI_MAX_EXT_AD_LENGTH],
    pub scan_rsp_changed: bool,
    pub per_adv_data_len: __u16,
    pub per_adv_data: [__u8; HCI_MAX_PER_AD_LENGTH],
    pub tx_power: __s8,
    pub min_interval: __u32,
    pub max_interval: __u32,
    pub random_addr: bdaddr_t,
    pub rpa_expired: bool,
    pub rpa_expired_cb: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_queue {
    pub queue: sk_buff_head,
    pub extra: c_uint,
    pub tracked: c_uint,
}

pub const HCI_MAX_ADV_INSTANCES: c_int = 5;
pub const HCI_DEFAULT_ADV_DURATION: c_int = 2;
pub const HCI_ADV_TX_POWER_NO_PREFERENCE: c_uint = 0x7F;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct monitored_device {
    pub list: list_head,
    pub bdaddr: bdaddr_t,
    pub addr_type: __u8,
    pub handle: __u16,
    pub notified: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv_pattern {
    pub list: list_head,
    pub ad_type: __u8,
    pub offset: __u8,
    pub length: __u8,
    pub value: [__u8; HCI_MAX_EXT_AD_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv_rssi_thresholds {
    pub low_threshold: __s8,
    pub high_threshold: __s8,
    pub low_threshold_timeout: __u16,
    pub high_threshold_timeout: __u16,
    pub sampling_period: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv_monitor {
    pub patterns: list_head,
    pub rssi: adv_rssi_thresholds,
    pub handle: __u16,
    pub state: },
}

pub const HCI_MIN_ADV_MONITOR_HANDLE: c_int = 1;
pub const HCI_MAX_ADV_MONITOR_NUM_HANDLES: c_int = 32;
pub const HCI_MAX_ADV_MONITOR_NUM_PATTERNS: c_int = 16;
pub const HCI_ADV_MONITOR_EXT_NONE: c_int = 1;
pub const HCI_ADV_MONITOR_EXT_MSFT: c_int = 2;
pub const HCI_MAX_SHORT_NAME_LENGTH: c_int = 10;
pub const HCI_CONN_HANDLE_MAX: c_uint = 0x0eff;

// Min encryption key size to match with SMP
pub const HCI_MIN_ENC_KEY_SIZE: c_int = 7;
// Default LE RPA expiry time, 15 minutes

// Default min/max age of connection information (1s/3s)
pub const DEFAULT_CONN_INFO_MIN_AGE: c_int = 1000;
pub const DEFAULT_CONN_INFO_MAX_AGE: c_int = 3000;
// Default authenticated payload timeout 30s
pub const DEFAULT_AUTH_PAYLOAD_TIMEOUT: c_uint = 0x0bb8;
pub const HCI_MAX_PAGES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_dev {
    pub list: list_head,
    pub srcu: srcu_struct,
    pub lock: mutex,
    pub unset_handle_ida: ida,
    pub name: *const c_char,
    pub flags: c_ulong,
    pub id: __u16,
    pub bus: __u8,
    pub bdaddr: bdaddr_t,
    pub setup_addr: bdaddr_t,
    pub public_addr: bdaddr_t,
    pub random_addr: bdaddr_t,
    pub static_addr: bdaddr_t,
    pub adv_addr_type: __u8,
    pub dev_name: [__u8; HCI_MAX_NAME_LENGTH],
    pub short_name: [__u8; HCI_MAX_SHORT_NAME_LENGTH],
    pub eir: [__u8; HCI_MAX_EIR_LENGTH],
    pub appearance: __u16,
    pub dev_class: [__u8; 3],
    pub major_class: __u8,
    pub minor_class: __u8,
    pub max_page: __u8,
    pub features: [__u8; HCI_MAX_PAGES][8],
    pub le_features: [__u8; 248],
    pub le_accept_list_size: __u8,
    pub le_resolv_list_size: __u8,
    pub le_num_of_adv_sets: __u8,
    pub le_states: [__u8; 8],
    pub mesh_ad_types: [__u8; 16],
    pub mesh_send_ref: __u8,
    pub commands: [__u8; 64],
    pub hci_ver: __u8,
    pub hci_rev: __u16,
    pub lmp_ver: __u8,
    pub manufacturer: __u16,
    pub lmp_subver: __u16,
    pub voice_setting: __u16,
    pub num_iac: __u8,
    pub stored_max_keys: __u16,
    pub stored_num_keys: __u16,
    pub io_capability: __u8,
    pub inq_tx_power: __s8,
    pub err_data_reporting: __u8,
    pub page_scan_interval: __u16,
    pub page_scan_window: __u16,
    pub page_scan_type: __u8,
    pub le_adv_channel_map: __u8,
    pub le_adv_min_interval: __u16,
    pub le_adv_max_interval: __u16,
    pub le_scan_type: __u8,
    pub le_scan_interval: __u16,
    pub le_scan_window: __u16,
    pub le_scan_int_suspend: __u16,
    pub le_scan_window_suspend: __u16,
    pub le_scan_int_discovery: __u16,
    pub le_scan_window_discovery: __u16,
    pub le_scan_int_adv_monitor: __u16,
    pub le_scan_window_adv_monitor: __u16,
    pub le_scan_int_connect: __u16,
    pub le_scan_window_connect: __u16,
    pub le_conn_min_interval: __u16,
    pub le_conn_max_interval: __u16,
    pub le_conn_latency: __u16,
    pub le_supv_timeout: __u16,
    pub le_min_rate_interval: __u16,
    pub le_def_tx_len: __u16,
    pub le_def_tx_time: __u16,
    pub le_max_tx_len: __u16,
    pub le_max_tx_time: __u16,
    pub le_max_rx_len: __u16,
    pub le_max_rx_time: __u16,
    pub le_max_key_size: __u8,
    pub le_min_key_size: __u8,
    pub discov_interleaved_timeout: __u16,
    pub conn_info_min_age: __u16,
    pub conn_info_max_age: __u16,
    pub auth_payload_timeout: __u16,
    pub min_enc_key_size: __u8,
    pub max_enc_key_size: __u8,
    pub pairing_opts: __u8,
    pub ssp_debug_mode: __u8,
    pub hw_error_code: __u8,
    pub clock: __u32,
    pub advmon_allowlist_duration: __u16,
    pub advmon_no_filter_duration: __u16,
    pub enable_advmon_interleave_scan: __u8,
    pub devid_source: __u16,
    pub devid_vendor: __u16,
    pub devid_product: __u16,
    pub devid_version: __u16,
    pub def_page_scan_type: __u8,
    pub def_page_scan_int: __u16,
    pub def_page_scan_window: __u16,
    pub def_inq_scan_type: __u8,
    pub def_inq_scan_int: __u16,
    pub def_inq_scan_window: __u16,
    pub def_br_lsto: __u16,
    pub def_page_timeout: __u16,
    pub def_multi_adv_rotation_duration: __u16,
    pub def_le_autoconnect_timeout: __u16,
    pub min_le_tx_power: __s8,
    pub max_le_tx_power: __s8,
    pub pkt_type: __u16,
    pub esco_type: __u16,
    pub link_policy: __u16,
    pub link_mode: __u16,
    pub idle_timeout: __u32,
    pub sniff_min_interval: __u16,
    pub sniff_max_interval: __u16,
    pub auto_accept_delay: c_uint,
    pub __HCI_NUM_QUIRKS): DECLARE_BITMAP(quirk_flags,,
    pub cmd_cnt: core::sync::atomic::AtomicI32,
    pub acl_cnt: c_uint,
    pub sco_cnt: c_uint,
    pub le_cnt: c_uint,
    pub iso_cnt: c_uint,
    pub acl_mtu: c_uint,
    pub sco_mtu: c_uint,
    pub le_mtu: c_uint,
    pub iso_mtu: c_uint,
    pub acl_pkts: c_uint,
    pub sco_pkts: c_uint,
    pub le_pkts: c_uint,
    pub iso_pkts: c_uint,
    pub acl_last_tx: c_ulong,
    pub le_last_tx: c_ulong,
    pub iso_last_tx: c_ulong,
    pub le_tx_def_phys: __u8,
    pub le_rx_def_phys: __u8,
    pub workqueue: *mut workqueue_struct,
    pub req_workqueue: *mut workqueue_struct,
    pub power_on: work_struct,
    pub power_off: delayed_work,
    pub error_reset: work_struct,
    pub cmd_sync_work: work_struct,
    pub cmd_sync_work_list: list_head,
    pub cmd_sync_work_lock: mutex,
    pub unregister_lock: mutex,
    pub cmd_sync_cancel_work: work_struct,
    pub reenable_adv_work: work_struct,
    pub discov_timeout: __u16,
    pub discov_off: delayed_work,
    pub service_cache: delayed_work,
    pub cmd_timer: delayed_work,
    pub ncmd_timer: delayed_work,
    pub rx_work: work_struct,
    pub cmd_work: work_struct,
    pub tx_work: work_struct,
    pub le_scan_disable: delayed_work,
    pub rx_q: sk_buff_head,
    pub raw_q: sk_buff_head,
    pub cmd_q: sk_buff_head,
    pub sent_cmd: *mut sk_buff,
    pub recv_event: *mut sk_buff,
    pub req_lock: mutex,
    pub req_wait_q: wait_queue_head_t,
    pub req_status: __u32,
    pub req_result: __u32,
    pub req_skb: *mut sk_buff,
    pub req_rsp: *mut sk_buff,
    pub smp_data: *mut c_void,
    pub smp_bredr_data: *mut c_void,
    pub discovery: discovery_state,
    pub discovery_paused: bool,
    pub advertising_old_state: c_int,
    pub advertising_paused: bool,
    pub suspend_notifier: notifier_block,
    pub suspend_state_next: suspended_state,
    pub suspend_state: suspended_state,
    pub scanning_paused: bool,
    pub suspended: bool,
    pub wake_reason: u8,
    pub wake_addr: bdaddr_t,
    pub wake_addr_type: u8,
    pub conn_hash: hci_conn_hash,
    pub mesh_pending: list_head,
    pub mgmt_pending_lock: mutex,
    pub mgmt_pending: list_head,
    pub reject_list: list_head,
    pub accept_list: list_head,
    pub uuids: list_head,
    pub link_keys: list_head,
    pub long_term_keys: list_head,
    pub identity_resolving_keys: list_head,
    pub remote_oob_data: list_head,
    pub le_accept_list: list_head,
    pub le_resolv_list: list_head,
    pub le_conn_params: list_head,
    pub pend_le_conns: list_head,
    pub pend_le_reports: list_head,
    pub blocked_keys: list_head,
    pub local_codecs: list_head,
    pub stat: hci_dev_stats,
    pub promisc: core::sync::atomic::AtomicI32,
    pub hw_info: *const c_char,
    pub fw_info: *const c_char,
    pub debugfs: *mut dentry,
    pub dump: hci_devcoredump,
    pub dev: device,
    pub rfkill: *mut rfkill,
    pub __HCI_NUM_FLAGS): DECLARE_BITMAP(dev_flags,,
    pub conn_flags: hci_conn_flags_t,
    pub adv_tx_power: __s8,
    pub adv_data: [__u8; HCI_MAX_EXT_AD_LENGTH],
    pub adv_data_len: __u8,
    pub scan_rsp_data: [__u8; HCI_MAX_EXT_AD_LENGTH],
    pub scan_rsp_data_len: __u8,
    pub per_adv_data: [__u8; HCI_MAX_PER_AD_LENGTH],
    pub per_adv_data_len: __u8,
    pub adv_instances: list_head,
    pub adv_instance_cnt: c_uint,
    pub cur_adv_instance: __u8,
    pub adv_instance_timeout: __u16,
    pub adv_instance_expire: delayed_work,
    pub adv_monitors_idr: idr,
    pub adv_monitors_cnt: c_uint,
    pub irk: [__u8; 16],
    pub rpa_timeout: __u32,
    pub rpa_expired: delayed_work,
    pub rpa: bdaddr_t,
    pub mesh_send_done: delayed_work,
    pub interleave_scan_state: },
    pub interleave_scan: delayed_work,
    pub monitored_devices: list_head,
    pub advmon_pend_notify: bool,
    pub hci_drv: *mut hci_drv,

    pub power_led: *mut led_trigger,

    pub msft_opcode: __u16,
    pub msft_data: *mut c_void,
    pub msft_curve_validity: bool,

    pub aosp_capable: bool,
    pub aosp_quality_report: bool,

    pub hdev): *mut *mut int (open)(struct hci_dev,
    pub hdev): *mut *mut int (close)(struct hci_dev,
    pub hdev): *mut *mut int (flush)(struct hci_dev,
    pub hdev): *mut *mut int (setup)(struct hci_dev,
    pub hdev): *mut *mut int (shutdown)(struct hci_dev,
    pub skb): *mut *mut *mut int (send)(struct hci_dev hdev, struct sk_buff,
// Handle HCI_EV_VENDOR; return true if handled, false otherwise
    pub skb): *mut *mut *mut bool (handle_ev_vendor)(struct hci_dev hdev, struct sk_buff,
    pub evt): *mut *mut *mut void (notify)(struct hci_dev hdev, unsigned int,
    pub code): *mut *mut *mut void (hw_error)(struct hci_dev hdev, u8,
    pub hdev): *mut *mut int (post_init)(struct hci_dev,
    pub enable): *mut *mut *mut int (set_diag)(struct hci_dev hdev, bool,
    pub bdaddr): *const *const *const int (set_bdaddr)(struct hci_dev hdev, bdaddr_t,
    pub hdev): *mut *mut void (reset)(struct hci_dev,
    pub hdev): *mut *mut bool (wakeup)(struct hci_dev,
    pub enable): *mut *mut *mut int (set_quality_report)(struct hci_dev hdev, bool,
    pub data_path): *mut *mut *mut int (get_data_path_id)(struct hci_dev hdev, __u8,
    pub vnd_data): *mut __u8,
    pub skb): *mut *mut *mut u8 (classify_pkt_type)(struct hci_dev hdev, struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum conn_reasons {
    CONN_REASON_PAIR_DEVICE,
    CONN_REASON_L2CAP_CHAN,
    CONN_REASON_SCO_CONNECT,
    CONN_REASON_ISO_CONNECT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_conn {
    pub list: list_head,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub dst: bdaddr_t,
    pub dst_type: __u8,
    pub src: bdaddr_t,
    pub src_type: __u8,
    pub init_addr: bdaddr_t,
    pub init_addr_type: __u8,
    pub resp_addr: bdaddr_t,
    pub resp_addr_type: __u8,
    pub adv_instance: __u8,
    pub handle: __u16,
    pub sync_handle: __u16,
    pub sid: __u8,
    pub state: __u16,
    pub mtu: __u16,
    pub mode: __u8,
    pub type: __u8,
    pub role: __u8,
    pub out: bool,
    pub attempt: __u8,
    pub dev_class: [__u8; 3],
    pub features: [__u8; HCI_MAX_PAGES][8],
    pub le_features: [__u8; 248],
    pub pkt_type: __u16,
    pub link_policy: __u16,
    pub key_type: __u8,
    pub auth_type: __u8,
    pub sec_level: __u8,
    pub pending_sec_level: __u8,
    pub pin_length: __u8,
    pub enc_key_size: __u8,
    pub io_capability: __u8,
    pub passkey_notify: __u32,
    pub passkey_entered: __u8,
    pub disc_timeout: __u16,
    pub conn_timeout: __u16,
    pub setting: __u16,
    pub auth_payload_timeout: __u16,
    pub le_conn_min_interval: __u16,
    pub le_conn_max_interval: __u16,
    pub le_conn_interval: __u16,
    pub le_conn_latency: __u16,
    pub le_supv_timeout: __u16,
    pub le_rate_interval: __u16,
    pub le_subrate: __u16,
    pub le_rate_latency: __u16,
    pub le_cont_num: __u16,
    pub le_rate_supv_timeout: __u16,
    pub le_adv_data: [__u8; HCI_MAX_EXT_AD_LENGTH],
    pub le_adv_data_len: __u8,
    pub le_per_adv_data: [__u8; HCI_MAX_PER_AD_TOT_LEN],
    pub le_per_adv_data_len: __u16,
    pub le_per_adv_data_offset: __u16,
    pub le_adv_phy: __u8,
    pub le_adv_sec_phy: __u8,
    pub le_tx_def_phys: __u8,
    pub le_rx_def_phys: __u8,
    pub le_tx_phy: __u8,
    pub le_rx_phy: __u8,
    pub rssi: __s8,
    pub tx_power: __s8,
    pub max_tx_power: __s8,
    pub iso_qos: bt_iso_qos,
    pub num_bis: __u8,
    pub bis: [__u8; HCI_MAX_ISO_BIS],
    pub flags: c_ulong,
    pub conn_reason: conn_reasons,
    pub abort_reason: __u8,
    pub clock: __u32,
    pub clock_accuracy: __u16,
    pub conn_info_timestamp: c_ulong,
    pub remote_cap: __u8,
    pub remote_auth: __u8,
    pub sent: c_uint,
    pub data_q: sk_buff_head,
    pub chan_list: list_head,
    pub tx_q: tx_queue,
    pub disc_work: delayed_work,
    pub auto_accept_work: delayed_work,
    pub idle_work: delayed_work,
    pub le_conn_timeout: delayed_work,
    pub dev: device,
    pub debugfs: *mut dentry,
    pub hdev: *mut hci_dev,
    pub /: *mut *mut spinlock_t proto_lock; / lock guarding protocol data,
    pub &hdev->lock): *mut *mut void l2cap_data __guarded_by(&proto_lock,,
    pub sco_data: *mut c_void,
    pub __guarded_by(&proto_lock): *mut *mut void iso_data,
    pub link_list: list_head,
    pub parent: *mut hci_conn,
    pub link: *mut hci_link,
    pub codec: bt_codec,
    pub status): *mut *mut *mut void (connect_cfm_cb) (struct hci_conn conn, u8,
    pub status): *mut *mut *mut void (security_cfm_cb) (struct hci_conn conn, u8,
    pub reason): *mut *mut *mut void (disconn_cfm_cb) (struct hci_conn conn, u8,
    pub conn): *mut *mut void (cleanup)(struct hci_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_link {
    pub list: list_head,
    pub conn: *mut hci_conn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_chan {
    pub list: list_head,
    pub handle: __u16,
    pub conn: *mut hci_conn,
    pub data_q: sk_buff_head,
    pub sent: c_uint,
    pub state: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_conn_params {
    pub list: list_head,
    pub action: list_head,
    pub addr: bdaddr_t,
    pub addr_type: u8,
    pub conn_min_interval: u16,
    pub conn_max_interval: u16,
    pub conn_latency: u16,
    pub supervision_timeout: u16,
    pub rate_min_interval: u16,
    pub rate_max_interval: u16,
    pub subrate_min: u16,
    pub subrate_max: u16,
    pub max_latency: u16,
    pub cont_num: u16,
    pub rate_supv_timeout: u16,
    pub auto_connect: },
    pub conn: *mut hci_conn,
    pub explicit_connect: bool,
// Accessed without hdev->lock:
    pub flags: hci_conn_flags_t,
    pub privacy_mode: u8,
}

// ----- HCI interface to upper protocols -----
extern "C" {
    pub fn l2cap_connect_ind(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t) -> c_int;
}
extern "C" {
    pub fn l2cap_disconn_ind(hcon: *mut hci_conn) -> c_int;
}

extern "C" {
    pub fn sco_connect_ind(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t, flags: *mut __u8) -> c_int;
}
extern "C" {
    pub fn sco_recv_scodata(hdev: *mut hci_dev, handle: u16, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn iso_connect_ind(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t, flags: *mut __u8) -> c_int;
}

// ----- Inquiry cache -----

extern "C" {
    pub fn hci_discovery_active(hdev: *mut hci_dev) -> bool;
}
extern "C" {
    pub fn hci_discovery_set_state(hdev: *mut hci_dev, state: c_int);
}
extern "C" {
    pub fn list_empty(_arg: &hdev->discovery.all) -> return;
}
extern "C" {
    pub fn hci_inquiry_cache_flush(hdev: *mut hci_dev);
}
// ----- HCI Connections -----
// Match CIG ID if set
// Match CIS ID if set
// Match destination address if set
// Ignore the listen hcon, we are looking
// for the child hcon that was created as
// a result of the PA sync established event.
//
extern "C" {
    pub fn void(conn: *mut *mut hci_conn_func_t)(struct hci_conn, data: *mut c_void) -> typedef;
}
// Returns true if an le connection is in the scanning state
extern "C" {
    pub fn hci_disconnect(conn: *mut hci_conn, reason: __u8) -> c_int;
}
extern "C" {
    pub fn hci_setup_sync(conn: *mut hci_conn, handle: __u16) -> bool;
}
extern "C" {
    pub fn hci_sco_setup(conn: *mut hci_conn, status: __u8);
}
extern "C" {
    pub fn hci_iso_setup_path(conn: *mut hci_conn) -> bool;
}
extern "C" {
    pub fn hci_le_create_cis_pending(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_conn_check_create_cis(conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_conn_del(conn: *mut hci_conn);
}
extern "C" {
    pub fn hci_conn_hash_flush(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_chan_del(chan: *mut hci_chan);
}
extern "C" {
    pub fn hci_chan_list_flush(conn: *mut hci_conn);
}
extern "C" {
    pub fn hci_connect_le_scan_cleanup(conn: *mut hci_conn, status: u8);
}
extern "C" {
    pub fn hci_past_bis(conn: *mut hci_conn, dst: *mut bdaddr_t, dst_type: __u8) -> c_int;
}
extern "C" {
    pub fn hci_conn_check_link_mode(conn: *mut hci_conn) -> c_int;
}
extern "C" {
    pub fn hci_conn_check_secure(conn: *mut hci_conn, sec_level: __u8) -> c_int;
}
extern "C" {
    pub fn hci_conn_switch_role(conn: *mut hci_conn, role: __u8) -> c_int;
}
extern "C" {
    pub fn hci_conn_enter_active_mode(conn: *mut hci_conn, force_active: __u8);
}
extern "C" {
    pub fn hci_conn_failed(conn: *mut hci_conn, status: u8);
}
extern "C" {
    pub fn hci_conn_set_handle(conn: *mut hci_conn, handle: u16) -> u8;
}
extern "C" {
    pub fn hci_conn_tx_queue(conn: *mut hci_conn, skb: *mut sk_buff);
}
extern "C" {
    pub fn hci_conn_tx_dequeue(conn: *mut hci_conn);
}
// sockc = (struct sockcm_cookie) {
//
// hci_conn_get() and hci_conn_put() are used to control the life-time of an
// "hci_conn" object. They do not guarantee that the hci_conn object is running,
// working or anything else. They just guarantee that the object is available
// and can be dereferenced. So you can use its locks, local variables and any
// other constant data.
// Before accessing runtime data, you _must_ lock the object and then check that
// it is still running. As soon as you release the locks, the connection might
// get dropped, though.
//
// On the other hand, hci_conn_hold() and hci_conn_drop() are used to control
// how long the underlying connection is held. So every channel that runs on the
// hci_conn object calls this to prevent the connection from disappearing. As
// long as you hold a device, you must also guarantee that you have a valid
// reference to the device via hci_conn_get() (or the initial reference from
// hci_conn_add()).
// The hold()/drop() ref-count is known to drop below 0 sometimes, which doesn't
// break because nobody cares for that. But this means, we cannot use
// _get()/_drop() in it, but require the caller to have a valid ref (FIXME).
//
// ----- HCI Devices -----

extern "C" {
    pub fn dev_get_drvdata(_arg: &hdev->dev) -> return;
}
extern "C" {
    pub fn hci_alloc_dev_priv(_arg: 0) -> return;
}
extern "C" {
    pub fn hci_free_dev(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_register_dev(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_unregister_dev(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_release_dev(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_register_suspend_notifier(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_unregister_suspend_notifier(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_suspend_dev(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_resume_dev(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn __hci_reset_dev(hdev: *mut hci_dev, hw_err_code: u8) -> c_int;
}
extern "C" {
    pub fn __hci_reset_dev(_arg: hdev, _arg: 0) -> return;
}
extern "C" {
    pub fn hci_recv_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn hci_recv_diag(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
}

extern "C" {
    pub fn hci_dev_open(dev: __u16) -> c_int;
}
extern "C" {
    pub fn hci_dev_close(dev: __u16) -> c_int;
}
extern "C" {
    pub fn hci_dev_do_close(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_dev_reset(dev: __u16) -> c_int;
}
extern "C" {
    pub fn hci_dev_reset_stat(dev: __u16) -> c_int;
}
extern "C" {
    pub fn hci_dev_cmd(cmd: c_uint, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn hci_get_dev_list(arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn hci_get_dev_info(arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn hci_get_conn_list(arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn hci_get_conn_info(hdev: *mut hci_dev, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn hci_get_auth_info(hdev: *mut hci_dev, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn hci_inquiry(arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn hci_bdaddr_list_add(list: *mut list_head, bdaddr: *mut bdaddr_t, type: u8) -> c_int;
}
extern "C" {
    pub fn hci_bdaddr_list_del(list: *mut list_head, bdaddr: *mut bdaddr_t, type: u8) -> c_int;
}
extern "C" {
    pub fn hci_bdaddr_list_clear(list: *mut list_head);
}
extern "C" {
    pub fn hci_conn_params_del(hdev: *mut hci_dev, addr: *mut bdaddr_t, addr_type: u8);
}
extern "C" {
    pub fn hci_conn_params_clear_disabled(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_conn_params_free(param: *mut hci_conn_params);
}
extern "C" {
    pub fn hci_pend_le_list_del_init(param: *mut hci_conn_params);
}
extern "C" {
    pub fn hci_uuids_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_link_keys_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_remove_ltk(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t, bdaddr_type: u8) -> c_int;
}
extern "C" {
    pub fn hci_smp_ltks_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_remove_link_key(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t) -> c_int;
}
extern "C" {
    pub fn hci_remove_irk(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t, addr_type: u8);
}
extern "C" {
    pub fn hci_is_blocked_key(hdev: *mut hci_dev, type: u8, val[16]: u8) -> bool;
}
extern "C" {
    pub fn hci_blocked_keys_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_smp_irks_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_bdaddr_is_paired(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t, type: u8) -> bool;
}
extern "C" {
    pub fn hci_remote_oob_data_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_adv_instances_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_remove_adv_instance(hdev: *mut hci_dev, instance: u8) -> c_int;
}
extern "C" {
    pub fn hci_adv_instances_set_rpa_expired(hdev: *mut hci_dev, rpa_expired: bool);
}
extern "C" {
    pub fn hci_adv_instance_flags(hdev: *mut hci_dev, instance: u8) -> u32;
}
extern "C" {
    pub fn hci_adv_instance_is_scannable(hdev: *mut hci_dev, instance: u8) -> bool;
}
extern "C" {
    pub fn hci_adv_monitors_clear(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_free_adv_monitor(hdev: *mut hci_dev, monitor: *mut adv_monitor);
}
extern "C" {
    pub fn hci_add_adv_monitor(hdev: *mut hci_dev, monitor: *mut adv_monitor) -> c_int;
}
extern "C" {
    pub fn hci_remove_single_adv_monitor(hdev: *mut hci_dev, handle: u16) -> c_int;
}
extern "C" {
    pub fn hci_remove_all_adv_monitor(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_is_adv_monitoring(hdev: *mut hci_dev) -> bool;
}
extern "C" {
    pub fn hci_get_adv_monitor_offload_ext(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn hci_event_packet(hdev: *mut hci_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn hci_init_sysfs(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_conn_init_sysfs(conn: *mut hci_conn);
}
extern "C" {
    pub fn hci_conn_add_sysfs(conn: *mut hci_conn);
}
extern "C" {
    pub fn hci_conn_del_sysfs(conn: *mut hci_conn);
}

// ----- LMP capabilities -----

// ----- Extended LMP capabilities -----

// ----- Host capabilities -----

// Use enhanced synchronous connection if command is supported and its quirk
// has not been set.
//

// Use ext scanning if set ext scan param and ext scan enable is supported

// Use ext create connection if command is supported

// Extended advertising support

// Maximum advertising length

// BLUETOOTH CORE SPECIFICATION Version 5.3 | Vol 4, Part E page 1789:
//
// C24: Mandatory if the LE Controller supports Connection State and either
// LE Feature (LL Privacy) or LE Feature (Extended Advertising) is supported
//

// Periodic advertising support

// CIS Master/Slave and BIS support

// Channel sounding support

// ----- HCI protocols -----
pub const HCI_PROTO_DEFER: c_uint = 0x01;
extern "C" {
    pub fn l2cap_connect_ind(_arg: hdev, _arg: bdaddr) -> return;
}
extern "C" {
    pub fn sco_connect_ind(_arg: hdev, _arg: bdaddr, _arg: flags) -> return;
}
extern "C" {
    pub fn iso_connect_ind(_arg: hdev, _arg: bdaddr, _arg: flags) -> return;
}
extern "C" {
    pub fn l2cap_disconn_ind(_arg: conn) -> return;
}
// ----- HCI callbacks -----
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_cb {
    pub list: list_head,
    pub name: *mut c_char,
    pub status): *mut *mut *mut void (connect_cfm) (struct hci_conn conn, __u8,
    pub status): *mut *mut *mut void (disconn_cfm) (struct hci_conn conn, __u8,
    pub encrypt): __u8,
    pub status): *mut *mut *mut void (key_change_cfm) (struct hci_conn conn, __u8,
    pub role): *mut *mut *mut void (role_switch_cfm) (struct hci_conn conn, __u8 status, __u8,
}

// Check for Random Static address type
extern "C" {
    pub fn hci_find_irk_by_rpa(_arg: hdev, _arg: bdaddr) -> return;
}
extern "C" {
    pub fn hci_register_cb(hcb: *mut hci_cb) -> c_int;
}
extern "C" {
    pub fn hci_unregister_cb(hcb: *mut hci_cb) -> c_int;
}
extern "C" {
    pub fn hci_send_acl(chan: *mut hci_chan, skb: *mut sk_buff, flags: __u16);
}
extern "C" {
    pub fn hci_send_sco(conn: *mut hci_conn, skb: *mut sk_buff);
}
extern "C" {
    pub fn hci_send_iso(conn: *mut hci_conn, skb: *mut sk_buff);
}
extern "C" {
    pub fn hci_conn_get_phy(conn: *mut hci_conn) -> u32;
}
extern "C" {
    pub fn hci_conn_set_phy(conn: *mut hci_conn, phys: u32) -> c_int;
}
// ----- HCI Sockets -----
extern "C" {
    pub fn hci_send_to_sock(hdev: *mut hci_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn hci_send_to_monitor(hdev: *mut hci_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn hci_sock_dev_event(hdev: *mut hci_dev, event: c_int);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_mgmt_handler {
    pub data_len): u16,
    pub data_len: usize,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_mgmt_chan {
    pub list: list_head,
    pub channel: c_ushort,
    pub handler_count: usize,
    pub handlers: *const hci_mgmt_handler,
    pub hdev): *mut *mut *mut void (hdev_init) (struct sock sk, struct hci_dev,
}

extern "C" {
    pub fn hci_mgmt_chan_register(c: *mut hci_mgmt_chan) -> c_int;
}
extern "C" {
    pub fn hci_mgmt_chan_unregister(c: *mut hci_mgmt_chan);
}
// Management interface

// These LE scan and inquiry parameters were chosen according to LE General
// Discovery Procedure specification.
//
pub const DISCOV_LE_SCAN_WIN: c_uint = 0x0012 /* 11.25 msec */;
pub const DISCOV_LE_SCAN_INT: c_uint = 0x0012 /* 11.25 msec */;
pub const DISCOV_LE_SCAN_INT_FAST: c_uint = 0x0060 /* 60 msec */;
pub const DISCOV_LE_SCAN_WIN_FAST: c_uint = 0x0030 /* 30 msec */;
pub const DISCOV_LE_SCAN_INT_CONN: c_uint = 0x0060 /* 60 msec */;
pub const DISCOV_LE_SCAN_WIN_CONN: c_uint = 0x0060 /* 60 msec */;
pub const DISCOV_LE_SCAN_INT_SLOW1: c_uint = 0x0800 /* 1.28 sec */;
pub const DISCOV_LE_SCAN_WIN_SLOW1: c_uint = 0x0012 /* 11.25 msec */;
pub const DISCOV_LE_SCAN_INT_SLOW2: c_uint = 0x1000 /* 2.56 sec */;
pub const DISCOV_LE_SCAN_WIN_SLOW2: c_uint = 0x0024 /* 22.5 msec */;
pub const DISCOV_CODED_SCAN_INT_FAST: c_uint = 0x0120 /* 180 msec */;
pub const DISCOV_CODED_SCAN_WIN_FAST: c_uint = 0x0090 /* 90 msec */;
pub const DISCOV_CODED_SCAN_INT_SLOW1: c_uint = 0x1800 /* 3.84 sec */;
pub const DISCOV_CODED_SCAN_WIN_SLOW1: c_uint = 0x0036 /* 33.75 msec */;
pub const DISCOV_CODED_SCAN_INT_SLOW2: c_uint = 0x3000 /* 7.68 sec */;
pub const DISCOV_CODED_SCAN_WIN_SLOW2: c_uint = 0x006c /* 67.5 msec */;

pub const DISCOV_INTERLEAVED_INQUIRY_LEN: c_uint = 0x04;
pub const DISCOV_BREDR_INQUIRY_LEN: c_uint = 0x08;

pub const DISCOV_LE_FAST_ADV_INT_MIN: c_uint = 0x00A0	/* 100 msec */;
pub const DISCOV_LE_FAST_ADV_INT_MAX: c_uint = 0x00F0	/* 150 msec */;
pub const DISCOV_LE_PER_ADV_INT_MIN: c_uint = 0x00A0	/* 200 msec */;
pub const DISCOV_LE_PER_ADV_INT_MAX: c_uint = 0x00A0	/* 200 msec */;
pub const DISCOV_LE_ADV_MESH_MIN: c_uint = 0x00A0  /* 100 msec */;
pub const DISCOV_LE_ADV_MESH_MAX: c_uint = 0x00A0  /* 100 msec */;

extern "C" {
    pub fn mgmt_fill_version_info(ver: *mut c_void);
}
extern "C" {
    pub fn mgmt_new_settings(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn mgmt_index_added(hdev: *mut hci_dev);
}
extern "C" {
    pub fn mgmt_index_removed(hdev: *mut hci_dev);
}
extern "C" {
    pub fn mgmt_set_powered_failed(hdev: *mut hci_dev, err: c_int);
}
extern "C" {
    pub fn mgmt_power_on(hdev: *mut hci_dev, err: c_int);
}
extern "C" {
    pub fn __mgmt_power_off(hdev: *mut hci_dev);
}
extern "C" {
    pub fn hci_to_mgmt_reason(err: u8) -> u8;
}
extern "C" {
    pub fn mgmt_pin_code_request(hdev: *mut hci_dev, bdaddr: *mut bdaddr_t, secure: u8);
}
extern "C" {
    pub fn mgmt_auth_failed(conn: *mut hci_conn, status: u8);
}
extern "C" {
    pub fn mgmt_auth_enable_complete(hdev: *mut hci_dev, status: u8);
}
extern "C" {
    pub fn mgmt_set_local_name_complete(hdev: *mut hci_dev, name: *mut u8, status: u8);
}
extern "C" {
    pub fn mgmt_discovering(hdev: *mut hci_dev, discovering: u8);
}
extern "C" {
    pub fn mgmt_suspending(hdev: *mut hci_dev, state: u8);
}
extern "C" {
    pub fn mgmt_powering_down(hdev: *mut hci_dev) -> bool;
}
extern "C" {
    pub fn mgmt_new_ltk(hdev: *mut hci_dev, key: *mut smp_ltk, persistent: bool);
}
extern "C" {
    pub fn mgmt_new_irk(hdev: *mut hci_dev, irk: *mut smp_irk, persistent: bool);
}
extern "C" {
    pub fn mgmt_smp_complete(conn: *mut hci_conn, complete: bool);
}
extern "C" {
    pub fn mgmt_get_connectable(hdev: *mut hci_dev) -> bool;
}
extern "C" {
    pub fn mgmt_get_adv_discov_flags(hdev: *mut hci_dev) -> u8;
}
extern "C" {
    pub fn mgmt_phy_configuration_changed(hdev: *mut hci_dev, skip: *mut sock) -> c_int;
}
extern "C" {
    pub fn hci_abort_conn(conn: *mut hci_conn, reason: u8) -> c_int;
}
pub const SCO_AIRMODE_MASK: c_uint = 0x0003;
pub const SCO_AIRMODE_CVSD: c_uint = 0x0000;
pub const SCO_AIRMODE_TRANSP: c_uint = 0x0003;

pub const TRANSPORT_TYPE_MAX: c_uint = 0x04;
