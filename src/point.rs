// Copyright 2013 The Lmath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use dim::Dimensional;
use vec::{Vec2, Vec3};

mod dim_macros;

/// A geometric point
pub trait Point<T,V>: Eq + ApproxEq<T> + ToStr {
    pub fn translate(&self, offset: &V) -> Self;
    pub fn distance(&self, other: &Self) -> T;
}

/// A two-dimensional point
#[deriving(Eq)]
pub struct Point2<T>(Vec2<T>);

impl_dimensional!(Point2, T, 2)
impl_dimensional_fns!(Point2, T, 2)
impl_approx!(Point2)

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2(Vec2::new(x, y))
    }
}

impl<T:Copy + Real> Point<T,Vec2<T>> for Point2<T> {
    pub fn translate(&self, offset: &Vec2<T>) -> Point2<T> {
        Point2(self.add_v(offset))
    }

    pub fn distance(&self, other: &Point2<T>) -> T {
        (**self).distance(&**other)
    }
}

impl<T> ToStr for Point2<T> {
    pub fn to_str(&self) -> ~str {
        fmt!("[%?, %?]", self.x, self.y)
    }
}

/// A three-dimensional point
#[deriving(Eq)]
pub struct Point3<T>(Vec3<T>);

impl_dimensional!(Point3, T, 3)
impl_dimensional_fns!(Point3, T, 3)
impl_approx!(Point3)

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3(Vec3::new(x, y, z))
    }
}

impl<T:Copy + Real> Point<T,Vec3<T>> for Point3<T> {
    pub fn translate(&self, offset: &Vec3<T>) -> Point3<T> {
        Point3(self.add_v(offset))
    }

    pub fn distance(&self, other: &Point3<T>) -> T {
        (**self).distance(&**other)
    }
}

impl<T> ToStr for Point3<T> {
    pub fn to_str(&self) -> ~str {
        fmt!("[%?, %?, %?]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test_point2 {
    use point::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Point2::new(1, 2).to_str(), ~"[1, 2]");
    }
}

#[cfg(test)]
mod test_point3 {
    use point::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Point3::new(1, 2, 3).to_str(), ~"[1, 2, 3]");
    }
}
