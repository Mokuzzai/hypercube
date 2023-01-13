# Hypercube

## Todo
### create `OffsetShape<S: Shape, const D: usize>`
a `Shape` offset by some amount

#### Api
* `positions(self) -> OffsetPositions<Self>`

### create `OffsetPositions<S: Shape, D>`
* impl `Iterator<Item = Vector<i32, D>>`

### create `Aabb<D>`
* `positions(&self) -> OffsetPositions<..?>`

### `World`
* add methods `min[_|position|chunk]`, `max[_|position|chunk]`, `bouding_box(&self) -> Aabb` 
