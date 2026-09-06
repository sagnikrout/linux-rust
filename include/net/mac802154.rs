//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mac802154.h
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
// IEEE802.15.4-2003 specification
//
// Copyright (C) 2007-2012 Siemens AG
//

//
// enum ieee802154_hw_addr_filt_flags - hardware address filtering flags
//
// The following flags are used to indicate changed address settings from
// the stack to the hardware.
//
// @IEEE802154_AFILT_SADDR_CHANGED: Indicates that the short address will be
// change.
//
// @IEEE802154_AFILT_IEEEADDR_CHANGED: Indicates that the extended address
// will be change.
//
// @IEEE802154_AFILT_PANID_CHANGED: Indicates that the pan id will be change.
//
// @IEEE802154_AFILT_PANC_CHANGED: Indicates that the address filter will
// do frame address filtering as a pan coordinator.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_hw_addr_filt_flags {
    IEEE802154_AFILT_SADDR_CHANGED		= BIT(0),
    IEEE802154_AFILT_IEEEADDR_CHANGED	= BIT(1),
    IEEE802154_AFILT_PANID_CHANGED		= BIT(2),
    IEEE802154_AFILT_PANC_CHANGED		= BIT(3),
}

//
// struct ieee802154_hw_addr_filt - hardware address filtering settings
//
// @pan_id: pan_id which should be set to the hardware address filter.
//
// @short_addr: short_addr which should be set to the hardware address filter.
//
// @ieee_addr: extended address which should be set to the hardware address
// filter.
//
// @pan_coord: boolean if hardware filtering should be operate as coordinator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_hw_addr_filt {
    pub pan_id: __le16,
    pub short_addr: __le16,
    pub ieee_addr: __le64,
    pub pan_coord: bool,
}

//
// struct ieee802154_hw - ieee802154 hardware
//
// @extra_tx_headroom: headroom to reserve in each transmit skb for use by the
// driver (e.g. for transmit headers.)
//
// @flags: hardware flags, see &enum ieee802154_hw_flags
//
// @parent: parent device of the hardware.
//
// @priv: pointer to private area that was allocated for driver use along with
// this structure.
//
// @phy: This points to the &struct wpan_phy allocated for this 802.15.4 PHY.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_hw {
// filled by the driver
    pub extra_tx_headroom: c_int,
    pub flags: u32,
    pub parent: *mut device,
    pub priv: *mut c_void,
// filled by mac802154 core
    pub phy: *mut wpan_phy,
}

//
// enum ieee802154_hw_flags - hardware flags
//
// These flags are used to indicate hardware capabilities to
// the stack. Generally, flags here should have their meaning
// done in a way that the simplest hardware doesn't need setting
// any particular flags. There are some exceptions to this rule,
// however, so you are advised to review these flags carefully.
//
// @IEEE802154_HW_TX_OMIT_CKSUM: Indicates that xmitter will add FCS on it's
// own.
//
// @IEEE802154_HW_LBT: Indicates that transceiver will support listen before
// transmit.
//
// @IEEE802154_HW_CSMA_PARAMS: Indicates that transceiver will support csma
// parameters (max_be, min_be, backoff exponents).
//
// @IEEE802154_HW_FRAME_RETRIES: Indicates that transceiver will support ARET
// frame retries setting.
//
// @IEEE802154_HW_AFILT: Indicates that transceiver will support hardware
// address filter setting.
//
// @IEEE802154_HW_PROMISCUOUS: Indicates that transceiver will support
// promiscuous mode setting.
//
// @IEEE802154_HW_RX_OMIT_CKSUM: Indicates that receiver omits FCS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_hw_flags {
    IEEE802154_HW_TX_OMIT_CKSUM	= BIT(0),
    IEEE802154_HW_LBT		= BIT(1),
    IEEE802154_HW_CSMA_PARAMS	= BIT(2),
    IEEE802154_HW_FRAME_RETRIES	= BIT(3),
    IEEE802154_HW_AFILT		= BIT(4),
    IEEE802154_HW_PROMISCUOUS	= BIT(5),
    IEEE802154_HW_RX_OMIT_CKSUM	= BIT(6),
}

