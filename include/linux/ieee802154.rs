//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee802154.h
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
// Copyright (C) 2007, 2008 Siemens AG
//
// Written by:
// Pavel Smolenskiy <pavel.smolenskiy@gmail.com>
// Maxim Gorbachyov <maxim.gorbachev@siemens.com>
// Maxim Osipov <maxim.osipov@siemens.com>
// Dmitry Eremin-Solenikov <dbaryshkov@gmail.com>
// Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//

pub const IEEE802154_MTU: c_int = 127;
pub const IEEE802154_ACK_PSDU_LEN: c_int = 5;
pub const IEEE802154_MIN_PSDU_LEN: c_int = 9;
pub const IEEE802154_FCS_LEN: c_int = 2;
pub const IEEE802154_MAX_AUTH_TAG_LEN: c_int = 16;
pub const IEEE802154_FC_LEN: c_int = 2;
pub const IEEE802154_SEQ_LEN: c_int = 1;
// General MAC frame format:
// 2 bytes: Frame Control
// 1 byte:  Sequence Number
// 20 bytes: Addressing fields
// 14 bytes: Auxiliary Security Header
//

pub const IEEE802154_PAN_ID_BROADCAST: c_uint = 0xffff;
pub const IEEE802154_ADDR_SHORT_BROADCAST: c_uint = 0xffff;
pub const IEEE802154_ADDR_SHORT_UNSPEC: c_uint = 0xfffe;
pub const IEEE802154_EXTENDED_ADDR_LEN: c_int = 8;
pub const IEEE802154_SHORT_ADDR_LEN: c_int = 2;
pub const IEEE802154_PAN_ID_LEN: c_int = 2;
// Duration in superframe order
pub const IEEE802154_MAX_SCAN_DURATION: c_int = 14;
pub const IEEE802154_ACTIVE_SCAN_DURATION: c_int = 15;
// Superframe duration in slots
pub const IEEE802154_SUPERFRAME_PERIOD: c_int = 16;
// Various periods expressed in symbols
pub const IEEE802154_SLOT_PERIOD: c_int = 60;
pub const IEEE802154_LIFS_PERIOD: c_int = 40;
pub const IEEE802154_SIFS_PERIOD: c_int = 12;
pub const IEEE802154_MAX_SIFS_FRAME_SIZE: c_int = 18;
pub const IEEE802154_MAX_CHANNEL: c_int = 26;
pub const IEEE802154_MAX_PAGE: c_int = 31;
pub const IEEE802154_FC_TYPE_BEACON: c_uint = 0x0	/* Frame is beacon */;
pub const IEEE802154_FC_TYPE_DATA: c_uint = 0x1	/* Frame is data */;
pub const IEEE802154_FC_TYPE_ACK: c_uint = 0x2	/* Frame is acknowledgment */;
pub const IEEE802154_FC_TYPE_MAC_CMD: c_uint = 0x3	/* Frame is MAC command */;
pub const IEEE802154_FC_TYPE_SHIFT: c_int = 0;

pub const IEEE802154_FC_SECEN_SHIFT: c_int = 3;

pub const IEEE802154_FC_FRPEND_SHIFT: c_int = 4;

pub const IEEE802154_FC_ACK_REQ_SHIFT: c_int = 5;

pub const IEEE802154_FC_INTRA_PAN_SHIFT: c_int = 6;

pub const IEEE802154_FC_SAMODE_SHIFT: c_int = 14;

pub const IEEE802154_FC_DAMODE_SHIFT: c_int = 10;

pub const IEEE802154_FC_VERSION_SHIFT: c_int = 12;

pub const IEEE802154_SCF_SECLEVEL_MASK: c_int = 7;
pub const IEEE802154_SCF_SECLEVEL_SHIFT: c_int = 0;

pub const IEEE802154_SCF_KEY_ID_MODE_SHIFT: c_int = 3;

