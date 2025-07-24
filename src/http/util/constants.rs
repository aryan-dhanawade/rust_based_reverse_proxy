pub struct HttpLimits;

impl HttpLimits {
    pub const MAX_HEADERS:         usize  = 100; // Validated 
    pub const MAX_HEADER_NAME_LEN: usize  = 64;
    pub const MAX_HEADER_VALUE_LEN: usize = 8192; // 8 KB
    pub const MAX_QUERY_PARAMS:    usize  = 100; 
    pub const MAX_URL_LENGTH:      usize  = 2048; // Maximum URL length
    pub const MAX_BODY_SIZE:       usize  = 10485760; // 10 MB
}