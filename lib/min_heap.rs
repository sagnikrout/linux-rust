//! Automatically rewritten from C to Rust
//! Source: lib/min_heap.c
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

#[no_mangle]
pub unsafe extern "C" fn __min_heap_init(heap: *mut min_heap_char, data: *mut c_void, size: usize) {
    void __min_heap_init(min_heap_char *heap, void *data, size_t size)
    {
    __min_heap_init_inline(heap, data, size);
    }
    EXPORT_SYMBOL(__min_heap_init);
    void *__min_heap_peek(struct min_heap_char *heap)
    {
    return __min_heap_peek_inline(heap);
    }
    EXPORT_SYMBOL(__min_heap_peek);
#[no_mangle]
pub unsafe extern "C" fn __min_heap_full(heap: *mut min_heap_char) -> bool {
    bool __min_heap_full(min_heap_char *heap)
    {
    return __min_heap_full_inline(heap);
    }
    EXPORT_SYMBOL(__min_heap_full);
    void __min_heap_sift_down(min_heap_char *heap, size_t pos, size_t elem_size,
    const struct min_heap_callbacks *func, void *args)
    {
    __min_heap_sift_down_inline(heap, pos, elem_size, func, args);
    }
    EXPORT_SYMBOL(__min_heap_sift_down);
    void __min_heap_sift_up(min_heap_char *heap, size_t elem_size, size_t idx,
    const struct min_heap_callbacks *func, void *args)
    {
    __min_heap_sift_up_inline(heap, elem_size, idx, func, args);
    }
    EXPORT_SYMBOL(__min_heap_sift_up);
    void __min_heapify_all(min_heap_char *heap, size_t elem_size,
    const struct min_heap_callbacks *func, void *args)
    {
    __min_heapify_all_inline(heap, elem_size, func, args);
    }
    EXPORT_SYMBOL(__min_heapify_all);
    bool __min_heap_pop(min_heap_char *heap, size_t elem_size,
    const struct min_heap_callbacks *func, void *args)
    {
    return __min_heap_pop_inline(heap, elem_size, func, args);
    }
    EXPORT_SYMBOL(__min_heap_pop);
    void __min_heap_pop_push(min_heap_char *heap, const void *element, size_t elem_size,
    const struct min_heap_callbacks *func, void *args)
    {
    __min_heap_pop_push_inline(heap, element, elem_size, func, args);
    }
    EXPORT_SYMBOL(__min_heap_pop_push);
    bool __min_heap_push(min_heap_char *heap, const void *element, size_t elem_size,
    const struct min_heap_callbacks *func, void *args)
    {
    return __min_heap_push_inline(heap, element, elem_size, func, args);
    }
    EXPORT_SYMBOL(__min_heap_push);
    bool __min_heap_del(min_heap_char *heap, size_t elem_size, size_t idx,
    const struct min_heap_callbacks *func, void *args)
    {
    return __min_heap_del_inline(heap, elem_size, idx, func, args);
    }
    EXPORT_SYMBOL(__min_heap_del);
