use rustracing::tag::Tag;

pub fn debug_tag<N, D>(key: N, val: D) -> Tag
where
    N: Into<std::borrow::Cow<'static, str>>,
    D: std::fmt::Debug,
{
    Tag::new(key, format!("{:?}", val))
}
