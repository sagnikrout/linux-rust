//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_sa.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Voltaire, Inc.  All rights reserved.
// Copyright (c) 2006 Intel Corporation.  All rights reserved.
//

pub const OPA_SA_CLASS_VERSION: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_sa_selector {
    IB_SA_GT   = 0,
    IB_SA_LT   = 1,
    IB_SA_EQ   = 2,
//
// The meaning of "best" depends on the attribute: for
// example, for MTU best will return the largest available
// MTU, while for packet life time, best will return the
// smallest available life time.
//
    IB_SA_BEST = 3
}

//
// There are 4 types of join states:
// FullMember, NonMember, SendOnlyNonMember, SendOnlyFullMember.
// The order corresponds to JoinState bits in MCMemberRecord.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_sa_mc_join_states {
    FULLMEMBER_JOIN,
    NONMEMBER_JOIN,
    SENDONLY_NONMEBER_JOIN,
    SENDONLY_FULLMEMBER_JOIN,
    NUM_JOIN_MEMBERSHIP_TYPES,
}

//
// Structures for SA records are named "struct ib_sa_xxx_rec."  No
// attempt is made to pack structures to match the physical layout of
// SA records in SA MADs; all packing and unpacking is handled by the
// SA query code.
//
// For a record with structure ib_sa_xxx_rec, the naming convention
// for the component mask value for field yyy is IB_SA_XXX_REC_YYY (we
// never use different abbreviations or otherwise change the spelling
// of xxx/yyy between ib_sa_xxx_rec.yyy and IB_SA_XXX_REC_YYY).
//
// Reserved rows are indicated with comments to help maintainability.
//

