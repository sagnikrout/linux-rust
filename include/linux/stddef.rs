//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stddef.h
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
// sizeof_field() - Report the size of a struct field in bytes
//
// @TYPE: The structure containing the field of interest
// @MEMBER: The field to return the size of
//

//
// offsetofend() - Report the offset of a struct field within the struct
//
// @TYPE: The type of the structure
// @MEMBER: The member within the structure to get the end offset of
//

//
// struct_group() - Wrap a set of declarations in a mirrored struct
//
// @NAME: The identifier name of the mirrored sub-struct
// @MEMBERS: The member declarations for the mirrored structs
//
// Used to create an anonymous union of two structs with identical
// layout and size: one anonymous and one named. The former can be
// used normally without sub-struct naming, and the latter can be
// used to reason about the start, end, and size of the group of
// struct members.
//

//
// struct_group_attr() - Create a struct_group() with trailing attributes
//
// @NAME: The identifier name of the mirrored sub-struct
// @ATTRS: Any struct attributes to apply
// @MEMBERS: The member declarations for the mirrored structs
//
// Used to create an anonymous union of two structs with identical
// layout and size: one anonymous and one named. The former can be
// used normally without sub-struct naming, and the latter can be
// used to reason about the start, end, and size of the group of
// struct members. Includes structure attributes argument.
//

//
// struct_group_tagged() - Create a struct_group with a reusable tag
//
// @TAG: The tag name for the named sub-struct
// @NAME: The identifier name of the mirrored sub-struct
// @MEMBERS: The member declarations for the mirrored structs
//
// Used to create an anonymous union of two structs with identical
// layout and size: one anonymous and one named. The former can be
// used normally without sub-struct naming, and the latter can be
// used to reason about the start, end, and size of the group of
// struct members. Includes struct tag argument for the named copy,
// so the specified layout can be reused later.
//

//
// DECLARE_FLEX_ARRAY() - Declare a flexible array usable in a union
//
// @TYPE: The type of each flexible array element
// @NAME: The name of the flexible array member
//
// In order to have a flexible array member in a union or alone in a
// struct, it needs to be wrapped in an anonymous struct with at least 1
// named member, but that member can be empty.
//

//
// __TRAILING_OVERLAP() - Overlap a flexible-array member with trailing
// members.
//
// Creates a union between a flexible-array member (FAM) in a struct and a set
// of additional members that would otherwise follow it.
//
// Beware that, as this helper encloses TYPE NAME and MEMBERS in the same
// union, designated initializers for MEMBERS may overwrite portions
// previously initialized through NAME.
//
// For example::
//
// struct flex {
// size_t count;
// u8 fam[];
// };
//
// struct composite {
// ...
// __TRAILING_OVERLAP(struct flex, flex, fam, __packed,
// u8 data;
// );
// } __packed;
//
// static struct composite comp = {
// .flex = {
// .count = 1,
// },
// .data = 2,
// };
//
// In the example above, .flex and .data initialize different views of the same
// union storage. Since .data is initialized last, it _may_ overwrite portions
// previously initialized through .flex, leading to .flex.count being zeroed
// out.
//
// A couple of alternatives are shown below.
//
// a) Initialize only one view of the overlapped storage and assign the rest
// at runtime::
//
// static struct composite comp = {
// .flex = {
// .count = 1,
// },
// };
//
// static void foo(void)
// {
// comp.data = 2;
// ...
// }
//
// b) Alternatively, replace designated initializers with runtime assignments::
//
// static void foo(void)
// {
// struct composite comp;
//
// comp.flex.count = 1;
// comp.data = 2;
// ...
// }
//
// Compiler Explorer test code: https://godbolt.org/z/voM4E36dT
//
// For another example of the above see commit 5e54510a9389 ("acpi: nfit:
// intel: avoid multiple -Wflex-array-member-not-at-end warnings")
//
// Link: https://git.kernel.org/linus/5e54510a9389caa9
//
// @TYPE: Flexible structure type name, including "struct" keyword.
// @NAME: Name for a variable to define.
// @FAM: The flexible-array member within @TYPE
// @ATTRS: Any struct attributes (usually empty)
// @MEMBERS: Trailing overlapping members.
//

//
// TRAILING_OVERLAP() - Overlap a flexible-array member with trailing members.
//
// Creates a union between a flexible-array member (FAM) in a struct and a set
// of additional members that would otherwise follow it.
//
// @TYPE: Flexible structure type name, including "struct" keyword.
// @NAME: Name for a variable to define.
// @FAM: The flexible-array member within @TYPE
// @MEMBERS: Trailing overlapping members.
//

