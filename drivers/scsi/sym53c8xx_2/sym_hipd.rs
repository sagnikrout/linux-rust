//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sym53c8xx_2/sym_hipd.h
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
// Device driver for the SYMBIOS/LSILOGIC 53C8XX and 53C1010 family
// of PCI-SCSI IO processors.
//
// Copyright (C) 1999-2001  Gerard Roudier <groudier@free.fr>
//
// This driver is derived from the Linux sym53c8xx driver.
// Copyright (C) 1998-2000  Gerard Roudier
//
// The sym53c8xx driver is derived from the ncr53c8xx driver that had been
// a port of the FreeBSD ncr driver to Linux-1.2.13.
//
// The original ncr driver has been written for 386bsd and FreeBSD by
// Wolfgang Stanglmeier        <wolf@cologne.de>
// Stefan Esser                <se@mi.Uni-Koeln.de>
// Copyright (C) 1994  Wolfgang Stanglmeier
//
// Other major contributions:
//
// NVRAM detection and reading.
// Copyright (C) 1997 Richard Waltham <dormouse@farsrobt.demon.co.uk>
//
// -----------------------------------------------------------------------------
//

//
// Generic driver options.
//
// They may be defined in platform specific headers, if they
// are useful.
//
// SYM_OPT_HANDLE_DEVICE_QUEUEING
// When this option is set, the driver will use a queue per
// device and handle QUEUE FULL status requeuing internally.
//
// SYM_OPT_LIMIT_COMMAND_REORDERING
// When this option is set, the driver tries to limit tagged
// command reordering to some reasonable value.
// (set for Linux)
//

// Macro flag: #define SYM_OPT_HANDLE_DEVICE_QUEUEING
// Macro flag: #define SYM_OPT_LIMIT_COMMAND_REORDERING

//
// Active debugging tags and verbosity.
// Both DEBUG_FLAGS and sym_verbose can be redefined
// by the platform specific code to something else.
//

//
// These ones should have been already defined.
//

//
// Number of tasks per device we want to handle.
//

//
// Donnot use more tasks that we can handle.
//

//
// This one means 'NO TAG for this job'
//

//
// Number of SCSI targets.
//

//
// Number of logical units per target.
//

//
// Asynchronous pre-scaler (ns). Shall be 40 for
// the SCSI timings to be compliant.
//

//
// MEMORY ALLOCATOR.
//

//
// Shortest memory chunk is (1<<SYM_MEM_SHIFT), currently 16.
// Actual allocations happen as SYM_MEM_CLUSTER_SIZE sized.
// (1 PAGE at a time is just fine).
//
pub const SYM_MEM_SHIFT: c_int = 4;

//
// Number of entries in the START and DONE queues.
//
// We limit to 1 PAGE in order to succeed allocation of
// these queues. Each entry is 8 bytes long (2 DWORDS).
//

//
// For this one, we want a short name :-)
//

//
// Common definitions for both bus space based and legacy IO methods.
//

//
// We normally want the chip to have a consistent view
// of driver internal data structures when we restart it.
// Thus these macros.
//

//
// Command control block states.
//

//
// Software Interrupt Codes
//

//
// Extended error bit codes.
// xerr_status field of struct sym_ccb.
//

//
// Negotiation status.
// nego_status field of struct sym_ccb.
//

//
// A CCB hashed table is used to retrieve CCB address
// from DSA value.
//
pub const CCB_HASH_SHIFT: c_int = 8;

//
// We may want to use segment registers for 64 bit DMA.
// 16 segments registers -> up to 64 GB addressable.
//

//
// Device flags.
//

//
// Host adapter miscellaneous flags.
//

//
// Misc.
//

pub const BUS_8_BIT: c_int = 0;
pub const BUS_16_BIT: c_int = 1;
//
// Gather negotiable parameters value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_trans {
    pub period: u8,
    pub offset: u8,
    pub width:1: c_uint,
    pub iu:1: c_uint,
    pub dt:1: c_uint,
    pub qas:1: c_uint,
    pub check_nego:1: c_uint,
    pub renego:2: c_uint,
}

