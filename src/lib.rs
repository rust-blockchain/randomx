pub const HASH_SIZE: usize = sys::RANDOMX_HASH_SIZE as usize;

pub struct FullVM {
	cache_ptr: *mut sys::randomx_cache,
	dataset_ptr: *mut sys::randomx_dataset,
	ptr: *mut sys::randomx_vm,
}

unsafe impl Send for FullVM { }
unsafe impl Sync for FullVM { }

impl FullVM {
	pub fn new(key: &[u8]) -> Self {
		let flags = sys::randomx_flags_RANDOMX_FLAG_DEFAULT
			| sys::randomx_flags_RANDOMX_FLAG_JIT
			| sys::randomx_flags_RANDOMX_FLAG_FULL_MEM;

		let cache_ptr = unsafe {
			let ptr = sys::randomx_alloc_cache(flags);
			sys::randomx_init_cache(
				ptr,
				key.as_ptr() as *const std::ffi::c_void,
				key.len()
			);

			ptr
		};

		let dataset_ptr = unsafe {
			let ptr = sys::randomx_alloc_dataset(flags);
			let count = sys::randomx_dataset_item_count();
			sys::randomx_init_dataset(ptr, cache_ptr, 0, count);
			ptr
		};

		let ptr = unsafe {
			sys::randomx_create_vm(
				flags,
				cache_ptr,
				dataset_ptr
			)
		};

		Self { cache_ptr, dataset_ptr, ptr }
	}

	pub fn calculate(&self, input: &[u8]) -> [u8; HASH_SIZE] {
		let ret = [0u8; HASH_SIZE];

		unsafe {
			sys::randomx_calculate_hash(
				self.ptr,
				input.as_ptr() as *const std::ffi::c_void,
				input.len(),
				ret.as_ptr() as *mut std::ffi::c_void,
			);
		}

		ret
	}
}

impl Drop for FullVM {
	fn drop(&mut self) {
		unsafe {
			sys::randomx_release_cache(self.cache_ptr);
			sys::randomx_release_dataset(self.dataset_ptr);
			sys::randomx_destroy_vm(self.ptr);
		}
	}
}

pub struct VM {
	cache_ptr: *mut sys::randomx_cache,
	ptr: *mut sys::randomx_vm,
}

unsafe impl Send for VM { }
unsafe impl Sync for VM { }

impl VM {
	pub fn new(key: &[u8]) -> Self {
		let flags = sys::randomx_flags_RANDOMX_FLAG_DEFAULT
			| sys::randomx_flags_RANDOMX_FLAG_JIT;

		let cache_ptr = unsafe {
			let ptr = sys::randomx_alloc_cache(flags);
			sys::randomx_init_cache(
				ptr,
				key.as_ptr() as *const std::ffi::c_void,
				key.len()
			);

			ptr
		};

		let ptr = unsafe {
			sys::randomx_create_vm(
				flags,
				cache_ptr,
				std::ptr::null_mut()
			)
		};

		Self { cache_ptr, ptr }
	}

	pub fn calculate(&self, input: &[u8]) -> [u8; HASH_SIZE] {
		let ret = [0u8; HASH_SIZE];

		unsafe {
			sys::randomx_calculate_hash(
				self.ptr,
				input.as_ptr() as *const std::ffi::c_void,
				input.len(),
				ret.as_ptr() as *mut std::ffi::c_void,
			);
		}

		ret
	}
}

impl Drop for VM {
	fn drop(&mut self) {
		unsafe {
			sys::randomx_release_cache(self.cache_ptr);
			sys::randomx_destroy_vm(self.ptr);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_create_vm() {
		let mut vm = VM::new(&b"RandomX example key"[..]);
		let hash = vm.calculate(&b"RandomX example input"[..]);
		assert_eq!(hash, [210, 164, 216, 149, 3, 68, 116, 1, 239, 110, 111, 48, 180, 102, 53, 180, 91, 84, 242, 90, 101, 12, 71, 70, 75, 83, 17, 249, 214, 253, 71, 89]);
	}

	#[test]
	fn should_work_with_full_vm() {
		let mut vm = VM::new(&b"RandomX example key"[..]);
		let hash = vm.calculate(&b"RandomX example input"[..]);
		let mut full_vm = FullVM::new(&b"RandomX example key"[..]);
		let full_hash = full_vm.calculate(&b"RandomX example input"[..]);
		assert_eq!(hash, full_hash);
	}
}
