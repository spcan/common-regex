
//! Module for common regex without capture

use regex::Regex;


pub mod numbers {
	use super::*;

	lazy_static! {
		pub static ref ALL: Regex = Regex::new(
			r"\b([\+-])?\s*(?:(?:\d+)(?:[\.,]\d+)?|(?:\d*)?(?:[\.,]\d+))\b").unwrap();
	}

	lazy_static! {
		pub static ref INT: Regex = Regex::new(
			r"([\+-])?\s*(\d+)").unwrap();
	}

	lazy_static! {
		pub static ref INT_WITH_SEPARATORS: Regex = Regex::new(
			r"([\+-])?\s*([\d_]+)").unwrap();
	}

	lazy_static! {   
		pub static ref FLOAT: Regex = Regex::new(
			r"(([\+-])?\s*((?:[0-9]+[\.,][0-9]*)|(?:[0-9]*[\.,][0-9]+)))").unwrap();
	}

	lazy_static! {
		pub static ref FLOAT_DOT: Regex = Regex::new(
			r"(([\+-])?\s*((?:[0-9]+\.[0-9]*)|(?:[0-9]*\.[0-9]+)))").unwrap();
	}

	lazy_static! {
		pub static ref FLOAT_COMMA: Regex = Regex::new(
			r"(([\+-])?\s*((?:[0-9]+,[0-9]*)|(?:[0-9]*,[0-9]+)))").unwrap();
	}

	lazy_static! {
		pub static ref ALL_WITH_SEPARATORS: Regex = Regex::new(
			r"\b([\+-])?\s*([0-9_]+[\.,][0-9_]*|[0-9_]*[\.,][0-9_]+|[0-9_]+)\b").unwrap();
	}

	lazy_static! {
		pub static ref COMPLEX: Regex = Regex::new(
			r"(([\+-])?\s*((?:\d+)(?:[\.,]\d+)?))?\s*(([\+-])\s*((?:\d+)(?:[\.,]\d+)?))\s*[ij]").unwrap();
	}

	lazy_static! {
		pub static ref HEX_NUMBER: Regex = Regex::new(
			r"#?([a-fA-F0-9]{6})").unwrap();
	}

	lazy_static! {
		pub static ref HEX_NUMBER_SHORT: Regex = Regex::new(
			r"#?([a-fA-F0-9]{3})").unwrap();
	}

	lazy_static! {
		pub static ref HEX_NUMBER_ALL: Regex = Regex::new(
			r"#?([a-fA-F0-9]{6}|[a-fA-F0-9]{3})").unwrap();
	}

	lazy_static! {
		pub static ref HEX_MACHINE_CODE: Regex = Regex::new(
			r"((?:\s*[a-fA-F0-9]{2})*)").unwrap();
	}

	lazy_static! {
		pub static ref BINARY: Regex = Regex::new(
			r"\b((?:[01])+)\b").unwrap();
	}

	lazy_static! {
		pub static ref BINARY_4: Regex = Regex::new(
			r"\b([01]{4})\b").unwrap();
	}

	lazy_static! {
		pub static ref BINARY_8: Regex = Regex::new(
			r"\b([01]{8})\b").unwrap();
	}

	lazy_static! {
		pub static ref BINARY_16: Regex = Regex::new(
			r"\b((?:[01]{16})|(?:[01]{8}_[01]{8}))\b").unwrap();
	}

	lazy_static! {
		pub static ref BINARY_32: Regex = Regex::new(
			r"\b((?:[01]{32})|(?:[01]{8}(?:_(?:[01]{8})){3}))\b").unwrap();
	}

	lazy_static! {
		pub static ref BINARY_64: Regex = Regex::new(
			r"\b((?:[01]{64})|(?:[01]{8}(?:_(?:[01]{8})){7}))\b").unwrap();
	}

	lazy_static! {
		pub static ref OCTAL: Regex = Regex::new(
			r"\b([0-7]+)\b").unwrap();
	}
}


pub mod strings {
	use super::*;

	lazy_static! {
		pub static ref CAPITALIZED: Regex = Regex::new(
			r"\b([A-Z][a-z]+)\b").unwrap();
	}

	lazy_static! {
		pub static ref CAMELCASE: Regex = Regex::new(
			r"\b([A-Za-z][A-Za-z]+)\b").unwrap();
	}

	lazy_static! {
		pub static ref SNAKE_CASE: Regex = Regex::new(
			r"\b([a-z][a-z_]+)\b").unwrap();
	}
}

pub mod time {
	use super::*;

	lazy_static! {
		pub static ref TIME_24H_A: Regex = Regex::new(
			//r"\b\d\b|0\d|1\d|2[0-3]:[0-5]\d:?[0-5]?\d?\.?\d{3}").unwrap();
			r"\b([01]\d|\d|2[0123]):([0-5]\d):?(?(?:[0-5]\d)(?:\.(?:\d{3}))?)?\b").unwrap();
	}

