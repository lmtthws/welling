pub(super) trait VisibleChar {
    fn is_visible_char(self) -> bool;
}

pub(super) trait ObsText {
    fn is_obs_text_char(self) -> bool;
}

pub(super) trait Token {
	fn is_token_char(self) -> bool;
}

pub(super) trait WhiteSpace {
	fn is_whitespace_char(self) -> bool;
}

pub(super) trait FieldComment {
    fn is_field_comment_start(self) -> bool;
    fn is_field_comment_end(self) -> bool;
}

pub(super) trait Escape {
    fn is_escape_char(self) -> bool;
}

pub(super) trait DQuote {
    fn is_dquote_char(self) -> bool;
}

pub(super) trait QuotedText {
    fn is_quoted_text(self) -> bool;
}

pub(super) trait CommentText {
    fn is_comment_text(self) -> bool;
}

pub(super) trait HeaderFieldDelim {
    fn is_header_field_delim(self) -> bool;
}


impl VisibleChar for char {
    fn is_visible_char(self) -> bool {
        match self {
            '!'...'~' => true,
            _ => false
        }
    }
}

impl ObsText for char {
    fn is_obs_text_char(self) -> bool {
        (self as u8).is_obs_text_char()
    }
}

impl ObsText for u8 {
    fn is_obs_text_char(self) -> bool {
        match self {
            0x80...0xFF => true,
            _ => false
        }
    }
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
    #[inline]
	fn is_whitespace_char(self) -> bool {
		self == ' ' || self == '\t'
	}
}


impl Escape for char {
    #[inline]
    fn is_escape_char(self) -> bool {
        self == '\\'
    }
}


impl FieldComment for char {
    #[inline]
    fn is_field_comment_start(self) -> bool {
        self == '('
    }

    #[inline]
    fn is_field_comment_end(self) -> bool {
        self == ')'
    }
}


impl DQuote for char {
    #[inline]
    fn is_dquote_char(self) -> bool {
        self == '"'
    }
}

impl HeaderFieldDelim for char {
    fn is_header_field_delim(self) -> bool {
        self == ':'
    }
}

impl QuotedText for char {
    fn is_quoted_text(self) -> bool {
        if self.is_dquote_char() {
            false
        } else if self.is_escape_char() {
            false
        } else if self.is_whitespace_char() {
            true
        } else if self.is_visible_char() {
            true
        } else if self.is_obs_text_char() {
            true
        } else {
            false
        }
    }
}

impl CommentText for char {
    fn is_comment_text(self) -> bool {
        if self.is_whitespace_char() {
            true
        } else if self.is_field_comment_start() {
            false
        } else if self.is_field_comment_end() {
            false
        } else if self.is_escape_char() {
            false
        } else if self.is_visible_char() {
            true
        } else if self.is_obs_text_char() {
            true
        } else {
            false
        }
    }
}

macro_rules! extend_to_u8 {
	($t: ident, $m: ident) => { 
        impl $t for u8 {
            extend_to_u8!($m);
        }
    };
    ($m: ident) => {
        #[inline]
        fn $m(self) -> bool {
            char::from(self).$m()
        }
    };
}

extend_to_u8!(VisibleChar, is_visible_char);
extend_to_u8!(Token, is_token_char);
extend_to_u8!(WhiteSpace, is_whitespace_char);
extend_to_u8!(Escape, is_escape_char);
extend_to_u8!(DQuote, is_dquote_char);
extend_to_u8!(HeaderFieldDelim, is_header_field_delim);
extend_to_u8!(QuotedText, is_quoted_text);
extend_to_u8!(CommentText, is_comment_text);

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



    #[test]
    fn header_delim_matches() {
        for c in vec!(':') {
            assert!(c.is_header_field_delim())
        }
    }
    #[test]
    fn header_delim_u8_matches() {
        for c in vec!(58) {
            assert!(c.is_header_field_delim())
        }
    }


    #[test]
    fn obs_text_matches() {
        let mut range: Vec<char> = (0x80..=0xFF).map(|d| char::from(d)).collect(); //a-z
        for c in range {
            assert!(c.is_obs_text_char())
        }

        let mut range: Vec<char> = (0x00..=0x79).map(|d| char::from(d)).collect();
        for c in range {
            assert!(!c.is_obs_text_char())
        }
    }

    #[test]
    fn obs_text_u8_matches() {
        let mut range: Vec<u8> = (0x80..=0xFF).collect(); //a-z
        for c in range {
            assert!(c.is_obs_text_char())
        }

        let mut range: Vec<u8> = (0x00..=0x79).collect();
        for c in range {
            assert!(!c.is_obs_text_char())
        }
    }

    #[test]
    fn visible_text_matches() {
        let mut range: Vec<char> = (0x21..=0x7E).map(|u| char::from(u)).collect(); //a-z
        for c in range {
            assert!(c.is_visible_char())
        }

        let mut range: Vec<char> = (0x00u8..=0x20).map(|u| char::from(u)).collect();
        range.append(&mut (0x7F..=0xFF).map(|u| char::from(u)).collect::<Vec<char>>());
        for c in range {
            assert!(!c.is_visible_char())
        }
    }

    #[test]
    fn visible_text_u8_matches() {
        let mut range: Vec<u8> = (0x21..=0x7E).collect(); //a-z
        for c in range {
            assert!(c.is_visible_char())
        }

        let mut range: Vec<u8> = (0x00u8..=0x20).collect();
        range.append(&mut (0x7F..=0xFF).collect::<Vec<u8>>());
        for c in range {
            assert!(!c.is_visible_char())
        }
    }


    #[test]
    fn quoted_text_has_exceptions() {
        for c in vec!('"', '\\') {
            assert!(!c.is_quoted_text());
        }

        for c in vec!(' ','\t','(',')') {
            assert!(c.is_quoted_text())
        }
    }

    #[test]
    fn quoted_text_u8_has_exceptions() {
        for c in vec!(0x22, 0x5C) {
            assert!(!c.is_quoted_text());
        }

        for c in vec!(0x09, 0x20, 0x28, 0x29) {
            assert!(c.is_quoted_text())
        }
    }

    #[test]
    fn comment_text_has_exceptions() {
         for c in vec!('(',')','\\') {
            assert!(!c.is_comment_text());
        }

        for c in vec!('\t',' ','"') {
            assert!(c.is_comment_text())
        }
    }

    #[test]
    fn comment_text_u8_has_exceptions() {
         for c in vec!(0x28, 0x29, 0x5C) {
            assert!(!c.is_comment_text());
        }

        for c in vec!(0x09, 0x20, 0x22) {
            assert!(c.is_comment_text())
        }
    }

}