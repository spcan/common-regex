//! Common regex crate

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod common;
pub mod capture;


#[cfg(test)]
pub mod numbers_test {

	#[test]
	fn int_test() {
		use common::numbers::*;

		assert!( ALL.is_match("09876"));
		assert!( ALL.is_match("-345"));
		assert!( ALL.is_match("+876"));
		assert!( ALL.is_match("0"));

		assert!( INT.is_match("09876"));
		assert!( INT.is_match("-345"));
		assert!( INT.is_match("+876"));
		assert!( INT.is_match("0"));

		assert!( INT_WITH_SEPARATORS.is_match("09876"));
		assert!( INT_WITH_SEPARATORS.is_match("-345"));
		assert!( INT_WITH_SEPARATORS.is_match("+876"));
		assert!( INT_WITH_SEPARATORS.is_match("0"));
		assert!( INT_WITH_SEPARATORS.is_match("1_000_000"));
		assert!( INT_WITH_SEPARATORS.is_match("987_2"));
		assert!( INT_WITH_SEPARATORS.is_match("987_"));

		assert!( ALL_WITH_SEPARATORS.is_match("1_000_000"));
		assert!( ALL_WITH_SEPARATORS.is_match("987_"));
	}

	#[test]
	fn float_test() {
		use common::numbers::*;

		assert!( ALL.is_match("0.0"));
		assert!( ALL.is_match(".9"));
		assert!( ALL.is_match("9."));
		assert!( ALL.is_match("1000.1000"));

		assert!( FLOAT.is_match("0.0"));
		assert!( FLOAT.is_match(".9"));
		assert!( FLOAT.is_match("9."));
		assert!(!FLOAT.is_match("9"));
		assert!(!FLOAT.is_match("."));

		assert!( FLOAT.is_match("0,0"));
		assert!( FLOAT.is_match(",9"));
		assert!( FLOAT.is_match("9,"));
		assert!(!FLOAT.is_match("9"));
		assert!(!FLOAT.is_match(","));

		assert!( FLOAT_DOT.is_match("0.0"));
		assert!( FLOAT_DOT.is_match(".9"));
		assert!( FLOAT_DOT.is_match("9."));
		assert!(!FLOAT_DOT.is_match("9"));
		assert!(!FLOAT_DOT.is_match("."));

		assert!( FLOAT_COMMA.is_match("0,0"));
		assert!( FLOAT_COMMA.is_match(",9"));
		assert!( FLOAT_COMMA.is_match("9,"));
		assert!(!FLOAT_COMMA.is_match("9"));
		assert!(!FLOAT_COMMA.is_match(","));

		assert!( ALL_WITH_SEPARATORS.is_match("0.0"));
		assert!( ALL_WITH_SEPARATORS.is_match(".9"));
		assert!( ALL_WITH_SEPARATORS.is_match("9."));
		assert!( ALL_WITH_SEPARATORS.is_match("9_9.9_9"));
	}

	#[test]
	fn complex_test() {
		use common::numbers::*;

		assert!( COMPLEX.is_match("0.9-0j"));
		assert!( COMPLEX.is_match("0-0.9j"));
		assert!( COMPLEX.is_match("0.9-0.9j"));
		assert!( COMPLEX.is_match("+0.9-0.9j"));
		assert!( COMPLEX.is_match("0-0.9j"));
		assert!( COMPLEX.is_match("0-0j"));
		assert!( COMPLEX.is_match("-0j"));
		assert!( COMPLEX.is_match("-0.9j"));
		assert!( COMPLEX.is_match("0.9-0i"));
	}

	#[test]
	fn binary_test() {
		use common::numbers::*;

		assert!( BINARY.is_match("0110011001"));
		assert!(!BINARY.is_match("0120011"));

		assert!( BINARY_4.is_match("0101"));
		assert!(!BINARY_4.is_match("0102"));

		assert!( BINARY_8.is_match("01001101"));
		assert!(!BINARY_8.is_match("02001101"));

		assert!( BINARY_16.is_match("0100110101100110"));
		assert!( BINARY_16.is_match("01100110_01100110"));

		assert!( BINARY_32.is_match("01100110011001100110011001100110"));
		assert!( BINARY_32.is_match("01100110_01100110_01100110_01100110"));

		assert!( BINARY_64.is_match("0110011001100110011001100110011001100110011001100110011001100110"));
		assert!( BINARY_64.is_match("01100110_01100110_01100110_01100110_01100110_01100110_01100110_01100110"));
	}

