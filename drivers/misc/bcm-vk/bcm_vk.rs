//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/bcm-vk/bcm_vk.h
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
// Copyright 2018-2020 Broadcom.
//

//
// Load Image is completed in two stages:
//
// 1) When the VK device boot-up, M7 CPU runs and executes the BootROM.
// The Secure Boot Loader (SBL) as part of the BootROM will run
// to open up ITCM for host to push BOOT1 image.
// SBL will authenticate the image before jumping to BOOT1 image.
//
// 2) Because BOOT1 image is a secured image, we also called it the
// Secure Boot Image (SBI). At second stage, SBI will initialize DDR
// and wait for host to push BOOT2 image to DDR.
// SBI will authenticate the image before jumping to BOOT2 image.
//
// Location of registers of interest in BAR0
// Request register for Secure Boot Loader (SBL) download
pub const BAR_CODEPUSH_SBL: c_uint = 0x400;
// Start of ITCM
pub const CODEPUSH_BOOT1_ENTRY: c_uint = 0x00400000;
pub const CODEPUSH_MASK: c_uint = 0xfffff000;

// Boot Status register
pub const BAR_BOOT_STATUS: c_uint = 0x404;

// Firmware loader progress status definitions

// Boot1/2 is running in standalone mode

// definitions for boot status register

pub const BOOT_ERR_SHIFT: c_int = 4;

pub const BOOT_PROG_MASK: c_uint = 0xf;
pub const BROM_STATUS_NOT_RUN: c_uint = 0x2;

pub const BROM_STATUS_COMPLETE: c_uint = 0x6;

pub const BOOT1_STATUS_COMPLETE: c_uint = 0x6;

pub const BOOT2_STATUS_COMPLETE: c_uint = 0x6;

// Boot request for Secure Boot Image (SBI)
pub const BAR_CODEPUSH_SBI: c_uint = 0x408;
// 64M mapped to BAR2
pub const CODEPUSH_BOOT2_ENTRY: c_uint = 0x60000000;
pub const BAR_CARD_STATUS: c_uint = 0x410;
// CARD_STATUS definitions

pub const BAR_BOOT1_STDALONE_PROGRESS: c_uint = 0x420;

pub const BAR_METADATA_VERSION: c_uint = 0x440;
pub const BAR_OS_UPTIME: c_uint = 0x444;
pub const BAR_CHIP_ID: c_uint = 0x448;

pub const BAR_CARD_TEMPERATURE: c_uint = 0x45c;
// defines for all temperature sensor
pub const BCM_VK_TEMP_FIELD_MASK: c_uint = 0xff;
pub const BCM_VK_CPU_TEMP_SHIFT: c_int = 0;
pub const BCM_VK_DDR0_TEMP_SHIFT: c_int = 8;
pub const BCM_VK_DDR1_TEMP_SHIFT: c_int = 16;
pub const BAR_CARD_VOLTAGE: c_uint = 0x460;
// defines for voltage rail conversion
pub const BCM_VK_VOLT_RAIL_MASK: c_uint = 0xffff;
pub const BCM_VK_3P3_VOLT_REG_SHIFT: c_int = 16;
pub const BAR_CARD_ERR_LOG: c_uint = 0x464;
// Error log register bit definition - register for error alerts

// warnings

// Alert bit definitions detectd on host

pub const BAR_CARD_ERR_MEM: c_uint = 0x468;
// defines for mem err, all fields have same width
pub const BCM_VK_MEM_ERR_FIELD_MASK: c_uint = 0xff;
pub const BCM_VK_ECC_MEM_ERR_SHIFT: c_int = 0;
pub const BCM_VK_UECC_MEM_ERR_SHIFT: c_int = 8;
// threshold of event occurrence and logs start to come out
pub const BCM_VK_ECC_THRESHOLD: c_int = 10;
pub const BCM_VK_UECC_THRESHOLD: c_int = 1;
pub const BAR_CARD_PWR_AND_THRE: c_uint = 0x46c;
// defines for power and temp threshold, all fields have same width
pub const BCM_VK_PWR_AND_THRE_FIELD_MASK: c_uint = 0xff;
pub const BCM_VK_LOW_TEMP_THRE_SHIFT: c_int = 0;
pub const BCM_VK_HIGH_TEMP_THRE_SHIFT: c_int = 8;
pub const BCM_VK_PWR_STATE_SHIFT: c_int = 16;
pub const BAR_CARD_STATIC_INFO: c_uint = 0x470;
pub const BAR_INTF_VER: c_uint = 0x47c;
pub const BAR_INTF_VER_MAJOR_SHIFT: c_int = 16;
pub const BAR_INTF_VER_MASK: c_uint = 0xffff;
//
// major and minor semantic version numbers supported
// Please update as required on interface changes
//
pub const SEMANTIC_MAJOR: c_int = 1;
pub const SEMANTIC_MINOR: c_int = 0;
//
// first door bell reg, ie for queue = 0.  Only need the first one, as
// we will use the queue number to derive the others
//
pub const VK_BAR0_REGSEG_DB_BASE: c_uint = 0x484;

