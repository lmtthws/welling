enum SupportedAuthMethods {
    mysql_old_password,
    mysql_native_password,
}

impl Display for SupportedAuthMethods {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            ref mysql_old_password => write!(f,"mysql_old_password"),
            ref my_sql_native_auth => write!(f,"mysql_native_password")
        }
    }
}

impl SupportAuthMethods {
    fn 
}


fn native_password_hash(password: &str) -> [u8;5]