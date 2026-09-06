//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_smc.c
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

    char _license[] SEC("license") = "GPL";

pub const SMC_HS_CTRL_NAME_MAX: c_int = 16;

    enum {
    BPF_SMC_LISTEN	= 10,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_sock___local {
    pub sk: sock,
    pub listen_smc: *mut smc_sock,
    pub use_fallback: bool,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_hs_ctrl___local {
    pub name: [c_char; SMC_HS_CTRL_NAME_MAX],
    pub ): *mut *mut int (syn_option)(struct tcp_sock,
    pub ): *const *const *const int (synack_option)(struct tcp_sock , struct inet_request_sock,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_smc___local {
    pub hs_ctrl: *mut smc_hs_ctrl___local,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net___local {
    pub smc: netns_smc___local,
    pub __attribute__((preserve_access_index)): },
    pub 0: int smc_cnt =,
    pub 0: int fallback_cnt =,
    SEC("fentry/smc_release")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_smc_release, sock: *mut socket) -> c_int {
    int BPF_PROG(bpf_smc_release, struct socket *sock)
    {
// only count from one side (client)
    if (sock.sk.__sk_common.skc_state == BPF_SMC_LISTEN)
    pub 0: return,
    pub 0: return,
    }
    SEC("fentry/smc_switch_to_fallback")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_smc_switch_to_fallback, smc: *mut smc_sock___local) -> c_int {
    int BPF_PROG(bpf_smc_switch_to_fallback, struct smc_sock___local *smc)
    {
// only count from one side (client)
    if (smc && !smc.listen_smc)
    pub 0: return,
    }
// go with default value if no strat was found
    pub true: bool default_ip_strat_value =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_policy_ip_key {
    pub sip: __u32,
    pub dip: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_policy_ip_value {
    pub mode: __u8,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(struct smc_policy_ip_key));
    __uint(value_size, sizeof(struct smc_policy_ip_value));
    __uint(max_entries, 128);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    } smc_policy_ip SEC(".maps");
#[no_mangle]
unsafe extern "C" fn smc_check(src: __u32, dst: __u32) -> bool {
    static bool smc_check(__u32 src, __u32 dst)
    {
    struct smc_policy_ip_value *value;
    struct smc_policy_ip_key key = {
    .sip = src,
    .dip = dst,
    };
    value = bpf_map_lookup_elem(&smc_policy_ip, &key);
    return value ? value.mode : default_ip_strat_value;
    }
    SEC("fmod_ret/update_socket_protocol")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: smc_run, family: c_int, type: c_int, protocol: c_int) -> c_int {
    int BPF_PROG(smc_run, int family, int type, int protocol)
    {
    struct task_struct *task;
    if (family != AF_INET && family != AF_INET6)
    return protocol;
    if ((type & 0xf) != SOCK_STREAM)
    return protocol;
    if (protocol != 0 && protocol != IPPROTO_TCP)
    return protocol;
    task = bpf_get_current_task_btf();
// Prevent from affecting other tests
    if (!task) {
    return protocol;
    } else {
    struct net___local *net = (struct net___local *)task.nsproxy.net_ns;
    if (!bpf_core_field_exists(struct net___local, smc) || !net.smc.hs_ctrl)
    return protocol;
    }
    return IPPROTO_SMC;
    }
    SEC("struct_ops")
    int BPF_PROG(bpf_smc_set_tcp_option_cond, const struct tcp_sock *tp,
    struct inet_request_sock *ireq)
    {
    return smc_check(ireq.req.__req_common.skc_daddr,
    ireq.req.__req_common.skc_rcv_saddr);
    }
    SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_smc_set_tcp_option, tp: *mut tcp_sock) -> c_int {
    int BPF_PROG(bpf_smc_set_tcp_option, struct tcp_sock *tp)
    {
    return smc_check(tp.inet_conn.icsk_inet.sk.__sk_common.skc_rcv_saddr,
    tp.inet_conn.icsk_inet.sk.__sk_common.skc_daddr);
    }
    SEC(".struct_ops")
    struct smc_hs_ctrl___local  linkcheck = {
    .name		= "linkcheck",
    .syn_option	= (void *)bpf_smc_set_tcp_option,
    .synack_option	= (void *)bpf_smc_set_tcp_option_cond,
    };
