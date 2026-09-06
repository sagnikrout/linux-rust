//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/macsec.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// MACsec netdev header, used for h/w accelerated implementations.
//
// Copyright (c) 2015 Sabrina Dubroca <sd@queasysnail.net>
//

pub const MACSEC_DEFAULT_PN_LEN: c_int = 4;
pub const MACSEC_XPN_PN_LEN: c_int = 8;

pub const MACSEC_SCI_LEN: c_int = 8;

pub const MACSEC_TCI_VERSION: c_uint = 0x80;
pub const MACSEC_TCI_ES: c_uint = 0x40 /* end station */;
pub const MACSEC_TCI_SC: c_uint = 0x20 /* SCI present */;
pub const MACSEC_TCI_SCB: c_uint = 0x10 /* epon */;
pub const MACSEC_TCI_E: c_uint = 0x08 /* encryption */;
pub const MACSEC_TCI_C: c_uint = 0x04 /* changed text */;
pub const MACSEC_AN_MASK: c_uint = 0x03 /* association number */;

pub const MACSEC_DEFAULT_ICV_LEN: c_int = 16;
pub type sci_t = u64 ;
pub type ssci_t = u32 ;

//
// struct macsec_key - SA key
// @id: user-provided key identifier
// @tfm: crypto struct, key storage
// @salt: salt used to generate IV in XPN cipher suites
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_key {
    pub id: [u8; MACSEC_KEYID_LEN],
    pub tfm: *mut crypto_aead,
    pub salt: salt_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_rx_sc_stats {
    pub InOctetsValidated: __u64,
    pub InOctetsDecrypted: __u64,
    pub InPktsUnchecked: __u64,
    pub InPktsDelayed: __u64,
    pub InPktsOK: __u64,
    pub InPktsInvalid: __u64,
    pub InPktsLate: __u64,
    pub InPktsNotValid: __u64,
    pub InPktsNotUsingSA: __u64,
    pub InPktsUnusedSA: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_rx_sa_stats {
    pub InPktsOK: __u32,
    pub InPktsInvalid: __u32,
    pub InPktsNotValid: __u32,
    pub InPktsNotUsingSA: __u32,
    pub InPktsUnusedSA: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_tx_sa_stats {
    pub OutPktsProtected: __u32,
    pub OutPktsEncrypted: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_tx_sc_stats {
    pub OutPktsProtected: __u64,
    pub OutPktsEncrypted: __u64,
    pub OutOctetsProtected: __u64,
    pub OutOctetsEncrypted: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_dev_stats {
    pub OutPktsUntagged: __u64,
    pub InPktsUntagged: __u64,
    pub OutPktsTooLong: __u64,
    pub InPktsNoTag: __u64,
    pub InPktsBadTag: __u64,
    pub InPktsUnknownSCI: __u64,
    pub InPktsNoSCI: __u64,
    pub InPktsOverrun: __u64,
}

//
// struct macsec_rx_sa - receive secure association
// @active:
// @next_pn: packet number expected for the next packet
// @lock: protects next_pn manipulations
// @key: key structure
// @ssci: short secure channel identifier
// @stats: per-SA stats
// @destroy_work: deferred work to free the SA in process context after RCU grace period
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_rx_sa {
    pub key: macsec_key,
    pub ssci: ssci_t,
    pub lock: spinlock_t,
    pub next_pn_halves: pn_t,
    pub next_pn: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_rx_sc_stats {
    pub stats: macsec_rx_sc_stats,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_tx_sc_stats {
    pub stats: macsec_tx_sc_stats,
    pub syncp: u64_stats_sync,
}

//
// struct macsec_rx_sc - receive secure channel
// @sci: secure channel identifier for this SC
// @active: channel is active
// @sa: array of secure associations
// @stats: per-SC stats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_rx_sc {
    pub next: *mut macsec_rx_sc __rcu,
    pub sci: sci_t,
    pub active: bool,
    pub sa: [*mut macsec_rx_sa __rcu; MACSEC_NUM_AN],
    pub stats: *mut pcpu_rx_sc_stats __percpu,
    pub refcnt: refcount_t,
    pub rcu_head: rcu_head,
}

//
// struct macsec_tx_sa - transmit secure association
// @active:
// @next_pn: packet number to use for the next packet
// @lock: protects next_pn manipulations
// @key: key structure
// @ssci: short secure channel identifier
// @stats: per-SA stats
// @destroy_work: deferred work to free the SA in process context after RCU grace period
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_tx_sa {
    pub key: macsec_key,
    pub ssci: ssci_t,
    pub lock: spinlock_t,
    pub next_pn_halves: pn_t,
    pub next_pn: u64,
}

//
// struct macsec_tx_sc - transmit secure channel
// @active:
// @encoding_sa: association number of the SA currently in use
// @encrypt: encrypt packets on transmit, or authenticate only
// @send_sci: always include the SCI in the SecTAG
// @end_station:
// @scb: single copy broadcast flag
// @sa: array of secure associations
// @stats: stats for this TXSC
// @md_dst: MACsec offload metadata dst
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_tx_sc {
    pub active: bool,
    pub encoding_sa: u8,
    pub encrypt: bool,
    pub send_sci: bool,
    pub end_station: bool,
    pub scb: bool,
    pub sa: [*mut macsec_tx_sa __rcu; MACSEC_NUM_AN],
    pub stats: *mut pcpu_tx_sc_stats __percpu,
    pub md_dst: *mut metadata_dst,
}

//
// struct macsec_secy - MACsec Security Entity
// @netdev: netdevice for this SecY
// @n_rx_sc: number of receive secure channels configured on this SecY
// @sci: secure channel identifier used for tx
// @key_len: length of keys used by the cipher suite
// @icv_len: length of ICV used by the cipher suite
// @validate_frames: validation mode
// @xpn: enable XPN for this SecY
// @operational: MAC_Operational flag
// @protect_frames: enable protection for this SecY
// @replay_protect: enable packet number checks on receive
// @replay_window: size of the replay window
// @tx_sc: transmit secure channel
// @rx_sc: linked list of receive secure channels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_secy {
    pub netdev: *mut net_device,
    pub n_rx_sc: c_uint,
    pub sci: sci_t,
    pub key_len: u16,
    pub icv_len: u16,
    pub validate_frames: macsec_validation_type,
    pub xpn: bool,
    pub operational: bool,
    pub protect_frames: bool,
    pub replay_protect: bool,
    pub replay_window: u32,
    pub tx_sc: macsec_tx_sc,
    pub rx_sc: *mut macsec_rx_sc __rcu,
}

//
// struct macsec_context - MACsec context for hardware offloading
// @netdev: a valid pointer to a struct net_device if @offload ==
// MACSEC_OFFLOAD_MAC
// @phydev: a valid pointer to a struct phy_device if @offload ==
// MACSEC_OFFLOAD_PHY
// @offload: MACsec offload status
// @secy: pointer to a MACsec SecY
// @rx_sc: pointer to a RX SC
// @update_pn: when updating the SA, update the next PN
// @assoc_num: association number of the target SA
// @key: key of the target SA
// @rx_sa: pointer to an RX SA if a RX SA is added/updated/removed
// @tx_sa: pointer to an TX SA if a TX SA is added/updated/removed
// @tx_sc_stats: pointer to TX SC stats structure
// @tx_sa_stats: pointer to TX SA stats structure
// @rx_sc_stats: pointer to RX SC stats structure
// @rx_sa_stats: pointer to RX SA stats structure
// @dev_stats: pointer to dev stats structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_context {
    pub netdev: *mut net_device,
    pub phydev: *mut phy_device,
}

//
// struct macsec_ops - MACsec offloading operations
// @mdo_dev_open: called when the MACsec interface transitions to the up state
// @mdo_dev_stop: called when the MACsec interface transitions to the down
// state
// @mdo_add_secy: called when a new SecY is added
// @mdo_upd_secy: called when the SecY flags are changed or the MAC address of
// the MACsec interface is changed
// @mdo_del_secy: called when the hw offload is disabled or the MACsec
// interface is removed
// @mdo_add_rxsc: called when a new RX SC is added
// @mdo_upd_rxsc: called when a certain RX SC is updated
// @mdo_del_rxsc: called when a certain RX SC is removed
// @mdo_add_rxsa: called when a new RX SA is added
// @mdo_upd_rxsa: called when a certain RX SA is updated
// @mdo_del_rxsa: called when a certain RX SA is removed
// @mdo_add_txsa: called when a new TX SA is added
// @mdo_upd_txsa: called when a certain TX SA is updated
// @mdo_del_txsa: called when a certain TX SA is removed
// @mdo_get_dev_stats: called when dev stats are read
// @mdo_get_tx_sc_stats: called when TX SC stats are read
// @mdo_get_tx_sa_stats: called when TX SA stats are read
// @mdo_get_rx_sc_stats: called when RX SC stats are read
// @mdo_get_rx_sa_stats: called when RX SA stats are read
// @mdo_insert_tx_tag: called to insert the TX tag
// @needed_headroom: number of bytes reserved at the beginning of the sk_buff
// for the TX tag
// @needed_tailroom: number of bytes reserved at the end of the sk_buff for the
// TX tag
// @rx_uses_md_dst: whether MACsec device offload supports sk_buff md_dst
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct macsec_ops {
// Device wide
    pub ctx): *mut *mut int (mdo_dev_open)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_dev_stop)(struct macsec_context,
// SecY
    pub ctx): *mut *mut int (mdo_add_secy)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_upd_secy)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_del_secy)(struct macsec_context,
// Security channels
    pub ctx): *mut *mut int (mdo_add_rxsc)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_upd_rxsc)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_del_rxsc)(struct macsec_context,
// Security associations
    pub ctx): *mut *mut int (mdo_add_rxsa)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_upd_rxsa)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_del_rxsa)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_add_txsa)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_upd_txsa)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_del_txsa)(struct macsec_context,
// Statistics
    pub ctx): *mut *mut int (mdo_get_dev_stats)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_get_tx_sc_stats)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_get_tx_sa_stats)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_get_rx_sc_stats)(struct macsec_context,
    pub ctx): *mut *mut int (mdo_get_rx_sa_stats)(struct macsec_context,
// Offload tag
    pub skb): *mut sk_buff,
    pub needed_headroom: c_uint,
    pub needed_tailroom: c_uint,
    pub rx_uses_md_dst: bool,
}

extern "C" {
    pub fn macsec_pn_wrapped(secy: *mut macsec_secy, tx_sa: *mut macsec_tx_sa);
}
extern "C" {
    pub fn macsec_netdev_is_offloaded(dev: *mut net_device) -> bool;
}

extern "C" {
    pub fn netdev_priv(_arg: vlan_dev_priv(dev)->real_dev) -> return;
}

extern "C" {
    pub fn netdev_priv(_arg: dev) -> return;
}
extern "C" {
    pub fn be64_to_cpu(__be64)sci: () -> return;
}
