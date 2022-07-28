
use mysql::*;
use mysql::prelude::*;

// #[derive(Debug, PartialEq, Eq)]
// struct Payment {
//     customer_id: i32,
//     amount: i32,
//     account_name: Option<String>,
// }

pub fn conn() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://admin:admin1234@192.168.10.17:8806/octopus";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    conn.query_iter("select version()")?.for_each(|row|{
        let r:String = from_row(row.unwrap());
        println!("{}", r);
    });
    Ok(())
}

pub struct Rdb {
    pool: Pool,
    conn:PooledConn
}

impl Rdb {
    pub fn new(url:&str)->Result<Rdb> {
        let pool = Pool::new(url)?;
        let conn = pool.get_conn()?;
        Ok(Rdb{
            pool:pool,
            conn:conn
        })
    }

}