	#[test]
	fn hex_test() {
		use common::numbers::*;

		assert!( HEX_NUMBER.is_match( "0ae456"));
		assert!( HEX_NUMBER.is_match("#0ae456"));
		assert!( HEX_NUMBER.is_match( "0AE456"));
		assert!( HEX_NUMBER.is_match("#0AE456"));

		assert!( HEX_NUMBER_SHORT.is_match( "0fc"));
		assert!( HEX_NUMBER_SHORT.is_match("#0fc"));
		assert!( HEX_NUMBER_SHORT.is_match( "0FC"));
		assert!( HEX_NUMBER_SHORT.is_match("#0FC"));

		assert!( HEX_NUMBER_ALL.is_match( "0FD897"));
		assert!( HEX_NUMBER_ALL.is_match("#0FD897"));
		assert!( HEX_NUMBER_ALL.is_match( "0FD89753A"));
		assert!( HEX_NUMBER_ALL.is_match("#0FD89753A"));
		assert!( HEX_NUMBER_ALL.is_match( "0FD"));
		assert!( HEX_NUMBER_ALL.is_match("#0FD"));

		assert!( HEX_MACHINE_CODE.is_match("0a 0c 9a 5F 6c 77 88 09"));
	}

	#[test]
	fn octal_test() {
		use common::numbers::*;

		assert!( OCTAL.is_match("0452341234"));
		assert!(!OCTAL.is_match("0834524512"));
	}
}



#[cfg(test)]
pub mod time_date_tests {

	#[test]
	fn time_test() {
		use common::time::*;

		assert!( TIME_24H_A.is_match("09:45"));
		assert!( TIME_24H_A.is_match("09:45:33"));
		assert!( TIME_24H_A.is_match("09:45:33.345"));
		assert!( TIME_24H_A.is_match( "9:45:33.345"));
		assert!( TIME_24H_A.is_match("22:45:33.345"));
		assert!(!TIME_24H_A.is_match("24:45:33.345"));

		assert!( TIME_24H_B.is_match("09:45"));
		assert!( TIME_24H_B.is_match("09:45:33"));
		assert!( TIME_24H_B.is_match("09:45:33.345"));
		assert!( TIME_24H_B.is_match( "9:45:33.345"));
		assert!( TIME_24H_B.is_match("22:45:33.345"));
		assert!(!TIME_24H_B.is_match("24:45:33.345"));


		assert!( TIME_12H_A.is_match("09:45 am"));
		assert!( TIME_12H_A.is_match("09:45 AM"));
		assert!( TIME_12H_A.is_match("09:45 pm"));
		assert!( TIME_12H_A.is_match("09:45 PM"));
		assert!( TIME_12H_A.is_match("09:45:33 pm"));
		assert!( TIME_12H_A.is_match("09:45:33.345 pm"));
		assert!( TIME_12H_A.is_match( "9:45:33.345 pm"));
		assert!( TIME_12H_A.is_match("11:45:33.345 pm"));
		assert!(!TIME_12H_A.is_match("13:45:33.345 pm"));

		assert!( TIME_12H_B.is_match("09:45 am"));
		assert!( TIME_12H_B.is_match("09:45 AM"));
		assert!( TIME_12H_B.is_match("09:45 pm"));
		assert!( TIME_12H_B.is_match("09:45 PM"));
		assert!( TIME_12H_B.is_match("09:45:33 pm"));
		assert!( TIME_12H_B.is_match("09:45:33.345 pm"));
		assert!( TIME_12H_B.is_match( "9:45:33.345 pm"));
		assert!( TIME_12H_B.is_match("11:45:33.345 pm"));
		assert!(!TIME_12H_B.is_match("13:45:33.345 pm"));
	}

	#[test]
	fn date_test() {
		use common::time::*;

		assert!( DATE_YMD.is_match("2017/12/31"));
		assert!( DATE_YMD.is_match("2017/09/09"));
		assert!( DATE_YMD.is_match("2017/9/9"));
		assert!( DATE_YMD.is_match( "17/12/31"));
		assert!( DATE_YMD.is_match("'17/12/31"));
		assert!(!DATE_YMD.is_match("2017/13/31"));
		assert!(!DATE_YMD.is_match("2017/12/32"));


		assert!( DATE_YDM.is_match("2017/31/12"));
		assert!( DATE_YDM.is_match("2017/09/09"));
		assert!( DATE_YDM.is_match("2017/9/9"));
		assert!( DATE_YDM.is_match( "17/31/12"));
		assert!( DATE_YDM.is_match("'17/31/12"));
		assert!(!DATE_YDM.is_match("2017/31/13"));
		assert!(!DATE_YDM.is_match("2017/32/12"));


		assert!( DATE_DMY.is_match("31/12/2017"));
		assert!( DATE_DMY.is_match("09/09/2017"));
		assert!( DATE_DMY.is_match("9/9/2017"));
		assert!( DATE_DMY.is_match("31/12/17"));
		assert!( DATE_DMY.is_match("31/12/'17"));
		assert!(!DATE_DMY.is_match("31/13/2017"));
		assert!(!DATE_DMY.is_match("32/12/2017"));
	}
}

