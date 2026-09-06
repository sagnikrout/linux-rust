//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/scsw.h
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
// Helper functions for scsw access.
//
// Copyright IBM Corp. 2008, 2012
// Author(s): Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

//
// struct cmd_scsw - command-mode subchannel status word
// @key: subchannel key
// @sctl: suspend control
// @eswf: esw format
// @cc: deferred condition code
// @fmt: format
// @pfch: prefetch
// @isic: initial-status interruption control
// @alcc: address-limit checking control
// @ssi: suppress-suspended interruption
// @zcc: zero condition code
// @ectl: extended control
// @pno: path not operational
// @res: reserved
// @fctl: function control
// @actl: activity control
// @stctl: status control
// @cpa: channel program address
// @dstat: device status
// @cstat: subchannel status
// @count: residual count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_scsw {
    pub 4: __u32 key :,
    pub 1: __u32 sctl :,
    pub 1: __u32 eswf :,
    pub 2: __u32 cc :,
    pub 1: __u32 fmt :,
    pub 1: __u32 pfch :,
    pub 1: __u32 isic :,
    pub 1: __u32 alcc :,
    pub 1: __u32 ssi :,
    pub 1: __u32 zcc :,
    pub 1: __u32 ectl :,
    pub 1: __u32 pno :,
    pub 1: __u32 res :,
    pub 3: __u32 fctl :,
    pub 7: __u32 actl :,
    pub 5: __u32 stctl :,
    pub cpa: dma32_t,
    pub 8: __u32 dstat :,
    pub 8: __u32 cstat :,
    pub 16: __u32 count :,
// C attribute field omitted
//
// struct tm_scsw - transport-mode subchannel status word
// @key: subchannel key
// @eswf: esw format
// @cc: deferred condition code
// @fmt: format
// @x: IRB-format control
// @q: interrogate-complete
// @ectl: extended control
// @pno: path not operational
// @fctl: function control
// @actl: activity control
// @stctl: status control
// @tcw: TCW address
// @dstat: device status
// @cstat: subchannel status
// @fcxs: FCX status
// @schxs: subchannel-extended status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm_scsw {
    pub key:4: u32,
    pub :1: u32,
    pub eswf:1: u32,
    pub cc:2: u32,
    pub fmt:3: u32,
    pub x:1: u32,
    pub q:1: u32,
    pub :1: u32,
    pub ectl:1: u32,
    pub pno:1: u32,
    pub :1: u32,
    pub fctl:3: u32,
    pub actl:7: u32,
    pub stctl:5: u32,
    pub tcw: dma32_t,
    pub dstat:8: u32,
    pub cstat:8: u32,
    pub fcxs:8: u32,
    pub ifob:1: u32,
    pub sesq:7: u32,
// C attribute field omitted
//
// struct eadm_scsw - subchannel status word for eadm subchannels
// @key: subchannel key
// @eswf: esw format
// @cc: deferred condition code
// @ectl: extended control
// @fctl: function control
// @actl: activity control
// @stctl: status control
// @aob: AOB address
// @dstat: device status
// @cstat: subchannel status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eadm_scsw {
    pub key:4: u32,
    pub eswf:1: u32,
    pub cc:2: u32,
    pub ectl:1: u32,
    pub fctl:3: u32,
    pub actl:7: u32,
    pub stctl:5: u32,
    pub aob: dma32_t,
    pub dstat:8: u32,
    pub cstat:8: u32,
    pub __packed: },
//
// union scsw - subchannel status word
// @cmd: command-mode SCSW
// @tm: transport-mode SCSW
// @eadm: eadm SCSW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union scsw {
    pub cmd: cmd_scsw,
    pub tm: tm_scsw,
    pub eadm: eadm_scsw,
    pub __packed: },
