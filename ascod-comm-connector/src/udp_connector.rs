use ascod_comm_data::driver_messages::{DriverInputMessage, DriverInputStructure, DriverOutputMessage, DriverOutputStructure};
use ascod_comm_data::turret_messages::{TurretInputMessage, TurretInputStructure, TurretOutputMessage, TurretOutputStructure};
use ascod_comm_data::SIZE_MAX_UDP_MESSAGE_BUFFER;
use crossbeam_queue::ArrayQueue;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn as spawn_thread, JoinHandle};
use std::time::Duration;

pub const LOOP_GRACE_DURATION_MS: u64 = 1;
pub const MAX_PENDING_TX: usize = 1024;
pub const MAX_PENDING_RX: usize = 1024;

pub struct UDPConnector {
    stop_flag: Arc<AtomicBool>,
    loop_handle: Mutex<Option<JoinHandle<()>>>,
    pending_rx_driver: Arc<ArrayQueue<DriverInputStructure>>,
    pending_rx_turret: Arc<ArrayQueue<TurretInputStructure>>,
    pending_tx_driver: Arc<ArrayQueue<DriverOutputStructure>>,
    pending_tx_turret: Arc<ArrayQueue<TurretOutputStructure>>,
}

impl Default for UDPConnector {
    fn default() -> Self {
        Self {
            stop_flag: Arc::new(AtomicBool::new(false)),
            loop_handle: Mutex::new(None),
            pending_rx_driver: Arc::new(ArrayQueue::new(MAX_PENDING_RX)),
            pending_rx_turret: Arc::new(ArrayQueue::new(MAX_PENDING_RX)),
            pending_tx_driver: Arc::new(ArrayQueue::new(MAX_PENDING_TX)),
            pending_tx_turret: Arc::new(ArrayQueue::new(MAX_PENDING_TX)),
        }
    }
}
