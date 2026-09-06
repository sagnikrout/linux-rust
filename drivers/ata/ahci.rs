//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ata/ahci.h
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
// ahci.h - Common AHCI SATA definitions and declarations
//
// Maintained by:  Tejun Heo <tj@kernel.org>
// Please ALWAYS copy linux-ide@vger.kernel.org
// on emails.
//
// Copyright 2004-2005 Red Hat, Inc.
//
// libata documentation is available via 'make {ps|pdf}docs',
// as Documentation/driver-api/libata.rst
//
// AHCI hardware documentation:
// http://www.intel.com/technology/serialata/pdf/rev1_0.pdf
// http://www.intel.com/technology/serialata/pdf/rev1_1.pdf
//

// Enclosure Management Control
pub const EM_CTRL_MSG_TYPE: c_uint = 0x000f0000;
// Enclosure Management LED Message Type
pub const EM_MSG_LED_HBA_PORT: c_uint = 0x0000000f;
pub const EM_MSG_LED_PMP_SLOT: c_uint = 0x0000ff00;
pub const EM_MSG_LED_VALUE: c_uint = 0xffff0000;
pub const EM_MSG_LED_VALUE_ACTIVITY: c_uint = 0x00070000;
pub const EM_MSG_LED_VALUE_OFF: c_uint = 0xfff80000;
pub const EM_MSG_LED_VALUE_ON: c_uint = 0x00010000;
// global controller registers
// HOST_CTL bits
// HOST_CAP bits
// HOST_CAP2 bits
// registers for each SATA port
// PORT_IRQ_{STAT,MASK} bits
// PORT_CMD bits
// PORT_CMD capabilities mask
// PORT_FBS bits
// PORT_DEVSLP bits
// hpriv->flags bits

// compile out MSI infrastructure

