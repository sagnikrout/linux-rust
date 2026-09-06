//! Automatically rewritten from C to Rust
//! Source: lib/dhry_2.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// "DHRYSTONE" Benchmark Program
// -----------------------------
//
// Version:    C, Version 2.1
//
// File:       dhry_2.c (part 3 of 3)
//
// Date:       May 25, 1988
//
// Author:     Reinhold P. Weicker
//

#[no_mangle]
unsafe extern "C" fn Func_3(Enum_Par_Val: Enumeration) -> Boolean {
    static Boolean Func_3(Enumeration Enum_Par_Val)
//
// executed once
// Enum_Par_Val == Ident_3
    {
    Enumeration Enum_Loc;
    Enum_Loc = Enum_Par_Val;
    if (Enum_Loc == Ident_3) {
// then, executed
    return true;
    } else {
// not executed
    return false;
    }
    } /* Func_3 */
#[no_mangle]
pub unsafe extern "C" fn Proc_6(Enum_Val_Par: Enumeration, Enum_Ref_Par: *mut Enumeration) {
    void Proc_6(Enumeration  Enum_Val_Par, Enumeration *Enum_Ref_Par)
//
// executed once
// Enum_Val_Par == Ident_3, Enum_Ref_Par becomes Ident_2
    {
// Enum_Ref_Par = Enum_Val_Par;
    if (!Func_3(Enum_Val_Par)) {
// then, not executed
// Enum_Ref_Par = Ident_4;
    }
    switch (Enum_Val_Par) {
    case Ident_1:
// Enum_Ref_Par = Ident_1;
    break;
    case Ident_2:
    if (Int_Glob > 100) {
// then
// Enum_Ref_Par = Ident_1;
    } else {
// Enum_Ref_Par = Ident_4;
    }
    break;
    case Ident_3: /* executed */
// Enum_Ref_Par = Ident_2;
    break;
    case Ident_4:
    break;
    case Ident_5:
// Enum_Ref_Par = Ident_3;
    break;
    } /* switch */
    } /* Proc_6 */
#[no_mangle]
pub unsafe extern "C" fn Proc_7(Int_1_Par_Val: One_Fifty, Int_2_Par_Val: One_Fifty, Int_Par_Ref: *mut One_Fifty) {
    void Proc_7(One_Fifty Int_1_Par_Val, One_Fifty Int_2_Par_Val, One_Fifty *Int_Par_Ref)
//
// executed three times
// first call:      Int_1_Par_Val == 2, Int_2_Par_Val == 3,
// Int_Par_Ref becomes 7
// second call:     Int_1_Par_Val == 10, Int_2_Par_Val == 5,
// Int_Par_Ref becomes 17
// third call:      Int_1_Par_Val == 6, Int_2_Par_Val == 10,
// Int_Par_Ref becomes 18
    {
    One_Fifty Int_Loc;
    Int_Loc = Int_1_Par_Val + 2;
// Int_Par_Ref = Int_2_Par_Val + Int_Loc;
    } /* Proc_7 */
#[no_mangle]
pub unsafe extern "C" fn Proc_8(Arr_1_Par_Ref: Arr_1_Dim, Arr_2_Par_Ref: Arr_2_Dim, Int_1_Par_Val: c_int, Int_2_Par_Val: c_int) {
    void Proc_8(Arr_1_Dim Arr_1_Par_Ref, Arr_2_Dim Arr_2_Par_Ref, int Int_1_Par_Val, int Int_2_Par_Val)
//
// executed once
// Int_Par_Val_1 == 3
// Int_Par_Val_2 == 7
    {
    One_Fifty Int_Index;
    One_Fifty Int_Loc;
    Int_Loc = Int_1_Par_Val + 5;
    Arr_1_Par_Ref[Int_Loc] = Int_2_Par_Val;
    Arr_1_Par_Ref[Int_Loc+1] = Arr_1_Par_Ref[Int_Loc];
    Arr_1_Par_Ref[Int_Loc+30] = Int_Loc;
    for (Int_Index = Int_Loc; Int_Index <= Int_Loc+1; ++Int_Index)
    Arr_2_Par_Ref[Int_Loc][Int_Index] = Int_Loc;
    Arr_2_Par_Ref[Int_Loc][Int_Loc-1] += 1;
    Arr_2_Par_Ref[Int_Loc+20][Int_Loc] = Arr_1_Par_Ref[Int_Loc];
    Int_Glob = 5;
    } /* Proc_8 */
#[no_mangle]
pub unsafe extern "C" fn Func_1(Ch_1_Par_Val: Capital_Letter, Ch_2_Par_Val: Capital_Letter) -> Enumeration {
    Enumeration Func_1(Capital_Letter Ch_1_Par_Val, Capital_Letter Ch_2_Par_Val)
//
// executed three times
// first call:      Ch_1_Par_Val == 'H', Ch_2_Par_Val == 'R'
// second call:     Ch_1_Par_Val == 'A', Ch_2_Par_Val == 'C'
// third call:      Ch_1_Par_Val == 'B', Ch_2_Par_Val == 'C'
    {
    Capital_Letter Ch_1_Loc;
    Capital_Letter Ch_2_Loc;
    Ch_1_Loc = Ch_1_Par_Val;
    Ch_2_Loc = Ch_1_Loc;
    if (Ch_2_Loc != Ch_2_Par_Val) {
// then, executed
    return Ident_1;
    } else {
// not executed
    Ch_1_Glob = Ch_1_Loc;
    return Ident_2;
    }
    } /* Func_1 */
#[no_mangle]
pub unsafe extern "C" fn Func_2(Str_1_Par_Ref: Str_30, Str_2_Par_Ref: Str_30) -> Boolean {
    Boolean Func_2(Str_30 Str_1_Par_Ref, Str_30 Str_2_Par_Ref)
//
// executed once
// Str_1_Par_Ref == "DHRYSTONE PROGRAM, 1'ST STRING"
// Str_2_Par_Ref == "DHRYSTONE PROGRAM, 2'ND STRING"
    {
    One_Thirty Int_Loc;
    Capital_Letter Ch_Loc;
    Int_Loc = 2;
    while (Int_Loc <= 2) {
// loop body executed once
    if (Func_1(Str_1_Par_Ref[Int_Loc],
    Str_2_Par_Ref[Int_Loc+1]) == Ident_1) {
// then, executed
    Ch_Loc = 'A';
    Int_Loc += 1;
    }
    } /* if, while */
    if (Ch_Loc >= 'W' && Ch_Loc < 'Z') {
// then, not executed
    Int_Loc = 7;
    }
    if (Ch_Loc == 'R') {
// then, not executed
    return true;
    } else {
// executed
    if (strcmp(Str_1_Par_Ref, Str_2_Par_Ref) > 0) {
// then, not executed
    Int_Loc += 7;
    Int_Glob = Int_Loc;
    return true;
    } else {
// executed
    return false;
    }
    } /* if Ch_Loc */
    } /* Func_2 */
