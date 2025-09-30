pub trait TapLog {
    /// Logs the value of `self` at the specified [`tracing::Level`] with a contextual message,
    /// in the format `{ctx}: {self}` or `{self}` if `ctx` is empty, then returns `self` unchanged.
    ///
    /// This method is useful for logging values in method chains or pipelines without disrupting
    /// flow. It emits a log message using the appropriate macro from the `tracing` crate,
    /// depending on the provided level.
    ///
    /// # Example
    /// ```rust
    /// use tracing::Level;
    /// use tap_log::TapLog;
    ///
    /// let number = 42.tap_log(Level::INFO, "my_val"); // output INFO: my_val: 42
    /// assert_eq!(number, 42);
    /// let number = 42.tap_log(Level::INFO, ""); // output INFO: 42
    /// assert_eq!(number, 42);
    /// ```
    fn tap_log(self, level: tracing::Level, ctx: &str) -> Self
    where
        Self: std::fmt::Debug + Sized,
    {
        let space = if ctx.is_empty() { "" } else { ": " };
        match level {
            tracing::Level::TRACE => tracing::trace!("{ctx}{space}{:?}", self),
            tracing::Level::DEBUG => tracing::debug!("{ctx}{space}{:?}", self),
            tracing::Level::INFO => tracing::info!("{ctx}{space}{:?}", self),
            tracing::Level::WARN => tracing::warn!("{ctx}{space}{:?}", self),
            tracing::Level::ERROR => tracing::error!("{ctx}{space}{:?}", self),
        }
        self
    }
}
impl<T> TapLog for T {}
