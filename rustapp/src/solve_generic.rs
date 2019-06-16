fn insert_after(old_path: &mut Path, new_segment: Path) {}


struct Path<'a> {
    from_event: &'a Event,
    to_event: &'a Event,
    matrix: DistanceMatrix,
//    middle_point:
}

struct PathList<'a> {
    path: Path<'a>,
    next_node: Box<PathList<'a>>,
}



//fn squash<'a>(first: Path<'a>, second: Path<'a>) -> (Path<'a>, u64) {
//    let mut result = Path{
//        from_event: first.from_event,
//        to_event: second.to_event,
//        matrix: DistanceMatrix
//    };
//
//}