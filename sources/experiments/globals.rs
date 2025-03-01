

use ::vrl_preludes::std_plus_extras::*;

use ::vrl_errors::*;








define_error! (pub GlobalError, result : GlobalResult);








pub struct Global <Object, const NAMESPACE : u128 = 0>
	where
		Object : Send + Sync + 'static,
{
	cell : GlobalCell<Object>,
}


impl <Object, const NAMESPACE : u128> Global <Object, NAMESPACE>
	where
		Object : Send + Sync + 'static,
{
	pub const fn new () -> Self {
		Self {
			cell : GlobalCell::new (NAMESPACE),
		}
	}
	
	pub fn as_ref (&self) -> &Object {
		self.try_ref () .else_panic (0x7db115f8)
	}
	
	pub fn try_ref (&self) -> GlobalResult<&Object> {
		let _cell = self.cell.resolve ();
		_cell.get () .else_wrap (0xa781e1f2)
	}
	
	pub fn initialize (&self, _object : Object) -> GlobalResult {
		let _cell = self.cell.resolve ();
		_cell.set (_object) .else_replace (0xf922cb8d)
	}
}








pub struct GlobalRoLock <Object, const NAMESPACE : u128 = 0>
	where
		Object : Send + 'static,
{
	cell : GlobalCell<Mutex<Object>>,
}


pub struct GlobalRoLockGuard <Object>
	where
		Object : Send + 'static,
{
	guard : MutexGuard<'static, Object>,
}


impl <Object, const NAMESPACE : u128> GlobalRoLock <Object, NAMESPACE>
	where
		Object : Send + 'static,
{
	pub const fn new () -> Self {
		Self {
			cell : GlobalCell::new (NAMESPACE),
		}
	}
	
	pub fn as_ref (&self) -> GlobalRoLockGuard<Object> {
		self.try_ref (true) .else_panic (0x7db115f8)
	}
	
	pub fn try_ref (&self, _wait : bool) -> GlobalResult<GlobalRoLockGuard<Object>> {
		let _cell = self.cell.resolve ();
		let _mutex = _cell.get () .else_wrap (0xa781e1f2) ?;
		let _guard = if _wait {
			match _mutex.lock () {
				Ok (_guard) =>
					_guard,
				Err (_) =>
					fail! (0x064c7ae6),
			}
		} else {
			match _mutex.try_lock () {
				Ok (_guard) =>
					_guard,
				Err (sync::TryLockError::WouldBlock) =>
					fail! (0xfd616453),
				Err (_) =>
					fail! (0xa00e4bab),
			}
		};
		let _guard = GlobalRoLockGuard {
				guard : _guard,
			};
		Ok (_guard)
	}
	
	pub fn initialize (&self, _object : Object) -> GlobalResult {
		let _cell = self.cell.resolve ();
		_cell.set (Mutex::new (_object)) .else_replace (0xf922cb8d)
	}
}


impl <Object> Deref for GlobalRoLockGuard <Object>
	where
		Object : Send + 'static,
{
	type Target = Object;
	
	fn deref (&self) -> &Object {
		Deref::deref (&self.guard)
	}
}








pub struct GlobalCell <Global>
	where
		Global : Send + Sync + 'static,
{
	pub namespace : u128,
	cell : OnceLock<&'static OnceLock<Global>>,
}


impl <Global> GlobalCell <Global>
	where
		Global : Send + Sync + 'static,
{
	pub const fn new (_namespace : u128) -> Self {
		Self {
				namespace : _namespace,
				cell : OnceLock::new (),
			}
	}
	
	pub fn resolve (&self) -> &'static OnceLock<Global> {
		
		fn _cell_box_new <Global> () -> Box<dyn Any + Send + Sync>
			where
				Global : Send + Sync + 'static,
		{
			let _cell = OnceLock::<Global>::new ();
			let _cell_box = Box::new (_cell);
			let _cell_ref = Box::leak (_cell_box);
			let _cell_ref = _cell_ref as &'static OnceLock<Global>;
			let _cell_ref_box = Box::new (_cell_ref);
			_cell_ref_box
		}
		
		fn _cell_box_ref <Global> (_namespace : u128) -> &'static OnceLock<Global>
			where
				Global : Send + Sync + 'static,
		{
			let _cells = if let Some (_cells) = GLOBAL_CELLS.get () {
				_cells
			} else {
				GLOBAL_CELLS.set (Mutex::new (HashMap::new ())) .infallible_unexpected (0x20224914);
				GLOBAL_CELLS.get () .infallible (0x2784a811)
			};
			let mut _cells = _cells.try_lock () .infallible_unexpected (0x8f8da52c);
			let _cells = _cells.deref_mut ();
			let _cell_key = (_namespace, TypeId::of::<Global> ());
			let _cell_box = _cells.entry (_cell_key) .or_insert_with (_cell_box_new::<Global>);
			let _cell_box_ref = _cell_box.downcast_ref::<&'static OnceLock<Global>> () .infallible (0x6e3d371b);
			_cell_box_ref
		}
		
		if let Some (_cell_ref) = self.cell.get () {
			_cell_ref
		} else {
			let _cell_ref = _cell_box_ref (self.namespace);
			self.cell.set (_cell_ref) .infallible_unexpected (0x886d5457);
			_cell_ref
		}
	}
}


static GLOBAL_CELLS : OnceLock<Mutex<HashMap<(u128, TypeId), Box<dyn Any + Send + Sync>>>> = OnceLock::new ();


