//! Automatically rewritten from C to Rust
//! Source: lib/kunit/attributes.c
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
// KUnit API to save and access test attributes
//
// Copyright (C) 2023, Google LLC.
// Author: Rae Moar <rmoar@google.com>
//

// Options for printing attributes:
// PRINT_ALWAYS - attribute is printed for every test case and suite if set
// PRINT_SUITE - attribute is printed for every suite if set but not for test cases
// PRINT_NEVER - attribute is never printed
//
    enum print_ops {
    PRINT_ALWAYS,
    PRINT_SUITE,
    PRINT_NEVER,
    };
//
// struct kunit_attr - represents a test attribute and holds flexible
// helper functions to interact with attribute.
//
// @name: name of test attribute, eg. speed
// @get_attr: function to return attribute value given a test
// @to_string: function to return string representation of given
// attribute value
// @filter: function to indicate whether a given attribute value passes a
// filter
// @attr_default: default attribute value used during filtering
// @print: value of enum print_ops to indicate when to print attribute
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_attr {
    pub name: *const c_char,
    pub is_test): *mut *mut *mut *mut void (get_attr)(void test_or_suite, bool,
    pub to_free): *const *const *const *const char (to_string)(void attr, bool,
    pub err): *const *const *const *const int (filter)(void attr, char input, int,
    pub attr_default: *mut c_void,
    pub print: enum print_ops,
}

// String Lists for enum Attributes
    static const char * const speed_str_list[] = {"unset", "very_slow", "slow", "normal"};
// To String Methods
    static const char *attr_enum_to_string(void *attr, const char * const str_list[], bool *to_free)
    {
    let mut val: c_long = (long)attr;
// to_free = false;
    if (!val)
    return core::ptr::null_mut();
    return str_list[val];
    }
    static const char *attr_bool_to_string(void *attr, bool *to_free)
    {
    let mut val: bool = (bool)attr;
// to_free = false;
    if (val)
    return "true";
    return "false";
    }
    static const char *attr_speed_to_string(void *attr, bool *to_free)
    {
    return attr_enum_to_string(attr, speed_str_list, to_free);
    }
    static const char *attr_string_to_string(void *attr, bool *to_free)
    {
// to_free = false;
    return (char *) attr;
    }
// Filter Methods
    static const char op_list[] = "<>!=";
