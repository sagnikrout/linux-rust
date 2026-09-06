//! Automatically rewritten from C Header to Rust Module
//! Source: net/netfilter/nft_set_pipapo.h
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

// Count of concatenated fields depends on count of 32-bit nftables registers

// Restrict usage to multiple fields, make sure rbtree is used otherwise
pub const NFT_PIPAPO_MIN_FIELDS: c_int = 2;
// Largest supported field size

// Bits to be grouped together in table buckets depending on set size

pub const NFT_PIPAPO_GROUP_BITS_SMALL_SET: c_int = 8;
pub const NFT_PIPAPO_GROUP_BITS_LARGE_SET: c_int = 4;

// If a lookup table gets bigger than NFT_PIPAPO_LT_SIZE_HIGH, switch to the
// small group width, and switch to the big group width if the table gets
// smaller than NFT_PIPAPO_LT_SIZE_LOW.
//
// Picking 2MiB as threshold (for a single table) avoids as much as possible
// crossing page boundaries on most architectures (x86-64 and MIPS huge pages,
// ARMv7 supersections, POWER "large" pages, SPARC Level 1 regions, etc.), which
// keeps performance nice in case kvmalloc() gives us non-contiguous areas.
//

// Fields are padded to 32 bits in input registers

// Number of buckets given by 2 ^ n, with n bucket bits

// Each n-bit range maps to up to n * 2 rules

// Use the rest of mapping table buckets for rule indices, but it makes no sense
// to exceed 32 bits
//

pub const NFT_PIPAPO_MAP_TOBITS: c_int = 32;

// ...which gives us the highest allowed index for a rule

// Definitions for vectorised implementations

pub const NFT_PIPAPO_ALIGN_HEADROOM: c_int = 0;

//
// union nft_pipapo_map_bucket - Bucket of mapping table
// @to:		First rule number (in next field) this rule maps to
// @n:		Number of rules (in next field) this rule maps to
// @e:		If there's no next field, pointer to element this rule maps to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union nft_pipapo_map_bucket {

    pub 32): static_assert(NFT_PIPAPO_MAP_TOBITS <=,
    pub to: u32,
    pub 32): static_assert(NFT_PIPAPO_MAP_NBITS <=,
    pub n: u32,

    pub to:NFT_PIPAPO_MAP_TOBITS: c_ulong,
    pub n:NFT_PIPAPO_MAP_NBITS: c_ulong,

}

//
// struct nft_pipapo_field - Lookup, mapping tables and related data for a field
// @rules:	Number of inserted rules
// @bsize:	Size of each bucket in lookup table, in longs
// @rules_alloc: Number of allocated rules, always >= rules
// @groups:	Amount of bit groups
// @bb:		Number of bits grouped together in lookup table buckets
// @lt:		Lookup table: 'groups' rows of buckets
// @mt:		Mapping table: one bucket per rule
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_pipapo_field {
    pub rules: c_uint,
    pub bsize: c_uint,
    pub rules_alloc: c_uint,
    pub groups: u8,
    pub bb: u8,
    pub lt: *mut c_ulong,
    pub mt: *mut nft_pipapo_map_bucket,
}

//
// struct nft_pipapo_scratch - percpu data used for lookup and matching
// @bh_lock:    PREEMPT_RT local spinlock
// @map_index:	Current working bitmap index, toggled between field matches
// @__map:	store partial matching results during lookup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_pipapo_scratch {
    pub bh_lock: local_lock_t,
    pub map_index: u8,
    pub __map: [c_ulong; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nft_pipapo_clone_state {
    NFT_PIPAPO_CLONE_NEW,
    NFT_PIPAPO_CLONE_MOD,
    NFT_PIPAPO_CLONE_ERR,
}

//
// struct nft_pipapo_match - Data used for lookup and matching
// @field_count:	Amount of fields in set
// @state:		add/delete state; used from control plane
// @bsize_max:		Maximum lookup table bucket size of all fields, in longs
// @scratch:		Preallocated per-CPU maps for partial matching results
// @rcu:		Matching data is swapped on commits
// @f:			Fields, with lookup and mapping tables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_pipapo_match {
    pub field_count: u8,
    pub state:8: nft_pipapo_clone_state,
    pub bsize_max: c_uint,
    pub scratch: *mut *mut nft_pipapo_scratch  __percpu,
    pub rcu: rcu_head,
    pub __counted_by(field_count): nft_pipapo_field f[],
}

//
// struct nft_pipapo - Representation of a set
// @match:	Currently in-use matching data
// @clone:	Copy where pending insertions and deletions are kept
// @width:	Total bytes to be matched for one packet, including padding
// @last_gc:	Timestamp of last garbage collection run, jiffies
// @gc_head:	list of nft_trans_gc to queue up for mem reclaim
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_pipapo {
    pub match: *mut nft_pipapo_match __rcu,
    pub clone: *mut nft_pipapo_match,
    pub width: c_int,
    pub last_gc: c_ulong,
    pub gc_head: list_head,
}

//
// struct nft_pipapo_elem - API-facing representation of single set element
// @priv:	element placeholder
// @ext:	nftables API extensions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_pipapo_elem {
    pub priv: nft_elem_priv,
    pub ext: nft_set_ext,
}

//
// pipapo_and_field_buckets_4bit() - Intersect 4-bit buckets
// @f:		Field including lookup table
// @dst:	Area to store result
// @data:	Input data selecting table buckets
//
// pipapo_and_field_buckets_8bit() - Intersect 8-bit buckets
// @f:		Field including lookup table
// @dst:	Area to store result
// @data:	Input data selecting table buckets
//
// pipapo_estimate_size() - Estimate worst-case for set size
// @desc:	Set description, element count and field description used here
//
// The size for this set type can vary dramatically, as it depends on the number
// of rules (composing netmasks) the entries expand to. We compute the worst
// case here.
//
// In general, for a non-ranged entry or a single composing netmask, we need
// one bit in each of the sixteen NFT_PIPAPO_BUCKETS, for each 4-bit group (that
// is, each input bit needs four bits of matching data), plus a bucket in the
// mapping table for each field.
//
// Return: worst-case set size in bytes, 0 on any overflow
//
// Worst-case ranges for each concatenated field: each n-bit
// field can expand to up to n * 2 rules in each bucket, and
// each rule also needs a mapping bucket.
//
// Rules in lookup and mapping tables are needed for each entry
//
// pipapo_resmap_init() - Initialise result map before first use
// @m:		Matching data, including mapping table
// @res_map:	Result map
//
// Initialize all bits covered by the first field to one, so that after
// the first step, only the matching bits of the first bit group remain.
//
// If other fields have a large bitmap, set remainder of res_map to 0.
//
