//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netlabel.h
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
// NetLabel System
//
// The NetLabel system manages static and dynamic label mappings for network
// protocols such as CIPSO and RIPSO.
//
// Author: Paul Moore <paul@paul-moore.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006, 2008
//

//
// NetLabel - A management interface for maintaining network packet label
// mapping tables for explicit packet labeling protocols.
//
// Network protocols such as CIPSO and RIPSO require a label translation layer
// to convert the label on the packet into something meaningful on the host
// machine.  In the current Linux implementation these mapping tables live
// inside the kernel; NetLabel provides a mechanism for user space applications
// to manage these mapping tables.
//
// NetLabel makes use of the Generic NETLINK mechanism as a transport layer to
// send messages between kernel and user space.  The general format of a
// NetLabel message is shown below:
//
// +-----------------+-------------------+--------- --- -- -
// | struct nlmsghdr | struct genlmsghdr | payload
// +-----------------+-------------------+--------- --- -- -
//
// The 'nlmsghdr' and 'genlmsghdr' structs should be dealt with like normal.
// The payload is dependent on the subsystem specified in the
// 'nlmsghdr->nlmsg_type' and should be defined below, supporting functions
// should be defined in the corresponding net/netlabel/netlabel_<subsys>.h|c
// file.  All of the fields in the NetLabel payload are NETLINK attributes, see
// the include/net/netlink.h file for more information on NETLINK attributes.
//
// NetLabel NETLINK protocol
//
// NetLabel NETLINK protocol version
// 1: initial version
// 2: added static labels for unlabeled connections
// 3: network selectors added to the NetLabel/LSM domain mapping and the
// CIPSO_V4_MAP_LOCAL CIPSO mapping was added
//
pub const NETLBL_PROTO_VERSION: c_int = 3;
// NetLabel NETLINK types/families
pub const NETLBL_NLTYPE_NONE: c_int = 0;
pub const NETLBL_NLTYPE_MGMT: c_int = 1;

pub const NETLBL_NLTYPE_RIPSO: c_int = 2;

pub const NETLBL_NLTYPE_CIPSOV4: c_int = 3;

pub const NETLBL_NLTYPE_CIPSOV6: c_int = 4;

pub const NETLBL_NLTYPE_UNLABELED: c_int = 5;

pub const NETLBL_NLTYPE_ADDRSELECT: c_int = 6;

pub const NETLBL_NLTYPE_CALIPSO: c_int = 7;

//
// NetLabel - Kernel API for accessing the network packet label mappings.
//
// The following functions are provided for use by other kernel modules,
// specifically kernel LSM modules, to provide a consistent, transparent API
// for dealing with explicit packet labeling protocols such as CIPSO and
// RIPSO.  The functions defined here are implemented in the
// net/netlabel/netlabel_kapi.c file.
//
// NetLabel audit information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_audit {
    pub prop: lsm_prop,
    pub loginuid: kuid_t,
    pub sessionid: c_uint,
}

//
// LSM security attributes
//
// struct netlbl_lsm_cache - NetLabel LSM security attribute cache
// @refcount: atomic reference counter
// @free: LSM supplied function to free the cache data
// @data: LSM supplied cache data
//
// Description:
// This structure is provided for LSMs which wish to make use of the NetLabel
// caching mechanism to store LSM specific data/attributes in the NetLabel
// cache.  If the LSM has to perform a lot of translation from the NetLabel
// security attributes into it's own internal representation then the cache
// mechanism can provide a way to eliminate some or all of that translation
// overhead on a cache hit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_lsm_cache {
    pub refcount: refcount_t,
    pub data): *const *const void (free) (void,
    pub data: *mut c_void,
}