pub const SCSW_FCTL_CLEAR_FUNC: c_uint = 0x1;
pub const SCSW_FCTL_HALT_FUNC: c_uint = 0x2;
pub const SCSW_FCTL_START_FUNC: c_uint = 0x4;
pub const SCSW_ACTL_SUSPENDED: c_uint = 0x1;
pub const SCSW_ACTL_DEVACT: c_uint = 0x2;
pub const SCSW_ACTL_SCHACT: c_uint = 0x4;
pub const SCSW_ACTL_CLEAR_PEND: c_uint = 0x8;
pub const SCSW_ACTL_HALT_PEND: c_uint = 0x10;
pub const SCSW_ACTL_START_PEND: c_uint = 0x20;
pub const SCSW_ACTL_RESUME_PEND: c_uint = 0x40;
pub const SCSW_STCTL_STATUS_PEND: c_uint = 0x1;
pub const SCSW_STCTL_SEC_STATUS: c_uint = 0x2;
pub const SCSW_STCTL_PRIM_STATUS: c_uint = 0x4;
pub const SCSW_STCTL_INTER_STATUS: c_uint = 0x8;
pub const SCSW_STCTL_ALERT_STATUS: c_uint = 0x10;
pub const DEV_STAT_ATTENTION: c_uint = 0x80;
pub const DEV_STAT_STAT_MOD: c_uint = 0x40;
pub const DEV_STAT_CU_END: c_uint = 0x20;
pub const DEV_STAT_BUSY: c_uint = 0x10;
pub const DEV_STAT_CHN_END: c_uint = 0x08;
pub const DEV_STAT_DEV_END: c_uint = 0x04;
pub const DEV_STAT_UNIT_CHECK: c_uint = 0x02;
pub const DEV_STAT_UNIT_EXCEP: c_uint = 0x01;
pub const SCHN_STAT_PCI: c_uint = 0x80;
pub const SCHN_STAT_INCORR_LEN: c_uint = 0x40;
pub const SCHN_STAT_PROG_CHECK: c_uint = 0x20;
pub const SCHN_STAT_PROT_CHECK: c_uint = 0x10;
pub const SCHN_STAT_CHN_DATA_CHK: c_uint = 0x08;
pub const SCHN_STAT_CHN_CTRL_CHK: c_uint = 0x04;
pub const SCHN_STAT_INTF_CTRL_CHK: c_uint = 0x02;
pub const SCHN_STAT_CHAIN_CHECK: c_uint = 0x01;
pub const SCSW_SESQ_DEV_NOFCX: c_int = 3;
pub const SCSW_SESQ_PATH_NOFCX: c_int = 4;
//
// architectured values for first sense byte
//
pub const SNS0_CMD_REJECT: c_uint = 0x80;

