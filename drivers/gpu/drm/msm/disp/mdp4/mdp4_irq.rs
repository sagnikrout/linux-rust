//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/disp/mdp4/mdp4_irq.c
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

    void mdp4_set_irqmask(struct mdp_kms *mdp_kms, uint32_t irqmask,
    uint32_t old_irqmask)
    {
    mdp4_write(to_mdp4_kms(mdp_kms), REG_MDP4_INTR_CLEAR,
    irqmask ^ (irqmask & old_irqmask));
    mdp4_write(to_mdp4_kms(mdp_kms), REG_MDP4_INTR_ENABLE, irqmask);
    }
#[no_mangle]
unsafe extern "C" fn mdp4_irq_error_handler(irq: *mut mdp_irq, irqstatus: u32) {
    static void mdp4_irq_error_handler(struct mdp_irq *irq, uint32_t irqstatus)
    {
    struct mdp4_kms *mdp4_kms = container_of(irq, struct mdp4_kms, error_handler);
    static DEFINE_RATELIMIT_STATE(rs, 5*HZ, 1);
    extern bool dumpstate;
    DRM_ERROR_RATELIMITED("errors: %08x\n", irqstatus);
    if (dumpstate && __ratelimit(&rs)) {
    let mut p: drm_printer = drm_info_printer(mdp4_kms.dev.dev);
    drm_state_dump(mdp4_kms.dev, &p);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mdp4_irq_preinstall(kms: *mut msm_kms) {
    void mdp4_irq_preinstall(struct msm_kms *kms)
    {
    struct mdp4_kms *mdp4_kms = to_mdp4_kms(to_mdp_kms(kms));
    mdp4_enable(mdp4_kms);
    mdp4_write(mdp4_kms, REG_MDP4_INTR_CLEAR, 0xffffffff);
    mdp4_write(mdp4_kms, REG_MDP4_INTR_ENABLE, 0x00000000);
    mdp4_disable(mdp4_kms);
    }
#[no_mangle]
pub unsafe extern "C" fn mdp4_irq_postinstall(kms: *mut msm_kms) -> c_int {
    int mdp4_irq_postinstall(struct msm_kms *kms)
    {
    struct mdp_kms *mdp_kms = to_mdp_kms(kms);
    struct mdp4_kms *mdp4_kms = to_mdp4_kms(mdp_kms);
    struct mdp_irq *error_handler = &mdp4_kms.error_handler;
    error_handler.irq = mdp4_irq_error_handler;
    error_handler.irqmask = MDP4_IRQ_PRIMARY_INTF_UDERRUN |
    MDP4_IRQ_EXTERNAL_INTF_UDERRUN;
    mdp_irq_register(mdp_kms, error_handler);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mdp4_irq_uninstall(kms: *mut msm_kms) {
    void mdp4_irq_uninstall(struct msm_kms *kms)
    {
    struct mdp4_kms *mdp4_kms = to_mdp4_kms(to_mdp_kms(kms));
    mdp4_enable(mdp4_kms);
    mdp4_write(mdp4_kms, REG_MDP4_INTR_ENABLE, 0x00000000);
    mdp4_disable(mdp4_kms);
    }
#[no_mangle]
pub unsafe extern "C" fn mdp4_irq(kms: *mut msm_kms) -> irqreturn_t {
    irqreturn_t mdp4_irq(struct msm_kms *kms)
    {
    struct mdp_kms *mdp_kms = to_mdp_kms(kms);
    struct mdp4_kms *mdp4_kms = to_mdp4_kms(mdp_kms);
    struct drm_device *dev = mdp4_kms.dev;
    struct drm_crtc *crtc;
    uint32_t status, enable;
    enable = mdp4_read(mdp4_kms, REG_MDP4_INTR_ENABLE);
    status = mdp4_read(mdp4_kms, REG_MDP4_INTR_STATUS) & enable;
    mdp4_write(mdp4_kms, REG_MDP4_INTR_CLEAR, status);
    VERB("status=%08x", status);
    mdp_dispatch_irqs(mdp_kms, status);
    drm_for_each_crtc(crtc, dev)
    if (status & mdp4_crtc_vblank(crtc))
    drm_crtc_handle_vblank(crtc);
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn mdp4_enable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc) -> c_int {
    int mdp4_enable_vblank(struct msm_kms *kms, struct drm_crtc *crtc)
    {
    struct mdp4_kms *mdp4_kms = to_mdp4_kms(to_mdp_kms(kms));
    mdp4_enable(mdp4_kms);
    mdp_update_vblank_mask(to_mdp_kms(kms),
    mdp4_crtc_vblank(crtc), true);
    mdp4_disable(mdp4_kms);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mdp4_disable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc) {
    void mdp4_disable_vblank(struct msm_kms *kms, struct drm_crtc *crtc)
    {
    struct mdp4_kms *mdp4_kms = to_mdp4_kms(to_mdp_kms(kms));
    mdp4_enable(mdp4_kms);
    mdp_update_vblank_mask(to_mdp_kms(kms),
    mdp4_crtc_vblank(crtc), false);
    mdp4_disable(mdp4_kms);
    }
