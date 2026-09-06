//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/libata.h
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
// Copyright 2003-2005 Red Hat, Inc.  All rights reserved.
// Copyright 2003-2005 Jeff Garzik
//
// libata documentation is available via 'make {ps|pdf}docs',
// as Documentation/driver-api/libata.rst
//

//
// Define if arch has non-standard setup.  This is a _PCI_ standard
// not a legacy or ISA standard.
//

pub const ATA_PRIMARY_IRQ(dev): c_int = 14;
pub const ATA_SECONDARY_IRQ(dev): c_int = 15;

//
// compile-time options: to be removed as soon as all the drivers are
// converted to the new debugging mechanism
//

// defines only for the constants which don't work well as enums
pub const ATA_TAG_POISON: c_uint = 0xfafbfcfdU;
//
// Quirk flags bits.
// ata_device->quirks is an u64, so __ATA_QUIRK_MAX must not exceed 64.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_quirks {
    __ATA_QUIRK_DIAGNOSTIC,		/* Failed boot diag */
    __ATA_QUIRK_NODMA,		/* DMA problems */
    __ATA_QUIRK_NONCQ,		/* Don't use NCQ */
    __ATA_QUIRK_BROKEN_HPA,		/* Broken HPA */
    __ATA_QUIRK_DISABLE,		/* Disable it */
    __ATA_QUIRK_HPA_SIZE,		/* Native size off by one */
    __ATA_QUIRK_IVB,		/* cbl det validity bit bugs */
    __ATA_QUIRK_STUCK_ERR,		/* Stuck ERR on next PACKET */
    __ATA_QUIRK_BRIDGE_OK,		/* No bridge limits */
    __ATA_QUIRK_ATAPI_MOD16_DMA,	/* Use ATAPI DMA for commands that */
// are not a multiple of 16 bytes
    __ATA_QUIRK_FIRMWARE_WARN,	/* Firmware update warning */
    __ATA_QUIRK_1_5_GBPS,		/* Force 1.5 Gbps */
    __ATA_QUIRK_NOSETXFER,		/* Skip SETXFER, SATA only */
    __ATA_QUIRK_BROKEN_FPDMA_AA,	/* Skip AA */
    __ATA_QUIRK_DUMP_ID,		/* Dump IDENTIFY data */
    __ATA_QUIRK_MAX_SEC_LBA48,	/* Set max sects to 65535 */
    __ATA_QUIRK_ATAPI_DMADIR,	/* Device requires dmadir */
    __ATA_QUIRK_NO_NCQ_TRIM,	/* Do not use queued TRIM */
    __ATA_QUIRK_NOLPM,		/* Do not use LPM */
    __ATA_QUIRK_WD_BROKEN_LPM,	/* Some WDs have broken LPM */
    __ATA_QUIRK_ZERO_AFTER_TRIM,	/* Guarantees zero after trim */
    __ATA_QUIRK_NO_DMA_LOG,		/* Do not use DMA for log read */
    __ATA_QUIRK_NOTRIM,		/* Do not use TRIM */
    __ATA_QUIRK_MAX_SEC,		/* Limit max sectors */
    __ATA_QUIRK_MAX_TRIM_128M,	/* Limit max trim size to 128M */
    __ATA_QUIRK_NO_NCQ_ON_ATI,	/* Disable NCQ on ATI chipset */
    __ATA_QUIRK_NO_LPM_ON_ATI,	/* Disable LPM on ATI chipset */
    __ATA_QUIRK_NO_ID_DEV_LOG,	/* Identify device log missing */
    __ATA_QUIRK_NO_LOG_DIR,		/* Do not read log directory */
    __ATA_QUIRK_NO_FUA,		/* Do not use FUA */

    __ATA_QUIRK_MAX,
}

