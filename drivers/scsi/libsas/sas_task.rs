//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/libsas/sas_task.c
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


// SPDX-License-Identifier: GPL-2.0-only

// fill task_status_struct based on SSP response frame
    void sas_ssp_task_response(struct device *dev, struct sas_task *task,
    struct ssp_response_iu *iu)
    {
    struct task_status_struct *tstat = &task.task_status;
    tstat.resp = SAS_TASK_COMPLETE;
    switch (iu.datapres) {
    case SAS_DATAPRES_NO_DATA:
    tstat.stat = iu.status;
    break;
    case SAS_DATAPRES_RESPONSE_DATA:
    tstat.stat = iu.resp_data[3];
    break;
    case SAS_DATAPRES_SENSE_DATA:
    tstat.stat = SAS_SAM_STAT_CHECK_CONDITION;
    tstat.buf_valid_size =
    min_t(int, SAS_STATUS_BUF_SIZE,
    be32_to_cpu(iu.sense_data_len));
    memcpy(tstat.buf, iu.sense_data, tstat.buf_valid_size);
    if (iu.status != SAM_STAT_CHECK_CONDITION)
    dev_warn(dev, "dev %016llx sent sense data, but stat(0x%x) is not CHECK CONDITION\n",
    SAS_ADDR(task.dev.sas_addr), iu.status);
    break;
    default:
// when datapres contains corrupt/unknown value...
    tstat.stat = SAS_SAM_STAT_CHECK_CONDITION;
    }
    }
    EXPORT_SYMBOL_GPL(sas_ssp_task_response);
