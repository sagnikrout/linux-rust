//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/macsec/macsec_struct.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//
// ! Represents the bitfields of a single row in the Egress CTL Filter
// table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_ctlf_record {
// ! This is used to store the 48 bit value used to compare SA, DA or
// halfDA+half SA value.
//
    pub sa_da: [u32; 2],
// ! This is used to store the 16 bit ethertype value used for
// comparison.
//
    pub eth_type: u32,
// ! The match mask is per-nibble. 0 means don't care, i.e. every value
// will match successfully. The total data is 64 bit, i.e. 16 nibbles
// masks.
//
    pub match_mask: u32,
// ! 0: No compare, i.e. This entry is not used
// 1: compare DA only
// 2: compare SA only
// 3: compare half DA + half SA
// 4: compare ether type only
// 5: compare DA + ethertype
// 6: compare SA + ethertype
// 7: compare DA+ range.
//
    pub match_type: u32,
// ! 0: Bypass the remaining modules if matched.
// 1: Forward to next module for more classifications.
//
    pub action: u32,
}

// ! Represents the bitfields of a single row in the Egress Packet
// Classifier table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_class_record {
// ! VLAN ID field.
    pub vlan_id: u32,
// ! VLAN UP field.
    pub vlan_up: u32,
// ! VLAN Present in the Packet.
    pub vlan_valid: u32,
// ! The 8 bit value used to compare with extracted value for byte 3.
    pub byte3: u32,
// ! The 8 bit value used to compare with extracted value for byte 2.
    pub byte2: u32,
// ! The 8 bit value used to compare with extracted value for byte 1.
    pub byte1: u32,
// ! The 8 bit value used to compare with extracted value for byte 0.
    pub byte0: u32,
// ! The 8 bit TCI field used to compare with extracted value.
    pub tci: u32,
// ! The 64 bit SCI field in the SecTAG.
    pub sci: [u32; 2],
// ! The 16 bit Ethertype (in the clear) field used to compare with
// extracted value.
//
    pub eth_type: u32,
// ! This is to specify the 40bit SNAP header if the SNAP header's mask
// is enabled.
//
    pub snap: [u32; 2],
// ! This is to specify the 24bit LLC header if the LLC header's mask is
// enabled.
//
    pub llc: u32,
// ! The 48 bit MAC_SA field used to compare with extracted value.
    pub mac_sa: [u32; 2],
// ! The 48 bit MAC_DA field used to compare with extracted value.
    pub mac_da: [u32; 2],
// ! The 32 bit Packet number used to compare with extracted value.
    pub pn: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte3_location: u32,
// ! 0: don't care
// 1: enable comparison of extracted byte pointed by byte 3 location.
//
    pub byte3_mask: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte2_location: u32,
// ! 0: don't care
// 1: enable comparison of extracted byte pointed by byte 2 location.
//
    pub byte2_mask: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte1_location: u32,
// ! 0: don't care
// 1: enable comparison of extracted byte pointed by byte 1 location.
//
    pub byte1_mask: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte0_location: u32,
// ! 0: don't care
// 1: enable comparison of extracted byte pointed by byte 0 location.
//
    pub byte0_mask: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of extracted VLAN ID field.
//
    pub vlan_id_mask: u32,
// ! 0: don't care
// 1: enable comparison of extracted VLAN UP field.
//
    pub vlan_up_mask: u32,
// ! 0: don't care
// 1: enable comparison of extracted VLAN Valid field.
//
    pub vlan_valid_mask: u32,
// ! This is bit mask to enable comparison the 8 bit TCI field,
// including the AN field.
// For explicit SECTAG, AN is hardware controlled. For sending
// packet w/ explicit SECTAG, rest of the TCI fields are directly
// from the SECTAG.
//
    pub tci_mask: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of SCI
// Note: If this field is not 0, this means the input packet's
// SECTAG is explicitly tagged and MACSEC module will only update
// the MSDU.
// PN number is hardware controlled.
//
    pub sci_mask: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of Ethertype.
//
    pub eth_type_mask: u32,
// ! Mask is per-byte.
// 0: don't care and no SNAP header exist.
// 1: compare the SNAP header.
// If this bit is set to 1, the extracted filed will assume the
// SNAP header exist as encapsulated in 802.3 (RFC 1042). I.E. the
// next 5 bytes after the LLC header is SNAP header.
//
    pub snap_mask: u32,
