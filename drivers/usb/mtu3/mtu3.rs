//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/mtu3/mtu3.h
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
// mtu3.h - MediaTek USB3 DRD header
//
// Copyright (C) 2016 MediaTek Inc.
//
// Author: Chunfeng Yun <chunfeng.yun@mediatek.com>
//

pub const MTU3_U3_IP_SLOT_DEFAULT: c_int = 2;
pub const MTU3_U2_IP_SLOT_DEFAULT: c_int = 1;
//
// IP TRUNK version
// from 0x1003 version, USB3 Gen2 is supported, two changes affect driver:
// 1. MAXPKT and MULTI bits layout of TXCSR1 and RXCSR1 are adjusted,
// but not backward compatible
// 2. QMU extend buffer length supported
//
pub const MTU3_TRUNK_VERS_1003: c_uint = 0x1003;
//
// Normally the device works on HS or SS, to simplify fifo management,
// divide fifo into some 512B parts, use bitmap to manage it; And
// 128 bits size of bitmap is large enough, that means it can manage
// up to 64KB fifo size.
// NOTE: MTU3_EP_FIFO_UNIT should be power of two
//

pub const MTU3_FIFO_BIT_SIZE: c_int = 128;
pub const MTU3_U2_IP_EP0_FIFO_SIZE: c_int = 64;
//
// Maximum size of ep0 response buffer for ch9 requests,
// the SET_SEL request uses 6 so far, and GET_STATUS is 2
//
pub const EP0_RESPONSE_BUF: c_int = 6;
pub const BULK_CLKS_CNT: c_int = 6;
// device operated link and speed got from DEVICE_CONF register
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtu3_speed {
    MTU3_SPEED_INACTIVE = 0,
    MTU3_SPEED_FULL = 1,
    MTU3_SPEED_HIGH = 3,
    MTU3_SPEED_SUPER = 4,
    MTU3_SPEED_SUPER_PLUS = 5,
}

//
// enum mtu3_g_ep0_state - endpoint 0 states
// @MU3D_EP0_STATE_SETUP: waits for SETUP or received a SETUP
// without data stage.
// @MU3D_EP0_STATE_TX: IN data stage
// @MU3D_EP0_STATE_RX: OUT data stage
// @MU3D_EP0_STATE_TX_END: the last IN data is transferred, and
// waits for its completion interrupt
// @MU3D_EP0_STATE_STALL: ep0 is in stall status, will be auto-cleared
// after receives a SETUP.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtu3_g_ep0_state {
    MU3D_EP0_STATE_SETUP = 1,
    MU3D_EP0_STATE_TX,
    MU3D_EP0_STATE_RX,
    MU3D_EP0_STATE_TX_END,
    MU3D_EP0_STATE_STALL,
}

//
// enum mtu3_dr_force_mode - indicates host/OTG operating mode
// @MTU3_DR_FORCE_NONE: automatically switch host and peripheral mode
// by IDPIN signal.
// @MTU3_DR_FORCE_HOST: force to enter host mode and override OTG
// IDPIN signal.
// @MTU3_DR_FORCE_DEVICE: force to enter peripheral mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtu3_dr_force_mode {
    MTU3_DR_FORCE_NONE = 0,
    MTU3_DR_FORCE_HOST,
    MTU3_DR_FORCE_DEVICE,
}

//
// struct mtu3_fifo_info - HW FIFO description and management data
// @base: the base address of fifo
// @limit: the bitmap size in bits
// @bitmap: fifo bitmap in unit of @MTU3_EP_FIFO_UNIT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtu3_fifo_info {
    pub base: u32,
    pub limit: u32,
    pub MTU3_FIFO_BIT_SIZE): DECLARE_BITMAP(bitmap,,
}

//
// struct qmu_gpd - General Purpose Descriptor (GPD):
// The format of TX GPD is a little different from RX one.
// And the size of GPD is 16 bytes.
//
// @dw0_info:
// bit0: Hardware Own (HWO)
// bit1: Buffer Descriptor Present (BDP), always 0, BD is not supported
// bit2: Bypass (BPS), 1: HW skips this GPD if HWO = 1
// bit6: [EL] Zero Length Packet (ZLP), moved from @dw3_info[29]
// bit7: Interrupt On Completion (IOC)
// bit[31:16]: ([EL] bit[31:12]) allow data buffer length (RX ONLY),
// the buffer length of the data to receive
// bit[23:16]: ([EL] bit[31:24]) extension address (TX ONLY),
// lower 4 bits are extension bits of @buffer,
// upper 4 bits are extension bits of @next_gpd
// @next_gpd: Physical address of the next GPD
// @buffer: Physical address of the data buffer
// @dw3_info:
// bit[15:0]: ([EL] bit[19:0]) data buffer length,
// (TX): the buffer length of the data to transmit
// (RX): The total length of data received
// bit[23:16]: ([EL] bit[31:24]) extension address (RX ONLY),
// lower 4 bits are extension bits of @buffer,
// upper 4 bits are extension bits of @next_gpd
// bit29: ([EL] abandoned) Zero Length Packet (ZLP) (TX ONLY)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmu_gpd {
    pub dw0_info: __le32,
    pub next_gpd: __le32,
    pub buffer: __le32,
    pub dw3_info: __le32,
    pub __packed: },
