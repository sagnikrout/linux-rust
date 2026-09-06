//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/cfg802154.h
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
// Copyright (C) 2007, 2008, 2009 Siemens AG
//
// Written by:
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg802154_ops {
    pub type): c_int,
    pub dev): *mut net_device,
    pub wpan_phy): *mut *mut int (suspend)(struct wpan_phy,
    pub wpan_phy): *mut *mut int (resume)(struct wpan_phy,
    pub extended_addr): __le64,
    pub wpan_dev): *mut wpan_dev,
    pub channel): *mut *mut *mut int (set_channel)(struct wpan_phy wpan_phy, u8 page, u8,
    pub cca): *const wpan_phy_cca,
    pub ed_level): *mut *mut *mut int (set_cca_ed_level)(struct wpan_phy wpan_phy, s32,
    pub power): *mut *mut *mut int (set_tx_power)(struct wpan_phy wpan_phy, s32,
    pub pan_id): *mut *mut wpan_dev wpan_dev, __le16,
    pub short_addr): *mut *mut wpan_dev wpan_dev, __le16,
    pub max_be): u8,
    pub max_csma_backoffs): u8,
    pub max_frame_retries): i8,
    pub mode): *mut *mut wpan_dev wpan_dev, bool,
    pub ackreq): *mut *mut wpan_dev wpan_dev, bool,
    pub request): *mut cfg802154_scan_request,
    pub wpan_dev): *mut wpan_dev,
    pub request): *mut cfg802154_beacon_request,
    pub wpan_dev): *mut wpan_dev,
    pub coord): *mut ieee802154_addr,
    pub target): *mut ieee802154_addr,

    pub table): *mut ieee802154_llsec_table,
    pub wpan_dev): *mut wpan_dev,
    pub wpan_dev): *mut wpan_dev,
// TODO remove locking/get table callbacks, this is part of the
// nl802154 interface and should be accessible from ieee802154 layer.
//
    pub params): *mut ieee802154_llsec_params,
    pub changed): c_int,
    pub key): *const ieee802154_llsec_key,
    pub id): *const ieee802154_llsec_key_id,
    pub sl): *const ieee802154_llsec_seclevel,
    pub sl): *const ieee802154_llsec_seclevel,
    pub dev): *const ieee802154_llsec_device,
    pub extended_addr): *mut *mut wpan_dev wpan_dev, __le64,
    pub key): *const ieee802154_llsec_device_key,
    pub key): *const ieee802154_llsec_device_key,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpan_phy_supported {
    pub iftypes: cca_modes, cca_opts,,
    pub lbt: nl802154_supported_bool_states,
    pub max_csma_backoffs: min_csma_backoffs,,
    pub max_frame_retries: s8 min_frame_retries,,
    pub cca_ed_levels_size: size_t tx_powers_size,,
    pub cca_ed_levels: *const *const s32 tx_powers,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpan_phy_cca {
    pub mode: nl802154_cca_modes,
    pub opt: nl802154_cca_opts,
}

//
// enum wpan_phy_flags - WPAN PHY state flags
// @WPAN_PHY_FLAG_TXPOWER: Indicates that transceiver will support
// transmit power setting.
// @WPAN_PHY_FLAG_CCA_ED_LEVEL: Indicates that transceiver will support cca ed
// level setting.
// @WPAN_PHY_FLAG_CCA_MODE: Indicates that transceiver will support cca mode
// setting.
// @WPAN_PHY_FLAG_STATE_QUEUE_STOPPED: Indicates that the transmit queue was
// temporarily stopped.
// @WPAN_PHY_FLAG_DATAGRAMS_ONLY: Indicates that transceiver is only able to
// send/receive datagrams.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wpan_phy_flags {
    WPAN_PHY_FLAG_TXPOWER		= BIT(1),
    WPAN_PHY_FLAG_CCA_ED_LEVEL	= BIT(2),
    WPAN_PHY_FLAG_CCA_MODE		= BIT(3),
    WPAN_PHY_FLAG_STATE_QUEUE_STOPPED = BIT(4),
    WPAN_PHY_FLAG_DATAGRAMS_ONLY	= BIT(5),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpan_phy {
// If multiple wpan_phys are registered and you're handed e.g.
// a regular netdev with assigned ieee802154_ptr, you won't
// know whether it points to a wpan_phy your driver has registered
// or not. Assign this to something global to your driver to
// help determine whether you own this wpan_phy or not.
//
    pub privid: *const c_void,
    pub flags: c_ulong,
//
// This is a PIB according to 802.15.4-2011.
// We do not provide timing-related variables, as they
// aren't used outside of driver
//
    pub current_channel: u8,
    pub current_page: u8,
    pub supported: wpan_phy_supported,
// current transmit_power in mBm
    pub transmit_power: i32,
    pub cca: wpan_phy_cca,
    pub perm_extended_addr: __le64,
// current cca ed threshold in mBm
    pub cca_ed_level: i32,
// PHY depended MAC PIB values
// 802.15.4 acronym: Tdsym in nsec
    pub symbol_duration: u32,
// lifs and sifs periods timing
    pub lifs_period: u16,
    pub sifs_period: u16,
    pub dev: device,
// the network namespace this phy lives in currently
    pub _net: possible_net_t,
// Transmission monitoring and control
    pub queue_lock: spinlock_t,
    pub ongoing_txs: core::sync::atomic::AtomicI32,
    pub hold_txs: core::sync::atomic::AtomicI32,
    pub sync_txq: wait_queue_head_t,
// Current filtering level on reception.
// Only allowed to be changed if phy is not operational.
//
    pub filtering: ieee802154_filtering_level,
    pub __aligned(NETDEV_ALIGN): char priv[],
}

extern "C" {
    pub fn read_pnet(_arg: &wpan_phy->_net) -> return;
}
//
// struct ieee802154_addr - IEEE802.15.4 device address
// @mode: Address mode from frame header. Can be one of:
// - @IEEE802154_ADDR_NONE
// - @IEEE802154_ADDR_SHORT
// - @IEEE802154_ADDR_LONG
// @pan_id: The PAN ID this address belongs to
// @short_addr: address if @mode is @IEEE802154_ADDR_SHORT
// @extended_addr: address if @mode is @IEEE802154_ADDR_LONG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_addr {
    pub mode: u8,
    pub pan_id: __le16,
    pub short_addr: __le16,
    pub extended_addr: __le64,
}