// ! 0: don't care and no LLC header exist.
// 1: compare the LLC header.
// If this bit is set to 1, the extracted filed will assume the
// LLC header exist as encapsulated in 802.3 (RFC 1042). I.E. the
// next three bytes after the 802.3MAC header is LLC header.
//
    pub llc_mask: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of MAC_SA.
//
    pub sa_mask: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of MAC_DA.
//
    pub da_mask: u32,
// ! Mask is per-byte.
    pub pn_mask: u32,
// ! Reserved. This bit should be always 0.
    pub eight02dot2: u32,
// ! 1: For explicit sectag case use TCI_SC from table
// 0: use TCI_SC from explicit sectag.
//
    pub tci_sc: u32,
// ! 1: For explicit sectag case,use TCI_V,ES,SCB,E,C from table
// 0: use TCI_V,ES,SCB,E,C from explicit sectag.
//
    pub tci_87543: u32,
// ! 1: indicates that incoming packet has explicit sectag.
    pub exp_sectag_en: u32,
// ! If packet matches and tagged as controlled-packet, this SC/SA
// index is used for later SC and SA table lookup.
//
    pub sc_idx: u32,
// ! This field is used to specify how many SA entries are
// associated with 1 SC entry.
// 2'b00: 1 SC has 4 SA.
// SC index is equivalent to {SC_Index[4:2], 1'b0}.
// SA index is equivalent to {SC_Index[4:2], SC entry's current AN[1:0]
// 2'b10: 1 SC has 2 SA.
// SC index is equivalent to SC_Index[4:1]
// SA index is equivalent to {SC_Index[4:1], SC entry's current AN[0]}
// 2'b11: 1 SC has 1 SA. No SC entry exists for the specific SA.
// SA index is equivalent to SC_Index[4:0]
// Note: if specified as 2'b11, hardware AN roll over is not
// supported.
//
    pub sc_sa: u32,
// ! 0: the packets will be sent to MAC FIFO
// 1: The packets will be sent to Debug/Loopback FIFO.
// If the above's action is drop, this bit has no meaning.
//
    pub debug: u32,
// ! 0: forward to remaining modules
// 1: bypass the next encryption modules. This packet is considered
// un-control packet.
// 2: drop
// 3: Reserved.
//
    pub action: u32,
// ! 0: Not valid entry. This entry is not used
// 1: valid entry.
//
    pub valid: u32,
}

// ! Represents the bitfields of a single row in the Egress SC Lookup table.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_sc_record {
// ! This is to specify when the SC was first used. Set by HW.
    pub start_time: u32,
// ! This is to specify when the SC was last used. Set by HW.
    pub stop_time: u32,
// ! This is to specify which of the SA entries are used by current HW.
// Note: This value need to be set by SW after reset.  It will be
// automatically updated by HW, if AN roll over is enabled.
//
    pub curr_an: u32,
// ! 0: Clear the SA Valid Bit after PN expiry.
// 1: Do not Clear the SA Valid bit after PN expiry of the current SA.
// When the Enable AN roll over is set, S/W does not need to
// program the new SA's and the H/W will automatically roll over
// between the SA's without session expiry.
// For normal operation, Enable AN Roll over will be set to '0'
// and in which case, the SW needs to program the new SA values
// after the current PN expires.
//
    pub an_roll: u32,
// ! This is the TCI field used if packet is not explicitly tagged.
    pub tci: u32,
// ! This value indicates the offset where the decryption will start.
// [[Values of 0, 4, 8-50].
//
    pub enc_off: u32,
// ! 0: Do not protect frames, all the packets will be forwarded
// unchanged. MIB counter (OutPktsUntagged) will be updated.
// 1: Protect.
//
    pub protect: u32,
// ! 0: when none of the SA related to SC has inUse set.
// 1: when either of the SA related to the SC has inUse set.
// This bit is set by HW.
//
    pub recv: u32,
// ! 0: H/W Clears this bit on the first use.
// 1: SW updates this entry, when programming the SC Table.
//
    pub fresh: u32,
// ! AES Key size
// 00 - 128bits
// 01 - 192bits
// 10 - 256bits
// 11 - Reserved.
//
    pub sak_len: u32,