//
// Returns whether the inputted integer value matches the filter given
// by the operation string and inputted integer.
//
#[no_mangle]
unsafe extern "C" fn int_filter(val: c_long, op: *const c_char, input: c_int, err: *mut c_int) -> c_int {
    static int int_filter(long val, const char *op, int input, int *err)
    {
    if (!strncmp(op, "<=", 2))
    return (val <= input);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(op, _arg: ">=", _arg: 2)) -> else {
    else if (!strncmp(op, ">=", 2))
    return (val >= input);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(op, _arg: "!=", _arg: 2)) -> else {
    else if (!strncmp(op, "!=", 2))
    return (val != input);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(op, _arg: ">", _arg: 1)) -> else {
    else if (!strncmp(op, ">", 1))
    return (val > input);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(op, _arg: "<", _arg: 1)) -> else {
    else if (!strncmp(op, "<", 1))
    return (val < input);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(op, _arg: "=", _arg: 1)) -> else {
    else if (!strncmp(op, "=", 1))
    return (val == input);
// err = -EINVAL;
    pr_err("kunit executor: invalid filter operation: %s\n", op);
    return false;
    }
//
// Returns whether the inputted enum value "attr" matches the filter given
// by the input string. Note: the str_list includes the corresponding string
// list to the enum values.
//
    static int attr_enum_filter(void *attr, const char *input, int *err,
    const char * const str_list[], int max)
    {
    int i, j, input_int = -1;
    let mut test_val: c_long = (long)attr;
    const char *input_val = core::ptr::null_mut();
    for (i = 0; input[i]; i++) {
    if (!strchr(op_list, input[i])) {
    input_val = input + i;
    break;
    }
    }
    if (!input_val) {
// err = -EINVAL;
    pr_err("kunit executor: filter value not found: %s\n", input);
    return false;
    }
    for (j = 0; j <= max; j++) {
    if (!strcmp(input_val, str_list[j]))
    input_int = j;
    }
    if (input_int < 0) {
// err = -EINVAL;
    pr_err("kunit executor: invalid filter input: %s\n", input);
    return false;
    }
    return int_filter(test_val, input, input_int, err);
    }
#[no_mangle]
unsafe extern "C" fn attr_speed_filter(attr: *mut c_void, input: *const c_char, err: *mut c_int) -> c_int {
    static int attr_speed_filter(void *attr, const char *input, int *err)
    {
    return attr_enum_filter(attr, input, err, speed_str_list, KUNIT_SPEED_MAX);
    }
//
// Returns whether the inputted string value (attr) matches the filter given
// by the input string.
//
#[no_mangle]
unsafe extern "C" fn attr_string_filter(attr: *mut c_void, input: *const c_char, err: *mut c_int) -> c_int {
    static int attr_string_filter(void *attr, const char *input, int *err)
    {
    char *str = attr;
    if (!strncmp(input, "<", 1)) {
// err = -EINVAL;
    pr_err("kunit executor: invalid filter input: %s\n", input);
    return false;
    } else if (!strncmp(input, ">", 1)) {
// err = -EINVAL;
    pr_err("kunit executor: invalid filter input: %s\n", input);
    return false;
    } else if (!strncmp(input, "!=", 2)) {
    return (strcmp(input + 2, str) != 0);
    } else if (!strncmp(input, "=", 1)) {
    return (strcmp(input + 1, str) == 0);
    }
// err = -EINVAL;
    pr_err("kunit executor: invalid filter operation: %s\n", input);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn attr_bool_filter(attr: *mut c_void, input: *const c_char, err: *mut c_int) -> c_int {
    static int attr_bool_filter(void *attr, const char *input, int *err)
    {
    int i, input_int = -1;
    let mut val: c_long = (long)attr;
    const char *input_str = core::ptr::null_mut();
    for (i = 0; input[i]; i++) {
    if (!strchr(op_list, input[i])) {
    input_str = input + i;
    break;
    }
    }
    if (!input_str) {
// err = -EINVAL;
    pr_err("kunit executor: filter value not found: %s\n", input);
    return false;
    }
    if (!strcmp(input_str, "true"))
    input_int = (int)true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(input_str, _arg: "false")) -> else {
    else if (!strcmp(input_str, "false"))
    input_int = (int)false;
    else {
// err = -EINVAL;
    pr_err("kunit executor: invalid filter input: %s\n", input);
    return false;
    }
    return int_filter(val, input, input_int, err);
    }
// Get Attribute Methods
    static void *attr_speed_get(void *test_or_suite, bool is_test)
    {
    struct kunit_suite *suite = is_test ? core::ptr::null_mut() : test_or_suite;
    struct kunit_case *test = is_test ? test_or_suite : core::ptr::null_mut();
    if (test)
    return ((void *) test.attr.speed);
    else
    return ((void *) suite.attr.speed);
    }
    static void *attr_module_get(void *test_or_suite, bool is_test)
    {
    struct kunit_suite *suite = is_test ? core::ptr::null_mut() : test_or_suite;
    struct kunit_case *test = is_test ? test_or_suite : core::ptr::null_mut();
// Suites get their module attribute from their first test_case
    if (test)
    return ((void *) test.module_name);
#[no_mangle]
pub unsafe extern "C" fn if(0: kunit_suite_num_test_cases(suite) >) -> else {
    else if (kunit_suite_num_test_cases(suite) > 0)
    return ((void *) suite.test_cases[0].module_name);
    else
    return (void *) "";
    }
    static void *attr_is_init_get(void *test_or_suite, bool is_test)
    {
    struct kunit_suite *suite = is_test ? core::ptr::null_mut() : test_or_suite;
    struct kunit_case *test = is_test ? test_or_suite : core::ptr::null_mut();
    if (test)
    return ((void *) core::ptr::null_mut());
    else
    return ((void *) suite.is_init);
    }
// List of all Test Attributes
    static struct kunit_attr kunit_attr_list[] = {
    {
    .name = "speed",
    .get_attr = attr_speed_get,
    .to_string = attr_speed_to_string,
    .filter = attr_speed_filter,
    .attr_default = (void *)KUNIT_SPEED_NORMAL,
    .print = PRINT_ALWAYS,
    },
    {
    .name = "module",
    .get_attr = attr_module_get,
    .to_string = attr_string_to_string,
    .filter = attr_string_filter,
    .attr_default = (void *)"",
    .print = PRINT_SUITE,
    },
    {
    .name = "is_init",
    .get_attr = attr_is_init_get,
    .to_string = attr_bool_to_string,
    .filter = attr_bool_filter,
    .attr_default = (void *)false,
    .print = PRINT_SUITE,
    }
    };
// Helper Functions to Access Attributes
    const char *kunit_attr_filter_name(struct kunit_attr_filter filter)
    {
    return filter.attr.name;
    }
#[no_mangle]
pub unsafe extern "C" fn kunit_print_attr(test_or_suite: *mut c_void, is_test: bool, test_level: c_uint) {
    void kunit_print_attr(void *test_or_suite, bool is_test, unsigned int test_level)
    {
    int i;
    let mut to_free: bool = false;
    void *attr;
    const char *attr_name, *attr_str;
    struct kunit_suite *suite = is_test ? core::ptr::null_mut() : test_or_suite;
    struct kunit_case *test = is_test ? test_or_suite : core::ptr::null_mut();
    for (i = 0; i < ARRAY_SIZE(kunit_attr_list); i++) {
    if (kunit_attr_list[i].print == PRINT_NEVER ||
    (test && kunit_attr_list[i].print == PRINT_SUITE))
    continue;
    attr = kunit_attr_list[i].get_attr(test_or_suite, is_test);
    if (attr) {
    attr_name = kunit_attr_list[i].name;
    attr_str = kunit_attr_list[i].to_string(attr, &to_free);
    if (test) {
    kunit_log(KERN_INFO, test, "%*s# %s.%s: %s",
    KUNIT_INDENT_LEN * test_level, "", test.name,
    attr_name, attr_str);
    } else {
    kunit_log(KERN_INFO, suite, "%*s# %s: %s",
    KUNIT_INDENT_LEN * test_level, "", attr_name, attr_str);
    }
// Free to_string of attribute if needed
    if (to_free)
    kfree(attr_str);
    }
    }
    }
// Helper Functions to Filter Attributes
#[no_mangle]
pub unsafe extern "C" fn kunit_get_filter_count(input: *mut c_char) -> c_int {
    int kunit_get_filter_count(char *input)
    {
    int i, comma_index = 0, count = 0;
    for (i = 0; input[i]; i++) {
    if (input[i] == ',') {
    if ((i - comma_index) > 1)
    count++;
    comma_index = i;
    }
    }
    if ((i - comma_index) > 0)
    count++;
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn kunit_next_attr_filter(filters: *mut c_char, err: *mut c_int) -> kunit_attr_filter {
    struct kunit_attr_filter kunit_next_attr_filter(char **filters, int *err)
    {
    let mut filter: kunit_attr_filter = {};
    int i, j, comma_index = 0, new_start_index = 0;
    let mut op_index: c_int = -1, attr_index = -1;
    char op;
    char *input = *filters;
// Parse input until operation
    for (i = 0; input[i]; i++) {
    if (op_index < 0 && strchr(op_list, input[i])) {
    op_index = i;
    } else if (!comma_index && input[i] == ',') {
    comma_index = i;
    } else if (comma_index && input[i] != ' ') {
    new_start_index = i;
    break;
    }
    }
    if (op_index <= 0) {
// err = -EINVAL;
    pr_err("kunit executor: filter operation not found: %s\n", input);
    return filter;
    }
// Temporarily set operator to \0 character.
    op = input[op_index];
    input[op_index] = '\0';
// Find associated kunit_attr object
    for (j = 0; j < ARRAY_SIZE(kunit_attr_list); j++) {
    if (!strcmp(input, kunit_attr_list[j].name)) {
    attr_index = j;
    break;
    }
    }
    input[op_index] = op;
    if (attr_index < 0) {
// err = -EINVAL;
    pr_err("kunit executor: attribute not found: %s\n", input);
    } else {
    filter.attr = &kunit_attr_list[attr_index];
    }
    if (comma_index > 0) {
    input[comma_index] = '\0';
    filter.input = input + op_index;
    input = input + new_start_index;
    } else {
    filter.input = input + op_index;
    input = core::ptr::null_mut();
    }
// filters = input;
    return filter;
    }
    struct kunit_suite *kunit_filter_attr_tests(const struct kunit_suite *const suite,
    struct kunit_attr_filter filter, char *action, int *err)
    {
    let mut n: c_int = 0;
    struct kunit_case *filtered, *test_case;
    struct kunit_suite *copy;
    void *suite_val, *test_val;
    bool suite_result, test_result, default_result, result;
// Allocate memory for new copy of suite and list of test cases
    copy = kmemdup(suite, sizeof(*copy), GFP_KERNEL);
    if (!copy)
    return ERR_PTR(-ENOMEM);
    kunit_suite_for_each_test_case(suite, test_case) { n++; }
    filtered = kzalloc_objs(*filtered, n + 1);
    if (!filtered) {
    kfree(copy);
    return ERR_PTR(-ENOMEM);
    }
    n = 0;
// Save filtering result on default value
    default_result = filter.attr.filter(filter.attr.attr_default, filter.input, err);
    if (*err)
    goto err;
// Save suite attribute value and filtering result on that value
    suite_val = filter.attr.get_attr((void *)suite, false);
    suite_result = filter.attr.filter(suite_val, filter.input, err);
    if (*err)
    goto err;
// For each test case, save test case if passes filtering.
    kunit_suite_for_each_test_case(suite, test_case) {
    test_val = filter.attr.get_attr((void *) test_case, true);
    test_result = filter.attr.filter(filter.attr.get_attr(test_case, true),
    filter.input, err);
    if (*err)
    goto err;
//
// If attribute value of test case is set, filter on that value.
// If not, filter on suite value if set. If not, filter on
// default value.
//
    result = false;
    if (test_val) {
    if (test_result)
    result = true;
    } else if (suite_val) {
    if (suite_result)
    result = true;
    } else if (default_result) {
    result = true;
    }
    if (result) {
    filtered[n++] = *test_case;
    } else if (action && strcmp(action, "skip") == 0) {
    test_case.status = KUNIT_SKIPPED;
    filtered[n++] = *test_case;
    }
    }
    err:
    if (n == 0 || *err) {
    kfree(copy);
    kfree(filtered);
    return core::ptr::null_mut();
    }
    copy.test_cases = filtered;
    return copy;
    }
