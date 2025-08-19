// A `no_std` and no `alloc` library for more efficient array processing.
// Copyright (C) 2025  joker2770

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use core::cmp::Ordering;

/// Generic array min/max value and index calculation
///
/// Supports signed integers, unsigned integers, and floating-point numbers (handles NaN correctly)
///
/// # Trait requirements
/// - `T: PartialOrd + Copy`: Elements must be comparable and copyable
pub struct ArrayExtrema;

/// Check if the value is valid (handles floating-point NaN)
fn is_valid<T: PartialOrd>(value: T) -> bool {
    // Use `value != value` trick to detect NaN
    value.eq(&value)
}

/// Generic comparison function (handles floating-point NaN correctly)
fn compare_items<T: PartialOrd>(a: T, b: T) -> Ordering {
    match a.partial_cmp(&b) {
        Some(ordering) => ordering,
        None => {
            // Handle NaN cases
            if !a.eq(&a) {
                Ordering::Less
            } else if !b.eq(&b) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

/// Extremum result, containing value and index
#[derive(Debug, PartialEq)]
pub struct Extremum<T> {
    pub value: T,
    pub index: usize,
}

/// Extremum pair, containing min and max values
#[derive(Debug, PartialEq)]
pub struct ExtremumPair<T> {
    pub min: Extremum<T>,
    pub max: Extremum<T>,
}

impl ArrayExtrema {
    /// Find both the minimum and maximum values and their indices in the array
    ///
    /// # Parameters
    /// - `arr`: The array slice to process
    ///
    /// # Return value
    /// - `Some(ExtremumPair<T>)`: The found min and max values and their indices
    /// - `None`: Empty array or all elements are NaN
    pub fn min_max_with_indices<T>(arr: &[T]) -> Option<ExtremumPair<T>>
    where
        T: PartialOrd + Copy,
    {
        if arr.is_empty() {
            return None;
        }

        let mut min = Extremum {
            value: arr[0],
            index: 0,
        };
        let mut max = Extremum {
            value: arr[0],
            index: 0,
        };
        let mut found_valid = false;

        for (i, &item) in arr.iter().enumerate() {
            if !is_valid(item) {
                continue;
            }

            if !found_valid {
                min.value = item;
                min.index = i;
                max.value = item;
                max.index = i;
                found_valid = true;
                continue;
            }

            if compare_items(item, min.value) == Ordering::Less {
                min.value = item;
                min.index = i;
            }

            if compare_items(item, max.value) == Ordering::Greater {
                max.value = item;
                max.index = i;
            }
        }

        if found_valid {
            Some(ExtremumPair { min, max })
        } else {
            None
        }
    }

    /// Find the minimum value and its index in the array
    pub fn min_with_index<T>(arr: &[T]) -> Option<Extremum<T>>
    where
        T: PartialOrd + Copy,
    {
        if arr.is_empty() {
            return None;
        }

        let mut min = Extremum {
            value: arr[0],
            index: 0,
        };
        let mut found_valid = false;

        for (i, &item) in arr.iter().enumerate() {
            if !is_valid(item) {
                continue;
            }

            if !found_valid {
                min.value = item;
                min.index = i;
                found_valid = true;
                continue;
            }

            if compare_items(item, min.value) == Ordering::Less {
                min.value = item;
                min.index = i;
            }
        }

        if found_valid { Some(min) } else { None }
    }

    /// Find the maximum value and its index in the array
    pub fn max_with_index<T>(arr: &[T]) -> Option<Extremum<T>>
    where
        T: PartialOrd + Copy,
    {
        if arr.is_empty() {
            return None;
        }

        let mut max = Extremum {
            value: arr[0],
            index: 0,
        };
        let mut found_valid = false;

        for (i, &item) in arr.iter().enumerate() {
            if !is_valid(item) {
                continue;
            }

            if !found_valid {
                max.value = item;
                max.index = i;
                found_valid = true;
                continue;
            }

            if compare_items(item, max.value) == Ordering::Greater {
                max.value = item;
                max.index = i;
            }
        }

        if found_valid { Some(max) } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_integers() {
        let arr = [-5, 10, 3, -8, 0, 10];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, -8);
        assert_eq!(result.min.index, 3);
        assert_eq!(result.max.value, 10);
        assert!(result.max.index == 1 || result.max.index == 5);
    }

    #[test]
    fn test_unsigned_integers() {
        let arr = [5u32, 10, 3, 8, 0, 8, 8];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, 0);
        assert_eq!(result.min.index, 4);
        assert_eq!(result.max.value, 10);
        assert_eq!(result.max.index, 1);
    }

    #[test]
    fn test_floats() {
        let arr = [1.5, 3.2, 2.8, 4.7, 2.8];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, 1.5);
        assert_eq!(result.min.index, 0);
        assert_eq!(result.max.value, 4.7);
        assert_eq!(result.max.index, 3);
    }

    #[test]
    fn test_floats_with_nan() {
        let arr = [1.5, f64::NAN, 3.2, 2.8, f64::NAN];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, 1.5);
        assert_eq!(result.min.index, 0);
        assert_eq!(result.max.value, 3.2);
        assert_eq!(result.max.index, 2);
    }

    #[test]
    fn test_all_nan() {
        let arr = [f32::NAN; 5];
        assert!(ArrayExtrema::min_max_with_indices(&arr).is_none());
        assert!(ArrayExtrema::min_with_index(&arr).is_none());
        assert!(ArrayExtrema::max_with_index(&arr).is_none());
    }

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert!(ArrayExtrema::min_max_with_indices(&arr).is_none());
        assert!(ArrayExtrema::min_with_index(&arr).is_none());
        assert!(ArrayExtrema::max_with_index(&arr).is_none());
    }

    #[test]
    fn test_single_element() {
        let arr = [42];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, 42);
        assert_eq!(result.min.index, 0);
        assert_eq!(result.max.value, 42);
        assert_eq!(result.max.index, 0);
    }

    #[test]
    fn test_multiple_max_values() {
        let arr = [1, 5, 2, 5, 3];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, 1);
        assert_eq!(result.min.index, 0);
        assert_eq!(result.max.value, 5);
        assert!(result.max.index == 1 || result.max.index == 3);
    }

    #[test]
    fn test_multiple_min_values() {
        let arr = [2, 1, 3, 1, 4];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, 1);
        assert!(result.min.index == 1 || result.min.index == 3);
        assert_eq!(result.max.value, 4);
        assert_eq!(result.max.index, 4);
    }

    #[test]
    fn test_individual_min_function() {
        let arr = [1.5, 3.2, 2.8, 4.7, 2.8];
        let min = ArrayExtrema::min_with_index(&arr).unwrap();

        assert_eq!(min.value, 1.5);
        assert_eq!(min.index, 0);
    }

    #[test]
    fn test_individual_max_function() {
        let arr = [1.5, 3.2, 2.8, 4.7, 2.8];
        let max = ArrayExtrema::max_with_index(&arr).unwrap();

        assert_eq!(max.value, 4.7);
        assert_eq!(max.index, 3);
    }

    #[test]
    fn test_large_array() {
        let mut arr = [0; 1000];
        arr[500] = 100;
        arr[999] = -100;

        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, -100);
        assert_eq!(result.min.index, 999);
        assert_eq!(result.max.value, 100);
        assert_eq!(result.max.index, 500);
    }

    #[test]
    fn test_min_max_with_infinity() {
        let arr = [f64::NEG_INFINITY, 1.5, f64::INFINITY, 2.8];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, f64::NEG_INFINITY);
        assert_eq!(result.min.index, 0);
        assert_eq!(result.max.value, f64::INFINITY);
        assert_eq!(result.max.index, 2);
    }

    #[test]
    fn test_first_occurrence_precedence() {
        // When there are multiple identical min/max values, return the index of the first occurrence
        let arr = [5, 2, 5, 3, 2];
        let result = ArrayExtrema::min_max_with_indices(&arr).unwrap();

        assert_eq!(result.min.value, 2);
        assert_eq!(result.min.index, 1); // Index of the first 2
        assert_eq!(result.max.value, 5);
        assert_eq!(result.max.index, 0); // Index of the first 5
    }
}
