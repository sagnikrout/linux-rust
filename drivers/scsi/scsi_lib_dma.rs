//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_lib_dma.c
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
// SCSI library functions depending on DMA
//

//
// scsi_dma_map - perform DMA mapping against command's sg lists
// @cmd:	scsi command
//
// Returns the number of sg lists actually used, zero if the sg lists
// is NULL, or -ENOMEM if the mapping failed.
//
#[no_mangle]
pub unsafe extern "C" fn scsi_dma_map(cmd: *mut scsi_cmnd) -> c_int {
    int scsi_dma_map(struct scsi_cmnd *cmd)
    {
    let mut nseg: c_int = 0;
    if (scsi_sg_count(cmd)) {
    struct device *dev = cmd.device.host.dma_dev;
    nseg = dma_map_sg(dev, scsi_sglist(cmd), scsi_sg_count(cmd),
    cmd.sc_data_direction);
    if (unlikely(!nseg))
    return -ENOMEM;
    }
    return nseg;
    }
    EXPORT_SYMBOL(scsi_dma_map);
//
// scsi_dma_unmap - unmap command's sg lists mapped by scsi_dma_map
// @cmd:	scsi command
//
#[no_mangle]
pub unsafe extern "C" fn scsi_dma_unmap(cmd: *mut scsi_cmnd) {
    void scsi_dma_unmap(struct scsi_cmnd *cmd)
    {
    if (scsi_sg_count(cmd)) {
    struct device *dev = cmd.device.host.dma_dev;
    dma_unmap_sg(dev, scsi_sglist(cmd), scsi_sg_count(cmd),
    cmd.sc_data_direction);
    }
    }
    EXPORT_SYMBOL(scsi_dma_unmap);
