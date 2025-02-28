enum SnapPoint {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    CenterLeft,
    CenterRight,
    CenterTop,
    CenterBottom,
}

trait Snappable {
    fn snap_to(&mut self, target: dyn Snappable, snap_point: SnapPoint);
}