//
// Global TCB HEADER.
//
// Due to lack of indirect addressing on earlier NCR chips,
// this substructure is copied from the TCB to a global
// address after selection.
// For SYMBIOS chips that support LOAD/STORE this copy is
// not needed and thus not performed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_tcbh {
//
// Scripts bus addresses of LUN table accessed from scripts.
// LUN #0 is a special case, since multi-lun devices are rare,
// and we we want to speed-up the general case and not waste
// resources.
//
    pub /: *mut *mut u32 luntbl_sa; / bus address of this table,
    pub /: *mut *mut u32 lun0_sa; / bus address of LCB #0,
//
// Actual SYNC/WIDE IO registers value for this target.
// 'sval', 'wval' and 'uval' are read from SCRIPTS and
// so have alignment constraints.
//
// 0*/	u_char	uval;		/* -> SCNTL4 register
// 1*/	u_char	sval;		/* -> SXFER  io register
// 2*/	u_char	filler1;
// 3*/	u_char	wval;		/* -> SCNTL3 io register
}

//
// Target Control Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_tcb {
//
// TCB header.
// Assumed at offset 0.
//
// 0*/	struct sym_tcbh head;
//
// LUN table used by the SCRIPTS processor.
// An array of bus addresses is used on reselection.
//
    pub /: *mut *mut *mut u32 luntbl; / LCBs bus address table,
    pub /: *mut *mut int nlcb; / Number of valid LCBs (including LUN #0),
//
// LUN table used by the C code.
//
    pub /: *mut *mut *mut sym_lcb lun0p; / LCB of LUN #0 (usual case),

    pub /: *mut *mut *mut *mut sym_lcb lunmp; / Other LCBs [1..MAX_LUN],

//
// O/S specific data structure.
//
    pub s: sym_stcb,

// Transfer goal
    pub tgoal: sym_trans,
// Last printed transfer speed
    pub tprint: sym_trans,
//
// Keep track of the CCB used for the negotiation in order
// to ensure that only 1 negotiation is queued at a time.
//
    pub /: *mut *mut *mut sym_ccb  nego_cp; / CCB used for the nego,
//
// Set when we want to reset the device.
//
    pub to_reset: u_char,
//
// Other user settable limits and options.
// These limits are read from the NVRAM if present.
//
    pub usrflags: c_uchar,
    pub usr_period: c_uchar,
    pub usr_width: c_uchar,
    pub usrtags: c_ushort,
    pub starget: *mut scsi_target,
}

//
// Global LCB HEADER.
//
// Due to lack of indirect addressing on earlier NCR chips,
// this substructure is copied from the LCB to a global
// address after selection.
// For SYMBIOS chips that support LOAD/STORE this copy is
// not needed and thus not performed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_lcbh {
//
// SCRIPTS address jumped by SCRIPTS on reselection.
// For not probed logical units, this address points to
// SCRIPTS that deal with bad LU handling (must be at
// offset zero of the LCB for that reason).
//
// 0*/	u32	resel_sa;
//
// Task (bus address of a CCB) read from SCRIPTS that points
// to the unique ITL nexus allowed to be disconnected.
//
    pub itl_task_sa: u32,
//
// Task table bus address (read from SCRIPTS).
//
    pub itlq_tbl_sa: u32,
}

//
// Logical Unit Control Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_lcb {
//
// TCB header.
// Assumed at offset 0.
//
// 0*/	struct sym_lcbh head;
//
// Task table read from SCRIPTS that contains pointers to
// ITLQ nexuses. The bus address read from SCRIPTS is
// inside the header.
//
    pub /: *mut *mut *mut u32 itlq_tbl; / Kernel virtual address,
//
// Busy CCBs management.
//
    pub /: *mut *mut u_short busy_itlq; / Number of busy tagged CCBs,
    pub /: *mut *mut u_short busy_itl; / Number of busy untagged CCBs,
