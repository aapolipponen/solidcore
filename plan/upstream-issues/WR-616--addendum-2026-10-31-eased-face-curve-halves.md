Updates: WR-616

### Eased corner face-curve halves (#1386)

The ease extension introduced a failure on a 40 mm box with all edges rounded
r8, setbacks 12 and ease 4. Both all eight eased corners and the issue's four
set-back corners (only (40,40,0) eased) return a non-manifold body. Eases 0, 3 and
6 and the G2 blend at ease 4 are successful controls.

The first incorrect geometry change precedes stitching. Instrumenting the failing
Rust case shows `intersect/curve_class/p7.rs:1261`
(`set_do_filleting_geometry_with`, start-vertex hand-over) moving the patch's middle
vertex from (31.098349570550443,31.098349570550443,0) to (28,32,0). The classifier
has paired the middle with an end of the face curve. This leaves two incident
edges' curve endpoints 3.226878299263343 from their vertex. Subsequently
`topology/brep/p1.rs:2038` (`update_vertex_edge_intervals`) raises that vertex's
tolerance from 0.00004526973309926298 to 6.453756598526686. Only afterward does
`heal/stitch/p2.rs:579` glue (32,28,0) to (28,32,0), 5.656854249492381 apart.
The coincident-vertex merge immediately before the tolerance increase combines
two vertices at the same point; it is not the initial geometry error.

The pinned C++ has the same unconditional start-vertex hand-over
(`source/SMLib/src/SmCurveClass.cpp:17037`) and builds N-sided corner faces from
surface isocurves (`SmFilletCorner.cpp:3832-3843`). Those fitted boundaries are
not the eased face curve's own halves: the regression's sampled parameterwise
difference is about 1-2e-9. The pinned C++ has no ease extension, so this new path
cannot be compared directly with that reference.

For corners with `FilletCorner::ease_dist > 0`, `make_face_brep_with` now replaces
an isocurve matching a face-curve half's endpoints with that exact half, split at
the original curve's parameter midpoint. The half is oriented and reparameterized
to the isocurve. Interior boundaries and cross sections retain their isocurves.
This prevents the erroneous endpoint hand-over in the reproducer and preserves
the original quintics. It implements the face-curve-half approach proposed in
#1386; the fork's `4702889` itself was not available in the pinned sources.

This corrects the existing explicit opt-in `FilletSession::set_corner_ease`;
`ease_dist` defaults to zero, which keeps the pinned construction unchanged.
There is no new `UpstreamFixes` field and no change to the general classifier,
stitching, tolerances or C++ reference. Regression and controls are in
`crates/solidcore/tests/issue_1386_setback_ease_four.rs`.