pub const IEEE802154_SCF_KEY_IMPLICIT: c_int = 0;
pub const IEEE802154_SCF_KEY_INDEX: c_int = 1;
pub const IEEE802154_SCF_KEY_SHORT_INDEX: c_int = 2;
pub const IEEE802154_SCF_KEY_HW_INDEX: c_int = 3;
pub const IEEE802154_SCF_SECLEVEL_NONE: c_int = 0;
pub const IEEE802154_SCF_SECLEVEL_MIC32: c_int = 1;
pub const IEEE802154_SCF_SECLEVEL_MIC64: c_int = 2;
pub const IEEE802154_SCF_SECLEVEL_MIC128: c_int = 3;
pub const IEEE802154_SCF_SECLEVEL_ENC: c_int = 4;
pub const IEEE802154_SCF_SECLEVEL_ENC_MIC32: c_int = 5;
pub const IEEE802154_SCF_SECLEVEL_ENC_MIC64: c_int = 6;
pub const IEEE802154_SCF_SECLEVEL_ENC_MIC128: c_int = 7;
// MAC footer size

// MAC's Command Frames Identifiers
pub const IEEE802154_CMD_ASSOCIATION_REQ: c_uint = 0x01;
pub const IEEE802154_CMD_ASSOCIATION_RESP: c_uint = 0x02;
pub const IEEE802154_CMD_DISASSOCIATION_NOTIFY: c_uint = 0x03;
pub const IEEE802154_CMD_DATA_REQ: c_uint = 0x04;
pub const IEEE802154_CMD_PANID_CONFLICT_NOTIFY: c_uint = 0x05;
pub const IEEE802154_CMD_ORPHAN_NOTIFY: c_uint = 0x06;
pub const IEEE802154_CMD_BEACON_REQ: c_uint = 0x07;
pub const IEEE802154_CMD_COORD_REALIGN_NOTIFY: c_uint = 0x08;
pub const IEEE802154_CMD_GTS_REQ: c_uint = 0x09;
//
// The return values of MAC operations
//
// The requested operation was completed successfully.
// For a transmission request, this value indicates
// a successful transmission.
//
// The requested operation failed.
// The requested operation has been cancelled.
//
// Device is ready to poll the coordinator for data in a non beacon
// enabled PAN.
//
// Wrong frame counter.
//
// The frame does not conforms to the incoming key usage policy checking
// procedure.
//
// The frame does not conforms to the incoming security level usage
// policy checking procedure.
//
// Secured frame received with an empty Frame Version field.
//
// A secured frame is received or must be sent but security is not
// enabled in the device. Or, the Auxiliary Security Header has security
// level of zero in it.
//
// The beacon was lost following a synchronization request.
//
// A transmission could not take place due to activity on the
// channel, i.e., the CSMA-CA mechanism has failed.
//
// The GTS request has been denied by the PAN coordinator.
// The attempt to disable the transceiver has failed.
//
// The received frame induces a failed security check according to
// the security suite.
//
// The frame resulting from secure processing has a length that is
// greater than aMACMaxFrameSize.
//
// The requested GTS transmission failed because the specified GTS
// either did not have a transmit GTS direction or was not defined.
//
// A request to purge an MSDU from the transaction queue was made using
// an MSDU handle that was not found in the transaction table.
//
// A parameter in the primitive is out of the valid range.
// No acknowledgment was received after aMaxFrameRetries.
// A scan operation failed to find any network beacons.
// No response data were available following a request.
// The operation failed because a short address was not allocated.
//
// A receiver enable request was unsuccessful because it could not be
// completed within the CAP.
//
// A PAN identifier conflict has been detected and communicated to the
// PAN coordinator.
//
// A coordinator realignment command has been received.
// The transaction has expired and its information discarded.
// There is no capacity to store the transaction.
//
// The transceiver was in the transmitter enabled state when the
// receiver was requested to be enabled.
//
// The appropriate key is not available in the ACL.
//
// A SET/GET request was issued with the identifier of a PIB attribute
// that is not supported.
//
// Missing source or destination address or address mode.
//
// MLME asked to turn the receiver on, but the on time duration is too
// big compared to the macBeaconOrder.
//
// MLME asaked to turn the receiver on, but the request was delayed for
// too long before getting processed.
//
// The StartTime parameter is nonzero, and the MLME is not currently
// tracking the beacon of the coordinator through which it is
// associated.
//
// The index inside the hierarchical values in PIBAttribute is out of
// range.
//
// The number of PAN descriptors discovered during a scan has been
// reached.
//
// The PIBAttribute parameter specifies an attribute that is a read-only
// attribute.
//
// A request to perform a scan operation failed because the MLME was
// in the process of performing a previously initiated scan operation.
//
// The outgoing superframe overlaps the incoming superframe.
// Any other error situation.
//
// enum ieee802154_filtering_level - Filtering levels applicable to a PHY
//
// @IEEE802154_FILTERING_NONE: No filtering at all, what is received is
// forwarded to the softMAC
// @IEEE802154_FILTERING_1_FCS: First filtering level, frames with an invalid
// FCS should be dropped
// @IEEE802154_FILTERING_2_PROMISCUOUS: Second filtering level, promiscuous
// mode as described in the spec, identical in terms of filtering to the
// level one on PHY side, but at the MAC level the frame should be
// forwarded to the upper layer directly
// @IEEE802154_FILTERING_3_SCAN: Third filtering level, scan related, where
// only beacons must be processed, all remaining traffic gets dropped
// @IEEE802154_FILTERING_4_FRAME_FIELDS: Fourth filtering level actually
// enforcing the validity of the content of the frame with various checks
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee802154_filtering_level {
    IEEE802154_FILTERING_NONE,
    IEEE802154_FILTERING_1_FCS,
    IEEE802154_FILTERING_2_PROMISCUOUS,
    IEEE802154_FILTERING_3_SCAN,
    IEEE802154_FILTERING_4_FRAME_FIELDS,
}