//
// Circular tag allocation buffer.
//
    pub /: *mut *mut u_short ia_tag; / Tag allocation index,
    pub /: *mut *mut u_short if_tag; / Tag release index,
    pub /: *mut *mut *mut u_char cb_tags; / Circular tags buffer,
//
// O/S specific data structure.
//

    pub s: sym_slcb,

//
// Optionnaly the driver can handle device queueing,
// and requeues internally command to redo.
//
    pub waiting_ccbq: SYM_QUEHEAD,
    pub started_ccbq: SYM_QUEHEAD,
    pub num_sgood: c_int,
    pub started_tags: u_short,
    pub started_no_tag: u_short,
    pub started_max: u_short,
    pub started_limit: u_short,

//
// Optionally the driver can try to prevent SCSI
// IOs from being reordered too much.
//
    pub /: *mut *mut u_char tags_si; / Current index to tags sum,
    pub /: *mut *mut u_short tags_sum[2]; / Tags sum counters,
    pub /: *mut *mut u_short tags_since; / # of tags since last switch,

//
// Set when we want to clear all tasks.
//
    pub to_clear: u_char,
//
// Capabilities.
//
    pub user_flags: u_char,
    pub curr_flags: u_char,
}

//
// Action from SCRIPTS on a task.
// Is part of the CCB, but is also used separately to plug
// error handling action to perform from SCRIPTS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_actscr {
    pub /: *mut *mut u32 start; / Jumped by SCRIPTS after selection,
    pub /: *mut *mut u32 restart; / Jumped by SCRIPTS on relection,
}

//
// Phase mismatch context.
//
// It is part of the CCB and is used as parameters for the
// DATA pointer. We need two contexts to handle correctly the
// SAVED DATA POINTER.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_pmc {
    pub /: *mut *mut sym_tblmove sg; / Updated interrupted SG block,
    pub /: *mut *mut u32 ret; / SCRIPT return address,
}

//
// LUN control block lookup.
// We use a direct pointer for LUN #0, and a table of
// pointers which is only allocated for devices that support
// LUN(s) > 0.
//

//
// Status are used by the host and the script processor.
//
// The last four bytes (status[4]) are copied to the
// scratchb register (declared as scr0..scr3) just after the
// select/reselect, and copied back just after disconnecting.
// Inside the script the XX_REG are used.
//
// Last four bytes (script)
//

//
// Last four bytes (host)
//

//
// Host flags
//

//
// More host flags
//

//
// Global CCB HEADER.
//
// Due to lack of indirect addressing on earlier NCR chips,
// this substructure is copied from the ccb to a global
// address after selection (or reselection) and copied back
// before disconnect.
// For SYMBIOS chips that support LOAD/STORE this copy is
// not needed and thus not performed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_ccbh {
//
// Start and restart SCRIPTS addresses (must be at 0).
//
// 0*/	struct sym_actscr go;
//
// SCRIPTS jump address that deal with data pointers.
// 'savep' points to the position in the script responsible
// for the actual transfer of data.
// It's written on reception of a SAVE_DATA_POINTER message.
//
    pub /: *mut *mut u32 savep; / Jump address to saved data pointer,
    pub /: *mut *mut u32 lastp; / SCRIPTS address at end of data,
//
// Status fields.
//
    pub status: [u8; 4],
}

//
// GET/SET the value of the data pointer used by SCRIPTS.
//
// We must distinguish between the LOAD/STORE-based SCRIPTS
// that use directly the header in the CCB, and the NCR-GENERIC
// SCRIPTS that use the copy of the header in the HCB.
//

//
// Data Structure Block
//
// During execution of a ccb by the script processor, the
// DSA (data structure address) register points to this
// substructure of the ccb.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_dsb {
//
// CCB header.
// Also assumed at offset 0 of the sym_ccb structure.
//
// 0*/	struct sym_ccbh head;
//
// Phase mismatch contexts.
// We need two to handle correctly the SAVED DATA POINTER.
// MUST BOTH BE AT OFFSET < 256, due to using 8 bit arithmetic
// for address calculation from SCRIPTS.
//
    pub pm0: sym_pmc,
    pub pm1: sym_pmc,