//
// struct netlbl_lsm_catmap - NetLabel LSM secattr category bitmap
// @startbit: the value of the lowest order bit in the bitmap
// @bitmap: the category bitmap
// @next: pointer to the next bitmap "node" or NULL
//
// Description:
// This structure is used to represent category bitmaps.  Due to the large
// number of categories supported by most labeling protocols it is not
// practical to transfer a full bitmap internally so NetLabel adopts a sparse
// bitmap structure modeled after SELinux's ebitmap structure.
// The catmap bitmap field MUST be a power of two in length and large
// enough to hold at least 240 bits.  Special care (i.e. check the code!)
// should be used when changing these values as the LSM implementation
// probably has functions which rely on the sizes of these types to speed
// processing.
//
pub const NETLBL_CATMAP_MAPCNT: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_lsm_catmap {
    pub startbit: u32,
    pub bitmap: [u64; NETLBL_CATMAP_MAPCNT],
    pub next: *mut netlbl_lsm_catmap,
}

//
// struct netlbl_lsm_secattr - NetLabel LSM security attributes
// @flags: indicate structure attributes, see NETLBL_SECATTR_
// @type: indicate the NLTYPE of the attributes
// @domain: the NetLabel LSM domain
// @cache: NetLabel LSM specific cache
// @attr.mls: MLS sensitivity label
// @attr.mls.cat: MLS category bitmap
// @attr.mls.lvl: MLS sensitivity level
// @attr.secid: LSM specific secid token
//
// Description:
// This structure is used to pass security attributes between NetLabel and the
// LSM modules.  The flags field is used to specify which fields within the
// struct are valid and valid values can be created by bitwise OR'ing the
// NETLBL_SECATTR_* defines.  The domain field is typically set by the LSM to
// specify domain specific configuration settings and is not usually used by
// NetLabel itself when returning security attributes to the LSM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_lsm_secattr {
    pub flags: u32,
// bitmap values for 'flags'
pub const NETLBL_SECATTR_NONE: c_uint = 0x00000000;
pub const NETLBL_SECATTR_DOMAIN: c_uint = 0x00000001;

pub const NETLBL_SECATTR_CACHE: c_uint = 0x00000002;
pub const NETLBL_SECATTR_MLS_LVL: c_uint = 0x00000004;
pub const NETLBL_SECATTR_MLS_CAT: c_uint = 0x00000008;
pub const NETLBL_SECATTR_SECID: c_uint = 0x00000010;
// bitmap meta-values for 'flags'
pub const NETLBL_SECATTR_FREE_DOMAIN: c_uint = 0x01000000;

    pub type: u32,
    pub domain: *mut c_char,
    pub cache: *mut netlbl_lsm_cache,
    pub cat: *mut netlbl_lsm_catmap,
    pub lvl: u32,
    pub mls: },
    pub secid: u32,
    pub attr: },
}

//
// struct netlbl_calipso_ops - NetLabel CALIPSO operations
// @doi_add: add a CALIPSO DOI
// @doi_free: free a CALIPSO DOI
// @doi_remove: remove a CALIPSO DOI
// @doi_getdef: returns a reference to a DOI
// @doi_putdef: releases a reference of a DOI
// @doi_walk: enumerate the DOI list
// @sock_getattr: retrieve the socket's attr
// @sock_setattr: set the socket's attr
// @sock_delattr: remove the socket's attr
// @req_setattr: set the req socket's attr
// @req_delattr: remove the req socket's attr
// @opt_getattr: retrieve attr from memory block
// @skbuff_optptr: find option in packet
// @skbuff_setattr: set the skbuff's attr
// @skbuff_delattr: remove the skbuff's attr
// @cache_invalidate: invalidate cache
// @cache_add: add cache entry
//
// Description:
// This structure is filled out by the CALIPSO engine and passed
// to the NetLabel core via a call to netlbl_calipso_ops_register().
// It enables the CALIPSO engine (and hence IPv6) to be compiled
// as a module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlbl_calipso_ops {
    pub audit_info): *mut netlbl_audit,
    pub doi_def): *mut *mut void (doi_free)(struct calipso_doi,
    pub audit_info): *mut *mut int (doi_remove)(u32 doi, struct netlbl_audit,
    pub doi): *mut *mut *mut calipso_doi (doi_getdef)(u32,
    pub doi_def): *mut *mut void (doi_putdef)(struct calipso_doi,
    pub cb_arg): *mut c_void,
    pub secattr): *mut netlbl_lsm_secattr,
    pub secattr): *const netlbl_lsm_secattr,
    pub sk): *mut *mut void (sock_delattr)(struct sock,
    pub secattr): *const netlbl_lsm_secattr,
    pub req): *mut *mut void (req_delattr)(struct request_sock,
    pub secattr): *mut netlbl_lsm_secattr,
    pub skb): *const *const *const unsigned char (skbuff_optptr)(struct sk_buff,
    pub secattr): *const netlbl_lsm_secattr,
    pub skb): *mut *mut int (skbuff_delattr)(struct sk_buff,
    pub (*cache_invalidate)(void): *mut c_void,
    pub secattr): *const netlbl_lsm_secattr,
}

