//! Automatically rewritten from C Header to Rust Module
//! Source: net/ncsi/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright Gavin Shan, IBM Corporation 2016.
//
// Supported media status bits for Mellanox Mac affinity command.
// Bit (0-2) for different protocol support; Bit 1 for RBT support,
// bit 1 for SMBUS support and bit 2 for PCIE support. Bit (3-5)
// for different protocol availability. Bit 4 for RBT, bit 4 for
// SMBUS and bit 5 for PCIE.
//
// OEM Vendor Manufacture ID
pub const NCSI_OEM_MFR_MLX_ID: c_uint = 0x8119;
pub const NCSI_OEM_MFR_BCM_ID: c_uint = 0x113d;
pub const NCSI_OEM_MFR_INTEL_ID: c_uint = 0x157;
// Intel specific OEM command
pub const NCSI_OEM_INTEL_CMD_GMA: c_uint = 0x06   /* CMD ID for Get MAC */;
pub const NCSI_OEM_INTEL_CMD_KEEP_PHY: c_uint = 0x20   /* CMD ID for Keep PHY up */;
// Broadcom specific OEM Command
pub const NCSI_OEM_BCM_CMD_GMA: c_uint = 0x01   /* CMD ID for Get MAC */;
// Mellanox specific OEM Command
pub const NCSI_OEM_MLX_CMD_GMA: c_uint = 0x00   /* CMD ID for Get MAC */;
pub const NCSI_OEM_MLX_CMD_GMA_PARAM: c_uint = 0x1b   /* Parameter for GMA  */;
pub const NCSI_OEM_MLX_CMD_SMAF: c_uint = 0x01   /* CMD ID for Set MC Affinity */;
pub const NCSI_OEM_MLX_CMD_SMAF_PARAM: c_uint = 0x07   /* Parameter for SMAF         */;
// OEM Command payload lengths
pub const NCSI_OEM_INTEL_CMD_GMA_LEN: c_int = 5;
pub const NCSI_OEM_INTEL_CMD_KEEP_PHY_LEN: c_int = 7;
pub const NCSI_OEM_BCM_CMD_GMA_LEN: c_int = 12;
pub const NCSI_OEM_MLX_CMD_GMA_LEN: c_int = 8;
pub const NCSI_OEM_MLX_CMD_SMAF_LEN: c_int = 60;
// Offset in OEM request

