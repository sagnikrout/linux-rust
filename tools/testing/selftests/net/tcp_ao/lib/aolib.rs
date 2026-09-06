//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/net/tcp_ao/lib/aolib.h
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
// TCP-AO selftest library. Provides helpers to unshare network
// namespaces, create veth, assign ip addresses, set routes,
// manipulate socket options, read network counter and etc.
// Author: Dmitry Safonov <dima@arista.com>
//

// can't include <netinet/tcp.h> as including <linux/tcp.h>

// Working around ksft, see the comment in lib/setup.c
extern "C" {
    pub fn __test_msg(buf: *const c_char);
}
extern "C" {
    pub fn __test_ok(buf: *const c_char);
}
extern "C" {
    pub fn __test_fail(buf: *const c_char);
}
extern "C" {
    pub fn __test_xfail(buf: *const c_char);
}
extern "C" {
    pub fn __test_error(buf: *const c_char);
}
extern "C" {
    pub fn __test_skip(buf: *const c_char);
}

pub const KSFT_FAIL: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_fault {
    FAULT_TIMEOUT = 1,
    FAULT_KEYREJECT,
    FAULT_PREINSTALL_AO,
    FAULT_PREINSTALL_MD5,
    FAULT_POSTINSTALL,
    FAULT_BUSY,
    FAULT_CURRNEXT,
    FAULT_FIXME,
}

pub type fault_t = test_fault;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_needs_kconfig {
    KCONFIG_NET_NS = 0,		/* required */
    KCONFIG_VETH,			/* required */
    KCONFIG_TCP_AO,			/* required */
    KCONFIG_TCP_MD5,		/* optional, for TCP-MD5 features */
    KCONFIG_NET_VRF,		/* optional, for L3/VRF testing */
    KCONFIG_FTRACE,			/* optional, for tracepoints checks */
    __KCONFIG_LAST__
}