// ap->flags bits
// em constants
// em_ctl bits
// em message type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_cmd_hdr {
    pub opts: __le32,
    pub status: __le32,
    pub tbl_addr: __le32,
    pub tbl_addr_hi: __le32,
    pub reserved: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_sg {
    pub addr: __le32,
    pub addr_hi: __le32,
    pub reserved: __le32,
    pub flags_size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_em_priv {
    pub blink_policy: sw_activity,
    pub timer: timer_list,
    pub saved_activity: c_ulong,
    pub activity: c_ulong,
    pub led_state: c_ulong,
    pub link: *mut ata_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_port_priv {
    pub active_link: *mut ata_link,
    pub cmd_slot: *mut ahci_cmd_hdr,
    pub cmd_slot_dma: dma_addr_t,
    pub cmd_tbl: *mut c_void,
    pub cmd_tbl_dma: dma_addr_t,
    pub rx_fis: *mut c_void,
    pub rx_fis_dma: dma_addr_t,
// for NCQ spurious interrupt analysis
    pub ncq_saw_d2h:1: c_uint,
    pub ncq_saw_dmas:1: c_uint,
    pub ncq_saw_sdb:1: c_uint,
    pub /: *mut *mut spinlock_t lock; / protects parent ata_port,
    pub /: *mut *mut u32 intr_mask; / interrupts to enable,
    pub /: *mut *mut bool fbs_supported; / set iff FBS is supported,
    pub /: *mut *mut bool fbs_enabled; / set iff FBS is enabled,
    pub /: *mut *mut int fbs_last_dev; / save FBS.DEV of last FIS,
// enclosure management info per PM slot
    pub em_priv: [ahci_em_priv; EM_MAX_SLOTS],
    pub /: *mut *mut *mut char irq_desc; / desc in /proc/interrupts,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahci_host_priv {
// Input fields
    pub /: *mut *mut *mut unsigned int flags; / AHCI_HFLAG_,
    pub /: *mut *mut u32 mask_port_map; / Mask of valid ports,
    pub /: *mut *mut u32 mask_port_ext; / Mask of ports ext capability,
    pub /: *mut *mut *mut void __iomem  mmio; / bus-independent mem map,
    pub /: *mut *mut u32 cap; / cap to use,
    pub /: *mut *mut u32 cap2; / cap2 to use,
    pub /: *mut *mut u32 version; / cached version,
    pub /: *mut *mut u32 port_map; / port map to use,
    pub /: *mut *mut u32 saved_cap; / saved initial cap,
    pub /: *mut *mut u32 saved_cap2; / saved initial cap2,
    pub /: *mut *mut u32 saved_port_map; / saved initial port_map,
    pub /: *mut *mut u32 saved_port_cap[AHCI_MAX_PORTS]; / saved port_cap,
    pub /: *mut *mut u32 em_loc; / enclosure management location,
    pub /: *mut *mut u32 em_buf_sz; / EM buffer size in byte,
    pub /: *mut *mut u32 em_msg_type; / EM message type,
    pub /: *mut *mut u32 remapped_nvme; / NVMe remapped device count,
    pub /: *mut *mut bool got_runtime_pm; / Did we do pm_runtime_get?,
    pub n_clks: c_uint,
    pub /: *mut *mut *mut clk_bulk_data clks; / Optional,
    pub f_rsts: c_uint,
    pub /: *mut *mut *mut reset_control rsts; / Optional,
    pub /: *mut *mut *mut *mut regulator target_pwrs; / Optional,
    pub /: *mut *mut *mut regulator ahci_regulator;/ Optional,
    pub /: *mut *mut *mut regulator phy_regulator;/ Optional,
//
// If platform uses PHYs. There is a 1:1 relation between the port number and
// the PHY position in this array.
//
    pub /: *mut *mut unsigned nports; / Number of ports,
    pub /: *mut *mut *mut void plat_data; / Other platform data,
    pub /: *mut *mut unsigned int irq; / interrupt line,
//
// Optional ahci_start_engine override, if not set this gets set to the
// default ahci_start_engine during ahci_save_initial_config, this can
// be overridden anytime before the host is activated.
//
    pub ap): *mut *mut void (start_engine)(struct ata_port,
//
// Optional ahci_stop_engine override, if not set this gets set to the
// default ahci_stop_engine during ahci_save_initial_config, this can
// be overridden anytime before the host is activated.
//
    pub ap): *mut *mut int (stop_engine)(struct ata_port,
    pub dev_instance): *mut *mut irqreturn_t (irq_handler)(int irq, void,
// only required for per-port MSI(-X) support
    pub port): c_int,
    pub __counted_by(nports): *mut *mut phy phys[],
}

//
// Return true if a port should be ignored because it is excluded from
// the host port map.
//
// mask_port_map not set means that all ports are available
//
// This must be instantiated by the edge drivers.  Read the comments
// for ATA_BASE_SHT
//

extern "C" {
    pub fn ahci_dev_classify(ap: *mut ata_port) -> c_uint;
}
extern "C" {
    pub fn ahci_init_controller(host: *mut ata_host);
}
extern "C" {
    pub fn ahci_reset_controller(host: *mut ata_host) -> c_int;
}
extern "C" {
    pub fn ahci_qc_issue(qc: *mut ata_queued_cmd) -> c_uint;
}
extern "C" {
    pub fn ahci_stop_engine(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ahci_start_fis_rx(ap: *mut ata_port);
}
extern "C" {
    pub fn ahci_start_engine(ap: *mut ata_port);
}
extern "C" {
    pub fn ahci_check_ready(link: *mut ata_link) -> c_int;
}
extern "C" {
    pub fn ahci_kick_engine(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ahci_port_resume(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ahci_reset_em(host: *mut ata_host) -> c_int;
}
extern "C" {
    pub fn ahci_print_info(host: *mut ata_host, scc_s: *const c_char);
}
extern "C" {
    pub fn ahci_host_activate(host: *mut ata_host, sht: *const scsi_host_template) -> c_int;
}
extern "C" {
    pub fn ahci_handle_port_intr(host: *mut ata_host, irq_masked: u32) -> u32;
}
extern "C" {
    pub fn __ahci_port_base(_arg: hpriv, _arg: ap->port_no) -> return;
}
