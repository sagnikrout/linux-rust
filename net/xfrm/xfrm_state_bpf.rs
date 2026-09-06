//! Automatically rewritten from C to Rust
//! Source: net/xfrm/xfrm_state_bpf.c
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
// Unstable XFRM state BPF helpers.
//
// Note that it is allowed to break compatibility for these functions since the
// interface they are exposed through to BPF programs is explicitly unstable.
//

// bpf_xfrm_state_opts - Options for XFRM state lookup helpers
//
// Members:
// @error      - Out parameter, set for any errors encountered
// Values:
// -EINVAL - netns_id is less than -1
// -EINVAL - opts__sz isn't BPF_XFRM_STATE_OPTS_SZ
// -ENONET - No network namespace found for netns_id
// -ENOENT - No xfrm_state found
// @netns_id	- Specify the network namespace for lookup
// Values:
// BPF_F_CURRENT_NETNS (-1)
// Use namespace associated with ctx
// [0, S32_MAX]
// Network Namespace ID
// @mark	- XFRM mark to match on
// @daddr	- Destination address to match on
// @spi		- Security parameter index to match on
// @proto	- IP protocol to match on (eg. IPPROTO_ESP)
// @family	- Protocol family to match on (AF_INET/AF_INET6)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_xfrm_state_opts {
    pub error: i32,
    pub netns_id: i32,
    pub mark: u32,
    pub daddr: xfrm_address_t,
    pub spi: __be32,
    pub proto: u8,
    pub family: u16,
}

    enum {
    BPF_XFRM_STATE_OPTS_SZ = sizeof(struct bpf_xfrm_state_opts),
    };
    __bpf_kfunc_start_defs();
// bpf_xdp_get_xfrm_state - Get XFRM state
//
// A `struct xfrm_state *`, if found, must be released with a corresponding
// bpf_xdp_xfrm_state_release.
//
// Parameters:
// @ctx	- Pointer to ctx (xdp_md) in XDP program
// Cannot be NULL
// @opts	- Options for lookup (documented above)
// Cannot be NULL
// @opts__sz	- Length of the bpf_xfrm_state_opts structure
// Must be BPF_XFRM_STATE_OPTS_SZ
//
    __bpf_kfunc struct xfrm_state *
    bpf_xdp_get_xfrm_state(struct xdp_md *ctx, struct bpf_xfrm_state_opts *opts, u32 opts__sz)
    {
    struct xdp_buff *xdp = (struct xdp_buff *)ctx;
    struct net *net = dev_net(xdp.rxq.dev);
    struct xfrm_state *x;
    if (opts__sz < sizeof(opts.error))
    return core::ptr::null_mut();
    if (opts__sz != BPF_XFRM_STATE_OPTS_SZ) {
    opts.error = -EINVAL;
    return core::ptr::null_mut();
    }
    if (unlikely(opts.netns_id < BPF_F_CURRENT_NETNS)) {
    opts.error = -EINVAL;
    return core::ptr::null_mut();
    }
    if (opts.netns_id >= 0) {
    net = get_net_ns_by_id(net, opts.netns_id);
    if (unlikely(!net)) {
    opts.error = -ENONET;
    return core::ptr::null_mut();
    }
    }
    x = xfrm_state_lookup(net, opts.mark, &opts.daddr, opts.spi,
    opts.proto, opts.family);
    if (opts.netns_id >= 0)
    put_net(net);
    if (!x)
    opts.error = -ENOENT;
    return x;
    }
// bpf_xdp_xfrm_state_release - Release acquired xfrm_state object
//
// This must be invoked for referenced PTR_TO_BTF_ID, and the verifier rejects
// the program if any references remain in the program in all of the explored
// states.
//
// Parameters:
// @x		- Pointer to referenced xfrm_state object, obtained using
// bpf_xdp_get_xfrm_state.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_xdp_xfrm_state_release(x: *mut xfrm_state) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_xdp_xfrm_state_release(struct xfrm_state *x)
    {
    xfrm_state_put(x);
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(xfrm_state_kfunc_set)
    BTF_ID_FLAGS(func, bpf_xdp_get_xfrm_state, KF_RET_NULL | KF_ACQUIRE)
    BTF_ID_FLAGS(func, bpf_xdp_xfrm_state_release, KF_RELEASE)
    BTF_KFUNCS_END(xfrm_state_kfunc_set)
    static const struct btf_kfunc_id_set xfrm_state_xdp_kfunc_set = {
    .owner = THIS_MODULE,
    .set   = &xfrm_state_kfunc_set,
    };
#[no_mangle]
pub unsafe extern "C" fn register_xfrm_state_bpf() -> int __init {
    int __init register_xfrm_state_bpf(void)
    {
    return register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP,
    &xfrm_state_xdp_kfunc_set);
    }
