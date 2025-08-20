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

/// Moving average filter structure
pub struct MovingAverage<const N: usize> {
    buffer: [f32; N],
    index: usize,
    count: usize,
    sum: f32,
}

impl<const N: usize> MovingAverage<N> {
    /// Create a new moving average filter
    pub const fn new() -> Self {
        Self {
            buffer: [0.0; N],
            index: 0,
            count: 0,
            sum: 0.0,
        }
    }

    /// Add a new value and return the current average
    pub fn add(&mut self, value: f32) -> f32 {
        if self.count < N {
            // Buffer is not full yet
            self.sum += value;
            self.buffer[self.count] = value;
            self.count += 1;
        } else {
            // Buffer is full, replace the oldest value
            self.sum -= self.buffer[self.index];
            self.sum += value;
            self.buffer[self.index] = value;
            self.index = (self.index + 1) % N;
        }

        self.sum / (self.count as f32)
    }

    /// Get the current average without adding a new value
    pub fn average(&self) -> f32 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / (self.count as f32)
        }
    }

    /// Reset the filter state
    pub fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.sum = 0.0;
    }

    /// Get the number of valid data points
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if the filter is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let mut ma: MovingAverage<3> = MovingAverage::new();

        // Add the first value
        assert_eq!(ma.add(1.0), 1.0);

        // Add the second value
        assert_eq!(ma.add(2.0), 1.5);

        // Add the third value
        assert_eq!(ma.add(3.0), 2.0);

        // Add the fourth value, replacing the first value
        assert_eq!(ma.add(4.0), 3.0); // (2+3+4)/3 = 3.0
    }

    #[test]
    fn test_reset() {
        let mut ma: MovingAverage<3> = MovingAverage::new();

        ma.add(1.0);
        ma.add(2.0);
        ma.reset();

        assert_eq!(ma.len(), 0);
        assert_eq!(ma.average(), 0.0);
    }
}
