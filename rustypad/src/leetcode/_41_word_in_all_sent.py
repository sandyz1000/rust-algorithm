"""
Count of words that are present in all the given sentences

Given n sentences. The task is to count the number of words that appear in all of 
these sentences. 
Note that every word consists of only lowercase English alphabets.
"""
from typing import List
from collections import defaultdict


def get_top_words(documents: List[str], k: int):
    # - create a dictionary of word as key and documents index as value
    # - iterate each word and check if documents >= k then add to result
    bow = defaultdict(list)
    for doc_id, doc in enumerate(documents):
        for token in doc.split():
            bow[token].append(doc_id)
    results = [word for word, doc_ids in bow.items() if len(doc_ids) >= k]
    return results


def __get_top_words(docs: List[str], k: int) -> List[str]:
    """
    :param _type_ docs: _description_
    :param _type_ k: _description_
    :return _type_: _description_
    """
    from itertools import chain
    from collections import Counter
    token = chain(*[doc.split() for doc in docs])
    most_common = Counter(token).most_common(k)
    return [token for token, _ in most_common]


def main():
    arr = ["there is a cow",
           "cow is our mother",
           "cow gives us milk and milk is sweet",
           "there is a boy who loves cow"]
    K = len(arr)
    print(__get_top_words(arr, K))


if __name__ == "__main__":
    main()
