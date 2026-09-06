//! Automatically rewritten from C to Rust
//! Source: tools/testing/radix-tree/regression3.c
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
// Regression3
// Description:
// Helper radix_tree_iter_retry resets next_index to the current index.
// In following radix_tree_next_slot current chunk size becomes zero.
// This isn't checked and it tries to dereference null pointer in slot.
//
// Helper radix_tree_iter_resume reset slot to NULL and next_index to index + 1,
// for tagger iteraction it also must reset cached tags in iterator to abort
// next radix_tree_next_slot and go to slow-path into radix_tree_next_chunk.
//
// Running:
// This test should run to completion immediately. The above bug would
// cause it to segfault.
//
// Upstream commit:
// Not yet
//

#[no_mangle]
pub unsafe extern "C" fn regression3_test() {
    void regression3_test(void)
    {
    RADIX_TREE(root, GFP_KERNEL);
    void *ptr0 = (void *)4ul;
    void *ptr = (void *)8ul;
    struct radix_tree_iter iter;
    void **slot;
    bool first;
    printv(1, "running regression test 3 (should take milliseconds)\n");
    radix_tree_insert(&root, 0, ptr0);
    radix_tree_tag_set(&root, 0, 0);
    first = true;
    radix_tree_for_each_tagged(slot, &root, &iter, 0, 0) {
    printv(2, "tagged %ld %p\n", iter.index, *slot);
    if (first) {
    radix_tree_insert(&root, 1, ptr);
    radix_tree_tag_set(&root, 1, 0);
    first = false;
    }
    if (radix_tree_deref_retry(*slot)) {
    printv(2, "retry at %ld\n", iter.index);
    slot = radix_tree_iter_retry(&iter);
    continue;
    }
    }
    radix_tree_delete(&root, 1);
    first = true;
    radix_tree_for_each_slot(slot, &root, &iter, 0) {
    printv(2, "slot %ld %p\n", iter.index, *slot);
    if (first) {
    radix_tree_insert(&root, 1, ptr);
    first = false;
    }
    if (radix_tree_deref_retry(*slot)) {
    printv(2, "retry at %ld\n", iter.index);
    slot = radix_tree_iter_retry(&iter);
    continue;
    }
    }
    radix_tree_for_each_slot(slot, &root, &iter, 0) {
    printv(2, "slot %ld %p\n", iter.index, *slot);
    if (!iter.index) {
    printv(2, "next at %ld\n", iter.index);
    slot = radix_tree_iter_resume(slot, &iter);
    }
    }
    radix_tree_tag_set(&root, 0, 0);
    radix_tree_tag_set(&root, 1, 0);
    radix_tree_for_each_tagged(slot, &root, &iter, 0, 0) {
    printv(2, "tagged %ld %p\n", iter.index, *slot);
    if (!iter.index) {
    printv(2, "next at %ld\n", iter.index);
    slot = radix_tree_iter_resume(slot, &iter);
    }
    }
    radix_tree_delete(&root, 0);
    radix_tree_delete(&root, 1);
    printv(1, "regression test 3 passed\n");
    }