// ! 0: Invalid SC
// 1: Valid SC.
//
    pub valid: u32,
}

// ! Represents the bitfields of a single row in the Egress SA Lookup table.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_sa_record {
// ! This is to specify when the SC was first used. Set by HW.
    pub start_time: u32,
// ! This is to specify when the SC was last used. Set by HW.
    pub stop_time: u32,
// ! This is set by SW and updated by HW to store the Next PN number
// used for encryption.
//
    pub next_pn: u32,
// ! The Next_PN number is going to wrapped around from 0xFFFF_FFFF
// to 0. set by HW.
//
    pub sat_pn: u32,
// ! 0: This SA is in use.
// 1: This SA is Fresh and set by SW.
//
    pub fresh: u32,
// ! 0: Invalid SA
// 1: Valid SA.
//
    pub valid: u32,
}

// ! Represents the bitfields of a single row in the Egress SA Key
// Lookup table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_sakey_record {
// ! Key for AES-GCM processing.
    pub key: [u32; 8],
}

// ! Represents the bitfields of a single row in the Ingress Pre-MACSec
// CTL Filter table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_prectlf_record {
// ! This is used to store the 48 bit value used to compare SA, DA
// or halfDA+half SA value.
//
    pub sa_da: [u32; 2],
// ! This is used to store the 16 bit ethertype value used for
// comparison.
//
    pub eth_type: u32,
// ! The match mask is per-nibble. 0 means don't care, i.e. every
// value will match successfully. The total data is 64 bit, i.e.
// 16 nibbles masks.
//
    pub match_mask: u32,
// ! 0: No compare, i.e. This entry is not used
// 1: compare DA only
// 2: compare SA only
// 3: compare half DA + half SA
// 4: compare ether type only
// 5: compare DA + ethertype
// 6: compare SA + ethertype
// 7: compare DA+ range.
//
    pub match_type: u32,
// ! 0: Bypass the remaining modules if matched.
// 1: Forward to next module for more classifications.
//
    pub action: u32,
}

// ! Represents the bitfields of a single row in the Ingress Pre-MACSec
// Packet Classifier table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_preclass_record {
// ! The 64 bit SCI field used to compare with extracted value.
// Should have SCI value in case TCI[SCI_SEND] == 0. This will be
// used for ICV calculation.
//
    pub sci: [u32; 2],
// ! The 8 bit TCI field used to compare with extracted value.
    pub tci: u32,
// ! 8 bit encryption offset.
    pub encr_offset: u32,
// ! The 16 bit Ethertype (in the clear) field used to compare with
// extracted value.
//
    pub eth_type: u32,
// ! This is to specify the 40bit SNAP header if the SNAP header's
// mask is enabled.
//
    pub snap: [u32; 2],
// ! This is to specify the 24bit LLC header if the LLC header's
// mask is enabled.
//
    pub llc: u32,
// ! The 48 bit MAC_SA field used to compare with extracted value.
    pub mac_sa: [u32; 2],
// ! The 48 bit MAC_DA field used to compare with extracted value.
    pub mac_da: [u32; 2],
// ! 0: this is to compare with non-LPBK packet
// 1: this is to compare with LPBK packet.
// This value is used to compare with a controlled-tag which goes
// with the packet when looped back from Egress port.
//
    pub lpbk_packet: u32,
// ! The value of this bit mask will affects how the SC index and SA
// index created.
// 2'b00: 1 SC has 4 SA.
// SC index is equivalent to {SC_Index[4:2], 1'b0}.
// SA index is equivalent to {SC_Index[4:2], SECTAG's AN[1:0]}
// Here AN bits are not compared.
// 2'b10: 1 SC has 2 SA.
// SC index is equivalent to SC_Index[4:1]
// SA index is equivalent to {SC_Index[4:1], SECTAG's AN[0]}
// Compare AN[1] field only
// 2'b11: 1 SC has 1 SA. No SC entry exists for the specific SA.
// SA index is equivalent to SC_Index[4:0]
// AN[1:0] bits are compared.
// NOTE: This design is to supports different usage of AN. User
// can either ping-pong buffer 2 SA by using only the AN[0] bit.
// Or use 4 SA per SC by use AN[1:0] bits. Or even treat each SA
// as independent. i.e. AN[1:0] is just another matching pointer
// to select SA.
//
    pub an_mask: u32,