pub const SNS0_INTERVENTION_REQ: c_uint = 0x40;
pub const SNS0_BUS_OUT_CHECK: c_uint = 0x20;
pub const SNS0_EQUIPMENT_CHECK: c_uint = 0x10;
pub const SNS0_DATA_CHECK: c_uint = 0x08;
pub const SNS0_OVERRUN: c_uint = 0x04;
pub const SNS0_INCOMPL_DOMAIN: c_uint = 0x01;
//
// architectured values for second sense byte
//
pub const SNS1_PERM_ERR: c_uint = 0x80;
pub const SNS1_INV_TRACK_FORMAT: c_uint = 0x40;
pub const SNS1_EOC: c_uint = 0x20;
pub const SNS1_MESSAGE_TO_OPER: c_uint = 0x10;
pub const SNS1_NO_REC_FOUND: c_uint = 0x08;
pub const SNS1_FILE_PROTECTED: c_uint = 0x04;
pub const SNS1_WRITE_INHIBITED: c_uint = 0x02;
pub const SNS1_INPRECISE_END: c_uint = 0x01;
//
// architectured values for third sense byte
//
pub const SNS2_REQ_INH_WRITE: c_uint = 0x80;
pub const SNS2_CORRECTABLE: c_uint = 0x40;
pub const SNS2_FIRST_LOG_ERR: c_uint = 0x20;
pub const SNS2_ENV_DATA_PRESENT: c_uint = 0x10;
pub const SNS2_INPRECISE_END: c_uint = 0x04;
//
// architectured values for PPRC errors
//
pub const SNS7_INVALID_ON_SEC: c_uint = 0x0e;
//
// scsw_is_tm - check for transport mode scsw
// @scsw: pointer to scsw
//
// Return non-zero if the specified scsw is a transport mode scsw, zero
// otherwise.
//
    pub 1): return css_general_characteristics.fcx && (scsw->tm.x ==,
//
// scsw_key - return scsw key field
// @scsw: pointer to scsw
//
// Return the value of the key field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.key: return,
    pub scsw->cmd.key: return,
//
// scsw_eswf - return scsw eswf field
// @scsw: pointer to scsw
//
// Return the value of the eswf field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.eswf: return,
    pub scsw->cmd.eswf: return,
//
// scsw_cc - return scsw cc field
// @scsw: pointer to scsw
//
// Return the value of the cc field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.cc: return,
    pub scsw->cmd.cc: return,
//
// scsw_ectl - return scsw ectl field
// @scsw: pointer to scsw
//
// Return the value of the ectl field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.ectl: return,
    pub scsw->cmd.ectl: return,
//
// scsw_pno - return scsw pno field
// @scsw: pointer to scsw
//
// Return the value of the pno field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.pno: return,
    pub scsw->cmd.pno: return,
//
// scsw_fctl - return scsw fctl field
// @scsw: pointer to scsw
//
// Return the value of the fctl field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.fctl: return,
    pub scsw->cmd.fctl: return,
//
// scsw_actl - return scsw actl field
// @scsw: pointer to scsw
//
// Return the value of the actl field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.actl: return,
    pub scsw->cmd.actl: return,
//
// scsw_stctl - return scsw stctl field
// @scsw: pointer to scsw
//
// Return the value of the stctl field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.stctl: return,
    pub scsw->cmd.stctl: return,
//
// scsw_dstat - return scsw dstat field
// @scsw: pointer to scsw
//
// Return the value of the dstat field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.dstat: return,
    pub scsw->cmd.dstat: return,
//
// scsw_cstat - return scsw cstat field
// @scsw: pointer to scsw
//
// Return the value of the cstat field of the specified scsw, regardless of
// whether it is a transport mode or command mode scsw.
//
    pub scsw->tm.cstat: return,
    pub scsw->cmd.cstat: return,
//
// scsw_cmd_is_valid_key - check key field validity
// @scsw: pointer to scsw
//
// Return non-zero if the key field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->cmd.fctl &,
//
// scsw_cmd_is_valid_sctl - check sctl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the sctl field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->cmd.fctl &,
//
// scsw_cmd_is_valid_eswf - check eswf field validity
// @scsw: pointer to scsw
//
// Return non-zero if the eswf field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_STCTL_STATUS_PEND): return (scsw->cmd.stctl &,
//
// scsw_cmd_is_valid_cc - check cc field validity
// @scsw: pointer to scsw
//
// Return non-zero if the cc field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_STCTL_STATUS_PEND): (scsw->cmd.stctl &,
//
// scsw_cmd_is_valid_fmt - check fmt field validity
// @scsw: pointer to scsw
//
// Return non-zero if the fmt field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->cmd.fctl &,
//
// scsw_cmd_is_valid_pfch - check pfch field validity
// @scsw: pointer to scsw
//
// Return non-zero if the pfch field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->cmd.fctl &,
//
// scsw_cmd_is_valid_isic - check isic field validity
// @scsw: pointer to scsw
//
// Return non-zero if the isic field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->cmd.fctl &,
//
// scsw_cmd_is_valid_alcc - check alcc field validity
// @scsw: pointer to scsw
//
// Return non-zero if the alcc field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->cmd.fctl &,
//
// scsw_cmd_is_valid_ssi - check ssi field validity
// @scsw: pointer to scsw
//
// Return non-zero if the ssi field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->cmd.fctl &,
//
// scsw_cmd_is_valid_zcc - check zcc field validity
// @scsw: pointer to scsw
//
// Return non-zero if the zcc field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub SCSW_STCTL_INTER_STATUS): (scsw->cmd.stctl &,
//
// scsw_cmd_is_valid_ectl - check ectl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the ectl field of the specified command mode scsw is
// valid, zero otherwise.
//
// Must be status pending.
    pub 0: return,
// Must have alert status.
    pub 0: return,
// Must be alone or together with primary, secondary or both,
// => no intermediate status.
//
    pub 0: return,
    pub 1: return,
//
// scsw_cmd_is_valid_pno - check pno field validity
// @scsw: pointer to scsw
//
// Return non-zero if the pno field of the specified command mode scsw is
// valid, zero otherwise.
//
// Must indicate at least one I/O function.
    pub 0: return,
// Must be status pending.
    pub 0: return,
// Can be status pending alone, or with any combination of primary,
// secondary and alert => no intermediate status.
//
    pub 1: return,
// If intermediate, must be suspended.
    pub 1: return,
    pub 0: return,
//
// scsw_cmd_is_valid_fctl - check fctl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the fctl field of the specified command mode scsw is
// valid, zero otherwise.
//
// Only valid if pmcw.dnv == 1
    pub 1: return,
//
// scsw_cmd_is_valid_actl - check actl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the actl field of the specified command mode scsw is
// valid, zero otherwise.
//
// Only valid if pmcw.dnv == 1
    pub 1: return,
//
// scsw_cmd_is_valid_stctl - check stctl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the stctl field of the specified command mode scsw is
// valid, zero otherwise.
//
// Only valid if pmcw.dnv == 1
    pub 1: return,
//
// scsw_cmd_is_valid_dstat - check dstat field validity
// @scsw: pointer to scsw
//
// Return non-zero if the dstat field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub 3): (scsw->cmd.cc !=,
//
// scsw_cmd_is_valid_cstat - check cstat field validity
// @scsw: pointer to scsw
//
// Return non-zero if the cstat field of the specified command mode scsw is
// valid, zero otherwise.
//
    pub 3): (scsw->cmd.cc !=,
//
// scsw_tm_is_valid_key - check key field validity
// @scsw: pointer to scsw
//
// Return non-zero if the key field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub SCSW_FCTL_START_FUNC): return (scsw->tm.fctl &,
//
// scsw_tm_is_valid_eswf - check eswf field validity
// @scsw: pointer to scsw
//
// Return non-zero if the eswf field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub SCSW_STCTL_STATUS_PEND): return (scsw->tm.stctl &,
//
// scsw_tm_is_valid_cc - check cc field validity
// @scsw: pointer to scsw
//
// Return non-zero if the cc field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub SCSW_STCTL_STATUS_PEND): (scsw->tm.stctl &,
//
// scsw_tm_is_valid_fmt - check fmt field validity
// @scsw: pointer to scsw
//
// Return non-zero if the fmt field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub 1: return,
//
// scsw_tm_is_valid_x - check x field validity
// @scsw: pointer to scsw
//
// Return non-zero if the x field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub 1: return,
//
// scsw_tm_is_valid_q - check q field validity
// @scsw: pointer to scsw
//
// Return non-zero if the q field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub 1: return,
//
// scsw_tm_is_valid_ectl - check ectl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the ectl field of the specified transport mode scsw is
// valid, zero otherwise.
//
// Must be status pending.
    pub 0: return,
// Must have alert status.
    pub 0: return,
// Must be alone or together with primary, secondary or both,
// => no intermediate status.
//
    pub 0: return,
    pub 1: return,
//
// scsw_tm_is_valid_pno - check pno field validity
// @scsw: pointer to scsw
//
// Return non-zero if the pno field of the specified transport mode scsw is
// valid, zero otherwise.
//
// Must indicate at least one I/O function.
    pub 0: return,
// Must be status pending.
    pub 0: return,
// Can be status pending alone, or with any combination of primary,
// secondary and alert => no intermediate status.
//
    pub 1: return,
// If intermediate, must be suspended.
    pub 1: return,
    pub 0: return,
//
// scsw_tm_is_valid_fctl - check fctl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the fctl field of the specified transport mode scsw is
// valid, zero otherwise.
//
// Only valid if pmcw.dnv == 1
    pub 1: return,
//
// scsw_tm_is_valid_actl - check actl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the actl field of the specified transport mode scsw is
// valid, zero otherwise.
//
// Only valid if pmcw.dnv == 1
    pub 1: return,
//
// scsw_tm_is_valid_stctl - check stctl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the stctl field of the specified transport mode scsw is
// valid, zero otherwise.
//
// Only valid if pmcw.dnv == 1
    pub 1: return,
//
// scsw_tm_is_valid_dstat - check dstat field validity
// @scsw: pointer to scsw
//
// Return non-zero if the dstat field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub 3): (scsw->tm.cc !=,
//
// scsw_tm_is_valid_cstat - check cstat field validity
// @scsw: pointer to scsw
//
// Return non-zero if the cstat field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub 3): (scsw->tm.cc !=,
//
// scsw_tm_is_valid_fcxs - check fcxs field validity
// @scsw: pointer to scsw
//
// Return non-zero if the fcxs field of the specified transport mode scsw is
// valid, zero otherwise.
//
    pub 1: return,
//
// scsw_tm_is_valid_schxs - check schxs field validity
// @scsw: pointer to scsw
//
// Return non-zero if the schxs field of the specified transport mode scsw is
// valid, zero otherwise.
//
// scsw_is_valid_actl - check actl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the actl field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_actl(scsw): return,
    pub scsw_cmd_is_valid_actl(scsw): return,
//
// scsw_is_valid_cc - check cc field validity
// @scsw: pointer to scsw
//
// Return non-zero if the cc field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_cc(scsw): return,
    pub scsw_cmd_is_valid_cc(scsw): return,
//
// scsw_is_valid_cstat - check cstat field validity
// @scsw: pointer to scsw
//
// Return non-zero if the cstat field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_cstat(scsw): return,
    pub scsw_cmd_is_valid_cstat(scsw): return,
//
// scsw_is_valid_dstat - check dstat field validity
// @scsw: pointer to scsw
//
// Return non-zero if the dstat field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_dstat(scsw): return,
    pub scsw_cmd_is_valid_dstat(scsw): return,
//
// scsw_is_valid_ectl - check ectl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the ectl field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_ectl(scsw): return,
    pub scsw_cmd_is_valid_ectl(scsw): return,
//
// scsw_is_valid_eswf - check eswf field validity
// @scsw: pointer to scsw
//
// Return non-zero if the eswf field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_eswf(scsw): return,
    pub scsw_cmd_is_valid_eswf(scsw): return,
//
// scsw_is_valid_fctl - check fctl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the fctl field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_fctl(scsw): return,
    pub scsw_cmd_is_valid_fctl(scsw): return,
//
// scsw_is_valid_key - check key field validity
// @scsw: pointer to scsw
//
// Return non-zero if the key field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_key(scsw): return,
    pub scsw_cmd_is_valid_key(scsw): return,
//
// scsw_is_valid_pno - check pno field validity
// @scsw: pointer to scsw
//
// Return non-zero if the pno field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_pno(scsw): return,
    pub scsw_cmd_is_valid_pno(scsw): return,
//
// scsw_is_valid_stctl - check stctl field validity
// @scsw: pointer to scsw
//
// Return non-zero if the stctl field of the specified scsw is valid,
// regardless of whether it is a transport mode or command mode scsw.
// Return zero if the field does not contain a valid value.
//
    pub scsw_tm_is_valid_stctl(scsw): return,
    pub scsw_cmd_is_valid_stctl(scsw): return,
//
// scsw_cmd_is_solicited - check for solicited scsw
// @scsw: pointer to scsw
//
// Return non-zero if the command mode scsw indicates that the associated
// status condition is solicited, zero if it is unsolicited.
//
    pub SCSW_STCTL_ALERT_STATUS)): (SCSW_STCTL_STATUS_PEND |,
//
// scsw_tm_is_solicited - check for solicited scsw
// @scsw: pointer to scsw
//
// Return non-zero if the transport mode scsw indicates that the associated
// status condition is solicited, zero if it is unsolicited.
//
    pub SCSW_STCTL_ALERT_STATUS)): (SCSW_STCTL_STATUS_PEND |,
//
// scsw_is_solicited - check for solicited scsw
// @scsw: pointer to scsw
//
// Return non-zero if the transport or command mode scsw indicates that the
// associated status condition is solicited, zero if it is unsolicited.
//
    pub scsw_tm_is_solicited(scsw): return,
    pub scsw_cmd_is_solicited(scsw): return,
