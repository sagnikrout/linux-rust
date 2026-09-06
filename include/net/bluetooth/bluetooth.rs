//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/bluetooth.h
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

pub const BT_SUBSYS_VERSION: c_int = 2;
pub const BT_SUBSYS_REVISION: c_int = 22;

pub const AF_BLUETOOTH: c_int = 31;

// Bluetooth versions
pub const BLUETOOTH_VER_1_1: c_int = 1;
pub const BLUETOOTH_VER_1_2: c_int = 2;
pub const BLUETOOTH_VER_2_0: c_int = 3;
pub const BLUETOOTH_VER_2_1: c_int = 4;
pub const BLUETOOTH_VER_4_0: c_int = 6;
// Reserv for core and drivers use
pub const BT_SKB_RESERVE: c_int = 8;
pub const BTPROTO_L2CAP: c_int = 0;
pub const BTPROTO_HCI: c_int = 1;
pub const BTPROTO_SCO: c_int = 2;
pub const BTPROTO_RFCOMM: c_int = 3;
pub const BTPROTO_BNEP: c_int = 4;
pub const BTPROTO_CMTP: c_int = 5;
pub const BTPROTO_HIDP: c_int = 6;
pub const BTPROTO_AVDTP: c_int = 7;
pub const BTPROTO_ISO: c_int = 8;

pub const SOL_HCI: c_int = 0;
pub const SOL_L2CAP: c_int = 6;
pub const SOL_SCO: c_int = 17;
pub const SOL_RFCOMM: c_int = 18;
pub const BT_SECURITY: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_security {
    pub level: __u8,
    pub key_size: __u8,
}

pub const BT_SECURITY_SDP: c_int = 0;
pub const BT_SECURITY_LOW: c_int = 1;
pub const BT_SECURITY_MEDIUM: c_int = 2;
pub const BT_SECURITY_HIGH: c_int = 3;
pub const BT_SECURITY_FIPS: c_int = 4;
pub const BT_DEFER_SETUP: c_int = 7;
pub const BT_FLUSHABLE: c_int = 8;
pub const BT_FLUSHABLE_OFF: c_int = 0;
pub const BT_FLUSHABLE_ON: c_int = 1;
pub const BT_POWER: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_power {
    pub force_active: __u8,
}

pub const BT_POWER_FORCE_ACTIVE_OFF: c_int = 0;
pub const BT_POWER_FORCE_ACTIVE_ON: c_int = 1;
pub const BT_CHANNEL_POLICY: c_int = 10;
// BR/EDR only (default policy)
// AMP controllers cannot be used.
// Channel move requests from the remote device are denied.
// If the L2CAP channel is currently using AMP, move the channel to BR/EDR.
//
pub const BT_CHANNEL_POLICY_BREDR_ONLY: c_int = 0;
// BR/EDR Preferred
// Allow use of AMP controllers.
// If the L2CAP channel is currently on AMP, move it to BR/EDR.
// Channel move requests from the remote device are allowed.
//
pub const BT_CHANNEL_POLICY_BREDR_PREFERRED: c_int = 1;
// AMP Preferred
// Allow use of AMP controllers
// If the L2CAP channel is currently on BR/EDR and AMP controller
// resources are available, initiate a channel move to AMP.
// Channel move requests from the remote device are allowed.
// If the L2CAP socket has not been connected yet, try to create
// and configure the channel directly on an AMP controller rather
// than BR/EDR.
//
pub const BT_CHANNEL_POLICY_AMP_PREFERRED: c_int = 2;
pub const BT_VOICE: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_voice {
    pub setting: __u16,
}

pub const BT_VOICE_TRANSPARENT: c_uint = 0x0003;
pub const BT_VOICE_CVSD_16BIT: c_uint = 0x0060;
pub const BT_VOICE_TRANSPARENT_16BIT: c_uint = 0x0063;
pub const BT_SNDMTU: c_int = 12;
pub const BT_RCVMTU: c_int = 13;
pub const BT_PHY: c_int = 14;

