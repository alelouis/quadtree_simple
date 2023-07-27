# Simple Quadtree

Quadtree rust implementation as a exercise.

**Performance details:**
- Nodes are stored in a 1D vector, with children referenced as indices in this vector.
- In-place partitioning is used in order to rearrange input points according to leafs (better cache hit on iteration).

The member `.indices` can be used to fetch ranges of points of all end leafs of the tree. 