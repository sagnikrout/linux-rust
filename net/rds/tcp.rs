//! Automatically rewritten from C Header to Rust Module
//! Source: net/rds/tcp.h
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
pub const RDS_TCP_PORT: c_int = 16385;
// per-network namespace private data for this module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_tcp_net {
// serialize "rds_tcp_accept_one" with "rds_tcp_accept_lock"
// to protect "rds_tcp_accepted_sock"
//
    pub rds_tcp_accept_lock: mutex,
    pub rds_tcp_listen_sock: *mut socket,
    pub rds_tcp_accepted_sock: *mut socket,
    pub rds_tcp_accept_w: work_struct,
    pub rds_tcp_sysctl: *mut ctl_table_header,
    pub ctl_table: *const ctl_table,
    pub sndbuf_size: c_int,
    pub rcvbuf_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_tcp_incoming {
    pub ti_inc: rds_incoming,
    pub ti_skb_list: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_tcp_connection {
    pub t_tcp_node: list_head,
    pub t_tcp_node_detached: bool,
    pub t_cpath: *mut rds_conn_path,
// t_conn_path_lock synchronizes the connection establishment between
// rds_tcp_accept_one and rds_tcp_conn_path_connect
//
    pub t_conn_path_lock: mutex,
    pub t_sock: *mut socket,
    pub t_client_port_group: u32,
    pub t_rtn: *mut rds_tcp_net,
    pub t_orig_write_space: *mut c_void,
    pub t_orig_data_ready: *mut c_void,
    pub t_orig_state_change: *mut c_void,
    pub t_tinc: *mut rds_tcp_incoming,
    pub t_tinc_hdr_rem: usize,
    pub t_tinc_data_rem: usize,
// XXX error report?
    pub t_conn_w: work_struct,
    pub t_send_w: work_struct,
    pub t_down_w: work_struct,
    pub t_recv_w: work_struct,
// for info exporting only
    pub t_list_item: list_head,
    pub t_last_sent_nxt: u32,
    pub t_last_expected_una: u32,
    pub t_last_seen_una: u32,
// for rds_tcp_conn_path_shutdown
    pub t_recv_done_waitq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_tcp_statistics {
    pub s_tcp_data_ready_calls: u64,
    pub s_tcp_write_space_calls: u64,
    pub s_tcp_sndbuf_full: u64,
    pub s_tcp_connect_raced: u64,
    pub s_tcp_listen_closed_stale: u64,
}

// tcp.c
extern "C" {
    pub fn rds_tcp_tune(sock: *mut socket) -> bool;
}
extern "C" {
    pub fn rds_tcp_set_callbacks(sock: *mut socket, cp: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_tcp_reset_callbacks(sock: *mut socket, cp: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_tcp_write_seq(tc: *mut rds_tcp_connection) -> u32;
}
extern "C" {
    pub fn rds_tcp_snd_una(tc: *mut rds_tcp_connection) -> u32;
}
extern "C" {
    pub fn rds_tcp_accept_work(rtn: *mut rds_tcp_net);
}
// tcp_connect.c
extern "C" {
    pub fn rds_tcp_conn_path_connect(cp: *mut rds_conn_path) -> c_int;
}
extern "C" {
    pub fn rds_tcp_conn_path_shutdown(conn: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_tcp_state_change(sk: *mut sock);
}
// tcp_listen.c
extern "C" {
    pub fn rds_tcp_listen_stop(sock: *mut socket, acceptor: *mut work_struct);
}
extern "C" {
    pub fn rds_tcp_listen_data_ready(sk: *mut sock);
}
extern "C" {
    pub fn rds_tcp_conn_slots_available(conn: *mut rds_connection, fan_out: bool);
}
extern "C" {
    pub fn rds_tcp_accept_one(rtn: *mut rds_tcp_net) -> c_int;
}
extern "C" {
    pub fn rds_tcp_keepalive(sock: *mut socket);
}
// tcp_recv.c
extern "C" {
    pub fn rds_tcp_recv_init() -> c_int;
}
extern "C" {
    pub fn rds_tcp_recv_exit();
}
extern "C" {
    pub fn rds_tcp_data_ready(sk: *mut sock);
}
extern "C" {
    pub fn rds_tcp_recv_path(cp: *mut rds_conn_path) -> c_int;
}
extern "C" {
    pub fn rds_tcp_inc_free(inc: *mut rds_incoming);
}
extern "C" {
    pub fn rds_tcp_inc_copy_to_user(inc: *mut rds_incoming, to: *mut iov_iter) -> c_int;
}
// tcp_send.c
extern "C" {
    pub fn rds_tcp_xmit_path_prepare(cp: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_tcp_xmit_path_complete(cp: *mut rds_conn_path);
}
extern "C" {
    pub fn rds_tcp_is_acked(rm: *mut rds_message, ack: u64) -> c_int;
}
extern "C" {
    pub fn rds_tcp_write_space(sk: *mut sock);
}
// tcp_stats.c

