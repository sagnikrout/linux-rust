//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/pm8001/pm8001_defs.h
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


//
// PMC-Sierra 8001/8081/8088/8089 SAS/SATA based host adapters driver
//
// Copyright (c) 2008-2009 USI Co., Ltd.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chip_flavors {
    chip_8001,
    chip_8008,
    chip_8009,
    chip_8018,
    chip_8019,
    chip_8074,
    chip_8076,
    chip_8077,
    chip_8006,
    chip_8070,
    chip_8072
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_speed {
    PHY_SPEED_15 = 0x01,
    PHY_SPEED_30 = 0x02,
    PHY_SPEED_60 = 0x04,
    PHY_SPEED_120 = 0x08,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_direction {
    DATA_DIR_NONE = 0x0,	/* NO TRANSFER */
    DATA_DIR_IN = 0x01,	/* INBOUND */
    DATA_DIR_OUT = 0x02,	/* OUTBOUND */
    DATA_DIR_BYRECIPIENT = 0x04, /* UNSPECIFIED */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_type {
    PORT_TYPE_SAS = (1L << 1),
    PORT_TYPE_SATA = (1L << 0),
}

// driver compile-time configuration

pub const PM8001_MAX_INB_NUM: c_int = 64;
pub const PM8001_MAX_OUTB_NUM: c_int = 64;

// Inbound/Outbound queue size
pub const IOMB_SIZE_SPC: c_int = 64;
pub const IOMB_SIZE_SPCV: c_int = 128;
// unchangeable hardware details

pub const PM8001_RESERVE_SLOT: c_int = 128;
pub const PM8001_SECTOR_SIZE: c_int = 512;
pub const PM8001_PAGE_SIZE_4K: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum memory_region_num {
    AAP1 = 0x0, /* application acceleration processor */
    IOP,	    /* IO processor */
    NVMD,	    /* NVM device */
    FW_FLASH,    /* memory for fw flash update */
    FORENSIC_MEM,  /* memory for fw forensic data */
    USI_MAX_MEMCNT_BASE
}

//
// maximum DMA memory regions(number of IBQ + number of IBQ CI
// + number of  OBQ + number of OBQ PI)
//

// error code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpi_err {
    MPI_IO_STATUS_SUCCESS = 0x0,
    MPI_IO_STATUS_BUSY = 0x01,
    MPI_IO_STATUS_FAIL = 0x02,
}

//
// Phy Control constants
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_control_type {
    PHY_LINK_RESET = 0x01,
    PHY_HARD_RESET = 0x02,
    PHY_NOTIFY_ENABLE_SPINUP = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm8001_hba_info_flags {
    PM8001F_INIT_TIME	= (1U << 0),
    PM8001F_RUN_TIME	= (1U << 1),
}

//
// Phy Status
//
pub const PHY_LINK_DISABLE: c_uint = 0x00;
pub const PHY_LINK_DOWN: c_uint = 0x01;
pub const PHY_STATE_LINK_UP_SPCV: c_uint = 0x2;
pub const PHY_STATE_LINK_UP_SPC: c_uint = 0x1;