// Indicates that receiver omits FCS and xmitter will add FCS on it's own.

// struct ieee802154_ops - callbacks from mac802154 to the driver
//
// This structure contains various callbacks that the driver may
// handle or, in some cases, must handle, for example to transmit
// a frame.
//
// start: Handler that 802.15.4 module calls for device initialization.
// This function is called before the first interface is attached.
//
// stop:  Handler that 802.15.4 module calls for device cleanup.
// This function is called after the last interface is removed.
//
// xmit_sync:
// Handler that 802.15.4 module calls for each transmitted frame.
// skb contains the buffer starting from the IEEE 802.15.4 header.
// The low-level driver should send the frame based on available
// configuration. This is called by a workqueue and useful for
// synchronous 802.15.4 drivers.
// This function should return zero or negative errno.
//
// WARNING:
// This will be deprecated soon. We don't accept synced xmit callbacks
// drivers anymore.
//
// xmit_async:
// Handler that 802.15.4 module calls for each transmitted frame.
// skb contains the buffer starting from the IEEE 802.15.4 header.
// The low-level driver should send the frame based on available
// configuration.
// This function should return zero or negative errno.
//
// ed:    Handler that 802.15.4 module calls for Energy Detection.
// This function should place the value for detected energy
// (usually device-dependant) in the level pointer and return
// either zero or negative errno. Called with pib_lock held.
//
// set_channel:
// Set radio for listening on specific channel.
// Set the device for listening on specified channel.
// Returns either zero, or negative errno. Called with pib_lock held.
//
// set_hw_addr_filt:
// Set radio for listening on specific address.
// Set the device for listening on specified address.
// Returns either zero, or negative errno.
//
// set_txpower:
// Set radio transmit power in mBm. Called with pib_lock held.
// Returns either zero, or negative errno.
//
// set_lbt
// Enables or disables listen before talk on the device. Called with
// pib_lock held.
// Returns either zero, or negative errno.
//
// set_cca_mode
// Sets the CCA mode used by the device. Called with pib_lock held.
// Returns either zero, or negative errno.
//
// set_cca_ed_level
// Sets the CCA energy detection threshold in mBm. Called with pib_lock
// held.
// Returns either zero, or negative errno.
//
// set_csma_params
// Sets the CSMA parameter set for the PHY. Called with pib_lock held.
// Returns either zero, or negative errno.
//
// set_frame_retries
// Sets the retransmission attempt limit. Called with pib_lock held.
// Returns either zero, or negative errno.
//
// set_promiscuous_mode
// Enables or disable promiscuous mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee802154_ops {
    pub owner: *mut module,
    pub hw): *mut *mut int (start)(struct ieee802154_hw,
    pub hw): *mut *mut void (stop)(struct ieee802154_hw,
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,
    pub level): *mut *mut *mut int (ed)(struct ieee802154_hw hw, u8,
    pub channel): u8,
    pub changed): c_ulong,
    pub mbm): *mut *mut *mut int (set_txpower)(struct ieee802154_hw hw, s32,
    pub on): *mut *mut *mut int (set_lbt)(struct ieee802154_hw hw, bool,
    pub cca): *const wpan_phy_cca,
    pub mbm): *mut *mut *mut int (set_cca_ed_level)(struct ieee802154_hw hw, s32,
    pub retries): u8 min_be, u8 max_be, u8,
    pub retries): i8,
    pub on): bool,
}

