// The Application ID of the Dexcom Share App
pub const DEXCOM_APP_ID: &str = "d89443d2-327c-4a6f-89e5-496bbb0317db";

// 0'd out UUID. if the api returns this, you have an error typically
pub const DEXCOM_NULL_UUID: &str = "00000000-0000-0000-0000-000000000000";

// Base URL of the Dexcom Share API (inside the USA)
pub const DEXCOM_BASE_URL_US: &str = "https://share2.dexcom.com/ShareWebServices/Services";

// Base URL of the Dexcom Share API (outside the USA)
pub const DEXCOM_BASE_URL_NON_US: &str = "https://shareous1.dexcom.com/ShareWebServices/Services";

/* Dexcom URL Endpoints, which are the same for both US and non-US */

// Login Endpoint for fetching a session ID
pub const DEXCOM_LOGIN_ENDPOINT: &str = "General/LoginPublisherAccountById";

// Authentication Endpoint for getting an account ID
pub const DEXCOM_AUTH_ENDPOINT: &str = "General/AuthenticatePublisherAccount";

// Glucose Data endpoint to get glucose values
pub const DEXCOM_GLUCOSE_DATA_ENDPOINT: &str = "Publisher/ReadPublisherLatestGlucoseValues";

// Conversion factor between mg/dL and mmol/L
pub const MMOL_CONVERSION_FACTOR: f32 = 0.0555;