//
// struct mtu3_gpd_ring - GPD ring descriptor
// @dma: physical base address of GPD segment
// @start: virtual base address of GPD segment
// @end: the last GPD element
// @enqueue: the first empty GPD to use
// @dequeue: the first completed GPD serviced by ISR
//
// NOTE: the size of GPD ring should be >= 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtu3_gpd_ring {
    pub dma: dma_addr_t,
    pub start: *mut qmu_gpd,
    pub end: *mut qmu_gpd,
    pub enqueue: *mut qmu_gpd,
    pub dequeue: *mut qmu_gpd,
}

//
// struct otg_switch_mtk - OTG/dual-role switch management
// @vbus: vbus 5V used by host mode
// @edev: external connector used to detect vbus and iddig changes
// @id_nb : notifier for iddig(idpin) detection
// @dr_work : work for drd mode switch, used to avoid sleep in atomic context
// @desired_role : role desired to switch
// @default_role : default mode while usb role is USB_ROLE_NONE
// @role_sw : use USB Role Switch to support dual-role switch, can't use
// extcon at the same time, and extcon is deprecated.
// @role_sw_used : true when the USB Role Switch is used.
// @is_u3_drd: whether port0 supports usb3.0 dual-role device or not
// @manual_drd_enabled: it's true when supports dual-role device by debugfs
// to switch host/device modes depending on user input.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otg_switch_mtk {
    pub vbus: *mut regulator,
    pub edev: *mut extcon_dev,
    pub id_nb: notifier_block,
    pub dr_work: work_struct,
    pub desired_role: usb_role,
    pub default_role: usb_role,
    pub role_sw: *mut usb_role_switch,
    pub role_sw_used: bool,
    pub is_u3_drd: bool,
    pub manual_drd_enabled: bool,
}

//
// struct ssusb_mtk - SuperSpeed USB descriptor (MTK)
// @mac_base: register base address of device MAC, exclude xHCI's
// @ippc_base: register base address of IP Power and Clock interface (IPPC)
// @vusb33: usb3.3V shared by device/host IP
// @dr_mode: works in which mode:
// host only, device only or dual-role mode
// @u2_ports: number of usb2.0 host ports
// @u3_ports: number of usb3.0 host ports
// @u2p_dis_msk: mask of disabling usb2 ports, e.g. bit0==1 to
// disable u2port0, bit1==1 to disable u2port1,... etc,
// but when use dual-role mode, can't disable u2port0
// @u3p_dis_msk: mask of disabling usb3 ports, for example, bit0==1 to
// disable u3port0, bit1==1 to disable u3port1,... etc
// @dbgfs_root: only used when supports manual dual-role switch via debugfs
// @uwk_en: it's true when supports remote wakeup in host mode
// @uwk: syscon including usb wakeup glue layer between SSUSB IP and SPM
// @uwk_reg_base: the base address of the wakeup glue layer in @uwk
// @uwk_vers: the version of the wakeup glue layer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssusb_mtk {
    pub dev: *mut device,
    pub u3d: *mut mtu3,
    pub mac_base: *mut void __iomem,
    pub ippc_base: *mut void __iomem,
    pub phys: *mut phy,
    pub num_phys: c_int,
    pub wakeup_irq: c_int,
// common power & clock
    pub vusb33: *mut regulator,
    pub clks: [clk_bulk_data; BULK_CLKS_CNT],
// otg
    pub otg_switch: otg_switch_mtk,
    pub dr_mode: usb_dr_mode,
    pub is_host: bool,
    pub u2_ports: c_int,
    pub u3_ports: c_int,
    pub u2p_dis_msk: c_int,
    pub u3p_dis_msk: c_int,
    pub dbgfs_root: *mut dentry,
// usb wakeup for host mode
    pub uwk_en: bool,
    pub uwk: *mut regmap,
    pub uwk_reg_base: u32,
    pub uwk_vers: u32,
}

