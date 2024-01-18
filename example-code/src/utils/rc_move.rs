// FILE: utils/rc_move.rs
// ___________________________________________________________

use std::fmt;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

// Import the fmt module for the Debug trait

/// `ArcMove` is a custom smart pointer providing mutable and read-only access
/// to a shared value in a thread-safe manner. It combines `Arc` (Atomic Reference Counted)
/// for safe shared ownership across threads and `RwLock` for thread-safe read-write access.
/// This structure is beneficial in multi-threaded environments where shared state
/// needs to be read or modified safely by multiple owners.
///
/// # Examples
///
/// ```
/// let data = ArcMove::new(5);
///
/// // Try to access the data in a read-only manner
/// match data.read_state() {
///     Ok(value) => println!("Read-only access to data: {:?}", value),
///     Err(e) => eprintln!("Failed to access data: {:?}", e),
/// }
///
/// // Modify the data with error handling
/// match data.update_state() {
///     Ok(mut value) => *value = 10,
///     Err(e) => eprintln!("Failed to modify data: {:?}", e),
/// }
///
/// // Access the data in a read-only manner with error handling
/// let value = data.read_state().unwrap_or_else(|e| {
///     eprintln!("Failed to access data: {:?}", e);
///     0 // example default value
/// });
///
/// println!("Value after modification: {}", value);
/// ```
#[allow(dead_code)]
pub struct ArcMove<ObjType>(Arc<RwLock<ObjType>>);
// _______________________________________________

// Type alias for the result of read_state,
// representing a read lock guard and potential errors.
type ReadStateResult<'a, ObjType> = Result<
  RwLockReadGuard<'a, ObjType>,
  PoisonError<RwLockReadGuard<'a, ObjType>>
>;

// Type alias for the result of update_state,
// representing a write lock guard and potential errors.
type UpdateStateResult<'a, ObjType> = Result<
  RwLockWriteGuard<'a, ObjType>,
  PoisonError<RwLockWriteGuard<'a, ObjType>>
>;
// _______________________________________________

impl<ObjType> ArcMove<ObjType> {
  pub fn new(value: ObjType) -> ArcMove<ObjType> {
    ArcMove(Arc::new(RwLock::new(value)))
  }

  /// `read_state`
  /// Acquires a read lock on the wrapped data, returning a read lock guard.
  /// This method attempts to acquire a read lock, allowing multiple readers concurrently.
  /// It returns a `ReadStateResult` which is a `Result` wrapping a read lock guard,
  /// or an error if the lock is poisoned.
  pub fn read_state(&self) -> ReadStateResult<ObjType> {
    self.0.read()
  }

  /// `update_state`
  /// Acquires a write lock on the wrapped data, returning a write lock guard.
  /// This method attempts to acquire a write lock, which is exclusive, meaning no other
  /// readers or writers can access the data until the write lock is released.
  /// It returns an `UpdateStateResult` which is a `Result` wrapping a write lock guard,
  /// or an error if the lock is poisoned.
  pub fn update_state(&self) -> UpdateStateResult<ObjType> {
    self.0.write()
  }
}
// _______________________________________________

// Debug implementation for ArcMove
impl<ObjType> fmt::Debug for ArcMove<ObjType> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ArcMove<ObjType>")
  }
}

// ___________________________________________________________
