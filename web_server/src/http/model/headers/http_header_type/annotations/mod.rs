pub trait MutltiValued {
    fn is_multi_valued(&self) -> bool {
        return false;
    }
}

pub trait CaseInsensitiveValue {
    fn is_case_insensitive_value(&self) -> bool {
        return false;
    }
}

pub trait Commentable {
    fn is_commentable(&self) -> bool {
        return false;
    }
}