//
// struct mtu3_ep - common mtu3 endpoint description
// @fifo_size: it is (@slot + 1) * @fifo_seg_size
// @fifo_seg_size: it is roundup_pow_of_two(@maxp)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtu3_ep {
    pub ep: usb_ep,
    pub name: [c_char; 12],
    pub mtu: *mut mtu3,
    pub epnum: u8,
    pub type: u8,
    pub is_in: u8,
    pub maxp: u16,
    pub slot: c_int,
    pub fifo_size: u32,
    pub fifo_addr: u32,
    pub fifo_seg_size: u32,
    pub fifo: *mut mtu3_fifo_info,
    pub req_list: list_head,
    pub gpd_ring: mtu3_gpd_ring,
    pub comp_desc: *const usb_ss_ep_comp_descriptor,
    pub desc: *const usb_endpoint_descriptor,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtu3_request {
    pub request: usb_request,
    pub list: list_head,
    pub mep: *mut mtu3_ep,
    pub mtu: *mut mtu3,
    pub gpd: *mut qmu_gpd,
    pub epnum: c_int,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: dev) -> return;
}
//
// struct mtu3 - device driver instance data.
// @slot: MTU3_U2_IP_SLOT_DEFAULT for U2 IP only,
// MTU3_U3_IP_SLOT_DEFAULT for U3 IP
// @may_wakeup: means device's remote wakeup is enabled
// @is_self_powered: is reported in device status and the config descriptor
// @delayed_status: true when function drivers ask for delayed status
// @gen2cp: compatible with USB3 Gen2 IP
// @ep0_req: dummy request used while handling standard USB requests
// for GET_STATUS and SET_SEL
// @setup_buf: ep0 response buffer for GET_STATUS and SET_SEL requests
// @u3_capable: is capable of supporting USB3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtu3 {
    pub lock: spinlock_t,
    pub ssusb: *mut ssusb_mtk,
    pub dev: *mut device,
    pub mac_base: *mut void __iomem,
    pub ippc_base: *mut void __iomem,
    pub irq: c_int,
    pub tx_fifo: mtu3_fifo_info,
    pub rx_fifo: mtu3_fifo_info,
    pub ep_array: *mut mtu3_ep,
    pub in_eps: *mut mtu3_ep,
    pub out_eps: *mut mtu3_ep,
    pub ep0: *mut mtu3_ep,
    pub num_eps: c_int,
    pub slot: c_int,
    pub active_ep: c_int,
    pub qmu_gpd_pool: *mut dma_pool,
    pub ep0_state: mtu3_g_ep0_state,
    pub /: *mut *mut usb_gadget g; / the gadget,
    pub gadget_driver: *mut usb_gadget_driver,
    pub ep0_req: mtu3_request,
    pub setup_buf: [u8; EP0_RESPONSE_BUF],
    pub max_speed: usb_device_speed,
    pub speed: usb_device_speed,
    pub is_active:1: unsigned,
    pub may_wakeup:1: unsigned,
    pub is_self_powered:1: unsigned,
    pub test_mode:1: unsigned,
    pub softconnect:1: unsigned,
    pub u1_enable:1: unsigned,
    pub u2_enable:1: unsigned,
    pub u3_capable:1: unsigned,
    pub delayed_status:1: unsigned,
    pub gen2cp:1: unsigned,
    pub connected:1: unsigned,
    pub async_callbacks:1: unsigned,
    pub separate_fifo:1: unsigned,
    pub address: u8,
    pub test_mode_nr: u8,
    pub hw_version: u32,
}

extern "C" {
    pub fn container_of(_arg: g, mtu3: struct, _arg: g) -> return;
}
extern "C" {
    pub fn readl(offset: base +) -> return;
}
extern "C" {
    pub fn ssusb_check_clocks(ssusb: *mut ssusb_mtk, ex_clks: u32) -> c_int;
}
extern "C" {
    pub fn mtu3_free_request(ep: *mut usb_ep, req: *mut usb_request);
}
extern "C" {
    pub fn mtu3_deconfig_ep(mtu: *mut mtu3, mep: *mut mtu3_ep);
}
extern "C" {
    pub fn mtu3_ep_stall_set(mep: *mut mtu3_ep, set: bool);
}
extern "C" {
    pub fn mtu3_start(mtu: *mut mtu3);
}
extern "C" {
    pub fn mtu3_stop(mtu: *mut mtu3);
}
extern "C" {
    pub fn mtu3_dev_on_off(mtu: *mut mtu3, is_on: c_int);
}
extern "C" {
    pub fn mtu3_gadget_setup(mtu: *mut mtu3) -> c_int;
}
extern "C" {
    pub fn mtu3_gadget_cleanup(mtu: *mut mtu3);
}
extern "C" {
    pub fn mtu3_gadget_reset(mtu: *mut mtu3);
}
extern "C" {
    pub fn mtu3_gadget_suspend(mtu: *mut mtu3);
}
extern "C" {
    pub fn mtu3_gadget_resume(mtu: *mut mtu3);
}
extern "C" {
    pub fn mtu3_gadget_disconnect(mtu: *mut mtu3);
}
extern "C" {
    pub fn mtu3_ep0_isr(mtu: *mut mtu3) -> irqreturn_t;
}
