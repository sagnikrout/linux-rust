//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-mtk.h
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
// Copyright (c) 2015 MediaTek Inc.
// Author:
// Zhigang.Wei <zhigang.wei@mediatek.com>
// Chunfeng.Yun <chunfeng.yun@mediatek.com>
//

pub const BULK_CLKS_NUM: c_int = 6;
pub const BULK_VREGS_NUM: c_int = 2;
// support at most 64 ep, use 32 size hash table
pub const SCH_EP_HASH_BITS: c_int = 5;
//
// To simplify scheduler algorithm, set a upper limit for ESIT,
// if a synchromous ep's ESIT is larger than @XHCI_MTK_MAX_ESIT,
// round down to the limit value, that means allocating more
// bandwidth to it.
//

pub const UFRAMES_PER_FRAME: c_int = 8;

//
// struct mu3h_sch_tt - TT scheduling data
// @fs_bus_bw_out: save bandwidth used by FS/LS OUT eps in each uframes
// @fs_bus_bw_in: save bandwidth used by FS/LS IN eps in each uframes
// @ls_bus_bw: save bandwidth used by LS eps in each uframes
// @fs_frame_bw: save bandwidth used by FS/LS eps in each FS frames
// @in_ss_cnt: the count of Start-Split for IN eps
// @ep_list: Endpoints using this TT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mu3h_sch_tt {
    pub fs_bus_bw_out: [u16; XHCI_MTK_MAX_ESIT],
    pub fs_bus_bw_in: [u16; XHCI_MTK_MAX_ESIT],
    pub ls_bus_bw: [u8; XHCI_MTK_MAX_ESIT],
    pub fs_frame_bw: [u16; XHCI_MTK_FRAMES_CNT],
    pub in_ss_cnt: [u8; XHCI_MTK_MAX_ESIT],
    pub ep_list: list_head,
}

//
// struct mu3h_sch_bw_info - schedule information for bandwidth domain
//
// @bus_bw: array to keep track of bandwidth already used at each uframes
//
// treat a HS root port as a bandwidth domain, but treat a SS root port as
// two bandwidth domains, one for IN eps and another for OUT eps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mu3h_sch_bw_info {
    pub bus_bw: [u32; XHCI_MTK_MAX_ESIT],
}

//
// struct mu3h_sch_ep_info - schedule information for endpoint
//
// @esit: unit is 125us, equal to 2 << Interval field in ep-context
// @num_esit: number of @esit in a period
// @num_budget_microframes: number of continuous uframes
// (@repeat==1) scheduled within the interval
// @hentry: hash table entry
// @endpoint: linked into bandwidth domain which it belongs to
// @tt_endpoint: linked into mu3h_sch_tt's list which it belongs to
// @bw_info: bandwidth domain which this endpoint belongs
// @sch_tt: mu3h_sch_tt linked into
// @ep_type: endpoint type
// @maxpkt: max packet size of endpoint
// @ep: address of usb_host_endpoint struct
// @speed: usb device speed
// @allocated: the bandwidth is aready allocated from bus_bw
// @offset: which uframe of the interval that transfer should be
// scheduled first time within the interval
// @repeat: the time gap between two uframes that transfers are
// scheduled within a interval. in the simple algorithm, only
// assign 0 or 1 to it; 0 means using only one uframe in a
// interval, and 1 means using @num_budget_microframes
// continuous uframes
// @pkts: number of packets to be transferred in the scheduled uframes
// @cs_count: number of CS that host will trigger
// @burst_mode: burst mode for scheduling. 0: normal burst mode,
// distribute the bMaxBurst+1 packets for a single burst
// according to @pkts and @repeat, repeate the burst multiple
// times; 1: distribute the (bMaxBurst+1)*(Mult+1) packets
// according to @pkts and @repeat. normal mode is used by
// default
// @bw_budget_table: table to record bandwidth budget per microframe
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mu3h_sch_ep_info {
    pub esit: u32,
    pub num_esit: u32,
    pub num_budget_microframes: u32,
    pub endpoint: list_head,
    pub hentry: hlist_node,
    pub tt_endpoint: list_head,
    pub bw_info: *mut mu3h_sch_bw_info,
    pub sch_tt: *mut mu3h_sch_tt,
    pub ep_type: u32,
    pub maxpkt: u32,
    pub ep: *mut usb_host_endpoint,
    pub speed: usb_device_speed,
    pub allocated: bool,
//
// mtk xHCI scheduling information put into reserved DWs
// in ep context
//
    pub offset: u32,
    pub repeat: u32,
    pub pkts: u32,
    pub cs_count: u32,
    pub burst_mode: u32,
    pub bw_budget_table: [u32; ],
}

pub const MU3C_U3_PORT_MAX: c_int = 4;
pub const MU3C_U2_PORT_MAX: c_int = 5;
//
// struct mu3c_ippc_regs - MTK ssusb ip port control registers
// @ip_pw_ctr0~3: ip power and clock control registers
// @ip_pw_sts1~2: ip power and clock status registers
// @ip_xhci_cap: ip xHCI capability register
// @u3_ctrl_p[x]: ip usb3 port x control register, only low 4bytes are used
// @u2_ctrl_p[x]: ip usb2 port x control register, only low 4bytes are used
// @u2_phy_pll: usb2 phy pll control register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mu3c_ippc_regs {
    pub ip_pw_ctr0: __le32,
    pub ip_pw_ctr1: __le32,
    pub ip_pw_ctr2: __le32,
    pub ip_pw_ctr3: __le32,
    pub ip_pw_sts1: __le32,
    pub ip_pw_sts2: __le32,
    pub reserved0: [__le32; 3],
    pub ip_xhci_cap: __le32,
    pub reserved1: [__le32; 2],
    pub u3_ctrl_p: [__le64; MU3C_U3_PORT_MAX],
    pub u2_ctrl_p: [__le64; MU3C_U2_PORT_MAX],
    pub reserved2: __le32,
    pub u2_phy_pll: __le32,
    pub /: *mut *mut __le32 reserved3[33]; / 0x80 ~ 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_hcd_mtk {
    pub dev: *mut device,
    pub hcd: *mut usb_hcd,
    pub sch_array: *mut mu3h_sch_bw_info,
    pub bw_ep_chk_list: list_head,
    pub SCH_EP_HASH_BITS): DECLARE_HASHTABLE(sch_ep_hash,,
    pub ippc_regs: *mut mu3c_ippc_regs __iomem,
    pub num_u2_ports: c_int,
    pub num_u3_ports: c_int,
    pub u2p_dis_msk: c_int,
    pub u3p_dis_msk: c_int,
    pub clks: [clk_bulk_data; BULK_CLKS_NUM],
    pub supplies: [regulator_bulk_data; BULK_VREGS_NUM],
    pub has_ippc:1: c_uint,
    pub lpm_support:1: c_uint,
    pub u2_lpm_disable:1: c_uint,
// usb remote wakeup
    pub uwk_en:1: c_uint,
    pub uwk: *mut regmap,
    pub uwk_reg_base: u32,
    pub uwk_vers: u32,
// quirk
    pub rxfifo_depth: u32,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: hcd->self.controller) -> return;
}
extern "C" {
    pub fn xhci_mtk_sch_init(mtk: *mut xhci_hcd_mtk) -> c_int;
}
extern "C" {
    pub fn xhci_mtk_sch_exit(mtk: *mut xhci_hcd_mtk);
}
extern "C" {
    pub fn xhci_mtk_check_bandwidth(hcd: *mut usb_hcd, udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn xhci_mtk_reset_bandwidth(hcd: *mut usb_hcd, udev: *mut usb_device);
}
