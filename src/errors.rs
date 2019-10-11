macro_rules! implement_error_traits {
    ($error:ident, $kind:ident) => {
        impl Fail for $error {
            fn cause(&self) -> Option<&dyn Fail> {
                self.inner.cause()
            }

            fn backtrace(&self) -> Option<&Backtrace> {
                self.inner.backtrace()
            }
        }

        impl Display for $error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                Display::fmt(&self.inner, f)
            }
        }
        impl $error {
            pub fn kind(&self) -> $kind {
                *self.inner.get_context()
            }
        }

        impl From<$kind> for $error {
            fn from(kind: $kind) -> $error {
                $error { inner: Context::new(kind) }
            }
        }

        impl From<Context<$kind>> for $error {
            fn from(inner: Context<$kind>) -> $error {
                $error { inner: inner }
            }
        }
    }
}
