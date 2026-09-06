//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/snic/snic_ctl.c
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.

//
// snic_handle_link : Handles link flaps.
//
    void
    snic_handle_link(struct work_struct *work)
    {
    struct snic *snic = container_of(work, struct snic, link_work);
    if (snic.config.xpt_type == SNIC_DAS)
    return;
    snic.link_status = svnic_dev_link_status(snic.vdev);
    snic.link_down_cnt = svnic_dev_link_down_cnt(snic.vdev);
    SNIC_HOST_INFO(snic.shost, "Link Event: Link %s.\n",
    ((snic.link_status) ? "Up" : "Down"));
    SNIC_ASSERT_NOT_IMPL(1);
    }
//
// snic_ver_enc : Encodes version str to int
// version string is similar to netmask string
//
    static int
    snic_ver_enc(const char *s)
    {
    int v[4] = {0};
    let mut i: c_int = 0, x = 0;
    char c;
    const char *p = s;
// validate version string
    if ((strlen(s) > 15) || (strlen(s) < 7))
    goto end;
    while ((c = *p++)) {
    if (c == '.') {
    i++;
    continue;
    }
    if (i > 3 || !isdigit(c))
    goto end;
    v[i] = v[i] * 10 + (c - '0');
    }
// validate sub version numbers
    for (i = 3; i >= 0; i--)
    if (v[i] > 0xff)
    goto end;
    x |= (v[0] << 24) | v[1] << 16 | v[2] << 8 | v[3];
    end:
    if (x == 0) {
    SNIC_ERR("Invalid version string [%s].\n", s);
    return -1;
    }
    return x;
    } /* end of snic_ver_enc */
//
// snic_qeueue_exch_ver_req :
//
// Queues Exchange Version Request, to communicate host information
// in return, it gets firmware version details
//
    int
    snic_queue_exch_ver_req(struct snic *snic)
    {
    struct snic_req_info *rqi = core::ptr::null_mut();
    struct snic_host_req *req = core::ptr::null_mut();
    let mut ver: u32 = 0;
    let mut ret: c_int = 0;
    SNIC_HOST_INFO(snic.shost, "Exch Ver Req Preparing...\n");
    rqi = snic_req_init(snic, 0);
    if (!rqi) {
    SNIC_HOST_ERR(snic.shost, "Init Exch Ver Req failed\n");
    ret = -ENOMEM;
    goto error;
    }
    req = rqi_to_req(rqi);
// Initialize snic_host_req
    snic_io_hdr_enc(&req.hdr, SNIC_REQ_EXCH_VER, 0, SCSI_NO_TAG,
    snic.config.hid, 0, (ulong)rqi);
    ver = snic_ver_enc(SNIC_DRV_VERSION);
    req.u.exch_ver.drvr_ver = cpu_to_le32(ver);
    req.u.exch_ver.os_type = cpu_to_le32(SNIC_OS_LINUX);
    snic_handle_untagged_req(snic, rqi);
    ret = snic_queue_wq_desc(snic, req, sizeof(*req));
    if (ret) {
    snic_release_untagged_req(snic, rqi);
    SNIC_HOST_ERR(snic.shost,
    "Queuing Exch Ver Req failed, err = %d\n",
    ret);
    goto error;
    }
    SNIC_HOST_INFO(snic.shost, "Exch Ver Req is issued. ret = %d\n", ret);
    error:
    return ret;
    } /* end of snic_queue_exch_ver_req */
