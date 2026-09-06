//! Automatically rewritten from C to Rust
//! Source: net/ipv4/tcp_scalable.c
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
// Tom Kelly's Scalable TCP
//
// See http://www.deneholme.net/tom/scalable
//
// John Heffner <jheffner@sc.edu>
//

// These factors derived from the recommended values in the aer:
// .01 and 7/8.
//

pub const TCP_SCALABLE_MD_SCALE: c_int = 3;
#[no_mangle]
unsafe extern "C" fn tcp_scalable_cong_avoid(sk: *mut sock, ack: u32, acked: u32) {
    static void tcp_scalable_cong_avoid(struct sock *sk, u32 ack, u32 acked)
    {
    struct tcp_sock *tp = tcp_sk(sk);
    if (!tcp_is_cwnd_limited(sk))
    return;
    if (tcp_in_slow_start(tp)) {
    acked = tcp_slow_start(tp, acked);
    if (!acked)
    return;
    }
    tcp_cong_avoid_ai(tp, min(tcp_snd_cwnd(tp), TCP_SCALABLE_AI_CNT),
    acked);
    }
#[no_mangle]
unsafe extern "C" fn tcp_scalable_ssthresh(sk: *mut sock) -> u32 {
    static u32 tcp_scalable_ssthresh(struct sock *sk)
    {
    const struct tcp_sock *tp = tcp_sk(sk);
    return max(tcp_snd_cwnd(tp) - (tcp_snd_cwnd(tp)>>TCP_SCALABLE_MD_SCALE), 2U);
    }
    static struct tcp_congestion_ops tcp_scalable __read_mostly = {
    .ssthresh	= tcp_scalable_ssthresh,
    .undo_cwnd	= tcp_reno_undo_cwnd,
    .cong_avoid	= tcp_scalable_cong_avoid,
    .owner		= THIS_MODULE,
    .name		= "scalable",
    };
#[no_mangle]
unsafe extern "C" fn tcp_scalable_register() -> int __init {
    static int __init tcp_scalable_register(void)
    {
    return tcp_register_congestion_control(&tcp_scalable);
    }
#[no_mangle]
unsafe extern "C" fn tcp_scalable_unregister() -> void __exit {
    static void __exit tcp_scalable_unregister(void)
    {
    tcp_unregister_congestion_control(&tcp_scalable);
    }
    module_init(tcp_scalable_register);
    module_exit(tcp_scalable_unregister);
    MODULE_AUTHOR("John Heffner");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Scalable TCP");
