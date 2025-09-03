# B+Tree

A B+Tree is a perfect balanced tree data structure used for indexing data in most
of databases today.


## Characteristics:
- If a leaf node is full, split it into half and update the parent, if the parent is full
do the split recursively. So B+ Tree only expands from the root direction.
- B+ Tree only stores values in the leaf nodes, inner nodes only guides the
search process
- B+ Tree has sibling links between its leaf nodes so that we don't have
to go back into the inner nodes for scanning data spanning multiple leaves


## Benchmark vs std::collections::BTreeMap
![comparison](./benches/result/Benchmark vs std collections/report/violin.svg)
<img src="./benches/result/Benchmark vs std collections/report/violin.svg">

For detailed report see ![Report](benches/result/report/index.html)