//
// Table data for Script
//
    pub select: sym_tblsel,
    pub smsg: sym_tblmove,
    pub smsg_ext: sym_tblmove,
    pub cmd: sym_tblmove,
    pub sense: sym_tblmove,
    pub wresid: sym_tblmove,
    pub [SYM_CONF_MAX_SG]: sym_tblmove data,
}

//
// Our Command Control Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_ccb {
//
// This is the data structure which is pointed by the DSA
// register when it is executed by the script processor.
// It must be the first entry.
//
    pub phys: sym_dsb,
//
// Pointer to CAM ccb and related stuff.
//
    pub /: *mut *mut *mut scsi_cmnd cmd; / CAM scsiio ccb,
    pub /: *mut *mut u8 cdb_buf[16]; / Copy of CDB,
pub const SYM_SNS_BBUF_LEN: c_int = 32;
    pub /: *mut *mut u8 sns_bbuf[SYM_SNS_BBUF_LEN]; / Bounce buffer for sense data,
    pub /: *mut *mut int data_len; / Total data length,
    pub /: *mut *mut int segments; / Number of SG segments,
    pub /: *mut *mut u8 order; / Tag type (if tagged command),
    pub /: *mut *mut unsigned char odd_byte_adjustment; / odd-sized req on wide bus,
    pub /: *mut *mut u_char nego_status; / Negotiation status,
    pub /: *mut *mut u_char xerr_status; / Extended error flags,
    pub /: *mut *mut u32 extra_bytes; / Extraneous bytes transferred,
//
// Message areas.
// We prepare a message to be sent after selection.
// We may use a second one if the command is rescheduled
// due to CHECK_CONDITION or COMMAND TERMINATED.
// Contents are IDENTIFY and SIMPLE_TAG.
// While negotiating sync or wide transfer,
// a SDTR or WDTR message is appended.
//
    pub [12]: u_char scsi_smsg,
    pub scsi_smsg2: [u_char; 12],
//
// Auto request sense related fields.
//
    pub /: *mut *mut u_char sensecmd[6]; / Request Sense command,
    pub /: *mut *mut u_char sv_scsi_status; / Saved SCSI status,
    pub /: *mut *mut u_char sv_xerr_status; / Saved extended status,
    pub /: *mut *mut int sv_resid; / Saved residual,
//
// Other fields.
//
    pub /: *mut *mut u32 ccb_ba; / BUS address of this CCB,
    pub /: *mut *mut u_short tag; / Tag for this transfer,
// NO_TAG means no tag
    pub target: u_char,
    pub lun: u_char,
    pub /: *mut *mut *mut sym_ccb link_ccbh; / Host adapter CCB hash chain,
    pub /: *mut *mut SYM_QUEHEAD link_ccbq; / Link to free/busy CCB queue,
    pub /: *mut *mut u32 startp; / Initial data pointer,
    pub /: *mut *mut u32 goalp; / Expected last data pointer,
    pub /: *mut *mut int ext_sg; / Extreme data pointer, used,
    pub /: *mut *mut int ext_ofs; / to calculate the residual.,

    pub /: *mut *mut SYM_QUEHEAD link2_ccbq; / Link for device queueing,
    pub /: *mut *mut u_char started; / CCB queued to the squeue,

    pub /: *mut *mut u_char to_abort; / Want this IO to be aborted,

    pub /: *mut *mut u_char tags_si; / Lun tags sum index (0,1),

}

//
// Host Control Block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sym_hcb {
//
// Global headers.
// Due to poorness of addressing capabilities, earlier
// chips (810, 815, 825) copy part of the data structures
// (CCB, TCB and LCB) in fixed areas.
//

    pub ccb_head: sym_ccbh,
    pub tcb_head: sym_tcbh,
    pub lcb_head: sym_lcbh,

//
// Idle task and invalid task actions and
// their bus addresses.
//
    pub bad_itlq: sym_actscr idletask, notask, bad_itl,,
    pub bad_itlq_ba: u32 idletask_ba, notask_ba, bad_itl_ba,,
