//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/powernv/vas.h
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
// Copyright 2016-17 IBM Corp.
//

//
// Overview of Virtual Accelerator Switchboard (VAS).
//
// VAS is a hardware "switchboard" that allows senders and receivers to
// exchange messages with _minimal_ kernel involvment. The receivers are
// typically NX coprocessor engines that perform compression or encryption
// in hardware, but receivers can also be other software threads.
//
// Senders are user/kernel threads that submit compression/encryption or
// other requests to the receivers. Senders must format their messages as
// Coprocessor Request Blocks (CRB)s and submit them using the "copy" and
// "paste" instructions which were introduced in Power9.
//
// A Power node can have (upto?) 8 Power chips. There is one instance of
// VAS in each Power9 chip. Each instance of VAS has 64K windows or ports,
// Senders and receivers must each connect to a separate window before they
// can exchange messages through the switchboard.
//
// Each window is described by two types of window contexts:
//
// Hypervisor Window Context (HVWC) of size VAS_HVWC_SIZE bytes
//
// OS/User Window Context (UWC) of size VAS_UWC_SIZE bytes.
//
// A window context can be viewed as a set of 64-bit registers. The settings
// in these registers configure/control/determine the behavior of the VAS
// hardware when messages are sent/received through the window. The registers
// in the HVWC are configured by the kernel while the registers in the UWC can
// be configured by the kernel or by the user space application that is using
// the window.
//
// The HVWCs for all windows on a specific instance of VAS are in a contiguous
// range of hardware addresses or Base address region (BAR) referred to as the
// HVWC BAR for the instance. Similarly the UWCs for all windows on an instance
// are referred to as the UWC BAR for the instance.
//
// The two BARs for each instance are defined Power9 MMIO Ranges spreadsheet
// and available to the kernel in the VAS node's "reg" property in the device
// tree:
//
// /proc/device-tree/vasm@.../reg
//
// (see vas_probe() for details on the reg property).
//
// The kernel maps the HVWC and UWC BAR regions into the kernel address
// space (hvwc_map and uwc_map). The kernel can then access the window
// contexts of a specific window using:
//
// hvwc = hvwc_map + winid * VAS_HVWC_SIZE.
// uwc = uwc_map + winid * VAS_UWC_SIZE.
//
// where winid is the window index (0..64K).
//
// As mentioned, a window context is used to "configure" a window. Besides
// this configuration address, each _send_ window also has a unique hardware
// "paste" address that is used to submit requests/CRBs (see vas_paste_crb()).
//
// The hardware paste address for a window is computed using the "paste
// base address" and "paste win id shift" reg properties in the VAS device
// tree node using:
//
// paste_addr = paste_base + ((winid << paste_win_id_shift))
//
// (again, see vas_probe() for ->paste_base_addr and ->paste_win_id_shift).
//
// The kernel maps this hardware address into the sender's address space
// after which they can use the 'paste' instruction (new in Power9) to
// send a message (submit a request aka CRB) to the coprocessor.
//
// NOTE: In the initial version, senders can only in-kernel drivers/threads.
// Support for user space threads will be added in follow-on patches.
//
// TODO: Do we need to map the UWC into user address space so they can return
// credits? Its NA for NX but may be needed for other receive windows.
//

//
// Hypervisor and OS/USer Window Context sizes
//
pub const VAS_HVWC_SIZE: c_int = 512;

//
// Initial per-process credits.
// Max send window credits:    4K-1 (12-bits in VAS_TX_WCRED)
//
// TODO: Needs tuning for per-process credits
//

//
// VAS Window Context Register Offsets and bitmasks.
// See Section 3.1.4 of VAS Work book
//
pub const VAS_LPID_OFFSET: c_uint = 0x010;

pub const VAS_PID_OFFSET: c_uint = 0x018;

pub const VAS_XLATE_MSR_OFFSET: c_uint = 0x020;

pub const VAS_XLATE_LPCR_OFFSET: c_uint = 0x028;

pub const VAS_XLATE_CTL_OFFSET: c_uint = 0x030;

pub const VAS_AMR_OFFSET: c_uint = 0x040;

pub const VAS_SEIDR_OFFSET: c_uint = 0x048;

pub const VAS_FAULT_TX_WIN_OFFSET: c_uint = 0x050;

pub const VAS_OSU_INTR_SRC_RA_OFFSET: c_uint = 0x060;

pub const VAS_HV_INTR_SRC_RA_OFFSET: c_uint = 0x070;

pub const VAS_PSWID_OFFSET: c_uint = 0x078;

