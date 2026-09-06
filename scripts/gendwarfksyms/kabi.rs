//! Automatically rewritten from C to Rust
//! Source: scripts/gendwarfksyms/kabi.c
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

//
// The rule section consists of four null-terminated strings per
// entry:
//
// 1. version
// Entry format version. Must match KABI_RULE_VERSION.
//
// 2. type
// Type of the kABI rule. Must be one of the tags defined below.
//
// 3. target
// Rule-dependent target, typically the fully qualified name of
// the target DIE.
//
// 4. value
// Rule-dependent value.
//

    (/* version\0 */ 2 + /* type\0 */ 2 + /* target\0" */ 1 + \
// value\0 */ 1)

//
// Rule: declonly
// - For the struct/enum/union in the target field, treat it as a
// declaration only even if a definition is available.
//

//
// Rule: enumerator_ignore
// - For the enum_field in the target field, ignore the enumerator.
//

//
// Rule: enumerator_value
// - For the fqn_field in the target field, set the value to the
// unsigned integer in the value field.
//

//
// Rule: byte_size
// - For the fqn_field in the target field, set the byte_size
// attribute to the value in the value field.
//

//
// Rule: type_string
// - For the type reference in the fqn field, use the type string
// in the value field.
//

    enum kabi_rule_type {
    KABI_RULE_TYPE_UNKNOWN,
    KABI_RULE_TYPE_DECLONLY,
    KABI_RULE_TYPE_ENUMERATOR_IGNORE,
    KABI_RULE_TYPE_ENUMERATOR_VALUE,
    KABI_RULE_TYPE_BYTE_SIZE,
    KABI_RULE_TYPE_TYPE_STRING,
    };
pub const RULE_HASH_BITS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rule {
    pub type: enum kabi_rule_type,
    pub target: *const c_char,
    pub value: *const c_char,
    pub hash: hlist_node,
}

// { type, target } -> struct rule
    static HASHTABLE_DEFINE(rules, 1 << RULE_HASH_BITS);
    static inline unsigned int rule_values_hash(enum kabi_rule_type type,
    const char *target)
    {
    return hash_32(type) ^ hash_str(target);
    }
#[no_mangle]
pub unsafe extern "C" fn rule_hash(rule: *const rule) -> c_uint {
    static inline unsigned int rule_hash(const struct rule *rule)
    {
    return rule_values_hash(rule.type, rule.target);
    }
    static inline const char *get_rule_field(const char **pos, ssize_t *left)
    {
    const char *start = *pos;
    size_t len;
    if (*left <= 0)
    error("unexpected end of kABI rules");
    len = strnlen(start, *left) + 1;
// pos += len;
// left -= len;
    return start;
    }
#[no_mangle]
pub unsafe extern "C" fn kabi_read_rules(fd: c_int) {
    void kabi_read_rules(int fd)
    {
    GElf_Shdr shdr_mem;
    GElf_Shdr *shdr;
    Elf_Data *rule_data = core::ptr::null_mut();
    Elf_Scn *scn;
    Elf *elf;
    size_t shstrndx;
    const char *rule_str;
    ssize_t left;
    int i;
    const struct {
    enum kabi_rule_type type;
    const char *tag;
    } rule_types[] = {
    {
    .type = KABI_RULE_TYPE_DECLONLY,
    .tag = KABI_RULE_TAG_DECLONLY,
    },
    {
    .type = KABI_RULE_TYPE_ENUMERATOR_IGNORE,
    .tag = KABI_RULE_TAG_ENUMERATOR_IGNORE,
    },
    {
    .type = KABI_RULE_TYPE_ENUMERATOR_VALUE,
    .tag = KABI_RULE_TAG_ENUMERATOR_VALUE,
    },
    {
    .type = KABI_RULE_TYPE_BYTE_SIZE,
    .tag = KABI_RULE_TAG_BYTE_SIZE,
    },
    {
    .type = KABI_RULE_TYPE_TYPE_STRING,
    .tag = KABI_RULE_TAG_TYPE_STRING,
    },
    };
    if (!stable)
    return;
    if (elf_version(EV_CURRENT) != EV_CURRENT)
    error("elf_version failed: %s", elf_errmsg(-1));
    elf = elf_begin(fd, ELF_C_READ_MMAP, core::ptr::null_mut());
    if (!elf)
    error("elf_begin failed: %s", elf_errmsg(-1));
    if (elf_getshdrstrndx(elf, &shstrndx) < 0)
    error("elf_getshdrstrndx failed: %s", elf_errmsg(-1));
    scn = elf_nextscn(elf, core::ptr::null_mut());
    while (scn) {
    const char *sname;
    shdr = gelf_getshdr(scn, &shdr_mem);
    if (!shdr)
    error("gelf_getshdr failed: %s", elf_errmsg(-1));
    sname = elf_strptr(elf, shstrndx, shdr.sh_name);
    if (!sname)
    error("elf_strptr failed: %s", elf_errmsg(-1));
    if (!strcmp(sname, KABI_RULE_SECTION)) {
    rule_data = elf_getdata(scn, core::ptr::null_mut());
    if (!rule_data)
    error("elf_getdata failed: %s", elf_errmsg(-1));
    break;
    }
    scn = elf_nextscn(elf, scn);
    }
    if (!rule_data) {
    debug("kABI rules not found");
    check(elf_end(elf));
    return;
    }
    rule_str = rule_data.d_buf;
    left = shdr.sh_size;
    if (left < KABI_RULE_MIN_ENTRY_SIZE)
    error("kABI rule section too small: %zd bytes", left);
    if (rule_str[left - 1] != '\0')
    error("kABI rules are not null-terminated");
    while (left > KABI_RULE_MIN_ENTRY_SIZE) {
    let mut type: enum kabi_rule_type = KABI_RULE_TYPE_UNKNOWN;
    const char *field;
    struct rule *rule;
// version
    field = get_rule_field(&rule_str, &left);
    if (strcmp(field, KABI_RULE_VERSION))
    error("unsupported kABI rule version: '%s'", field);
// type
    field = get_rule_field(&rule_str, &left);
    for (i = 0; i < ARRAY_SIZE(rule_types); i++) {
    if (!strcmp(field, rule_types[i].tag)) {
    type = rule_types[i].type;
    break;
    }
    }
    if (type == KABI_RULE_TYPE_UNKNOWN)
    error("unsupported kABI rule type: '%s'", field);
    rule = xmalloc(sizeof(*rule));
    rule.type = type;
    rule.target = xstrdup(get_rule_field(&rule_str, &left));
    rule.value = xstrdup(get_rule_field(&rule_str, &left));
    hash_add(rules, &rule.hash, rule_hash(rule));
    debug("kABI rule: type: '%s', target: '%s', value: '%s'", field,
    rule.target, rule.value);
    }
    if (left > 0)
    warn("unexpected data at the end of the kABI rules section");
    check(elf_end(elf));
    }
    static char *get_enumerator_target(const char *fqn, const char *field)
    {
    char *target = core::ptr::null_mut();
    if (asprintf(&target, "%s %s", fqn, field) < 0)
    error("asprintf failed for '%s %s'", fqn, field);
    return target;
    }
    static struct rule *find_rule(enum kabi_rule_type type, const char *target)
    {
    struct rule *rule;
    if (!stable)
    return core::ptr::null_mut();
    if (!target || !*target)
    return core::ptr::null_mut();
    hash_for_each_possible(rules, rule, hash,
    rule_values_hash(type, target)) {
    if (rule.type == type && !strcmp(target, rule.target))
    return rule;
    }
    return core::ptr::null_mut();
    }
    static struct rule *find_enumerator_rule(enum kabi_rule_type type,
    const char *fqn, const char *field)
    {
    struct rule *rule;
    char *target;
    if (!stable)
    return core::ptr::null_mut();
    if (!fqn || !*fqn || !field || !*field)
    return core::ptr::null_mut();
    target = get_enumerator_target(fqn, field);
    rule = find_rule(type, target);
    free(target);
    return rule;
    }
#[no_mangle]
pub unsafe extern "C" fn kabi_is_declonly(fqn: *const c_char) -> bool {
    bool kabi_is_declonly(const char *fqn)
    {
    return !!find_rule(KABI_RULE_TYPE_DECLONLY, fqn);
    }
#[no_mangle]
unsafe extern "C" fn get_ulong_value(value: *const c_char) -> c_ulong {
    static unsigned long get_ulong_value(const char *value)
    {
    let mut result: c_ulong = 0;
    char *endptr = core::ptr::null_mut();
    errno = 0;
    result = strtoul(value, &endptr, 10);
    if (errno || *endptr)
    error("invalid unsigned value '%s'", value);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn kabi_is_enumerator_ignored(fqn: *const c_char, field: *const c_char) -> bool {
    bool kabi_is_enumerator_ignored(const char *fqn, const char *field)
    {
    return !!find_enumerator_rule(KABI_RULE_TYPE_ENUMERATOR_IGNORE, fqn,
    field);
    }
    bool kabi_get_enumerator_value(const char *fqn, const char *field,
    unsigned long *value)
    {
    struct rule *rule;
    rule = find_enumerator_rule(KABI_RULE_TYPE_ENUMERATOR_VALUE, fqn,
    field);
    if (rule) {
// value = get_ulong_value(rule->value);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn kabi_get_byte_size(fqn: *const c_char, value: *mut c_ulong) -> bool {
    bool kabi_get_byte_size(const char *fqn, unsigned long *value)
    {
    struct rule *rule;
    rule = find_rule(KABI_RULE_TYPE_BYTE_SIZE, fqn);
    if (rule) {
// value = get_ulong_value(rule->value);
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn kabi_get_type_string(type: *const c_char, str: *const c_char) -> bool {
    bool kabi_get_type_string(const char *type, const char **str)
    {
    struct rule *rule;
    rule = find_rule(KABI_RULE_TYPE_TYPE_STRING, type);
    if (rule) {
// str = rule->value;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn kabi_free() {
    void kabi_free(void)
    {
    struct hlist_node *tmp;
    struct rule *rule;
    hash_for_each_safe(rules, rule, tmp, hash) {
    free((void *)rule.target);
    free((void *)rule.value);
    free(rule);
    }
    hash_init(rules);
    }