//
// Dummy lun table to protect us against target
// returning bad lun number on reselection.
//
    pub /: *mut *mut *mut u32 badluntbl; / Table physical address,
    pub /: *mut *mut u32 badlun_sa; / SCRIPT handler BUS address,
//
// Bus address of this host control block.
//
    pub hcb_ba: u32,
//
// Bit 32-63 of the on-chip RAM bus address in LE format.
// The START_RAM64 script loads the MMRS and MMWS from this
// field.
//
    pub scr_ram_seg: u32,
//
// Initial value of some IO register bits.
// These values are assumed to have been set by BIOS, and may
// be used to probe adapter implementation differences.
//
// Actual initial value of IO register bits used by the
// driver. They are loaded at initialisation according to
// features that are to be enabled/disabled.
//
    pub rv_scntl4: rv_ctest5, rv_stest2, rv_ccntl0, rv_ccntl1,,
//
// Target data.
//
    pub target: [sym_tcb; SYM_CONF_MAX_TARGET],
//
// Target control block bus address array used by the SCRIPT
// on reselection.
//
    pub targtbl: *mut u32,
    pub targtbl_ba: u32,
//
// DMA pool handle for this HBA.
//
    pub bus_dmat: m_pool_ident_t,
//
// O/S specific data structure
//
    pub s: sym_shcb,
//
// Physical bus addresses of the chip.
//
    pub /: *mut *mut u32 mmio_ba; / MMIO 32 bit BUS address,
    pub /: *mut *mut u32 ram_ba; / RAM 32 bit BUS address,
//
// SCRIPTS virtual and physical bus addresses.
// 'script'  is loaded in the on-chip RAM if present.
// 'scripth' stays in main memory for all chips except the
// 53C895A, 53C896 and 53C1010 that provide 8K on-chip RAM.
//
    pub /: *mut *mut *mut u_char scripta0; / Copy of scripts A, B, Z,
    pub scriptb0: *mut u_char,
    pub scriptz0: *mut u_char,
    pub /: *mut *mut u32 scripta_ba; / Actual scripts A, B, Z,
    pub /: *mut *mut u32 scriptb_ba; / 32 bit bus addresses.,
    pub scriptz_ba: u32,
    pub Z*/: *mut *mut u_short scripta_sz; / Actual size of script A, B,,
    pub scriptb_sz: u_short,
    pub scriptz_sz: u_short,