// DB register gap,
// DB1 at 0x48c and DB2 at 0x494
//
// reset register and specific values
pub const VK_BAR0_RESET_DB_NUM: c_int = 3;
pub const VK_BAR0_RESET_DB_SOFT: c_uint = 0xffffffff;
pub const VK_BAR0_RESET_DB_HARD: c_uint = 0xfffffffd;
pub const VK_BAR0_RESET_RAMPDUMP: c_uint = 0xa0000000;

pub const BAR_BOOTSRC_SELECT: c_uint = 0xc78;
// BOOTSRC definitions

// Card OS Firmware version size
pub const BAR_FIRMWARE_TAG_SIZE: c_int = 50;
pub const FIRMWARE_STATUS_PRE_INIT_DONE: c_uint = 0x1f;
// VK MSG_ID defines
pub const VK_MSG_ID_BITMAP_SIZE: c_int = 4096;

pub const VK_MSG_ID_OVERFLOW: c_uint = 0xffff;
//
// BAR1
//
// BAR1 message q definition
// indicate if msgq ctrl in BAR1 is populated
pub const VK_BAR1_MSGQ_DEF_RDY: c_uint = 0x60c0;
// ready marker value for the above location, normal boot2
pub const VK_BAR1_MSGQ_RDY_MARKER: c_uint = 0xbeefcafe;
// ready marker value for the above location, normal boot2
pub const VK_BAR1_DIAG_RDY_MARKER: c_uint = 0xdeadcafe;
// number of msgqs in BAR1
pub const VK_BAR1_MSGQ_NR: c_uint = 0x60c4;
// BAR1 queue control structure offset
pub const VK_BAR1_MSGQ_CTRL_OFF: c_uint = 0x60c8;
// BAR1 ucode and boot1 version tag
pub const VK_BAR1_UCODE_VER_TAG: c_uint = 0x6170;
pub const VK_BAR1_BOOT1_VER_TAG: c_uint = 0x61b0;
pub const VK_BAR1_VER_TAG_SIZE: c_int = 64;
// Memory to hold the DMA buffer memory address allocated for boot2 download
pub const VK_BAR1_DMA_BUF_OFF_HI: c_uint = 0x61e0;

// Scratch memory allocated on host for VK
pub const VK_BAR1_SCRATCH_OFF_HI: c_uint = 0x61f0;

pub const VK_BAR1_SCRATCH_DEF_NR_PAGES: c_int = 32;
// BAR1 DAUTH info
pub const VK_BAR1_DAUTH_BASE_ADDR: c_uint = 0x6200;
pub const VK_BAR1_DAUTH_STORE_SIZE: c_uint = 0x48;
pub const VK_BAR1_DAUTH_VALID_SIZE: c_uint = 0x8;
pub const VK_BAR1_DAUTH_MAX: c_int = 4;

// BAR1 SOTP AUTH and REVID info
pub const VK_BAR1_SOTP_REVID_BASE_ADDR: c_uint = 0x6340;
pub const VK_BAR1_SOTP_REVID_SIZE: c_uint = 0x10;
pub const VK_BAR1_SOTP_REVID_MAX: c_int = 2;

// VK device supports a maximum of 3 bars
pub const MAX_BAR: c_int = 3;
// default number of msg blk for inband SGL
pub const BCM_VK_DEF_IB_SGL_BLK_LEN: c_int = 16;
pub const BCM_VK_IB_SGL_BLK_MAX: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_barno {
    BAR_0 = 0,
    BAR_1,
    BAR_2
}

pub const BCM_VK_NUM_TTY: c_int = 2;