#[cfg(test)]
pub mod currency_tests {

	#[test]
	fn currency_test() {
		use common::currency::*;

		assert!( DOLLAR.is_match("$98"));
		assert!( DOLLAR.is_match("$ 98"));
		assert!( DOLLAR.is_match("$98.0"));
		assert!( DOLLAR.is_match("$98.9"));
		assert!(!DOLLAR.is_match("$.09"));
		assert!( DOLLAR.is_match("98$"));
		assert!( DOLLAR.is_match("98 $"));
		assert!( DOLLAR.is_match("98.9 $"));

		assert!( EURO.is_match("€98"));
		assert!( EURO.is_match("€ 98"));
		assert!( EURO.is_match("€98.0"));
		assert!( EURO.is_match("€98.9"));
		assert!(!EURO.is_match("€.09"));
		assert!( EURO.is_match("98€"));
		assert!( EURO.is_match("98 €"));
		assert!( EURO.is_match("98.9 €"));

		assert!( YEN.is_match("¥98"));
		assert!( YEN.is_match("¥ 98"));
		assert!( YEN.is_match("¥98.0"));
		assert!( YEN.is_match("¥98.9"));
		assert!(!YEN.is_match("¥.09"));
		assert!( YEN.is_match("98¥"));
		assert!( YEN.is_match("98 ¥"));
		assert!( YEN.is_match("98.9 ¥"));
	}
}

#[cfg(test)]
pub mod web_test {

	#[test]
	fn js_test() {
		use common::web::*;

		assert!( JS_SCRIPT.is_match("<script> This is a script 1234567890 _ << >> ¿?¡!</script>"));
		assert!( JS_SCRIPT.is_match("<script src=\"my_scource\"></script>"));
		assert!( JS_SCRIPT.is_match("<script src=\"my_scource\" async=\"true\"></script>"));
		assert!(!JS_SCRIPT.is_match("<script> This isn't a script"));
	}

	#[test]
	fn ipv4_test() {
		use common::web::*;

		assert!( IPV4_ADDR.is_match("127.0.0.1"));
		assert!( IPV4_ADDR.is_match("255.255.255.255"));
		assert!( IPV4_ADDR.is_match("0.0.0.0"));
		assert!( IPV4_ADDR.is_match("127:0:0:1"));

		assert!(!IPV4_ADDR.is_match("256.0.0.0"));
		assert!(!IPV4_ADDR.is_match("2561.0.0.0"));
	}

	#[test]
	fn email_test() {
		use common::web::*;

		assert!( EMAIL_ADDR.is_match("myname123@myservice.com"));
		assert!( EMAIL_ADDR.is_match("myname.myfriend123@myservice.com"));
		assert!( EMAIL_ADDR.is_match("myname.myfriend@myservice.mycompany123.com"));
		assert!( EMAIL_ADDR.is_match("myname_1-2@myservice.com"));
		assert!(!EMAIL_ADDR.is_match("myname @myservice.com"));
	}