	lazy_static! {
		pub static ref TIME_24H_B: Regex = Regex::new(
			r"\b\d\b|0\d|1\d|2[0-3]:[0-5]\d:?[0-5]?\d?\.?\d{3}").unwrap();
			//r"\b(?:[01]\d|\d|2[0123]):(?:[0-5]\d):?(?:(?:[0-5]\d)(?:\.(?:\d{3}))?)?\b").unwrap();
	}

	lazy_static! {
		pub static ref TIME_12H_A: Regex = Regex::new(
			//r"\b\d\b|0\d|1[01]:[0-5]\d:?[0-5]?\d?\.?\d{3} pm|PM|am|AM").unwrap();
			r"\b(?:0[1-9]|[1-9]|1[012]):(?:[0-5]\d):?(?:(?:[0-5]\d)(?:\.(?:\d{3}))?)? pm|PM|am|AM\b").unwrap();
	}

	lazy_static! {
		pub static ref TIME_12H_B: Regex = Regex::new(
			r"\b\d\b|0\d|1[01]:[0-5]\d:?[0-5]?\d?\.?\d{3} pm|PM|am|AM").unwrap();
			//r"\b(?:0[1-9]|[1-9]|1[012]):(?:[0-5]\d):?(?:(?:[0-5]\d)(?:\.(?:\d{3}))?)?\b").unwrap();
	}

	lazy_static! {
		pub static ref DATE_YMD: Regex = Regex::new(
			r"\b(?:(?:(?:19|2\d)\d{2}|\d{2}|'\d{2})[/-](?:[1-9]|[0][1-9]|1[012])[/-](?:[1-9]|0[1-9]|[12]\d|3[01]))\b").unwrap();
	}

	lazy_static! {
		pub static ref DATE_YDM: Regex = Regex::new(
			r"\b(?:(?:(?:19|2\d)\d{2}|\d{2}|'\d{2})[/-](?:[1-9]|0[1-9]|[12]\d|3[01])[/-](?:[1-9]|[0][1-9]|1[012]))\b").unwrap();
	}

	lazy_static! {
		pub static ref DATE_DMY: Regex = Regex::new(
			r"\b(?:(?:[1-9]|0[1-9]|[12]\d|3[01])[/-](?:[1-9]|[0][1-9]|1[012])[/-](?:(?:19|2\d)\d{2}|\d{2}|'\d{2}))\b").unwrap();
	}
}

pub mod web {
	use super::*;

	lazy_static! {
		pub static ref HTTPS_URL: Regex = Regex::new(
			r"\bhttps://(?:www\d?)?(?:[\./\?=](?:[a-zA-Z0-9_-])*)+\b").unwrap();
	}

	lazy_static! {
		pub static ref HTTP_URL: Regex = Regex::new(
			r"\bhttp://(?:www\d?)?(?:[\./\?=](?:[a-zA-Z0-9_-])*)+\b").unwrap();
	}

	lazy_static! {
		pub static ref URL: Regex = Regex::new(
			r"\b(?:(?:[a-zA-Z])(?:\://)(?:www\d?)?)?(?:[\./\?=](?:[\w\d_-])*)+\b").unwrap();
	}

	lazy_static! {
		pub static ref WWW_URL: Regex = Regex::new(
			r"\b(?:www\d?)(?:[\./\?=](?:[a-zA-Z0-9_-])*)+\b").unwrap();
	}

	lazy_static! {
		pub static ref EMAIL_ADDR: Regex = Regex::new(
			r"\b(?:[a-zA-Z\d_\.-]+)@(?:(?:[\da-zA-Z\.-]+)\.(?:[a-zA-Z\.]*))\b").unwrap();
	}

	lazy_static! {
		pub static ref IPV4_ADDR: Regex = Regex::new(
			r"\b(?:(?:(?:\d{1,2})|(?:1[0-9][0-9])|(?:2(?:(?:[0-4][0-9])|(?:5[0-5]))))(?:[\.:](?:(?:\d{1,2})|(?:1[0-9][0-9])|(?:2(?:(?:[0-4][0-9])|(?:5[0-5]))))){3})\b").unwrap();
	}

