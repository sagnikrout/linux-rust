//! Automatically rewritten from C to Rust
//! Source: net/ipv4/metrics.c
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

    static int ip_metrics_convert(struct nlattr *fc_mx,
    int fc_mx_len, u32 *metrics,
    struct netlink_ext_ack *extack)
    {
    let mut ecn_ca: bool = false;
    struct nlattr *nla;
    int remaining;
    nla_for_each_attr(nla, fc_mx, fc_mx_len, remaining) {
    let mut type: c_int = nla_type(nla);
    u32 val;
    if (!type)
    continue;
    if (type > RTAX_MAX) {
    NL_SET_ERR_MSG(extack, "Invalid metric type");
    return -EINVAL;
    }
    type = array_index_nospec(type, RTAX_MAX + 1);
    if (type == RTAX_CC_ALGO) {
    char tmp[TCP_CA_NAME_MAX];
    nla_strscpy(tmp, nla, sizeof(tmp));
    val = tcp_ca_get_key_by_name(tmp, &ecn_ca);
    if (val == TCP_CA_UNSPEC) {
    NL_SET_ERR_MSG(extack, "Unknown tcp congestion algorithm");
    return -EINVAL;
    }
    } else {
    if (nla_len(nla) != sizeof(u32)) {
    NL_SET_ERR_MSG_ATTR(extack, nla,
    "Invalid attribute in metrics");
    return -EINVAL;
    }
    val = nla_get_u32(nla);
    }
    if (type == RTAX_ADVMSS && val > 65535 - 40)
    val = 65535 - 40;
    if (type == RTAX_MTU && val > 65535 - 15)
    val = 65535 - 15;
    if (type == RTAX_HOPLIMIT && val > 255)
    val = 255;
    if (type == RTAX_FEATURES && (val & ~RTAX_FEATURE_MASK)) {
    NL_SET_ERR_MSG(extack, "Unknown flag set in feature mask in metrics attribute");
    return -EINVAL;
    }
    metrics[type - 1] = val;
    }
    if (ecn_ca)
    metrics[RTAX_FEATURES - 1] |= DST_FEATURE_ECN_CA;
    return 0;
    }
    struct dst_metrics *ip_fib_metrics_init(struct nlattr *fc_mx,
    int fc_mx_len,
    struct netlink_ext_ack *extack)
    {
    struct dst_metrics *fib_metrics;
    int err;
    if (!fc_mx)
    return (struct dst_metrics *)&dst_default_metrics;
    fib_metrics = kzalloc_obj(*fib_metrics);
    if (unlikely(!fib_metrics))
    return ERR_PTR(-ENOMEM);
    err = ip_metrics_convert(fc_mx, fc_mx_len, fib_metrics.metrics,
    extack);
    if (!err) {
    refcount_set(&fib_metrics.refcnt, 1);
    } else {
    kfree(fib_metrics);
    fib_metrics = ERR_PTR(err);
    }
    return fib_metrics;
    }
