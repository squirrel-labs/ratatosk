//! This module is used to instantiate and use websocket connections
//!
//! # Examples
//! ```should_panic
//! let ws = crate::websocket::WebSocketAdapter::new("wss://echo.websocket.org").expect("Websocket creation failed");
//! ws.send_str("hallo");
//! ws.close();
//! ```

//use game_engine::game::state;
use futures::future::*;
use futures::*;
use js_sys::{ArrayBuffer, Uint8Array};
use log::{debug, error};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::*;
use web_sys::{ErrorEvent, FileReaderSync, MessageEvent, WebSocket};
use webhogg_wasm_shared::ClientError;

pub struct WebSocketAdapter {
    ws: WebSocket,
}

impl WebSocketAdapter {
    /// Used to instantiate a Websocket connection
    /// # Examples
    /// ```should_panic
    /// let ws = crate::websocket::WebSocketAdapter::new("wss://echo.websocket.org").expect("Websocket creation failed");
    /// ```
    ///
    /// # Errors
    /// Returns a WebSocketError if the creation failed
    ///
    pub fn new(url: &str) -> Result<WebSocketAdapter, ClientError> {
        debug!("Websocket enry");

        // connect to the server
        let ws = WebSocket::new(url)?;
        //let ws = WebSocket::new_with_str(url, "tuesday")?;

        // register the message callback
        let onmessage_callback = Closure::wrap(
            Box::new(WebSocketAdapter::message_callback) as Box<dyn FnMut(MessageEvent)>
        );
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // keep the closure alive, although it went out of scope
        onmessage_callback.forget();

        // register the error callback
        let onerror_callback =
            Closure::wrap(Box::new(WebSocketAdapter::error_callback) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let cloned_ws = ws.clone();
        // register the open callback
        let onopen_callback = Closure::wrap(
            //Box::new(WebSocketAdapter::open_callback)
            Box::new(move |_| WebSocketAdapter::open_callback(&cloned_ws))
                as Box<dyn FnMut(JsValue)>,
        );
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        Ok(WebSocketAdapter { ws: ws })
    }

    /// Close the WebSocket connention
    pub fn close(&self) -> Result<(), ClientError> {
        Ok(self.ws.close()?)
    }

    /// Sends a `&str` if the ws is in the ready state
    ///
    /// # Errors
    /// Returns a WebSocketError if the connention is not ready or a different error occured
    ///
    pub fn send_str(&self, message: &str) -> Result<(), ClientError> {
        match self.ws.ready_state() {
            1 => self.ws.send_with_str(message).map_err(|e| e.into()),
            _ => Err(ClientError::WebSocketError(JsValue::from(
                "Websocket is not ready",
            ))),
        }
    }

    /// Sends a `&mut [u8]` if the ws is in the ready state
    ///
    /// # Errors
    /// Returns a WebSocketError if the connention is not ready or a different error occured
    ///
    pub fn send_u8_arr(&self, message: &mut [u8]) -> Result<(), ClientError> {
        //debug!("reached uint8: {:#?}", message);
        let view = unsafe { Uint8Array::view(message) };
        //let buffer = ArrayBuffer::new(message.len() as u32);
        //buffer.slice(((&mut *message as *mut [u8] as *const u8) as usize) as u32);

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace=console, js_name=log)]
            fn olog(v: &Uint8Array);
        }
        olog(&view);
        let data_view = js_sys::DataView::new(&view.buffer(), 0, message.len());

        match self.ws.ready_state() {
            1 => {
                debug!(
                    "{:?}",
                    self.ws
                        .send_with_array_buffer_view(&view.slice(0, message.len() as u32))
                );
                Ok(())
            }
            //.map_err(|e| e.into()),
            _ => Err(ClientError::WebSocketError(JsValue::from(
                "Websocket is not ready",
            ))),
        }
    }
    fn fool_the_compiler_and_js_and_shit() {}

    fn message_callback(e: MessageEvent) {
        // handle message
        let data = e.data();
        if data.is_string() {
            let response = e
                .data()
                .as_string()
                .expect("Can't convert received data to a string");
            debug!("message event, received data: {:?}", response);
        } else {
            //let fun = js_sys::Function::new_no_args("arrayBuffer");
            let blob: web_sys::Blob = data.into(); //fun.call0(&(e.data())).unwrap();
                                                   // let blob = blob.slice().unwrap();
            let reader = FileReaderSync::new().unwrap();
            let buff = reader.read_as_array_buffer(&blob).unwrap();
            let u8_arr: js_sys::Uint8Array = js_sys::Uint8Array::new(&buff);
            //let u8_arr = u8_arr.fill(3, 0, 6);
            let size = u8_arr.length();
            //let value = js_sys::Reflect::get(u8_arr.as_ref(), &JsValue::from_f64(3.0)).unwrap();
            //let mut res: *mut Vec<u8> = &mut vec![0u8; size as usize];
            //let mut res: *mut [u8] = &mut [0u8; 10];
            //res = &blob as *const web_sys::Blob as *mut [u8];
            let mut res = vec![0u8; size as usize];

            u8_arr.copy_to(&mut res);
            // let ptr = &blob as *const _;
            // let slice: &[u8] = unsafe { std::slice::from_raw_parts(ptr as *const u8, 100) };
            // debug!("Blob lives at: {} and contains: {:?}", ptr as u32, res);
            debug!("arr: {:?}", res);
            //u8_arr.copy_to(&mut res);
        }
    }

    fn error_callback(e: ErrorEvent) {
        // handle error
        error!("error event: {:?}", e);
    }

    fn open_callback(cloned_ws: &WebSocket) {
        // handle open event
        debug!("socket opend");
        match cloned_ws.send_with_str("hallo") {
            Ok(_) => debug!("message delivered"),
            Err(err) => error!("error sending message: {:#?}", err),
        }
    }
}
