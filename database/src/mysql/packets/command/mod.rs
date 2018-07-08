pub mod column;
pub mod text;
pub mod row;


pub enum SupportedCommands {
    //COM_SLEEP,
    COM_QUIT,
    COM_INIT_DB,
    COM_QUERY,
    //COM_FIELD_LIST,
    //COM_CREATE_DB,
    //COM_DROP_DB,
    //COM_REFRESH,
    //COM_DEPRECATED_1,
    COM_STATISTICS,
    //COM_PROCESS_INFO,
    //COM_CONNECT,
    //COM_PROCESS_KILL,
    COM_DEBUG,
    COM_PING,
    //COM_TIME,
    //COM_CHANGE_USER,
    //COM_DELAYED_INSERT,
    COM_CHANGE_USER,
    //COM_BINLOG_DUMP,
    //COM_TABLE_DUMP,
    //COM_CONNECT_OUT,
    //COM_REGISTER_SLAVE,
    COM_STMT_PREPARE,
    COM_STMT_EXECUTE,
    COM_STMT_SEND_LONG_DATA,
    COM_STMT_CLOSE,
    COM_STMT_RESET,
    COM_SET_OPTION,
    COM_STMT_FETCH,
    //COM_DAEMON,
    //COM_BINLOG_DUMP_GTID,
    COM_RESET_CONNECTION,
    //COM_END
}

impl SupportedCommands {
    pub fn identifier(&self) -> u8 {
        match *self {
            SupportedCommands::COM_QUIT => 1,
            SupportedCommands::COM_INIT_DB => 2,
            SupportedCommands::COM_QUERY => 3,
            SupportedCommands::COM_STATISTICS => 9,
            SupportedCommands::COM_DEBUG => 13,
            SupportedCommands::COM_PING => 14,
            SupportedCommands::COM_CHANGE_USER => 18,
            SupportedCommands::COM_STMT_PREPARE => 23,
            SupportedCommands::COM_STMT_EXECUTE => 24,
            SupportedCommands::COM_STMT_SEND_LONG_DATA => 25,
            SupportedCommands::COM_STMT_CLOSE => 26,
            SupportedCommands::COM_STMT_RESET => 27,
            SupportedCommands::COM_SET_OPTION => 28,
            SupportedCommands::COM_STMT_FETCH => 29,
            SupportedCommands::COM_RESET_CONNECTION => 32
        }
    }
}

//TODO: Spend some time learning about macros and associated types...
//      create a table result outside the mysql client
//      Optional, create a command struct (probably not needed right now...)
//      
//      Have the client, if it gets a non-OK and non error response, serialize the response into the table result

//      Check: need to make sure capabilitites used in subsequent processing are those agreed to by client and servwer
