from typing import Dict, List
from collections import defaultdict


def topic_occurence(topics: Dict[str, List[str]], rev: str) -> Dict[str, int]:
    # 1. Create a keyword_topic reverse map
    # 2. Create a topic_counter dictionary
    # 3. If current_str exist in key keyword_topic
    # 4. Increase the counter in topic_counter for this topic
    keyword_topic: Dict = {key: topic for topic, keywords in topics.items()
                           for key in keywords}
    # keyword_topic: Dict = {key: topic for topic, keywords in topics.items()
    #                        for key in zip(keywords[:-1], keywords[1:])}
    topic_counter: Dict = defaultdict(int)
    rev = rev.split()
    # for 1 gram
    for current in rev:
        if current.lower() in keyword_topic:
            topic = keyword_topic[current.lower()]
            topic_counter[topic] += 1
    # for 2 gram
    for current in zip(rev[:-1], rev[1:]):
        if current.lower() in keyword_topic:
            topic = keyword_topic[current.lower()]
            topic_counter[topic] += 1
    return topic_counter


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
    topics = {"Price": ["cheap", "expensive", "price"],
              "Business": ["gnome", "gnomes"],
              "Harry": ["harry"]}
    rev = "Harry shurb did Harry cheap price price harry of the gnome gnomes"
    x = topic_occurence(topics, rev)
    print(x)


if __name__ == "__main__":
    main()
