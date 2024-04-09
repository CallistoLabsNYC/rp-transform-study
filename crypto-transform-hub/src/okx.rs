use serde::Deserialize;
use crate::CryptoCandle;

/*
OKX payload
{
> ts	String	Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. 1597026383085
> o	String	Open price
> h	String	Highest price
> l	String	Lowest price
> c	String	Close price
}
"{
    \"arg\":{
        \"channel\":\"mark-price-candle1m\",
        \"instId\":\"BTC-USD-SWAP\"
    },
    \"data\":
        [[
            \"1712597400000\",
            \"71338.4\",
            \"71338.8\",
            \"71338.1\",
            \"71338.8\",
            \"0\"
        ]]
    }"
}
*/

#[derive(Deserialize)]
pub struct OkxMessage {
    data: Vec<Vec<String>>,
}

impl TryFrom<OkxMessage> for CryptoCandle {
    type Error = &'static str;
    fn try_from(m: OkxMessage) -> Result<Self, Self::Error> {
        if m.data.len() == 0 || m.data[0].len() == 0 {
            return Err("missing data");
        }

        let timestamp = m.data[0][0].parse().unwrap_or_default();
        let open = m.data[0][1].parse().unwrap_or_default();
        let high = m.data[0][2].parse().unwrap_or_default();
        let low = m.data[0][3].parse().unwrap_or_default();
        let close = m.data[0][4].parse().unwrap_or_default();
        let volume = 0_f64;

        Ok(CryptoCandle {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            source: "Okx".to_owned(),
        })
    }
}