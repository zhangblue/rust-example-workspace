use redis::RedisResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_client = redis::Client::open("redis://default:cyberkl_password@172.16.103.10:6379/")?;
    let mut redis_connection = redis_client.get_connection()?;

    let cc = set_ttl_value(&mut redis_connection)?;


    Ok(())
}

fn set_ttl_value(con: &mut redis::Connection) -> redis::RedisResult<()> {


    // let _:RedisResult<()> = con.zadd("user_1","a",1);
    // let _:RedisResult<()> = con.zadd("user_1","b",2);
    // let _:RedisResult<()> = con.zadd("user_1","c",3);
    // let _:RedisResult<()> = con.zadd("user_1","d",4);
    redis::cmd("zadd").arg("user_1").arg(10).arg("cc").execute(con);

    let result: RedisResult<Vec<String>> = redis::cmd("zrangebyscore").arg("user_2").arg(1).arg(100).query(con);
    if let Ok(data) = result {
        for value in data.iter() {
            println!("-----{}", value);
        }
    } else {
        println!("错误......");
    }

    if let Ok(flag) = redis::cmd("exists").arg("user_3").query::<bool>(con) {
        println!("====={}", flag);
    }



    // let _:RedisResult<()> = con.expire("user_1",10);
    // let _:() = redis::cmd("SET").arg("user_1").arg(42).query(con)?;
    Ok(())
}

