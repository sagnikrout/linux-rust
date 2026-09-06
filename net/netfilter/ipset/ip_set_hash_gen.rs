//! Automatically rewritten from C Header to Rust Module
//! Source: net/netfilter/ipset/ip_set_hash_gen.h
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
// Copyright (C) 2013 Jozsef Kadlecsik <kadlec@netfilter.org>

// Hashing which uses arrays to resolve clashing. The hash table is resized
// (doubled) when searching becomes too long.
// Internally jhash is used with the assumption that the size of the
// stored data is a multiple of sizeof(u32).
//
// Readers and resizing
//
// Resizing can be triggered by userspace command only, and those
// are serialized by the nfnl mutex. During resizing the set is
// read-locked, so the only possible concurrent operations are
// the kernel side readers. Those must be protected by proper RCU locking.
//
// Number of elements to store in an initial array block
pub const AHASH_INIT_SIZE: c_int = 2;
// Max number of elements to store in an array block

// Max muber of elements in the array block when tuned
pub const AHASH_MAX_TUNED: c_int = 64;

// A hash bucket
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hbucket {
    pub /: *mut *mut rcu_head rcu; / for call_rcu,
// Which positions are used in the array
    pub AHASH_MAX_TUNED): DECLARE_BITMAP(used,,
    pub /: *mut *mut u8 size; / size of the array,
    pub /: *mut *mut u8 pos; / position of the first free entry,
}

// Region size for locking == 2^HTABLE_REGION_BITS
pub const HTABLE_REGION_BITS: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htable_gc {
    pub dwork: delayed_work,
    pub /: *mut *mut *mut ip_set set; / Set the gc belongs to,
    pub /: *mut *mut spinlock_t lock; / Lock to exclude gc and resize,
    pub /: *mut *mut u32 region; / Last gc run position,
}

// The hash table: the table size stored here in order to make resizing easy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htable {
    pub /: *mut *mut bool resizing; / Mark ongoing resize,
    pub /: *mut *mut atomic_t uref; / References for dumping and gc,
    pub /: *mut *mut u8 htable_bits; / size of hash table == 2^htable_bits,
    pub /: *mut *mut u32 maxelem; / Maxelem per region,
    pub /: *mut *mut list_head ad; / Resize add|del backlist,
    pub /: *mut *mut *mut ip_set_region hregion; / Region locks and ext sizes,
    pub /: *mut *mut *mut hbucket __rcu bucket[]; / hashtable buckets,
}

pub const IPSET_NET_COUNT: c_int = 1;

//
// struct net_prefix - Representation of a network prefix.
// @cidr: The CIDR prefix length.
// @count: Number of occurrences.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_prefix {
    pub cidr:8: u32,
    pub count:24: u32,
}

//
// struct net_prefixes - A collection of network prefixes.
// @rcu: RCU head
// @seq: Sequence counter guarding in-place reordering of @nets
// @len: Number of entries in the array.
// @nets: Array of net_prefix structures (sorted by CIDR descending).
//
// @nets entries are updated in place under @set's lock. A single entry's
// cidr/count pair is always updated atomically via READ_ONCE()/WRITE_ONCE(),
// but removing an entry also shifts every following entry down by one slot.
// Lockless readers that scan the whole array (i.e. more than a single
// indexed slot) must use @seq to detect and retry across such a shift.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_prefixes {
    pub rcu: rcu_head,
    pub seq: seqcount_spinlock_t,
    pub len: u8,
    pub __counted_by(len): net_prefix nets[],
}

// Compute the hash table size
// We must fit both into u32 in jhash and INT_MAX in kvmalloc_node()
extern "C" {
    pub fn sizeof(htable: *mut *mut hbucket ) + sizeof(struct) -> *mut return hsize;
}

// When cidr is packed with nomatch, cidr - 1 is stored in the data entry

// Family dependent templates

pub const mtype_do_data_match(d): c_int = 1;

// The generic hash structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htype {
    pub /: *mut *mut *mut htable __rcu table; / the hash table,
    pub /: *mut *mut *mut net_prefixes __rcu rnets[IPSET_NET_COUNT]; / cidr prefixes,
    pub /: *mut *mut htable_gc gc; / gc workqueue,
    pub /: *mut *mut u32 maxelem; / max elements in the hash,
    pub /: *mut *mut u32 initval; / random jhash init value,

    pub /: *mut *mut u32 markmask; / markmask value for mark mask to store,

    pub /: *mut *mut u8 bucketsize; / max elements in an array block,

    pub /: *mut *mut u8 netmask; / netmask value for subnets to store,
    pub /: *mut *mut nf_inet_addr bitmask; / stores bitmask,

