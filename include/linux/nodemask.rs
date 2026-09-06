//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nodemask.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Nodemasks provide a bitmap suitable for representing the
// set of Node's in a system, one bit position per Node number.
//
// See detailed comments in the file linux/bitmap.h describing the
// data type on which these nodemasks are based.
//
// For details of nodemask_parse_user(), see bitmap_parse_user() in
// lib/bitmap.c.  For details of nodelist_parse(), see bitmap_parselist(),
// also in bitmap.c.  For details of node_remap(), see bitmap_bitremap in
// lib/bitmap.c.  For details of nodes_remap(), see bitmap_remap in
// lib/bitmap.c.  For details of nodes_onto(), see bitmap_onto in
// lib/bitmap.c.  For details of nodes_fold(), see bitmap_fold in
// lib/bitmap.c.
//
// The available nodemask operations are:
//
// void node_set(node, mask)		turn on bit 'node' in mask
// void node_clear(node, mask)		turn off bit 'node' in mask
// void nodes_setall(mask)		set all bits
// void nodes_clear(mask)		clear all bits
// int node_isset(node, mask)		true iff bit 'node' set in mask
// bool node_test_and_set(node, mask)	test and set bit 'node' in mask
//
// bool nodes_and(dst, src1, src2)	dst = src1 & src2  [intersection]
// void nodes_or(dst, src1, src2)	dst = src1 | src2  [union]
// void nodes_xor(dst, src1, src2)	dst = src1 ^ src2
// bool nodes_andnot(dst, src1, src2)	dst = src1 & ~src2
// void nodes_complement(dst, src)	dst = ~src
//
// bool nodes_equal(mask1, mask2)	Does mask1 == mask2?
// bool nodes_intersects(mask1, mask2)	Do mask1 and mask2 intersect?
// bool nodes_subset(mask1, mask2)	Is mask1 a subset of mask2?
// bool nodes_empty(mask)		Is mask empty (no bits sets)?
// bool nodes_full(mask)		Is mask full (all bits sets)?
// int nodes_weight(mask)		Hamming weight - number of set bits
//
// unsigned int first_node(mask)	Number lowest set bit, or MAX_NUMNODES
// unsigned int next_node(node, mask)	Next node past 'node', or MAX_NUMNODES
// unsigned int next_node_in(node, mask) Next node past 'node', or wrap to first,
// or MAX_NUMNODES
// unsigned int first_unset_node(mask)	First node not set in mask, or
// MAX_NUMNODES
//
// nodemask_t nodemask_of_node(node)	Return nodemask with bit 'node' set
// NODE_MASK_ALL			Initializer - all bits set
// NODE_MASK_NONE			Initializer - no bits set
// unsigned long *nodes_addr(mask)	Array of unsigned long's in mask
//
// int nodemask_parse_user(ubuf, ulen, mask)	Parse ascii string as nodemask
// int nodelist_parse(buf, map)		Parse ascii string as nodelist
// int node_remap(oldbit, old, new)	newbit = map(old, new)(oldbit)
// void nodes_remap(dst, src, old, new)	*dst = map(old, new)(src)
// void nodes_onto(dst, orig, relmap)	*dst = orig relative to relmap
// void nodes_fold(dst, orig, sz)	dst bits = orig bits mod sz
//
// for_each_node_mask(node, mask)	for-loop node over mask
//
// int num_online_nodes()		Number of online Nodes
// int num_possible_nodes()		Number of all possible Nodes
//
// int node_random(mask)		Random node with set bit in mask
//
// int node_online(node)		Is some node online?
// int node_possible(node)		Is some node possible?
//
// node_set_online(node)		set bit 'node' in node_online_map
// node_set_offline(node)		clear bit 'node' in node_online_map
//
// for_each_node(node)			for-loop node over node_possible_map
// for_each_online_node(node)		for-loop node over node_online_map
//
// Subtlety:
// 1) The 'type-checked' form of node_isset() causes gcc (3.3.2, anyway)
// to generate slightly worse code.  So use a simple one-line #define
// for node_isset(), instead of wrapping an inline inside a macro, the
// way we do the other calls.
//
// NODEMASK_SCRATCH
// When doing above logical AND, OR, XOR, Remap operations the callers tend to
// need temporary nodemask_t's on the stack. But if NODES_SHIFT is large,
// nodemask_t's consume too much stack space.  NODEMASK_SCRATCH is a helper
// for such situations. See below and CPUMASK_ALLOC also.
//

