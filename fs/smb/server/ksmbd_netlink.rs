//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/server/ksmbd_netlink.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2018 Samsung Electronics Co., Ltd.
//
// linux-ksmbd-devel@lists.sourceforge.net
//

//
// This is a userspace ABI to communicate data between ksmbd and user IPC
// daemon using netlink. This is added to track and cache user account DB
// and share configuration info from userspace.
//
// - KSMBD_EVENT_HEARTBEAT_REQUEST(ksmbd_heartbeat)
// This event is to check whether user IPC daemon is alive. If user IPC
// daemon is dead, ksmbd keep existing connection till disconnecting and
// new connection will be denied.
//
// - KSMBD_EVENT_STARTING_UP(ksmbd_startup_request)
// This event is to receive the information that initializes the ksmbd
// server from the user IPC daemon and to start the server. The global
// section parameters are given from smb.conf as initialization
// information.
//
// - KSMBD_EVENT_SHUTTING_DOWN(ksmbd_shutdown_request)
// This event is to shutdown ksmbd server.
//
// - KSMBD_EVENT_LOGIN_REQUEST/RESPONSE(ksmbd_login_request/response)
// This event is to get user account info to user IPC daemon.
//
// - KSMBD_EVENT_SHARE_CONFIG_REQUEST/RESPONSE(ksmbd_share_config_request/response)
// This event is to get net share configuration info.
//
// - KSMBD_EVENT_TREE_CONNECT_REQUEST/RESPONSE(ksmbd_tree_connect_request/response)
// This event is to get session and tree connect info.
//
// - KSMBD_EVENT_TREE_DISCONNECT_REQUEST(ksmbd_tree_disconnect_request)
// This event is to send tree disconnect info to user IPC daemon.
//
// - KSMBD_EVENT_LOGOUT_REQUEST(ksmbd_logout_request)
// This event is to send logout request to user IPC daemon.
//
// - KSMBD_EVENT_RPC_REQUEST/RESPONSE(ksmbd_rpc_command)
// This event is to make DCE/RPC request like srvsvc, wkssvc, lsarpc,
// samr to be processed in userspace.
//
// - KSMBD_EVENT_SPNEGO_AUTHEN_REQUEST/RESPONSE(ksmbd_spnego_authen_request/response)
// This event is to make kerberos authentication to be processed in
// userspace.
//
// - KSMBD_EVENT_LOGIN_REQUEST_EXT/RESPONSE_EXT(ksmbd_login_request_ext/response_ext)
// This event is to get user account extension info to user IPC daemon.
//

pub const KSMBD_GENL_VERSION: c_uint = 0x01;
pub const KSMBD_REQ_MAX_ACCOUNT_NAME_SZ: c_int = 48;
pub const KSMBD_REQ_MAX_HASH_SZ: c_int = 18;
pub const KSMBD_REQ_MAX_SHARE_NAME: c_int = 64;
//
// IPC heartbeat frame to check whether user IPC daemon is alive.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_heartbeat {
    pub handle: __u32,
}

//
// Global config flags.
//