pub const VAS_SPARE1_OFFSET: c_uint = 0x080;
pub const VAS_SPARE2_OFFSET: c_uint = 0x088;
pub const VAS_SPARE3_OFFSET: c_uint = 0x090;
pub const VAS_SPARE4_OFFSET: c_uint = 0x130;
pub const VAS_SPARE5_OFFSET: c_uint = 0x160;
pub const VAS_SPARE6_OFFSET: c_uint = 0x188;
pub const VAS_LFIFO_BAR_OFFSET: c_uint = 0x0A0;

pub const VAS_LDATA_STAMP_CTL_OFFSET: c_uint = 0x0A8;

pub const VAS_LDMA_CACHE_CTL_OFFSET: c_uint = 0x0B0;

pub const VAS_LRFIFO_PUSH_OFFSET: c_uint = 0x0B8;

pub const VAS_CURR_MSG_COUNT_OFFSET: c_uint = 0x0C0;

pub const VAS_LNOTIFY_AFTER_COUNT_OFFSET: c_uint = 0x0C8;

pub const VAS_LRX_WCRED_OFFSET: c_uint = 0x0E0;

pub const VAS_LRX_WCRED_ADDER_OFFSET: c_uint = 0x190;

pub const VAS_TX_WCRED_OFFSET: c_uint = 0x0F0;

pub const VAS_TX_WCRED_ADDER_OFFSET: c_uint = 0x1A0;

pub const VAS_LFIFO_SIZE_OFFSET: c_uint = 0x100;

pub const VAS_WINCTL_OFFSET: c_uint = 0x108;

pub const VAS_WIN_STATUS_OFFSET: c_uint = 0x110;

pub const VAS_WIN_CTX_CACHING_CTL_OFFSET: c_uint = 0x118;

pub const VAS_TX_RSVD_BUF_COUNT_OFFSET: c_uint = 0x120;

pub const VAS_LRFIFO_WIN_PTR_OFFSET: c_uint = 0x128;

//
// Local Notification Control Register controls what happens in _response_
// to a paste command and hence applies only to receive windows.
//
pub const VAS_LNOTIFY_CTL_OFFSET: c_uint = 0x138;

pub const VAS_LNOTIFY_PID_OFFSET: c_uint = 0x140;

pub const VAS_LNOTIFY_LPID_OFFSET: c_uint = 0x148;

pub const VAS_LNOTIFY_TID_OFFSET: c_uint = 0x150;

pub const VAS_LNOTIFY_SCOPE_OFFSET: c_uint = 0x158;

pub const VAS_NX_UTIL_OFFSET: c_uint = 0x1B0;

// SE: Side effects
pub const VAS_NX_UTIL_SE_OFFSET: c_uint = 0x1B8;

pub const VAS_NX_UTIL_ADDER_OFFSET: c_uint = 0x180;

//
// VREG(x):
// Expand a register's short name (eg: LPID) into two parameters:
// - the register's short name in string form ("LPID"), and
// - the name of the macro (eg: VAS_LPID_OFFSET), defining the
// register's offset in the window context
//

//
// Local Notify Scope Control Register. (Receive windows only).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vas_notify_scope {
    VAS_SCOPE_LOCAL,
    VAS_SCOPE_GROUP,
    VAS_SCOPE_VECTORED_GROUP,
    VAS_SCOPE_UNUSED,
}

//
// Local DMA Cache Control Register (Receive windows only).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vas_dma_type {
    VAS_DMA_TYPE_INJECT,
    VAS_DMA_TYPE_WRITE,
}

//
// Local Notify Scope Control Register. (Receive windows only).
// Not applicable to NX receive windows.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vas_notify_after_count {
    VAS_NOTIFY_AFTER_256 = 0,
    VAS_NOTIFY_NONE,
    VAS_NOTIFY_AFTER_2
}

//
// NX can generate an interrupt for multiple faults and expects kernel
// to process all of them. So read all valid CRB entries until find the
// invalid one. So use pswid which is pasted by NX and ccw[0] (reserved
// bit in BE) to check valid CRB. CCW[0] will not be touched by user
// space. Application gets CRB formt error if it updates this bit.
//
// Invalidate FIFO during allocation and process all entries from last
// successful read until finds invalid pswid and ccw[0] values.
// After reading each CRB entry from fault FIFO, the kernel invalidate
// it by updating pswid with FIFO_INVALID_ENTRY and CCW[0] with
// CCW0_INVALID.
//
pub const FIFO_INVALID_ENTRY: c_uint = 0xffffffff;
pub const CCW0_INVALID: c_int = 1;
//
// One per instance of VAS. Each instance will have a separate set of
// receive windows, one per coprocessor type.
//
// See also function header of set_vinst_win() for details on ->windows[]
// and ->rxwin[] tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_instance {
    pub vas_id: c_int,
    pub ida: ida,
    pub node: list_head,
    pub pdev: *mut platform_device,
    pub hvwc_bar_start: u64,
    pub uwc_bar_start: u64,
    pub paste_base_addr: u64,
    pub paste_win_id_shift: u64,
    pub irq_port: u64,
    pub virq: c_int,
    pub fault_crbs: c_int,
    pub fault_fifo_size: c_int,
    pub /: *mut *mut int fifo_in_progress; / To wake up thread or return IRQ_HANDLED,
    pub /: *mut *mut spinlock_t fault_lock; / Protects fifo_in_progress update,
    pub fault_fifo: *mut c_void,
    pub /: *mut *mut *mut pnv_vas_window fault_win; / Fault window,
    pub mutex: mutex,
    pub rxwin: [*mut pnv_vas_window; VAS_COP_TYPE_MAX],
    pub windows: [*mut pnv_vas_window; VAS_WINDOWS_PER_CHIP],
    pub name: *mut c_char,
    pub dbgname: *mut c_char,
    pub dbgdir: *mut dentry,
}

