use sys_info;

pub fn fetch_sys_info() {
    // 1. 获取操作系统相关信息
    println!("=== 操作系统信息 ===");

    let os_type = sys_info::os_type().unwrap_or_default();
    println!("os_type: {}", os_type);

    let os_release = sys_info::os_release().unwrap_or_default();
    println!("os_release: {}", os_release);

    if let Ok(info) = sys_info::disk_info() {
        println!("Disk Info: {:#?}", info);
    } else {
        println!("No Disk Info");
    }

    if let Ok(info) = sys_info::mem_info() {
        println!("Mem Info: {:#?}", info);
    } else {
        println!("No Mem Info");
    }

    let cpu_num = sys_info::cpu_num().unwrap_or_default();
    println!("CPU Num: {:#?}", cpu_num);

    let cpu_speed = sys_info::cpu_speed().unwrap_or_default();
    println!("CPU Speed: {:#?}", cpu_speed);
}
