use chrono::NaiveDateTime;
use sqlx::{
    types::chrono::{DateTime, Utc},
    Decode, Encode, Sqlite, Type,
};

#[derive(Clone)]
pub struct DbDateTime(pub DateTime<Utc>);

impl DbDateTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl Into<DateTime<Utc>> for DbDateTime {
    fn into(self) -> DateTime<Utc> {
        self.0
    }
}

impl From<DateTime<Utc>> for DbDateTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<NaiveDateTime> for DbDateTime {
    fn from(value: NaiveDateTime) -> Self {
        DbDateTime(DateTime::from_naive_utc_and_offset(value, Utc))
    }
}

impl Type<Sqlite> for DbDateTime {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <DateTime<Utc> as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for DbDateTime {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'_>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let datetime = self.0.format("%F %T.%f").to_string();
        let (datetime, microseconds) = datetime.split_once(".").unwrap();
        let nanoseconds = microseconds.to_string().parse::<u32>().unwrap();
        let milliseconds = nanoseconds / 1000_000;
        let datetime = format!("{datetime}.{milliseconds}");

        Encode::<Sqlite>::encode(datetime, buf)
    }
}

impl Decode<'_, Sqlite> for DbDateTime {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <DateTime<Utc> as Decode<'_, Sqlite>>::decode(value).map(DbDateTime)
    }
}
