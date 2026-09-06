//! Automatically rewritten from C to Rust
//! Source: tools/lib/bpf/btf_iter.c
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
// Copyright (c) 2021 Facebook
// Copyright (c) 2024, Oracle and/or its affiliates.

    int btf_field_iter_init(struct btf_field_iter *it, struct btf_type *t,
    enum btf_field_iter_kind iter_kind)
    {
    it.p = core::ptr::null_mut();
    it.m_idx = -1;
    it.off_idx = 0;
    it.vlen = 0;
    switch (iter_kind) {
    case BTF_FIELD_ITER_IDS:
    switch (btf_kind(t)) {
    case BTF_KIND_UNKN:
    case BTF_KIND_INT:
    case BTF_KIND_FLOAT:
    case BTF_KIND_ENUM:
    case BTF_KIND_ENUM64:
    it.desc = (struct btf_field_desc) {};
    break;
    case BTF_KIND_FWD:
    case BTF_KIND_CONST:
    case BTF_KIND_VOLATILE:
    case BTF_KIND_RESTRICT:
    case BTF_KIND_PTR:
    case BTF_KIND_TYPEDEF:
    case BTF_KIND_FUNC:
    case BTF_KIND_VAR:
    case BTF_KIND_DECL_TAG:
    case BTF_KIND_TYPE_TAG:
    it.desc = (struct btf_field_desc) { 1, {offsetof(struct btf_type, type)} };
    break;
    case BTF_KIND_ARRAY:
    it.desc = (struct btf_field_desc) {
    2, {sizeof(struct btf_type) + offsetof(struct btf_array, type),
    sizeof(struct btf_type) + offsetof(struct btf_array, index_type)}
    };
    break;
    case BTF_KIND_STRUCT:
    case BTF_KIND_UNION:
    it.desc = (struct btf_field_desc) {
    0, {},
    sizeof(struct btf_member),
    1, {offsetof(struct btf_member, type)}
    };
    break;
    case BTF_KIND_FUNC_PROTO:
    it.desc = (struct btf_field_desc) {
    1, {offsetof(struct btf_type, type)},
    sizeof(struct btf_param),
    1, {offsetof(struct btf_param, type)}
    };
    break;
    case BTF_KIND_DATASEC:
    it.desc = (struct btf_field_desc) {
    0, {},
    sizeof(struct btf_var_secinfo),
    1, {offsetof(struct btf_var_secinfo, type)}
    };
    break;
    default:
    return -EINVAL;
    }
    break;
    case BTF_FIELD_ITER_STRS:
    switch (btf_kind(t)) {
    case BTF_KIND_UNKN:
    it.desc = (struct btf_field_desc) {};
    break;
    case BTF_KIND_INT:
    case BTF_KIND_FLOAT:
    case BTF_KIND_FWD:
    case BTF_KIND_ARRAY:
    case BTF_KIND_CONST:
    case BTF_KIND_VOLATILE:
    case BTF_KIND_RESTRICT:
    case BTF_KIND_PTR:
    case BTF_KIND_TYPEDEF:
    case BTF_KIND_FUNC:
    case BTF_KIND_VAR:
    case BTF_KIND_DECL_TAG:
    case BTF_KIND_TYPE_TAG:
    case BTF_KIND_DATASEC:
    it.desc = (struct btf_field_desc) {
    1, {offsetof(struct btf_type, name_off)}
    };
    break;
    case BTF_KIND_ENUM:
    it.desc = (struct btf_field_desc) {
    1, {offsetof(struct btf_type, name_off)},
    sizeof(struct btf_enum),
    1, {offsetof(struct btf_enum, name_off)}
    };
    break;
    case BTF_KIND_ENUM64:
    it.desc = (struct btf_field_desc) {
    1, {offsetof(struct btf_type, name_off)},
    sizeof(struct btf_enum64),
    1, {offsetof(struct btf_enum64, name_off)}
    };
    break;
    case BTF_KIND_STRUCT:
    case BTF_KIND_UNION:
    it.desc = (struct btf_field_desc) {
    1, {offsetof(struct btf_type, name_off)},
    sizeof(struct btf_member),
    1, {offsetof(struct btf_member, name_off)}
    };
    break;
    case BTF_KIND_FUNC_PROTO:
    it.desc = (struct btf_field_desc) {
    1, {offsetof(struct btf_type, name_off)},
    sizeof(struct btf_param),
    1, {offsetof(struct btf_param, name_off)}
    };
    break;
    default:
    return -EINVAL;
    }
    break;
    default:
    return -EINVAL;
    }
    if (it.desc.m_sz)
    it.vlen = btf_vlen(t);
    it.p = t;
    return 0;
    }
    __u32 *btf_field_iter_next(struct btf_field_iter *it)
    {
    if (!it.p)
    return core::ptr::null_mut();
    if (it.m_idx < 0) {
    if (it.off_idx < it.desc.t_off_cnt)
    return it.p + it.desc.t_offs[it.off_idx++];
// move to per-member iteration
    it.m_idx = 0;
    it.p += sizeof(struct btf_type);
    it.off_idx = 0;
    }
// if type doesn't have members, stop
    if (it.desc.m_sz == 0) {
    it.p = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
    if (it.off_idx >= it.desc.m_off_cnt) {
// exhausted this member's fields, go to the next member
    it.m_idx++;
    it.p += it.desc.m_sz;
    it.off_idx = 0;
    }
    if (it.m_idx < it.vlen)
    return it.p + it.desc.m_offs[it.off_idx++];
    it.p = core::ptr::null_mut();
    return core::ptr::null_mut();
    }
