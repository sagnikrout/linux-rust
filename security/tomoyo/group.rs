//! Automatically rewritten from C to Rust
//! Source: security/tomoyo/group.c
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
// security/tomoyo/group.c
//
// Copyright (C) 2005-2011  NTT DATA CORPORATION
//

//
// tomoyo_same_path_group - Check for duplicated "struct tomoyo_path_group" entry.
//
// @a: Pointer to "struct tomoyo_acl_head".
// @b: Pointer to "struct tomoyo_acl_head".
//
// Returns true if @a == @b, false otherwise.
//
    static bool tomoyo_same_path_group(const struct tomoyo_acl_head *a,
    const struct tomoyo_acl_head *b)
    {
    return container_of(a, struct tomoyo_path_group, head).member_name ==
    container_of(b, struct tomoyo_path_group, head).member_name;
    }
//
// tomoyo_same_number_group - Check for duplicated "struct tomoyo_number_group" entry.
//
// @a: Pointer to "struct tomoyo_acl_head".
// @b: Pointer to "struct tomoyo_acl_head".
//
// Returns true if @a == @b, false otherwise.
//
    static bool tomoyo_same_number_group(const struct tomoyo_acl_head *a,
    const struct tomoyo_acl_head *b)
    {
    return !memcmp(&container_of(a, struct tomoyo_number_group, head)
    .number,
    &container_of(b, struct tomoyo_number_group, head)
    .number,
    sizeof(container_of(a, struct tomoyo_number_group, head)
    .number));
    }
//
// tomoyo_same_address_group - Check for duplicated "struct tomoyo_address_group" entry.
//
// @a: Pointer to "struct tomoyo_acl_head".
// @b: Pointer to "struct tomoyo_acl_head".
//
// Returns true if @a == @b, false otherwise.
//
    static bool tomoyo_same_address_group(const struct tomoyo_acl_head *a,
    const struct tomoyo_acl_head *b)
    {
    const struct tomoyo_address_group *p1 = container_of(a, typeof(*p1),
    head);
    const struct tomoyo_address_group *p2 = container_of(b, typeof(*p2),
    head);
    return tomoyo_same_ipaddr_union(&p1.address, &p2.address);
    }
//
// tomoyo_write_group - Write "struct tomoyo_path_group"/"struct tomoyo_number_group"/"struct tomoyo_address_group" list.
//
// @param: Pointer to "struct tomoyo_acl_param".
// @type:  Type of this group.
//
// Returns 0 on success, negative value otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn tomoyo_write_group(param: *mut tomoyo_acl_param, type: u8) -> c_int {
    int tomoyo_write_group(struct tomoyo_acl_param *param, const u8 type)
    {
    struct tomoyo_group *group = tomoyo_get_group(param, type);
    let mut error: c_int = -EINVAL;
    if (!group)
    return -ENOMEM;
    param.list = &group.member_list;
    if (type == TOMOYO_PATH_GROUP) {
    let mut e: tomoyo_path_group = { };
    e.member_name = tomoyo_get_name(tomoyo_read_token(param));
    if (!e.member_name) {
    error = -ENOMEM;
    goto out;
    }
    error = tomoyo_update_policy(&e.head, sizeof(e), param,
    tomoyo_same_path_group);
    tomoyo_put_name(e.member_name);
    } else if (type == TOMOYO_NUMBER_GROUP) {
    let mut e: tomoyo_number_group = { };
    if (param.data[0] == '@' ||
    !tomoyo_parse_number_union(param, &e.number))
    goto out;
    error = tomoyo_update_policy(&e.head, sizeof(e), param,
    tomoyo_same_number_group);
//
// tomoyo_put_number_union() is not needed because
// param->data[0] != '@'.
//
    } else {
    let mut e: tomoyo_address_group = { };
    if (param.data[0] == '@' ||
    !tomoyo_parse_ipaddr_union(param, &e.address))
    goto out;
    error = tomoyo_update_policy(&e.head, sizeof(e), param,
    tomoyo_same_address_group);
    }
    out:
    tomoyo_put_group(group);
    return error;
    }
//
// tomoyo_path_matches_group - Check whether the given pathname matches members of the given pathname group.
//
// @pathname: The name of pathname.
// @group:    Pointer to "struct tomoyo_path_group".
//
// Returns matched member's pathname if @pathname matches pathnames in @group,
// NULL otherwise.
//
// Caller holds tomoyo_read_lock().
//
    const struct tomoyo_path_info *
    tomoyo_path_matches_group(const struct tomoyo_path_info *pathname,
    const struct tomoyo_group *group)
    {
    struct tomoyo_path_group *member;
    list_for_each_entry_rcu(member, &group.member_list, head.list,
    srcu_read_lock_held(&tomoyo_ss)) {
    if (member.head.is_deleted)
    continue;
    if (!tomoyo_path_matches_pattern(pathname, member.member_name))
    continue;
    return member.member_name;
    }
    return core::ptr::null_mut();
    }
//
// tomoyo_number_matches_group - Check whether the given number matches members of the given number group.
//
// @min:   Min number.
// @max:   Max number.
// @group: Pointer to "struct tomoyo_number_group".
//
// Returns true if @min and @max partially overlaps @group, false otherwise.
//
// Caller holds tomoyo_read_lock().
//
    bool tomoyo_number_matches_group(const unsigned long min,
    const unsigned long max,
    const struct tomoyo_group *group)
    {
    struct tomoyo_number_group *member;
    let mut matched: bool = false;
    list_for_each_entry_rcu(member, &group.member_list, head.list,
    srcu_read_lock_held(&tomoyo_ss)) {
    if (member.head.is_deleted)
    continue;
    if (min > member.number.values[1] ||
    max < member.number.values[0])
    continue;
    matched = true;
    break;
    }
    return matched;
    }
//
// tomoyo_address_matches_group - Check whether the given address matches members of the given address group.
//
// @is_ipv6: True if @address is an IPv6 address.
// @address: An IPv4 or IPv6 address.
// @group:   Pointer to "struct tomoyo_address_group".
//
// Returns true if @address matches addresses in @group group, false otherwise.
//
// Caller holds tomoyo_read_lock().
//
    bool tomoyo_address_matches_group(const bool is_ipv6, const __be32 *address,
    const struct tomoyo_group *group)
    {
    struct tomoyo_address_group *member;
    let mut matched: bool = false;
    let mut size: u8 = is_ipv6 ? 16 : 4;
    list_for_each_entry_rcu(member, &group.member_list, head.list,
    srcu_read_lock_held(&tomoyo_ss)) {
    if (member.head.is_deleted)
    continue;
    if (member.address.is_ipv6 != is_ipv6)
    continue;
    if (memcmp(&member.address.ip[0], address, size) > 0 ||
    memcmp(address, &member.address.ip[1], size) > 0)
    continue;
    matched = true;
    break;
    }
    return matched;
    }
