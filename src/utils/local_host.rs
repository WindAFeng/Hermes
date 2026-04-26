use local_ip_address::list_afinet_netifas;
use std::net::{IpAddr};
use crate::errors::HermesError;
use crate::utils::log;

pub fn get_local_host() -> Result<String, HermesError> {
    match list_afinet_netifas() {
        Ok(interfaces) => {
            for (_, ip) in interfaces {
                if let IpAddr::V4(ipv4) = ip {
                    if !ipv4.is_loopback() && !ipv4.is_unspecified() && ipv4.is_private() {
                        return Ok(ipv4.to_string());
                    }
                }
            }
            log::warn("Not found LAN IP");
            Err(HermesError::Internal("Not found LAN IP".to_string()))
        }
        Err(e) => {
            Err(HermesError::Network(format!("Not Found local IP: {}", e)))
        }
    }
}