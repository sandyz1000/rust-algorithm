//! An ideomatic way of mapping a [Result](std::result::Result) to a [Future](core::future::Future)
//! of [Result](std::result::Result).
//!
//! # Example usage
//!
//! ```
//! use future_result::FutureResult;
//!
//! async fn add_1(x: u32) -> u32 {
//!     x + 1
//! }
//!
//! fn main() {
//!     let fut = async {
//!         let ok: Result<u32, ()> = Ok(41);
//!         let ok = ok.then_map(add_1).await;
//!
//!         assert_eq!(Ok(42), ok);
//!
//!         // ...
//!
//!         let err: Result<(), u32> = Err(9);
//!         let err = err.then_map_err(add_1).await;
//!
//!         assert_eq!(Err(10), err);
//!     };
//!
//!     futures::executor::block_on(fut);
//! }
//! ```

use async_trait::async_trait;
use core::future::Future;

/// A [Result](std::result::Result), which can be transformed into a
/// [Future](core::future::Future) of [Result](std::result::Result)
#[async_trait(?Send)]
pub trait FutureResult {
    /// The success value of the `Result`
    type Ok;

    /// The error value of the `Result`
    type Err;

    /// Maps a `Result<T, E>` to a `Future<Output = Result<U, E>` by applying
    /// a function to the contained [`Ok`] value.
    /// [`Err`] is left untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// # use future_result::FutureResult;
    /// let ok: Result<u8, ()> = Ok(1);
    /// let err: Result<bool, ()> = Err(());
    ///
    /// futures::executor::block_on(async move {
    ///     let ok = ok.then_map(|x| async move { x + 6 }).await;
    ///     assert_eq!(Ok(7), ok);
    ///
    ///     let err = err.then_map(|b| async move { !b }).await;
    ///     assert_eq!(Err(()), err);
    /// });
    /// ```
    async fn then_map<U, F, Fut>(self, f: F) -> Result<U, Self::Err>
    where
        F: FnOnce(Self::Ok) -> Fut,
        Fut: Future<Output = U>;

    /// Maps a `Result<T, E>` to a `Future<Output = Result<T, U>` by applying
    /// a function to the contained [`Err`] value.
    /// [`Ok`] is left untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// # use future_result::FutureResult;
    /// let ok: Result<(), bool> = Ok(());
    /// let err: Result<(), u8> = Err(8);
    ///
    /// futures::executor::block_on(async move {
    ///     let err = err.then_map_err(|x| async move { x - 3 }).await;
    ///     assert_eq!(Err(5), err);
    ///
    ///     let ok = ok.then_map_err(|b| async move { !b }).await;
    ///     assert_eq!(Ok(()), ok);
    /// });
    /// ```
    async fn then_map_err<U, F, Fut>(self, f: F) -> Result<Self::Ok, U>
    where
        F: FnOnce(Self::Err) -> Fut,
        Fut: Future<Output = U>;
}

#[async_trait(?Send)]
impl<T: Sync, E: Sync> FutureResult for Result<T, E> {
    type Ok = T;
    type Err = E;

    async fn then_map<U, F, Fut>(self, f: F) -> Result<U, E>
    where
        Self: Sized,
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = U>,
    {
        match self {
            Ok(x) => Ok(f(x).await),
            Err(e) => Err(e),
        }
    }

    async fn then_map_err<U, F, Fut>(self, f: F) -> Result<T, U>
    where
        Self: Sized,
        F: FnOnce(E) -> Fut,
        Fut: Future<Output = U>,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(f(e).await),
        }
    }
}

