struct SearchTree<'a> {
    root: Node<'a>,

}

struct Node<'a> {
    is_terminal: bool,
    children: Vec<Node<'a>>,
    parent: Option<&'a Node<'a>>
}

fn expand_node(node: &mut Node){

}
//fn squash<'a>(first: Path<'a>, second: Path<'a>) -> (Path<'a>, u64) {
//    let mut result = Path{
//        from_event: first.from_event,
//        to_event: second.to_event,
//        matrix: DistanceMatrix
//    };
//
//}