//
// snic_io_exch_ver_cmpl_handler
//
    void
    snic_io_exch_ver_cmpl_handler(struct snic *snic, struct snic_fw_req *fwreq)
    {
    struct snic_req_info *rqi = core::ptr::null_mut();
    struct snic_exch_ver_rsp *exv_cmpl = &fwreq.u.exch_ver_cmpl;
    u8 typ, hdr_stat;
    u32 cmnd_id, hid, max_sgs;
    let mut ctx: c_ulong = 0;
    unsigned long flags;
    SNIC_HOST_INFO(snic.shost, "Exch Ver Compl Received.\n");
    snic_io_hdr_dec(&fwreq.hdr, &typ, &hdr_stat, &cmnd_id, &hid, &ctx);
    SNIC_BUG_ON(snic.config.hid != hid);
    rqi = (struct snic_req_info *) ctx;
    if (hdr_stat) {
    SNIC_HOST_ERR(snic.shost,
    "Exch Ver Completed w/ err status %d\n",
    hdr_stat);
    goto exch_cmpl_end;
    }
    spin_lock_irqsave(&snic.snic_lock, flags);
    snic.fwinfo.fw_ver = le32_to_cpu(exv_cmpl.version);
    snic.fwinfo.hid = le32_to_cpu(exv_cmpl.hid);
    snic.fwinfo.max_concur_ios = le32_to_cpu(exv_cmpl.max_concur_ios);
    snic.fwinfo.max_sgs_per_cmd = le32_to_cpu(exv_cmpl.max_sgs_per_cmd);
    snic.fwinfo.max_io_sz = le32_to_cpu(exv_cmpl.max_io_sz);
    snic.fwinfo.max_tgts = le32_to_cpu(exv_cmpl.max_tgts);
    snic.fwinfo.io_tmo = le16_to_cpu(exv_cmpl.io_timeout);
    SNIC_HOST_INFO(snic.shost,
    "vers %u hid %u max_concur_ios %u max_sgs_per_cmd %u max_io_sz %u max_tgts %u fw tmo %u\n",
    snic.fwinfo.fw_ver,
    snic.fwinfo.hid,
    snic.fwinfo.max_concur_ios,
    snic.fwinfo.max_sgs_per_cmd,
    snic.fwinfo.max_io_sz,
    snic.fwinfo.max_tgts,
    snic.fwinfo.io_tmo);
    SNIC_HOST_INFO(snic.shost,
    "HBA Capabilities = 0x%x\n",
    le32_to_cpu(exv_cmpl.hba_cap));
// Updating SGList size
    max_sgs = snic.fwinfo.max_sgs_per_cmd;
    if (max_sgs && max_sgs < SNIC_MAX_SG_DESC_CNT) {
    snic.shost.sg_tablesize = max_sgs;
    SNIC_HOST_INFO(snic.shost, "Max SGs set to %d\n",
    snic.shost.sg_tablesize);
    } else if (max_sgs > snic.shost.sg_tablesize) {
    SNIC_HOST_INFO(snic.shost,
    "Target type %d Supports Larger Max SGList %d than driver's Max SG List %d.\n",
    snic.config.xpt_type, max_sgs,
    snic.shost.sg_tablesize);
    }
    if (snic.shost.can_queue > snic.fwinfo.max_concur_ios)
    snic.shost.can_queue = snic.fwinfo.max_concur_ios;
    snic.shost.max_sectors = snic.fwinfo.max_io_sz >> 9;
    if (snic.fwinfo.wait)
    complete(snic.fwinfo.wait);
    spin_unlock_irqrestore(&snic.snic_lock, flags);
    exch_cmpl_end:
    snic_release_untagged_req(snic, rqi);
    SNIC_HOST_INFO(snic.shost, "Exch_cmpl Done, hdr_stat %d.\n", hdr_stat);
    } /* end of snic_io_exch_ver_cmpl_handler */
//
// snic_get_conf
//
// Synchronous call, and Retrieves snic params.
//
    int
    snic_get_conf(struct snic *snic)
    {
    DECLARE_COMPLETION_ONSTACK(wait);
    unsigned long flags;
    int ret;
    let mut nr_retries: c_int = 3;
    SNIC_HOST_INFO(snic.shost, "Retrieving snic params.\n");
    spin_lock_irqsave(&snic.snic_lock, flags);
    memset(&snic.fwinfo, 0, sizeof(snic.fwinfo));
    snic.fwinfo.wait = &wait;
    spin_unlock_irqrestore(&snic.snic_lock, flags);
// Additional delay to handle HW Resource initialization.
    msleep(50);
//
// Exch ver req can be ignored by FW, if HW Resource initialization
// is in progress, Hence retry.
//
    do {
    ret = snic_queue_exch_ver_req(snic);
    if (ret)
    return ret;
    wait_for_completion_timeout(&wait, msecs_to_jiffies(2000));
    spin_lock_irqsave(&snic.snic_lock, flags);
    ret = (snic.fwinfo.fw_ver != 0) ? 0 : -ETIMEDOUT;
    if (ret)
    SNIC_HOST_ERR(snic.shost,
    "Failed to retrieve snic params,\n");
// Unset fwinfo.wait, on success or on last retry
    if (ret == 0 || nr_retries == 1)
    snic.fwinfo.wait = core::ptr::null_mut();
    spin_unlock_irqrestore(&snic.snic_lock, flags);
    } while (ret && --nr_retries);
    return ret;
    } /* end of snic_get_info */
