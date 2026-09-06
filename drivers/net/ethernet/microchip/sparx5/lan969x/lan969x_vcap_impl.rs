//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/lan969x/lan969x_vcap_impl.c
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


// SPDX-License-Identifier: GPL-2.0+

    const struct sparx5_vcap_inst lan969x_vcap_inst_cfg[] = {
    {
    .vtype = VCAP_TYPE_IS0, /* CLM-0 */
    .vinst = 0,
    .map_id = 1,
    .lookups = SPARX5_IS0_LOOKUPS,
    .lookups_per_instance = SPARX5_IS0_LOOKUPS / 3,
    .first_cid = SPARX5_VCAP_CID_IS0_L0,
    .last_cid = SPARX5_VCAP_CID_IS0_L2 - 1,
    .blockno = 2,
    .blocks = 1,
    .ingress = true,
    },
    {
    .vtype = VCAP_TYPE_IS0, /* CLM-1 */
    .vinst = 1,
    .map_id = 2,
    .lookups = SPARX5_IS0_LOOKUPS,
    .lookups_per_instance = SPARX5_IS0_LOOKUPS / 3,
    .first_cid = SPARX5_VCAP_CID_IS0_L2,
    .last_cid = SPARX5_VCAP_CID_IS0_L4 - 1,
    .blockno = 3,
    .blocks = 1,
    .ingress = true,
    },
    {
    .vtype = VCAP_TYPE_IS0, /* CLM-2 */
    .vinst = 2,
    .map_id = 3,
    .lookups = SPARX5_IS0_LOOKUPS,
    .lookups_per_instance = SPARX5_IS0_LOOKUPS / 3,
    .first_cid = SPARX5_VCAP_CID_IS0_L4,
    .last_cid = SPARX5_VCAP_CID_IS0_MAX,
    .blockno = 4,
    .blocks = 1,
    .ingress = true,
    },
    {
    .vtype = VCAP_TYPE_IS2, /* IS2-0 */
    .vinst = 0,
    .map_id = 4,
    .lookups = SPARX5_IS2_LOOKUPS,
    .lookups_per_instance = SPARX5_IS2_LOOKUPS / 2,
    .first_cid = SPARX5_VCAP_CID_IS2_L0,
    .last_cid = SPARX5_VCAP_CID_IS2_L2 - 1,
    .blockno = 0,
    .blocks = 1,
    .ingress = true,
    },
    {
    .vtype = VCAP_TYPE_IS2, /* IS2-1 */
    .vinst = 1,
    .map_id = 5,
    .lookups = SPARX5_IS2_LOOKUPS,
    .lookups_per_instance = SPARX5_IS2_LOOKUPS / 2,
    .first_cid = SPARX5_VCAP_CID_IS2_L2,
    .last_cid = SPARX5_VCAP_CID_IS2_MAX,
    .blockno = 1,
    .blocks = 1,
    .ingress = true,
    },
    {
    .vtype = VCAP_TYPE_ES0,
    .lookups = SPARX5_ES0_LOOKUPS,
    .lookups_per_instance = SPARX5_ES0_LOOKUPS,
    .first_cid = SPARX5_VCAP_CID_ES0_L0,
    .last_cid = SPARX5_VCAP_CID_ES0_MAX,
    .count = 1536,
    .ingress = false,
    },
    {
    .vtype = VCAP_TYPE_ES2,
    .lookups = SPARX5_ES2_LOOKUPS,
    .lookups_per_instance = SPARX5_ES2_LOOKUPS,
    .first_cid = SPARX5_VCAP_CID_ES2_L0,
    .last_cid = SPARX5_VCAP_CID_ES2_MAX,
    .count = 1024,
    .ingress = false,
    },
    };
