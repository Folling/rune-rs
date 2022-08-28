pub enum RangeOpenness {
    Closed,
    HalfOpenLower,
    HalfOpenUpper,
    Open,
}

pub struct Range<T> {
    openness: RangeOpenness,
    from: T,
    to: T,
}