//
// Quirk flags: may be set by libata or controller drivers on drives.
// Some quirks may be drive/controller pair dependent.
//
// struct ata_device flags
//
// various global constants
// struct ata_taskfile flags
// sturct ata_device class.
// struct ata_link flags
// NOTE: struct ata_force_param currently stores lflags in u16
// struct ata_port flags
// (doesn't imply presence)
// doesn't handle PIO interrupts
// management
// led
// bits 24:31 of ap->flags are reserved for LLD specific flags
// struct ata_port pflags
// struct ata_queued_cmd flags
// host set flags
// bits 24:31 of host->flags are reserved for LLD specific flags
// Various lengths of time
//
// GoVault needs 2s and iVDR disk HHD424020F7SV00 800ms.  2s
// is too much without parallel probing.  Use 2s if parallel
// probing is available, 800ms otherwise.
//
// Spec mandates to wait for ">= 2ms" before checking status
// after reset.  We wait 150ms, because that was the magic
// delay used for ATAPI devices in Hale Landis's ATADRVR, for
// the period of time between when the ATA command register is
// written, and then status is checked.  Because waiting for
// "a while" before checking status is fine, post SRST, we
// perform this magic delay here as well.
//
// Old drivers/ide uses the 2mS rule and then waits for ready.
//
// If PMP is supported, we have to do follow-up SRST.  As some
// PMPs don't send D2H Reg FIS after hardreset, LLDs are
// advised to wait only for the following duration before
// doing SRST.
//
// When the LPM policy is set to ATA_LPM_MAX_POWER, there might
// be a spurious PHY event, so ignore the first PHY event that
// occurs within 10s after the policy change.
//
// ATA bus states
// SATA port states
// encoding various smaller bitmaps into a single
// unsigned int bitmap
//
// size of buffer to pad xfers ending on unaligned boundaries
// ering size
// return values for ->qc_defer
// desc_len for ata_eh_info and context
// reset / recovery action types
// ata_eh_info->flags
// mask of flags to transfer *to* the slave link
// max tries if error condition is still set after ->error_handler
// sometimes resuming a link requires several retries
// how hard are we gonna try to probe/recover devices
// This should match the actual table size of
// ata_eh_cmd_timeout_table in libata-eh.c.
//
// User visible DMA mask for DMA control. DO NOT renumber.
// ATAPI command types
// Timing constants
// ACPI constants
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_xfer_mask {
    ATA_MASK_PIO		= ((1U << ATA_NR_PIO_MODES) - 1) << ATA_SHIFT_PIO,
    ATA_MASK_MWDMA		= ((1U << ATA_NR_MWDMA_MODES) - 1) << ATA_SHIFT_MWDMA,
    ATA_MASK_UDMA		= ((1U << ATA_NR_UDMA_MODES) - 1) << ATA_SHIFT_UDMA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hsm_task_states {
    HSM_ST_IDLE,		/* no command on going */
    HSM_ST_FIRST,		/* (waiting the device to)
    write CDB or first data block */
    HSM_ST,			/* (waiting the device to) transfer data */
    HSM_ST_LAST,		/* (waiting the device to) complete command */
    HSM_ST_ERR,		/* error */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_completion_errors {
    AC_ERR_OK		= 0,	    /* no error */
    AC_ERR_DEV		= (1 << 0), /* device reported error */
    AC_ERR_HSM		= (1 << 1), /* host state machine violation */
    AC_ERR_TIMEOUT		= (1 << 2), /* timeout */
    AC_ERR_MEDIA		= (1 << 3), /* media error */
    AC_ERR_ATA_BUS		= (1 << 4), /* ATA bus error */
    AC_ERR_HOST_BUS		= (1 << 5), /* host bus error */
    AC_ERR_SYSTEM		= (1 << 6), /* system error */
    AC_ERR_INVALID		= (1 << 7), /* invalid argument */
    AC_ERR_OTHER		= (1 << 8), /* unknown */
    AC_ERR_NODEV_HINT	= (1 << 9), /* polling device detection hint */
    AC_ERR_NCQ		= (1 << 10), /* marker for offending NCQ qc */
}

//
// Link Power Management (LPM) policies.
//
// The default LPM policy to use for a device link is defined using these values
// with the CONFIG_SATA_MOBILE_LPM_POLICY config option and applied through the
// target_lpm_policy field of struct ata_port.
//
// If you alter this, you also need to alter the policy names used with the
// sysfs attribute link_power_management_policy defined in libata-sata.c.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_lpm_policy {
// Keep firmware settings
    ATA_LPM_UNKNOWN,
// No power savings (maximum performance)
    ATA_LPM_MAX_POWER,
// HIPM (Partial)
    ATA_LPM_MED_POWER,
// HIPM (Partial) and DIPM (Partial and Slumber)
    ATA_LPM_MED_POWER_WITH_DIPM,
// HIPM (Partial and DevSleep) and DIPM (Partial and Slumber)
    ATA_LPM_MIN_POWER_WITH_PARTIAL,
// HIPM (Slumber and DevSleep) and DIPM (Partial and Slumber)
    ATA_LPM_MIN_POWER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_lpm_hints {
    ATA_LPM_EMPTY		= (1 << 0), /* port empty/probing */
    ATA_LPM_HIPM		= (1 << 1), /* may use HIPM */
    ATA_LPM_WAKE_ONLY	= (1 << 2), /* only wake up link */
}

// forward declarations
// typedefs
extern "C" {
    pub fn void(qc: *mut *mut ata_qc_cb_t) (struct ata_queued_cmd) -> typedef;
}
extern "C" {
    pub fn int(link: *mut *mut ata_prereset_fn_t)(struct ata_link, deadline: c_ulong) -> typedef;
}
extern "C" {
    pub fn void(link: *mut *mut ata_postreset_fn_t)(struct ata_link, classes: *mut c_uint) -> typedef;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sw_activity {
    OFF,
    BLINK_ON,
    BLINK_OFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_taskfile {
    pub /: *mut *mut unsigned long flags; / ATA_TFLAG_xxx,
    pub /: *mut *mut u8 protocol; / ATA_PROT_xxx,
    pub /: *mut *mut u8 ctl; / control reg,
    pub /: *mut *mut u8 hob_feature; / additional data,
    pub /: *mut *mut u8 hob_nsect; / to support LBA48,
    pub hob_lbal: u8,
    pub hob_lbam: u8,
    pub hob_lbah: u8,
    pub error: u8,
    pub feature: u8,
}

// from SATA 3.1 and
// ATA-8 ACS-3

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_ioports {
    pub cmd_addr: *mut void __iomem,
    pub data_addr: *mut void __iomem,
    pub error_addr: *mut void __iomem,
    pub feature_addr: *mut void __iomem,
    pub nsect_addr: *mut void __iomem,
    pub lbal_addr: *mut void __iomem,
    pub lbam_addr: *mut void __iomem,
    pub lbah_addr: *mut void __iomem,
    pub device_addr: *mut void __iomem,
    pub status_addr: *mut void __iomem,
    pub command_addr: *mut void __iomem,
    pub altstatus_addr: *mut void __iomem,
    pub ctl_addr: *mut void __iomem,

    pub bmdma_addr: *mut void __iomem,

    pub scr_addr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_host {
    pub lock: spinlock_t,
    pub dev: *mut device,
    pub iomap: *const *const void __iomem,
    pub n_ports: c_uint,
    pub /: *mut *mut unsigned int n_tags; / nr of NCQ tags,
    pub private_data: *mut c_void,
    pub ops: *mut ata_port_operations,
    pub flags: c_ulong,
    pub kref: kref,
    pub eh_mutex: mutex,
    pub eh_owner: *mut task_struct,
    pub /: *mut *mut *mut ata_port simplex_claimed; / channel owning the DMA,
    pub ports: [*mut ata_port; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_queued_cmd {
    pub ap: *mut ata_port,
    pub dev: *mut ata_device,
    pub scsicmd: *mut scsi_cmnd,
    pub ): *mut *mut void (scsidone)(struct scsi_cmnd,
    pub tf: ata_taskfile,
    pub cdb: [u8; ATAPI_CDB_LEN],
    pub /: *mut *mut unsigned long flags; / ATA_QCFLAG_xxx,
    pub /: *mut *mut unsigned int tag; / libata core tag,
    pub /: *mut *mut unsigned int hw_tag; / driver tag,
    pub n_elem: c_uint,
    pub orig_n_elem: c_uint,
    pub dma_dir: c_int,
    pub sect_size: c_uint,
    pub nbytes: c_uint,
    pub extrabytes: c_uint,
    pub curbytes: c_uint,
    pub sgent: scatterlist,
    pub sg: *mut scatterlist,
    pub cursg: *mut scatterlist,
    pub cursg_ofs: c_uint,
    pub err_mask: c_uint,
    pub result_tf: ata_taskfile,
    pub complete_fn: ata_qc_cb_t,
    pub private_data: *mut c_void,
    pub lldd_task: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_port_stats {
    pub unhandled_irq: c_ulong,
    pub idle_irq: c_ulong,
    pub rw_reqbuf: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_ering_entry {
    pub eflags: c_uint,
    pub err_mask: c_uint,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_ering {
    pub cursor: c_int,
    pub ring: [ata_ering_entry; ATA_ERING_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_cpr {
    pub num: u8,
    pub num_storage_elements: u8,
    pub start_lba: u64,
    pub num_lbas: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_cpr_log {
    pub nr_cpr: u8,
    pub __counted_by(nr_cpr): ata_cpr cpr[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_cdl {
//
// Buffer to cache the CDL log page 18h (command duration descriptors)
// for SCSI-ATA translation.
//
    pub desc_log_buf: [u8; ATA_LOG_CDL_SIZE],
//
// Buffer to handle reading the sense data for successful NCQ Commands
// log page for commands using a CDL with one of the limits policy set
// to 0xD (successful completion with sense data available bit set).
//
    pub ncq_sense_log_buf: [u8; ATA_LOG_SENSE_NCQ_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_device {
    pub link: *mut ata_link,
    pub /: *mut *mut unsigned int devno; / 0 or 1,
    pub /: *mut *mut u64 quirks; / List of broken features,
    pub /: *mut *mut unsigned long flags; / ATA_DFLAG_xxx,
    pub /: *mut *mut *mut scsi_device sdev; / attached SCSI device,
    pub private_data: *mut c_void,

    pub gtf_cache: *mut acpi_object,
    pub gtf_filter: c_uint,

    pub zpodd: *mut c_void,

    pub tdev: device,
// n_sector is CLEAR_BEGIN, read comment above CLEAR_BEGIN
    pub /: *mut *mut u64 n_sectors; / size of device, if ATA,
    pub /: *mut *mut u64 n_native_sectors; / native size, if ATA,
    pub /: *mut *mut unsigned int class; / ATA_DEV_xxx,
    pub unpark_deadline: c_ulong,
    pub pio_mode: u8,
    pub dma_mode: u8,
    pub xfer_mode: u8,
    pub /: *mut *mut unsigned int xfer_shift; / ATA_SHIFT_xxx,
    pub for: *mut *mut unsigned int multi_count; / sectors count,
    pub /: *mut *mut unsigned int max_sectors; / per-device max sectors,
    pub cdb_len: c_uint,
// per-dev xfer mask
    pub pio_mask: c_uint,
    pub mwdma_mask: c_uint,
    pub udma_mask: c_uint,
// for CHS addressing
    pub /: *mut *mut u16 cylinders; / Number of cylinders,
    pub /: *mut *mut u16 heads; / Number of heads,
    pub /: *mut *mut u16 sectors; / Number of sectors per track,
    pub /: *mut *mut u16 id[ATA_ID_WORDS]; / IDENTIFY xxx DEVICE data,
    pub /: *mut *mut u32 gscr[SATA_PMP_GSCR_DWORDS]; / PMP GSCR block,
    pub ____cacheline_aligned: },
// General Purpose Log Directory log page
    pub ____cacheline_aligned: u8 gp_log_dir[ATA_SECT_SIZE],
// DEVSLP Timing Variables from Identify Device Data Log
    pub devslp_timing: [u8; ATA_LOG_DEVSLP_SIZE],
// NCQ send and receive log subcommand support
    pub ncq_send_recv_cmds: [u8; ATA_LOG_NCQ_SEND_RECV_SIZE],
    pub ncq_non_data_cmds: [u8; ATA_LOG_NCQ_NON_DATA_SIZE],
// ZAC zone configuration
    pub zac_zoned_cap: u32,
    pub zac_zones_optimal_open: u32,
    pub zac_zones_optimal_nonseq: u32,
    pub zac_zones_max_open: u32,
// Concurrent positioning ranges
    pub cpr_log: *mut ata_cpr_log,
// Command Duration Limits support
    pub cdl: *mut ata_cdl,
// error history
    pub spdn_cnt: c_int,
// ering is CLEAR_END, read comment above CLEAR_END
    pub ering: ata_ering,
// For EH
    pub ____cacheline_aligned: u8 sector_buf[ATA_SECT_SIZE],
}

// Fields between ATA_DEVICE_CLEAR_BEGIN and ATA_DEVICE_CLEAR_END are
// cleared to zero on ata_dev_init().
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_eh_info {
    pub /: *mut *mut *mut ata_device dev; / offending device,
    pub /: *mut *mut u32 serror; / SError from LLDD,
    pub /: *mut *mut unsigned int err_mask; / port-wide err_mask,
    pub /: *mut *mut *mut unsigned int action; / ATA_EH_ action mask,
    pub /: *mut *mut unsigned int dev_action[ATA_MAX_DEVICES]; / dev EH action,
    pub /: *mut *mut *mut unsigned int flags; / ATA_EHI_ flags,
    pub probe_mask: c_uint,
    pub desc: [c_char; ATA_EH_DESC_LEN],
    pub desc_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_eh_context {
    pub i: ata_eh_info,
    pub tries: [c_int; ATA_MAX_DEVICES],
    pub classes: [c_uint; ATA_MAX_DEVICES],
    pub did_probe_mask: c_uint,
    pub unloaded_mask: c_uint,
    pub saved_ncq_enabled: c_uint,
    pub saved_xfer_mode: [u8; ATA_MAX_DEVICES],
// timestamp for the last reset attempt or success
    pub last_reset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_acpi_gtm {
    pub drive: [ata_acpi_drive; 2],
    pub flags: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_link {
    pub ap: *mut ata_port,
    pub /: *mut *mut int pmp; / port multiplier port #,
    pub tdev: device,
    pub /: *mut *mut unsigned int active_tag; / active tag on this link,
    pub /: *mut *mut u32 sactive; / active NCQ commands,
    pub /: *mut *mut unsigned int flags; / ATA_LFLAG_xxx,
    pub /: *mut *mut u32 saved_scontrol; / SControl on probe,
    pub hw_sata_spd_limit: c_uint,
    pub sata_spd_limit: c_uint,
    pub /: *mut *mut unsigned int sata_spd; / current SATA PHY speed,
    pub lpm_policy: ata_lpm_policy,
    pub deferred_qc_work: work_struct,
    pub deferred_qc: *mut ata_queued_cmd,
// record runtime error info, protected by host_set lock
    pub eh_info: ata_eh_info,
// EH context
    pub eh_context: ata_eh_context,
    pub device: [ata_device; ATA_MAX_DEVICES],
    pub /: *mut *mut unsigned long last_lpm_change; / when last LPM change happened,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_port {
    pub /: *mut *mut *mut Scsi_Host scsi_host; / our co-allocated scsi host,
    pub ops: *mut ata_port_operations,
    pub lock: *mut spinlock_t,
// Flags owned by the EH context. Only EH should touch these once the
    pub /: *mut *mut unsigned long flags; / ATA_FLAG_xxx,
// Flags that change dynamically, protected by ap->lock
    pub /: *mut *mut unsigned int pflags; / ATA_PFLAG_xxx,
    pub /: *mut *mut unsigned int print_id; / user visible unique port ID,
    pub /: *mut *mut unsigned int port_no; / 0 based port no. inside the host,

    pub /: *mut *mut ata_ioports ioaddr; / ATA cmd/ctl/dma register blocks,
    pub /: *mut *mut u8 ctl; / cache of ATA control register,
    pub /: *mut *mut u8 last_ctl; / Cache last written value,
    pub /: *mut *mut *mut ata_link sff_pio_task_link; / link currently used,
    pub sff_pio_task: delayed_work,

    pub /: *mut *mut *mut ata_bmdma_prd bmdma_prd; / BMDMA SG list,
    pub /: *mut *mut dma_addr_t bmdma_prd_dma; / and its DMA mapping,

    pub pio_mask: c_uint,
    pub mwdma_mask: c_uint,
    pub udma_mask: c_uint,
    pub /: *mut *mut unsigned int cbl; / cable type; ATA_CBL_xxx,
    pub 1]: ata_queued_cmd qcmd[ATA_MAX_QUEUE +,
    pub qc_active: u64,
    pub /: *mut *mut int nr_active_links; / #links with active qcs,
    pub /: *mut *mut ata_link link; / host default link,
    pub /: *mut *mut *mut ata_link slave_link; / see ata_slave_link_init(),
    pub /: *mut *mut int nr_pmp_links; / nr of available PMP links,
    pub /: *mut *mut *mut ata_link pmp_link; / array of PMP links,
    pub /: *mut *mut *mut ata_link excl_link; / for PMP qc exclusion,
    pub stats: ata_port_stats,
    pub host: *mut ata_host,
    pub dev: *mut device,
    pub tdev: device,
    pub scsi_scan_mutex: mutex,
    pub hotplug_task: delayed_work,
    pub scsi_rescan_task: delayed_work,
    pub hsm_task_state: c_uint,
    pub eh_done_q: list_head,
    pub eh_wait_q: wait_queue_head_t,
    pub eh_tries: c_int,
    pub park_req_pending: completion,
    pub pm_mesg: pm_message_t,
    pub target_lpm_policy: ata_lpm_policy,
    pub fastdrain_timer: timer_list,
    pub fastdrain_cnt: c_uint,
    pub cookie: async_cookie_t,
    pub em_message_type: c_int,
    pub private_data: *mut c_void,

    pub /: *mut *mut ata_acpi_gtm __acpi_init_gtm; / use ata_acpi_init_gtm(),

}

// The following initializer overrides a method to NULL whether one of
// its parent has the method defined or not.  This is equivalent to
// ERR_PTR(-ENOENT).  Unfortunately, ERR_PTR doesn't render a constant
// expression and thus can't be used as an initializer.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_reset_operations {
    pub prereset: ata_prereset_fn_t,
    pub softreset: ata_reset_fn_t,
    pub hardreset: ata_reset_fn_t,
    pub postreset: ata_postreset_fn_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_port_operations {
//
// Command execution
//
    pub qc): *mut *mut int (qc_defer)(struct ata_queued_cmd,
    pub qc): *mut *mut int (check_atapi_dma)(struct ata_queued_cmd,
    pub qc): *mut *mut ata_completion_errors (qc_prep)(struct ata_queued_cmd,
    pub qc): *mut *mut unsigned int (qc_issue)(struct ata_queued_cmd,
    pub qc): *mut *mut void (qc_fill_rtf)(struct ata_queued_cmd,
    pub done_mask): *mut *mut *mut void (qc_ncq_fill_rtf)(struct ata_port ap, u64,
//
// Configuration and exception handling
//
    pub ap): *mut *mut int (cable_detect)(struct ata_port,
    pub xfer_mask): *mut *mut *mut unsigned int (mode_filter)(struct ata_device dev, unsigned int,
    pub dev): *mut *mut *mut void (set_piomode)(struct ata_port ap, struct ata_device,
    pub dev): *mut *mut *mut void (set_dmamode)(struct ata_port ap, struct ata_device,
    pub r_failed_dev): *mut *mut *mut int (set_mode)(struct ata_link link, struct ata_device,
    pub id): *mut __le16,
    pub dev): *mut *mut void (dev_config)(struct ata_device,
    pub ap): *mut *mut void (freeze)(struct ata_port,
    pub ap): *mut *mut void (thaw)(struct ata_port,
    pub reset: ata_reset_operations,
    pub pmp_reset: ata_reset_operations,
    pub ap): *mut *mut void (lost_interrupt)(struct ata_port,
    pub qc): *mut *mut void (post_internal_cmd)(struct ata_queued_cmd,
    pub ap): *mut *mut void (sched_eh)(struct ata_port,
    pub ap): *mut *mut void (end_eh)(struct ata_port,
//
// Optional features
//
    pub val): *mut *mut *mut int (scr_read)(struct ata_link link, unsigned int sc_reg, u32,
    pub val): *mut *mut *mut int (scr_write)(struct ata_link link, unsigned int sc_reg, u32,
    pub ap): *mut *mut void (pmp_attach)(struct ata_port,
    pub ap): *mut *mut void (pmp_detach)(struct ata_port,
    pub hints): unsigned,
//
// Start, stop, suspend and resume
//
    pub mesg): *mut *mut *mut int (port_suspend)(struct ata_port ap, pm_message_t,
    pub ap): *mut *mut int (port_resume)(struct ata_port,
    pub ap): *mut *mut int (port_start)(struct ata_port,
    pub ap): *mut *mut void (port_stop)(struct ata_port,
    pub host): *mut *mut void (host_stop)(struct ata_host,

//
// SFF / taskfile oriented ops
//
    pub device): *mut *mut *mut void (sff_dev_select)(struct ata_port ap, unsigned int,
    pub ctl): *mut *mut *mut void (sff_set_devctl)(struct ata_port ap, u8,
    pub ap): *mut *mut u8 (sff_check_status)(struct ata_port,
    pub ap): *mut *mut u8 (sff_check_altstatus)(struct ata_port,
    pub tf): *const *const *const void (sff_tf_load)(struct ata_port ap, struct ata_taskfile,
    pub tf): *mut *mut *mut void (sff_tf_read)(struct ata_port ap, struct ata_taskfile,
    pub tf): *const ata_taskfile,
    pub rw): *mut *mut unsigned char buf, unsigned int buflen, int,
    pub ): *mut *mut void (sff_irq_on)(struct ata_port,
    pub ): *mut *mut bool (sff_irq_check)(struct ata_port,
    pub ): *mut *mut void (sff_irq_clear)(struct ata_port,
    pub qc): *mut *mut void (sff_drain_fifo)(struct ata_queued_cmd,

    pub qc): *mut *mut void (bmdma_setup)(struct ata_queued_cmd,
    pub qc): *mut *mut void (bmdma_start)(struct ata_queued_cmd,
    pub qc): *mut *mut void (bmdma_stop)(struct ata_queued_cmd,
    pub ap): *mut *mut u8 (bmdma_status)(struct ata_port,

    pub buf): *mut *mut *mut ssize_t (em_show)(struct ata_port ap, char,
    pub size): usize,
    pub buf): *mut *mut *mut ssize_t (sw_activity_show)(struct ata_device dev, char,
    pub val): sw_activity,
    pub size): isize,
//
// ->inherits must be the last field and all the preceding
// fields must be pointers.
//
    pub inherits: *const ata_port_operations,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_port_info {
    pub flags: c_ulong,
    pub link_flags: c_ulong,
    pub pio_mask: c_uint,
    pub mwdma_mask: c_uint,
    pub udma_mask: c_uint,
    pub port_ops: *mut ata_port_operations,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ata_timing {
    pub /: *mut *mut unsigned short mode; / ATA mode,
    pub /: *mut *mut unsigned short setup; / t1,
    pub /: *mut *mut unsigned short act8b; / t2 for 8-bit I/O,
    pub /: *mut *mut unsigned short rec8b; / t2i for 8-bit I/O,
    pub /: *mut *mut unsigned short cyc8b; / t0 for 8-bit I/O,
    pub /: *mut *mut unsigned short active; / t2 or tD,
    pub /: *mut *mut unsigned short recover; / t2i or tK,
    pub /: *mut *mut unsigned short dmack_hold; / tj,
    pub /: *mut *mut unsigned short cycle; / t0,
    pub /: *mut *mut unsigned short udma; / t2CYCTYP/2,
}

//
// Core layer - drivers/ata/libata-core.c
//
extern "C" {
    pub fn ata_std_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int;
}
extern "C" {
    pub fn ata_std_postreset(link: *mut ata_link, classes: *mut c_uint);
}
extern "C" {
    pub fn ata_host_get(host: *mut ata_host);
}
extern "C" {
    pub fn ata_host_put(host: *mut ata_host);
}
extern "C" {
    pub fn ata_host_start(host: *mut ata_host) -> c_int;
}
extern "C" {
    pub fn ata_host_detach(host: *mut ata_host);
}
extern "C" {
    pub fn ata_host_init(: *mut ata_host, : *mut device, : *mut ata_port_operations);
}

extern "C" {
    pub fn ata_scsi_eh_timed_out(cmd: *mut scsi_cmnd) -> scsi_timeout_action;
}

extern "C" {
    pub fn ata_scsi_dma_need_drain(rq: *mut request) -> bool;
}

extern "C" {
    pub fn ata_link_online(link: *mut ata_link) -> bool;
}
extern "C" {
    pub fn ata_link_offline(link: *mut ata_link) -> bool;
}

extern "C" {
    pub fn ata_host_suspend(host: *mut ata_host, mesg: pm_message_t);
}
extern "C" {
    pub fn ata_host_resume(host: *mut ata_host);
}
extern "C" {
    pub fn ata_sas_port_suspend(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sas_port_resume(ap: *mut ata_port);
}

extern "C" {
    pub fn ata_ratelimit() -> c_int;
}
extern "C" {
    pub fn ata_msleep(ap: *mut ata_port, msecs: c_uint);
}
extern "C" {
    pub fn atapi_cmd_type(opcode: u8) -> c_int;
}
extern "C" {
    pub fn ata_xfer_mask2mode(xfer_mask: c_uint) -> u8;
}
extern "C" {
    pub fn ata_xfer_mode2mask(xfer_mode: u8) -> c_uint;
}
extern "C" {
    pub fn ata_xfer_mode2shift(xfer_mode: u8) -> c_int;
}
extern "C" {
    pub fn ata_id_xfermask(id: *const u16) -> c_uint;
}
extern "C" {
    pub fn ata_std_qc_defer(qc: *mut ata_queued_cmd) -> c_int;
}
extern "C" {
    pub fn ata_dev_classify(tf: *const ata_taskfile) -> c_uint;
}
extern "C" {
    pub fn ata_dev_disable(adev: *mut ata_device);
}
extern "C" {
    pub fn ata_qc_complete(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_qc_get_active(ap: *mut ata_port) -> u64;
}
extern "C" {
    pub fn ata_scsi_unlock_native_capacity(sdev: *mut scsi_device);
}
extern "C" {
    pub fn ata_scsi_sdev_init(sdev: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn ata_scsi_sdev_configure(sdev: *mut scsi_device, lim: *mut queue_limits) -> c_int;
}
extern "C" {
    pub fn ata_scsi_sdev_destroy(sdev: *mut scsi_device);
}
extern "C" {
    pub fn ata_set_mode(link: *mut ata_link, r_failed_dev: *mut ata_device) -> c_int;
}
extern "C" {
    pub fn ata_scsi_port_error_handler(host: *mut Scsi_Host, ap: *mut ata_port);
}
//
// SATA specific code - drivers/ata/libata-sata.c
//

extern "C" {
    pub fn sata_scr_valid(link: *mut ata_link) -> c_int;
}
extern "C" {
    pub fn sata_scr_read(link: *mut ata_link, reg: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn sata_scr_write(link: *mut ata_link, reg: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn sata_scr_write_flush(link: *mut ata_link, reg: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn sata_set_spd(link: *mut ata_link) -> c_int;
}
extern "C" {
    pub fn ata_eh_analyze_ncq_error(link: *mut ata_link);
}

// online = false;

extern "C" {
    pub fn ata_slave_link_init(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_port_probe(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_port_free(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_tport_add(parent: *mut device, ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_tport_delete(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_tf_from_fis(fis: *const u8, tf: *mut ata_taskfile);
}
extern "C" {
    pub fn ata_qc_complete_multiple(ap: *mut ata_port, qc_active: u64) -> c_int;
}
extern "C" {
    pub fn sata_lpm_ignore_phy_events(link: *mut ata_link) -> bool;
}
extern "C" {
    pub fn sata_async_notification(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_cable_40wire(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_cable_80wire(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_cable_sata(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_cable_ignore(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_cable_unknown(ap: *mut ata_port) -> c_int;
}
// Timing helpers
extern "C" {
    pub fn ata_pio_need_iordy(: *const ata_device) -> c_uint;
}
extern "C" {
    pub fn ata_timing_cycle2mode(xfer_shift: c_uint, cycle: c_int) -> u8;
}
// PCI

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bits {
    pub /: *mut *mut unsigned int reg; / PCI config register to read,
    pub /: *mut *mut unsigned int width; / 1 (8 bit), 2 (16 bit), 4 (32 bit),
    pub mask: c_ulong,
    pub val: c_ulong,
}

extern "C" {
    pub fn pci_test_config_bits(pdev: *mut pci_dev, bits: *const pci_bits) -> c_int;
}
extern "C" {
    pub fn ata_pci_shutdown_one(pdev: *mut pci_dev);
}
extern "C" {
    pub fn ata_pci_remove_one(pdev: *mut pci_dev);
}

extern "C" {
    pub fn ata_pci_device_do_suspend(pdev: *mut pci_dev, mesg: pm_message_t);
}
extern "C" {
    pub fn ata_pci_device_do_resume(pdev: *mut pci_dev) -> int __must_check;
}
extern "C" {
    pub fn ata_pci_device_suspend(pdev: *mut pci_dev, mesg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn ata_pci_device_resume(pdev: *mut pci_dev) -> c_int;
}

extern "C" {
    pub fn ata_platform_remove_one(pdev: *mut platform_device);
}
//
// ACPI - drivers/ata/libata-acpi.c
//

extern "C" {
    pub fn ata_acpi_stm(ap: *mut ata_port, stm: *const ata_acpi_gtm) -> c_int;
}
extern "C" {
    pub fn ata_acpi_gtm(ap: *mut ata_port, stm: *mut ata_acpi_gtm) -> c_int;
}
extern "C" {
    pub fn ata_acpi_cbl_pata_type(ap: *mut ata_port) -> c_int;
}

//
// EH - drivers/ata/libata-eh.c
//
extern "C" {
    pub fn ata_port_schedule_eh(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_port_wait_eh(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_link_abort(link: *mut ata_link) -> c_int;
}
extern "C" {
    pub fn ata_port_abort(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_port_freeze(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_eh_freeze_port(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_eh_thaw_port(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_std_sched_eh(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_std_end_eh(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_link_nr_enabled(link: *mut ata_link) -> c_int;
}
//
// Base operations to inherit from and initializers for sht
//
// Operations
//
// base  : Common to all libata drivers.
// sata  : SATA controllers w/ native interface.
// pmp   : SATA controllers w/ PMP support.
// sff   : SFF ATA controllers w/o BMDMA support.
// bmdma : SFF ATA controllers w/ BMDMA support.
//
// sht initializers
//
// BASE  : Common to all libata drivers.  The user must set
// sg_tablesize and dma_boundary.
// PIO   : SFF ATA controllers w/ only PIO support.
// BMDMA : SFF ATA controllers w/ BMDMA support.  sg_tablesize and
// dma_boundary are set to BMDMA limits.
// NCQ   : SATA controllers supporting NCQ.  The user must set
// sg_tablesize, dma_boundary and can_queue.
//
// All sht initializers (BASE, PIO, BMDMA, NCQ) must be instantiated
// by the edge drivers.  Because the 'module' field of sht must be the
// edge driver's module reference, otherwise the driver can be unloaded
// even if the scsi_device is being accessed.
//

//
// PMP helpers
//

//
// ata_eh_info helpers
//
extern "C" {
    pub fn __ata_ehi_push_desc(ehi: *mut ata_eh_info, fmt: *const c_char, ...);
}
extern "C" {
    pub fn ata_ehi_push_desc(ehi: *mut ata_eh_info, fmt: *const c_char, ...);
}
extern "C" {
    pub fn ata_ehi_clear_desc(ehi: *mut ata_eh_info);
}
//
// port description helpers
//
extern "C" {
    pub fn ata_port_desc(ap: *mut ata_port, fmt: *const c_char, ...);
}

//
// Internal use only, iterate commands ignoring error handling and
// status of 'qc'.
//

//
// Iterate all potential commands that can be queued
//

//
// Like ata_qc_for_each, but with the internal tag included
//

//
// device helpers
//
extern "C" {
    pub fn ata_class_enabled(_arg: dev->class) -> return;
}
extern "C" {
    pub fn ata_class_disabled(_arg: dev->class) -> return;
}
extern "C" {
    pub fn ata_class_absent(_arg: dev->class) -> return;
}
//
// link helpers
//
// Iterators
//
// ATA_LITER_* constants are used to select link iteration mode and
// ATA_DITER_* device iteration mode.
//
// For a custom iteration directly using ata_{link|dev}_next(), if
// @link or @dev, respectively, is NULL, the first element is
// returned.  @dev and @link can be any valid device or link and the
// next element according to the iteration mode will be returned.
// After the last element, NULL is returned.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_link_iter_mode {
    ATA_LITER_EDGE,		/* if present, PMP links only; otherwise,
// host link.  no slave link
    ATA_LITER_HOST_FIRST,	/* host link followed by PMP or slave links */
    ATA_LITER_PMP_FIRST,	/* PMP links followed by host link,
// slave link still comes after host link
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ata_dev_iter_mode {
    ATA_DITER_ENABLED,
    ATA_DITER_ENABLED_REVERSE,
    ATA_DITER_ALL,
    ATA_DITER_ALL_REVERSE,
}

//
// Shortcut notation for iterations
//
// ata_for_each_link() iterates over each link of @ap according to
// @mode.  @link points to the current link in the loop.  @link is
// NULL after loop termination.  ata_for_each_dev() works the same way
// except that it iterates over each device of @link.
//
// Note that the mode prefixes ATA_{L|D}ITER_ shouldn't need to be
// specified when using the following shorthand notations.  Only the
// mode itself (EDGE, HOST_FIRST, ENABLED, etc...) should be
// specified.  This not only increases brevity but also makes it
// impossible to use ATA_LITER_* for device iteration or vice-versa.
//

//
// ata_ncq_supported - Test whether NCQ is supported
// @dev: ATA device to test
//
// LOCKING:
// spin_lock_irqsave(host lock)
//
// RETURNS:
// true if @dev supports NCQ, false otherwise.
//
// ata_ncq_enabled - Test whether NCQ is enabled
// @dev: ATA device to test
//
// LOCKING:
// spin_lock_irqsave(host lock)
//
// RETURNS:
// true if NCQ is enabled for @dev, false otherwise.
//
extern "C" {
    pub fn ata_ncq_supported(ATA_DFLAG_NCQ_OFF: dev) && !(dev->flags &) -> return;
}

// init result_tf such that it indicates normal completion
// 0xff indicates either no device or device not ready
// Don't open code these in drivers as there are traps. Firstly the range may
//
// PATA timings - drivers/ata/libata-pata-timings.c
//
// PMP - drivers/ata/libata-pmp.c
//

extern "C" {
    pub fn sata_pmp_qc_defer_cmd_switch(qc: *mut ata_queued_cmd) -> c_int;
}

//
// SFF - drivers/ata/libata-sff.c
//

// PIO only, sg_tablesize and dma_boundary limits can be removed

extern "C" {
    pub fn ata_sff_dev_select(ap: *mut ata_port, device: c_uint);
}
extern "C" {
    pub fn ata_sff_check_status(ap: *mut ata_port) -> u8;
}
extern "C" {
    pub fn ata_sff_pause(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_dma_pause(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_wait_ready(link: *mut ata_link, deadline: c_ulong) -> c_int;
}
extern "C" {
    pub fn ata_sff_tf_load(ap: *mut ata_port, tf: *const ata_taskfile);
}
extern "C" {
    pub fn ata_sff_tf_read(ap: *mut ata_port, tf: *mut ata_taskfile);
}
extern "C" {
    pub fn ata_sff_irq_on(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_queue_work(work: *mut work_struct);
}
extern "C" {
    pub fn ata_sff_queue_pio_task(link: *mut ata_link, delay: c_ulong);
}
extern "C" {
    pub fn ata_sff_qc_issue(qc: *mut ata_queued_cmd) -> c_uint;
}
extern "C" {
    pub fn ata_sff_qc_fill_rtf(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_sff_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ata_sff_lost_interrupt(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_freeze(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_thaw(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_prereset(link: *mut ata_link, deadline: c_ulong) -> c_int;
}
extern "C" {
    pub fn ata_sff_postreset(link: *mut ata_link, classes: *mut c_uint);
}
extern "C" {
    pub fn ata_sff_drain_fifo(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_sff_std_ports(ioaddr: *mut ata_ioports);
}

extern "C" {
    pub fn ata_pci_sff_init_host(host: *mut ata_host) -> c_int;
}

extern "C" {
    pub fn ata_bmdma_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors;
}
extern "C" {
    pub fn ata_bmdma_qc_issue(qc: *mut ata_queued_cmd) -> c_uint;
}
extern "C" {
    pub fn ata_bmdma_dumb_qc_prep(qc: *mut ata_queued_cmd) -> ata_completion_errors;
}
extern "C" {
    pub fn ata_bmdma_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ata_bmdma_post_internal_cmd(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_bmdma_irq_clear(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_bmdma_setup(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_bmdma_start(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_bmdma_stop(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_bmdma_status(ap: *mut ata_port) -> u8;
}
extern "C" {
    pub fn ata_bmdma_port_start(ap: *mut ata_port) -> c_int;
}
extern "C" {
    pub fn ata_bmdma_port_start32(ap: *mut ata_port) -> c_int;
}

extern "C" {
    pub fn ata_pci_bmdma_clear_simplex(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn ata_pci_bmdma_init(host: *mut ata_host);
}

//
// ata_sff_busy_wait - Wait for a port status register
// @ap: Port to wait for.
// @bits: bits that must be clear
// @max: number of 10uS waits to perform
//
// Waits up to max*10 microseconds for the selected bits in the port's
// status register to be cleared.
// Returns final value of status register.
//
// LOCKING:
// Inherited from caller.
//
// ata_wait_idle - Wait for a port to be idle.
// @ap: Port to wait for.
//
// Waits up to 10ms for port's BUSY and DRQ signals to clear.
// Returns final value of status register.
//
// LOCKING:
// Inherited from caller.
//

