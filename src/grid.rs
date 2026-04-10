use rayon::prelude::*;

#[derive(Debug)]
pub struct Grid<T>  {
	pub width: u16,
	pub height: u16,
	pub data: Box<[T]>,
}

impl<T> Grid<T>{
	pub fn new<F>(width: u16, height: u16, generator: F) -> Self
	where
		F: Fn(u16, u16) -> T,
	{
		// iterate over all (x, y) pairs
		let data = (0..height)
			.flat_map(|y| (0..width).map(move |x| (x, y)))
			.map(|(x, y)| generator(x, y))
			.collect::<Vec<T>>()
			.into_boxed_slice();

		Self {
			width,
			height,
			data
		}
	}


	pub fn get(&self, x: u16, y: u16) -> Option<&T> {
		if x >= self.width {
			return None;
		}

		if y >= self.height {
			return None;
		}

		let idx = (y as usize) * (self.width as usize) + (x as usize);
		self.data.get(idx)
	}

	pub fn map<F, B>(self, f: F) -> Grid<B>
	where
		F: Fn(T) -> B,
	{
		let Grid { width, height, data } = self;

		let mapped_data = data
			.into_iter()
			.map(|item| f(item))
			.collect::<Vec<B>>()
			.into_boxed_slice();

		Grid { width, height, data: mapped_data }
	}

	pub fn par_map<F, B>(self, f: F) -> Grid<B>
	where
		F: Fn(T) -> B + std::marker::Sync,
		B: Send,
		Box<[T]>: IntoParallelIterator<Item = T>,
	{
		let Grid { width, height, data } = self;

		let mapped_data = data
			.into_par_iter()
			.map(|item| f(item))
			.collect::<Vec<B>>()
			.into_boxed_slice();

		Grid { width, height, data: mapped_data }
	}
}

impl<T> Grid<Option<T>> {
	pub fn map_some<F, B>(self, f: F) -> Grid<Option<B>>
	where
		F: Fn(T) -> B,
	{
		self.map(|value_opt| value_opt.map(&f))
	}
}
