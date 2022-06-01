use crate::utils::{ SLED_FILE_NAME, KVS_FILE_NAME, SLED_CODE, KVS_CODE, OK_RESPONSE, GET, SET, RM, BUFFER_LENGTH, RAYON_THREAD_POOL, SHARED_THREAD_POOL };
use crate::error::{ KvsError, Result };
use crate::engines::{ SledKvsEngine, KvStore, KvsEngine };
use crate::thread_pool::*;
use std::net::{ TcpListener, TcpStream };
use std::path::PathBuf;
use std::fs;
use std::io::{ Read, Write };
use num_cpus;

use tracing::info;
use tracing_subscriber;
pub struct KvsServer{}

impl KvsServer{
    
    pub fn route_request(ip_string: String, engine: String, pool: String) -> Result<()> {

        match engine.as_bytes() {
            KVS_CODE => KvsServer::listen_and_serve_requests_kvs(ip_string, engine, pool),
            SLED_CODE => KvsServer::listen_and_serve_requests_sled(ip_string, engine, pool),
            _ => return Err(KvsError::CommandError("Engine not found".to_string()))
        }

    }

    fn listen_and_serve_requests_sled(ip_string: String, engine: String, pool: String) -> Result<()> {

        let listener = TcpListener::bind(ip_string)?;

        match pool.as_str() {
            SHARED_THREAD_POOL => {
                let shared_thread_pool = SharedQueueThreadPool::new(num_cpus::get() as u32 )?;
                

                for stream in listener.incoming() {
                    let engine_clone = engine.clone();
                    shared_thread_pool.spawn(move || {
                        KvsServer::verify_database_type(engine_clone).expect("Verify database error");
            
                        let sled_db = SledKvsEngine::open(SLED_FILE_NAME).expect("Error opening SLED"); //TODO! How to implement error propogation within a thread?
                        
                        let sled_engine = SledKvsEngine { 
                            directory_path: PathBuf::from(SLED_FILE_NAME),
                            sled_db,
                        };
                        
                        let unwrapped_stream = stream.expect("Error unwrapping TcpStream"); //TODO! How to implement error propogation within a thread?
                        KvsServer::handle_request(unwrapped_stream, sled_engine).expect("SLED handle request error");
                    })
                }
            },
            RAYON_THREAD_POOL => {todo!()},
            _ => {return Err(KvsError::ThreadPoolError("Threadpool type not found".to_string()))}
        };

        for stream in listener.incoming() {

            KvsServer::verify_database_type(engine.clone())?;
            
            let sled_db = SledKvsEngine::open(SLED_FILE_NAME)?;
            
            let sled_engine = SledKvsEngine { 
                directory_path: PathBuf::from(SLED_FILE_NAME),
                sled_db: sled_db,
            };
            
            let unwrapped_stream = stream?;
            KvsServer::handle_request(unwrapped_stream, sled_engine)?

        }

        Ok(())
    }

    fn listen_and_serve_requests_kvs(ip_string: String, engine: String, pool: String) -> Result<()>{

        let path = PathBuf::from("");

        let listener = TcpListener::bind(ip_string)?;

        match pool.as_str() {
            SHARED_THREAD_POOL => {
                let shared_thread_pool = SharedQueueThreadPool::new(num_cpus::get() as u32 )?;

                
                for stream in listener.incoming() {
                    let engine_clone = engine.clone();
                    let path_clone = path.clone();

                    shared_thread_pool.spawn(move || {
                        KvsServer::verify_database_type(engine_clone).expect("Verify database error");
                        
                        let kv_store = KvStore::open(&path_clone).expect("Error opening KvStore"); //TODO! How to implement error propogation within a thread?
                        
                        let unwrapped_stream = stream.expect("Error unwrapping TcpStream"); //TODO! How to implement error propogation within a thread?
            
                        KvsServer::handle_request(unwrapped_stream, kv_store).expect("KvStore handle request error");
                    })


                }
            },
            RAYON_THREAD_POOL => {todo!()},
            _ => {return Err(KvsError::ThreadPoolError("Threadpool type not found".to_string()))}
        };

        Ok(())
    }

