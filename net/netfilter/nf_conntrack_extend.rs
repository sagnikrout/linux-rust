//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_extend.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Structure dynamic extension infrastructure
// Copyright (C) 2004 Rusty Russell IBM Corporation
// Copyright (C) 2007 Netfilter Core Team <coreteam@netfilter.org>
// Copyright (C) 2007 USAGI/WIDE Project <http://www.linux-ipv6.org>
//

    static const u8 nf_ct_ext_type_len[NF_CT_EXT_NUM] = {
    [NF_CT_EXT_HELPER] = sizeof(struct nf_conn_help),

    [NF_CT_EXT_NAT] = sizeof(struct nf_conn_nat),

    [NF_CT_EXT_SEQADJ] = sizeof(struct nf_conn_seqadj),
    [NF_CT_EXT_ACCT] = sizeof(struct nf_conn_acct),

    [NF_CT_EXT_ECACHE] = sizeof(struct nf_conntrack_ecache),

    [NF_CT_EXT_TSTAMP] = sizeof(struct nf_conn_tstamp),

    [NF_CT_EXT_TIMEOUT] = sizeof(struct nf_conn_timeout),

    [NF_CT_EXT_LABELS] = sizeof(struct nf_conn_labels),

    [NF_CT_EXT_SYNPROXY] = sizeof(struct nf_conn_synproxy),

    [NF_CT_EXT_ACT_CT] = sizeof(struct nf_conn_act_ct_ext),

    };
#[no_mangle]
unsafe extern "C" fn total_extension_size() -> __always_inline unsigned int {
    static __always_inline unsigned int total_extension_size(void)
    {
// remember to add new extensions below
    BUILD_BUG_ON(NF_CT_EXT_NUM > 10);
    return sizeof(struct nf_ct_ext) +
    sizeof(struct nf_conn_help)

    + sizeof(struct nf_conn_nat)

    + sizeof(struct nf_conn_seqadj)
    + sizeof(struct nf_conn_acct)

    + sizeof(struct nf_conntrack_ecache)

    + sizeof(struct nf_conn_tstamp)

    + sizeof(struct nf_conn_timeout)

    + sizeof(struct nf_conn_labels)

    + sizeof(struct nf_conn_synproxy)

    + sizeof(struct nf_conn_act_ct_ext)

    ;
    }
    void *nf_ct_ext_add(struct nf_conn *ct, enum nf_ct_ext_id id, gfp_t gfp)
    {
    unsigned int newlen, newoff, oldlen, alloc;
    struct nf_ct_ext *new;
// Conntrack must not be confirmed to avoid races on reallocation.
    WARN_ON(nf_ct_is_confirmed(ct));
// struct nf_ct_ext uses u8 to store offsets/size
    BUILD_BUG_ON(total_extension_size() > 255u);
    if (ct.ext) {
    const struct nf_ct_ext *old = ct.ext;
    if (__nf_ct_ext_exist(old, id))
    return core::ptr::null_mut();
    oldlen = old.len;
    } else {
    oldlen = sizeof(*new);
    }
    newoff = ALIGN(oldlen, __alignof__(struct nf_ct_ext));
    newlen = newoff + nf_ct_ext_type_len[id];
    alloc = max(newlen, NF_CT_EXT_PREALLOC);
    new = krealloc(ct.ext, alloc, gfp);
    if (!new)
    return core::ptr::null_mut();
    if (!ct.ext)
    memset(new.offset, 0, sizeof(new.offset));
    new.offset[id] = newoff;
    new.len = newlen;
    memset((void *)new + newoff, 0, newlen - newoff);
    ct.ext = new;
    return (void *)new + newoff;
    }
    EXPORT_SYMBOL(nf_ct_ext_add);
