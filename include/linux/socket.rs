//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/socket.h
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

extern "C" {
    pub fn socket_seq_show(seq: *mut seq_file);
}

pub type sa_family_t = __kernel_sa_family_t;
//
// 1003.1g requires sa_family_t and that sa_data is char.
//
// Deprecated for in-kernel use. Use struct sockaddr_unsized instead.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr {
    pub /: *mut *mut sa_family_t sa_family; / address family, AF_xxx,
    pub /: *mut *mut char sa_data[14]; / 14 bytes of protocol address,
}

//
// struct sockaddr_unsized - Unspecified size sockaddr for callbacks
// @sa_family: Address family (AF_UNIX, AF_INET, AF_INET6, etc.)
// @sa_data: Flexible array for address data
//
// This structure is designed for callback interfaces where the
// total size is known via the sockaddr_len parameter. Unlike struct
// sockaddr which has a fixed 14-byte sa_data limit or struct
// sockaddr_storage which has a fixed 128-byte sa_data limit, this
// structure can accommodate addresses of any size, but must be used
// carefully.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_unsized {
    pub /: *mut *mut __kernel_sa_family_t sa_family; / address family, AF_xxx,
    pub /: *mut *mut char sa_data[]; / flexible address data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linger {
    pub /: *mut *mut int l_onoff; / Linger active,
    pub /: *mut *mut int l_linger; / How long to linger for,
}

//
// As we do 4.4BSD message passing we use a 4.4BSD message passing
// system, not 4.3. Thus msg_accrights(len) are now missing. They
// belong in an obscure libc emulation or the bin.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msghdr {
    pub /: *mut *mut *mut void msg_name; / ptr to socket address structure,
    pub /: *mut *mut int msg_namelen; / size of socket address structure,
    pub /: *mut *mut int msg_inq; / output, data left in socket,
    pub /: *mut *mut iov_iter msg_iter; / data,
//
// Ancillary data. msg_control_user is the user buffer used for the
// recv* side when msg_control_is_user is set, msg_control is the kernel
// buffer used for all other cases.
//
    pub msg_control: *mut c_void,
    pub msg_control_user: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_msghdr {
    pub /: *mut *mut *mut void __user msg_name; / ptr to socket address structure,
    pub /: *mut *mut int msg_namelen; / size of socket address structure,
    pub /: *mut *mut *mut iovec __user msg_iov; / scatter/gather array,
    pub /: *mut *mut __kernel_size_t msg_iovlen; / # elements in msg_iov,
    pub /: *mut *mut *mut void __user msg_control; / ancillary data,
    pub /: *mut *mut __kernel_size_t msg_controllen; / ancillary data buffer length,
    pub /: *mut *mut unsigned int msg_flags; / flags on received message,
}

// For recvmmsg/sendmmsg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmsghdr {
    pub msg_hdr: user_msghdr,
    pub msg_len: c_uint,
}

//
// POSIX 1003.1g - ancillary data object information
// Ancillary data consists of a sequence of pairs of
// (cmsghdr, cmsg_data[])
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmsghdr {
    pub /: *mut *mut __kernel_size_t cmsg_len; / data byte count, including hdr,
    pub /: *mut *mut int cmsg_level; / originating protocol,
    pub /: *mut *mut int cmsg_type; / protocol-specific type,
}

//
// Ancillary data object information MACROS
// Table 5-14 of POSIX 1003.1g
//

//
// Get the next cmsg header
//
// PLEASE, do not touch this function. If you think, that it is
// incorrect, grep kernel sources and think about consequences
// before trying to improve it.
//
// Now it always returns valid, not truncated ancillary object
// HEADER. But caller still MUST check, that cmsg->cmsg_len is
// inside range, given by msg->msg_controllen before using
// ancillary object DATA.				--ANK (980731)
//
extern "C" {
    pub fn __cmsg_nxthdr(_arg: __msg->msg_control, _arg: __msg->msg_controllen, _arg: __cmsg) -> return;
}
extern "C" {
    pub fn iov_iter_count(_arg: &msg->msg_iter) -> return;
}
// "Socket"-level control message types:
pub const SCM_RIGHTS: c_uint = 0x01		/* rw: access rights (array of int) */;
pub const SCM_CREDENTIALS: c_uint = 0x02		/* rw: struct ucred		*/;
pub const SCM_SECURITY: c_uint = 0x03		/* rw: security label		*/;
pub const SCM_PIDFD: c_uint = 0x04		/* ro: pidfd (int)		*/;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucred {
    pub pid: __u32,
    pub uid: __u32,
    pub gid: __u32,
}

// Supported address families.
pub const AF_UNSPEC: c_int = 0;

pub const AF_NETLINK: c_int = 16;

// PF_SMC protocol family that
// reuses AF_INET address family
//

// transport protocol
//

// Protocol families, same as address families.

// Maximum queue length specifiable by listen.
pub const SOMAXCONN: c_int = 4096;
// Flags we can use with send/ and recv.
//
pub const MSG_OOB: c_int = 1;
pub const MSG_PEEK: c_int = 2;
pub const MSG_DONTROUTE: c_int = 4;

