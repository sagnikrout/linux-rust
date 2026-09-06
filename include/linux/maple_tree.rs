//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/maple_tree.h
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
// Maple Tree - An RCU-safe adaptive tree for storing ranges
// Copyright (c) 2018-2022 Oracle
// Authors:     Liam R. Howlett <liam@infradead.org>
// Matthew Wilcox <willy@infradead.org>
//

//
// Allocated nodes are mutable until they have been inserted into the tree,
// at which time they cannot change their type until they have been removed
// from the tree and an RCU grace period has passed.
//
// Removed nodes have their ->parent set to point to themselves.  RCU readers
// check ->parent before relying on the value that they loaded from the
// slots array.  This lets us reuse the slots array for the RCU head.
//
// Nodes in the tree point to their parent unless bit 0 is set.
//

// 64bit sizes

// 32bit sizes

//
// The node->parent of the root node has bit 0 set and the rest of the pointer
// is a pointer to the tree itself.  No more bits are available in this pointer
// (on m68k, the data structure may only be 2-byte aligned).
//
// Internal non-root nodes can only have maple_range_* nodes as parents.  The
// parent pointer is 256B aligned like all other tree nodes.  When storing a 32
// or 64 bit values, the offset can fit into 4 bits.  The 16 bit values need an
// extra bit to store the offset.  This extra bit comes from a reuse of the last
// bit in the node type.  This is possible by using bit 1 to indicate if bit 2
// is part of the type or the slot.
//
// Once the type is decided, the decision of an allocation range type or a
// range type is done by examining the immutable tree flag for the
// MT_FLAGS_ALLOC_RANGE flag.
//
// Node types:
// 0b??1 = Root
// 0b?00 = 16 bit nodes
// 0b010 = 32 bit nodes
// 0b110 = 64 bit nodes
//
// Slot size and location in the parent pointer:
// type  : slot location
// 0b??1 : Root
// 0b?00 : 16 bit values, type in 0-1, slot in 2-6
// 0b010 : 32 bit values, type in 0-2, slot in 3-6
// 0b110 : 64 bit values, type in 0-2, slot in 3-6
//
// This metadata is used to optimize the gap updating code and in reverse
// searching for gaps or any other code that needs to find the end of the data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_metadata {
    pub /: *mut *mut unsigned char end; / end of data,
    pub /: *mut *mut unsigned char gap; / offset of largest gap,
}

//
// Leaf nodes do not store pointers to nodes, they store user data.  Users may
// store almost any bit pattern.  As noted above, the optimisation of storing an
// entry at 0 in the root pointer cannot be done for data which have the bottom
// two bits set to '10'.  We also reserve values with the bottom two bits set to
// '10' which are below 4096 (ie 2, 6, 10 .. 4094) for internal use.  Some APIs
// return errnos as a negative errno shifted right by two bits and the bottom
// two bits set to '10', and while choosing to store these values in the array
// is not an error, it may lead to confusion if you're testing for an error with
// mas_is_err().
//
// Non-leaf nodes store the type of the node pointed to (enum maple_type in bits
// 3-6), bit 2 is reserved.  That leaves bits 0-1 unused for now.
//
// In regular B-Tree terms, pivots are called keys.  The term pivot is used to
// indicate that the tree is specifying ranges,  Pivots may appear in the
// subtree with an entry attached to the value whereas keys are unique to a
// specific position of a B-tree.  Pivot values are inclusive of the slot with
// the same index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_range_64 {
    pub parent: *mut maple_pnode,
    pub 1]: unsigned long pivot[MAPLE_RANGE64_SLOTS -,
    pub slot: [*mut void __rcu; MAPLE_RANGE64_SLOTS],
    pub 1]: *mut *mut void __rcu pad[MAPLE_RANGE64_SLOTS -,
    pub meta: maple_metadata,
}