//
// struct ieee802154_coord_desc - Coordinator descriptor
// @addr: PAN ID and coordinator address
// @page: page this coordinator is using
// @channel: channel this coordinator is using
// @superframe_spec: SuperFrame specification as received
// @link_quality: link quality indicator at which the beacon was received
// @gts_permit: the coordinator accepts GTS requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_coord_desc {
    pub addr: ieee802154_addr,
    pub page: u8,
    pub channel: u8,
    pub superframe_spec: u16,
    pub link_quality: u8,
    pub gts_permit: bool,
}

//
// struct ieee802154_pan_device - PAN device information
// @pan_id: the PAN ID of this device
// @mode: the preferred mode to reach the device
// @short_addr: the short address of this device
// @extended_addr: the extended address of this device
// @node: the list node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_pan_device {
    pub pan_id: __le16,
    pub mode: u8,
    pub short_addr: __le16,
    pub extended_addr: __le64,
    pub node: list_head,
}

//
// struct cfg802154_scan_request - Scan request
//
// @type: type of scan to be performed
// @page: page on which to perform the scan
// @channels: channels in te %page to be scanned
// @duration: time spent on each channel, calculated with:
// aBaseSuperframeDuration * (2 ^ duration + 1)
// @wpan_dev: the wpan device on which to perform the scan
// @wpan_phy: the wpan phy on which to perform the scan
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg802154_scan_request {
    pub type: nl802154_scan_types,
    pub page: u8,
    pub channels: u32,
    pub duration: u8,
    pub wpan_dev: *mut wpan_dev,
    pub wpan_phy: *mut wpan_phy,
}

//
// struct cfg802154_beacon_request - Beacon request descriptor
//
// @interval: interval n between sendings, in multiple order of the super frame
// duration: aBaseSuperframeDuration * (2^n) unless the interval
// order is greater or equal to 15, in this case beacons won't be
// passively sent out at a fixed rate but instead inform the device
// that it should answer beacon requests as part of active scan
// procedures
// @wpan_dev: the concerned wpan device
// @wpan_phy: the wpan phy this was for
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg802154_beacon_request {
    pub interval: u8,
    pub wpan_dev: *mut wpan_dev,
    pub wpan_phy: *mut wpan_phy,
}

//
// struct cfg802154_mac_pkt - MAC packet descriptor (beacon/command)
// @node: MAC packets to process list member
// @skb: the received sk_buff
// @sdata: the interface on which @skb was received
// @page: page configuration when @skb was received
// @channel: channel configuration when @skb was received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cfg802154_mac_pkt {
    pub node: list_head,
    pub skb: *mut sk_buff,
    pub sdata: *mut ieee802154_sub_if_data,
    pub page: u8,
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_key_id {
    pub mode: u8,
    pub id: u8,
    pub device_addr: ieee802154_addr,
    pub short_source: __le32,
    pub extended_source: __le64,
}

