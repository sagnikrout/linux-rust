//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_raid.h
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
// Copyright 2000-2020 Broadcom Inc. All rights reserved.
//
// Name:  mpi2_raid.h
// Title:  MPI Integrated RAID messages and structures
// Creation Date:  April 26, 2007
//
// mpi2_raid.h Version:  02.00.11
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 04-30-07  02.00.00  Corresponds to Fusion-MPT MPI Specification Rev A.
// 08-31-07  02.00.01  Modifications to RAID Action request and reply,
// including the Actions and ActionData.
// 02-29-08  02.00.02  Added MPI2_RAID_ACTION_ADATA_DISABL_FULL_REBUILD.
// 05-21-08  02.00.03  Added MPI2_RAID_VOL_CREATION_NUM_PHYSDISKS so that
// the PhysDisk array in MPI2_RAID_VOLUME_CREATION_STRUCT
// can be sized by the build environment.
// 07-30-09  02.00.04  Added proper define for the Use Default Settings bit of
// VolumeCreationFlags and marked the old one as obsolete.
// 05-12-10  02.00.05  Added MPI2_RAID_VOL_FLAGS_OP_MDC define.
// 08-24-10  02.00.06  Added MPI2_RAID_ACTION_COMPATIBILITY_CHECK along with
// related structures and defines.
// Added product-specific range to RAID Action values.
// 11-18-11  02.00.07  Incorporating additions for MPI v2.5.
// 02-06-12  02.00.08  Added MPI2_RAID_ACTION_PHYSDISK_HIDDEN.
// 07-26-12  02.00.09  Added ElapsedSeconds field to MPI2_RAID_VOL_INDICATOR.
// Added MPI2_RAID_VOL_FLAGS_ELAPSED_SECONDS_VALID define.
// 04-17-13  02.00.10  Added MPI25_RAID_ACTION_ADATA_ALLOW_PI.
// 11-18-14  02.00.11  Updated copyright information.
// --------------------------------------------------------------------------
//
// Integrated RAID Messages
//
// RAID Action messages
//
// ActionDataWord defines for use with MPI2_RAID_ACTION_CREATE_VOLUME action

// ActionDataWord defines for use with MPI2_RAID_ACTION_DELETE_VOLUME action

// use MPI2_RAIDVOL0_SETTING_ defines from mpi2_cnfg.h for
// MPI2_RAID_ACTION_CHANGE_VOL_WRITE_CACHE action
// ActionDataWord defines for use with
// MPI2_RAID_ACTION_DISABLE_ALL_VOLUMES action

// ActionDataWord for MPI2_RAID_ACTION_SET_RAID_FUNCTION_RATE Action

// ActionDataWord for MPI2_RAID_ACTION_START_RAID_FUNCTION Action
// PTR_MPI2_RAID_ACTION_START_RAID_FUNCTION,
// pMpi2RaidActionStartRaidFunction_t;
// defines for the RAIDFunction field

// defines for the Flags field

// ActionDataWord for MPI2_RAID_ACTION_STOP_RAID_FUNCTION Action
// PTR_MPI2_RAID_ACTION_STOP_RAID_FUNCTION,
// pMpi2RaidActionStopRaidFunction_t;
// defines for the RAIDFunction field

// defines for the Flags field

// ActionDataWord for MPI2_RAID_ACTION_CREATE_HOT_SPARE Action
// ActionDataWord for MPI2_RAID_ACTION_DEVICE_FW_UPDATE_MODE Action
// PTR_MPI2_RAID_ACTION_FW_UPDATE_MODE,
// pMpi2RaidActionFwUpdateMode_t;
// ActionDataWord defines for use with
// MPI2_RAID_ACTION_DEVICE_FW_UPDATE_MODE action

// RAID Action Request Message
// RAID Action request Action values

// RAID Volume Creation Structure
//
// The following define can be customized for the targeted product.
//

// defines for the PhysDiskMap field

// PTR_MPI2_RAID_VOLUME_CREATION_STRUCT,
// pMpi2RaidVolumeCreationStruct_t;
// use MPI2_RAID_VOL_TYPE_ defines from mpi2_cnfg.h for VolumeType
// defines for the VolumeCreationFlags field

// The following is an obsolete define.
// It must be shifted left 24 bits in order to set the proper bit.
//

// RAID Online Capacity Expansion Structure
// PTR_MPI2_RAID_ONLINE_CAPACITY_EXPANSION,
// pMpi2RaidOnlineCapacityExpansion_t;
// RAID Compatibility Input Structure
// PTR_MPI2_RAID_COMPATIBILITY_INPUT_STRUCT,
// pMpi2RaidCompatibilityInputStruct_t;
// defines for RAID Compatibility Structure Flags field

// RAID Volume Indicator Structure
// defines for RAID Volume Indicator Flags field

// RAID Compatibility Result Structure
// PTR_MPI2_RAID_COMPATIBILITY_RESULT_STRUCT,
// pMpi2RaidCompatibilityResultStruct_t;
// defines for RAID Compatibility Result Structure State field

// defines for RAID Compatibility Result Structure GenericAttributes field

// RAID Action Reply ActionData union
// use MPI2_RAIDVOL0_SETTING_ defines from mpi2_cnfg.h for
// MPI2_RAID_ACTION_CHANGE_VOL_WRITE_CACHE action
// RAID Action Reply Message
