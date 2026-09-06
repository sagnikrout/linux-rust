//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/regression2.c
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
// Regression2
// Description:
// Toshiyuki Okajima describes the following radix-tree bug:
//
// In the following case, we can get a hangup on
// radix_radix_tree_gang_lookup_tag_slot.
//
// 0.  The radix tree contains RADIX_TREE_MAP_SIZE items. And the tag of
// a certain item has PAGECACHE_TAG_DIRTY.
// 1.  radix_tree_range_tag_if_tagged(, start, end, , PAGECACHE_TAG_DIRTY,
// PAGECACHE_TAG_TOWRITE) is called to add PAGECACHE_TAG_TOWRITE tag
// for the tag which has PAGECACHE_TAG_DIRTY. However, there is no tag with
// PAGECACHE_TAG_DIRTY within the range from start to end. As the result,
// There is no tag with PAGECACHE_TAG_TOWRITE but the root tag has
// PAGECACHE_TAG_TOWRITE.
// 2.  An item is added into the radix tree and then the level of it is
// extended into 2 from 1. At that time, the new radix tree node succeeds
// the tag status of the root tag. Therefore the tag of the new radix tree
// node has PAGECACHE_TAG_TOWRITE but there is not slot with
// PAGECACHE_TAG_TOWRITE tag in the child node of the new radix tree node.
// 3.  The tag of a certain item is cleared with PAGECACHE_TAG_DIRTY.
// 4.  All items within the index range from 0 to RADIX_TREE_MAP_SIZE - 1 are
// released. (Only the item which index is RADIX_TREE_MAP_SIZE exist in the
// radix tree.) As the result, the slot of the radix tree node is NULL but
// the tag which corresponds to the slot has PAGECACHE_TAG_TOWRITE.
// 5.  radix_tree_gang_lookup_tag_slot(PAGECACHE_TAG_TOWRITE) calls
// __lookup_tag. __lookup_tag returns with 0. And __lookup_tag doesn't
// change the index that is the input and output parameter. Because the 1st
// slot of the radix tree node is NULL, but the tag which corresponds to
// the slot has PAGECACHE_TAG_TOWRITE.
// Therefore radix_tree_gang_lookup_tag_slot tries to get some items by
// calling __lookup_tag, but it cannot get any items forever.
//
// The fix is to change that radix_tree_tag_if_tagged doesn't tag the root tag
// if it doesn't set any tags within the specified range.
//
// Running:
// This test should run to completion immediately. The above bug would cause it
// to hang indefinitely.
//
// Upstream commit:
// Not yet
//

    static RADIX_TREE(mt_tree, GFP_KERNEL);
    let mut page_count: c_ulong = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page {
    pub index: c_ulong,
}

    static struct page *page_alloc(void)
    {
    struct page *p;
    p = malloc(sizeof(struct page));
    p.index = page_count++;
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn regression2_test() {
    void regression2_test(void)
    {
    int i;
    struct page *p;
    let mut max_slots: c_int = RADIX_TREE_MAP_SIZE;
    unsigned long int start, end;
    struct page *pages[1];
    printv(1, "running regression test 2 (should take milliseconds)\n");
// 0.
    for (i = 0; i <= max_slots - 1; i++) {
    p = page_alloc();
    radix_tree_insert(&mt_tree, i, p);
    }
    radix_tree_tag_set(&mt_tree, max_slots - 1, PAGECACHE_TAG_DIRTY);
// 1.
    start = 0;
    end = max_slots - 2;
    tag_tagged_items(&mt_tree, start, end, 1,
    PAGECACHE_TAG_DIRTY, PAGECACHE_TAG_TOWRITE);
// 2.
    p = page_alloc();
    radix_tree_insert(&mt_tree, max_slots, p);
// 3.
    radix_tree_tag_clear(&mt_tree, max_slots - 1, PAGECACHE_TAG_DIRTY);
// 4.
    for (i = max_slots - 1; i >= 0; i--)
    free(radix_tree_delete(&mt_tree, i));
// 5.
// NOTE: start should not be 0 because radix_tree_gang_lookup_tag_slot
// can return.
    start = 1;
    end = max_slots - 2;
    radix_tree_gang_lookup_tag_slot(&mt_tree, (void ***)pages, start, end,
    PAGECACHE_TAG_TOWRITE);
// We remove all the remained nodes
    free(radix_tree_delete(&mt_tree, max_slots));
    BUG_ON(!radix_tree_empty(&mt_tree));
    printv(1, "regression test 2, done\n");
    }
