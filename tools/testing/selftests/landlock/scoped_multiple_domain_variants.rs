//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/landlock/scoped_multiple_domain_variants.h
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
// Landlock variants for three processes with various domains.
//
// Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sandbox_type {
    NO_SANDBOX,
    SCOPE_SANDBOX,
// Any other type of sandboxing domain
    OTHER_SANDBOX,
}

// clang-format on
//
// .-----------------.
// |         ####### |  P3 -> P2 : allow
// |   P1----# P2  # |  P3 -> P1 : deny
// |         #  |  # |
// |         # P3  # |
// |         ####### |
// '-----------------'
//
// clang-format off
// clang-format on
//
// ###################
// #         ####### #  P3 -> P2 : allow
// #   P1----# P2  # #  P3 -> P1 : deny
// #         #  |  # #
// #         # P3  # #
// #         ####### #
// ###################
//
// clang-format off
// clang-format on
//
// .-----------------.
// |         .-----. |  P3 -> P2 : allow
// |   P1----| P2  | |  P3 -> P1 : allow
// |         |     | |
// |         | P3  | |
// |         '-----' |
// '-----------------'
//
// clang-format off
// clang-format on
//
// .----.    ######   P3 -> P2 : allow
// | P1 |----# P2 #   P3 -> P1 : allow
// '----'    ######
// |
// P3
//
// clang-format off
// clang-format on
//
// ######    .-----.   P3 -> P2 : allow
// # P1 #----| P2  |   P3 -> P1 : allow
// ######    '-----'
// |
// P3
//
// clang-format off
// clang-format on
//
// ######    ######   P3 -> P2 : allow
// # P1 #----# P2 #   P3 -> P1 : allow
// ######    ######
// |
// .----.
// | P3 |
// '----'
//
// clang-format off
// clang-format on
//
// ######		P3 -> P2 : deny
// # P1 #----P2	P3 -> P1 : deny
// ######     |
// |
// ######
// # P3 #
// ######
//
// clang-format off
// clang-format on