//
// LSM security attribute operations (inline)
//
// netlbl_secattr_cache_alloc - Allocate and initialize a secattr cache
// @flags: the memory allocation flags
//
// Description:
// Allocate and initialize a netlbl_lsm_cache structure.  Returns a pointer
// on success, NULL on failure.
//

//
// netlbl_secattr_cache_free - Frees a netlbl_lsm_cache struct
// @cache: the struct to free
//
// Description:
// Frees @secattr including all of the internal buffers.
//
// netlbl_catmap_alloc - Allocate a LSM secattr catmap
// @flags: memory allocation flags
//
// Description:
// Allocate memory for a LSM secattr catmap, returns a pointer on success, NULL
// on failure.
//
extern "C" {
    pub fn kzalloc_noprof(netlbl_lsm_catmap): sizeof(struct, _arg: flags) -> return;
}

//
// netlbl_catmap_free - Free a LSM secattr catmap
// @catmap: the category bitmap
//
// Description:
// Free a LSM secattr catmap.
//
// netlbl_secattr_init - Initialize a netlbl_lsm_secattr struct
// @secattr: the struct to initialize
//
// Description:
// Initialize an already allocated netlbl_lsm_secattr struct.
//
// netlbl_secattr_destroy - Clears a netlbl_lsm_secattr struct
// @secattr: the struct to clear
//
// Description:
// Destroys the @secattr struct, including freeing all of the internal buffers.
// The struct must be reset with a call to netlbl_secattr_init() before reuse.
//
// netlbl_secattr_alloc - Allocate and initialize a netlbl_lsm_secattr struct
// @flags: the memory allocation flags
//
// Description:
// Allocate and initialize a netlbl_lsm_secattr struct.  Returns a valid
// pointer on success, or NULL on failure.
//
extern "C" {
    pub fn kzalloc_noprof(netlbl_lsm_secattr): sizeof(struct, _arg: flags) -> return;
}

//
// netlbl_secattr_free - Frees a netlbl_lsm_secattr struct
// @secattr: the struct to free
//
// Description:
// Frees @secattr including all of the internal buffers.
//

//
// LSM configuration operations
//
extern "C" {
    pub fn netlbl_cfg_cipsov4_del(doi: u32, audit_info: *mut netlbl_audit);
}
extern "C" {
    pub fn netlbl_cfg_calipso_del(doi: u32, audit_info: *mut netlbl_audit);
}
//
// LSM security attribute operations
//
extern "C" {
    pub fn netlbl_catmap_walk(catmap: *mut netlbl_lsm_catmap, offset: u32) -> c_int;
}
extern "C" {
    pub fn netlbl_catmap_walkrng(catmap: *mut netlbl_lsm_catmap, offset: u32) -> c_int;
}
// Bitmap functions
//
extern "C" {
    pub fn netlbl_bitmap_setbit(bitmap: *mut c_uchar, bit: u32, state: u8);
}
//
// LSM protocol operations (NetLabel LSM/kernel API)
//
extern "C" {
    pub fn netlbl_enabled() -> c_int;
}
extern "C" {
    pub fn netlbl_sock_delattr(sk: *mut sock);
}
extern "C" {
    pub fn netlbl_req_delattr(req: *mut request_sock);
}
extern "C" {
    pub fn netlbl_skbuff_err(skb: *mut sk_buff, family: u16, error: c_int, gateway: c_int);
}
extern "C" {
    pub fn netlbl_sk_lock_check(sk: *mut sock) -> bool;
}
//
// LSM label mapping cache operations
//
extern "C" {
    pub fn netlbl_cache_invalidate();
}
//
// Protocol engine operations
//

