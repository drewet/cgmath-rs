// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
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


extern crate cgmath;

use cgmath::{Point, Point3, Vector, Vector3};
use cgmath::{Bound, Relation, Plane};
use cgmath::{ApproxEq};

#[test]
fn test_homogeneous() {
	let p = Point3::new(1.0f64, 2.0f64, 3.0f64);
    assert!(p.approx_eq( &Point3::from_homogeneous( &p.to_homogeneous() ) ));
}

#[test]
fn test_bound() {
    let point = Point3::new(1f32, 2.0, 3.0);
    let normal = Vector3::new(0f32, -0.8, -0.36);
    let plane = Plane::from_point_normal(point, normal);

    assert_eq!(point.relate_plane(&plane), Relation::Cross);
    assert_eq!(point.add_v(&normal).relate_plane(&plane), Relation::In);
    assert_eq!(point.add_v(&normal.mul_s(-1.0)).relate_plane(&plane), Relation::Out);
}
