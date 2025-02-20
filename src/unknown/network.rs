// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{IpNetwork, MacAddr, NetworkData};

use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct NetworksInner {
    pub(crate) interfaces: HashMap<String, NetworkData>,
}

impl NetworksInner {
    pub(crate) fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
        }
    }

    pub(crate) fn list(&self) -> &HashMap<String, NetworkData> {
        &self.interfaces
    }

    pub(crate) fn refresh(&mut self, _remove_not_listed_interfaces: bool) {}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct NetworkDataInner;

impl NetworkDataInner {
    pub(crate) fn received(&self) -> u64 {
        0
    }

    pub(crate) fn total_received(&self) -> u64 {
        0
    }

    pub(crate) fn transmitted(&self) -> u64 {
        0
    }

    pub(crate) fn total_transmitted(&self) -> u64 {
        0
    }

    pub(crate) fn packets_received(&self) -> u64 {
        0
    }

    pub(crate) fn total_packets_received(&self) -> u64 {
        0
    }

    pub(crate) fn packets_transmitted(&self) -> u64 {
        0
    }

    pub(crate) fn total_packets_transmitted(&self) -> u64 {
        0
    }

    pub(crate) fn errors_on_received(&self) -> u64 {
        0
    }

    pub(crate) fn total_errors_on_received(&self) -> u64 {
        0
    }

    pub(crate) fn errors_on_transmitted(&self) -> u64 {
        0
    }

    pub(crate) fn total_errors_on_transmitted(&self) -> u64 {
        0
    }

    pub(crate) fn mac_address(&self) -> MacAddr {
        MacAddr::UNSPECIFIED
    }

    pub(crate) fn ip_networks(&self) -> &[IpNetwork] {
        &[]
    }

    pub(crate) fn mtu(&self) -> u64 {
        0
    }
}
