//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/libsas/sas_phy.c
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
// Serial Attached SCSI (SAS) Phy class
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

// ---------- Phy events ----------
#[no_mangle]
unsafe extern "C" fn sas_phye_loss_of_signal(work: *mut work_struct) {
    static void sas_phye_loss_of_signal(struct work_struct *work)
    {
    struct asd_sas_event *ev = to_asd_sas_event(work);
    struct asd_sas_phy *phy = ev.phy;
    phy.error = 0;
    sas_deform_port(phy, true);
    }
#[no_mangle]
unsafe extern "C" fn sas_phye_oob_done(work: *mut work_struct) {
    static void sas_phye_oob_done(struct work_struct *work)
    {
    struct asd_sas_event *ev = to_asd_sas_event(work);
    struct asd_sas_phy *phy = ev.phy;
    phy.error = 0;
    }
#[no_mangle]
unsafe extern "C" fn sas_phye_oob_error(work: *mut work_struct) {
    static void sas_phye_oob_error(struct work_struct *work)
    {
    struct asd_sas_event *ev = to_asd_sas_event(work);
    struct asd_sas_phy *phy = ev.phy;
    struct sas_ha_struct *sas_ha = phy.ha;
    struct asd_sas_port *port = phy.port;
    struct sas_internal *i =
    to_sas_internal(sas_ha.shost.transportt);
    sas_deform_port(phy, true);
    if (!port && phy.enabled && i.dft.lldd_control_phy) {
    phy.error++;
    switch (phy.error) {
    case 1:
    case 2:
    i.dft.lldd_control_phy(phy, PHY_FUNC_HARD_RESET,
    core::ptr::null_mut());
    break;
    case 3:
    default:
    phy.error = 0;
    phy.enabled = 0;
    i.dft.lldd_control_phy(phy, PHY_FUNC_DISABLE, core::ptr::null_mut());
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn sas_phye_spinup_hold(work: *mut work_struct) {
    static void sas_phye_spinup_hold(struct work_struct *work)
    {
    struct asd_sas_event *ev = to_asd_sas_event(work);
    struct asd_sas_phy *phy = ev.phy;
    struct sas_ha_struct *sas_ha = phy.ha;
    struct sas_internal *i =
    to_sas_internal(sas_ha.shost.transportt);
    phy.error = 0;
    i.dft.lldd_control_phy(phy, PHY_FUNC_RELEASE_SPINUP_HOLD, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sas_phye_resume_timeout(work: *mut work_struct) {
    static void sas_phye_resume_timeout(struct work_struct *work)
    {
    struct asd_sas_event *ev = to_asd_sas_event(work);
    struct asd_sas_phy *phy = ev.phy;
// phew, lldd got the phy back in the nick of time
    if (!phy.suspended) {
    dev_info(&phy.phy.dev, "resume timeout cancelled\n");
    return;
    }
    phy.error = 0;
    phy.suspended = 0;
    sas_deform_port(phy, true);
    }
#[no_mangle]
unsafe extern "C" fn sas_phye_shutdown(work: *mut work_struct) {
    static void sas_phye_shutdown(struct work_struct *work)
    {
    struct asd_sas_event *ev = to_asd_sas_event(work);
    struct asd_sas_phy *phy = ev.phy;
    struct sas_ha_struct *sas_ha = phy.ha;
    struct sas_internal *i =
    to_sas_internal(sas_ha.shost.transportt);
    if (phy.enabled) {
    int ret;
    phy.error = 0;
    phy.enabled = 0;
    ret = i.dft.lldd_control_phy(phy, PHY_FUNC_DISABLE, core::ptr::null_mut());
    if (ret)
    pr_notice("lldd disable phy%d returned %d\n", phy.id,
    ret);
    } else
    pr_notice("phy%d is not enabled, cannot shutdown\n", phy.id);
    phy.in_shutdown = 0;
    }
// ---------- Phy class registration ----------
#[no_mangle]
pub unsafe extern "C" fn sas_register_phys(sas_ha: *mut sas_ha_struct) -> c_int {
    int sas_register_phys(struct sas_ha_struct *sas_ha)
    {
    int i;
    int err;
// Now register the phys.
    for (i = 0; i < sas_ha.num_phys; i++) {
    struct asd_sas_phy *phy = sas_ha.sas_phy[i];
    phy.error = 0;
    atomic_set(&phy.event_nr, 0);
    INIT_LIST_HEAD(&phy.port_phy_el);
    phy.port = core::ptr::null_mut();
    phy.ha = sas_ha;
    spin_lock_init(&phy.frame_rcvd_lock);
    spin_lock_init(&phy.sas_prim_lock);
    phy.frame_rcvd_size = 0;
    phy.phy = sas_phy_alloc(&sas_ha.shost.shost_gendev, i);
    if (!phy.phy) {
    err = -ENOMEM;
    goto rollback;
    }
    phy.phy.identify.initiator_port_protocols =
    phy.iproto;
    phy.phy.identify.target_port_protocols = phy.tproto;
    phy.phy.identify.sas_address = SAS_ADDR(sas_ha.sas_addr);
    phy.phy.identify.phy_identifier = i;
    phy.phy.minimum_linkrate_hw = SAS_LINK_RATE_UNKNOWN;
    phy.phy.maximum_linkrate_hw = SAS_LINK_RATE_UNKNOWN;
    phy.phy.minimum_linkrate = SAS_LINK_RATE_UNKNOWN;
    phy.phy.maximum_linkrate = SAS_LINK_RATE_UNKNOWN;
    phy.phy.negotiated_linkrate = SAS_LINK_RATE_UNKNOWN;
    err = sas_phy_add(phy.phy);
    if (err) {
    sas_phy_free(phy.phy);
    goto rollback;
    }
    }
    return 0;
    rollback:
    for (i-- ; i >= 0 ; i--) {
    struct asd_sas_phy *phy = sas_ha.sas_phy[i];
    sas_phy_delete(phy.phy);
    sas_phy_free(phy.phy);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sas_unregister_phys(sas_ha: *mut sas_ha_struct) {
    void sas_unregister_phys(struct sas_ha_struct *sas_ha)
    {
    int i;
    for (i = 0 ; i < sas_ha.num_phys ; i++) {
    struct asd_sas_phy *phy = sas_ha.sas_phy[i];
    sas_phy_delete(phy.phy);
    sas_phy_free(phy.phy);
    }
    }
    const work_func_t sas_phy_event_fns[PHY_NUM_EVENTS] = {
    [PHYE_LOSS_OF_SIGNAL] = sas_phye_loss_of_signal,
    [PHYE_OOB_DONE] = sas_phye_oob_done,
    [PHYE_OOB_ERROR] = sas_phye_oob_error,
    [PHYE_SPINUP_HOLD] = sas_phye_spinup_hold,
    [PHYE_RESUME_TIMEOUT] = sas_phye_resume_timeout,
    [PHYE_SHUTDOWN] = sas_phye_shutdown,
    };
