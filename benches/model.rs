use hypercube;

use criterion::*;

macro_rules! function_name {
	() => {{
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str {
			std::any::type_name::<T>()
		}
		let name = type_name_of(f);
		&name[..name.len() - 3]
	}};
}

criterion_main!(benches);
criterion_group!(benches, model_positions, model_for_each);

fn model_positions(c: &mut Criterion) {
	let mut model = hypercube::model::Model3::<hypercube::model::AaPlane3, ()>::default();
	let chunk = black_box(hypercube::prelude3::Chunk::<Box<[bool]>, _>::from_shape_index(hypercube::prelude3::ct::Uniform::<32>::new(), |index| index & 1 != 0));

	c.bench_function(function_name!(), |b| b.iter(|| {
		model.clear();

		for (position, &block) in chunk.block_positions() {
			if block {
				model.push_cube(position, ());
			}
		}

		black_box(&model);
	}));
}

fn model_for_each(c: &mut Criterion) {
	let mut model = hypercube::model::Model3::<hypercube::model::AaPlane3, ()>::default();
	let chunk = black_box(hypercube::prelude3::Chunk::<Box<[bool]>, _>::from_shape_index(hypercube::prelude3::ct::Uniform::<32>::new(), |index| index & 1 != 0));

	c.bench_function(function_name!(), |b| b.iter(|| {
		model.clear();

		let mut i = 0..(32 * 32 * 32);

		for z in 0..32 {
			for y in 0..32 {
				for x in 0..32 {
					let index = i.next().unwrap();
					let &block = chunk.storage.get(index).unwrap();

					if block {
						model.push_cube(nalgebra::Point3::new(x, y, z).cast(), ());
					}
				}
			}
		}

		black_box(&model);
	}));
}

