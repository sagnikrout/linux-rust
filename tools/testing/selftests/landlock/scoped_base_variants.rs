//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/landlock/scoped_base_variants.h
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
// Landlock scoped_domains test variant definition.
//
// This file defines a fixture variant "scoped_domains" that has all
// permutations of parent/child process being in separate or shared
// Landlock domain, or not being in a Landlock domain at all.
//
// Scoped access tests can include this file to avoid repeating these
// combinations.
//
// Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2019-2020 ANSSI
// Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
//
// clang-format on
//
// No domain
//
// P1-.               P1 -> P2 : allow
// \              P2 -> P1 : allow
// 'P2
//
// clang-format off
// clang-format on
//
// Child domain
//
// P1--.              P1 -> P2 : allow
// \             P2 -> P1 : deny
// .'-----.
// |  P2  |
// '------'
//
// clang-format off
// clang-format on
//
// Parent domain
// .------.
// |  P1  --.           P1 -> P2 : deny
// '------'  \          P2 -> P1 : allow
// '
// P2
//
// clang-format off
// clang-format on
//
// Parent + child domain (siblings)
// .------.
// |  P1  ---.          P1 -> P2 : deny
// '------'   \         P2 -> P1 : deny
// .---'--.
// |  P2  |
// '------'
//
// clang-format off
// clang-format on
//
// Same domain (inherited)
// .-------------.
// | P1----.     |      P1 -> P2 : allow
// |        \    |      P2 -> P1 : allow
// |         '   |
// |         P2  |
// '-------------'
//
// clang-format off
// clang-format on
//
// Inherited + child domain
// .-----------------.
// |  P1----.        |  P1 -> P2 : allow
// |         \       |  P2 -> P1 : deny
// |        .-'----. |
// |        |  P2  | |
// |        '------' |
// '-----------------'
//
// clang-format off
// clang-format on
//
// Inherited + parent domain
// .-----------------.
// |.------.         |  P1 -> P2 : deny
// ||  P1  ----.     |  P2 -> P1 : allow
// |'------'    \    |
// |             '   |
// |             P2  |
// '-----------------'
//
// clang-format off
// clang-format on
//
// Inherited + parent and child domain (siblings)
// .-----------------.
// | .------.        |  P1 -> P2 : deny
// | |  P1  .        |  P2 -> P1 : deny
// | '------'\       |
// |          \      |
// |        .--'---. |
// |        |  P2  | |
// |        '------' |
// '-----------------'
//
// clang-format off
// clang-format on
