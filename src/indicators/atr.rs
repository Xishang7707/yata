use crate::core::{
	Error, IndicatorConfig, IndicatorInstance, IndicatorResult, Method, MovingAverageConstructor,
	PeriodType, ValueType, OHLCV,
};
use crate::helpers::MA;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATR<M: MovingAverageConstructor = MA> {
	pub ma: M,
	pub period: PeriodType,
}

impl<M: MovingAverageConstructor> IndicatorConfig for ATR<M> {
	type Instance = ATRInstance<M>;
	const NAME: &'static str = "ATR";

	fn validate(&self) -> bool {
		self.ma.ma_period() >= 1
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"ma" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.ma = value,
			},
			"period" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.period = value,
			},
			_ => {
				return Err(Error::ParameterParse(name.to_string(), value));
			}
		};

		Ok(())
	}

	fn size(&self) -> (u8, u8) {
		(1, 0)
	}

	fn init<T: OHLCV>(self, candle: &T) -> Result<Self::Instance, Error> {
		if !self.validate() {
			return Err(Error::WrongConfig);
		}

		let cfg = self;
		let tr = candle.tr_close(f64::NAN);

		Ok(Self::Instance {
			prev_close: candle.close(),
			ma: cfg.ma.init(tr)?,
			cfg,
		})
	}
}

impl Default for ATR {
	fn default() -> Self {
		Self {
			ma: MA::RMA(14),
			period: 14,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATRInstance<M: MovingAverageConstructor = MA> {
	cfg: ATR<M>,

	prev_close: ValueType,
	ma: M::Instance,
}

impl<M: MovingAverageConstructor> IndicatorInstance for ATRInstance<M> {
	type Config = ATR<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let atr = self.ma.next(&candle.tr_close(self.prev_close));
		let values = [atr];
		IndicatorResult::new(&values, &[])
	}
}
