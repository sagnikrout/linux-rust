//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/psp/types.h
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

pub const PSP_DEFAULT_UDP_PORT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psphdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub crypt_offset: u8,
    pub verfl: u8,
    pub spi: __be32,
    pub iv: __be64,
    pub /: *mut *mut __be64 vc[]; / optional,
}

//
// struct psp_dev_config - PSP device configuration
// @versions: PSP versions enabled on the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_dev_config {
    pub versions: u32,
}

// Max number of devices that can be associated with a single PSP device.
// Each entry consumes ~24 bytes in the netlink dev-get response, and the
// response must fit in GENLMSG_DEFAULT_SIZE (~3.7KB).
//
pub const PSP_ASSOC_DEV_MAX: c_int = 128;
//
// struct psp_assoc_dev - wrapper for associated net_device
// @dev_list: list node for psp_dev::assoc_dev_list
// @assoc_dev: the associated net_device
// @dev_tracker: tracker for the net_device reference
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_assoc_dev {
    pub dev_list: list_head,
    pub assoc_dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
}

//
// struct psp_dev - PSP device struct
// @main_netdev: original netdevice of this PSP device
// @assoc_dev_list: list of psp_assoc_dev entries associated with this PSP device
// @assoc_dev_cnt: number of entries in @assoc_dev_list
// @ops:	driver callbacks
// @caps:	device capabilities
// @drv_priv:	driver priv pointer
// @lock:	instance lock, protects all fields
// @refcnt:	reference count for the instance
// @id:		instance id
// @generation:	current generation of the device key
// @config:	current device configuration
// @active_assocs:	list of registered associations
// @prev_assocs:	associations which use old (but still usable)
// device key
// @stale_assocs:	associations which use a rotated out key
//
// @stats:	statistics maintained by the core
// @stats.rotations:	See stats attr key-rotations
// @stats.stales:	See stats attr stale-events
//
// @rcu:	RCU head for freeing the structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_dev {
    pub main_netdev: *mut net_device,
    pub assoc_dev_list: list_head,
    pub assoc_dev_cnt: c_int,
    pub ops: *mut psp_dev_ops,
    pub caps: *mut psp_dev_caps,
    pub drv_priv: *mut c_void,
    pub lock: mutex,
    pub refcnt: refcount_t,
    pub id: u32,
    pub generation: u8,
    pub config: psp_dev_config,
    pub active_assocs: list_head,
    pub prev_assocs: list_head,
    pub stale_assocs: list_head,
    pub rotations: c_ulong,
    pub stales: c_ulong,
    pub stats: },
    pub rcu: rcu_head,
}

pub const PSP_GEN_VALID_MASK: c_uint = 0x7f;
//
// struct psp_dev_caps - PSP device capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_dev_caps {
//
// @versions: mask of supported PSP versions
// Set this field to 0 to indicate PSP is not supported at all.
//
    pub versions: u32,
//
// @assoc_drv_spc: size of driver-specific state in Tx assoc
// Determines the size of struct psp_assoc::drv_data
//
    pub assoc_drv_spc: u32,
}

pub const PSP_MAX_KEY: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_skb_ext {
    pub spi: __be32,
    pub dev_id: u16,
    pub generation: u8,
    pub version: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_key_parsed {
    pub spi: __be32,
    pub key: [u8; PSP_MAX_KEY],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_assoc {
    pub psd: *mut psp_dev,
    pub dev_id: u16,
    pub generation: u8,
    pub version: u8,
    pub peer_tx: u8,
    pub upgrade_seq: u32,
    pub tx: psp_key_parsed,
    pub rx: psp_key_parsed,
    pub refcnt: refcount_t,
    pub rcu: rcu_head,
    pub work: work_struct,
    pub assocs_list: list_head,
    pub __aligned(8): u8 drv_data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_dev_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_auth_fail: u64,
    pub rx_error: u64,
    pub rx_bad: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_error: u64,
}

//
// struct psp_dev_ops - netdev driver facing PSP callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_dev_ops {
//
// @set_config: set configuration of a PSP device
// Driver can inspect @psd->config for the previous configuration.
// Core will update @psd->config with @config on success.
//
    pub extack): *mut netlink_ext_ack,
//
// @key_rotate: rotate the device key
//
    pub extack): *mut *mut *mut int (key_rotate)(struct psp_dev psd, struct netlink_ext_ack,
//
// @rx_spi_alloc: allocate an Rx SPI+key pair
// Allocate an Rx SPI and resulting derived key.
// This key should remain valid until key rotation.
//
    pub extack): *mut netlink_ext_ack,
//
// @tx_key_add: add a Tx key to the device
// Install an association in the device. Core will allocate space
// for the driver to use at drv_data.
//
    pub extack): *mut netlink_ext_ack,
//
// @tx_key_del: remove a Tx key from the device
// Remove an association from the device.
//
    pub pas): *mut *mut *mut void (tx_key_del)(struct psp_dev psd, struct psp_assoc,
//
// @get_stats: get statistics from the device
// Stats required by the spec must be maintained and filled in.
// Stats must be filled in member-by-member, never memset the struct.
//
    pub stats): *mut *mut *mut void (get_stats)(struct psp_dev psd, struct psp_dev_stats,
}