//
// IPC request for ksmbd server startup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_startup_request {
    pub /: *mut *mut __u32 flags; / Flags for global config,
    pub /: *mut *mut __s32 signing; / Signing enabled,
    pub /: *mut *mut __s8 min_prot[16]; / The minimum SMB protocol version,
    pub /: *mut *mut __s8 max_prot[16]; / The maximum SMB protocol version,
    pub netbios_name: [__s8; 16],
    pub /: *mut *mut __s8 work_group[64]; / Workgroup,
    pub /: *mut *mut __s8 server_string[64]; / Server string,
    pub /: *mut *mut __u16 tcp_port; / tcp port,
    pub /*: *mut __u16 ipc_timeout;,
// specifies the number of seconds
// server will wait for the userspace to
// reply to heartbeat frames.
//
    pub /: *mut *mut __u32 deadtime; / Number of minutes of inactivity,
    pub /: *mut *mut __u32 file_max; / Limits the maximum number of open files,
    pub /: *mut *mut __u32 smb2_max_write; / MAX write size,
    pub /: *mut *mut __u32 smb2_max_read; / MAX read size,
    pub /: *mut *mut __u32 smb2_max_trans; / MAX trans size,
    pub /*: *mut __u32 share_fake_fscaps;,
// Support some special application that
// makes QFSINFO calls to check whether
// we set the SPARSE_FILES bit (0x40).
//
    pub /: *mut *mut __u32 sub_auth[3]; / Subauth value for Security ID,
    pub /: *mut *mut __u32 smb2_max_credits; / MAX credits,
    pub /: *mut *mut __u32 smbd_max_io_size; / smbd read write size,
    pub /: *mut *mut __u32 max_connections; / Number of maximum simultaneous connections,
    pub bind_interfaces_only: __s8,
    pub /: *mut *mut __u32 max_ip_connections; / Number of maximum connection per ip address,
    pub /: *mut *mut __s8 aapl_model[32]; / AAPL model string for Finder icon, e.g. "Xserve",
    pub /: *mut *mut __s8 reserved[467]; / Reserved room,
    pub /: *mut *mut __u32 ifc_list_sz; / interfaces list size,
    pub ____payload: [__s8; ],
    pub __packed: },

//
// IPC request to shutdown ksmbd server.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_shutdown_request {
    pub reserved: [__s32; 16],
}

//
// IPC user login request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_login_request {
    pub handle: __u32,
    pub /: *mut *mut __s8 account[KSMBD_REQ_MAX_ACCOUNT_NAME_SZ]; / user account name,
    pub /: *mut *mut __u32 reserved[16]; / Reserved room,
}

//
// IPC user login response.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_login_response {
    pub handle: __u32,
    pub /: *mut *mut __u32 gid; / group id,
    pub /: *mut *mut __u32 uid; / user id,
    pub /: *mut *mut __s8 account[KSMBD_REQ_MAX_ACCOUNT_NAME_SZ]; / user account name,
    pub status: __u16,
    pub /: *mut *mut __u16 hash_sz; / hash size,
    pub /: *mut *mut __s8 hash[KSMBD_REQ_MAX_HASH_SZ]; / password hash,
    pub /: *mut *mut __u32 reserved[16]; / Reserved room,
}

//
// IPC user login response extension.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_login_response_ext {
    pub handle: __u32,
    pub /: *mut *mut __s32 ngroups; / supplementary group count,
    pub /: *mut *mut __s8 reserved[128]; / Reserved room,
    pub ____payload: [__s8; ],
}

//
// IPC request to fetch net share config.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_share_config_request {
    pub handle: __u32,
    pub /: *mut *mut __s8 share_name[KSMBD_REQ_MAX_SHARE_NAME]; / share name,
    pub /: *mut *mut __u32 reserved[16]; / Reserved room,
}

//
// IPC response to the net share config request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_share_config_response {
    pub handle: __u32,
    pub flags: __u32,
    pub create_mask: __u16,
    pub directory_mask: __u16,
    pub force_create_mode: __u16,
    pub force_directory_mode: __u16,
    pub force_uid: __u16,
    pub force_gid: __u16,
    pub share_name: [__s8; KSMBD_REQ_MAX_SHARE_NAME],
    pub /: *mut *mut __u32 reserved[111]; / Reserved room,
    pub payload_sz: __u32,
    pub veto_list_sz: __u32,
    pub ____payload: [__s8; ],
}

//
// IPC request for tree connection. This request include session and tree
// connect info from client.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_tree_connect_request {
    pub handle: __u32,
    pub account_flags: __u16,
    pub flags: __u16,
    pub session_id: __u64,
    pub connect_id: __u64,
    pub account: [__s8; KSMBD_REQ_MAX_ACCOUNT_NAME_SZ],
    pub share: [__s8; KSMBD_REQ_MAX_SHARE_NAME],
    pub peer_addr: [__s8; 64],
    pub /: *mut *mut __u32 reserved[16]; / Reserved room,
}

//
// IPC Response structure for tree connection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_tree_connect_response {
    pub handle: __u32,
    pub status: __u16,
    pub connection_flags: __u16,
    pub /: *mut *mut __u32 reserved[16]; / Reserved room,
}

//
// IPC Request structure to disconnect tree connection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_tree_disconnect_request {
    pub /: *mut *mut __u64 session_id; / session id,
    pub /: *mut *mut __u64 connect_id; / tree connection id,
    pub /: *mut *mut __u32 reserved[16]; / Reserved room,
}

//
// IPC Response structure to logout user account.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_logout_request {
    pub /: *mut *mut __s8 account[KSMBD_REQ_MAX_ACCOUNT_NAME_SZ]; / user account name,
    pub account_flags: __u32,
    pub /: *mut *mut __u32 reserved[16]; / Reserved room,
}

//
// RPC command structure to send rpc request like srvsvc or wkssvc to
// IPC user daemon.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_rpc_command {
    pub handle: __u32,
    pub flags: __u32,
    pub payload_sz: __u32,
    pub payload: [__u8; ],
}

//
// IPC Request Kerberos authentication
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_spnego_authen_request {
    pub handle: __u32,
    pub /: *mut *mut __u16 spnego_blob_len; / the length of spnego_blob,
    pub /*: *mut __u8 spnego_blob[];,
// the GSS token from SecurityBuffer of
// SMB2 SESSION SETUP request
//
}

//
// Response data which includes the GSS token and the session key generated by
// user daemon.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksmbd_spnego_authen_response {
    pub handle: __u32,
    pub /*: *mut ksmbd_login_response login_response;,
// the login response with
// a user identified by the
// GSS token from a client
//
    pub /: *mut *mut __u16 session_key_len; / the length of the session key,
    pub /*: *mut __u16 spnego_blob_len;,
// the length of  the GSS token which will be
// stored in SecurityBuffer of SMB2 SESSION
// SETUP response
//
    pub /: *mut *mut __u64 session_expiry; / Kerberos ticket expiry time,
    pub /: *mut *mut __u8 payload[]; / session key + AP_REP,
}

//
// This also used as NETLINK attribute type value.
//
// NOTE:
// Response message type value should be equal to
// request message type value + 1.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ksmbd_event {
    KSMBD_EVENT_UNSPEC			= 0,
    KSMBD_EVENT_HEARTBEAT_REQUEST,

    KSMBD_EVENT_STARTING_UP,
    KSMBD_EVENT_SHUTTING_DOWN,

    KSMBD_EVENT_LOGIN_REQUEST,
    KSMBD_EVENT_LOGIN_RESPONSE		= 5,

    KSMBD_EVENT_SHARE_CONFIG_REQUEST,
    KSMBD_EVENT_SHARE_CONFIG_RESPONSE,

    KSMBD_EVENT_TREE_CONNECT_REQUEST,
    KSMBD_EVENT_TREE_CONNECT_RESPONSE,

    KSMBD_EVENT_TREE_DISCONNECT_REQUEST	= 10,

    KSMBD_EVENT_LOGOUT_REQUEST,

    KSMBD_EVENT_RPC_REQUEST,
    KSMBD_EVENT_RPC_RESPONSE,

    KSMBD_EVENT_SPNEGO_AUTHEN_REQUEST,
    KSMBD_EVENT_SPNEGO_AUTHEN_RESPONSE	= 15,

    KSMBD_EVENT_LOGIN_REQUEST_EXT,
    KSMBD_EVENT_LOGIN_RESPONSE_EXT,

    __KSMBD_EVENT_MAX,
    KSMBD_EVENT_MAX = __KSMBD_EVENT_MAX - 1
}

//
// Enumeration for IPC tree connect status.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KSMBD_TREE_CONN_STATUS {
    KSMBD_TREE_CONN_STATUS_OK		= 0,
    KSMBD_TREE_CONN_STATUS_NOMEM,
    KSMBD_TREE_CONN_STATUS_NO_SHARE,
    KSMBD_TREE_CONN_STATUS_NO_USER,
    KSMBD_TREE_CONN_STATUS_INVALID_USER,
    KSMBD_TREE_CONN_STATUS_HOST_DENIED	= 5,
    KSMBD_TREE_CONN_STATUS_CONN_EXIST,
    KSMBD_TREE_CONN_STATUS_TOO_MANY_CONNS,
    KSMBD_TREE_CONN_STATUS_TOO_MANY_SESSIONS,
    KSMBD_TREE_CONN_STATUS_ERROR,
}

//
// User config flags.
//

//
// Share config flags.
//

// Keep BIT(19) reserved for the existing ksmbd-tools WIDE_LINKS flag.

//
// Tree connect request flags.
//

//
// Tree connect flags.
//

//
// RPC over IPC.
//

//
// RPC status definitions.
//
pub const KSMBD_RPC_OK: c_int = 0;
pub const KSMBD_RPC_EBAD_FUNC: c_uint = 0x00000001;
pub const KSMBD_RPC_EACCESS_DENIED: c_uint = 0x00000005;
pub const KSMBD_RPC_EBAD_FID: c_uint = 0x00000006;
pub const KSMBD_RPC_ENOMEM: c_uint = 0x00000008;
pub const KSMBD_RPC_EBAD_DATA: c_uint = 0x0000000D;
pub const KSMBD_RPC_ENOTIMPLEMENTED: c_uint = 0x00000040;
pub const KSMBD_RPC_EINVALID_PARAMETER: c_uint = 0x00000057;
pub const KSMBD_RPC_EMORE_DATA: c_uint = 0x000000EA;
pub const KSMBD_RPC_EINVALID_LEVEL: c_uint = 0x0000007C;
pub const KSMBD_RPC_SOME_NOT_MAPPED: c_uint = 0x00000107;
pub const KSMBD_CONFIG_OPT_DISABLED: c_int = 0;
pub const KSMBD_CONFIG_OPT_ENABLED: c_int = 1;
pub const KSMBD_CONFIG_OPT_AUTO: c_int = 2;
pub const KSMBD_CONFIG_OPT_MANDATORY: c_int = 3;
