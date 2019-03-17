// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//  d888b  d88888b  .d88b.  .88b  d88. d88888b d888888b d8888b. db    db      dD
// 88' Y8b 88'     .8P  Y8. 88'YbdP`88 88'     `~~88~~' 88  `8D `8b  d8'     d8'
// 88      88ooooo 88    88 88  88  88 88ooooo    88    88oobY'  `8bd8'     d8'
// 88  ooo 88~~~~~ 88    88 88  88  88 88~~~~~    88    88`8b      88      d8'
// 88. ~8~ 88.     `8b  d8' 88  88  88 88.        88    88 `88.    88     d8'
//  Y888P  Y88888P  `Y88P'  YP  YP  YP Y88888P    YP    88   YD    YP    C8'
//
// d8888b.  .d88b.  d888888b d8b   db d888888b
// 88  `8D .8P  Y8.   `88'   888o  88 `~~88~~'
// 88oodD' 88    88    88    88V8o 88    88
// 88~~~   88    88    88    88 V8o88    88
// 88      `8b  d8'   .88.   88  V888    88
// 88       `Y88P'  Y888888P VP   V8P    YP

// Transparent alias. In docs and user-facing APIs, this resolves to (U, U).
pub type PointType<U> = (U, U);

// Lightweight data type to represent a point. Should be passed by value.
#[derive(PartialEq, Clone, Copy)]
pub struct Point<U> {
    inner: PointType<U>,
}

impl<U> std::fmt::Debug for Point<U>
where
    U: num::PrimInt + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl<U> From<PointType<U>> for Point<U>
where
    U: num::PrimInt,
{
    fn from(xy: PointType<U>) -> Self {
        Point { inner: xy }
    }
}

impl<U> Into<PointType<U>> for Point<U>
where
    U: num::PrimInt,
{
    fn into(self) -> PointType<U> {
        self.inner
    }
}

impl<U> std::ops::Add for Point<U>
where
    U: num::PrimInt,
{
    type Output = Point<U>;
    fn add(self, other: Point<U>) -> Point<U> {
        let new_x: U = self.x() + other.x();
        let new_y: U = self.y() + other.y();
        Point {
            inner: (new_x, new_y),
        }
    }
}

impl<U> std::ops::Sub for Point<U>
where
    U: num::PrimInt,
{
    type Output = Point<U>;
    fn sub(self, other: Point<U>) -> Point<U> {
        let new_x: U = self.x() - other.x();
        let new_y: U = self.y() - other.y();
        Point {
            inner: (new_x, new_y),
        }
    }
}

impl<U> Point<U>
where
    U: num::PrimInt,
{
    // Accessors //
    pub fn x(&self) -> U {
        self.inner.0
    }

    pub fn y(&self) -> U {
        self.inner.1
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn xy_addition() {
        debug_assert_eq!(Point::from((0, 0)) + Point::from((0, 1)), (0, 1).into());
        debug_assert_eq!(Point::from((0, 1)) + Point::from((0, 1)), (0, 2).into());
        debug_assert_eq!(Point::from((1, 1)) + Point::from((0, 0)), (1, 1).into());
        debug_assert_eq!(Point::from((1, 0)) + Point::from((0, 1)), (1, 1).into());
        debug_assert_eq!(Point::from((0, 0)) + Point::from((4, 5)), (4, 5).into());
        debug_assert_eq!(Point::from((4, 5)) + Point::from((0, 0)), (4, 5).into());
    }

    #[test]
    fn xy_subtraction() {
        debug_assert_eq!(Point::from((0, 1)) - Point::from((0, 0)), (0, 1).into());
        debug_assert_eq!(Point::from((0, 1)) - Point::from((0, 1)), (0, 0).into());
        debug_assert_eq!(Point::from((1, 1)) - Point::from((0, 0)), (1, 1).into());
        debug_assert_eq!(Point::from((1, 1)) - Point::from((0, 1)), (1, 0).into());
        debug_assert_eq!(Point::from((4, 5)) - Point::from((2, 2)), (2, 3).into());
        debug_assert_eq!(Point::from((4, 5)) - Point::from((0, 0)), (4, 5).into());
    }

    // Test addition / subtraction which reaches into the realm of negative numbers.

    #[test]
    fn subtracting_positive_numbers() {
        debug_assert_eq!(Point::from((0, 0)) - (1, 1).into(), (-1, -1).into());
        debug_assert_eq!(Point::from((0, 0)) - (0, 1).into(), (0, -1).into());
        debug_assert_eq!(Point::from((0, 0)) - (1, 0).into(), (-1, 0).into());

        debug_assert_eq!(Point::from((1, 10)) - (2, 20).into(), (-1, -10).into());
    }

    #[test]
    fn adding_negative_numbers() {
        debug_assert_eq!(Point::from((0, 0)) + (-1, 0).into(), (-1, 0).into());
        debug_assert_eq!(Point::from((0, 0)) + (-1, -1).into(), (-1, -1).into());
        debug_assert_eq!(Point::from((0, 0)) + (0, -1).into(), (0, -1).into());

        debug_assert_eq!(Point::from((1, 10)) + (-2, -20).into(), (-1, -10).into());
    }
}