// reserved:								 7

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sa_path_rec_type {
    SA_PATH_REC_TYPE_IB,
    SA_PATH_REC_TYPE_ROCE_V1,
    SA_PATH_REC_TYPE_ROCE_V2,
    SA_PATH_REC_TYPE_OPA
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_path_rec_ib {
    pub dlid: __be16,
    pub slid: __be16,
    pub raw_traffic: u8,
}

//
// struct sa_path_rec_roce - RoCE specific portion of the path record entry
// @route_resolved:	When set, it indicates that this route is already
// resolved for this path record entry.
// @dmac:		Destination mac address for the given DGID entry
// of the path record entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_path_rec_roce {
    pub route_resolved: bool,
    pub dmac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_path_rec_opa {
    pub dlid: __be32,
    pub slid: __be32,
    pub raw_traffic: u8,
    pub l2_8B: u8,
    pub l2_10B: u8,
    pub l2_9B: u8,
    pub l2_16B: u8,
    pub qos_type: u8,
    pub qos_priority: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_path_rec {
    pub dgid: ib_gid,
    pub sgid: ib_gid,
    pub service_id: __be64,
// reserved
    pub flow_label: __be32,
    pub hop_limit: u8,
    pub traffic_class: u8,
    pub reversible: u8,
    pub numb_path: u8,
    pub pkey: __be16,
    pub qos_class: __be16,
    pub sl: u8,
    pub mtu_selector: u8,
    pub mtu: u8,
    pub rate_selector: u8,
    pub rate: u8,
    pub packet_life_time_selector: u8,
    pub packet_life_time: u8,
    pub preference: u8,
    pub ib: sa_path_rec_ib,
    pub roce: sa_path_rec_roce,
    pub opa: sa_path_rec_opa,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_service_rec {
    pub id: __be64,
    pub gid: [__u8; 16],
    pub pkey: __be16,
    pub reserved: [__u8; 2],
    pub lease: __be32,
    pub key: [__u8; 16],
    pub name: [__u8; 64],
    pub data_8: [__u8; 16],
    pub data_16: [__be16; 8],
    pub data_32: [__be32; 4],
    pub data_64: [__be64; 2],
}

// Create OPA GID and zero out the LID
// Convert from OPA to IB path record
// dest = *src;
// Convert from IB to OPA path record
// Do a structure copy and overwrite the relevant fields
// dest = *src;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sa_mcmember_rec {
    pub mgid: ib_gid,
    pub port_gid: ib_gid,
    pub qkey: __be32,
    pub mlid: __be16,
    pub mtu_selector: u8,
    pub mtu: u8,
    pub traffic_class: u8,
    pub pkey: __be16,
    pub rate_selector: u8,
    pub rate: u8,
    pub packet_life_time_selector: u8,
    pub packet_life_time: u8,
    pub sl: u8,
    pub flow_label: __be32,
    pub hop_limit: u8,
    pub scope: u8,
    pub join_state: u8,
    pub proxy_join: u8,
}

// Service Record Component Mask Sec 15.2.5.14 Ver 1.1

// reserved:								 3

pub const IB_DEFAULT_SERVICE_LEASE: c_uint = 0xFFFFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sa_guidinfo_rec {
    pub lid: __be16,
    pub block_num: u8,
// reserved
    pub res1: u8,
    pub res2: __be32,
    pub guid_info_list: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sa_client {
    pub users: core::sync::atomic::AtomicI32,
    pub comp: completion,
}

//
// ib_sa_register_client - Register an SA client.
//
extern "C" {
    pub fn ib_sa_register_client(client: *mut ib_sa_client);
}
//
// ib_sa_unregister_client - Deregister an SA client.
// @client: Client object to deregister.
//
extern "C" {
    pub fn ib_sa_unregister_client(client: *mut ib_sa_client);
}
extern "C" {
    pub fn ib_sa_cancel_query(id: c_int, query: *mut ib_sa_query);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sa_multicast {
    pub rec: ib_sa_mcmember_rec,
    pub comp_mask: ib_sa_comp_mask,
    pub multicast): *mut ib_sa_multicast,
    pub context: *mut c_void,
}

//
// ib_sa_join_multicast - Initiates a join request to the specified multicast
// group.
// @client: SA client
// @device: Device associated with the multicast group.
// @port_num: Port on the specified device to associate with the multicast
// group.
// @rec: SA multicast member record specifying group attributes.
// @comp_mask: Component mask indicating which group attributes of %rec are
// valid.
// @gfp_mask: GFP mask for memory allocations.
// @callback: User callback invoked once the join operation completes.
// @context: User specified context stored with the ib_sa_multicast structure.
//
// This call initiates a multicast join request with the SA for the specified
// multicast group.  If the join operation is started successfully, it returns
// an ib_sa_multicast structure that is used to track the multicast operation.
// Users must free this structure by calling ib_free_multicast, even if the
// join operation later fails.  (The callback status is non-zero.)
//
// If the join operation fails; status will be non-zero, with the following
// failures possible:
// -ETIMEDOUT: The request timed out.
// -EIO: An error occurred sending the query.
// -EINVAL: The MCMemberRecord values differed from the existing group's.
// -ENETRESET: Indicates that an fatal error has occurred on the multicast
// group, and the user must rejoin the group to continue using it.
//
// multicast),
//
// ib_free_multicast - Frees the multicast tracking structure, and releases
// any reference on the multicast group.
// @multicast: Multicast tracking structure allocated by ib_join_multicast.
//
// This call blocks until the multicast identifier is destroyed.  It may
// not be called from within the multicast callback; however, returning a non-
// zero value from the callback will result in destroying the multicast
// tracking structure.
//
extern "C" {
    pub fn ib_sa_free_multicast(multicast: *mut ib_sa_multicast);
}
//
// ib_get_mcmember_rec - Looks up a multicast member record by its MGID and
// returns it if found.
// @device: Device associated with the multicast group.
// @port_num: Port on the specified device to associate with the multicast
// group.
// @mgid: MGID of multicast group.
// @rec: Location to copy SA multicast member record.
//
// ib_init_ah_from_mcmember - Initialize address handle attributes based on
// an SA multicast member record.
//
// ib_sa_pack_path - Conert a path record from struct ib_sa_path_rec
// to IB MAD wire format.
//
extern "C" {
    pub fn ib_sa_pack_path(rec: *mut sa_path_rec, attribute: *mut c_void);
}
//
// ib_sa_pack_service - Convert a service record from struct ib_sa_service_rec
// to IB MAD wire format.
//
extern "C" {
    pub fn ib_sa_pack_service(rec: *mut sa_service_rec, attribute: *mut c_void);
}
//
// ib_sa_unpack_service - Convert a service record from MAD format to struct
// ib_sa_service_rec.
//
extern "C" {
    pub fn ib_sa_unpack_service(attribute: *mut c_void, rec: *mut sa_service_rec);
}
//
// ib_sa_unpack_path - Convert a path record from MAD format to struct
// ib_sa_path_rec.
//
extern "C" {
    pub fn ib_sa_unpack_path(attribute: *mut c_void, rec: *mut sa_path_rec);
}
// Support GuidInfoRecord
extern "C" {
    pub fn htonl(_arg: ntohs(rec->ib.slid)) -> return;
}
extern "C" {
    pub fn htonl(_arg: ntohs(rec->ib.dlid)) -> return;
}
