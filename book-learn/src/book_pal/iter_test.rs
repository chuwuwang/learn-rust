use regex::Regex;

pub fn iter_test() {
    tracing::info!("查找IP地址:");
    let ip_re = Regex::new(r"\b(?:\d{1,3}.){3}\d{1,3}\b").unwrap();
    let ip_text = "服务器地址: 192.168.1.1, 备用地址: 10.0.0.1, 无效IP: 999.999.999.999";

    for mat in ip_re.find_iter(ip_text) {
        let ip = mat.as_str();
        let rang = mat.range();
        tracing::info!("找到IP: '{}' at {:?}", ip, rang);
    }
}