//! Automatically rewritten from C to Rust
//! Source: scripts/gendwarfksyms/types.c
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
// Macro flag: #define _GNU_SOURCE

    static struct cache expansion_cache;
//
// A simple linked list of shared or owned strings to avoid copying strings
// around when not necessary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_list_entry {
    pub str: *const c_char,
    pub owned: *mut c_void,
    pub list: list_head,
}

#[no_mangle]
unsafe extern "C" fn type_list_free(list: *mut list_head) {
    static void type_list_free(struct list_head *list)
    {
    struct type_list_entry *entry;
    struct type_list_entry *tmp;
    list_for_each_entry_safe(entry, tmp, list, list) {
    if (entry.owned)
    free(entry.owned);
    free(entry);
    }
    INIT_LIST_HEAD(list);
    }
#[no_mangle]
unsafe extern "C" fn type_list_append(list: *mut list_head, s: *const c_char, owned: *mut c_void) -> c_int {
    static int type_list_append(struct list_head *list, const char *s, void *owned)
    {
    struct type_list_entry *entry;
    if (!s)
    return 0;
    entry = xmalloc(sizeof(*entry));
    entry.str = s;
    entry.owned = owned;
    list_add_tail(&entry.list, list);
    return strlen(entry.str);
    }
#[no_mangle]
unsafe extern "C" fn type_list_write(list: *mut list_head, file: *mut FILE) {
    static void type_list_write(struct list_head *list, FILE *file)
    {
    struct type_list_entry *entry;
    list_for_each_entry(entry, list, list) {
    if (entry.str)
    checkp(fputs(entry.str, file));
    }
    }
//
// An expanded type string in symtypes format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct type_expansion {
    pub name: *mut c_char,
    pub len: usize,
    pub expanded: list_head,
    pub hash: hlist_node,
}

#[no_mangle]
unsafe extern "C" fn type_expansion_init(type: *mut type_expansion) {
    static void type_expansion_init(struct type_expansion *type)
    {
    type.name = core::ptr::null_mut();
    type.len = 0;
    INIT_LIST_HEAD(&type.expanded);
    }
#[no_mangle]
pub unsafe extern "C" fn type_expansion_free(type: *mut type_expansion) {
    static inline void type_expansion_free(struct type_expansion *type)
    {
    free(type.name);
    type.name = core::ptr::null_mut();
    type.len = 0;
    type_list_free(&type.expanded);
    }
    static void type_expansion_append(struct type_expansion *type, const char *s,
    void *owned)
    {
    type.len += type_list_append(&type.expanded, s, owned);
    }
//
// type_map -- the longest expansions for each type.
//
// const char *name -> struct type_expansion
//
pub const TYPE_HASH_BITS: c_int = 12;
    static HASHTABLE_DEFINE(type_map, 1 << TYPE_HASH_BITS);
#[no_mangle]
unsafe extern "C" fn __type_map_get(name: *const c_char, res: *mut type_expansion) -> c_int {
    static int __type_map_get(const char *name, struct type_expansion **res)
    {
    struct type_expansion *e;
    hash_for_each_possible(type_map, e, hash, hash_str(name)) {
    if (!strcmp(name, e.name)) {
// res = e;
    return 0;
    }
    }
    return -1;
    }
    static struct type_expansion *type_map_add(const char *name,
    struct type_expansion *type)
    {
    struct type_expansion *e;
    if (__type_map_get(name, &e)) {
    e = xmalloc(sizeof(*e));
    type_expansion_init(e);
    e.name = xstrdup(name);
    hash_add(type_map, &e.hash, hash_str(e.name));
    if (dump_types)
    debug("adding %s", e.name);
    } else {
// Use the longest available expansion
    if (type.len <= e.len)
    return e;
    type_list_free(&e.expanded);
    if (dump_types)
    debug("replacing %s", e.name);
    }
// Take ownership of type->expanded
    list_replace_init(&type.expanded, &e.expanded);
    e.len = type.len;
    if (dump_types) {
    checkp(fputs(e.name, stderr));
    checkp(fputs(" ", stderr));
    type_list_write(&e.expanded, stderr);
    checkp(fputs("\n", stderr));
    }
    return e;
    }
    static void type_parse(const char *name, const char *str,
    struct type_expansion *type);
#[no_mangle]
unsafe extern "C" fn type_map_get(name: *const c_char, res: *mut type_expansion) -> c_int {
    static int type_map_get(const char *name, struct type_expansion **res)
    {
    struct type_expansion type;
    const char *override;
    if (!__type_map_get(name, res))
    return 0;
//
// If die_map didn't contain a type, we might still have
// a type_string kABI rule that defines it.
//
    if (stable && kabi_get_type_string(name, &override)) {
    type_expansion_init(&type);
    type_parse(name, override, &type);
// res = type_map_add(name, &type);
    type_expansion_free(&type);
    return 0;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn cmp_expansion_name(p1: *const c_void, p2: *const c_void) -> c_int {
    static int cmp_expansion_name(const void *p1, const void *p2)
    {
    struct type_expansion *const *e1 = p1;
    struct type_expansion *const *e2 = p2;
    return strcmp((*e1).name, (*e2).name);
    }
#[no_mangle]
unsafe extern "C" fn type_map_write(file: *mut FILE) {
    static void type_map_write(FILE *file)
    {
    struct type_expansion *e;
    struct hlist_node *tmp;
    struct type_expansion **es;
    let mut count: usize = 0;
    let mut i: usize = 0;
    if (!file)
    return;
    hash_for_each_safe(type_map, e, tmp, hash)
    ++count;
    es = xmalloc(count * sizeof(*es));
    hash_for_each_safe(type_map, e, tmp, hash)
    es[i++] = e;
    qsort(es, count, sizeof(*es), cmp_expansion_name);
    for (i = 0; i < count; ++i) {
    checkp(fputs(es[i].name, file));
    checkp(fputs(" ", file));
    type_list_write(&es[i].expanded, file);
    checkp(fputs("\n", file));
    }
    free(es);
    }
#[no_mangle]
unsafe extern "C" fn type_map_free() {
    static void type_map_free(void)
    {
    struct type_expansion *e;
    struct hlist_node *tmp;
    hash_for_each_safe(type_map, e, tmp, hash) {
    type_expansion_free(e);
    free(e);
    }
    hash_init(type_map);
    }
//
// CRC for a type, with an optional fully expanded type string for
// debugging.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct version {
    pub type: type_expansion,
    pub crc: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn version_init(version: *mut version) {
    static void version_init(struct version *version)
    {
    version.crc = crc32(0, core::ptr::null_mut(), 0);
    type_expansion_init(&version.type);
    }
#[no_mangle]
unsafe extern "C" fn version_free(version: *mut version) {
    static void version_free(struct version *version)
    {
    type_expansion_free(&version.type);
    }
#[no_mangle]
unsafe extern "C" fn version_add(version: *mut version, s: *const c_char) {
    static void version_add(struct version *version, const char *s)
    {
    version.crc = crc32(version.crc, (void *)s, strlen(s));
    if (dump_versions)
    type_expansion_append(&version.type, s, core::ptr::null_mut());
    }
//
// Type reference format: <prefix>#<name>, where prefix:
// s -> structure
// u -> union
// e -> enum
// t -> typedef
//
// Names with spaces are additionally wrapped in single quotes.
//
#[no_mangle]
pub unsafe extern "C" fn is_type_prefix(s: *const c_char) -> bool {
    static inline bool is_type_prefix(const char *s)
    {
    return (s[0] == 's' || s[0] == 'u' || s[0] == 'e' || s[0] == 't') &&
    s[1] == '#';
    }
#[no_mangle]
unsafe extern "C" fn get_type_prefix(tag: c_int) -> c_char {
    static char get_type_prefix(int tag)
    {
    switch (tag) {
    case DW_TAG_class_type:
    case DW_TAG_structure_type:
    return 's';
    case DW_TAG_union_type:
    return 'u';
    case DW_TAG_enumeration_type:
    return 'e';
    case DW_TAG_typedef_type:
    return 't';
    default:
    return 0;
    }
    }
    static char *get_type_name(struct die *cache)
    {
    const char *quote;
    char prefix;
    char *name;
    if (cache.state == DIE_INCOMPLETE) {
    warn("found incomplete cache entry: %p", cache);
    return core::ptr::null_mut();
    }
    if (cache.state == DIE_SYMBOL || cache.state == DIE_FQN)
    return core::ptr::null_mut();
    if (!cache.fqn || !*cache.fqn)
    return core::ptr::null_mut();
    prefix = get_type_prefix(cache.tag);
    if (!prefix)
    return core::ptr::null_mut();
// Wrap names with spaces in single quotes
    quote = strstr(cache.fqn, " ") ? "'" : "";
// <prefix>#<type_name>\0
    if (asprintf(&name, "%c#%s%s%s", prefix, quote, cache.fqn, quote) < 0)
    error("asprintf failed for '%s'", cache.fqn);
    return name;
    }
    static void __calculate_version(struct version *version,
    struct type_expansion *type)
    {
    struct type_list_entry *entry;
    struct type_expansion *e;
// Calculate a CRC over an expanded type string
    list_for_each_entry(entry, &type.expanded, list) {
    if (is_type_prefix(entry.str)) {
    if (type_map_get(entry.str, &e))
    error("unknown type reference to '%s' when expanding '%s'",
    entry.str, type.name);
//
// It's sufficient to expand each type reference just
// once to detect changes.
//
    if (cache_was_expanded(&expansion_cache, e)) {
    version_add(version, entry.str);
    } else {
    cache_mark_expanded(&expansion_cache, e);
    __calculate_version(version, e);
    }
    } else {
    version_add(version, entry.str);
    }
    }
    }
    static void calculate_version(struct version *version,
    struct type_expansion *type)
    {
    version_init(version);
    __calculate_version(version, type);
    cache_free(&expansion_cache);
    }
#[no_mangle]
unsafe extern "C" fn __type_expand(cache: *mut die, type: *mut type_expansion) {
    static void __type_expand(struct die *cache, struct type_expansion *type)
    {
    struct die_fragment *df;
    struct die *child;
    char *name;
    list_for_each_entry(df, &cache.fragments, list) {
    switch (df.type) {
    case FRAGMENT_STRING:
    type_expansion_append(type, df.data.str, core::ptr::null_mut());
    break;
    case FRAGMENT_DIE:
// Use a complete die_map expansion if available
    if (__die_map_get(df.data.addr, DIE_COMPLETE,
    &child) &&
    __die_map_get(df.data.addr, DIE_UNEXPANDED,
    &child))
    error("unknown child: %" PRIxPTR,
    df.data.addr);
    name = get_type_name(child);
    if (name)
    type_expansion_append(type, name, name);
    else
    __type_expand(child, type);
    break;
    case FRAGMENT_LINEBREAK:
//
// Keep whitespace in the symtypes format, but avoid
// repeated spaces.
//
    if (list_is_last(&df.list, &cache.fragments) ||
    list_next_entry(df, list).type !=
    FRAGMENT_LINEBREAK)
    type_expansion_append(type, " ", core::ptr::null_mut());
    break;
    default:
    error("empty die_fragment in %p", cache);
    }
    }
    }
    static void type_expand(const char *name, struct die *cache,
    struct type_expansion *type)
    {
    const char *override;
    type_expansion_init(type);
    if (stable && kabi_get_type_string(name, &override))
    type_parse(name, override, type);
    else
    __type_expand(cache, type);
    }
    static void type_parse(const char *name, const char *str,
    struct type_expansion *type)
    {
    char *fragment;
    let mut start: usize = 0;
    size_t end;
    size_t pos;
    if (!*str)
    error("empty type string override for '%s'", name);
    for (pos = 0; str[pos]; ++pos) {
    bool empty;
    let mut marker: c_char = ' ';
    if (!is_type_prefix(&str[pos]))
    continue;
    end = pos + 2;
//
// Find the end of the type reference. If the type name contains
// spaces, it must be in single quotes.
//
    if (str[end] == '\'') {
    marker = '\'';
    ++end;
    }
    while (str[end] && str[end] != marker)
    ++end;
// Check that we have a non-empty type name
    if (marker == '\'') {
    if (str[end] != marker)
    error("incomplete %c# type reference for '%s' (string : '%s')",
    str[pos], name, str);
    empty = end == pos + 3;
    ++end;
    } else {
    empty = end == pos + 2;
    }
    if (empty)
    error("empty %c# type name for '%s' (string: '%s')",
    str[pos], name, str);
// Append the part of the string before the type reference
    if (pos > start) {
    fragment = xstrndup(&str[start], pos - start);
    type_expansion_append(type, fragment, fragment);
    }
//
// Append the type reference -- note that if the reference
// is invalid, i.e. points to a non-existent type, we will
// print out an error when calculating versions.
//
    fragment = xstrndup(&str[pos], end - pos);
    type_expansion_append(type, fragment, fragment);
    start = end;
    pos = end - 1;
    }
// Append the rest of the type string, if there's any left
    if (str[start])
    type_expansion_append(type, &str[start], core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn expand_type(cache: *mut die, arg: *mut c_void) {
    static void expand_type(struct die *cache, void *arg)
    {
    struct type_expansion type;
    char *name;
    if (cache.mapped)
    return;
    cache.mapped = true;
//
// Skip unexpanded die_map entries if there's a complete
// expansion available for this DIE.
//
    if (cache.state == DIE_UNEXPANDED &&
    !__die_map_get(cache.addr, DIE_COMPLETE, &cache)) {
    if (cache.mapped)
    return;
    cache.mapped = true;
    }
    name = get_type_name(cache);
    if (!name)
    return;
    debug("%s", name);
    type_expand(name, cache, &type);
    type_map_add(name, &type);
    type_expansion_free(&type);
    free(name);
    }
#[no_mangle]
unsafe extern "C" fn expand_symbol(sym: *mut symbol, arg: *mut c_void) {
    static void expand_symbol(struct symbol *sym, void *arg)
    {
    struct type_expansion type;
    struct version version;
    struct die *cache;
//
// No need to expand again unless we want a symtypes file entry
// for the symbol. Note that this means `sym` has the same address
// as another symbol that was already processed.
//
    if (!symtypes && sym.state == SYMBOL_PROCESSED)
    return;
    if (__die_map_get(sym.die_addr, DIE_SYMBOL, &cache))
    return; /* We'll warn about missing CRCs later. */
    type_expand(sym.name, cache, &type);
// If the symbol already has a version, don't calculate it again.
    if (sym.state != SYMBOL_PROCESSED) {
    calculate_version(&version, &type);
    symbol_set_crc(sym, version.crc);
    debug("%s = %lx", sym.name, version.crc);
    if (dump_versions) {
    checkp(fputs(sym.name, stderr));
    checkp(fputs(" ", stderr));
    type_list_write(&version.type.expanded, stderr);
    checkp(fputs("\n", stderr));
    }
    version_free(&version);
    }
// These aren't needed in type_map unless we want a symtypes file.
    if (symtypes)
    type_map_add(sym.name, &type);
    type_expansion_free(&type);
    }
#[no_mangle]
pub unsafe extern "C" fn generate_symtypes_and_versions(file: *mut FILE) {
    void generate_symtypes_and_versions(FILE *file)
    {
    cache_init(&expansion_cache);
//
// die_map processing:
//
// 1. die_map contains all types referenced in exported symbol
// signatures, but can contain duplicates just like the original
// DWARF, and some references may not be fully expanded depending
// on how far we processed the DIE tree for that specific symbol.
//
// For each die_map entry, find the longest available expansion,
// and add it to type_map.
//
    die_map_for_each(expand_type, core::ptr::null_mut());
//
// 2. For each exported symbol, expand the die_map type, and use
// type_map expansions to calculate a symbol version from the
// fully expanded type string.
//
    symbol_for_each(expand_symbol, core::ptr::null_mut());
//
// 3. If a symtypes file is requested, write type_map contents to
// the file.
//
    type_map_write(file);
    type_map_free();
    }