//
// At tree creation time, the user can specify that they're willing to trade off
// storing fewer entries in a tree in return for storing more information in
// each node.
//
// The maple tree supports recording the largest range of NULL entries available
// in this node, also called gaps.  This optimises the tree for allocating a
// range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_arange_64 {
    pub parent: *mut maple_pnode,
    pub 1]: unsigned long pivot[MAPLE_ARANGE64_SLOTS -,
    pub slot: [*mut void __rcu; MAPLE_ARANGE64_SLOTS],
    pub gap: [c_ulong; MAPLE_ARANGE64_SLOTS],
    pub meta: maple_metadata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_topiary {
    pub parent: *mut maple_pnode,
    pub /: *mut *mut *mut maple_enode next; / Overlaps the pivot,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum maple_type {
    maple_dense,
    maple_leaf_64,
    maple_range_64,
    maple_arange_64,
    maple_copy,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum store_type {
    wr_invalid,
    wr_new_root,
    wr_store_root,
    wr_exact_fit,
    wr_spanning_store,
    wr_split_store,
    wr_rebalance,
    wr_append,
    wr_node_store,
    wr_slot_store,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_copy {
//
// min, max, and pivots are values
// start, end, split are indexes into arrays
// data is a size
//
    pub node: *mut maple_node,
    pub max: c_ulong,
    pub mt: maple_type,
    pub dst: [}; 3],
    pub node: *mut maple_node,
    pub max: c_ulong,
    pub start: c_uchar,
    pub end: c_uchar,
    pub mt: maple_type,
    pub src: [}; 4],
// Simulated node
    pub slot: [*mut void __rcu; 3],
    pub gap: [c_ulong; 3],
    pub min: c_ulong,
    pub pivot: [c_ulong; 3],
    pub _pad: [*mut c_void; 2],
    pub max: c_ulong,
}

// Avoid passing these around
//
// DOC: Maple tree flags
//
// * MT_FLAGS_ALLOC_RANGE	- Track gaps in this tree
// * MT_FLAGS_USE_RCU		- Operate in RCU mode
// * MT_FLAGS_HEIGHT_OFFSET	- The position of the tree height in the flags
// * MT_FLAGS_HEIGHT_MASK	- The mask for the maple tree height value
// * MT_FLAGS_LOCK_MASK		- How the mt_lock is used
// * MT_FLAGS_LOCK_IRQ		- Acquired irq-safe
// * MT_FLAGS_LOCK_BH		- Acquired bh-safe
// * MT_FLAGS_LOCK_EXTERN	- mt_lock is not used
//
// MAPLE_HEIGHT_MAX	The largest height that can be stored
//
pub const MT_FLAGS_ALLOC_RANGE: c_uint = 0x01;
pub const MT_FLAGS_USE_RCU: c_uint = 0x02;
pub const MT_FLAGS_HEIGHT_OFFSET: c_uint = 0x02;
pub const MT_FLAGS_HEIGHT_MASK: c_uint = 0x7C;
pub const MT_FLAGS_LOCK_MASK: c_uint = 0x300;
pub const MT_FLAGS_LOCK_IRQ: c_uint = 0x100;
pub const MT_FLAGS_LOCK_BH: c_uint = 0x200;
pub const MT_FLAGS_LOCK_EXTERN: c_uint = 0x300;
pub const MT_FLAGS_ALLOC_WRAPPED: c_uint = 0x0800;
pub const MAPLE_HEIGHT_MAX: c_int = 31;
pub const MAPLE_NODE_TYPE_MASK: c_uint = 0x0F;
pub const MAPLE_NODE_TYPE_SHIFT: c_uint = 0x03;
pub const MAPLE_RESERVED_RANGE: c_int = 4096;

pub const mt_lock_is_held(mt): c_int = 1;
pub const mt_write_lock_is_held(mt): c_int = 1;

//
// If the tree contains a single entry at index 0, it is usually stored in
// tree->ma_root.  To optimise for the page cache, an entry which ends in '00',
// '01' or '11' is stored in the root, but an entry which ends in '10' will be
// stored in a node.  Bits 3-6 are used to store enum maple_type.
//
// The flags are used both to store some immutable information about this tree
// (set at tree creation time) and dynamic information set under the spinlock.
//
// Another use of flags are to indicate global states of the tree.  This is the
// case with the MT_FLAGS_USE_RCU flag, which indicates the tree is currently in
// RCU mode.  This mode was added to allow the tree to reuse nodes instead of
// re-allocating and RCU freeing nodes when there is a single user.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_tree {
    pub ma_lock: spinlock_t,

    pub ma_external_lock: *mut lockdep_map,

}

//
// MTREE_INIT() - Initialize a maple tree
// @name: The maple tree name
// @__flags: The maple tree flags
//

//
// MTREE_INIT_EXT() - Initialize a maple tree with an external lock.
// @name: The tree name
// @__flags: The maple tree flags
// @__lock: The external lock
//

//
// The Maple Tree squeezes various bits in at various points which aren't
// necessarily obvious.  Usually, this is done by observing that pointers are
// N-byte aligned and thus the bottom log_2(N) bits are available for use.  We
// don't use the high bits of pointers to store additional information because
// we don't know what bits are unused on any given architecture.
//
// Nodes are 256 bytes in size and are also aligned to 256 bytes, giving us 8
// low bits for our own purposes.  Nodes are currently of 4 types:
// 1. Single pointer (Range is 0-0)
// 2. Non-leaf Allocation Range nodes
// 3. Non-leaf Range nodes
// 4. Leaf Range nodes All nodes consist of a number of node slots,
// pivots, and a parent pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maple_node {
    pub parent: *mut maple_pnode,
    pub slot: [*mut void __rcu; MAPLE_NODE_SLOTS],
}

