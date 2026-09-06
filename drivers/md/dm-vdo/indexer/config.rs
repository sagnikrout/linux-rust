//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/config.h
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
// Copyright 2023 Red Hat
//

//
// The uds_configuration records a variety of parameters used to configure a new UDS index. Some
// parameters are provided by the client, while others are fixed or derived from user-supplied
// values. It is created when an index is created, and it is recorded in the index metadata.
//
// A set of configuration parameters for the indexer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_configuration {
// Storage device for the index
    pub bdev: *mut block_device,
// The maximum allowable size of the index
    pub size: usize,
// The offset where the index should start
    pub offset: off_t,
// Parameters for the volume
// The volume layout
    pub geometry: index_geometry,
// Index owner's nonce
    pub nonce: u64,
// The number of threads used to process index requests
    pub zone_count: c_uint,
// The number of threads used to read volume pages
    pub read_threads: c_uint,
// Size of the page cache and sparse chapter index cache in chapters
    pub cache_chapters: u32,
// Parameters for the volume index
// The mean delta for the volume index
    pub volume_index_mean_delta: u32,
// Sampling rate for sparse indexing
    pub sparse_sample_rate: u32,
}

// On-disk structure of data for a version 8.02 index.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_configuration_8_02 {
// Smaller (16), Small (64) or large (256) indices
    pub record_pages_per_chapter: u32,
// Total number of chapters per volume
    pub chapters_per_volume: u32,
// Number of sparse chapters per volume
    pub sparse_chapters_per_volume: u32,
// Size of the page cache, in chapters
    pub cache_chapters: u32,
// Unused field
    pub unused: u32,
// The volume index mean delta to use
    pub volume_index_mean_delta: u32,
// Size of a page, used for both record pages and index pages
    pub bytes_per_page: u32,
// Sampling rate for sparse indexing
    pub sparse_sample_rate: u32,
// Index owner's nonce
    pub nonce: u64,
// Virtual chapter remapped from physical chapter 0
    pub remapped_virtual: u64,
// New physical chapter which remapped chapter was moved to
    pub remapped_physical: u64,
    pub __packed: },
// On-disk structure of data for a version 6.02 index.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uds_configuration_6_02 {
// Smaller (16), Small (64) or large (256) indices
    pub record_pages_per_chapter: u32,
// Total number of chapters per volume
    pub chapters_per_volume: u32,
// Number of sparse chapters per volume
    pub sparse_chapters_per_volume: u32,
// Size of the page cache, in chapters
    pub cache_chapters: u32,
// Unused field
    pub unused: u32,
// The volume index mean delta to use
    pub volume_index_mean_delta: u32,
// Size of a page, used for both record pages and index pages
    pub bytes_per_page: u32,
// Sampling rate for sparse indexing
    pub sparse_sample_rate: u32,
// Index owner's nonce
    pub nonce: u64,
    pub __packed: },
    pub config_ptr): *mut uds_configuration,
    pub config): *mut void uds_free_configuration(struct uds_configuration,
    pub config): *mut uds_configuration,
    pub version): *mut *mut uds_configuration config, u32,
    pub config): *mut void uds_log_configuration(struct uds_configuration,
