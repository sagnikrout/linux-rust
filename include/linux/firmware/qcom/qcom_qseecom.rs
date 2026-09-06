//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/qcom/qcom_qseecom.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Qualcomm Secure Execution Environment (SEE) interface (QSEECOM).
// Responsible for setting up and managing QSEECOM client devices.
//
// Copyright (C) 2023 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// struct qseecom_client - QSEECOM client device.
// @aux_dev: Underlying auxiliary device.
// @app_id: ID of the loaded application.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qseecom_client {
    pub aux_dev: auxiliary_device,
    pub app_id: u32,
}

//
// qcom_qseecom_app_send() - Send to and receive data from a given QSEE app.
// @client:   The QSEECOM client associated with the target app.
// @req:      Request buffer sent to the app (must be TZ memory).
// @req_size: Size of the request buffer.
// @rsp:      Response buffer, written to by the app (must be TZ memory).
// @rsp_size: Size of the response buffer.
//
// Sends a request to the QSEE app associated with the given client and read
// back its response. The caller must provide two DMA memory regions, one for
// the request and one for the response, and fill out the @req region with the
// respective (app-specific) request data. The QSEE app reads this and returns
// its response in the @rsp region.
//
// Note: This is a convenience wrapper around qcom_scm_qseecom_app_send().
// Clients should prefer to use this wrapper.
//
// Return: Zero on success, nonzero on failure.
//
extern "C" {
    pub fn qcom_scm_qseecom_app_send(_arg: client->app_id, _arg: req, _arg: req_size, _arg: rsp, _arg: rsp_size) -> return;
}