pub const MSG_CTRUNC: c_int = 8;
pub const MSG_PROBE: c_uint = 0x10	/* Do not send. Only probe path f.e. for MTU */;
pub const MSG_TRUNC: c_uint = 0x20;
pub const MSG_DONTWAIT: c_uint = 0x40	/* Nonblocking io		 */;
pub const MSG_EOR: c_uint = 0x80	/* End of record */;
pub const MSG_WAITALL: c_uint = 0x100	/* Wait for a full request */;
pub const MSG_FIN: c_uint = 0x200;
pub const MSG_SYN: c_uint = 0x400;
pub const MSG_CONFIRM: c_uint = 0x800	/* Confirm path validity */;
pub const MSG_RST: c_uint = 0x1000;
pub const MSG_ERRQUEUE: c_uint = 0x2000	/* Fetch message from error queue */;
pub const MSG_NOSIGNAL: c_uint = 0x4000	/* Do not generate SIGPIPE */;
pub const MSG_MORE: c_uint = 0x8000	/* Sender will send more */;
pub const MSG_WAITFORONE: c_uint = 0x10000	/* recvmmsg(): block until 1+ packets avail */;
pub const MSG_SENDPAGE_NOPOLICY: c_uint = 0x10000 /* sendpage() internal : do no apply policy */;
pub const MSG_BATCH: c_uint = 0x40000 /* sendmmsg(): more messages coming */;

pub const MSG_NO_SHARED_FRAGS: c_uint = 0x80000 /* sendpage() internal : page frags are not shared */;
pub const MSG_SENDPAGE_DECRYPTED: c_uint = 0x100000 /* sendpage() internal : page may carry;
// plain text and require encryption
//
pub const MSG_SOCK_DEVMEM: c_uint = 0x2000000	/* Receive devmem skbs as cmsg */;
pub const MSG_ZEROCOPY: c_uint = 0x4000000	/* Use user data in kernel path */;
pub const MSG_SPLICE_PAGES: c_uint = 0x8000000	/* Splice the pages from the iterator in sendmsg() */;
pub const MSG_FASTOPEN: c_uint = 0x20000000	/* Send data in TCP SYN */;
pub const MSG_CMSG_CLOEXEC: c_uint = 0x40000000	/* Set close_on_exec for file;

pub const MSG_CMSG_COMPAT: c_uint = 0x80000000	/* This message needs 32 bit fixups */;

// Flags to be cleared on entry by sendmsg and sendmmsg syscalls

// Setsockoptions(2) level. Thanks to BSD these must match IPPROTO_xxx
pub const SOL_IP: c_int = 0;
// #define SOL_ICMP	1	No-no-no! Due to Linux :-) we cannot use SOL_ICMP=1
pub const SOL_TCP: c_int = 6;
pub const SOL_UDP: c_int = 17;
pub const SOL_IPV6: c_int = 41;
pub const SOL_ICMPV6: c_int = 58;
pub const SOL_SCTP: c_int = 132;

pub const SOL_RAW: c_int = 255;
pub const SOL_IPX: c_int = 256;
pub const SOL_AX25: c_int = 257;
pub const SOL_ATALK: c_int = 258;
pub const SOL_NETROM: c_int = 259;
pub const SOL_ROSE: c_int = 260;
pub const SOL_DECNET: c_int = 261;
pub const SOL_X25: c_int = 262;
pub const SOL_PACKET: c_int = 263;

pub const SOL_IRDA: c_int = 266;
pub const SOL_NETBEUI: c_int = 267;
pub const SOL_LLC: c_int = 268;
pub const SOL_DCCP: c_int = 269;
pub const SOL_NETLINK: c_int = 270;
pub const SOL_TIPC: c_int = 271;
pub const SOL_RXRPC: c_int = 272;
pub const SOL_PPPOL2TP: c_int = 273;
pub const SOL_BLUETOOTH: c_int = 274;
pub const SOL_PNPIPE: c_int = 275;
pub const SOL_RDS: c_int = 276;
pub const SOL_IUCV: c_int = 277;
pub const SOL_CAIF: c_int = 278;
pub const SOL_ALG: c_int = 279;
pub const SOL_NFC: c_int = 280;
pub const SOL_KCM: c_int = 281;
pub const SOL_TLS: c_int = 282;
pub const SOL_XDP: c_int = 283;
pub const SOL_MPTCP: c_int = 284;
pub const SOL_MCTP: c_int = 285;
pub const SOL_SMC: c_int = 286;
pub const SOL_VSOCK: c_int = 287;
// IPX options
pub const IPX_TYPE: c_int = 1;
extern "C" {
    pub fn move_addr_to_kernel(uaddr: *mut void __user, ulen: c_int, kaddr: *mut sockaddr_storage) -> c_int;
}
extern "C" {
    pub fn put_cmsg(msghdr*: *mut struct, level: c_int, type: c_int, len: c_int, data: *mut c_void) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_timestamping_internal {
    pub ts: [ktime_t; 3],
}

extern "C" {
    pub fn put_cmsg_scm_timestamping64(msg: *mut msghdr, tss: *mut scm_timestamping_internal);
}
extern "C" {
    pub fn put_cmsg_scm_timestamping(msg: *mut msghdr, tss: *mut scm_timestamping_internal);
}
// The __sys_...msg variants allow MSG_CMSG_COMPAT iff
// forbid_cmsg_compat==false
//
// helpers which do the actual work for syscalls
extern "C" {
    pub fn __sys_socket(family: c_int, type: c_int, protocol: c_int) -> c_int;
}
extern "C" {
    pub fn __sys_bind(fd: c_int, umyaddr: *mut sockaddr __user, addrlen: c_int) -> c_int;
}
extern "C" {
    pub fn __sys_listen(fd: c_int, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn __sys_listen_socket(sock: *mut socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn __sys_shutdown_sock(sock: *mut socket, how: c_int) -> c_int;
}
extern "C" {
    pub fn __sys_shutdown(fd: c_int, how: c_int) -> c_int;
}