pub const BCM_VK_NUM_TTY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_tty {
    pub port: tty_port,
    pub /: *mut *mut u32 to_offset; / bar offset to use,
    pub /: *mut *mut u32 to_size; / to VK buffer size,
    pub /: *mut *mut u32 wr; / write offset shadow,
    pub /: *mut *mut u32 from_offset; / bar offset to use,
    pub /: *mut *mut u32 from_size; / from VK buffer size,
    pub /: *mut *mut u32 rd; / read offset shadow,
    pub pid: pid_t,
    pub irq_enabled: bool,
    pub /: *mut *mut bool is_opened; / tracks tty open/close,
}

// VK device max power state, supports 3, full, reduced and low
pub const MAX_OPP: c_int = 3;
pub const MAX_CARD_INFO_TAG_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_card_info {
    pub version: u32,
    pub os_tag: [c_char; MAX_CARD_INFO_TAG_SIZE],
    pub cmpt_tag: [c_char; MAX_CARD_INFO_TAG_SIZE],
    pub cpu_freq_mhz: u32,
    pub cpu_scale: [u32; MAX_OPP],
    pub ddr_freq_mhz: u32,
    pub ddr_size_MB: u32,
    pub video_core_freq_mhz: u32,
}

// DAUTH related info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_dauth_key {
    pub store: [c_char; VK_BAR1_DAUTH_STORE_SIZE],
    pub valid: [c_char; VK_BAR1_DAUTH_VALID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_dauth_info {
    pub keys: [bcm_vk_dauth_key; VK_BAR1_DAUTH_MAX],
}

//
// Control structure of logging messages from the card.  This
// buffer is for logmsg that comes from vk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_peer_log {
    pub rd_idx: u32,
    pub wr_idx: u32,
    pub buf_size: u32,
    pub mask: u32,
}

// max buf size allowed

// max size per line of peer log
pub const BCM_VK_PEER_LOG_LINE_MAX: c_int = 256;
//
// single entry for processing type + utilization
//
pub const BCM_VK_PROC_TYPE_TAG_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_proc_mon_entry_t {
    pub tag: [c_char; BCM_VK_PROC_TYPE_TAG_LEN],
    pub used: u32,
    pub /: *mut *mut *mut u32 max; /< max capacity,
}

