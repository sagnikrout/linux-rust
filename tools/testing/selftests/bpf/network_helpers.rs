//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/network_helpers.h
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

pub type __sum16 = __u16;

pub const MAGIC_VAL: c_uint = 0x1234;
pub const NUM_ITER: c_int = 100000;
pub const VIP_NUM: c_int = 5;
pub const MAGIC_BYTES: c_int = 123;
// include/linux/net.h

pub const SOCK_TYPE_MASK: c_uint = 0xf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct network_helper_opts {
    pub timeout_ms: c_int,
    pub proto: c_int,
// +ve: Passed to listen() as-is.
// 0: Default when the test does not set
// a particular value during the struct init.
// It is changed to 1 before passing to listen().
// Most tests only have one on-going connection.
// -ve: It is changed to 0 before passing to listen().
// It is useful to force syncookie without
// changing the "tcp_syncookies" sysctl from 1 to 2.
//
    pub backlog: c_int,
    pub opts): *mut *mut int (post_socket_cb)(int fd, void,
    pub cb_opts: *mut c_void,
}

// ipv4 test vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv4_packet {
    pub eth: ethhdr,
    pub iph: iphdr,
    pub tcp: tcphdr,
    pub __packed: },
    pub pkt_v4: extern struct ipv4_packet,
// ipv6 test vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_packet {
    pub eth: ethhdr,
    pub iph: ipv6hdr,
    pub tcp: tcphdr,
    pub __packed: },
    pub pkt_v6: extern struct ipv6_packet,
    pub timeout_ms): int settimeo(int fd, int,
    pub opts): *const network_helper_opts,
    pub timeout_ms): c_int,
    pub nr_listens): c_uint,
    pub opts): *const network_helper_opts,
    pub nr_close_fds): *mut *mut void free_fds(int fds, unsigned int,
    pub opts): *const network_helper_opts,
    pub opts): *const network_helper_opts,
    pub opts): *const network_helper_opts,
    pub timeout_ms): int connect_to_fd(int server_fd, int,
    pub opts): *const int connect_to_fd_opts(int server_fd, struct network_helper_opts,
    pub timeout_ms): int connect_fd_to_fd(int client_fd, int server_fd, int,
    pub timeout_ms): c_int,
    pub len): *mut *mut sockaddr_storage addr, socklen_t,
    pub family): *mut *mut char ping_command(int,
    pub sock_fd): int get_socket_local_port(int,
    pub ring_param): *mut *mut int get_hw_ring_size(char ifname, struct ethtool_ringparam,
    pub ring_param): *mut *mut int set_hw_ring_size(char ifname, struct ethtool_ringparam,
    pub need_mac): *const *const int open_tuntap(char dev_name, bool,
    pub nstoken: struct,
//
// open_netns() - Switch to specified network namespace by name.
//
// Returns token with which to restore the original namespace
// using close_netns().
//
    pub name): *const *const nstoken open_netns(char,
    pub token): *mut void close_netns(struct nstoken,
    pub total_bytes): int send_recv_data(int lfd, int fd, uint32_t,
    pub name): *const int make_netns(char,
    pub name): *const int remove_netns(char,
//
// append_tid() - Append thread ID to the given string.
//
// @str: string to extend
// @sz: string's size
//
// 8 characters are used to append the thread ID (7 digits + '\0')
//
// Returns -1 on errors, 0 otherwise
//
    pub sz): *mut *mut int append_tid(char str, size_t,
    pub 16): csum = (csum & 0xffff) + (csum >>,
    pub 16): csum = (csum & 0xffff) + (csum >>,
    pub (__u16)~csum: return,
    pub )buf: *mut *mut __u16 p = (__u16,
    pub 1: int num_u16 = len >>,
    pub i: c_int,
    pub i++): for (i = 0; i < num_u16;,
    pub p: [sum +=; i],
    pub sum: return,
    pub 0: __u32 sum =,
    pub p: *mut __u16,
    pub 0: iph->check =,
    pub )iph: *mut p = (void,
    pub 0): sum = csum_partial(p, iph->ihl << 2,,
    pub csum_fold(sum): return,
//
// csum_tcpudp_magic - compute IP pseudo-header checksum
//
// Compute the IPv4 pseudo header checksum. The helper can take a
// accumulated sum from the transport layer to accumulate it and directly
// return the transport layer
//
// @saddr: IP source address
// @daddr: IP dest address
// @len: IP data size
// @proto: transport layer protocol
// @csum: The accumulated partial sum to add to the computation
//
// Returns the folded sum
//
    pub csum: __u64 s =,
    pub (__u32)saddr: s +=,
    pub (__u32)daddr: s +=,
    pub len): s += htons(proto +,
    pub 32): s = (s & 0xffffffff) + (s >>,
    pub 32): s = (s & 0xffffffff) + (s >>,
    pub csum_fold((__u32)s): return,
//
// csum_ipv6_magic - compute IPv6 pseudo-header checksum
//
// Compute the ipv6 pseudo header checksum. The helper can take a
// accumulated sum from the transport layer to accumulate it and directly
// return the transport layer
//
// @saddr: IPv6 source address
// @daddr: IPv6 dest address
// @len: IPv6 data size
// @proto: transport layer protocol
// @csum: The accumulated partial sum to add to the computation
//
// Returns the folded sum
//
    pub csum: __u64 s =,
    pub i: c_int,
    pub i++): for (i = 0; i < 4;,
    pub (__u32)saddr->s6_addr32[i]: s +=,
    pub i++): for (i = 0; i < 4;,
    pub (__u32)daddr->s6_addr32[i]: s +=,
    pub len): s += htons(proto +,
    pub 32): s = (s & 0xffffffff) + (s >>,
    pub 32): s = (s & 0xffffffff) + (s >>,
    pub csum_fold((__u32)s): return,
//
// build_udp_v4_csum - compute UDP checksum for UDP over IPv4
//
// Compute the checksum to embed in UDP header, composed of the sum of IP
// pseudo-header checksum, UDP header checksum and UDP data checksum
// @iph IP header
// @udph UDP header, which must be immediately followed by UDP data
//
// Returns the total checksum
//
    pub sum: c_ulong,
    pub 0): sum = csum_partial(udph, ntohs(udph->len),,
    pub sum): IPPROTO_UDP,,
