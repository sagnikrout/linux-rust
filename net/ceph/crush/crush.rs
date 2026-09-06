//! Automatically rewritten from C to Rust
//! Source: net/ceph/crush/crush.c
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

    const char *crush_bucket_alg_name(int alg)
    {
    switch (alg) {
    case CRUSH_BUCKET_UNIFORM: return "uniform";
    case CRUSH_BUCKET_LIST: return "list";
    case CRUSH_BUCKET_TREE: return "tree";
    case CRUSH_BUCKET_STRAW: return "straw";
    case CRUSH_BUCKET_STRAW2: return "straw2";
    default: return "unknown";
    }
    }
//
// crush_get_bucket_item_weight - Get weight of an item in given bucket
// @b: bucket pointer
// @p: item index in bucket
//
#[no_mangle]
pub unsafe extern "C" fn crush_get_bucket_item_weight(b: *const crush_bucket, p: c_int) -> c_int {
    int crush_get_bucket_item_weight(const struct crush_bucket *b, int p)
    {
    if ((__u32)p >= b.size)
    return 0;
    switch (b.alg) {
    case CRUSH_BUCKET_UNIFORM:
    return ((struct crush_bucket_uniform *)b).item_weight;
    case CRUSH_BUCKET_LIST:
    return ((struct crush_bucket_list *)b).item_weights[p];
    case CRUSH_BUCKET_TREE:
    return ((struct crush_bucket_tree *)b).node_weights[crush_calc_tree_node(p)];
    case CRUSH_BUCKET_STRAW:
    return ((struct crush_bucket_straw *)b).item_weights[p];
    case CRUSH_BUCKET_STRAW2:
    return ((struct crush_bucket_straw2 *)b).item_weights[p];
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_uniform(b: *mut crush_bucket_uniform) {
    void crush_destroy_bucket_uniform(struct crush_bucket_uniform *b)
    {
    kfree(b.h.items);
    }
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_list(b: *mut crush_bucket_list) {
    void crush_destroy_bucket_list(struct crush_bucket_list *b)
    {
    kfree(b.item_weights);
    kfree(b.sum_weights);
    kfree(b.h.items);
    }
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_tree(b: *mut crush_bucket_tree) {
    void crush_destroy_bucket_tree(struct crush_bucket_tree *b)
    {
    kfree(b.h.items);
    kfree(b.node_weights);
    }
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_straw(b: *mut crush_bucket_straw) {
    void crush_destroy_bucket_straw(struct crush_bucket_straw *b)
    {
    kfree(b.straws);
    kfree(b.item_weights);
    kfree(b.h.items);
    }
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket_straw2(b: *mut crush_bucket_straw2) {
    void crush_destroy_bucket_straw2(struct crush_bucket_straw2 *b)
    {
    kfree(b.item_weights);
    kfree(b.h.items);
    }
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_bucket(b: *mut crush_bucket) {
    void crush_destroy_bucket(struct crush_bucket *b)
    {
    switch (b.alg) {
    case CRUSH_BUCKET_UNIFORM:
    crush_destroy_bucket_uniform((struct crush_bucket_uniform *)b);
    break;
    case CRUSH_BUCKET_LIST:
    crush_destroy_bucket_list((struct crush_bucket_list *)b);
    break;
    case CRUSH_BUCKET_TREE:
    crush_destroy_bucket_tree((struct crush_bucket_tree *)b);
    break;
    case CRUSH_BUCKET_STRAW:
    crush_destroy_bucket_straw((struct crush_bucket_straw *)b);
    break;
    case CRUSH_BUCKET_STRAW2:
    crush_destroy_bucket_straw2((struct crush_bucket_straw2 *)b);
    break;
    }
    kfree(b);
    }
//
// crush_destroy - Destroy a crush_map
// @map: crush_map pointer
//
#[no_mangle]
pub unsafe extern "C" fn crush_destroy(map: *mut crush_map) {
    void crush_destroy(struct crush_map *map)
    {
// buckets
    if (map.buckets) {
    __s32 b;
    for (b = 0; b < map.max_buckets; b++) {
    if (map.buckets[b] == core::ptr::null_mut())
    continue;
    crush_destroy_bucket(map.buckets[b]);
    }
    kfree(map.buckets);
    }
// rules
    if (map.rules) {
    __u32 b;
    for (b = 0; b < map.max_rules; b++)
    crush_destroy_rule(map.rules[b]);
    kfree(map.rules);
    }

    kfree(map.choose_tries);

    clear_crush_names(&map.type_names);
    clear_crush_names(&map.names);
    clear_choose_args(map);

    kfree(map);
    }
#[no_mangle]
pub unsafe extern "C" fn crush_destroy_rule(rule: *mut crush_rule) {
    void crush_destroy_rule(struct crush_rule *rule)
    {
    kfree(rule);
    }
