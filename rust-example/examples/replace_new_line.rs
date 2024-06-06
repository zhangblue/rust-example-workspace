fn main() {
    let request_header = "POST /UnionService/wap2_0/Core/getProvinceList.do HTTP/1.1
x-auth-token: A3EF60D601BC26BFEDB85DAD289A4CF5
charset: utf-8
channel: xcx
Host: weixin.csg.cn
Content-Length: 0
Accept-Encoding: gzip,compress,br,deflate
Referer: https://servicewechat.com/wx325a533aedaafe35/440/page-frame.html
User-Agent: Mozilla/5.0 (Linux; Android 12; NTH-AN00 Build/HONORNTH-AN00; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/116.0.0.0 Mobile Safari/537.36 XWEB/1160083 MMWEBSDK/20240301 MMWEBID/2645 MicroMessenger/8.0.48.2580(0x28003038) WeChat/arm64 Weixin NetType/WIFI Language/zh_CN ABI/arm64 MiniProgramEnv/android
cookie: CAMSID=A3EF60D601BC26BFEDB85DAD289A4CF5
content-type: application/x-www-form-urlencoded
x-tingyun: c=B|4gA5HRiCw8g
Connection: keep-alive
";

    let new_line = request_header.replace("\n", "==new_line==");

    println!("{new_line}");
    println!("=========");
    println!("{request_header}");
}
