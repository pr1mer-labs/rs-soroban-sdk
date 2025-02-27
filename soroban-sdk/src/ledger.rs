//! Ledger contains types for retrieving information about the current ledger.
use crate::{env::internal, unwrap::UnwrapInfallible, BytesN, Env, TryIntoVal};

/// Ledger retrieves information about the current ledger.
///
/// For more details about the ledger and the ledger header that the values in the Ledger are derived from, see:
///  - <https://developers.stellar.org/docs/learn/encyclopedia/network-configuration/ledger-headers>
///
/// ### Examples
///
/// ```
/// use soroban_sdk::Env;
///
/// # use soroban_sdk::{contract, contractimpl, BytesN};
/// #
/// # #[contract]
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env) {
/// let ledger = env.ledger();
///
/// let protocol_version = ledger.protocol_version();
/// let sequence = ledger.sequence();
/// let timestamp = ledger.timestamp();
/// let network_id = ledger.network_id();
/// #     }
/// # }
/// #
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     let env = Env::default();
/// #     let contract_id = env.register(Contract, ());
/// #     ContractClient::new(&env, &contract_id).f();
/// # }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
#[derive(Clone)]
pub struct Ledger(Env);

impl Ledger {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Ledger {
        Ledger(env.clone())
    }

    /// Returns the version of the protocol that the ledger created with.
    pub fn protocol_version(&self) -> u32 {
        internal::Env::get_ledger_version(self.env())
            .unwrap_infallible()
            .into()
    }

    /// Returns the sequence number of the ledger.
    ///
    /// The sequence number is a unique number for each ledger
    /// that is sequential, incremented by one for each new ledger.
    pub fn sequence(&self) -> u32 {
        internal::Env::get_ledger_sequence(self.env())
            .unwrap_infallible()
            .into()
    }

    /// Returns the maximum ledger sequence number that data can live to.
    #[doc(hidden)]
    pub fn max_live_until_ledger(&self) -> u32 {
        internal::Env::get_max_live_until_ledger(self.env())
            .unwrap_infallible()
            .into()
    }

    /// Returns a unix timestamp for when the ledger was closed.
    ///
    /// The timestamp is the number of seconds, excluding leap seconds, that
    /// have elapsed since unix epoch. Unix epoch is January 1st, 1970, at
    /// 00:00:00 UTC.
    ///
    /// For more details see:
    ///  - <https://developers.stellar.org/docs/learn/encyclopedia/network-configuration/ledger-headers#close-time>
    pub fn timestamp(&self) -> u64 {
        internal::Env::get_ledger_timestamp(self.env())
            .unwrap_infallible()
            .try_into_val(self.env())
            .unwrap()
    }

    /// Returns the network identifier.
    ///
    /// This is SHA-256 hash of the network passphrase, for example
    /// for the Public Network this returns:
    /// > SHA256(Public Global Stellar Network ; September 2015)
    ///
    /// Returns for the Test Network:
    /// > SHA256(Test SDF Network ; September 2015)
    pub fn network_id(&self) -> BytesN<32> {
        let env = self.env();
        let bin_obj = internal::Env::get_ledger_network_id(env).unwrap_infallible();
        unsafe { BytesN::<32>::unchecked_new(env.clone(), bin_obj) }
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::testutils;

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Ledger for Ledger {
    fn set(&self, li: testutils::LedgerInfo) {
        let env = self.env();
        env.host().set_ledger_info(li).unwrap();
    }

    fn set_protocol_version(&self, protocol_version: u32) {
        self.with_mut(|ledger_info| {
            ledger_info.protocol_version = protocol_version;
        });
    }

    fn set_sequence_number(&self, sequence_number: u32) {
        self.with_mut(|ledger_info| {
            ledger_info.sequence_number = sequence_number;
        });
    }

    fn set_timestamp(&self, timestamp: u64) {
        self.with_mut(|ledger_info| {
            ledger_info.timestamp = timestamp;
        });
    }

    fn set_network_id(&self, network_id: [u8; 32]) {
        self.with_mut(|ledger_info| {
            ledger_info.network_id = network_id;
        });
    }

    fn set_base_reserve(&self, base_reserve: u32) {
        self.with_mut(|ledger_info| {
            ledger_info.base_reserve = base_reserve;
        });
    }

    fn set_min_temp_entry_ttl(&self, min_temp_entry_ttl: u32) {
        self.with_mut(|ledger_info| {
            ledger_info.min_temp_entry_ttl = min_temp_entry_ttl;
        });
    }

    fn set_min_persistent_entry_ttl(&self, min_persistent_entry_ttl: u32) {
        self.with_mut(|ledger_info| {
            ledger_info.min_persistent_entry_ttl = min_persistent_entry_ttl;
        });
    }

    fn set_max_entry_ttl(&self, max_entry_ttl: u32) {
        self.with_mut(|ledger_info| {
            // For the sake of consistency across SDK methods,
            // we always make  TTL values to not include the current ledger.
            // The actual network setting in env expects this to include
            // the current ledger, so we need to add 1 here.
            ledger_info.max_entry_ttl = max_entry_ttl.saturating_add(1);
        });
    }

    fn get(&self) -> testutils::LedgerInfo {
        let env = self.env();
        env.host().with_ledger_info(|li| Ok(li.clone())).unwrap()
    }

    fn with_mut<F>(&self, f: F)
    where
        F: FnMut(&mut internal::LedgerInfo),
    {
        let env = self.env();
        env.host().with_mut_ledger_info(f).unwrap();
    }
}
