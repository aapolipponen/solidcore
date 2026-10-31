//! Regression for #1386: a 40 mm box rounded r8, setback 12, ease 4.
#![allow(clippy::unwrap_used)]

use solidcore::blend::fillet_executive::session::FilletSession;
use solidcore::math::Point3;
use solidcore::primitive::api_primitives::create_box;
use solidcore::topology::brep::p2::is_manifold_solid;
use solidcore::topology::brep::p5::{stitch_and_orient, StitchAndOrientOptions};
use solidcore::topology::vertex;
use solidcore::Kernel;

fn build(ease: f64, all_corners: bool, g2: bool) -> bool {
    let mut k = Kernel::default();
    k.fixes.fillet_setback_from_start_vertex = true;
    let mut b = create_box(&mut k, Point3::new(0.0, 0.0, 0.0), 40.0, 40.0, 40.0).unwrap();
    let edges: Vec<_> = b.edges().ids().collect();
    {
        let mut s = FilletSession::new(&mut k, &mut b).unwrap();
        s.set_blend_curve_at_thirds(g2);
        let (section, continuity) = if g2 { (2, 2) } else { (1, 1) };
        s.prepare_constant_radius_edges(&mut k, &edges, 8.0, section, continuity, 1.0)
            .unwrap();
        s.create_corners(&k, None).unwrap();
        for corner in s.fillet_corners().unwrap() {
            let v = s.corner(corner).unwrap().vertex.unwrap();
            let p = vertex::point(s.target(), v).unwrap();
            let selected = [
                Point3::new(40.0, 0.0, 0.0),
                Point3::new(0.0, 40.0, 0.0),
                Point3::new(40.0, 40.0, 0.0),
                Point3::new(40.0, 40.0, 40.0),
            ]
            .iter()
            .any(|q| p.distance(*q) < 1e-6);
            if all_corners || selected {
                s.set_corner_setback(corner, 12.0).unwrap();
                if all_corners || p.distance(Point3::new(40.0, 40.0, 0.0)) < 1e-6 {
                    s.set_corner_ease(corner, ease).unwrap();
                }
            }
        }
        s.do_filleting(&mut k, None).unwrap();
    }
    stitch_and_orient(&mut k, &mut b, &StitchAndOrientOptions::default()).unwrap();
    is_manifold_solid(&b).unwrap()
}

#[test]
fn all_corners_eased_four_builds_a_solid() {
    assert!(build(4.0, true, false));
}

#[test]
fn reported_four_corner_layout_builds_a_solid() {
    assert!(build(4.0, false, false));
}

#[test]
fn other_eases_stay_solid() {
    for ease in [0.0, 3.0, 6.0] {
        assert!(build(ease, true, false), "ease {ease}");
    }
}

#[test]
fn g2_at_four_stays_solid() {
    assert!(build(4.0, true, true));
}
