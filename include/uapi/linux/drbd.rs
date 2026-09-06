//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/drbd.h
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


// SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_io_error_p {
    EP_PASS_ON, /* FIXME should the better be named "Ignore"? */
    EP_CALL_HELPER,
    EP_DETACH
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_fencing_p {
    FP_NOT_AVAIL = -1, /* Not a policy */
    FP_DONT_CARE = 0,
    FP_RESOURCE,
    FP_STONITH
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_disconnect_p {
    DP_RECONNECT,
    DP_DROP_NET_CONF,
    DP_FREEZE_IO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_after_sb_p {
    ASB_DISCONNECT,
    ASB_DISCARD_YOUNGER_PRI,
    ASB_DISCARD_OLDER_PRI,
    ASB_DISCARD_ZERO_CHG,
    ASB_DISCARD_LEAST_CHG,
    ASB_DISCARD_LOCAL,
    ASB_DISCARD_REMOTE,
    ASB_CONSENSUS,
    ASB_DISCARD_SECONDARY,
    ASB_CALL_HELPER,
    ASB_VIOLENTLY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_on_no_data {
    OND_IO_ERROR,
    OND_SUSPEND_IO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_on_congestion {
    OC_BLOCK,
    OC_PULL_AHEAD,
    OC_DISCONNECT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_read_balancing {
    RB_PREFER_LOCAL,
    RB_PREFER_REMOTE,
    RB_ROUND_ROBIN,
    RB_LEAST_PENDING,
    RB_CONGESTED_REMOTE,
    RB_32K_STRIPING,
    RB_64K_STRIPING,
    RB_128K_STRIPING,
    RB_256K_STRIPING,
    RB_512K_STRIPING,
    RB_1M_STRIPING,
}

// KEEP the order, do not delete or insert. Only append.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_ret_code {
    ERR_CODE_BASE		= 100,
    NO_ERROR		= 101,
    ERR_LOCAL_ADDR		= 102,
    ERR_PEER_ADDR		= 103,
    ERR_OPEN_DISK		= 104,
    ERR_OPEN_MD_DISK	= 105,
    ERR_DISK_NOT_BDEV	= 107,
    ERR_MD_NOT_BDEV		= 108,
    ERR_DISK_TOO_SMALL	= 111,
    ERR_MD_DISK_TOO_SMALL	= 112,
    ERR_BDCLAIM_DISK	= 114,
    ERR_BDCLAIM_MD_DISK	= 115,
    ERR_MD_IDX_INVALID	= 116,
    ERR_IO_MD_DISK		= 118,
    ERR_MD_INVALID          = 119,
    ERR_AUTH_ALG		= 120,
    ERR_AUTH_ALG_ND		= 121,
    ERR_NOMEM		= 122,
    ERR_DISCARD_IMPOSSIBLE	= 123,
    ERR_DISK_CONFIGURED	= 124,
    ERR_NET_CONFIGURED	= 125,
    ERR_MANDATORY_TAG	= 126,
    ERR_MINOR_INVALID	= 127,
    ERR_INTR		= 129, /* EINTR */
    ERR_RESIZE_RESYNC	= 130,
    ERR_NO_PRIMARY		= 131,
    ERR_RESYNC_AFTER	= 132,
    ERR_RESYNC_AFTER_CYCLE	= 133,
    ERR_PAUSE_IS_SET	= 134,
    ERR_PAUSE_IS_CLEAR	= 135,
    ERR_PACKET_NR		= 137,
    ERR_NO_DISK		= 138,
    ERR_NOT_PROTO_C		= 139,
    ERR_NOMEM_BITMAP	= 140,
    ERR_INTEGRITY_ALG	= 141, /* DRBD 8.2 only */
    ERR_INTEGRITY_ALG_ND	= 142, /* DRBD 8.2 only */
    ERR_CPU_MASK_PARSE	= 143, /* DRBD 8.2 only */
    ERR_CSUMS_ALG		= 144, /* DRBD 8.2 only */
    ERR_CSUMS_ALG_ND	= 145, /* DRBD 8.2 only */
    ERR_VERIFY_ALG		= 146, /* DRBD 8.2 only */
    ERR_VERIFY_ALG_ND	= 147, /* DRBD 8.2 only */
    ERR_CSUMS_RESYNC_RUNNING= 148, /* DRBD 8.2 only */
    ERR_VERIFY_RUNNING	= 149, /* DRBD 8.2 only */
    ERR_DATA_NOT_CURRENT	= 150,
    ERR_CONNECTED		= 151, /* DRBD 8.3 only */
    ERR_PERM		= 152,
    ERR_NEED_APV_93		= 153,
    ERR_STONITH_AND_PROT_A  = 154,
    ERR_CONG_NOT_PROTO_A	= 155,
    ERR_PIC_AFTER_DEP	= 156,
    ERR_PIC_PEER_DEP	= 157,
    ERR_RES_NOT_KNOWN	= 158,
    ERR_RES_IN_USE		= 159,
    ERR_MINOR_CONFIGURED    = 160,
    ERR_MINOR_OR_VOLUME_EXISTS = 161,
    ERR_INVALID_REQUEST	= 162,
    ERR_NEED_APV_100	= 163,
    ERR_NEED_ALLOW_TWO_PRI  = 164,
    ERR_MD_UNCLEAN          = 165,
    ERR_MD_LAYOUT_CONNECTED = 166,
    ERR_MD_LAYOUT_TOO_BIG   = 167,
    ERR_MD_LAYOUT_TOO_SMALL = 168,
    ERR_MD_LAYOUT_NO_FIT    = 169,
    ERR_IMPLICIT_SHRINK     = 170,
// insert new ones above this line
    AFTER_LAST_ERR_CODE
}

pub const DRBD_PROT_A: c_int = 1;
pub const DRBD_PROT_B: c_int = 2;
pub const DRBD_PROT_C: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_role {
    R_UNKNOWN = 0,
    R_PRIMARY = 1,     /* role */
    R_SECONDARY = 2,   /* role */
    R_MASK = 3,
}

// The order of these constants is important.
// The lower ones (<C_WF_REPORT_PARAMS) indicate
// that there is no socket!
// >=C_WF_REPORT_PARAMS ==> There is a socket
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_conns {
    C_STANDALONE,
    C_DISCONNECTING,  /* Temporal state on the way to StandAlone. */
    C_UNCONNECTED,    /* >= C_UNCONNECTED -> inc_net() succeeds */

// These temporal states are all used on the way
// from >= C_CONNECTED to Unconnected.
// The 'disconnect reason' states
// I do not allow to change between them.
    C_TIMEOUT,
    C_BROKEN_PIPE,
    C_NETWORK_FAILURE,
    C_PROTOCOL_ERROR,
    C_TEAR_DOWN,

    C_WF_CONNECTION,
    C_WF_REPORT_PARAMS, /* we have a socket */
    C_CONNECTED,      /* we have introduced each other */
    C_STARTING_SYNC_S,  /* starting full sync by admin request. */
    C_STARTING_SYNC_T,  /* starting full sync by admin request. */
    C_WF_BITMAP_S,
    C_WF_BITMAP_T,
    C_WF_SYNC_UUID,

// All SyncStates are tested with this comparison
// xx >= C_SYNC_SOURCE && xx <= C_PAUSED_SYNC_T
    C_SYNC_SOURCE,
    C_SYNC_TARGET,
    C_VERIFY_S,
    C_VERIFY_T,
    C_PAUSED_SYNC_S,
    C_PAUSED_SYNC_T,

    C_AHEAD,
    C_BEHIND,

    C_MASK = 31
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_disk_state {
    D_DISKLESS,
    D_ATTACHING,      /* In the process of reading the meta-data */
    D_FAILED,         /* Becomes D_DISKLESS as soon as we told it the peer */
// when >= D_FAILED it is legal to access mdev->ldev
    D_NEGOTIATING,    /* Late attaching state, we need to talk to the peer */
    D_INCONSISTENT,
    D_OUTDATED,
    D_UNKNOWN,       /* Only used for the peer, never for myself */
    D_CONSISTENT,     /* Might be D_OUTDATED, might be D_UP_TO_DATE ... */
    D_UP_TO_DATE,       /* Only this disk state allows applications' IO ! */
    D_MASK = 15
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drbd_state {
// According to gcc's docs is the ...
// The order of allocation of bit-fields within a unit (C90 6.5.2.1, C99 6.7.2.1).
// Determined by ABI.
// pointed out by Maxim Uvarov q<muvarov@ru.mvista.com>
// even though we transmit as "cpu_to_be32(state)",
// the offsets of the bitfields still need to be swapped
// on different endianness.
//

    pub /: *mut *mut unsigned role:2 ; / 3/4 primary/secondary/unknown,
    pub /: *mut *mut unsigned peer:2 ; / 3/4 primary/secondary/unknown,
    pub /: *mut *mut unsigned conn:5 ; / 17/32 cstates,
    pub /: *mut *mut unsigned disk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub /: *mut *mut unsigned pdsk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub /: *mut *mut unsigned susp:1 ; / 2/2 IO suspended no/yes (by user),
    pub /: *mut *mut unsigned aftr_isp:1 ; / isp .. imposed sync pause,
    pub peer_isp:1: unsigned,
    pub user_isp:1: unsigned,
    pub /: *mut *mut unsigned susp_nod:1 ; / IO suspended because no data,
    pub runs*/: *mut *mut unsigned susp_fen:1 ; / IO suspended because fence peer handler,
    pub /: *mut *mut unsigned _pad:9; / 0 unused,

    pub _pad:9: unsigned,
    pub susp_fen:1: unsigned,
    pub susp_nod:1: unsigned,
    pub user_isp:1: unsigned,
    pub peer_isp:1: unsigned,
    pub /: *mut *mut unsigned aftr_isp:1 ; / isp .. imposed sync pause,
    pub /: *mut *mut unsigned susp:1 ; / 2/2 IO suspended no/yes,
    pub /: *mut *mut unsigned pdsk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub /: *mut *mut unsigned disk:4 ; / 8/16 from D_DISKLESS to D_UP_TO_DATE,
    pub /: *mut *mut unsigned conn:5 ; / 17/32 cstates,
    pub /: *mut *mut unsigned peer:2 ; / 3/4 primary/secondary/unknown,
    pub /: *mut *mut unsigned role:2 ; / 3/4 primary/secondary/unknown,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_state_rv {
    SS_CW_NO_NEED = 4,
    SS_CW_SUCCESS = 3,
    SS_NOTHING_TO_DO = 2,
    SS_SUCCESS = 1,
    SS_UNKNOWN_ERROR = 0, /* Used to sleep longer in _drbd_request_state */
    SS_TWO_PRIMARIES = -1,
    SS_NO_UP_TO_DATE_DISK = -2,
    SS_NO_LOCAL_DISK = -4,
    SS_NO_REMOTE_DISK = -5,
    SS_CONNECTED_OUTDATES = -6,
    SS_PRIMARY_NOP = -7,
    SS_RESYNC_RUNNING = -8,
    SS_ALREADY_STANDALONE = -9,
    SS_CW_FAILED_BY_PEER = -10,
    SS_IS_DISKLESS = -11,
    SS_DEVICE_IN_USE = -12,
    SS_NO_NET_CONFIG = -13,
    SS_NO_VERIFY_ALG = -14,       /* drbd-8.2 only */
    SS_NEED_CONNECTION = -15,    /* drbd-8.2 only */
    SS_LOWER_THAN_OUTDATED = -16,
    SS_NOT_SUPPORTED = -17,      /* drbd-8.2 only */
    SS_IN_TRANSIENT_STATE = -18,  /* Retry after the next state change */
    SS_CONCURRENT_ST_CHG = -19,   /* Concurrent cluster side state change! */
    SS_O_VOL_PEER_PRI = -20,
    SS_OUTDATE_WO_CONN = -21,
    SS_AFTER_LAST_ERROR = -22,    /* Keep this at bottom */
}

pub const SHARED_SECRET_MAX: c_int = 64;

pub const MAX_PEERS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_uuid_index {
    UI_CURRENT,
    UI_BITMAP,
    UI_HISTORY_START,
    UI_HISTORY_END,
    UI_SIZE,      /* nl-packet: number of dirty bits */
    UI_FLAGS,     /* nl-packet: flags */
    UI_EXTENDED_SIZE   /* Everything. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_timeout_flag {
    UT_DEFAULT      = 0,
    UT_DEGRADED     = 1,
    UT_PEER_OUTDATED = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_notification_type {
    NOTIFY_EXISTS,
    NOTIFY_CREATE,
    NOTIFY_CHANGE,
    NOTIFY_DESTROY,
    NOTIFY_CALL,
    NOTIFY_RESPONSE,

    NOTIFY_CONTINUES = 0x8000,
    NOTIFY_FLAGS = NOTIFY_CONTINUES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_peer_state {
    P_INCONSISTENT = 3,
    P_OUTDATED = 4,
    P_DOWN = 5,
    P_PRIMARY = 6,
    P_FENCING = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum write_ordering_e {
    WO_NONE,
    WO_DRAIN_IO,
    WO_BDEV_FLUSH,
    WO_BIO_BARRIER
}

// magic numbers used in meta data and network packets
pub const DRBD_MAGIC: c_uint = 0x83740267;
pub const DRBD_MAGIC_BIG: c_uint = 0x835a;
pub const DRBD_MAGIC_100: c_uint = 0x8620ec20;

// how I came up with this magic?
// base64 decode "actlog==" ;)
pub const DRBD_AL_MAGIC: c_uint = 0x69cb65a2;
// these are of type "int"

pub const DRBD_CPU_MASK_SIZE: c_int = 32;
//
// struct drbd_genlmsghdr - DRBD specific header used in NETLINK_GENERIC requests
// @minor:
// For admin requests (user -> kernel): which minor device to operate on.
// For (unicast) replies or informational (broadcast) messages
// (kernel -> user): which minor device the information is about.
// If we do not operate on minors, but on connections or resources,
// the minor value shall be (~0), and the attribute DRBD_NLA_CFG_CONTEXT
// is used instead.
// @flags: possible operation modifiers (relevant only for user->kernel):
// DRBD_GENL_F_SET_DEFAULTS
// @volume:
// When creating a new minor (adding it to a resource), the resource needs
// to know which volume number within the resource this is supposed to be.
// The volume number corresponds to the same volume number on the remote side,
// whereas the minor number on the remote side may be different
// (union with flags).
// @ret_code: kernel->userland unicast cfg reply return code (union with flags);
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_genlmsghdr {
    pub minor: __u32,
    pub flags: __u32,
    pub ret_code: __s32,
}

// To be used in drbd_genlmsghdr.flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_state_info_bcast_reason {
    SIB_GET_STATUS_REPLY = 1,
    SIB_STATE_CHANGE = 2,
    SIB_HELPER_PRE = 3,
    SIB_HELPER_POST = 4,
    SIB_SYNC_PROGRESS = 5,
}
