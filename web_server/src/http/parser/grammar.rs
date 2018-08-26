trait Token {
	fn is_token_char(self) -> bool;
}

trait WhiteSpace {
	fn is_whitespace_char(self) -> bool;
}

trait FieldComment {
    fn is_field_comment_start(self) -> bool;
    fn is_field_comment_end(self) -> bool;
}

trait Escape {
    fn is_escape_char(self) -> bool;
}

trait DQuote {
    fn is_dquote_char(self) -> bool;
}


impl Token for char {
	fn is_token_char(self) -> bool {
		match self {
			'!' 
			| '#' 
			| '$' 
			| '%' 
			| '&' 
			| '\'' 
			| '*'
			| '+'
			| '-' 
			| '.' 
			| '^'
			| '_'
			| '`'
			| '|'
			| '~'
			| '0'...'9'
			| 'A'...'Z'
			| 'a'...'z' => true,
			_ => false
		}
	}
}

impl WhiteSpace for char {
	fn is_whitespace_char(self) -> bool {
		match self {
			' ' | '\t' => true,
			_ => false
		}
	}
}


impl Escape for char {
    fn is_escape_char(self) -> bool {
        self == '\\'
    }
}

impl FieldComment for char {
    fn is_field_comment_start(self) -> bool {
        self == '(' 
    }

    fn is_field_comment_end(self) -> bool {
        self == ')'
    }
}

impl DQuote for char {
    fn is_dquote_char(self) -> bool {
        self == '"'
    }
}

macro_rules! extend_to_u8 {
	($t: ident, $m: ident) => { 
        impl $t for u8 {
            extend_to_u8!($m);
        }
    };
    ($m: ident) => {
        fn $m(self) -> bool {
            char::from(self).$m()
        }
    };
}
extend_to_u8!(Token, is_token_char);
extend_to_u8!(WhiteSpace, is_whitespace_char);
extend_to_u8!(Escape, is_escape_char);
extend_to_u8!(DQuote, is_dquote_char);

impl FieldComment for u8 {
    extend_to_u8!(is_field_comment_start);
    extend_to_u8!(is_field_comment_end);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expected_tokens_match() {
        let mut token_char = vec!('!','#', '$', '&', '\'', '+', '-' , '.', '^', '_', '`', '|', '~');
        
        let mut range: Vec<char> = (97..=122).map(|d| char::from(d)).collect(); //a-z
        token_char.append(&mut range);
        
        let mut range: Vec<char> = (65..=90).map(|d| char::from(d)).collect(); //A-Z
        token_char.append(&mut range);
        
        let mut range: Vec<char> = (48..=57).map(|d| char::from(d)).collect(); //0-9
        token_char.append(&mut range);

        for c in token_char  {
            assert!(c.is_token_char())
        }
    }
    #[test]
    fn whitespace_is_not_token() {
        for c in vec!(' ','\t') {
            assert!(!c.is_token_char())
        }
    }
    #[test]
    fn comment_is_not_token() {
        for c in vec!('(',')') {
            assert!(!c.is_token_char())
        }
    }
    #[test]
    fn dquote_is_not_token() {
        for c in vec!('"') {
            assert!(!c.is_token_char())
        }
    }
    #[test]
    fn escape_is_not_token() {
        for c in vec!('\\') {
            assert!(!c.is_token_char())
        }
    }



    #[test]
    fn dquote_matches() {
        for c in vec!('"') {
            assert!(c.is_dquote_char())
        }
    }
    #[test]
    fn dquote_u8_matches() {
        for c in vec!(34) {
            assert!(c.is_dquote_char())
        }
    }



    #[test]
    fn left_comment_matches() {
        for c in vec!('(') {
            assert!(c.is_field_comment_start())
        }
    }
    #[test]
    fn left_comment_u8_matches() {
        for c in vec!(40) {
            assert!(c.is_field_comment_start())
        }
    }
    #[test]
    fn right_comment_matches() {
        for c in vec!(')') {
            assert!(c.is_field_comment_end())
        }
    }
    #[test]
    fn right_comment_u8_matches() {
        for c in vec!(41) {
            assert!(c.is_field_comment_end())
        }
    }



    #[test]
    fn escape_matches() {
        for c in vec!('\\') {
            assert!(c.is_escape_char())
        }
    }
    #[test]
    fn escape_u8_matches() {
        for c in vec!(92) {
            assert!(c.is_escape_char())
        }
    }



    #[test]
    fn whitespace_matches() {
        for c in vec!(' ','\t') {
            assert!(c.is_whitespace_char())
        }
    }
    #[test]
    fn whitespace_u8_matches() {
        for c in vec!(9,32) {
            assert!(c.is_whitespace_char())
        }
    }

}