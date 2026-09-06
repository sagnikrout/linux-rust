//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic7xxx/aic7xxx_osm.h
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
// Adaptec AIC7xxx device driver for Linux.
//
// Copyright (c) 1994 John Aycock
// The University of Calgary Department of Computer Science.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, write to
// the Free Software Foundation, 675 Mass Ave, Cambridge, MA 02139, USA.
//
// Copyright (c) 2000-2003 Adaptec Inc.
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
// $Id: //depot/aic7xxx/linux/drivers/scsi/aic7xxx/aic7xxx_osm.h#151 $
//

// Core SCSI definitions

// Debugging

pub const AHC_DEBUG: c_int = 1;

//
// Compile in debugging code, but do not enable any printfs.
//
pub const AHC_DEBUG: c_int = 1;

// No debugging code.

// Forward Declarations
// Byte Order

// Configuration Data
// Bus Space/DMA
pub type bus_size_t = u32;
pub type bus_dma_tag_t = *mut ahc_linux_dma_tag;
pub type bus_dmamap_t = dma_addr_t;
extern "C" {
    pub fn bus_dma_filter_t(_arg: *mut c_void, _arg: dma_addr_t) -> typedef int;
}
extern "C" {
    pub fn bus_dmamap_callback_t(: *mut c_void, : *mut bus_dma_segment_t, _arg: c_int, _arg: c_int) -> typedef void;
}
pub const BUS_DMA_WAITOK: c_uint = 0x0;
pub const BUS_DMA_NOWAIT: c_uint = 0x1;
pub const BUS_DMA_ALLOCNOW: c_uint = 0x2;
pub const BUS_DMA_LOAD_SEGS: c_uint = 0x4	/*;
// Argument is an S/G list not
// a single buffer.
//
pub const BUS_SPACE_MAXADDR: c_uint = 0xFFFFFFFF;
pub const BUS_SPACE_MAXADDR_32BIT: c_uint = 0xFFFFFFFF;
pub const BUS_SPACE_MAXSIZE_32BIT: c_uint = 0xFFFFFFFF;
extern "C" {
    pub fn ahc_dma_tag_destroy(: *mut ahc_softc, /*tag*/: *mut bus_dma_tag_t);
}
extern "C" {
    pub fn ahc_dmamap_unload(: *mut ahc_softc, _arg: bus_dma_tag_t, _arg: bus_dmamap_t) -> c_int;
}
//
// Operations performed by ahc_dmamap_sync().
//
pub const BUS_DMASYNC_PREREAD: c_uint = 0x01	/* pre-read synchronization */;
pub const BUS_DMASYNC_POSTREAD: c_uint = 0x02	/* post-read synchronization */;
pub const BUS_DMASYNC_PREWRITE: c_uint = 0x04	/* pre-write synchronization */;
pub const BUS_DMASYNC_POSTWRITE: c_uint = 0x08	/* post-write synchronization */;
//
// XXX
// ahc_dmamap_sync is only used on buffers allocated with
// the dma_alloc_coherent() API.  Although I'm not sure how
// this works on architectures with a write buffer, Linux does
// not have an API to sync "coherent" memory.  Perhaps we need
// to do an mb()?
//

// Includes

pub const AIC_DEBUG_REGISTERS: c_int = 1;

pub const AIC_DEBUG_REGISTERS: c_int = 0;

// Timer Facilities
// SMP support

// Device Data Structures
//
// A per probed device structure used to deal with some error recovery
// scenarios that the Linux mid-layer code just doesn't know how to
// handle.  The structure allocated for a device only becomes persistent
// after a successfully completed inquiry command to the target when
// that inquiry data indicates a lun is present.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_linux_device {
//
// The number of transactions currently
// queued to the device.
//
    pub active: c_int,
//
// The currently allowed number of
// transactions that can be queued to
// the device.  Must be signed for
// conversion from tagged to untagged
// mode where the device may have more
// than one outstanding active transaction.
//
    pub openings: c_int,
//
// A positive count indicates that this
// device's queue is halted.
//
    pub qfrozen: u_int,
//
// Cumulative command counter.
//
    pub commands_issued: u_long,
//
// The number of tagged transactions when
// running at our current opening level
// that have been successfully received by
// this device since the last QUEUE FULL.
//
    pub tag_success_count: u_int,
pub const AHC_TAG_SUCCESS_INTERVAL: c_int = 50;
    pub flags: ahc_linux_dev_flags,
//
// The high limit for the tags variable.
//
    pub maxtags: u_int,
//
// The computed number of tags outstanding
// at the time of the last QUEUE FULL event.
//
    pub tags_on_last_queuefull: u_int,
//
// How many times we have seen a queue full
// with the same number of tags.  This is used
// to stop our adaptive queue depth algorithm
// on devices with a fixed number of tags.
//
    pub last_queuefull_same_count: u_int,
pub const AHC_LOCK_TAGS_COUNT: c_int = 50;
//
// How many transactions have been queued
// without the device going idle.  We use
// this statistic to determine when to issue
// an ordered tag to prevent transaction
// starvation.  This statistic is only updated
// if the AHC_DEV_PERIODIC_OTAG flag is set
// on this device.
//
    pub commands_since_idle_or_otag: u_int,
pub const AHC_OTAG_THRESH: c_int = 500;
}

// Definitions Required by the Core
//
// Number of SG segments we require.  So long as the S/G segments for
// a particular transaction are allocated in a physically contiguous
// manner and are allocated below 4GB, the number of S/G segments is
// unrestricted.
//
pub const AHC_NSEG: c_int = 128;
//
// Per-SCB OSM storage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scb_platform_data {
    pub dev: *mut ahc_linux_device,
    pub buf_busaddr: dma_addr_t,
    pub xfer_len: u32,
    pub /: *mut *mut uint32_t sense_resid; / Auto-Sense residual,
}

