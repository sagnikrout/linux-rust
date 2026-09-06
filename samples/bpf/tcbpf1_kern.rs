//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tcbpf1_kern.c
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


// compiler workaround

#[no_mangle]
pub unsafe extern "C" fn set_dst_mac(skb: *mut __sk_buff, mac: *mut c_char) {
    static inline void set_dst_mac(struct __sk_buff *skb, char *mac)
    {
    bpf_skb_store_bytes(skb, 0, mac, ETH_ALEN, 1);
    }

#[no_mangle]
pub unsafe extern "C" fn set_ip_tos(skb: *mut __sk_buff, new_tos: __u8) {
    static inline void set_ip_tos(struct __sk_buff *skb, __u8 new_tos)
    {
    let mut old_tos: __u8 = load_byte(skb, TOS_OFF);
    bpf_l3_csum_replace(skb, IP_CSUM_OFF, htons(old_tos), htons(new_tos), 2);
    bpf_skb_store_bytes(skb, TOS_OFF, &new_tos, sizeof(new_tos), 0);
    }

pub const IS_PSEUDO: c_uint = 0x10;
#[no_mangle]
pub unsafe extern "C" fn set_tcp_ip_src(skb: *mut __sk_buff, new_ip: __u32) {
    static inline void set_tcp_ip_src(struct __sk_buff *skb, __u32 new_ip)
    {
    let mut old_ip: __u32 = _htonl(load_word(skb, IP_SRC_OFF));
    bpf_l4_csum_replace(skb, TCP_CSUM_OFF, old_ip, new_ip, IS_PSEUDO | sizeof(new_ip));
    bpf_l3_csum_replace(skb, IP_CSUM_OFF, old_ip, new_ip, sizeof(new_ip));
    bpf_skb_store_bytes(skb, IP_SRC_OFF, &new_ip, sizeof(new_ip), 0);
    }

#[no_mangle]
pub unsafe extern "C" fn set_tcp_dest_port(skb: *mut __sk_buff, new_port: __u16) {
    static inline void set_tcp_dest_port(struct __sk_buff *skb, __u16 new_port)
    {
    let mut old_port: __u16 = htons(load_half(skb, TCP_DPORT_OFF));
    bpf_l4_csum_replace(skb, TCP_CSUM_OFF, old_port, new_port, sizeof(new_port));
    bpf_skb_store_bytes(skb, TCP_DPORT_OFF, &new_port, sizeof(new_port), 0);
    }
    SEC("classifier")
#[no_mangle]
pub unsafe extern "C" fn bpf_prog1(skb: *mut __sk_buff) -> c_int {
    int bpf_prog1(struct __sk_buff *skb)
    {
    let mut proto: __u8 = load_byte(skb, ETH_HLEN + offsetof(struct iphdr, protocol));
    long *value;
    if (proto == IPPROTO_TCP) {
    set_ip_tos(skb, 8);
    set_tcp_ip_src(skb, 0xA010101);
    set_tcp_dest_port(skb, 5001);
    }
    return 0;
    }
    SEC("redirect_xmit")
#[no_mangle]
pub unsafe extern "C" fn _redirect_xmit(skb: *mut __sk_buff) -> c_int {
    int _redirect_xmit(struct __sk_buff *skb)
    {
    return bpf_redirect(skb.ifindex + 1, 0);
    }
    SEC("redirect_recv")
#[no_mangle]
pub unsafe extern "C" fn _redirect_recv(skb: *mut __sk_buff) -> c_int {
    int _redirect_recv(struct __sk_buff *skb)
    {
    return bpf_redirect(skb.ifindex + 1, 1);
    }
    SEC("clone_redirect_xmit")
#[no_mangle]
pub unsafe extern "C" fn _clone_redirect_xmit(skb: *mut __sk_buff) -> c_int {
    int _clone_redirect_xmit(struct __sk_buff *skb)
    {
    bpf_clone_redirect(skb, skb.ifindex + 1, 0);
    return TC_ACT_SHOT;
    }
    SEC("clone_redirect_recv")
#[no_mangle]
pub unsafe extern "C" fn _clone_redirect_recv(skb: *mut __sk_buff) -> c_int {
    int _clone_redirect_recv(struct __sk_buff *skb)
    {
    bpf_clone_redirect(skb, skb.ifindex + 1, 1);
    return TC_ACT_SHOT;
    }
    char _license[] SEC("license") = "GPL";
