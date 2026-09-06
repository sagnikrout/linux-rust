//! Automatically rewritten from C to Rust
//! Source: scripts/gendwarfksyms/die.c
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
// Copyright (C) 2024 Google LLC
//

pub const DIE_HASH_BITS: c_int = 16;
// {die->addr, state} -> struct die *
    static HASHTABLE_DEFINE(die_map, 1 << DIE_HASH_BITS);
    static unsigned int map_hits;
    static unsigned int map_misses;
#[no_mangle]
pub unsafe extern "C" fn die_hash(addr: uintptr_t, state: enum die_state) -> c_uint {
    static inline unsigned int die_hash(uintptr_t addr, enum die_state state)
    {
    return hash_32(addr_hash(addr) ^ (unsigned int)state);
    }
#[no_mangle]
unsafe extern "C" fn init_die(cd: *mut die) {
    static void init_die(struct die *cd)
    {
    cd.state = DIE_INCOMPLETE;
    cd.mapped = false;
    cd.fqn = core::ptr::null_mut();
    cd.tag = -1;
    cd.addr = 0;
    INIT_LIST_HEAD(&cd.fragments);
    }
    static struct die *create_die(Dwarf_Die *die, enum die_state state)
    {
    struct die *cd;
    cd = xmalloc(sizeof(*cd));
    init_die(cd);
    cd.addr = (uintptr_t)die.addr;
    hash_add(die_map, &cd.hash, die_hash(cd.addr, state));
    return cd;
    }
#[no_mangle]
pub unsafe extern "C" fn __die_map_get(addr: uintptr_t, state: enum die_state, res: *mut die) -> c_int {
    int __die_map_get(uintptr_t addr, enum die_state state, struct die **res)
    {
    struct die *cd;
    hash_for_each_possible(die_map, cd, hash, die_hash(addr, state)) {
    if (cd.addr == addr && cd.state == state) {
// res = cd;
    return 0;
    }
    }
    return -1;
    }
    struct die *die_map_get(Dwarf_Die *die, enum die_state state)
    {
    struct die *cd;
    if (__die_map_get((uintptr_t)die.addr, state, &cd) == 0) {
    map_hits++;
    return cd;
    }
    map_misses++;
    return create_die(die, state);
    }
#[no_mangle]
unsafe extern "C" fn reset_die(cd: *mut die) {
    static void reset_die(struct die *cd)
    {
    struct die_fragment *tmp;
    struct die_fragment *df;
    list_for_each_entry_safe(df, tmp, &cd.fragments, list) {
    if (df.type == FRAGMENT_STRING)
    free(df.data.str);
    free(df);
    }
    if (cd.fqn && *cd.fqn)
    free(cd.fqn);
    init_die(cd);
    }
#[no_mangle]
pub unsafe extern "C" fn die_map_for_each(func: die_map_callback_t, arg: *mut c_void) {
    void die_map_for_each(die_map_callback_t func, void *arg)
    {
    struct hlist_node *tmp;
    struct die *cd;
    hash_for_each_safe(die_map, cd, tmp, hash) {
    func(cd, arg);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn die_map_free() {
    void die_map_free(void)
    {
    struct hlist_node *tmp;
    unsigned int stats[DIE_LAST + 1];
    struct die *cd;
    int i;
    memset(stats, 0, sizeof(stats));
    hash_for_each_safe(die_map, cd, tmp, hash) {
    stats[cd.state]++;
    reset_die(cd);
    free(cd);
    }
    hash_init(die_map);
    if (map_hits + map_misses > 0)
    debug("hits %u, misses %u (hit rate %.02f%%)", map_hits,
    map_misses,
    (100.0f * map_hits) / (map_hits + map_misses));
    for (i = 0; i <= DIE_LAST; i++)
    debug("%s: %u entries", die_state_name(i), stats[i]);
    }
    static struct die_fragment *append_item(struct die *cd)
    {
    struct die_fragment *df;
    df = xmalloc(sizeof(*df));
    df.type = FRAGMENT_EMPTY;
    list_add_tail(&df.list, &cd.fragments);
    return df;
    }
#[no_mangle]
pub unsafe extern "C" fn die_map_add_string(cd: *mut die, str: *const c_char) {
    void die_map_add_string(struct die *cd, const char *str)
    {
    struct die_fragment *df;
    if (!cd)
    return;
    df = append_item(cd);
    df.data.str = xstrdup(str);
    df.type = FRAGMENT_STRING;
    }
#[no_mangle]
pub unsafe extern "C" fn die_map_add_linebreak(cd: *mut die, linebreak: c_int) {
    void die_map_add_linebreak(struct die *cd, int linebreak)
    {
    struct die_fragment *df;
    if (!cd)
    return;
    df = append_item(cd);
    df.data.linebreak = linebreak;
    df.type = FRAGMENT_LINEBREAK;
    }
#[no_mangle]
pub unsafe extern "C" fn die_map_add_die(cd: *mut die, child: *mut die) {
    void die_map_add_die(struct die *cd, struct die *child)
    {
    struct die_fragment *df;
    if (!cd)
    return;
    df = append_item(cd);
    df.data.addr = child.addr;
    df.type = FRAGMENT_DIE;
    }
