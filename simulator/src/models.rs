use soroban_env_host::xdr::{Error, LedgerEntry, LedgerEntryData, LedgerEntryExt, Limits, ReadXdr, WriteXdr};

pub trait LedgerEntryPartialSerialize: Sized {
    fn partial_serialize(&self) -> Result<Vec<u8>, Error>;
    fn partial_deserialize(bytes: &[u8]) -> Result<Self, Error>;
}

impl LedgerEntryPartialSerialize for LedgerEntry {
    fn partial_serialize(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        self.last_modified_ledger_seq.write_xdr(&mut out)?;
        self.data.write_xdr(&mut out)?;
        Ok(out)
    }

    fn partial_deserialize(mut bytes: &[u8]) -> Result<Self, Error> {
        let last_modified_ledger_seq = u32::read_xdr(&mut bytes, Limits::none())?;
        let data = LedgerEntryData::read_xdr(&mut bytes, Limits::none())?;
        Ok(LedgerEntry {
            last_modified_ledger_seq,
            data,
            ext: LedgerEntryExt::V0,
        })
    }
}