extern "C" {
    pub fn kernel_config_has(k: test_needs_kconfig) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_addr {
    pub a4: in_addr,
    pub a6: in6_addr,
}

extern "C" {
    pub fn test_failed();
}
extern "C" {
    pub fn test_add_destructor((*d)(void): *mut c_void);
}
extern "C" {
    pub fn test_init_ftrace(nsfd1: c_int, nsfd2: c_int);
}
extern "C" {
    pub fn test_setup_tracing() -> c_int;
}
// To adjust optmem socket limit, approximately estimate a number,
// that is bigger than sizeof(struct tcp_ao_key).
//
pub const KERNEL_TCP_AO_KEY_SZ_ROUND_UP: c_int = 300;
extern "C" {
    pub fn test_set_optmem(value: usize);
}
extern "C" {
    pub fn test_get_optmem() -> usize;
}

extern "C" {
    pub fn synchronize_threads();
}
extern "C" {
    pub fn switch_ns(fd: c_int);
}
extern "C" {
    pub fn switch_save_ns(fd: c_int) -> c_int;
}
extern "C" {
    pub fn switch_close_ns(fd: c_int);
}
extern "C" {
    pub fn randomize_buffer(buf: *mut c_void, buflen: usize);
}
extern "C" {
    pub fn open_netns() -> c_int;
}
extern "C" {
    pub fn unshare_open_netns() -> c_int;
}
extern "C" {
    pub fn add_veth(name: *const c_char, nsfda: c_int, nsfdb: c_int) -> c_int;
}
extern "C" {
    pub fn add_vrf(name: *const c_char, tabid: u32, ifindex: c_int, nsfd: c_int) -> c_int;
}
extern "C" {
    pub fn link_set_up(intf: *const c_char) -> c_int;
}
extern "C" {
    pub fn test_wait_fd(sk: c_int, sec: time_t, write: bool) -> c_int;
}
extern "C" {
    pub fn __test_listen_socket(backlog: c_int, addr: *mut c_void, addr_sz: usize) -> c_int;
}
extern "C" {
    pub fn __test_listen_socket(_arg: backlog, )&addr: *mut (void, _arg: sizeof(addr)) -> return;
}
//
// In order for selftests to work under CONFIG_CRYPTO_FIPS=y,
// the password should be loger than 14 bytes, see hmac_setkey()
//
pub const TEST_TCP_AO_MINKEYLEN: c_int = 14;

pub const DEFAULT_TEST_PREFIX: c_int = 128;

pub const DEFAULT_TEST_PREFIX: c_int = 32;

//
// Timeout on syscalls where failure is not expected.
// You may want to rise it if the test machine is very busy.
//

pub const TEST_TIMEOUT_SEC: c_int = 5;

//
// Timeout on connect() where a failure is expected.
// If set to 0 - kernel will try to retransmit SYN number of times, set in
// /proc/sys/net/ipv4/tcp_syn_retries
// By default set to 1 to make tests pass faster on non-busy machine.
// [in process of removal, don't use in new tests]
//

pub const TEST_RETRANSMIT_SEC: c_int = 1;

extern "C" {
    pub fn _test_connect_socket(_arg: sk, _arg: taddr, _arg: port, _arg: false) -> return;
}
extern "C" {
    pub fn test_get_ao_info(sk: c_int, out: *mut tcp_ao_info_opt) -> c_int;
}
extern "C" {
    pub fn test_set_ao_info(sk: c_int, in: *mut tcp_ao_info_opt) -> c_int;
}
extern "C" {
    pub fn test_cmp_getsockopt_setsockopt(_arg: key, _arg: &key2) -> return;
}
extern "C" {
    pub fn test_verify_socket_key(_arg: sk, _arg: &tmp) -> return;
}
extern "C" {
    pub fn test_add_key_vrf(_arg: sk, _arg: key, _arg: 0, _arg: in_addr, _arg: prefix, _arg: 0, _arg: sndid, _arg: rcvid) -> return;
}
extern "C" {
    pub fn test_cmp_getsockopt_setsockopt_ao(_arg: ao, _arg: &ao2) -> return;
}
// Maybe ao_info wasn't allocated yet
extern "C" {
    pub fn test_verify_socket_ao(_arg: sk, _arg: &ao) -> return;
}
extern "C" {
    pub fn test_server_run(sk: c_int, quota: isize, timeout_sec: time_t) -> isize;
}
extern "C" {
    pub fn test_client_verify(sk: c_int, msg_len: usize, nr: usize) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_key_counters {
    pub sndid: u8,
    pub rcvid: u8,
    pub pkt_good: u64,
    pub pkt_bad: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_counters {
// per-netns
    pub netns_ao_good: u64,
    pub netns_ao_bad: u64,
    pub netns_ao_key_not_found: u64,
    pub netns_ao_required: u64,
    pub netns_ao_dropped_icmp: u64,
// per-socket
    pub ao_info_pkt_good: u64,
    pub ao_info_pkt_bad: u64,
    pub ao_info_pkt_key_not_found: u64,
    pub ao_info_pkt_ao_required: u64,
    pub ao_info_pkt_dropped_icmp: u64,
// per-key
    pub nr_keys: usize,
    pub key_cnts: *mut tcp_ao_key_counters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_counters {
    pub ao: tcp_ao_counters,
    pub netns_md5_notfound: u64,
    pub netns_md5_unexpected: u64,
    pub netns_md5_failure: u64,
}

extern "C" {
    pub fn test_get_tcp_counters(sk: c_int, out: *mut tcp_counters) -> c_int;
}

pub type test_cnt = u16;

// per-netns */							\
// per-socket */						\
// non-AO */							\

extern "C" {
    pub fn test_tcp_counters_free(cnts: *mut tcp_counters);
}
//
// Polling for netns and socket counters during select()/connect() and also
// client/server messaging. Instead of constant timeout on underlying select(),
// check the counters and return early. This allows to pass the tests where
// timeout is expected without waiting for that fixing timeout (tests speed-up).
// Previously shorter timeouts were used for tests expecting to time out,
// but that leaded to sporadic false positives on counter checks failures,
// as one second timeouts aren't enough for TCP retransmit.
//
// Two sides of the socketpair (client/server) should synchronize failures
// using a shared variable *err, so that they can detect the other side's
// failure.
//
// Frees buffers allocated in test_get_tcp_counters().
// The function doesn't expect new keys or keys removed between calls
// to test_get_tcp_counters(). Check key counters manually if they
// may change.
//
extern "C" {
    pub fn netstat_free(ns: *mut netstat);
}
extern "C" {
    pub fn netstat_print_diff(nsa: *mut netstat, nsb: *mut netstat);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sock_queue {
    pub seq: u32,
    pub buf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sock_state {
    pub info: tcp_info,
    pub trw: tcp_repair_window,
    pub out: tcp_sock_queue,
    pub /: *mut *mut int outq_len; / output queue size (not sent + not acked),
    pub /: *mut *mut int outq_nsd_len; / output queue size (not sent only),
    pub in: tcp_sock_queue,
    pub inq_len: c_int,
    pub mss: c_int,
    pub timestamp: c_int,
}

extern "C" {
    pub fn test_ao_checkpoint(sk: c_int, state: *mut tcp_ao_repair);
}
extern "C" {
    pub fn test_ao_restore(sk: c_int, state: *mut tcp_ao_repair);
}
extern "C" {
    pub fn test_sock_state_free(state: *mut tcp_sock_state);
}
extern "C" {
    pub fn test_enable_repair(sk: c_int);
}
extern "C" {
    pub fn test_disable_repair(sk: c_int);
}
extern "C" {
    pub fn test_kill_sk(sk: c_int);
}
extern "C" {
    pub fn test_verify_socket_key(_arg: sk, _arg: &tmp) -> return;
}
pub const DEFAULT_FTRACE_BUFFER_KB: c_int = 10000;
pub const DEFAULT_TRACER_LINES_ARR: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ftracer_op {
    FTRACER_LINE_DISCARD = 0,
    FTRACER_LINE_PRESERVE,
    FTRACER_EXIT,
}

extern "C" {
    pub fn destroy_ftracer(tracer: *mut test_ftracer);
}
extern "C" {
    pub fn tracer_get_savedlines_nr(tracer: *mut test_ftracer) -> usize;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum trace_events {
// TCP_HASH_EVENT
    TCP_HASH_BAD_HEADER = 0,
    TCP_HASH_MD5_REQUIRED,
    TCP_HASH_MD5_UNEXPECTED,
    TCP_HASH_MD5_MISMATCH,
    TCP_HASH_AO_REQUIRED,
// TCP_AO_EVENT
    TCP_AO_HANDSHAKE_FAILURE,
    TCP_AO_WRONG_MACLEN,
    TCP_AO_MISMATCH,
    TCP_AO_KEY_NOT_FOUND,
    TCP_AO_RNEXT_REQUEST,
// TCP_AO_EVENT_SK
    TCP_AO_SYNACK_NO_KEY,
// TCP_AO_EVENT_SNE
    TCP_AO_SND_SNE_UPDATE,
    TCP_AO_RCV_SNE_UPDATE,
    __MAX_TRACE_EVENTS
}

extern "C" {
    pub fn setup_aolib_ftracer() -> c_int;
}
