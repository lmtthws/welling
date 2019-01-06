    
    #[derive(Clone)]
    pub struct Timestamp {
        pub raw: String,
        pub date: Option<Date>,
        pub time: Option<Time>
    }

    #[derive(Copy,Clone)]
    pub struct Date {
        month: Month,
        day: u8,
        year: u8
    }

    #[derive(Copy,Clone)]
    pub enum Month {
        Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec
    }

    #[derive(Copy,Clone)]
    pub struct Time {
        hour: u8,
        minute: u8,
        second: u8,
        timezone: Timezone
    }

    #[derive(Copy,Clone)]
    pub enum Timezone {
        UTC
    }