    fn verify_database_type(engine: String) -> Result<()> {
        let sled_exists = fs::metadata(SLED_FILE_NAME);
        let kvs_exists = fs::metadata(KVS_FILE_NAME);

        // info!("sled_exists: {:?}", sled_exists);
        // info!("kvs_exists: {:?}", kvs_exists);

        if sled_exists.is_ok() && engine.as_bytes() == KVS_CODE {
            // info!("Cannot use kvs engine when sled db exists");
            return Err(KvsError::CommandError("Engine mismatch. Cannot use Kvs Engine for existing Sled Engine".to_string()))
        }

        if kvs_exists.is_ok() && engine.as_bytes() == SLED_CODE {
            // info!("Cannot use sled engine when kvs db exists");
            return Err(KvsError::CommandError("Engine mismatch. Cannot use Sled Engine for existing Kvs Engine".to_string()))
        }

        Ok(())

    }

    //TODO! Perform operation by calling KvsEngine
    fn handle_request(mut stream: TcpStream, mut engine: impl KvsEngine ) -> Result<()> {

        let _subscriber = tracing_subscriber::FmtSubscriber::new();

        info!("Connection initiated");
        
        let mut buffer = [0; BUFFER_LENGTH];

        stream.read(&mut buffer)?;

        //Split arguments by space
        let arguments:Vec<&[u8]> = buffer.split(|byte| &[*byte] == b"\n").collect();

        //TODO! translate bytes in the buffer to commands
        match arguments.get(0) {
            Some(&GET) => {
                info!("Processing GET Request");
                //decode key
                let key_bytes = arguments
                            .get(1)
                            .ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                
                if let Err(error) = key_bytes {
                    return Err(error)
                }

                let key = String::from_utf8(key_bytes.unwrap().to_vec())?; //NOTE! Is there a better way to handle this?

                //Handle get request (send response back)
                let result = engine.get(key)?;

                info!("Get result: {:?}", result);

                //NOTE! If the result is not Ok(value), then error should propogate to kvs-server and the below should not execute right?
                //Send result back (encapsulate in function?)
                let response = format!("+{}\n", result.unwrap_or("Key not found".to_string()));

                stream.write(response.as_bytes())?;
                stream.flush()?;
                
            },
            Some(&SET) => {
                info!("Processing SET Request");
                //decode key and value
                let key_bytes = arguments.get(1).ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                let value_bytes = arguments.get(2).ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                
                //Handle set request (send success reponse)

                if let Err(error) = key_bytes {
                    return Err(error)
                }

                if let Err(error) = value_bytes {
                    return Err(error)
                }

                let key = String::from_utf8(key_bytes.unwrap().to_vec())?;
                let value = String::from_utf8(value_bytes.unwrap().to_vec())?;

                let _result = engine.set(key, value)?;

                //NOTE! If the result is not Ok(value), then error should propogate to kvs-server and the below should not execute right?
                //Send result back (encapsulate in function?)
                let response = OK_RESPONSE;

                stream.write(response)?;
                stream.flush()?;



            },
            Some(&RM) => {
                info!("Processing Remove Request");
                //decode key
                let key_bytes = arguments.get(1).ok_or(KvsError::CommandError("Command unrecognized".to_string()));
                //Handle remove request

                if let Err(error) = key_bytes {
                    return Err(error)
                }

                let key = String::from_utf8(key_bytes.unwrap().to_vec())?;

                let result = engine.remove(key);

                //NOTE! If the result is err, then send back Key not found?
                //Send result back (encapsulate in function?)

                if let Err(_error) = result {
                    let result = "Key not found".to_string();
                    let response = format!("+{}\n", result);
                    stream.write(response.as_bytes())?;
                    stream.flush()?;
                }
            },
            _ => {
                //return error
                return Err(KvsError::CommandError("Command unrecognized".to_string()))
            }
        }

        Ok(())

    }

    //TODO! Return result or propogate error to client

}