//
// Bus addresses, setup and patch methods for
// the selected firmware.
//
    pub /: *mut *mut sym_fwa_ba fwa_bas; / Useful SCRIPTA bus addresses,
    pub /: *mut *mut sym_fwb_ba fwb_bas; / Useful SCRIPTB bus addresses,
    pub /: *mut *mut sym_fwz_ba fwz_bas; / Useful SCRIPTZ bus addresses,
    pub fw): *mut *mut *mut void (fw_setup)(struct sym_hcb np, struct sym_fw,
    pub ): *mut *mut void (fw_patch)(struct Scsi_Host,
    pub fw_name: *mut c_char,
//
// General controller parameters and configuration.
//
    pub /: *mut *mut u_int features; / Chip features map,
    pub /: *mut *mut u_char myaddr; / SCSI id of the adapter,
    pub /: *mut *mut u_char maxburst; / log base 2 of dwords burst,
    pub /: *mut *mut u_char maxwide; / Maximum transfer width,
    pub /: *mut *mut u_char minsync; / Min sync period factor (ST),
    pub /: *mut *mut u_char maxsync; / Max sync period factor (ST),
    pub /: *mut *mut u_char maxoffs; / Max scsi offset (ST),
    pub /: *mut *mut u_char minsync_dt; / Min sync period factor (DT),
    pub /: *mut *mut u_char maxsync_dt; / Max sync period factor (DT),
    pub /: *mut *mut u_char maxoffs_dt; / Max scsi offset (DT),
    pub /: *mut *mut u_char multiplier; / Clock multiplier (1,2,4),
    pub /: *mut *mut u_char clock_divn; / Number of clock divisors,
    pub /: *mut *mut u32 clock_khz; / SCSI clock frequency in KHz,
    pub /: *mut *mut u32 pciclk_khz; / Estimated PCI clock in KHz,
//
// Start queue management.
// It is filled up by the host processor and accessed by the
// SCRIPTS processor in order to start SCSI commands.
//
    pub /: *mut *mut *mut u32 squeue; / Start queue virtual address,
    pub /: *mut *mut u32 squeue_ba; / Start queue BUS address,
    pub /: *mut *mut u_short squeueput; / Next free slot of the queue,
    pub /: *mut *mut u_short actccbs; / Number of allocated CCBs,
//
// Command completion queue.
// It is the same size as the start queue to avoid overflow.
//
    pub /: *mut *mut u_short dqueueget; / Next position to scan,
    pub /: *mut *mut *mut u32 dqueue; / Completion (done) queue,
    pub /: *mut *mut u32 dqueue_ba; / Done queue BUS address,
//
// Miscellaneous buffers accessed by the scripts-processor.
// They shall be DWORD aligned, because they may be read or
// written with a script command.
//
    pub /: *mut *mut u_char msgout[8]; / Buffer for MESSAGE OUT,
    pub /: *mut *mut u_char msgin [8]; / Buffer for MESSAGE IN,
    pub /: *mut *mut u32 lastmsg; / Last SCSI message sent,
    pub /: *mut *mut u32 scratch; / Scratch for SCSI receive,
// Also used for cache test
//
// Miscellaneous configuration and status parameters.
//
    pub /: *mut *mut u_char usrflags; / Miscellaneous user flags,
    pub /: *mut *mut u_char scsi_mode; / Current SCSI BUS mode,
    pub controller*/: *mut *mut u_char verbose; / Verbosity for this,
//
// CCB lists and queue.
//
    pub /: *mut *mut *mut *mut sym_ccb ccbh; / CCBs hashed by DSA value,
// CCB_HASH_SIZE lists of CCBs
    pub /: *mut *mut SYM_QUEHEAD free_ccbq; / Queue of available CCBs,
    pub /: *mut *mut SYM_QUEHEAD busy_ccbq; / Queue of busy CCBs,
//
// During error handling and/or recovery,
// active CCBs that are to be completed with
// error or requeued are moved from the busy_ccbq
// to the comp_ccbq prior to completion.
//
    pub comp_ccbq: SYM_QUEHEAD,

    pub dummy_ccbq: SYM_QUEHEAD,

//
// IMMEDIATE ARBITRATION (IARB) control.
//
// We keep track in 'last_cp' of the last CCB that has been
// queued to the SCRIPTS processor and clear 'last_cp' when
// this CCB completes. If last_cp is not zero at the moment
// we queue a new CCB, we set a flag in 'last_cp' that is
// used by the SCRIPTS as a hint for setting IARB.
// We donnot set more than 'iarb_max' consecutive hints for
// IARB in order to leave devices a chance to reselect.
// By the way, any non zero value of 'iarb_max' is unfair. :)
//

    pub hints*/: *mut *mut u_short iarb_max; / Max. # consecutive IARB,
    pub /: *mut *mut u_short iarb_count; / Actual # of these hints,
    pub last_cp: *mut *mut sym_ccb,

//
// Command abort handling.
// We need to synchronize tightly with the SCRIPTS
// processor in order to handle things correctly.
//
    pub /: *mut *mut u_char abrt_msg[4]; / Message to send buffer,
    pub /: *mut *mut sym_tblmove abrt_tbl; / Table for the MOV of it,
    pub /: *mut *mut sym_tblsel abrt_sel; / Sync params for selection,
    pub /: *mut *mut u_char istat_sem; / Tells the chip to stop (SEM),
//
// 64 bit DMA handling.
//

    pub /: *mut *mut u_char use_dac; / Use PCI DAC cycles,

    pub /: *mut *mut u_char dmap_dirty; / Dma segments registers dirty,
    pub /: *mut *mut u32 dmap_bah[SYM_DMAP_SIZE];/ Segment registers map,

}