//
// In-kernel state a VAS window on PowerNV. One per window.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnv_vas_window {
    pub vas_win: vas_window,
// Fields common to send and receive windows
    pub vinst: *mut vas_instance,
    pub /: *mut *mut bool tx_win; / True if send window,
    pub /: *mut *mut bool nx_win; / True if NX window,
    pub /: *mut *mut bool user_win; / True if user space window,
    pub /: *mut *mut *mut void hvwc_map; / HV window context,
    pub /: *mut *mut *mut void uwc_map; / OS/User window context,
// Fields applicable only to send windows
    pub paste_kaddr: *mut c_void,
    pub paste_addr_name: *mut c_char,
    pub rxwin: *mut pnv_vas_window,
// Fields applicable only to receive windows
    pub num_txwins: core::sync::atomic::AtomicI32,
}

//
// Container for the hardware state of a window. One per-window.
//
// A VAS Window context is a 512-byte area in the hardware that contains
// a set of 64-bit registers. Individual bit-fields in these registers
// determine the configuration/operation of the hardware. struct vas_winctx
// is a container for the register fields in the window context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_winctx {
    pub rx_fifo: u64,
    pub rx_fifo_size: c_int,
    pub wcreds_max: c_int,
    pub rsvd_txbuf_count: c_int,
    pub user_win: bool,
    pub nx_win: bool,
    pub fault_win: bool,
    pub rsvd_txbuf_enable: bool,
    pub pin_win: bool,
    pub rej_no_credit: bool,
    pub tx_wcred_mode: bool,
    pub rx_wcred_mode: bool,
    pub tx_word_mode: bool,
    pub rx_word_mode: bool,
    pub data_stamp: bool,
    pub xtra_write: bool,
    pub notify_disable: bool,
    pub intr_disable: bool,
    pub fifo_disable: bool,
    pub notify_early: bool,
    pub notify_os_intr_reg: bool,
    pub lpid: c_int,
    pub /: *mut *mut int pidr; / value from SPRN_PID, not linux pid,
    pub lnotify_lpid: c_int,
    pub lnotify_pid: c_int,
    pub lnotify_tid: c_int,
    pub pswid: u32,
    pub rx_win_id: c_int,
    pub fault_win_id: c_int,
    pub tc_mode: c_int,
    pub irq_port: u64,
    pub dma_type: vas_dma_type,
    pub min_scope: vas_notify_scope,
    pub max_scope: vas_notify_scope,
    pub notify_after_count: vas_notify_after_count,
}

extern "C" {
    pub fn vas_init_dbgdir();
}
extern "C" {
    pub fn vas_instance_init_dbgdir(vinst: *mut vas_instance);
}
extern "C" {
    pub fn vas_window_init_dbgdir(win: *mut pnv_vas_window);
}
extern "C" {
    pub fn vas_window_free_dbgdir(win: *mut pnv_vas_window);
}
extern "C" {
    pub fn vas_setup_fault_window(vinst: *mut vas_instance) -> c_int;
}
extern "C" {
    pub fn vas_fault_thread_fn(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn vas_fault_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn vas_return_credit(window: *mut pnv_vas_window, tx: bool);
}
extern "C" {
    pub fn pid_vnr(_arg: window->task_ref.pid) -> return;
}
extern "C" {
    pub fn in_be64(_arg: win->hvwc_map+reg) -> return;
}
//
// Encode/decode the Partition Send Window ID (PSWID) for a window in
// a way that we can uniquely identify any window in the system. i.e.
// we should be able to locate the 'struct vas_window' given the PSWID.
//
// Bits	Usage
// 0:7	VAS id (8 bits)
// 8:15	Unused, 0 (3 bits)
// 16:31	Window id (16 bits)
//
// vasid = pswid >> (31 - 7) & 0xFF;
// winid = pswid & 0xFFFF;
