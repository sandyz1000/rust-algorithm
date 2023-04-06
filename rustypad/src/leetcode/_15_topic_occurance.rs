/* 

def topic_occurence2(topics: Dict[str, List[str]], rev: str) -> Dict[str, int]:
    # Internet solution
    rev = rev.split(" ")
    dt = {}
    count = 0
    for k, v in topics.items():
        if k == "Price" or k == "Business" or k == "Harry":
            for i in range(len(v)):
                for j in range(len(rev)):
                    if v[i] == rev[j]:
                        count += 1
            dt[k] = count
            count = 0
    return dt


def main():
    
    
    x = topic_occurence(topics, rev)
    print(x)


 */
use std::collections::HashMap;

#[derive(Debug)]
struct Solution;

impl Solution {
    fn topic_occurence(_topics: HashMap<&str, Vec<&str>>, _rev: &str) -> HashMap<&'static str, i32> {
        !todo!()
    }
    
}

/// .
#[test]
fn test() {
    let mut topics: HashMap<&str, Vec<&str>> = HashMap::new();
    topics.insert("Price", vec!["cheap", "expensive", "price"]);
    topics.insert("Business", vec!["gnome", "gnomes"]);
    topics.insert("Harry", vec!["harry"]);

    let rev = "Harry shurb did Harry cheap price price harry of the gnome gnomes";
    let ans = Solution::topic_occurence(topics, rev);
}