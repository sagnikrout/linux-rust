//! Automatically rewritten from C to Rust
//! Source: lib/generic-radix-tree.c
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


//
// Returns pointer to the specified byte @offset within @radix, or NULL if not
// allocated
//
    void *__genradix_ptr(struct __genradix *radix, size_t offset)
    {
    return __genradix_ptr_inlined(radix, offset);
    }
    EXPORT_SYMBOL(__genradix_ptr);
//
// Returns pointer to the specified byte @offset within @radix, allocating it if
// necessary - newly allocated slots are always zeroed out:
//
    void *__genradix_ptr_alloc(struct __genradix *radix, size_t offset,
    struct genradix_node **preallocated,
    gfp_t gfp_mask)
    {
    struct genradix_root *v = READ_ONCE(radix.root);
    struct genradix_node *n, *new_node = core::ptr::null_mut();
    unsigned level;
    if (preallocated)
    swap(new_node, *preallocated);
// Increase tree depth if necessary:
    while (1) {
    struct genradix_root *r = v, *new_root;
    n	= genradix_root_to_node(r);
    level	= genradix_root_to_depth(r);
    if (n && ilog2(offset) < genradix_depth_shift(level))
    break;
    if (!new_node) {
    new_node = genradix_alloc_node(gfp_mask);
    if (!new_node)
    return core::ptr::null_mut();
    }
    new_node.children[0] = n;
    new_root = ((struct genradix_root *)
    ((unsigned long) new_node | (n ? level + 1 : 0)));
    if ((v = cmpxchg_release(&radix.root, r, new_root)) == r) {
    v = new_root;
    new_node = core::ptr::null_mut();
    } else {
    new_node.children[0] = core::ptr::null_mut();
    }
    }
    while (level--) {
    struct genradix_node **p =
    &n.children[offset >> genradix_depth_shift(level)];
    offset &= genradix_depth_size(level) - 1;
    n = READ_ONCE(*p);
    if (!n) {
    if (!new_node) {
    new_node = genradix_alloc_node(gfp_mask);
    if (!new_node)
    return core::ptr::null_mut();
    }
    if (!(n = cmpxchg_release(p, core::ptr::null_mut(), new_node)))
    swap(n, new_node);
    }
    }
    if (new_node)
    genradix_free_node(new_node);
    return &n.data[offset];
    }
    EXPORT_SYMBOL(__genradix_ptr_alloc);
    void *__genradix_iter_peek(struct genradix_iter *iter,
    struct __genradix *radix,
    size_t objs_per_page)
    {
    struct genradix_root *r;
    struct genradix_node *n;
    unsigned level, i;
    if (iter.offset == SIZE_MAX)
    return core::ptr::null_mut();
    restart:
    r = READ_ONCE(radix.root);
    if (!r)
    return core::ptr::null_mut();
    n	= genradix_root_to_node(r);
    level	= genradix_root_to_depth(r);
    if (ilog2(iter.offset) >= genradix_depth_shift(level))
    return core::ptr::null_mut();
    while (level) {
    level--;
    i = (iter.offset >> genradix_depth_shift(level)) &
    (GENRADIX_ARY - 1);
    while (!n.children[i]) {
    let mut objs_per_ptr: usize = genradix_depth_size(level);
    if (iter.offset + objs_per_ptr < iter.offset) {
    iter.offset	= SIZE_MAX;
    iter.pos	= SIZE_MAX;
    return core::ptr::null_mut();
    }
    i++;
    iter.offset = round_down(iter.offset + objs_per_ptr,
    objs_per_ptr);
    iter.pos = (iter.offset >> GENRADIX_NODE_SHIFT) *
    objs_per_page;
    if (i == GENRADIX_ARY)
    goto restart;
    }
    n = n.children[i];
    }
    return &n.data[iter.offset & (GENRADIX_NODE_SIZE - 1)];
    }
    EXPORT_SYMBOL(__genradix_iter_peek);
    void *__genradix_iter_peek_prev(struct genradix_iter *iter,
    struct __genradix *radix,
    size_t objs_per_page,
    size_t obj_size_plus_page_remainder)
    {
    struct genradix_root *r;
    struct genradix_node *n;
    unsigned level, i;
    if (iter.offset == SIZE_MAX)
    return core::ptr::null_mut();
    restart:
    r = READ_ONCE(radix.root);
    if (!r)
    return core::ptr::null_mut();
    n	= genradix_root_to_node(r);
    level	= genradix_root_to_depth(r);
    if (ilog2(iter.offset) >= genradix_depth_shift(level)) {
    iter.offset = genradix_depth_size(level);
    iter.pos = (iter.offset >> GENRADIX_NODE_SHIFT) * objs_per_page;
    iter.offset -= obj_size_plus_page_remainder;
    iter.pos--;
    }
    while (level) {
    level--;
    i = (iter.offset >> genradix_depth_shift(level)) &
    (GENRADIX_ARY - 1);
    while (!n.children[i]) {
    let mut objs_per_ptr: usize = genradix_depth_size(level);
    iter.offset = round_down(iter.offset, objs_per_ptr);
    iter.pos = (iter.offset >> GENRADIX_NODE_SHIFT) * objs_per_page;
    if (!iter.offset)
    return core::ptr::null_mut();
    iter.offset -= obj_size_plus_page_remainder;
    iter.pos--;
    if (!i)
    goto restart;
    --i;
    }
    n = n.children[i];
    }
    return &n.data[iter.offset & (GENRADIX_NODE_SIZE - 1)];
    }
    EXPORT_SYMBOL(__genradix_iter_peek_prev);
#[no_mangle]
unsafe extern "C" fn genradix_free_recurse(n: *mut genradix_node, level: unsigned) {
    static void genradix_free_recurse(struct genradix_node *n, unsigned level)
    {
    if (level) {
    unsigned i;
    for (i = 0; i < GENRADIX_ARY; i++)
    if (n.children[i])
    genradix_free_recurse(n.children[i], level - 1);
    }
    genradix_free_node(n);
    }
    int __genradix_prealloc(struct __genradix *radix, size_t size,
    gfp_t gfp_mask)
    {
    size_t offset;
    for (offset = 0; offset < size; offset += GENRADIX_NODE_SIZE)
    if (!__genradix_ptr_alloc(radix, offset, core::ptr::null_mut(), gfp_mask))
    return -ENOMEM;
    return 0;
    }
    EXPORT_SYMBOL(__genradix_prealloc);
#[no_mangle]
pub unsafe extern "C" fn __genradix_free(radix: *mut __genradix) {
    void __genradix_free(struct __genradix *radix)
    {
    struct genradix_root *r = xchg(&radix.root, core::ptr::null_mut());
    genradix_free_recurse(genradix_root_to_node(r),
    genradix_root_to_depth(r));
    }
    EXPORT_SYMBOL(__genradix_free);
