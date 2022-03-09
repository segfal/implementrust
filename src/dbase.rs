//import rsqlite
extern crate rusqlite;
use rusqlite::{params,Connection, Result};







#[derive(Debug)]
struct Post {
    id: i32,
    title: String,
    body: String,
}



pub fn _findbase() -> Result<()>
{
    let conn = Connection::open("test.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts(
            id Integer PRIMARY KEY,
            title TEXT NOT NULL,
            body BLOB
            )",
    params![])?;


    let post = Post {
        id: 0,
        title: "Hello World".to_string(),
        body: "This is a test".to_string(),
    };
    conn.execute("INSERT INTO posts (title, body) VALUES (?1, ?2)",
                 params![post.title, post.body],
                )?;
    
    let mut stmt = conn.prepare("SELECT id, title, body FROM posts")?;
    let post_iter = stmt.query_map(params![], |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            body: row.get(2)?,
        })
    })?;


    for post in post_iter
    {
        println!("Found post: {:?}", post.unwrap());
    }
    

  

    return Ok(());


}


pub fn _insertdata(id : i32, title : String, body : String)
{
    let conn = Connection::open("test.db").expect("ERROR");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts(
            id Integer PRIMARY KEY,
            title TEXT NOT NULL,
            body BLOB
            )",
    params![]).expect("ERROR");


    let post = Post {
        id: id,
        title: title,
        body: body,
    };
    conn.execute("INSERT INTO posts (id, title, body) VALUES (?1, ?2, ?3)",
                 params![post.id, post.title, post.body],
                ).expect("NO DATA FOR U");

         
}



pub fn _remove_data(id : i32)
{
    let conn = Connection::open("test.db").expect("ERROr");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts(
            id Integer PRIMARY KEY,
            title TEXT NOT NULL,
            body BLOB
            )",
    params![]).expect("NOT FOUND");
    //DELETE FROM posts WHERE id = ?1
    conn.execute("DELETE FROM posts WHERE id = ?1",
                 params![id],
                ).expect("Not found");


}




   


pub fn _showdata()
{
    let conn = Connection::open("test.db")
        .expect("Error opening database");

    let mut stmt = conn.prepare("SELECT id, title, body FROM posts")
        .expect("Error preparing statement");

    let post_iter = stmt.query_map(params![], |row| {
        Ok(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            body: row.get(2)?,
        })
    }).expect("Error running query");

    for post in post_iter
    {
        println!("Found post: {:?}", post.unwrap());
    }
}

