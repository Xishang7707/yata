use serde::{Deserialize, Serialize};
use crate::core::{
	Error, IndicatorConfig, IndicatorInstance, IndicatorResult, Method, MovingAverageConstructor,
	PeriodType, ValueType, OHLCV,
};
use crate::helpers::MA;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATR<M: MovingAverageConstructor = MA> {
	pub method1: M,
	pub method2: M,
	pub period: PeriodType,
}

impl<M: MovingAverageConstructor> IndicatorConfig for ATR<M> {
	type Instance = ATRInstance<M>;
	const NAME: &'static str = "ATR";

	fn validate(&self) -> bool {
		self.method1.ma_period() >= 1 && self.method2.ma_period() >= 1 && self.period >= 1
	}

	fn set(&mut self, name: &str, value: String) -> Result<(), Error> {
		match name {
			"method1" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method1 = value,
			},
			"method2" => match value.parse() {
				Err(_) => return Err(Error::ParameterParse(name.to_string(), value.to_string())),
				Ok(value) => self.method2 = value,
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
		let tr = candle.tr(candle);

		Ok(Self::Instance {
			prev_close: candle.close(),
			tr_ma: cfg.method1.init(tr)?,
			ma: cfg.method2.init(0.0)?,
			cfg,
		})
	}
}

impl Default for ATR {
	fn default() -> Self {
		Self {
			method1: MA::RMA(14),
			method2: MA::RMA(14),
			period: 14,
		}
	}
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATRInstance<M: MovingAverageConstructor = MA> {
	cfg: ATR<M>,

	prev_close: ValueType,
	tr_ma: M::Instance,
	ma: M::Instance,
}

impl<M: MovingAverageConstructor> IndicatorInstance for ATRInstance<M> {
	type Config = ATR<M>;

	fn config(&self) -> &Self::Config {
		&self.cfg
	}

	fn next<T: OHLCV>(&mut self, candle: &T) -> IndicatorResult {
		let atr = self
			.ma
			.next(&self.tr_ma.next(&candle.tr_close(self.prev_close)));
		let values = [atr];
		IndicatorResult::new(&values, &[])
	}
}