//
// Define a structure used for each host adapter.  All members are
// aligned on a boundary >= the size of the member to honor the
// alignment restrictions of the various platforms supported by
// this driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ahc_platform_data {
//
// Fields accessed from interrupt context.
//
    pub starget: [*mut scsi_target; AHC_NUM_TARGETS],
    pub spin_lock: spinlock_t,
    pub qfrozen: u_int,
    pub eh_done: *mut completion,
    pub /: *mut *mut *mut Scsi_Host host; / pointer to scsi host,

    pub /: *mut *mut uint32_t irq; / IRQ for this adapter,
    pub bios_address: u32,
    pub /: *mut *mut resource_size_t mem_busaddr; / Mem Base Addr,
}

extern "C" {
    pub fn ahc_delay(_arg: c_long);
}
// Low Level I/O
extern "C" {
    pub fn ahc_inb(ahc: *mut *mut ahc_softc, port: c_long) -> u8;
}
extern "C" {
    pub fn ahc_outb(ahc: *mut *mut ahc_softc, port: c_long, val: u8);
}
// Initialization
// Locking
// Lock protecting internal data structures
// PCI Definitions
//
// PCIM_xxx: mask to locate subfield in register
// PCIR_xxx: config register offset
// PCIC_xxx: device class
// PCIS_xxx: device subclass
// PCIP_xxx: device programming interface
// PCIV_xxx: PCI vendor ID (only required to fixup ancient devices)
// PCID_xxx: device ID
//
pub const PCIR_DEVVENDOR: c_uint = 0x00;
pub const PCIR_VENDOR: c_uint = 0x00;
pub const PCIR_DEVICE: c_uint = 0x02;
pub const PCIR_COMMAND: c_uint = 0x04;
pub const PCIM_CMD_PORTEN: c_uint = 0x0001;
pub const PCIM_CMD_MEMEN: c_uint = 0x0002;
pub const PCIM_CMD_BUSMASTEREN: c_uint = 0x0004;
pub const PCIM_CMD_MWRICEN: c_uint = 0x0010;
pub const PCIM_CMD_PERRESPEN: c_uint = 0x0040;
pub const PCIM_CMD_SERRESPEN: c_uint = 0x0100;
pub const PCIR_STATUS: c_uint = 0x06;
pub const PCIR_REVID: c_uint = 0x08;
pub const PCIR_PROGIF: c_uint = 0x09;
pub const PCIR_SUBCLASS: c_uint = 0x0a;
pub const PCIR_CLASS: c_uint = 0x0b;
pub const PCIR_CACHELNSZ: c_uint = 0x0c;
pub const PCIR_LATTIMER: c_uint = 0x0d;
pub const PCIR_HEADERTYPE: c_uint = 0x0e;
pub const PCIM_MFDEV: c_uint = 0x80;
pub const PCIR_BIST: c_uint = 0x0f;
pub const PCIR_CAP_PTR: c_uint = 0x34;
// config registers for header type 0 devices
pub const PCIR_MAPS: c_uint = 0x10;
// VL/EISA Routines

extern "C" {
    pub fn ahc_linux_eisa_init() -> c_int;
}
extern "C" {
    pub fn ahc_linux_eisa_exit();
}
extern "C" {
    pub fn aic7770_map_int(ahc: *mut ahc_softc, irq: u_int) -> c_int;
}

// PCI Routines

extern "C" {
    pub fn ahc_linux_pci_init() -> c_int;
}
extern "C" {
    pub fn ahc_linux_pci_exit();
}
extern "C" {
    pub fn ahc_pci_map_registers(ahc: *mut ahc_softc) -> c_int;
}
extern "C" {
    pub fn ahc_pci_map_int(ahc: *mut ahc_softc) -> c_int;
}
extern "C" {
    pub fn ahc_get_pci_function(_arg: ahc_dev_softc_t) -> c_int;
}
extern "C" {
    pub fn ahc_get_pci_slot(_arg: ahc_dev_softc_t) -> c_int;
}
extern "C" {
    pub fn ahc_get_pci_bus(_arg: ahc_dev_softc_t) -> c_int;
}

extern "C" {
    pub fn ahc_flush_device_writes(: *mut ahc_softc);
}
// XXX Is this sufficient for all architectures???
// Proc FS Support
extern "C" {
    pub fn ahc_proc_write_seeprom(: *mut Scsi_Host, : *mut c_char, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn ahc_linux_show_info(: *mut seq_file, : *mut Scsi_Host) -> c_int;
}
// Domain Validation
// Transaction Access Wrappers
//
// Nothing to do for linux as the incoming transaction
// has no concept of tag/non tagged, etc.
//
extern "C" {
    pub fn scsi_get_resid(_arg: scb->io_ctx) -> return;
}
//
// We always perform autosense in Linux.
// On other platforms this is set on a
// per-transaction basis.
//
// Nothing to do here for linux
extern "C" {
    pub fn ahc_platform_alloc(ahc: *mut ahc_softc, platform_arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ahc_platform_free(ahc: *mut ahc_softc);
}
extern "C" {
    pub fn ahc_platform_freeze_devq(ahc: *mut ahc_softc, scb: *mut scb);
}
extern "C" {
    pub fn ahc_platform_flushwork(ahc: *mut ahc_softc);
}
extern "C" {
    pub fn ahc_done(ahc_softc*: *mut struct, scb*: *mut struct);
}
extern "C" {
    pub fn ahc_print_path(: *mut ahc_softc, : *mut scb);
}

pub const AHC_PCI_CONFIG: c_int = 1;

pub const AHC_PCI_CONFIG: c_int = 0;

