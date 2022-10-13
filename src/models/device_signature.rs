use diesel::pg::Pg;
use diesel::serialize::ToSql;
use diesel::sql_types::Bytea;
use diesel::AsExpression;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(AsExpression)]
#[diesel(sql_type = Bytea)]
pub struct DeviceSignature(SignitureBits);

#[repr(C)]
#[derive(Clone, Copy, Eq)]
union SignitureBits {
    integer: u128,
    bytes: [u8; 16]
}

impl PartialEq for SignitureBits {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            <u128 as PartialEq>::eq(&self.integer, &other.integer)
        }
    }

    fn ne(&self, other: &Self) -> bool {
        unsafe {
            <u128 as PartialEq>::ne(&self.integer, &other.integer)
        }
    }
}

impl core::fmt::Debug for DeviceSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#032X}", unsafe { self.0.integer })
    }
}

impl std::fmt::Display for DeviceSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#032X}", unsafe { self.0.integer })
    }
}

impl<'a> ToSql<Bytea, Pg> for DeviceSignature where &'a [u8]: ToSql<Bytea, Pg> {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <[u8] as ToSql::<Bytea, Pg>>::to_sql(unsafe { &self.0.bytes }, out)
    }
}