//
// ieee802154_get_fc_from_skb - get the frame control field from an skb
// @skb: skb where the frame control field will be get from
//
// check if we can fc at skb_mac_header of sk buffer
extern "C" {
    pub fn cpu_to_le16(_arg: 0) -> return;
}
//
// ieee802154_skb_dst_pan - get the pointer to destination pan field
// @fc: mac header frame control field
// @skb: skb where the destination pan pointer will be get from
//
// ieee802154_skb_src_pan - get the pointer to source pan field
// @fc: mac header frame control field
// @skb: skb where the source pan pointer will be get from
//
// if intra-pan and source addr mode is non none,
// then source pan id is equal destination pan id.
//
// ieee802154_skb_is_intra_pan_addressing - checks whenever the mac addressing
// is an intra pan communication
// @fc: mac header frame control field
// @skb: skb where the source and destination pan should be get from
//
// src_pan = ieee802154_skb_src_pan(fc, skb);
// if one is NULL is no intra pan addressing
//
// ieee802154_be64_to_le64 - copies and convert be64 to le64
// @le64_dst: le64 destination pointer
// @be64_src: be64 source pointer
//
// ieee802154_le64_to_be64 - copies and convert le64 to be64
// @be64_dst: be64 destination pointer
// @le64_src: le64 source pointer
//
// ieee802154_le16_to_be16 - copies and convert le16 to be16
// @be16_dst: be16 destination pointer
// @le16_src: le16 source pointer
//
// ieee802154_be16_to_le16 - copies and convert be16 to le16
// @le16_dst: le16 destination pointer
// @be16_src: be16 source pointer
//
// ieee802154_alloc_hw - Allocate a new hardware device
//
// This must be called once for each hardware device. The returned pointer
// must be used to refer to this device when calling other functions.
// mac802154 allocates a private data area for the driver pointed to by
// @priv in &struct ieee802154_hw, the size of this area is given as
// @priv_data_len.
//
// @priv_data_len: length of private data
// @ops: callbacks for this device
//
// Return: A pointer to the new hardware device, or %NULL on error.
//
// ieee802154_free_hw - free hardware descriptor
//
// This function frees everything that was allocated, including the
// private data for the driver. You must call ieee802154_unregister_hw()
// before calling this function.
//
// @hw: the hardware to free
//
extern "C" {
    pub fn ieee802154_free_hw(hw: *mut ieee802154_hw);
}
//
// ieee802154_register_hw - Register hardware device
//
// You must call this function before any other functions in
// mac802154. Note that before a hardware can be registered, you
// need to fill the contained wpan_phy's information.
//
// @hw: the device to register as returned by ieee802154_alloc_hw()
//
// Return: 0 on success. An error code otherwise.
//
extern "C" {
    pub fn ieee802154_register_hw(hw: *mut ieee802154_hw) -> c_int;
}
//
// ieee802154_unregister_hw - Unregister a hardware device
//
// This function instructs mac802154 to free allocated resources
// and unregister netdevices from the networking subsystem.
//
// @hw: the hardware to unregister
//
extern "C" {
    pub fn ieee802154_unregister_hw(hw: *mut ieee802154_hw);
}
//
// ieee802154_rx_irqsafe - receive frame
//
// Like ieee802154_rx() but can be called in IRQ context
// (internally defers to a tasklet.)
//
// @hw: the hardware this frame came in on
// @skb: the buffer to receive, owned by mac802154 after this call
// @lqi: link quality indicator
//
// ieee802154_xmit_complete - frame transmission complete
//
// @hw: pointer as obtained from ieee802154_alloc_hw().
// @skb: buffer for transmission
// @ifs_handling: indicate interframe space handling
//
// ieee802154_xmit_error - offloaded frame transmission failed
//
// @hw: pointer as obtained from ieee802154_alloc_hw().
// @skb: buffer for transmission
// @reason: error code
//
// ieee802154_xmit_hw_error - frame could not be offloaded to the transmitter
// because of a hardware error (bus error, timeout, etc)
//
// @hw: pointer as obtained from ieee802154_alloc_hw().
// @skb: buffer for transmission
//
extern "C" {
    pub fn ieee802154_xmit_hw_error(hw: *mut ieee802154_hw, skb: *mut sk_buff);
}