// ! This is bit mask to enable comparison the upper 6 bits TCI
// field, which does not include the AN field.
// 0: don't compare
// 1: enable comparison of the bits.
//
    pub tci_mask: u32,
// ! 0: don't care
// 1: enable comparison of SCI.
//
    pub sci_mask: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of Ethertype.
//
    pub eth_type_mask: u32,
// ! Mask is per-byte.
// 0: don't care and no SNAP header exist.
// 1: compare the SNAP header.
// If this bit is set to 1, the extracted filed will assume the
// SNAP header exist as encapsulated in 802.3 (RFC 1042). I.E. the
// next 5 bytes after the LLC header is SNAP header.
//
    pub snap_mask: u32,
// ! Mask is per-byte.
// 0: don't care and no LLC header exist.
// 1: compare the LLC header.
// If this bit is set to 1, the extracted filed will assume the
// LLC header exist as encapsulated in 802.3 (RFC 1042). I.E. the
// next three bytes after the 802.3MAC header is LLC header.
//
    pub llc_mask: u32,
// ! Reserved. This bit should be always 0.
    pub _802_2_encapsulate: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of MAC_SA.
//
    pub sa_mask: u32,
// ! Mask is per-byte.
// 0: don't care
// 1: enable comparison of MAC_DA.
//
    pub da_mask: u32,
// ! 0: don't care
// 1: enable checking if this is loopback packet or not.
//
    pub lpbk_mask: u32,
// ! If packet matches and tagged as controlled-packet. This SC/SA
// index is used for later SC and SA table lookup.
//
    pub sc_idx: u32,
// ! 0: the packets will be sent to MAC FIFO
// 1: The packets will be sent to Debug/Loopback FIFO.
// If the above's action is drop. This bit has no meaning.
//
    pub proc_dest: u32,
// ! 0: Process: Forward to next two modules for 802.1AE decryption.
// 1: Process but keep SECTAG: Forward to next two modules for
// 802.1AE decryption but keep the MACSEC header with added error
// code information. ICV will be stripped for all control packets.
// 2: Bypass: Bypass the next two decryption modules but processed
// by post-classification.
// 3: Drop: drop this packet and update counts accordingly.
//
    pub action: u32,
// ! 0: This is a controlled-port packet if matched.
// 1: This is an uncontrolled-port packet if matched.
//
    pub ctrl_unctrl: u32,
// ! Use the SCI value from the Table if 'SC' bit of the input
// packet is not present.
//
    pub sci_from_table: u32,
// ! Reserved.
    pub reserved: u32,
// ! 0: Not valid entry. This entry is not used
// 1: valid entry.
//
    pub valid: u32,
}

// ! Represents the bitfields of a single row in the Ingress SC Lookup table.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_sc_record {
// ! This is to specify when the SC was first used. Set by HW.
    pub stop_time: u32,
// ! This is to specify when the SC was first used. Set by HW.
    pub start_time: u32,
// ! 0: Strict
// 1: Check
// 2: Disabled.
//
    pub validate_frames: u32,
// ! 1: Replay control enabled.
// 0: replay control disabled.
//
    pub replay_protect: u32,
// ! This is to specify the window range for anti-replay. Default is 0.
// 0: is strict order enforcement.
//
    pub anti_replay_window: u32,
// ! 0: when none of the SA related to SC has inUse set.
// 1: when either of the SA related to the SC has inUse set.
// This bit is set by HW.
//
    pub receiving: u32,
// ! 0: when hardware processed the SC for the first time, it clears
// this bit
// 1: This bit is set by SW, when it sets up the SC.
//
    pub fresh: u32,
// ! 0: The AN number will not automatically roll over if Next_PN is
// saturated.
// 1: The AN number will automatically roll over if Next_PN is
// saturated.
// Rollover is valid only after expiry. Normal roll over between
// SA's should be normal process.
//
    pub an_rol: u32,
// ! Reserved.
    pub reserved: u32,
// ! 0: Invalid SC
// 1: Valid SC.
//
    pub valid: u32,
}

// ! Represents the bitfields of a single row in the Ingress SA Lookup table.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_sa_record {
// ! This is to specify when the SC was first used. Set by HW.
    pub stop_time: u32,
