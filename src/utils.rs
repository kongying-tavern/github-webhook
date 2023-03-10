use std::io::{BufReader, BufRead};
use std::process::{Command, Stdio, ExitStatus};
use tracing::info;
use sha2::Sha256;
use hmac::{Hmac, Mac};

type HmacSha256 = Hmac<Sha256>;

pub async fn exec_ex(cmd: &str, args: &[&str]) -> std::io::Result<ExitStatus> {
    let mut child = Command::new(cmd).args(args).stdout(Stdio::piped())
                                 .spawn()
                                 .unwrap();
    let mut out = BufReader::new(child.stdout.as_mut().unwrap());
    let mut line = String::new();
    while out.read_line(&mut line).unwrap() > 0 {
        info!("{}", line);
    }
    Ok(child.wait()?)
}

pub async fn shell_exec_ex(cmd: &str, args: &[&str]) -> std::io::Result<ExitStatus> {
    let mut args_vec = vec![];
    let shell = if cfg!(target_os = "windows") {
        args_vec.push("/s");
        args_vec.push("/c");
        "cmd"
    } else {
        args_vec.push("-c");
        "sh"
    };
    let cmd = cmd.to_string() + " " + &args.join(" ");
    args_vec.push(cmd.as_str());
    Ok(exec_ex(shell, args_vec.as_ref()).await.unwrap())
}

pub async fn shell_exec(cmd: &str) -> std::io::Result<ExitStatus> {
    let (cmd, args) = parse_cmd(cmd);
    Ok(shell_exec_ex(cmd, args.as_slice()).await.unwrap())
}

fn parse_cmd(cmd: &str) -> (&str, Vec<&str>) {
    let splited: Vec<&str> = cmd.split(" ").into_iter().collect();
    let splited = splited.split_first().unwrap();
    (splited.0, splited.1.to_vec())
}

pub fn hash_hmac_sha256(data: &[u8], key: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    return hex::encode(code_bytes);
}