	#[test]
	fn img_tag_test() {
		use common::web::*;

		assert!( HTML_IMG_TAG.is_match(r#"<img>"#));
		assert!( HTML_IMG_TAG.is_match(r#"<img src="mysource.jpg">"#));
		assert!( HTML_IMG_TAG.is_match(r#"<IMG src="mysource.jpg" alt="alt" height="height" width="width">"#));
	}
}

#[cfg(test)]
pub mod string_tests {

	#[test]
	fn string_styles() {
		use common::strings::*;

		assert!( CAPITALIZED.is_match("Alabama"));
		assert!(!CAPITALIZED.is_match("alabama"));
		assert!(!CAPITALIZED.is_match("alaBama"));
		assert!(!CAPITALIZED.is_match("ala_bama"));

		assert!( CAMELCASE.is_match("Alabama"));
		assert!( CAMELCASE.is_match("AlaBama"));
		assert!( CAMELCASE.is_match("alabama"));
		assert!( CAMELCASE.is_match("alaBama"));
		assert!(!CAMELCASE.is_match("ala_bama"));

		assert!(!SNAKE_CASE.is_match("Alabama"));
		assert!( SNAKE_CASE.is_match("alabama"));
		assert!(!SNAKE_CASE.is_match("alaBama"));
		assert!( SNAKE_CASE.is_match("ala_bama"));

	}
}

#[cfg(test)]
pub mod credit_cards {

	#[test]
	fn credit_card_test() {
		use common::credit_cards::*;

		/// JCB
		assert!( JCB.is_match("3536643045985920"));
		assert!( JCB.is_match("3532154262139484"));
		assert!( JCB.is_match("3533854221675050914"));
		// Testing for wrong prefix
		assert!(!JCB.is_match("3133854221675050914"));
		// Testing for extra characters
		assert!(!JCB.is_match("35321542621394846"));
		assert!(!JCB.is_match("35338542216750509146"));


		/// VISA
		assert!( VISA.is_match("4532047718961022"));
		assert!( VISA.is_match("4786059915563017"));
		assert!( VISA.is_match("4929046258996641920"));
		// Testing for wrong prefix
		assert!(!VISA.is_match("3929046258996641920"));
		// Testing for extra characters
		assert!(!VISA.is_match("47860599155630171"));
		assert!(!VISA.is_match("49290462589966419201"));


		/// VISA-ELECTRON
		assert!( VISA_ELECTRON.is_match("4913441710323993"));
		assert!( VISA_ELECTRON.is_match("4917510482383918"));
		assert!( VISA_ELECTRON.is_match("4175008556203895"));
		// Testing for wrong prefix
		assert!(!VISA_ELECTRON.is_match("4185008556203895"));
		// Testing for extra characters
		assert!(!VISA_ELECTRON.is_match("49175104823839181"));
		assert!(!VISA_ELECTRON.is_match("41750085562038951"));


		/// MASTERCARD
		assert!( MASTERCARD.is_match("5193196478500566"));
		assert!( MASTERCARD.is_match("2221008881568325"));
		assert!( MASTERCARD.is_match("2720994955959936"));
		// Testing for wrong prefix
		assert!(!MASTERCARD.is_match("2820994955959936"));
		assert!(!MASTERCARD.is_match("2220984955959936"));
		// Testing for extra characters
		assert!(!MASTERCARD.is_match("22210088815683251"));
		assert!(!MASTERCARD.is_match("27210049559599361"));


		/// MAESTRO
		assert!( MAESTRO.is_match("6761055171911696"));
		assert!( MAESTRO.is_match("5020739142024627"));
		assert!( MAESTRO.is_match("6763814483769103"));
		// Testing for wrong prefix
		assert!(!MAESTRO.is_match("5120739142024627"));
		assert!(!MAESTRO.is_match("6764814483769103"));
		// Testing for extra characters
		assert!(!MAESTRO.is_match("50207391420246271"));
		assert!(!MAESTRO.is_match("67638144837691031"));


		/// AMERICAN_EXPRESS
		assert!( AMERICAN_EXPRESS.is_match("373530378260116"));
		assert!( AMERICAN_EXPRESS.is_match("340266408484539"));
		assert!( AMERICAN_EXPRESS.is_match("345346384459328"));
		// Testing for wrong prefix
		assert!(!AMERICAN_EXPRESS.is_match("350266408484539"));
		assert!(!AMERICAN_EXPRESS.is_match("355346384459328"));
		// Testing for extra characters
		assert!(!AMERICAN_EXPRESS.is_match("3402664084845391"));
		assert!(!AMERICAN_EXPRESS.is_match("3453463844593281"));
	}
}

#[cfg(test)]
pub mod misc {

	#[test]
	fn isbn_test() {
		use common::misc::*;

		assert!( ISBN.is_match("978-1-4028-9462-6"));
		assert!( ISBN.is_match("1-4028-9462-7"));

		assert!( ISBN.is_match("ISBN-13: 978-1-4028-9462-6"));
		assert!( ISBN.is_match("ISBN 13: 978-1-4028-9462-6"));
		assert!( ISBN.is_match("ISBN-13 978-1-4028-9462-6"));
		assert!( ISBN.is_match("ISBN 978-1-4028-9462-6"));

		assert!( ISBN.is_match("ISBN-10: 1-4028-9462-7"));
		assert!( ISBN.is_match("ISBN 10: 1-4028-9462-7"));
		assert!( ISBN.is_match("ISBN-10 1-4028-9462-7"));
		assert!( ISBN.is_match("ISBN 1-4028-9462-7"));
	}
}