// ! This is to specify when the SC was first used. Set by HW.
    pub start_time: u32,
// ! This is updated by HW to store the expected NextPN number for
// anti-replay.
//
    pub next_pn: u32,
// ! The Next_PN number is going to wrapped around from 0XFFFF_FFFF
// to 0. set by HW.
//
    pub sat_nextpn: u32,
// ! 0: This SA is not yet used.
// 1: This SA is inUse.
//
    pub in_use: u32,
// ! 0: when hardware processed the SC for the first time, it clears
// this timer
// 1: This bit is set by SW, when it sets up the SC.
//
    pub fresh: u32,
// ! Reserved.
    pub reserved: u32,
// ! 0: Invalid SA.
// 1: Valid SA.
//
    pub valid: u32,
}

// ! Represents the bitfields of a single row in the Ingress SA Key
// Lookup table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_sakey_record {
// ! Key for AES-GCM processing.
    pub key: [u32; 8],
// ! AES key size
// 00 - 128bits
// 01 - 192bits
// 10 - 256bits
// 11 - reserved.
//
    pub key_len: u32,
}

// ! Represents the bitfields of a single row in the Ingress Post-
// MACSec Packet Classifier table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_postclass_record {
// ! The 8 bit value used to compare with extracted value for byte 0.
    pub byte0: u32,
// ! The 8 bit value used to compare with extracted value for byte 1.
    pub byte1: u32,
// ! The 8 bit value used to compare with extracted value for byte 2.
    pub byte2: u32,
// ! The 8 bit value used to compare with extracted value for byte 3.
    pub byte3: u32,
// ! Ethertype in the packet.
    pub eth_type: u32,
// ! Ether Type value > 1500 (0x5dc).
    pub eth_type_valid: u32,
// ! VLAN ID after parsing.
    pub vlan_id: u32,
// ! VLAN priority after parsing.
    pub vlan_up: u32,
// ! Valid VLAN coding.
    pub vlan_valid: u32,
// ! SA index.
    pub sai: u32,
// ! SAI hit, i.e. controlled packet.
    pub sai_hit: u32,
// ! Mask for payload ethertype field.
    pub eth_type_mask: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte3_location: u32,
// ! Mask for Byte Offset 3.
    pub byte3_mask: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte2_location: u32,
// ! Mask for Byte Offset 2.
    pub byte2_mask: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte1_location: u32,
// ! Mask for Byte Offset 1.
    pub byte1_mask: u32,
// ! 0~63: byte location used extracted by packets comparator, which
// can be anything from the first 64 bytes of the MAC packets.
// This byte location counted from MAC' DA address. i.e. set to 0
// will point to byte 0 of DA address.
//
    pub byte0_location: u32,
// ! Mask for Byte Offset 0.
    pub byte0_mask: u32,
// ! Mask for Ethertype valid field. Indicates 802.3 vs. Other.
    pub eth_type_valid_mask: u32,
// ! Mask for VLAN ID field.
    pub vlan_id_mask: u32,
// ! Mask for VLAN UP field.
    pub vlan_up_mask: u32,
// ! Mask for VLAN valid field.
    pub vlan_valid_mask: u32,
// ! Mask for SAI.
    pub sai_mask: u32,
// ! Mask for SAI_HIT.
    pub sai_hit_mask: u32,
// ! Action if only first level matches and second level does not.
// 0: pass
// 1: drop (fail).
//
    pub firstlevel_actions: u32,
// ! Action if both first and second level matched.
// 0: pass
// 1: drop (fail).
//
    pub secondlevel_actions: u32,
// ! Reserved.
    pub reserved: u32,
// ! 0: Not valid entry. This entry is not used
// 1: valid entry.
//
    pub valid: u32,
}

// ! Represents the bitfields of a single row in the Ingress Post-
// MACSec CTL Filter table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_postctlf_record {
// ! This is used to store the 48 bit value used to compare SA, DA
// or halfDA+half SA value.
//
    pub sa_da: [u32; 2],
// ! This is used to store the 16 bit ethertype value used for
// comparison.
//
    pub eth_type: u32,
// ! The match mask is per-nibble. 0 means don't care, i.e. every
// value will match successfully. The total data is 64 bit, i.e.
// 16 nibbles masks.
//
    pub match_mask: u32,