//
// nodemask_pr_args - printf args to output a nodemask
// @maskp: nodemask to be printed
//
// Can be used to provide arguments for '%*pb[l]' when printing a nodemask.
//

//
// The inline keyword gives the compiler room to decide to inline, or
// not inline a function as it sees best.  However, as these functions
// are called in both __init and non-__init functions, if they are not
// inlined we will end up with a section mismatch error (of the type of
// freeable items not being freed).  So we must use __always_inline here
// to fix the problem.  If other functions in the future also end up in
// this situation they will also need to be annotated as __always_inline
//

// No static inline type checking - see Subtlety (1) above.

extern "C" {
    pub fn test_and_set_bit(_arg: node, _arg: addr->bits) -> return;
}

extern "C" {
    pub fn bitmap_and(_arg: dstp->bits, _arg: src1p->bits, _arg: src2p->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_andnot(_arg: dstp->bits, _arg: src1p->bits, _arg: src2p->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_equal(_arg: src1p->bits, _arg: src2p->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_intersects(_arg: src1p->bits, _arg: src2p->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_subset(_arg: src1p->bits, _arg: src2p->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_empty(_arg: srcp->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_full(_arg: srcp->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_weight(_arg: srcp->bits, _arg: nbits) -> return;
}
// FIXME: better would be to fix all architectures to never return

extern "C" {
    pub fn min(_arg: MAX_NUMNODES, _arg: find_first_bit(srcp->bits, _arg: MAX_NUMNODES)) -> return;
}

extern "C" {
    pub fn min(_arg: MAX_NUMNODES, _arg: find_next_bit(srcp->bits, _arg: MAX_NUMNODES, _arg: n+1)) -> return;
}
//
// Find the next present node in src, starting after node n, wrapping around to
// the first node in src if needed.  Returns MAX_NUMNODES if src is empty.
//

extern "C" {
    pub fn min(_arg: MAX_NUMNODES, _arg: find_first_zero_bit(maskp->bits, _arg: MAX_NUMNODES)) -> return;
}

extern "C" {
    pub fn bitmap_parse_user(_arg: buf, _arg: len, _arg: dstp->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_parselist(_arg: buf, _arg: dstp->bits, _arg: nbits) -> return;
}

extern "C" {
    pub fn bitmap_bitremap(_arg: oldbit, _arg: oldp->bits, _arg: newp->bits, _arg: nbits) -> return;
}

//
// Bitmasks that are kept for all the nodes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum node_states {
    N_POSSIBLE,		/* The node could become online at some point */
    N_ONLINE,		/* The node is online */
    N_NORMAL_MEMORY,	/* The node has regular memory */

    N_HIGH_MEMORY,		/* The node has regular or high memory */

    N_HIGH_MEMORY = N_NORMAL_MEMORY,

    N_MEMORY,		/* The node has memory(regular, high, movable) */
    N_CPU,		/* The node has one or more cpus */
    N_GENERIC_INITIATOR,	/* The node has one or more Generic Initiators */
    NR_NODE_STATES
}

//
// The following particular system nodemasks and operations
// on them manage all possible and online nodes.
//

extern "C" {
    pub fn node_isset(_arg: node, _arg: node_states[state]) -> return;
}
extern "C" {
    pub fn nodes_weight(_arg: node_states[state]) -> return;
}

extern "C" {
    pub fn next_node(_arg: nid, _arg: node_states[N_ONLINE]) -> return;
}
extern "C" {
    pub fn next_node(_arg: nid, _arg: node_states[N_MEMORY]) -> return;
}

pub const first_online_node: c_int = 0;
pub const first_memory_node: c_int = 0;

//
// For nodemask scratch area.
// NODEMASK_ALLOC(type, name) allocates an object with a specified type and
// name.
//

// Example structure for using NODEMASK_ALLOC, used in mempolicy.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nodemask_scratch {
    pub mask1: nodemask_t,
    pub mask2: nodemask_t,
}