	lazy_static! {
		pub static ref JS_SCRIPT: Regex = Regex::new(
			r#"<script(?:\s+(?:(?:[a-zA-Z]+)="(?:[a-zA-Z0-9\._-]+)"))*>(?:[\w|\n|\t|\r|\W]*)</script>"#).unwrap();
	}

	lazy_static! {
		pub static ref HTML_IMG_TAG: Regex = Regex::new(
			r#"<(?:img|IMG)(?:\s+(?:(?:[a-zA-Z]+)="(?:[a-zA-Z0-9\._-]+)"))*>"#).unwrap();
	}
}

pub mod currency {
	use super::*;

	lazy_static! {
		pub static ref DOLLAR: Regex = Regex::new(
			r"(?:(?:\$\s*\d+(?:[\.,]\d{1,10})?)|(?:\d+(?:[\.,]\d{1,10})?\s*\$))").unwrap();
	}

	lazy_static! {
		pub static ref EURO: Regex = Regex::new(
			r"(?:(?:€\s*\d+(?:[\.,]\d{1,10})?)|(?:\d+(?:[\.,]\d{1,10})?\s*€))").unwrap();
	}

	lazy_static! {
		pub static ref YEN: Regex = Regex::new(
			r"(?:(?:¥\s*\d+(?:[\.,]\d{1,10})?)|(?:\d+(?:[\.,]\d{1,10})?\s*¥))").unwrap();
	}
}

pub mod credit_cards {
	use super::*;

	lazy_static! {
		pub static ref JCB: Regex = Regex::new(
			r"\b(?:(?:2131|1800|35\d{3})(?:\d{11}|\d{14}))\b").unwrap();
	}

	lazy_static! {
		pub static ref VISA: Regex = Regex::new(
			r"\b(?:4(?:\d{12}|\d{15}|\d{18}))\b").unwrap();
	}

	lazy_static! {
		pub static ref VISA_ELECTRON: Regex = Regex::new(
			r"\b(?:(?:(?:4026|4508|4844|4913|4917)(?:\d{12}))|(?:417500)(?:\d{10}))\b").unwrap();
	}

	lazy_static! {
		pub static ref MASTERCARD: Regex = Regex::new(
			r"\b(?:(?:5[1-5])(?:\d{14}))\b|\b(?:(?:2(?:(?:2(?:(?:2)[1-9]\d{2}|(?:[3-9]\d{3})))|(?:[3-6]\d{4})|(?:7(?:(?:[01]\d{3})|(20\d{2})))))(?:\d{10}))\b").unwrap();
	}

	lazy_static! {
		pub static ref MAESTRO: Regex = Regex::new(
			r"\b(?:(?:5018|5020|5038|5893|6304|6759|6761|6762|6763)(?:\d{12}|\d{15}))\b").unwrap();
	}

	lazy_static! {
		pub static ref AMERICAN_EXPRESS: Regex = Regex::new(
			r"\b(?:(?:34|37)(?:\d{13}))\b").unwrap();
	}
}

pub mod code {
	use super::*;

	lazy_static! {
		pub static ref BRAINFUCK: Regex = Regex::new(
			r"[+-<>.,\[\]\s\t\n\r]+");
	}
}

pub mod misc {
	use super::*;

	lazy_static! {
		pub static ref ISBN: Regex = Regex::new(
			r"\b(?:(?:(?:ISBN([\s-]10)?(?::)?)?\s*(?:\d{1,5})[\s-](?:\d{1,7})[\s-](?:\d{1,5})[\s-](?:[\dX]))|(?:(?:ISBN(?:[\s-]13)?(?::)?)?\s*(?:(?:978|979)[\s-](?:\d{1,5})[\s-](?:\d{1,7})[\s-](?:\d{1,5})[\s-](?:[\dX]))))\b").unwrap();
			//r"(ISBN([\s-]1(?:(0)|3))?:?)?\x20+(?(1)(?(2)(?:(?=.{13}$)\d{1,5}([ -])\d{1,7}\3\d{1,6}\3(?:\d|x)$)|(?:(?=.{17}$)97(?:8|9)([ -])\d{1,5}\4\d{1,7}\4\d{1,6}\4\d$))|(?(.{13}$)(?:\d{1,5}([ -])\d{1,7}\5\d{1,6}\5(?:\d|x)$)|(?:(?=.{17}$)97(?:8|9)([ -])\d{1,5}\6\d{1,7}\6\d{1,6}\6\d$)))").unwrap();
	}
}

//ISBN(-1(?:(0)|3))?:?\x20+(?(1)(?(2)(?:(?=.{13}$)\d{1,5}([ -])\d{1,7}\3\d{1,6}\3(?:\d|x)$)|(?:(?=.{17}$)97(?:8|9)([ -])\d{1,5}\4\d{1,7}\4\d{1,6}\4\d$))|(?(.{13}$)(?:\d{1,5}([ -])\d{1,7}\5\d{1,6}\5(?:\d|x)$)|(?:(?=.{17}$)97(?:8|9)([ -])\d{1,5}\6\d{1,7}\6\d{1,6}\6\d$)))