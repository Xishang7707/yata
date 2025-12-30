use crate::core::{Error, Method, PeriodType, ValueType, OHLCV};
use crate::helpers::Peekable;
use crate::methods::{EMA, RMA, SMA, TR, WMA};

pub enum ATRSmooth {
	RMA,
	SMA,
	EMA,
	WMA,
}

type ATRSmoothMA = dyn Method<Input = ValueType, Output = ValueType, Params = PeriodType>;

pub struct ATRParams {
	smooth: ATRSmooth,
	length: PeriodType,
}

pub struct ATR {
	tr: TR,
	ma: Box<ATRSmoothMA>,
	value: ValueType,
}

impl Method for ATR {
	type Params = ATRParams;
	type Input = dyn OHLCV;
	type Output = ValueType;

	fn new(parameters: Self::Params, value: &Self::Input) -> Result<Self, Error>
	where
		Self: Sized,
	{
		if parameters.length <= 0 {
			return Err(Error::WrongMethodParameters);
		}

		let ma: Box<ATRSmoothMA> = match parameters.smooth {
			ATRSmooth::RMA => Box::new(RMA::new(parameters.length, &value.close())?),
			ATRSmooth::SMA => Box::new(SMA::new(parameters.length, &value.close())?),
			ATRSmooth::EMA => Box::new(EMA::new(parameters.length, &value.close())?),
			ATRSmooth::WMA => Box::new(WMA::new(parameters.length, &value.close())?),
		};

		Ok(Self {
			tr: TR::new(value)?,
			ma,
			value: 0f64,
		})
	}

	fn next(&mut self, value: &Self::Input) -> Self::Output {
		self.ma.next(&self.tr.next(value))
	}
}

impl Peekable<<Self as Method>::Output> for ATR {
	fn peek(&self) -> <Self as Method>::Output {
		self.value
	}
}