// Because 'next' is IPv4/IPv6 dependent, no elements of this
// structure and referred in create() may come after 'next'.
//
    pub /: *mut *mut mtype_elem next; / temporary storage for uadd,
}

// ADD|DEL entries saved during resize
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtype_resize_ad {
    pub list: list_head,
    pub /: *mut *mut ipset_adt ad; / ADD|DEL element,
    pub /: *mut *mut mtype_elem d; / Element value,
    pub /: *mut *mut ip_set_ext ext; / Extensions for ADD,
    pub /: *mut *mut ip_set_ext mext; / Target extensions for ADD,
    pub /: *mut *mut u32 flags; / Flags for ADD,
}

//
// mtype_add_cidr - Add a CIDR entry to hash table bookkeeping
// @set: Pointer to the ip_set
// @h: Pointer to the htype
// @cidr: The CIDR prefix length
// @n: The index of the net_prefix array to add @cidr to
//
// Performs an update if @cidr is found, otherwise performs COW-style
// allocation and replacement via RCU.
//
// Return: 0 on success, negative error code on failure.
//
// Add in increasing prefix order, so larger cidr first
//
// mtype_del_cidr - Remove CIDR entry and maintain array integrity.
// @set: Pointer to the ip_set.
// @h: Pointer to the htype.
// @cidr: The CIDR prefix length.
// @n: The index of the net_prefix array to remove @cidr from
//
// If CIDR entry count falls to 0, this function performs a "shift-left"
// operation on all following elements. This ensures that the array remains
// contiguous and maintains its descending order by CIDR. The vacated slot
// at the end of the array is zeroed out (cidr=0, count=0).
//

// Calculate the actual memory size of the set data
extern "C" {
    pub fn sizeof(ahash_sizeof_regions(t->htable_bits: *mut *mut *mut h) + sizeof(t) +) -> return;
}
// Get the ith element from the array block n

// Flush a hash type of set: destroy all elements

// FIXME: use slab cache

// Destroy the hashtable part of the set

// FIXME: use slab cache

// Destroy a hash type of set
// Resizing changes htable_bits, so we ignore it

// Still try to delete expired elements.
// Resize a hash: create a new hash table with doubling the hashsize
// and inserting the elements to it. Repeat until we succeed or
// fail due to memory pressures.
//

// There can't be another parallel resizing,
// but dumping and kernel side add/del are possible
//
// Expire may replace a hbucket with another one

// We have readers running parallel with us,
// so the live data cannot be modified.
//

// dsize,

// There can't be any other writer.
// Give time to other readers of the set
// Add/delete elements processed by the SET target during resize.
// Kernel-side add cannot trigger a resize and userspace actions
// are serialized by the mutex.
//
// If there's nobody else using the table, destroy it

// Make sure parallel readers see that orig->resizing is false
// before we decrement uref
// Cleanup the backlog of ADD/DEL elements
// In case we have plenty of memory :-)
// Get the current number of elements and ext_size in the set
// ext_size += t->hregion[r].ext_size;
// Add an element to a hash and update the internal counters when succeeded,
// otherwise report the proper error code.
//
// Reuse first deleted entry
// Just the extensions could be overwritten
// Reuse first timed out entry
// Create a new slot

// Trigger rehashing

// Must come last for the case when timed out entry is reused
// Ensure all data writes are visible before updating position
// Resize is in process and kernel side add, save values
// Don't bother
// Delete an element from the hash and free up space if possible.
//
// Userspace add and resize is excluded by the mutex.
// Kernespace add does not trigger resize.
//
// Resize is in process and kernel side del,
// save values
//
// nomatch entries return -ENOTEMPTY
extern "C" {
    pub fn mtype_do_data_match(_arg: data) -> return;
}

// Special test function which takes into account the different network
// sizes added to the set
//

// No match, reset multiple match flag

// Test whether the element is added to the set

// If we test an IP address and not a network address,
// try all possible network sizes
//

extern "C" {
    pub fn jhash_size(_arg: htable_bits) -> return;
}
// Reply a HEADER request: fill out the header part of the set

// if netmask is set to anything other than HOST_MASK we know that the user supplied netmask
// and not bitmask. These two are mutually exclusive.

// Make possible to run dumping parallel with resizing
// Reply a LIST/SAVE request: dump the elements of the specified set
// We assume that one hash bucket fills into one page
// Expire may replace a hbucket with another one
// Set listing finished

// Separated condition in order to avoid directive in argument list

// we convert netmask to bitmask and store it

// bitmask and netmask do the same thing, allow only one of these options

// Compute htable_bits from the user input parameter hashsize.
// Assume that hashsize == 2^htable_bits,
// otherwise round up to the first 2^n value.
//