// frame control handling
pub const IEEE802154_FCTL_FTYPE: c_uint = 0x0003;
pub const IEEE802154_FCTL_ACKREQ: c_uint = 0x0020;
pub const IEEE802154_FCTL_SECEN: c_uint = 0x0004;
pub const IEEE802154_FCTL_INTRA_PAN: c_uint = 0x0040;
pub const IEEE802154_FCTL_DADDR: c_uint = 0x0c00;
pub const IEEE802154_FCTL_SADDR: c_uint = 0xc000;
pub const IEEE802154_FTYPE_DATA: c_uint = 0x0001;
pub const IEEE802154_FCTL_ADDR_NONE: c_uint = 0x0000;
pub const IEEE802154_FCTL_DADDR_SHORT: c_uint = 0x0800;
pub const IEEE802154_FCTL_DADDR_EXTENDED: c_uint = 0x0c00;
pub const IEEE802154_FCTL_SADDR_SHORT: c_uint = 0x8000;
pub const IEEE802154_FCTL_SADDR_EXTENDED: c_uint = 0xc000;
//
// ieee802154_is_data - check if type is IEEE802154_FTYPE_DATA
// @fc: frame control bytes in little-endian byteorder
//
// ieee802154_is_secen - check if Security bit is set
// @fc: frame control bytes in little-endian byteorder
//
// ieee802154_is_ackreq - check if acknowledgment request bit is set
// @fc: frame control bytes in little-endian byteorder
//
// ieee802154_is_intra_pan - check if intra pan id communication
// @fc: frame control bytes in little-endian byteorder
//
// ieee802154_daddr_mode - get daddr mode from fc
// @fc: frame control bytes in little-endian byteorder
//
// ieee802154_saddr_mode - get saddr mode from fc
// @fc: frame control bytes in little-endian byteorder
//
// ieee802154_is_valid_psdu_len - check if psdu len is valid
// available lengths:
// 0-4	Reserved
// 5	MPDU (Acknowledgment)
// 6-8	Reserved
// 9-127	MPDU
//
// @len: psdu len with (MHR + payload + MFR)
//
// ieee802154_is_valid_extended_unicast_addr - check if extended addr is valid
// @addr: extended addr to check
//
// Bail out if the address is all zero, or if the group
// address bit is set.
//
// ieee802154_is_broadcast_short_addr - check if short addr is broadcast
// @addr: short addr to check
//
// ieee802154_is_unspec_short_addr - check if short addr is unspecified
// @addr: short addr to check
//
// ieee802154_is_valid_src_short_addr - check if source short address is valid
// @addr: short addr to check
//
// ieee802154_random_extended_addr - generates a random extended address
// @addr: extended addr pointer to place the random address
//
// clear the group bit, and set the locally administered bit