//
// Structure for run time utilization
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_proc_mon_info {
    pub /: *mut *mut *mut u32 num; /< no of entries,
    pub /: *mut *mut *mut u32 entry_size; /< per entry size,
    pub entries: [bcm_vk_proc_mon_entry_t; BCM_VK_PROC_MON_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_hb_ctrl {
    pub work: delayed_work,
    pub last_uptime: u32,
    pub lost_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_alert {
    pub flags: u16,
    pub notfs: u16,
}

// some alert counters that the driver will keep track
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_alert_cnts {
    pub ecc: u16,
    pub uecc: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk {
    pub pdev: *mut pci_dev,
    pub bar: [*mut void __iomem; MAX_BAR],
    pub num_irqs: c_int,
    pub card_info: bcm_vk_card_info,
    pub proc_mon_info: bcm_vk_proc_mon_info,
    pub dauth_info: bcm_vk_dauth_info,
// mutex to protect the ioctls
    pub mutex: mutex,
    pub miscdev: miscdevice,
    pub /: *mut *mut int devid; / dev id allocated,

    pub tty_drv: *mut tty_driver,
    pub serial_timer: timer_list,
    pub tty: [bcm_vk_tty; BCM_VK_NUM_TTY],
    pub tty_wq_thread: *mut workqueue_struct,
    pub tty_wq_work: work_struct,

// Reference-counting to handle file operations
    pub kref: kref,
    pub /: *mut *mut spinlock_t msg_id_lock; / Spinlock for msg_id,
    pub msg_id: u16,
    pub VK_MSG_ID_BITMAP_SIZE): DECLARE_BITMAP(bmap,,
    pub /: *mut *mut spinlock_t ctx_lock; / Spinlock for component context,
    pub ctx: [bcm_vk_ctx; VK_CMPT_CTX_MAX],
    pub pid_ht: [bcm_vk_ht_entry; VK_PID_HT_SZ],
    pub /: *mut *mut pid_t reset_pid; / process that issue reset,
    pub /: *mut *mut atomic_t msgq_inited; / indicate if info has been synced with vk,
    pub to_v_msg_chan: bcm_vk_msg_chan,
    pub to_h_msg_chan: bcm_vk_msg_chan,
    pub wq_thread: *mut workqueue_struct,
    pub /: *mut *mut work_wq_work; / work queue for deferred job,
    pub /: *mut *mut unsigned long wq_offload[1]; / various flags on wq requested,
    pub /: *mut *mut *mut void tdma_vaddr; / test dma segment virtual addr,
    pub /: *mut *mut dma_addr_t tdma_addr; / test dma segment bus addr,
    pub panic_nb: notifier_block,
    pub /: *mut *mut u32 ib_sgl_size; / size allocated for inband sgl insertion,
// heart beat mechanism control structure
    pub hb_ctrl: bcm_vk_hb_ctrl,
// house-keeping variable of error logs
    pub /: *mut *mut spinlock_t host_alert_lock; / protection to access host_alert struct,
    pub host_alert: bcm_vk_alert,
    pub /: *mut *mut bcm_vk_alert peer_alert; / bits set by the card,
    pub alert_cnts: bcm_vk_alert_cnts,
// offset of the peer log control in BAR2
    pub peerlog_off: u32,
    pub /: *mut *mut bcm_vk_peer_log peerlog_info; / record of peer log info,
// offset of processing monitoring info in BAR2
    pub proc_mon_off: u32,
}

// wq offload work items bits definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm_vk_wq_offload_flags {
    BCM_VK_WQ_DWNLD_PEND = 0,
    BCM_VK_WQ_DWNLD_AUTO = 1,
    BCM_VK_WQ_NOTF_PEND  = 2,
}

// a macro to get an individual field with mask and shift

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_vk_entry {
    pub mask: u32,
    pub exp_val: u32,
    pub str: *const c_char,
}

// alerts that could be generated from peer
pub const BCM_VK_PEER_ERR_NUM: c_int = 12;
// alerts detected by the host
pub const BCM_VK_HOST_ERR_NUM: c_int = 3;
//
// check if PCIe interface is down on read.  Use it when it is
// certain that _val should never be all ones.
//

extern "C" {
    pub fn readl(offset: vk->bar[bar] +) -> return;
}
extern "C" {
    pub fn readb(offset: vk->bar[bar] +) -> return;
}
extern "C" {
    pub fn bcm_vk_open(inode: *mut inode, p_file: *mut file) -> c_int;
}
extern "C" {
    pub fn bcm_vk_poll(p_file: *mut file, wait: *mut poll_table_struct) -> __poll_t;
}
extern "C" {
    pub fn bcm_vk_release(inode: *mut inode, p_file: *mut file) -> c_int;
}
extern "C" {
    pub fn bcm_vk_release_data(kref: *mut kref);
}
extern "C" {
    pub fn bcm_vk_msgq_irqhandler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn bcm_vk_notf_irqhandler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn bcm_vk_tty_irqhandler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn bcm_vk_msg_init(vk: *mut bcm_vk) -> c_int;
}
extern "C" {
    pub fn bcm_vk_msg_remove(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_vk_drain_msg_on_reset(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_vk_sync_msgq(vk: *mut bcm_vk, force_sync: bool) -> c_int;
}
extern "C" {
    pub fn bcm_vk_blk_drv_access(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_to_h_msg_dequeue(vk: *mut bcm_vk) -> i32;
}
extern "C" {
    pub fn bcm_to_v_q_doorbell(vk: *mut bcm_vk, q_num: u32, db_val: u32);
}
extern "C" {
    pub fn bcm_vk_auto_load_all_images(vk: *mut bcm_vk) -> c_int;
}
extern "C" {
    pub fn bcm_vk_hb_init(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_vk_hb_deinit(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_vk_handle_notf(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_vk_drv_access_ok(vk: *mut bcm_vk) -> bool;
}
extern "C" {
    pub fn bcm_vk_set_host_alert(vk: *mut bcm_vk, bit_mask: u32);
}

extern "C" {
    pub fn bcm_vk_tty_init(vk: *mut bcm_vk, name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn bcm_vk_tty_exit(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_vk_tty_terminate_tty_user(vk: *mut bcm_vk);
}
extern "C" {
    pub fn bcm_vk_tty_wq_exit(vk: *mut bcm_vk);
}

