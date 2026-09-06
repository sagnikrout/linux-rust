//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_nat_bpf.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Unstable NAT Helpers for XDP and TC-BPF hook
//
// These are called from the XDP and SCHED_CLS BPF programs. Note that it is
// allowed to break compatibility for these functions since the interface they
// are exposed through to BPF programs is explicitly unstable.
//

    __bpf_kfunc_start_defs();
// bpf_ct_set_nat_info - Set source or destination nat address
//
// Set source or destination nat address of the newly allocated
// nf_conn before insertion. This must be invoked for referenced
// PTR_TO_BTF_ID to nf_conn___init.
//
// Parameters:
// @nfct	- Pointer to referenced nf_conn object, obtained using
// bpf_xdp_ct_alloc or bpf_skb_ct_alloc.
// @addr	- Nat source/destination address
// @port	- Nat source/destination port. Non-positive values are
// interpreted as select a random port.
// @manip	- NF_NAT_MANIP_SRC or NF_NAT_MANIP_DST
//
    __bpf_kfunc int bpf_ct_set_nat_info(struct nf_conn___init *nfct,
    union nf_inet_addr *addr, int port,
    enum nf_nat_manip_type manip)
    {
    struct nf_conn *ct = (struct nf_conn *)nfct;
    let mut proto: u16 = nf_ct_l3num(ct);
    struct nf_nat_range2 range;
    if (proto != NFPROTO_IPV4 && proto != NFPROTO_IPV6)
    return -EINVAL;
    memset(&range, 0, sizeof(struct nf_nat_range2));
    range.flags = NF_NAT_RANGE_MAP_IPS;
    range.min_addr = *addr;
    range.max_addr = range.min_addr;
    if (port > 0) {
    range.flags |= NF_NAT_RANGE_PROTO_SPECIFIED;
    range.min_proto.all = cpu_to_be16(port);
    range.max_proto.all = range.min_proto.all;
    }
    return nf_nat_setup_info(ct, &range, manip) == NF_DROP ? -ENOMEM : 0;
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(nf_nat_kfunc_set)
    BTF_ID_FLAGS(func, bpf_ct_set_nat_info)
    BTF_KFUNCS_END(nf_nat_kfunc_set)
    static const struct btf_kfunc_id_set nf_bpf_nat_kfunc_set = {
    .owner = THIS_MODULE,
    .set   = &nf_nat_kfunc_set,
    };
#[no_mangle]
pub unsafe extern "C" fn register_nf_nat_bpf() -> c_int {
    int register_nf_nat_bpf(void)
    {
    int ret;
    ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP,
    &nf_bpf_nat_kfunc_set);
    if (ret)
    return ret;
    return register_btf_kfunc_id_set(BPF_PROG_TYPE_SCHED_CLS,
    &nf_bpf_nat_kfunc_set);
    }
