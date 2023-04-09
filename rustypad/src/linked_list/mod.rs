mod doubly_linked_list;

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: ListLink
}

#[macro_export]
macro_rules! map {
    // let greet = map!("hello" => "world", "howdy" => "whatsupp");
    ($($key:expr => $value:expr),*) => {
        let mut hm = HashMap::new();
        $(
            hm.insert($key, $value);
        )*
        hm
    };
}

#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($e:expr) => {
        ListLink::link($e, None)
    };
    ($e:expr, $($tail:tt)*) => {
        ListLink::link($e, list!($($tail)*))
    };
}

pub type ListLink = Option<Box<ListNode>>;

pub trait ListMaker {
    /// .
    fn link(val: i32, next: ListLink) -> ListLink {
        
        Some(Box::new(ListNode { val, next }))
    }
}

impl ListMaker for ListLink {}