//
// build_udp_v6_csum - compute UDP checksum for UDP over IPv6
//
// Compute the checksum to embed in UDP header, composed of the sum of IPv6
// pseudo-header checksum, UDP header checksum and UDP data checksum
// @ip6h IPv6 header
// @udph UDP header, which must be immediately followed by UDP data
//
// Returns the total checksum
//
    pub sum: c_ulong,
    pub 0): sum = csum_partial(udph, ntohs(udph->len),,
    pub sum): IPPROTO_UDP,,
    pub tmonitor_ctx: struct,
    pub args): *const *const *const typedef int (tm_print_fn_t)(char format, va_list,
//
// tc_prog_attach - attach BPF program(s) to an interface
//
// Takes file descriptors pointing to at least one, at most two BPF
// programs, and attach those programs to an interface ingress, egress or
// both.
//
// @dev: string containing the interface name
// @ingress_fd: file descriptor of the program to attach to interface ingress
// @egress_fd: file descriptor of the program to attach to interface egress
//
// Returns 0 on success, -1 if no valid file descriptor has been found, if
// the interface name is invalid or if an error ocurred during attach.
//
    pub egress_fd): *const *const int tc_prog_attach(char dev, int ingress_fd, int,

    pub subtest_name): *const c_char,
    pub ctx): *mut void traffic_monitor_stop(struct tmonitor_ctx,
    pub fn): tm_print_fn_t traffic_monitor_set_print(tm_print_fn_t,

    pub NULL: return,
    pub NULL: return,

