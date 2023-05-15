
use hypercube;
use hypercube::Shape as _;

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
criterion_group!(benches, hypercube_positions, for_each_positions, hypercube_uniform_positions, for_each_uniform_positions);

fn hypercube_positions(c: &mut Criterion) {
	let shape = hypercube::prelude3::rt::Multiform::new(nalgebra::Vector3::new(8, 16, 32));
	let shape = black_box(shape);

	c.bench_function(function_name!(), |b| b.iter(|| {
		for position in shape.positions() {
			black_box(position);
		}
	}));
}

fn for_each_positions(c: &mut Criterion) {
	c.bench_function(function_name!(), |b| b.iter(|| {
		for z in 0..32 {
			for y in 0..16 {
				for x in 0..8 {
					black_box([x, y, z]);
				}
			}
		}
	}));
}

fn hypercube_uniform_positions(c: &mut Criterion) {
	let shape = hypercube::prelude3::rt::Multiform::new(nalgebra::Vector3::new(32, 32, 32));
	let shape = black_box(shape);

	c.bench_function(function_name!(), |b| b.iter(|| {
		for position in shape.positions() {
			black_box(position);
		}
	}));
}

fn for_each_uniform_positions(c: &mut Criterion) {
	c.bench_function(function_name!(), |b| b.iter(|| {
		for z in 0..32 {
			for y in 0..32 {
				for x in 0..32 {
					black_box([x, y, z]);
				}
			}
		}
	}));
}