// Mac address offset in OEM response
pub const BCM_MAC_ADDR_OFFSET: c_int = 28;
pub const MLX_MAC_ADDR_OFFSET: c_int = 8;
pub const INTEL_MAC_ADDR_OFFSET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_channel_version {
    pub /: *mut *mut u8 major; / NCSI version major,
    pub /: *mut *mut u8 minor; / NCSI version minor,
    pub /: *mut *mut u8 update; / NCSI version update,
    pub /: *mut *mut char alpha1; / NCSI version alpha1,
    pub /: *mut *mut char alpha2; / NCSI version alpha2,
    pub /: *mut *mut u8 fw_name[12 + 1]; / Firmware name string,
    pub /: *mut *mut u32 fw_version; / Firmware version,
    pub /: *mut *mut u16 pci_ids[4]; / PCI identification,
    pub /: *mut *mut u32 mf_id; / Manufacture ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_channel_cap {
    pub /: *mut *mut u32 index; / Index of channel capabilities,
    pub /: *mut *mut u32 cap; / NCSI channel capability,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_channel_mode {
    pub /: *mut *mut u32 index; / Index of channel modes,
    pub /: *mut *mut u32 enable; / Enabled or disabled,
    pub /: *mut *mut u32 size; / Valid entries in ncm_data[],
    pub /: *mut *mut u32 data[8]; / Data entries,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_channel_mac_filter {
    pub n_uc: u8,
    pub n_mc: u8,
    pub n_mixed: u8,
    pub bitmap: u64,
    pub addrs: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_channel_vlan_filter {
    pub n_vids: u8,
    pub bitmap: u64,
    pub vids: *mut u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_channel_stats {
    pub /: *mut *mut u64 hnc_cnt; / Counter cleared,
    pub /: *mut *mut u64 hnc_rx_bytes; / Rx bytes,
    pub /: *mut *mut u64 hnc_tx_bytes; / Tx bytes,
    pub /: *mut *mut u64 hnc_rx_uc_pkts; / Rx UC packets,
    pub /: *mut *mut u64 hnc_rx_mc_pkts; / Rx MC packets,
    pub /: *mut *mut u64 hnc_rx_bc_pkts; / Rx BC packets,
    pub /: *mut *mut u64 hnc_tx_uc_pkts; / Tx UC packets,
    pub /: *mut *mut u64 hnc_tx_mc_pkts; / Tx MC packets,
    pub /: *mut *mut u64 hnc_tx_bc_pkts; / Tx BC packets,
    pub /: *mut *mut u32 hnc_fcs_err; / FCS errors,
    pub /: *mut *mut u32 hnc_align_err; / Alignment errors,
    pub /: *mut *mut u32 hnc_false_carrier; / False carrier detection,
    pub /: *mut *mut u32 hnc_runt_pkts; / Rx runt packets,
    pub /: *mut *mut u32 hnc_jabber_pkts; / Rx jabber packets,
    pub /: *mut *mut u32 hnc_rx_pause_xon; / Rx pause XON frames,
    pub /: *mut *mut u32 hnc_rx_pause_xoff; / Rx XOFF frames,
    pub /: *mut *mut u32 hnc_tx_pause_xon; / Tx XON frames,
    pub /: *mut *mut u32 hnc_tx_pause_xoff; / Tx XOFF frames,
    pub /: *mut *mut u32 hnc_tx_s_collision; / Single collision frames,
    pub /: *mut *mut u32 hnc_tx_m_collision; / Multiple collision frames,
    pub /: *mut *mut u32 hnc_l_collision; / Late collision frames,
    pub /: *mut *mut u32 hnc_e_collision; / Excessive collision frames,
    pub /: *mut *mut u32 hnc_rx_ctl_frames; / Rx control frames,
    pub /: *mut *mut u32 hnc_rx_64_frames; / Rx 64-bytes frames,
    pub /: *mut *mut u32 hnc_rx_127_frames; / Rx 65-127 bytes frames,
    pub /: *mut *mut u32 hnc_rx_255_frames; / Rx 128-255 bytes frames,
    pub /: *mut *mut u32 hnc_rx_511_frames; / Rx 256-511 bytes frames,
    pub /: *mut *mut u32 hnc_rx_1023_frames; / Rx 512-1023 bytes frames,
    pub /: *mut *mut u32 hnc_rx_1522_frames; / Rx 1024-1522 bytes frames,
    pub /: *mut *mut u32 hnc_rx_9022_frames; / Rx 1523-9022 bytes frames,
    pub /: *mut *mut u32 hnc_tx_64_frames; / Tx 64-bytes frames,
    pub /: *mut *mut u32 hnc_tx_127_frames; / Tx 65-127 bytes frames,
    pub /: *mut *mut u32 hnc_tx_255_frames; / Tx 128-255 bytes frames,
    pub /: *mut *mut u32 hnc_tx_511_frames; / Tx 256-511 bytes frames,
    pub /: *mut *mut u32 hnc_tx_1023_frames; / Tx 512-1023 bytes frames,
    pub /: *mut *mut u32 hnc_tx_1522_frames; / Tx 1024-1522 bytes frames,
    pub /: *mut *mut u32 hnc_tx_9022_frames; / Tx 1523-9022 bytes frames,
    pub /: *mut *mut u64 hnc_rx_valid_bytes; / Rx valid bytes,
    pub /: *mut *mut u32 hnc_rx_runt_pkts; / Rx error runt packets,
    pub /: *mut *mut u32 hnc_rx_jabber_pkts; / Rx error jabber packets,
    pub /: *mut *mut u32 ncsi_rx_cmds; / Rx NCSI commands,
    pub /: *mut *mut u32 ncsi_dropped_cmds; / Dropped commands,
    pub /: *mut *mut u32 ncsi_cmd_type_errs; / Command type errors,
    pub /: *mut *mut u32 ncsi_cmd_csum_errs; / Command checksum errors,
    pub /: *mut *mut u32 ncsi_rx_pkts; / Rx NCSI packets,
    pub /: *mut *mut u32 ncsi_tx_pkts; / Tx NCSI packets,
    pub /: *mut *mut u32 ncsi_tx_aen_pkts; / Tx AEN packets,
    pub /: *mut *mut u32 pt_tx_pkts; / Tx packets,
    pub /: *mut *mut u32 pt_tx_dropped; / Tx dropped packets,
    pub /: *mut *mut u32 pt_tx_channel_err; / Tx channel errors,
    pub /: *mut *mut u32 pt_tx_us_err; / Tx undersize errors,
    pub /: *mut *mut u32 pt_rx_pkts; / Rx packets,
    pub /: *mut *mut u32 pt_rx_dropped; / Rx dropped packets,
    pub /: *mut *mut u32 pt_rx_channel_err; / Rx channel errors,
    pub /: *mut *mut u32 pt_rx_us_err; / Rx undersize errors,
    pub /: *mut *mut u32 pt_rx_os_err; / Rx oversize errors,
}

pub const NCSI_PACKAGE_SHIFT: c_int = 5;

pub const NCSI_RESERVED_CHANNEL: c_uint = 0x1f;

pub const NCSI_MAX_PACKAGE: c_int = 8;
pub const NCSI_MAX_CHANNEL: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_channel {
    pub id: c_uchar,
    pub state: c_int,
pub const NCSI_CHANNEL_INACTIVE: c_int = 1;
pub const NCSI_CHANNEL_ACTIVE: c_int = 2;
pub const NCSI_CHANNEL_INVISIBLE: c_int = 3;
    pub reconfigure_needed: bool,
    pub /: *mut *mut spinlock_t lock; / Protect filters etc,
    pub package: *mut ncsi_package,
    pub version: ncsi_channel_version,
    pub caps: [ncsi_channel_cap; NCSI_CAP_MAX],
    pub modes: [ncsi_channel_mode; NCSI_MODE_MAX],
// Filtering Settings
    pub mac_filter: ncsi_channel_mac_filter,
    pub vlan_filter: ncsi_channel_vlan_filter,
    pub stats: ncsi_channel_stats,
    pub timer: timer_list,
    pub enabled: bool,
    pub state: c_uint,
pub const NCSI_CHANNEL_MONITOR_START: c_int = 0;
pub const NCSI_CHANNEL_MONITOR_RETRY: c_int = 1;
pub const NCSI_CHANNEL_MONITOR_WAIT: c_int = 2;
pub const NCSI_CHANNEL_MONITOR_WAIT_MAX: c_int = 5;
    pub monitor: },
    pub node: list_head,
    pub link: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_package {
    pub /: *mut *mut unsigned char id; / NCSI 3-bits package ID,
    pub /: *mut *mut unsigned char uuid[16]; / UUID,
    pub /: *mut *mut *mut ncsi_dev_priv ndp; / NCSI device,
    pub /: *mut *mut spinlock_t lock; / Protect the package,
    pub /: *mut *mut unsigned int channel_num; / Number of channels,
    pub /: *mut *mut list_head channels; / List of channels,
    pub /: *mut *mut list_head node; / Form list of packages,
    pub /: *mut *mut bool multi_channel; / Enable multiple channels,
    pub /: *mut *mut u32 channel_whitelist; / Channels to configure,
    pub /: *mut *mut *mut ncsi_channel preferred_channel; / Primary channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_request {
    pub /: *mut *mut unsigned char id; / Request ID - 0 to 255,
    pub /: *mut *mut bool used; / Request that has been assigned,
    pub /: *mut *mut unsigned int flags; / NCSI request property,
pub const NCSI_REQ_FLAG_EVENT_DRIVEN: c_int = 1;
pub const NCSI_REQ_FLAG_NETLINK_DRIVEN: c_int = 2;
    pub /: *mut *mut *mut ncsi_dev_priv ndp; / Associated NCSI device,
    pub /: *mut *mut *mut sk_buff cmd; / Associated NCSI command packet,
    pub /: *mut *mut *mut sk_buff rsp; / Associated NCSI response packet,
    pub /: *mut *mut timer_list timer; / Timer on waiting for response,
    pub /: *mut *mut bool enabled; / Time has been enabled or not,
    pub /: *mut *mut u32 snd_seq; / netlink sending sequence number,
    pub /: *mut *mut u32 snd_portid; / netlink portid of sender,
    pub /: *mut *mut nlmsghdr nlhdr; / netlink message header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlan_vid {
    pub list: list_head,
    pub proto: __be16,
    pub vid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_dev_priv {
    pub /: *mut *mut ncsi_dev ndev; / Associated NCSI device,
    pub /: *mut *mut unsigned int flags; / NCSI device flags,

pub const NCSI_DEV_RESHUFFLE: c_int = 4;

    pub /: *mut *mut unsigned int gma_flag; / OEM GMA flag,
    pub /: *mut *mut sockaddr_storage pending_mac; / MAC address received from GMA,
    pub /: *mut *mut spinlock_t lock; / Protect the NCSI device,
    pub /: *mut *mut unsigned int package_probe_id;/ Current ID during probe,
    pub /: *mut *mut unsigned int package_num; / Number of packages,
    pub /: *mut *mut unsigned int channel_probe_id;/ Current cahnnel ID during probe,
    pub /: *mut *mut list_head packages; / List of packages,
    pub /: *mut *mut *mut ncsi_channel hot_channel; / Channel was ever active,
    pub /: *mut *mut ncsi_request requests[256]; / Request table,
    pub /: *mut *mut unsigned int request_id; / Last used request ID,
pub const NCSI_REQ_START_IDX: c_int = 1;
    pub /: *mut *mut unsigned int pending_req_num; / Number of pending requests,
    pub /: *mut *mut *mut ncsi_package active_package; / Currently handled package,
    pub /: *mut *mut *mut ncsi_channel active_channel; / Currently handled channel,
    pub /: *mut *mut list_head channel_queue; / Config queue of channels,
    pub /: *mut *mut work_work; / For channel management,
    pub /: *mut *mut packet_type ptype; / NCSI packet Rx handler,
    pub /: *mut *mut list_head node; / Form NCSI device list,
pub const NCSI_MAX_VLAN_VIDS: c_int = 15;
    pub /: *mut *mut list_head vlan_vids; / List of active VLAN IDs,
    pub /: *mut *mut bool multi_package; / Enable multiple packages,
    pub /: *mut *mut bool mlx_multi_host; / Enable multi host Mellanox,
    pub /: *mut *mut u32 package_whitelist; / Packages to configure,
    pub /: *mut *mut unsigned char channel_count; / Num of channels to probe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ncsi_cmd_arg {
    pub /: *mut *mut *mut ncsi_dev_priv ndp; / Associated NCSI device,
    pub /: *mut *mut unsigned char type; / Command in the NCSI packet,
    pub /: *mut *mut unsigned char id; / Request ID (sequence number),
    pub /: *mut *mut unsigned char package; / Destination package ID,
    pub /: *mut *mut unsigned char channel; / Destination channel ID or 0x1f,
    pub /: *mut *mut unsigned short payload; / Command packet payload length,
    pub /: *mut *mut unsigned int req_flags; / NCSI request properties,
    pub /: *mut *mut unsigned char bytes[16]; / Command packet specific data,
    pub words: [c_ushort; 8],
    pub dwords: [c_uint; 4],
}

// Resources
extern "C" {
    pub fn ncsi_reset_dev(nd: *mut ncsi_dev) -> c_int;
}
extern "C" {
    pub fn ncsi_start_channel_monitor(nc: *mut ncsi_channel);
}
extern "C" {
    pub fn ncsi_stop_channel_monitor(nc: *mut ncsi_channel);
}
extern "C" {
    pub fn ncsi_remove_package(np: *mut ncsi_package);
}
extern "C" {
    pub fn ncsi_free_request(nr: *mut ncsi_request);
}
extern "C" {
    pub fn ncsi_process_next_channel(ndp: *mut ncsi_dev_priv) -> c_int;
}
extern "C" {
    pub fn ncsi_channel_has_link(channel: *mut ncsi_channel) -> bool;
}
// Packet handlers
extern "C" {
    pub fn ncsi_calculate_checksum(data: *mut c_uchar, len: c_int) -> u32;
}
extern "C" {
    pub fn ncsi_xmit_cmd(nca: *mut ncsi_cmd_arg) -> c_int;
}
extern "C" {
    pub fn ncsi_aen_handler(ndp: *mut ncsi_dev_priv, skb: *mut sk_buff) -> c_int;
}