pub const BT_MODE: c_int = 15;
pub const BT_MODE_BASIC: c_uint = 0x00;
pub const BT_MODE_ERTM: c_uint = 0x01;
pub const BT_MODE_STREAMING: c_uint = 0x02;
pub const BT_MODE_LE_FLOWCTL: c_uint = 0x03;
pub const BT_MODE_EXT_FLOWCTL: c_uint = 0x04;
pub const BT_PKT_STATUS: c_int = 16;
pub const BT_SCM_PKT_STATUS: c_uint = 0x03;
pub const BT_SCM_ERROR: c_uint = 0x04;
pub const BT_ISO_QOS: c_int = 17;
pub const BT_ISO_QOS_CIG_UNSET: c_uint = 0xff;
pub const BT_ISO_QOS_CIS_UNSET: c_uint = 0xff;
pub const BT_ISO_QOS_BIG_UNSET: c_uint = 0xff;
pub const BT_ISO_QOS_BIS_UNSET: c_uint = 0xff;
pub const BT_ISO_SYNC_TIMEOUT: c_uint = 0x07d0 /* 20 secs */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_iso_io_qos {
    pub interval: __u32,
    pub latency: __u16,
    pub sdu: __u16,
    pub phys: __u8,
    pub rtn: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_iso_ucast_qos {
    pub cig: __u8,
    pub cis: __u8,
    pub sca: __u8,
    pub packing: __u8,
    pub framing: __u8,
    pub in: bt_iso_io_qos,
    pub out: bt_iso_io_qos,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_iso_bcast_qos {
    pub big: __u8,
    pub bis: __u8,
    pub sync_factor: __u8,
    pub packing: __u8,
    pub framing: __u8,
    pub in: bt_iso_io_qos,
    pub out: bt_iso_io_qos,
    pub encryption: __u8,
    pub bcode: [__u8; 16],
    pub options: __u8,
    pub skip: __u16,
    pub sync_timeout: __u16,
    pub sync_cte_type: __u8,
    pub mse: __u8,
    pub timeout: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_iso_qos {
    pub ucast: bt_iso_ucast_qos,
    pub bcast: bt_iso_bcast_qos,
}

pub const BT_CODEC: c_int = 19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_codec_caps {
    pub len: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_codec {
    pub id: __u8,
    pub cid: __u16,
    pub vid: __u16,
    pub data_path: __u8,
    pub num_caps: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_codecs {
    pub num_codecs: __u8,
    pub codecs: [bt_codec; ],
    pub __packed: },
pub const BT_CODEC_CVSD: c_uint = 0x02;
pub const BT_CODEC_TRANSPARENT: c_uint = 0x03;
pub const BT_CODEC_MSBC: c_uint = 0x05;
pub const BT_ISO_BASE: c_int = 20;
// Socket option value 21 reserved
pub const BT_PKT_SEQNUM: c_int = 22;
pub const BT_SCM_PKT_SEQNUM: c_uint = 0x05;
    pub ...): *const *const void bt_info(char fmt,,
    pub ...): *const *const void bt_warn(char fmt,,
    pub ...): *const *const void bt_err(char fmt,,

    pub enable): void bt_dbg_set(bool,
    pub bt_dbg_get(void): bool,
    pub ...): *const *const void bt_dbg(char fmt,,

    pub ...): *const *const void bt_warn_ratelimited(char fmt,,
    pub ...): *const *const void bt_err_ratelimited(char fmt,,

// Connection and socket states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_sock_state {
    BT_CONNECTED = 1, /* Equal to TCP_ESTABLISHED to make net code happy */
    BT_OPEN,
    BT_BOUND,
    BT_LISTEN,
    BT_CONNECT,
    BT_CONNECT2,
    BT_CONFIG,
    BT_DISCONN,
    BT_CLOSED
}

// If unused will be removed by compiler
    pub "BT_CONNECTED": return,
    pub "BT_OPEN": return,
    pub "BT_BOUND": return,
    pub "BT_LISTEN": return,
    pub "BT_CONNECT": return,
    pub "BT_CONNECT2": return,
    pub "BT_CONFIG": return,
    pub "BT_DISCONN": return,
    pub "BT_CLOSED": return,
    pub state": return "invalid,
// BD Address
    pub b: [__u8; 6],
    pub bdaddr_t: } __packed,
// BD Address type
pub const BDADDR_BREDR: c_uint = 0x00;
pub const BDADDR_LE_PUBLIC: c_uint = 0x01;
pub const BDADDR_LE_RANDOM: c_uint = 0x02;
    pub true: return,
    pub false: return,
    pub true: return,
    pub false: return,

// Copy, swap, convert BD Address
    pub sizeof(bdaddr_t)): return memcmp(ba1, ba2,,
    pub sizeof(bdaddr_t)): memcpy(dst, src,,
    pub src): *const *const void baswap(bdaddr_t dst, bdaddr_t,
// Common socket structures and functions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_sock {
    pub sk: sock,
    pub accept_q: list_head,
    pub /: *mut *mut spinlock_t accept_q_lock; / protects accept_q,
    pub parent: *mut sock,
    pub flags: c_ulong,
    pub ): *mut *mut *mut *mut void (skb_msg_name)(struct sk_buff , void , int,
    pub ): *mut *mut *mut *mut void (skb_put_cmsg)(struct sk_buff , struct msghdr , struct sock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_sock_list {
    pub head: hlist_head,
    pub lock: rwlock_t,

    pub ): *mut *mut *mut int ( custom_seq_show)(struct seq_file , void,

}

extern "C" {
    pub fn bt_sock_register(proto: c_int, ops: *const net_proto_family) -> c_int;
}
extern "C" {
    pub fn bt_sock_unregister(proto: c_int);
}
extern "C" {
    pub fn bt_sock_link(l: *mut bt_sock_list, s: *mut sock);
}
extern "C" {
    pub fn bt_sock_unlink(l: *mut bt_sock_list, s: *mut sock);
}
extern "C" {
    pub fn bt_sock_linked(l: *mut bt_sock_list, s: *mut sock) -> bool;
}
extern "C" {
    pub fn bt_sock_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn bt_sock_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn bt_sock_wait_state(sk: *mut sock, state: c_int, timeo: c_ulong) -> c_int;
}
extern "C" {
    pub fn bt_sock_wait_ready(sk: *mut sock, msg_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn bt_accept_enqueue(parent: *mut sock, sk: *mut sock, bh: bool);
}
extern "C" {
    pub fn bt_accept_unlink(sk: *mut sock);
}
// Skb helpers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cap_ctrl {
    pub reqseq: u16,
    pub txseq: u16,
    pub retries: u8,
    pub psm: __le16,
    pub bdaddr: bdaddr_t,
    pub chan: *mut l2cap_chan,
}

extern "C" {
    pub fn void(hdev: *mut *mut hci_req_complete_t)(struct hci_dev, status: u8, opcode: u16) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ctrl {
    pub sk: *mut sock,
    pub opcode: u16,
    pub req_flags: u8,
    pub req_event: u8,
    pub req_complete: hci_req_complete_t,
    pub req_complete_skb: hci_req_complete_skb_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_ctrl {
    pub hdev: *mut hci_dev,
    pub opcode: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_skb_cb {
    pub pkt_type: u8,
    pub force_active: u8,
    pub expect: u16,
    pub pkt_seqnum: u16,
    pub incoming:1: u8,
    pub pkt_status:2: u8,
    pub l2cap: l2cap_ctrl,
    pub hci: hci_ctrl,
    pub mgmt: mgmt_ctrl,
    pub creds: scm_creds,
}

// err = sock_error(sk);
// err = -ECONNRESET;
// Shall not be called with lock_sock held
extern "C" {
    pub fn ERR_PTR(_arg: err) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EFAULT) -> return;
}
// Similar to bt_skb_sendmsg but can split the msg into multiple fragments
// accourding to the MTU.
//
// Add remaining data over MTU as continuation fragments
// frag = tmp;
extern "C" {
    pub fn bt_to_errno(code: u16) -> c_int;
}
extern "C" {
    pub fn bt_status(err: c_int) -> __u8;
}
extern "C" {
    pub fn hci_sock_set_flag(sk: *mut sock, nr: c_int);
}
extern "C" {
    pub fn hci_sock_clear_flag(sk: *mut sock, nr: c_int);
}
extern "C" {
    pub fn hci_sock_test_flag(sk: *mut sock, nr: c_int) -> c_int;
}
extern "C" {
    pub fn hci_sock_get_channel(sk: *mut sock) -> c_ushort;
}
extern "C" {
    pub fn hci_sock_get_cookie(sk: *mut sock) -> u32;
}
extern "C" {
    pub fn hci_sock_init() -> c_int;
}
extern "C" {
    pub fn hci_sock_cleanup();
}
extern "C" {
    pub fn bt_sysfs_init() -> c_int;
}
extern "C" {
    pub fn bt_sysfs_cleanup();
}
extern "C" {
    pub fn bt_procfs_cleanup(net: *mut net, name: *const c_char);
}
extern "C" {
    pub fn l2cap_init() -> c_int;
}
extern "C" {
    pub fn l2cap_exit();
}

extern "C" {
    pub fn sco_init() -> c_int;
}
extern "C" {
    pub fn sco_exit();
}

extern "C" {
    pub fn iso_init() -> c_int;
}
extern "C" {
    pub fn iso_exit() -> c_int;
}
extern "C" {
    pub fn iso_inited() -> bool;
}

extern "C" {
    pub fn mgmt_init() -> c_int;
}
extern "C" {
    pub fn mgmt_exit();
}
extern "C" {
    pub fn mgmt_cleanup(sk: *mut sock);
}
extern "C" {
    pub fn bt_sock_reclassify_lock(sk: *mut sock, proto: c_int);
}