// ! 0: No compare, i.e. This entry is not used
// 1: compare DA only
// 2: compare SA only
// 3: compare half DA + half SA
// 4: compare ether type only
// 5: compare DA + ethertype
// 6: compare SA + ethertype
// 7: compare DA+ range.
//
    pub match_type: u32,
// ! 0: Bypass the remaining modules if matched.
// 1: Forward to next module for more classifications.
//
    pub action: u32,
}

// ! Represents the Egress MIB counters for a single SC. Counters are
// 64 bits, lower 32 bits in field[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_sc_counters {
// ! The number of integrity protected but not encrypted packets
// for this transmitting SC.
//
    pub sc_protected_pkts: [u32; 2],
// ! The number of integrity protected and encrypted packets for
// this transmitting SC.
//
    pub sc_encrypted_pkts: [u32; 2],
// ! The number of plain text octets that are integrity protected
// but not encrypted on the transmitting SC.
//
    pub sc_protected_octets: [u32; 2],
// ! The number of plain text octets that are integrity protected
// and encrypted on the transmitting SC.
//
    pub sc_encrypted_octets: [u32; 2],
}

// ! Represents the Egress MIB counters for a single SA. Counters are
// 64 bits, lower 32 bits in field[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_sa_counters {
// ! The number of dropped packets for this transmitting SA.
    pub sa_hit_drop_redirect: [u32; 2],
// ! TODO
    pub sa_protected2_pkts: [u32; 2],
// ! The number of integrity protected but not encrypted packets
// for this transmitting SA.
//
    pub sa_protected_pkts: [u32; 2],
// ! The number of integrity protected and encrypted packets for
// this transmitting SA.
//
    pub sa_encrypted_pkts: [u32; 2],
}

// ! Represents the common Egress MIB counters; the counter not
// associated with a particular SC/SA. Counters are 64 bits, lower 32
// bits in field[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_egress_common_counters {
// ! The number of transmitted packets classified as MAC_CTL packets.
    pub ctl_pkt: [u32; 2],
// ! The number of transmitted packets that did not match any rows
// in the Egress Packet Classifier table.
//
    pub unknown_sa_pkts: [u32; 2],
// ! The number of transmitted packets where the SC table entry has
// protect=0 (so packets are forwarded unchanged).
//
    pub untagged_pkts: [u32; 2],
// ! The number of transmitted packets discarded because the packet
// length is greater than the ifMtu of the Common Port interface.
//
    pub too_long: [u32; 2],
// ! The number of transmitted packets for which table memory was
// affected by an ECC error during processing.
//
    pub ecc_error_pkts: [u32; 2],
// ! The number of transmitted packets for where the matched row in
// the Egress Packet Classifier table has action=drop.
//
    pub unctrl_hit_drop_redir: [u32; 2],
}

// ! Represents the Ingress MIB counters for a single SA. Counters are
// 64 bits, lower 32 bits in field[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_sa_counters {
// ! For this SA, the number of received packets without a SecTAG.
    pub untagged_hit_pkts: [u32; 2],
// ! For this SA, the number of received packets that were dropped.
    pub ctrl_hit_drop_redir_pkts: [u32; 2],
// ! For this SA which is not currently in use, the number of
// received packets that have been discarded, and have either the
// packets encrypted or the matched row in the Ingress SC Lookup
// table has validate_frames=Strict.
//
    pub not_using_sa: [u32; 2],
// ! For this SA which is not currently in use, the number of
// received, unencrypted, packets with the matched row in the
// Ingress SC Lookup table has validate_frames!=Strict.
//
    pub unused_sa: [u32; 2],
// ! For this SA, the number discarded packets with the condition
// that the packets are not valid and one of the following
// conditions are true: either the matched row in the Ingress SC
// Lookup table has validate_frames=Strict or the packets
// encrypted.
//
    pub not_valid_pkts: [u32; 2],
// ! For this SA, the number of packets with the condition that the
// packets are not valid and the matched row in the Ingress SC
// Lookup table has validate_frames=Check.
//
    pub invalid_pkts: [u32; 2],
// ! For this SA, the number of validated packets.
    pub ok_pkts: [u32; 2],
