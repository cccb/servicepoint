use std::fmt::Debug;
use std::net::{ToSocketAddrs, UdpSocket};

use log::{debug, info};

use crate::Packet;

/// A connection to the display.
pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    /// Open a new UDP socket and connect to the provided host.
    ///
    /// Note that this is UDP, which means that the open call can succeed even if the display is unreachable.
    ///
    /// # Examples
    /// ```rust
    ///  let connection = servicepoint2::Connection::open("172.23.42.29:2342")
    ///     .expect("connection failed");
    /// ```
    pub fn open(addr: impl ToSocketAddrs + Debug) -> std::io::Result<Self> {
        info!("connecting to {addr:?}");
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }

    /// Send something packet-like to the display. Usually this is in the form of a Command.
    ///
    /// # Arguments
    ///
    /// * `packet`: the packet-like to send
    ///
    /// returns: Ok if packet was sent, otherwise socket error
    ///
    /// # Examples
    ///
    /// ```rust
    ///  use servicepoint2::CompressionCode;
    /// let connection = servicepoint2::Connection::open("172.23.42.29:2342")
    ///     .expect("connection failed");
    ///
    ///  // turn off all pixels
    ///  connection.send(servicepoint2::Command::Clear.into())
    ///     .expect("send failed");
    ///
    ///  // turn on all pixels
    ///  let mut pixels = servicepoint2::PixelGrid::max_sized();
    ///  pixels.fill(true);
    ///
    ///  // send pixels to display
    ///  connection.send(servicepoint2::Command::BitmapLinearWin(servicepoint2::Origin(0, 0), pixels, CompressionCode::Lzma).into())
    ///     .expect("send failed");
    /// ```
    pub fn send(&self, packet: Packet) -> Result<(), std::io::Error> {
        debug!("sending {packet:?}");
        let data: Vec<u8> = packet.into();
        self.socket.send(&data)?;
        Ok(())
    }
}

#[cfg(feature = "c_api")]
pub mod c_api {
    use std::ffi::{c_char, CStr};
    use std::ptr::null_mut;

    use crate::{Connection, Packet};

    /// Creates a new instance of Connection.
    /// The returned instance has to be deallocated with `connection_dealloc`.
    ///
    /// returns: NULL if connection fails or connected instance
    ///
    /// Panics: bad string encoding
    #[no_mangle]
    pub unsafe extern "C" fn sp2_connection_open(
        host: *const c_char,
    ) -> *mut Connection {
        let host = CStr::from_ptr(host).to_str().expect("Bad encoding");
        let connection = match Connection::open(host) {
            Err(_) => return null_mut(),
            Ok(value) => value,
        };

        Box::into_raw(Box::new(connection))
    }

    /// Sends the command instance. The instance is consumed / destroyed and cannot be used after this call.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_connection_send(
        connection: *const Connection,
        command_ptr: *mut Packet,
    ) -> bool {
        let packet = Box::from_raw(command_ptr);
        (*connection).send(*packet).is_ok()
    }

    /// Closes and deallocates a connection instance
    #[no_mangle]
    pub unsafe extern "C" fn sp2_connection_dealloc(ptr: *mut Connection) {
        _ = Box::from_raw(ptr);
    }
}
