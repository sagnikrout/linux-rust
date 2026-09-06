//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/disp/mdp5/mdp5_irq.c
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
//
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

    void mdp5_set_irqmask(struct mdp_kms *mdp_kms, uint32_t irqmask,
    uint32_t old_irqmask)
    {
    mdp5_write(to_mdp5_kms(mdp_kms), REG_MDP5_INTR_CLEAR,
    irqmask ^ (irqmask & old_irqmask));
    mdp5_write(to_mdp5_kms(mdp_kms), REG_MDP5_INTR_EN, irqmask);
    }
#[no_mangle]
unsafe extern "C" fn mdp5_irq_error_handler(irq: *mut mdp_irq, irqstatus: u32) {
    static void mdp5_irq_error_handler(struct mdp_irq *irq, uint32_t irqstatus)
    {
    struct mdp5_kms *mdp5_kms = container_of(irq, struct mdp5_kms, error_handler);
    static DEFINE_RATELIMIT_STATE(rs, 5*HZ, 1);
    extern bool dumpstate;
    DRM_ERROR_RATELIMITED("errors: %08x\n", irqstatus);
    if (dumpstate && __ratelimit(&rs)) {
    let mut p: drm_printer = drm_info_printer(mdp5_kms.dev.dev);
    drm_state_dump(mdp5_kms.dev, &p);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mdp5_irq_preinstall(kms: *mut msm_kms) {
    void mdp5_irq_preinstall(struct msm_kms *kms)
    {
    struct mdp5_kms *mdp5_kms = to_mdp5_kms(to_mdp_kms(kms));
    struct device *dev = &mdp5_kms.pdev.dev;
    pm_runtime_get_sync(dev);
    mdp5_write(mdp5_kms, REG_MDP5_INTR_CLEAR, 0xffffffff);
    mdp5_write(mdp5_kms, REG_MDP5_INTR_EN, 0x00000000);
    pm_runtime_put_sync(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn mdp5_irq_postinstall(kms: *mut msm_kms) -> c_int {
    int mdp5_irq_postinstall(struct msm_kms *kms)
    {
    struct mdp_kms *mdp_kms = to_mdp_kms(kms);
    struct mdp5_kms *mdp5_kms = to_mdp5_kms(mdp_kms);
    struct device *dev = &mdp5_kms.pdev.dev;
    struct mdp_irq *error_handler = &mdp5_kms.error_handler;
    error_handler.irq = mdp5_irq_error_handler;
    error_handler.irqmask = MDP5_IRQ_INTF0_UNDER_RUN |
    MDP5_IRQ_INTF1_UNDER_RUN |
    MDP5_IRQ_INTF2_UNDER_RUN |
    MDP5_IRQ_INTF3_UNDER_RUN;
    pm_runtime_get_sync(dev);
    mdp_irq_register(mdp_kms, error_handler);
    pm_runtime_put_sync(dev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mdp5_irq_uninstall(kms: *mut msm_kms) {
    void mdp5_irq_uninstall(struct msm_kms *kms)
    {
    struct mdp5_kms *mdp5_kms = to_mdp5_kms(to_mdp_kms(kms));
    struct device *dev = &mdp5_kms.pdev.dev;
    pm_runtime_get_sync(dev);
    mdp5_write(mdp5_kms, REG_MDP5_INTR_EN, 0x00000000);
    pm_runtime_put_sync(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn mdp5_irq(kms: *mut msm_kms) -> irqreturn_t {
    irqreturn_t mdp5_irq(struct msm_kms *kms)
    {
    struct mdp_kms *mdp_kms = to_mdp_kms(kms);
    struct mdp5_kms *mdp5_kms = to_mdp5_kms(mdp_kms);
    struct drm_device *dev = mdp5_kms.dev;
    struct drm_crtc *crtc;
    uint32_t status, enable;
    enable = mdp5_read(mdp5_kms, REG_MDP5_INTR_EN);
    status = mdp5_read(mdp5_kms, REG_MDP5_INTR_STATUS) & enable;
    mdp5_write(mdp5_kms, REG_MDP5_INTR_CLEAR, status);
    VERB("status=%08x", status);
    mdp_dispatch_irqs(mdp_kms, status);
    drm_for_each_crtc(crtc, dev)
    if (status & mdp5_crtc_vblank(crtc))
    drm_crtc_handle_vblank(crtc);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn mdp5_enable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc) -> c_int {
    int mdp5_enable_vblank(struct msm_kms *kms, struct drm_crtc *crtc)
    {
    struct mdp5_kms *mdp5_kms = to_mdp5_kms(to_mdp_kms(kms));
    struct device *dev = &mdp5_kms.pdev.dev;
    pm_runtime_get_sync(dev);
    mdp_update_vblank_mask(to_mdp_kms(kms),
    mdp5_crtc_vblank(crtc), true);
    pm_runtime_put_sync(dev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mdp5_disable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc) {
    void mdp5_disable_vblank(struct msm_kms *kms, struct drm_crtc *crtc)
    {
    struct mdp5_kms *mdp5_kms = to_mdp5_kms(to_mdp_kms(kms));
    struct device *dev = &mdp5_kms.pdev.dev;
    pm_runtime_get_sync(dev);
    mdp_update_vblank_mask(to_mdp_kms(kms),
    mdp5_crtc_vblank(crtc), false);
    pm_runtime_put_sync(dev);
    }
