use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn read_all_item(key: &RegKey, subkey: &str) -> io::Result<Vec<String>> {
    // 读取对应的项
    let item = key.open_subkey(subkey)?;
    
    // 创建一个动态数组
    let mut items = Vec::new();

    // 遍历出子项
    for sub_item in item.enum_keys().filter_map(Result::ok) {
        // 加入到数组中s
        items.push(sub_item);
    }

    Ok(items)
}

fn delete_item(key: &RegKey, subkey: &str) -> io::Result<()> {
    key.delete_subkey(subkey)?;

    Ok(())
}

fn main() {
    println!("Deleting Registry Keys..."),

    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let profiles = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\Profiles";
    let unmanaged = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\Signatures\Unmanaged";

    // 处理函数返回的数据
    match read_all_item(&local_machine, profiles) {
        Ok(paths) => {
            for path in paths {
                let item = format!("{}\\{}", profiles, path);
                match delete_item(&local_machine, &item) {
                    Ok(_) => println!("Deleted Registry Key: {}", path),
                    Err(e) => eprintln!("Failed to Delete Registry Key {}: {}", path, e),
                }
            }
        },
        Err(e) => eprintln!("Failed to Read Profiles Item: {}", e),
    }

    // 处理函数返回的数据
    match read_all_item(&local_machine, unmanaged) {
        Ok(paths) => {
            for path in paths {
                let item = format!("{}\\{}", unmanaged, path);
                match delete_item(&local_machine, &item) {
                    Ok(_) => println!("Deleted Registry Key: {}", path),
                    Err(e) => eprintln!("Failed to Delete Registry Key {}: {}", path, e),
                }
            }
        },
        Err(e) => eprintln!("Failed to Read Unmanaged Item: {}", e),
    }
}