//
// More complicated stores can cause two nodes to become one or three and
// potentially alter the height of the tree.  Either half of the tree may need
// to be rebalanced against the other.  The ma_topiary struct is used to track
// which nodes have been 'cut' from the tree so that the change can be done
// safely at a later date.  This is done to support RCU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma_topiary {
    pub head: *mut maple_enode,
    pub tail: *mut maple_enode,
    pub mtree: *mut maple_tree,
}

extern "C" {
    pub fn mtree_dup(mt: *mut maple_tree, new: *mut maple_tree, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn __mt_dup(mt: *mut maple_tree, new: *mut maple_tree, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn mtree_destroy(mt: *mut maple_tree);
}
extern "C" {
    pub fn __mt_destroy(mt: *mut maple_tree);
}
//
// mtree_empty() - Determine if a tree has any present entries.
// @mt: Maple Tree.
//
// Context: Any context.
// Return: %true if the tree contains only NULL pointers.
//
// Advanced API
//
// Maple State Status
// ma_active means the maple state is pointing to a node and offset and can
// continue operating on the tree.
// ma_start means we have not searched the tree.
// ma_root means we have searched the tree and the entry we found lives in
// the root of the tree (ie it has index 0, length 1 and is the only entry in
// the tree).
// ma_none means we have searched the tree and there is no node in the
// tree for this entry.  For example, we searched for index 1 in an empty
// tree.  Or we have a tree which points to a full leaf node and we
// searched for an entry which is larger than can be contained in that
// leaf node.
// ma_pause means the data within the maple state may be stale, restart the
// operation
// ma_overflow means the search has reached the upper limit of the search
// ma_underflow means the search has reached the lower limit of the search
// ma_error means there was an error, check the node for the error number.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum maple_status {
    ma_active,
    ma_start,
    ma_root,
    ma_none,
    ma_pause,
    ma_overflow,
    ma_underflow,
    ma_error,
}

//
// The maple state is defined in the struct ma_state and is used to keep track
// of information during operations, and even between operations when using the
// advanced API.
//
// If state->node has bit 0 set then it references a tree location which is not
// a node (eg the root).  If bit 1 is set, the rest of the bits are a negative
// errno.  Bit 2 (the 'unallocated slots' bit) is clear.  Bits 3-6 indicate the
// node type.
//
// state->alloc either has a request number of nodes or an allocated node.  If
// stat->alloc has a requested number of nodes, the first bit will be set (0x1)
// and the remaining bits are the value.  If state->alloc is a node, then the
// node will be of type maple_alloc.  maple_alloc has MAPLE_NODE_SLOTS - 1 for
// storing more allocated nodes, a total number of nodes allocated, and the
// node_count in this node.  node_count is the number of allocated nodes in this
// node.  The scaling beyond MAPLE_NODE_SLOTS - 1 is handled by storing further
// nodes into state->alloc->slot[0]'s node.  Nodes are taken from state->alloc
// by removing a node from the state->alloc node until state->alloc->node_count
// is 1, when state->alloc is returned and the state->alloc->slot[0] is promoted
// to state->alloc.  Nodes are pushed onto state->alloc by putting the current
// state->alloc into the pushed node's slot[0].
//
// The state also contains the implied min/max of the state->node, the depth of
// this search, and the offset. The implied min/max are either from the parent
// node or are 0-oo for the root node.  The depth is incremented or decremented
// every time a node is walked down or up.  The offset is the slot/pivot of
// interest in the node - either for reading or writing.
//
// When returning a value the maple state index and last respectively contain
// the start and end of the range for the entry.  Ranges are inclusive in the
// Maple Tree.
//
// The status of the state is used to determine how the next action should treat
// the state.  For instance, if the status is ma_start then the next action
// should start at the root of the tree and walk down.  If the status is
// ma_pause then the node may be stale data and should be discarded.  If the
// status is ma_overflow, then the last action hit the upper limit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma_state {
    pub /: *mut *mut *mut maple_tree tree; / The tree we're operating in,
    pub /: *mut *mut unsigned long index; / The index we're operating on - range start,
    pub /: *mut *mut unsigned long last; / The last index we're operating on - range end,
    pub /: *mut *mut *mut maple_enode node; / The node containing this entry,
    pub /: *mut *mut unsigned long min; / The minimum index of this node - implied pivot min,
    pub /: *mut *mut unsigned long max; / The maximum index of this node - implied pivot max,
    pub /: *mut *mut *mut slab_sheaf sheaf; / Allocated nodes for this operation,
    pub /: *mut *mut *mut maple_node alloc; / A single allocated node for fast path writes,
    pub /: *mut *mut unsigned long node_request; / The number of nodes to allocate for this operation,
    pub /: *mut *mut maple_status status; / The status of the state (active, start, none, etc),
    pub /: *mut *mut unsigned char depth; / depth of tree descent during write,
    pub offset: c_uchar,
    pub mas_flags: c_uchar,
    pub /: *mut *mut unsigned char end; / The end of the node,
    pub /: *mut *mut store_type store_type; / The type of store needed for this operation,

    pub ld_seq: u32,

    pub rcu_gp: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma_wr_state {
    pub mas: *mut ma_state,
    pub /: *mut *mut *mut maple_node node; / Decoded mas->node,
    pub /: *mut *mut unsigned long r_min; / range min,
    pub /: *mut *mut unsigned long r_max; / range max,
    pub /: *mut *mut maple_type type; / mas->node type,
    pub /: *mut *mut unsigned char offset_end; / The offset where the write ends,
    pub /: *mut *mut *mut unsigned long pivots; / mas->node->pivots pointer,
    pub /: *mut *mut unsigned long end_piv; / The pivot at the offset end,
    pub /: *mut *mut *mut *mut void __rcu slots; / mas->node->slots pointer,
    pub /: *mut *mut *mut void entry; / The entry to write,
    pub /: *mut *mut *mut void content; / The existing entry that is being overwritten,
    pub /: *mut *mut unsigned char vacant_height; / Height of lowest node with free space,
    pub /: *mut *mut unsigned char sufficient_height;/ Height of lowest node with min sufficiency + 1 nodes,
}

//
// Special values for ma_state.node.
// MA_ERROR represents an errno.  After dropping the lock and attempting
// to resolve the error, the walk would have to be restarted from the
// top of the tree as the tree may have been modified.
//

//
// When changing MA_STATE, remember to also change rust/kernel/maple_tree.rs
//

extern "C" {
    pub fn mas_store_gfp(mas: *mut ma_state, entry: *mut c_void, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn mas_store_prealloc(mas: *mut ma_state, entry: *mut c_void);
}
extern "C" {
    pub fn mas_preallocate(mas: *mut ma_state, entry: *mut c_void, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn mas_nomem(mas: *mut ma_state, gfp: gfp_t) -> bool;
}
extern "C" {
    pub fn mas_pause(mas: *mut ma_state);
}
extern "C" {
    pub fn maple_tree_init();
}
extern "C" {
    pub fn mas_destroy(mas: *mut ma_state);
}
//
// This finds an empty area from the highest address to the lowest.
// AKA "Topdown" version,
//
// mas_reset() - Reset a Maple Tree operation state.
// @mas: Maple Tree operation state.
//
// Resets the error or walk state of the @mas so future walks of the
// array will start from the root.  Use this if you have dropped the
// lock and want to reuse the ma_state.
//
// Context: Any context.
//
// mas_for_each() - Iterate over a range of the maple tree.
// @__mas: Maple Tree operation state (maple_state)
// @__entry: Entry retrieved from the tree
// @__max: maximum index to retrieve from the tree
//
// When returned, mas->index and mas->last will hold the entire range for the
// entry.
//
// Note: may return the zero entry.
//

//
// mas_for_each_rev() - Iterate over a range of the maple tree in reverse order.
// @__mas: Maple Tree operation state (maple_state)
// @__entry: Entry retrieved from the tree
// @__min: minimum index to retrieve from the tree
//
// When returned, mas->index and mas->last will hold the entire range for the
// entry.
//
// Note: may return the zero entry.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt_dump_format {
    mt_dump_dec,
    mt_dump_hex,
}

extern "C" {
    pub fn mt_dump(mt: *const maple_tree, format: mt_dump_format);
}
extern "C" {
    pub fn mas_dump(mas: *const ma_state);
}
extern "C" {
    pub fn mas_wr_dump(wr_mas: *const ma_wr_state);
}
extern "C" {
    pub fn mt_validate(mt: *mut maple_tree);
}
extern "C" {
    pub fn mt_cache_shrink();
}

//
// __mas_set_range() - Set up Maple Tree operation state to a sub-range of the
// current location.
// @mas: Maple Tree operation state.
// @start: New start of range in the Maple Tree.
// @last: New end of range in the Maple Tree.
//
// set the internal maple state values to a sub-range.
// Please use mas_set_range() if you do not know where you are in the tree.
//
// Ensure the range starts within the current slot
//
// mas_set_range() - Set up Maple Tree operation state for a different index.
// @mas: Maple Tree operation state.
// @start: New start of range in the Maple Tree.
// @last: New end of range in the Maple Tree.
//
// Move the operation state to refer to a different range.  This will
// have the effect of starting a walk from the top; see mas_next()
// to move to an adjacent index.
//
// mas_set() - Set up Maple Tree operation state for a different index.
// @mas: Maple Tree operation state.
// @index: New index into the Maple Tree.
//
// Move the operation state to refer to a different index.  This will
// have the effect of starting a walk from the top; see mas_next()
// to move to an adjacent index.
//
// mt_init_flags() - Initialise an empty maple tree with flags.
// @mt: Maple Tree
// @flags: maple tree flags.
//
// If you need to initialise a Maple Tree with special flags (eg, an
// allocation tree), use this function.
//
// Context: Any context.
//
// mt_init() - Initialise an empty maple tree.
// @mt: Maple Tree
//
// An empty Maple Tree.
//
// Context: Any context.
//
// mt_clear_in_rcu() - Switch the tree to non-RCU mode.
// @mt: The Maple Tree
//
// mt_set_in_rcu() - Switch the tree to RCU safe mode.
// @mt: The Maple Tree
//
// mt_for_each - Iterate over each entry starting at index until max.
// @__tree: The Maple Tree
// @__entry: The current entry
// @__index: The index to start the search from. Subsequently used as iterator.
// @__max: The maximum limit for @index
//
// This iterator skips all entries, which resolve to a NULL pointer,
// e.g. entries which has been reserved with XA_ZERO_ENTRY.
//