pub const IEEE802154_LLSEC_KEY_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_key {
    pub frame_types: u8,
    pub cmd_frame_ids: u32,
// TODO replace with NL802154_KEY_SIZE
    pub key: [u8; IEEE802154_LLSEC_KEY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_key_entry {
    pub list: list_head,
    pub rcu: rcu_head,
    pub id: ieee802154_llsec_key_id,
    pub key: *mut ieee802154_llsec_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_params {
    pub enabled: bool,
    pub frame_counter: __be32,
    pub out_level: u8,
    pub out_key: ieee802154_llsec_key_id,
    pub default_key_source: __le64,
    pub pan_id: __le16,
    pub hwaddr: __le64,
    pub coord_hwaddr: __le64,
    pub coord_shortaddr: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_table {
    pub keys: list_head,
    pub devices: list_head,
    pub security_levels: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_seclevel {
    pub list: list_head,
    pub frame_type: u8,
    pub cmd_frame_id: u8,
    pub device_override: bool,
    pub sec_levels: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_device {
    pub list: list_head,
    pub pan_id: __le16,
    pub short_addr: __le16,
    pub hwaddr: __le64,
    pub frame_counter: u32,
    pub seclevel_exempt: bool,
    pub key_mode: u8,
    pub keys: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_llsec_device_key {
    pub list: list_head,
    pub key_id: ieee802154_llsec_key_id,
    pub frame_counter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpan_dev_header_ops {
// TODO create callback currently assumes ieee802154_mac_cb inside
// skb->cb. This should be changed to give these information as
// parameter.
//
    pub len): c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpan_dev {
    pub wpan_phy: *mut wpan_phy,
    pub iftype: c_int,
// the remainder of this struct should be private to cfg802154
    pub list: list_head,
    pub netdev: *mut net_device,
    pub header_ops: *const wpan_dev_header_ops,
// lowpan interface, set when the wpan_dev belongs to one lowpan_dev
    pub lowpan_dev: *mut net_device,
    pub identifier: u32,
// MAC PIB
    pub pan_id: __le16,
    pub short_addr: __le16,
    pub extended_addr: __le64,
// MAC BSN field
    pub bsn: core::sync::atomic::AtomicI32,
// MAC DSN field
    pub dsn: core::sync::atomic::AtomicI32,
    pub min_be: u8,
    pub max_be: u8,
    pub csma_retries: u8,
    pub frame_retries: i8,
    pub lbt: bool,
// fallback for acknowledgment bit setting
    pub ackreq: bool,
// Associations
    pub association_lock: mutex,
    pub parent: *mut ieee802154_pan_device,
    pub children: list_head,
    pub max_associations: c_uint,
    pub nchildren: c_uint,
}

extern "C" {
    pub fn wpan_phy_register(phy: *mut wpan_phy) -> c_int;
}
extern "C" {
    pub fn wpan_phy_unregister(phy: *mut wpan_phy);
}
extern "C" {
    pub fn wpan_phy_free(phy: *mut wpan_phy);
}
// Same semantics as for class_for_each_device
extern "C" {
    pub fn wpan_phy_for_each(phy: *mut *mut int (fn)(struct wpan_phy, data): *mut c_void, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dev_name(_arg: &phy->dev) -> return;
}
//
// cfg802154_device_is_associated - Checks whether we are associated to any device
// @wpan_dev: the wpan device
// @return: true if we are associated
//
extern "C" {
    pub fn cfg802154_device_is_associated(wpan_dev: *mut wpan_dev) -> bool;
}
//
// cfg802154_device_is_parent - Checks if a device is our coordinator
// @wpan_dev: the wpan device
// @target: the expected parent
// @return: true if @target is our coordinator
//
// cfg802154_device_is_child - Checks whether a device is associated to us
// @wpan_dev: the wpan device
// @target: the expected child
// @return: the PAN device
//
// cfg802154_set_max_associations - Limit the number of future associations
// @wpan_dev: the wpan device
// @max: the maximum number of devices we accept to associate
// @return: the old maximum value
//
// cfg802154_get_free_short_addr - Get a free address among the known devices
// @wpan_dev: the wpan device
// @return: a random short address expectedly unused on our PAN
//
extern "C" {
    pub fn cfg802154_get_free_short_addr(wpan_dev: *mut wpan_dev) -> __le16;
}
