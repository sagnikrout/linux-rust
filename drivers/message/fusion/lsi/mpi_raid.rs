//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_raid.h
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
// Copyright (c) 2001-2008 LSI Corporation.
//
// Name:  mpi_raid.h
// Title:  MPI RAID message and structures
// Creation Date:  February 27, 2001
//
// mpi_raid.h Version:  01.05.05
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 02-27-01  01.01.01  Original release for this file.
// 03-27-01  01.01.02  Added structure offset comments.
// 08-08-01  01.02.01  Original release for v1.2 work.
// 09-28-01  01.02.02  Major rework for MPI v1.2 Integrated RAID changes.
// 10-04-01  01.02.03  Added ActionData defines for
// MPI_RAID_ACTION_DELETE_VOLUME action.
// 11-01-01  01.02.04  Added define for MPI_RAID_ACTION_ADATA_DO_NOT_SYNC.
// 03-14-02  01.02.05  Added define for MPI_RAID_ACTION_ADATA_LOW_LEVEL_INIT.
// 05-07-02  01.02.06  Added define for MPI_RAID_ACTION_ACTIVATE_VOLUME,
// MPI_RAID_ACTION_INACTIVATE_VOLUME, and
// MPI_RAID_ACTION_ADATA_INACTIVATE_ALL.
// 07-12-02  01.02.07  Added structures for Mailbox request and reply.
// 11-15-02  01.02.08  Added missing MsgContext field to MSG_MAILBOX_REQUEST.
// 04-01-03  01.02.09  New action data option flag for
// MPI_RAID_ACTION_DELETE_VOLUME.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Original release for MPI v1.5.
// 01-15-05  01.05.02  Added defines for the two new RAID Actions for
// _SET_RESYNC_RATE and _SET_DATA_SCRUB_RATE.
// 02-28-07  01.05.03  Added new RAID Action, Device FW Update Mode, and
// associated defines.
// 08-07-07  01.05.04  Added Disable Full Rebuild bit to the ActionDataWord
// for the RAID Action MPI_RAID_ACTION_DISABLE_VOLUME.
// 01-15-08  01.05.05  Added define for MPI_RAID_ACTION_SET_VOLUME_NAME.
// --------------------------------------------------------------------------
//
// R A I D    M e s s a g e s
//
// RAID Action Request
//
// RAID Action request Action values

// ActionDataWord defines for use with MPI_RAID_ACTION_CREATE_VOLUME action

// ActionDataWord defines for use with MPI_RAID_ACTION_DELETE_VOLUME action

// ActionDataWord defines for use with MPI_RAID_ACTION_DISABLE_VOLUME action

// ActionDataWord defines for use with MPI_RAID_ACTION_ACTIVATE_VOLUME action

// ActionDataWord defines for use with MPI_RAID_ACTION_SET_RESYNC_RATE action

// ActionDataWord defines for use with MPI_RAID_ACTION_SET_DATA_SCRUB_RATE action

// ActionDataWord defines for use with MPI_RAID_ACTION_DEVICE_FW_UPDATE_MODE action

// RAID Action reply message
// RAID Volume reply ActionStatus values

// RAID Volume reply RAID Volume Indicator structure
//
// SCSI IO RAID Passthrough Request
//
// SCSI IO RAID Passthrough reply structure
//
// Mailbox reqeust structure
//
// Mailbox reply structure