// ! For this SC, the number of received packets that have been
// discarded with the condition: the matched row in the Ingress
// SC Lookup table has replay_protect=1 and the PN of the packet
// is lower than the lower bound replay check PN.
//
    pub late_pkts: [u32; 2],
// ! For this SA, the number of packets with the condition that the
// PN of the packets is lower than the lower bound replay
// protection PN.
//
    pub delayed_pkts: [u32; 2],
// ! For this SC, the number of packets with the following condition:
// - the matched row in the Ingress SC Lookup table has
// replay_protect=0 or
// - the matched row in the Ingress SC Lookup table has
// replay_protect=1 and the packet is not encrypted and the
// integrity check has failed or
// - the matched row in the Ingress SC Lookup table has
// replay_protect=1 and the packet is encrypted and integrity
// check has failed.
//
    pub unchecked_pkts: [u32; 2],
// ! The number of octets of plaintext recovered from received
// packets that were integrity protected but not encrypted.
//
    pub validated_octets: [u32; 2],
// ! The number of octets of plaintext recovered from received
// packets that were integrity protected and encrypted.
//
    pub decrypted_octets: [u32; 2],
}

// ! Represents the common Ingress MIB counters; the counter not
// associated with a particular SA. Counters are 64 bits, lower 32
// bits in field[0].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aq_mss_ingress_common_counters {
// ! The number of received packets classified as MAC_CTL packets.
    pub ctl_pkts: [u32; 2],
// ! The number of received packets with the MAC security tag
// (SecTAG), not matching any rows in the Ingress Pre-MACSec
// Packet Classifier table.
//
    pub tagged_miss_pkts: [u32; 2],
// ! The number of received packets without the MAC security tag
// (SecTAG), not matching any rows in the Ingress Pre-MACSec
// Packet Classifier table.
//
    pub untagged_miss_pkts: [u32; 2],
// ! The number of received packets discarded without the MAC
// security tag (SecTAG) and with the matched row in the Ingress
// SC Lookup table having validate_frames=Strict.
//
    pub notag_pkts: [u32; 2],
// ! The number of received packets without the MAC security tag
// (SecTAG) and with the matched row in the Ingress SC Lookup
// table having validate_frames!=Strict.
//
    pub untagged_pkts: [u32; 2],
// ! The number of received packets discarded with an invalid
// SecTAG or a zero value PN or an invalid ICV.
//
    pub bad_tag_pkts: [u32; 2],
// ! The number of received packets discarded with unknown SCI
// information with the condition:
// the matched row in the Ingress SC Lookup table has
// validate_frames=Strict or the C bit in the SecTAG is set.
//
    pub no_sci_pkts: [u32; 2],
// ! The number of received packets with unknown SCI with the condition:
// The matched row in the Ingress SC Lookup table has
// validate_frames!=Strict and the C bit in the SecTAG is not set.
//
    pub unknown_sci_pkts: [u32; 2],
// ! The number of received packets by the controlled port service
// that passed the Ingress Post-MACSec Packet Classifier table
// check.
//
    pub ctrl_prt_pass_pkts: [u32; 2],
// ! The number of received packets by the uncontrolled port
// service that passed the Ingress Post-MACSec Packet Classifier
// table check.
//
    pub unctrl_prt_pass_pkts: [u32; 2],
// ! The number of received packets by the controlled port service
// that failed the Ingress Post-MACSec Packet Classifier table
// check.
//
    pub ctrl_prt_fail_pkts: [u32; 2],
// ! The number of received packets by the uncontrolled port
// service that failed the Ingress Post-MACSec Packet Classifier
// table check.
//
    pub unctrl_prt_fail_pkts: [u32; 2],
// ! The number of received packets discarded because the packet
// length is greater than the ifMtu of the Common Port interface.
//
    pub too_long_pkts: [u32; 2],
// ! The number of received packets classified as MAC_CTL by the
// Ingress Post-MACSec CTL Filter table.
//
    pub igpoc_ctl_pkts: [u32; 2],
// ! The number of received packets for which table memory was
// affected by an ECC error during processing.
//
    pub ecc_error_pkts: [u32; 2],
// ! The number of received packets by the uncontrolled port
// service that were dropped.
//
    pub unctrl_hit_drop_redir: [u32; 2],
}
