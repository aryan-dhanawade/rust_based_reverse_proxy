use crate::http::util::url_lib::url_decode;


pub fn extract_query_params(path: &str) -> (String, Vec<(String, String)>) {
    if let Some((base_path, query_string)) = path.split_once('?') {
        let params = query_string
            .split('&')
            .filter_map(|param| {
                param.split_once('=').map(|(k, v)| {
                    (
                        url_decode(k),
                        url_decode(v),
                    )
                })
            })
            .collect();
        
        (base_path.to_string(), params)
    } else {
        (path.to_string(), Vec::new())
    }
}