pub const use_dac(np): c_int = 0;

//
// FIRMWARES (sym_fw.c)
//
extern "C" {
    pub fn sym_find_firmware(chip: *mut sym_chip) -> *mut sym_fw;
}
extern "C" {
    pub fn sym_fw_bind_script(np: *mut sym_hcb, start: *mut u32, len: c_int);
}
//
// Driver methods called from O/S specific code.
//
extern "C" {
    pub fn sym_print_xerr(cmd: *mut scsi_cmnd, x_status: c_int);
}
extern "C" {
    pub fn sym_reset_scsi_bus(np: *mut sym_hcb, enab_int: c_int) -> c_int;
}

extern "C" {
    pub fn sym_start_next_ccbs(np: *mut sym_hcb, lp: *mut sym_lcb, maxn: c_int);
}

extern "C" {
    pub fn sym_put_start_queue(np: *mut sym_hcb, cp: *mut sym_ccb);
}

extern "C" {
    pub fn sym_start_up(: *mut Scsi_Host, reason: c_int);
}
extern "C" {
    pub fn sym_interrupt(: *mut Scsi_Host) -> irqreturn_t;
}
extern "C" {
    pub fn sym_clear_tasks(np: *mut sym_hcb, cam_status: c_int, target: c_int, lun: c_int, task: c_int) -> c_int;
}
extern "C" {
    pub fn sym_free_ccb(np: *mut sym_hcb, cp: *mut sym_ccb);
}
extern "C" {
    pub fn sym_free_lcb(np: *mut sym_hcb, tn: u_char, ln: u_char) -> c_int;
}
extern "C" {
    pub fn sym_queue_scsiio(np: *mut sym_hcb, csio: *mut scsi_cmnd, cp: *mut sym_ccb) -> c_int;
}
extern "C" {
    pub fn sym_abort_scsiio(np: *mut sym_hcb, ccb: *mut scsi_cmnd, timed_out: c_int) -> c_int;
}
extern "C" {
    pub fn sym_reset_scsi_target(np: *mut sym_hcb, target: c_int) -> c_int;
}
extern "C" {
    pub fn sym_hcb_free(np: *mut sym_hcb);
}
extern "C" {
    pub fn sym_hcb_attach(shost: *mut Scsi_Host, fw: *mut sym_fw, nvram: *mut sym_nvram) -> c_int;
}
//
// Build a scatter/gather entry.
//
// For 64 bit systems, we use the 8 upper bits of the size field
// to provide bus address bits 32-39 to the SCRIPTS processor.
// This allows the 895A, 896, 1010 to address up to 1 TB of memory.
//

extern "C" {
    pub fn sym_lookup_dmap(np: *mut sym_hcb, h: u32, s: c_int) -> c_int;
}

//
// MEMORY ALLOCATOR.
//

//
// Link between free memory chunks of a given size.
//
// Virtual to bus physical translation for a given cluster.
// Such a structure is only useful with DMA abstraction.
//
// Hash this stuff a bit to speed up translations
pub const VTOB_HASH_SHIFT: c_int = 5;

//
// Memory pool of a given kind.
// Ideally, we want to use:
// 1) 1 pool for memory we donnot need to involve in DMA.
// 2) The same pool for controllers that require same DMA
// constraints and features.
// The OS specific m_pool_id_t thing and the sym_m_pool_match()
// method are expected to tell the driver about.
//

//
// Alloc, free and translate addresses to bus physical
// for DMAable memory.
//
extern "C" {
    pub fn __sym_mfree_dma(dev_dmat: m_pool_ident_t, m: *mut c_void, size: c_int, name: *mut c_char);
}
extern "C" {
    pub fn __vtobus(dev_dmat: m_pool_ident_t, m: *mut c_void) -> dma_addr_t;
}
//
// Verbs used by the driver code for DMAable memory handling.
// The _uvptv_ macro avoids a nasty warning about pointer to volatile
// being discarded.
//

//
// We have to provide the driver memory allocator with methods for
// it to maintain virtual to bus physical address translations.
//

