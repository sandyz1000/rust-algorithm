//

#[derive(Debug)]
#[allow(dead_code)]
struct ListNode {
    val: i32,
    next: ListLink
}


type ListLink = Option<Box<ListNode>>;

#[macro_export]
macro_rules! llist {
    () => {
        None
    };
    ($t:expr) => {
        ListLink::new($t, None)
    };
    ($t:expr, $($tail:tt)*) => {
        ListLink::new($t, llist!(
            $( $tail )* 
        ))
    }
}

trait ListMaker {
    fn new(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode {val, next}))
    }
}

impl ListMaker for ListLink {}
