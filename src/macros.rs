#![macro_use]

macro_rules! map_to_error {
    ($derived: ty, $from_err: ty, $to_err: expr) => {
        impl From<$from_err> for $derived {
            fn from(err: $from_err) -> $derived {
                $to_err(err)
            }
        }
    }
}
