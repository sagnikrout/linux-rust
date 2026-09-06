//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/fungible/funcore/fun_hci.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_admin_op {
    FUN_ADMIN_OP_BIND = 0x1,
    FUN_ADMIN_OP_EPCQ = 0x11,
    FUN_ADMIN_OP_EPSQ = 0x12,
    FUN_ADMIN_OP_PORT = 0x13,
    FUN_ADMIN_OP_ETH = 0x14,
    FUN_ADMIN_OP_VI = 0x15,
    FUN_ADMIN_OP_SWUPGRADE = 0x1f,
    FUN_ADMIN_OP_RSS = 0x21,
    FUN_ADMIN_OP_ADI = 0x25,
    FUN_ADMIN_OP_KTLS = 0x26,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_req_common {
    pub op: __u8,
    pub len8: __u8,
    pub flags: __be16,
    pub suboff8: __u8,
    pub rsvd0: __u8,
    pub cid: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_rsp_common {
    pub op: __u8,
    pub len8: __u8,
    pub flags: __be16,
    pub suboff8: __u8,
    pub ret: __u8,
    pub cid: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_write48_req {
    pub key_to_data: __be64,
}

pub const FUN_ADMIN_WRITE48_REQ_KEY_M: c_uint = 0xff;

pub const FUN_ADMIN_WRITE48_REQ_DATA_M: c_uint = 0xffffffffffff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_write48_rsp {
    pub key_to_data: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_read48_req {
    pub key_pack: __be64,
}

pub const FUN_ADMIN_READ48_REQ_KEY_M: c_uint = 0xff;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_read48_rsp {
    pub key_to_data: __be64,
}

pub const FUN_ADMIN_READ48_RSP_KEY_M: c_uint = 0xff;

pub const FUN_ADMIN_READ48_RSP_RET_M: c_uint = 0xff;

pub const FUN_ADMIN_READ48_RSP_DATA_M: c_uint = 0xffffffffffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_admin_bind_type {
    FUN_ADMIN_BIND_TYPE_EPCQ = 0x1,
    FUN_ADMIN_BIND_TYPE_EPSQ = 0x2,
    FUN_ADMIN_BIND_TYPE_PORT = 0x3,
    FUN_ADMIN_BIND_TYPE_RSS = 0x4,
    FUN_ADMIN_BIND_TYPE_VI = 0x5,
    FUN_ADMIN_BIND_TYPE_ETH = 0x6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_bind_entry {
    pub type: __u8,
    pub rsvd0: [__u8; 3],
    pub id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_bind_req {
    pub common: fun_admin_req_common,
    pub entry: [fun_admin_bind_entry; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_bind_rsp {
    pub bind_rsp_common: fun_admin_rsp_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_simple_subop {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub data: __be32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_admin_subop {
    FUN_ADMIN_SUBOP_CREATE = 0x10,
    FUN_ADMIN_SUBOP_DESTROY = 0x11,
    FUN_ADMIN_SUBOP_MODIFY = 0x12,
    FUN_ADMIN_SUBOP_RES_COUNT = 0x14,
    FUN_ADMIN_SUBOP_READ = 0x15,
    FUN_ADMIN_SUBOP_WRITE = 0x16,
    FUN_ADMIN_SUBOP_NOTIFY = 0x17,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_generic_destroy_req {
    pub common: fun_admin_req_common,
    pub destroy: fun_admin_simple_subop,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_generic_create_rsp {
    pub common: fun_admin_rsp_common,
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_res_count_req {
    pub common: fun_admin_req_common,
    pub count: fun_admin_simple_subop,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_res_count_rsp {
    pub common: fun_admin_rsp_common,
    pub count: fun_admin_simple_subop,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_epcq_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union epcq_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_epcq_create_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub epsqid: __be32,
    pub rsvd1: __u8,
    pub entry_size_log2: __u8,
    pub nentries: __be16,
    pub address: __be64,
    pub /: *mut *mut __be16 tailroom; / per packet tailroom in bytes,
    pub /: *mut *mut __u8 headroom; / per packet headroom in 2B units,
    pub intcoal_kbytes: __u8,
    pub intcoal_holdoff_nentries: __u8,
    pub intcoal_holdoff_usecs: __u8,
    pub intid: __be16,
    pub scan_start_id: __be32,
    pub scan_end_id: __be32,
    pub tph_cpuid: __be16,
    pub rsvd3: [__u8; 6],
    pub create: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_epcq_modify_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub /: *mut *mut __be16 headroom; / headroom in bytes,
    pub rsvd1: [__u8; 6],
    pub modify: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_epsq_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union epsq_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_epsq_create_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub epcqid: __be32,
    pub rsvd1: __u8,
    pub entry_size_log2: __u8,
    pub nentries: __be16,
    pub /: *mut *mut __be64 address; / DMA address of epsq,
    pub rsvd2: [__u8; 3],
    pub intcoal_kbytes: __u8,
    pub intcoal_holdoff_nentries: __u8,
    pub intcoal_holdoff_usecs: __u8,
    pub intid: __be16,
    pub scan_start_id: __be32,
    pub scan_end_id: __be32,
    pub rsvd3: [__u8; 4],
    pub tph_cpuid: __be16,
    pub /: *mut *mut __u8 buf_size_log2; / log2 of RQ buffer size,
    pub /: *mut *mut __u8 head_wb_size_log2; / log2 of head write back size,
    pub /: *mut *mut __be64 head_wb_address; / DMA address for head writeback,
    pub create: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_port_brkout_mode {
    FUN_PORT_BRKMODE_NA = 0x0,
    FUN_PORT_BRKMODE_NONE = 0x1,
    FUN_PORT_BRKMODE_2X = 0x2,
    FUN_PORT_BRKMODE_4X = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_port_duplex_mode {
    FUN_PORT_FULL_DUPLEX = 0x0,
    FUN_PORT_HALF_DUPLEX = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_port_link_status {
    FUN_PORT_LINK_UP = 0x0,
    FUN_PORT_LINK_UP_WITH_ERR = 0x1,
    FUN_PORT_LINK_DOWN = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_port_led_type {
    FUN_PORT_LED_OFF = 0x0,
    FUN_PORT_LED_AMBER = 0x1,
    FUN_PORT_LED_GREEN = 0x2,
    FUN_PORT_LED_BEACON_ON = 0x3,
    FUN_PORT_LED_BEACON_OFF = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_port_lane_attr {
    FUN_PORT_LANE_1 = 0x1,
    FUN_PORT_LANE_2 = 0x2,
    FUN_PORT_LANE_4 = 0x4,
    FUN_PORT_LANE_SPEED_10G = 0x100,
    FUN_PORT_LANE_SPEED_25G = 0x200,
    FUN_PORT_LANE_SPEED_50G = 0x400,
    FUN_PORT_LANE_SPLIT = 0x8000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_admin_port_subop {
    FUN_ADMIN_PORT_SUBOP_XCVR_READ = 0x23,
    FUN_ADMIN_PORT_SUBOP_INETADDR_EVENT = 0x24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_admin_port_key {
    FUN_ADMIN_PORT_KEY_ILLEGAL = 0x0,
    FUN_ADMIN_PORT_KEY_MTU = 0x1,
    FUN_ADMIN_PORT_KEY_FEC = 0x2,
    FUN_ADMIN_PORT_KEY_SPEED = 0x3,
    FUN_ADMIN_PORT_KEY_DEBOUNCE = 0x4,
    FUN_ADMIN_PORT_KEY_DUPLEX = 0x5,
    FUN_ADMIN_PORT_KEY_MACADDR = 0x6,
    FUN_ADMIN_PORT_KEY_LINKMODE = 0x7,
    FUN_ADMIN_PORT_KEY_BREAKOUT = 0x8,
    FUN_ADMIN_PORT_KEY_ENABLE = 0x9,
    FUN_ADMIN_PORT_KEY_DISABLE = 0xa,
    FUN_ADMIN_PORT_KEY_ERR_DISABLE = 0xb,
    FUN_ADMIN_PORT_KEY_CAPABILITIES = 0xc,
    FUN_ADMIN_PORT_KEY_LP_CAPABILITIES = 0xd,
    FUN_ADMIN_PORT_KEY_STATS_DMA_LOW = 0xe,
    FUN_ADMIN_PORT_KEY_STATS_DMA_HIGH = 0xf,
    FUN_ADMIN_PORT_KEY_LANE_ATTRS = 0x10,
    FUN_ADMIN_PORT_KEY_LED = 0x11,
    FUN_ADMIN_PORT_KEY_ADVERT = 0x12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_subop_imm {
    pub /: *mut *mut __u8 subop; / see fun_data_subop enum,
    pub flags: __u8,
    pub nsgl: __u8,
    pub rsvd0: __u8,
    pub len: __be32,
    pub data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_subop_sgl_flags {
    FUN_SUBOP_SGL_USE_OFF8 = 0x1,
    FUN_SUBOP_FLAG_FREE_BUF = 0x2,
    FUN_SUBOP_FLAG_IS_REFBUF = 0x4,
    FUN_SUBOP_SGL_FLAG_LOCAL = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_data_op {
    FUN_DATAOP_INVALID = 0x0,
    FUN_DATAOP_SL = 0x1, /* scatter */
    FUN_DATAOP_GL = 0x2, /* gather */
    FUN_DATAOP_SGL = 0x3, /* scatter-gather */
    FUN_DATAOP_IMM = 0x4, /* immediate data */
    FUN_DATAOP_RQBUF = 0x8, /* rq buffer */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_dataop_gl {
    pub subop: __u8,
    pub flags: __u8,
    pub sgl_off: __be16,
    pub sgl_len: __be32,
    pub sgl_data: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_dataop_imm {
    pub subop: __u8,
    pub flags: __u8,
    pub rsvd0: __be16,
    pub sgl_len: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_subop_sgl {
    pub subop: __u8,
    pub flags: __u8,
    pub nsgl: __u8,
    pub rsvd0: __u8,
    pub sgl_len: __be32,
    pub sgl_data: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_dataop_rqbuf {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub cid: __be16,
    pub bufoff: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_dataop_hdr {
    pub nsgl: __u8,
    pub flags: __u8,
    pub ngather: __u8,
    pub nscatter: __u8,
    pub total_len: __be32,
    pub imm: [fun_dataop_imm; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_port_inetaddr_event_type {
    FUN_PORT_INETADDR_ADD = 0x1,
    FUN_PORT_INETADDR_DEL = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_port_inetaddr_addr_family {
    FUN_PORT_INETADDR_IPV4 = 0x1,
    FUN_PORT_INETADDR_IPV6 = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union port_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_create_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub create: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_write_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub /: *mut *mut __be32 id; / portid,
    pub write48: [fun_admin_write48_req; ],
    pub write: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_read_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub /: *mut *mut __be32 id; / portid,
    pub read48: [fun_admin_read48_req; ],
    pub read: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_xcvr_read_req {
    pub subop: u8,
    pub rsvd0: u8,
    pub flags: __be16,
    pub id: __be32,
    pub bank: u8,
    pub page: u8,
    pub offset: u8,
    pub length: u8,
    pub dev_addr: u8,
    pub rsvd1: [u8; 3],
    pub xcvr_read: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_inetaddr_event_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub event_type: __u8,
    pub addr_family: __u8,
    pub id: __be32,
    pub addr: [__u8; ],
    pub inetaddr_event: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_rsp {
    pub common: fun_admin_rsp_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union port_rsp_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_create_rsp {
    pub subop: __u8,
    pub rsvd0: [__u8; 3],
    pub id: __be32,
    pub lport: __be16,
    pub rsvd1: [__u8; 6],
    pub create: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_write_rsp {
    pub subop: __u8,
    pub rsvd0: [__u8; 3],
    pub /: *mut *mut __be32 id; / portid,
    pub write48: [fun_admin_write48_rsp; ],
    pub write: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_read_rsp {
    pub subop: __u8,
    pub rsvd0: [__u8; 3],
    pub /: *mut *mut __be32 id; / portid,
    pub read48: [fun_admin_read48_rsp; ],
    pub read: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_inetaddr_event_rsp {
    pub subop: __u8,
    pub rsvd0: [__u8; 3],
    pub /: *mut *mut __be32 id; / portid,
    pub inetaddr_event: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_xcvr_read_rsp {
    pub common: fun_admin_rsp_common,
    pub subop: u8,
    pub rsvd0: [u8; 3],
    pub id: __be32,
    pub bank: u8,
    pub page: u8,
    pub offset: u8,
    pub length: u8,
    pub dev_addr: u8,
    pub rsvd1: [u8; 3],
    pub data: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_xcvr_type {
    FUN_XCVR_BASET = 0x0,
    FUN_XCVR_CU = 0x1,
    FUN_XCVR_SMF = 0x2,
    FUN_XCVR_MMF = 0x3,
    FUN_XCVR_AOC = 0x4,
    FUN_XCVR_SFPP = 0x10, /* SFP+ or later */
    FUN_XCVR_QSFPP = 0x11, /* QSFP+ or later */
    FUN_XCVR_QSFPDD = 0x12, /* QSFP-DD */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_port_notif {
    pub common: fun_admin_rsp_common,
    pub subop: __u8,
    pub rsvd0: __u8,
    pub id: __be16,
    pub /: *mut *mut __be32 speed; / in 10 Mbps units,
    pub link_state: __u8,
    pub missed_events: __u8,
    pub link_down_reason: __u8,
    pub xcvr_type: __u8,
    pub flow_ctrl: __u8,
    pub fec: __u8,
    pub active_lanes: __u8,
    pub rsvd1: __u8,
    pub advertising: __be64,
    pub lp_advertising: __be64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_eth_rss_const {
    FUN_ETH_RSS_MAX_KEY_SIZE = 0x28,
    FUN_ETH_RSS_MAX_INDIR_ENT = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_eth_hash_alg {
    FUN_ETH_RSS_ALG_INVALID = 0x0,
    FUN_ETH_RSS_ALG_TOEPLITZ = 0x1,
    FUN_ETH_RSS_ALG_CRC32 = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_rss_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union rss_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_rss_create_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub rsvd1: __be32,
    pub /: *mut *mut __be32 viid; / VI flow id,
    pub metadata: [__be64; 1],
    pub alg: __u8,
    pub keylen: __u8,
    pub indir_nent: __u8,
    pub rsvd2: __u8,
    pub key_off: __be16,
    pub indir_off: __be16,
    pub dataop: fun_dataop_hdr,
    pub create: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_vi_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union vi_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_vi_create_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub rsvd1: __be32,
    pub /: *mut *mut __be32 portid; / port flow id,
    pub create: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_eth_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union eth_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_eth_create_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub rsvd1: __be32,
    pub /: *mut *mut __be32 portid; / port flow id,
    pub create: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_admin_swu_subop {
    FUN_ADMIN_SWU_SUBOP_GET_VERSION = 0x20,
    FUN_ADMIN_SWU_SUBOP_UPGRADE = 0x21,
    FUN_ADMIN_SWU_SUBOP_UPGRADE_DATA = 0x22,
    FUN_ADMIN_SWU_SUBOP_GET_ALL_VERSIONS = 0x23,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union swu_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_create_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub create: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_upgrade_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub fourcc: __be32,
    pub rsvd1: __be32,
    pub /: *mut *mut __be64 image_size; / upgrade image length,
    pub upgrade: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_upgrade_data_req {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub /: *mut *mut __be32 offset; / offset of data in this command,
    pub /: *mut *mut __be32 size; / total size of data in this command,
    pub upgrade_data: },
    pub u: },
    pub /: *mut *mut fun_subop_sgl sgl[]; / in, out buffers through sgl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_rsp {
    pub common: fun_admin_rsp_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union swu_rsp_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_create_rsp {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub create: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_upgrade_rsp {
    pub subop: __u8,
    pub rsvd0: [__u8; 3],
    pub id: __be32,
    pub fourcc: __be32,
    pub status: __be32,
    pub progress: __be32,
    pub unused: __be32,
    pub upgrade: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_swu_upgrade_data_rsp {
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub offset: __be32,
    pub size: __be32,
    pub upgrade_data: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_ktls_version {
    FUN_KTLS_TLSV2 = 0x20,
    FUN_KTLS_TLSV3 = 0x30,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_ktls_cipher {
    FUN_KTLS_CIPHER_AES_GCM_128 = 0x33,
    FUN_KTLS_CIPHER_AES_GCM_256 = 0x34,
    FUN_KTLS_CIPHER_AES_CCM_128 = 0x35,
    FUN_KTLS_CIPHER_CHACHA20_POLY1305 = 0x36,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_ktls_modify_flags {
    FUN_KTLS_MODIFY_REMOVE = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_ktls_create_req {
    pub common: fun_admin_req_common,
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_ktls_create_rsp {
    pub common: fun_admin_rsp_common,
    pub subop: __u8,
    pub rsvd0: [__u8; 3],
    pub id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_ktls_modify_req {
    pub common: fun_admin_req_common,
    pub subop: __u8,
    pub rsvd0: __u8,
    pub flags: __be16,
    pub id: __be32,
    pub tlsid: __be64,
    pub tcp_seq: __be32,
    pub version: __u8,
    pub cipher: __u8,
    pub rsvd1: [__u8; 2],
    pub record_seq: [__u8; 8],
    pub key: [__u8; 32],
    pub iv: [__u8; 16],
    pub salt: [__u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_ktls_modify_rsp {
    pub common: fun_admin_rsp_common,
    pub subop: __u8,
    pub rsvd0: [__u8; 3],
    pub id: __be32,
    pub tlsid: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_req_common {
    pub op: __u8,
    pub len8: __u8,
    pub flags: __be16,
    pub suboff8: __u8,
    pub rsvd0: __u8,
    pub cid: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_rsp_common {
    pub op: __u8,
    pub len8: __u8,
    pub flags: __be16,
    pub suboff8: __u8,
    pub ret: __u8,
    pub cid: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_cqe_info {
    pub sqhd: __be16,
    pub sqid: __be16,
    pub cid: __be16,
    pub sf_p: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_eprq_def {
    FUN_EPRQ_PKT_ALIGN = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_eprq_rqbuf {
    pub bufaddr: __be64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_eth_op {
    FUN_ETH_OP_TX = 0x1,
    FUN_ETH_OP_RX = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_eth_offload {
    pub /: *mut *mut __be16 flags; / combination of above flags,
    pub /: *mut *mut __be16 mss; / TSO max seg size,
    pub /: *mut *mut __be16 tcp_doff_flags; / TCP data offset + flags 16b word,
    pub vlan: __be16,
    pub /: *mut *mut __be16 inner_l3_off; / Inner L3 header offset,
    pub /: *mut *mut __be16 inner_l4_off; / Inner L4 header offset,
    pub /: *mut *mut __be16 outer_l3_off; / Outer L3 header offset,
    pub /: *mut *mut __be16 outer_l4_off; / Outer L4 header offset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_eth_tls {
    pub tlsid: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_eth_tx_req {
    pub op: __u8,
    pub len8: __u8,
    pub flags: __be16,
    pub suboff8: __u8,
    pub repr_idn: __u8,
    pub encap_proto: __be16,
    pub offload: fun_eth_offload,
    pub dataop: fun_dataop_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_eth_rx_cv {
    pub il4_prot_to_l2_type: __be16,
}

pub const FUN_ETH_RX_CV_IL4_PROT_M: c_uint = 0x3;

pub const FUN_ETH_RX_CV_IL3_PROT_M: c_uint = 0x3;

pub const FUN_ETH_RX_CV_OL4_PROT_M: c_uint = 0x7;

pub const FUN_ETH_RX_CV_ENCAP_TYPE_M: c_uint = 0x3;

pub const FUN_ETH_RX_CV_OL3_PROT_M: c_uint = 0x3;

pub const FUN_ETH_RX_CV_VLAN_TYPE_M: c_uint = 0x1;

pub const FUN_ETH_RX_CV_L2_TYPE_M: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_rx_cv {
    FUN_RX_CV_NONE = 0x0,
    FUN_RX_CV_IP = 0x2,
    FUN_RX_CV_IP6 = 0x3,
    FUN_RX_CV_TCP = 0x2,
    FUN_RX_CV_UDP = 0x3,
    FUN_RX_CV_VXLAN = 0x2,
    FUN_RX_CV_MPLS = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_eth_cqe {
    pub op: __u8,
    pub len8: __u8,
    pub nsgl: __u8,
    pub repr_idn: __u8,
    pub pkt_len: __be32,
    pub timestamp: __be64,
    pub pkt_cv: __be16,
    pub rsvd0: __be16,
    pub hash: __be32,
    pub encap_proto: __be16,
    pub vlan: __be16,
    pub rsvd1: __be32,
    pub buf_offset: __be32,
    pub headroom: __be16,
    pub csum: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fun_admin_adi_attr {
    FUN_ADMIN_ADI_ATTR_MACADDR = 0x1,
    FUN_ADMIN_ADI_ATTR_VLAN = 0x2,
    FUN_ADMIN_ADI_ATTR_RATE = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_adi_param {
#[repr(C)]
#[derive(Copy, Clone)]
pub union adi_param {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_adi_mac {
    pub addr: __be64,
    pub mac: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_adi_vlan {
    pub rsvd: __be32,
    pub eth_type: __be16,
    pub tci: __be16,
    pub vlan: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_adi_rate {
    pub rsvd: __be32,
    pub tx_mbps: __be32,
    pub rate: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_adi_req {
    pub common: fun_admin_req_common,
#[repr(C)]
#[derive(Copy, Clone)]
pub union adi_req_subop {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_admin_adi_write_req {
    pub subop: __u8,
    pub attribute: __u8,
    pub rsvd: __be16,
    pub id: __be32,
    pub param: fun_adi_param,
    pub write: },
    pub u: },
}

