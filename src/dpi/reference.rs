use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;

/// The collection of methods that enable a structure to find retrieve lists of lib codes (logic)
/// used to identify private data.
pub trait IdentifierLogic {
    /// This function retrieve a list of the lib codes
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::{IdentifierLogic, Lib};
    ///
    /// struct Logic {}
    /// impl IdentifierLogic for Logic {}
    /// let lists = Logic::get_list(26000, 26002);
    ///
    /// assert_eq!(lists.len(), 3);
    /// ```
    fn get_list(min: u16, max: u16) -> Vec<String> {
        let mut list = Vec::new();

        for l in min..max + 1 {
            match Lib::from_u16(l) {
                Ok(val) => match val.to_string() == *"<unknown code>" {
                    true => break,
                    false => list.push(val.to_string()),
                },
                Err(_err) => break,
            }
        }

        list
    }

    /// This function retreives all the words, regexs,
    /// and patterns that are used to identify basic private data
    /// such as names and addresses.
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::{IdentifierLogic, Lib};
    ///
    /// struct Logic {}
    /// impl IdentifierLogic for Logic {}
    /// let lists = Logic::basic_list();
    ///
    /// assert_eq!(lists.len(), 3);
    /// assert_eq!(lists.get("words").unwrap().len(), 0);
    /// assert_eq!(lists.get("regexs").unwrap().len(), 17);
    /// assert_eq!(lists.get("patterns").unwrap().len(), 0);
    /// ```
    fn basic_list() -> BTreeMap<String, Vec<String>> {
        let mut lists = BTreeMap::new();

        lists.insert("words".to_string(), Self::get_list(10000, 10999));
        lists.insert("regexs".to_string(), Self::get_list(20000, 20999));
        lists.insert("patterns".to_string(), Self::get_list(30000, 30999));

        lists
    }

    /// This function retreives all the words, regexs,
    /// and patterns that are used to identify Health related data.
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::{IdentifierLogic, Lib};
    ///
    /// struct Logic {}
    /// impl IdentifierLogic for Logic {}
    /// let lists = Logic::health_list();
    ///
    /// assert_eq!(lists.len(), 3);
    /// assert_eq!(lists.get("words").unwrap().len(), 0);
    /// assert_eq!(lists.get("regexs").unwrap().len(), 9);
    /// assert_eq!(lists.get("patterns").unwrap().len(), 0);
    /// ```
    fn health_list() -> BTreeMap<String, Vec<String>> {
        let mut lists = BTreeMap::new();

        lists.insert("words".to_string(), Self::get_list(0, 1));
        lists.insert("regexs".to_string(), Self::get_list(26000, 26999));
        lists.insert("patterns".to_string(), Self::get_list(0, 1));

        lists
    }

    /// This function retreives all the words, regexs,
    /// and patterns that are used to identify NPPI data.
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::{IdentifierLogic, Lib};
    ///
    /// struct Logic {}
    /// impl IdentifierLogic for Logic {}
    /// let lists = Logic::nppi_list();
    ///
    /// assert_eq!(lists.len(), 3);
    /// assert_eq!(lists.get("words").unwrap().len(), 3);
    /// assert_eq!(lists.get("regexs").unwrap().len(), 69);
    /// assert_eq!(lists.get("patterns").unwrap().len(), 5);
    /// ```
    fn nppi_list() -> BTreeMap<String, Vec<String>> {
        let mut lists = BTreeMap::new();

        lists.insert("words".to_string(), Self::get_list(15000, 15999));
        lists.insert("regexs".to_string(), Self::get_list(25000, 25999));
        lists.insert("patterns".to_string(), Self::get_list(35000, 35999));

        lists
    }

    /// This function retreives all the words, regexs,
    /// and patterns that are used to identify PCI data.
    ///
    /// #Example
    ///
    /// ```rust
    /// use pbd::dpi::DPI;
    /// use pbd::dpi::reference::{IdentifierLogic, Lib};
    ///
    /// struct Logic {}
    /// impl IdentifierLogic for Logic {}
    /// let lists = Logic::pci_list();
    ///
    /// assert_eq!(lists.len(), 3);
    /// assert_eq!(lists.get("words").unwrap().len(), 0);
    /// assert_eq!(lists.get("regexs").unwrap().len(), 20);
    /// assert_eq!(lists.get("patterns").unwrap().len(), 0);
    /// ```
    fn pci_list() -> BTreeMap<String, Vec<String>> {
        let mut lists = BTreeMap::new();

        lists.insert("words".to_string(), Self::get_list(0, 1));
        lists.insert("regexs".to_string(), Self::get_list(27000, 27999));
        lists.insert("patterns".to_string(), Self::get_list(0, 1));

        lists
    }
}

/// Represents a DPI Library Code
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lib(NonZeroU16);

/// A possible error value when converting a `DPI Library Code` from a `u16` or `&str`
///
/// This error indicates that the supplied input was not a valid number,
/// was less than 10000 or greater than 65535.
pub struct InvalidCode {
    _priv: (),
}

macro_rules! lib_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl Lib {
        $(
            $(#[$docs])*
            pub const $konst: Lib = Lib(unsafe { NonZeroU16::new_unchecked($num) });
        )+

        }

        fn get_value(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

lib_codes! {
    /// 15000 Social Security Number abbreviated
    (15000, TEXT_SSN_FULL, r"Social Security Number");
    /// 15001 Social Security Number abbreviated
    (15001, TEXT_SSN_ABBR, r"SSN");
    /// 15002 Account
    (15002, TEXT_ACCOUNT, r"account");

    /// Regex PII
    /// 20000 Human Name
    (20000, REGEX_HUMAN_NAME, r"^[a-zA-Z]+(([',. -][a-zA-Z ])?[a-zA-Z]*)*$");
    /// 20001 Avenue or Ave (case insensitive)
    (20001, REGEX_ADDR_AVE, r"/avenue|\bave\b/gim");
    /// 20002 Boulevard or Blvrd (case insensitive)
    (20002, REGEX_ADDR_BLVD, r"/boulevard|\bblvd\b/gim");
    /// 20003 Drive or Dr (case insensitive)
    (20003, REGEX_ADDR_DR, r"/drive|\bdr\b/gim");
    /// 20004 East(case insensitive)
    (20004, REGEX_ADDR_EAST, r"/east/gim");
    /// 20005 Highway or Hwy (case insensitive)
    (20005, REGEX_ADDR_HWY, r"/highway|\bhwy\b/gim");
    /// 20006 Lane or Ln (case insensitive)
    (20006, REGEX_ADDR_LN, r"/lane|\bln\b/gim");
    /// 20007 North (case insensitive)
    (20007, REGEX_ADDR_NORTH, r"/north/gim");
    /// 20008 Road or Rd (case insensitive)
    (20008, REGEX_ADDR_RD, r"/road|\brd\b/gim");
    /// 20009 South (case insensitive)
    (20009, REGEX_ADDR_SOUTH, r"/south/gim");
    /// 20010 Street or St or Str (case insensitive)
    (20010, REGEX_ADDR_STR, r"/street|\bst\b|\bstr\b/gim");
    /// 20011 Township or Twp (case insensitive)
    (20011, REGEX_ADDR_TWP, r"/township|\btwp\b/gim");
    /// 20012 West (case insensitive)
    (20012, REGEX_ADDR_WEST, r"/west/gim");
    /// 20013 US phone number
    //(20013, REGEX_US_PHONE, r"/([0-9]{10})|([0-9]{7})|^(\([0-9]{3}\))|([0-9]{3}-[0-9]{4})|([ext])/gim");
    (20013, REGEX_US_PHONE, r"^([0-9]( |-)?)?(\(?[0-9]{3}\)?|[0-9]{3})( |-)?([0-9]{3}( |-)?[0-9]{4}|[a-zA-Z0-9]{7})$");
    /// 20014 Email (case insensitive)
    (20014, REGEX_EMAIL, r"/([\w\.-]+)@([\da-zA-Z\.-]+)\.([a-zA-Z\.]{2,6})/gim");
    /// 20015 Zipcode
    (20015, REGEX_ZIPCODE, r"^\d{5}-\d{4}|\d{5}|[A-Z]\d[A-Z] \d[A-Z]\d$");
    /// 20016 IP address
    (20016, REGEX_IP, r"^(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])$");

    /// 25000 Social Security Number with dashes
    (25000, REGEX_SSN_DASHES, r"^\d{3}-\d{2}-\d{4}$");
    /// 25000 Social Security Number without dashes
    (25001, REGEX_SSN_NODASHES, r"^\d{9}$");
    /// 25002 Account - word
    (25002, REGEX_ACCOUNT, r"/account|\bacct\b|\bacc\b|\ba/c\b/gim");
    /// 25003 Driver's License - AL
    (25003, REGEX_DRV_LIC_AL, r"^[0-9]{1,7}$");
    /// 25004 Driver's License - AK
    (25004, REGEX_DRV_LIC_AK, r"^[0-9]{1,7}$");
    /// 25005 Driver's License - AZ
    (25005, REGEX_DRV_LIC_AZ, r"(^[A-Z]{1}[0-9]{1,8}$)|(^[A-Z]{2}[0-9]{2,5}$)|(^[0-9]{9}$)");
    /// 25006 Driver's License - AR
    (25006, REGEX_DRV_LIC_AR, r"^[0-9]{4,9}$");
    /// 25007 Driver's License - CA
    (25007, REGEX_DRV_LIC_CA, r"^[A-Z]{1}[0-9]{7}$");
    /// 25008 Driver's License - CO
    (25008, REGEX_DRV_LIC_CO, r"(^[0-9]{9}$)|(^[A-Z]{1}[0-9]{3,6}$)|(^[A-Z]{2}[0-9]{2,5}$)");
    /// 25009 Driver's License - CT
    (25009, REGEX_DRV_LIC_CT, r"^[0-9]{9}$");
    /// 25010 Driver's License - DE
    (25010, REGEX_DRV_LIC_DE, r"^[0-9]{1,7}$");
    /// 25011 Driver's License - DC
    (25011, REGEX_DRV_LIC_DC, r"(^[0-9]{7}$)|(^[0-9]{9}$)");
    /// 25012 Driver's License - FL
    (25012, REGEX_DRV_LIC_FL, r"^[A-Z]{1}[0-9]{12}$");
    /// 25013 Driver's License - GA
    (25013, REGEX_DRV_LIC_GA, r"^[0-9]{7,9}$");
    /// 25014 Driver's License - GU
    (25014, REGEX_DRV_LIC_GU, r"^[A-Z]{1}[0-9]{14}$");
    /// 25015 Driver's License - HI
    (25015, REGEX_DRV_LIC_HI, r"(^[A-Z]{1}[0-9]{8}$)|(^[0-9]{9}$)");
    /// 25016 Driver's License - ID
    (25016, REGEX_DRV_LIC_ID, r"(^[A-Z]{2}[0-9]{6}[A-Z]{1}$)|(^[0-9]{9}$)");
    /// 25017 Driver's License - IL
    (25017, REGEX_DRV_LIC_IL, r"^[A-Z]{1}[0-9]{11,12}$");
    /// 25018 Driver's License - IN
    (25018, REGEX_DRV_LIC_IN, r"(^[A-Z]{1}[0-9]{9}$)|(^[0-9]{9,10}$)");
    /// 25019 Driver's License - IA
    (25019, REGEX_DRV_LIC_IA, r"^([0-9]{9}|([0-9]{3}[A-Z]{2}[0-9]{4}))$");
    /// 25020 Driver's License - KS
    (25020, REGEX_DRV_LIC_KS, r"(^([A-Z]{1}[0-9]{1}){2}[A-Z]{1}$)|(^[A-Z]{1}[0-9]{8}$)|(^[0-9]{9}$)");
    /// 25021 Driver's License - KY
    (25021, REGEX_DRV_LIC_KY, r"(^[A_Z]{1}[0-9]{8,9}$)|(^[0-9]{9}$)");
    /// 25022 Driver's License - LA
    (25022, REGEX_DRV_LIC_LA, r"^[0-9]{1,9}$");
    /// 25023 Driver's License - ME
    (25023, REGEX_DRV_LIC_ME, r"(^[0-9]{7,8}$)|(^[0-9]{7}[A-Z]{1}$)");
    /// 25024 Driver's License - MD
    (25024, REGEX_DRV_LIC_MD, r"^[A-Z]{1}[0-9]{12}$");
    /// 25025 Driver's License - MA
    (25025, REGEX_DRV_LIC_MA, r"(^[A-Z]{1}[0-9]{8}$)|(^[0-9]{9}$)");
    /// 25026 Driver's License - MI
    (25026, REGEX_DRV_LIC_MI, r"(^[A-Z]{1}[0-9]{10}$)|(^[A-Z]{1}[0-9]{12}$)");
    /// 25027 Driver's License - MN
    (25027, REGEX_DRV_LIC_MN, r"^[A-Z]{1}[0-9]{12}$");
    /// 25028 Driver's License - MS
    (25028, REGEX_DRV_LIC_MS, r"^[0-9]{9}$");
    /// 25029 Driver's License - MO
    (25029, REGEX_DRV_LIC_MO, r"(^[A-Z]{1}[0-9]{5,9}$)|(^[A-Z]{1}[0-9]{6}[R]{1}$)|(^[0-9]{8}[A-Z]{2}$)|(^[0-9]{9}[A-Z]{1}$)|(^[0-9]{9}$)");
    /// 25030 Driver's License - MT
    (25030, REGEX_DRV_LIC_MT, r"(^[A-Z]{1}[0-9]{8}$)|(^[0-9]{13}$)|(^[0-9]{9}$)|(^[0-9]{14}$)");
    /// 25031 Driver's License - NE
    (25031, REGEX_DRV_LIC_NE, r"^[0-9]{1,7}$");
    /// 25032 Driver's License - NV
    (25032, REGEX_DRV_LIC_NV, r"(^[0-9]{9,10}$)|(^[0-9]{12}$)|(^[X]{1}[0-9]{8}$)");
    /// 25033 Driver's License - NH
    (25033, REGEX_DRV_LIC_NH, r"^[0-9]{2}[A-Z]{3}[0-9]{5}$");
    /// 250x34Driver's License - NJ
    (25034, REGEX_DRV_LIC_NJ, r"^[A-Z]{1}[0-9]{14}$");
    /// 25035 Driver's License - NM
    (25035, REGEX_DRV_LIC_NM, r"^[0-9]{8,9}$");
    /// 25036 Driver's License - NC
    (25036, REGEX_DRV_LIC_NC, r"^[0-9]{1,12}$");
    /// 25037 Driver's License - ND
    (25037, REGEX_DRV_LIC_ND, r"(^[A-Z]{3}[0-9]{6}$)|(^[0-9]{9}$)");
    /// 25038 Driver's License - OH
    (25038, REGEX_DRV_LIC_OH, r"(^[A-Z]{1}[0-9]{4,8}$)|(^[A-Z]{2}[0-9]{3,7}$)|(^[0-9]{8}$)");
    /// 25039 Driver's License - OK
    (25039, REGEX_DRV_LIC_OK, r"(^[A-Z]{1}[0-9]{9}$)|(^[0-9]{9}$)");
    /// 25040 Driver's License - OR
    (25040, REGEX_DRV_LIC_OR, r"^[0-9]{1,9}$");
    /// 25041 Driver's License - PA
    (25041, REGEX_DRV_LIC_PA, r"^[0-9]{8}$");
    /// 25042 Driver's License - PR
    (25042, REGEX_DRV_LIC_PR, r"(^[0-9]{9}$)|(^[0-9]{5,7}$)");
    /// 25043 Driver's License - RI
    (25043, REGEX_DRV_LIC_RI, r"^([0-9]{7}$)|(^[A-Z]{1}[0-9]{6}$)");
    /// 25044 Driver's License - SC
    (25044, REGEX_DRV_LIC_SC, r"^[0-9]{5,11}$");
    /// 25045 Driver's License - SD
    (25045, REGEX_DRV_LIC_SD, r"(^[0-9]{6,10}$)|(^[0-9]{12}$)");
    /// 25046 Driver's License - TN
    (25046, REGEX_DRV_LIC_TN, r"^[0-9]{7,9}$");
    /// 25047 Driver's License - TX
    (25047, REGEX_DRV_LIC_TX, r"^[0-9]{7,8}$");
    /// 25048 Driver's License - UT
    (25048, REGEX_DRV_LIC_UT, r"^[0-9]{4,10}$");
    /// 25049 Driver's License - VT
    (25049, REGEX_DRV_LIC_VT, r"(^[0-9]{8}$)|(^[0-9]{7}[A]$)");
    /// 25050 Driver's License - VA
    (25050, REGEX_DRV_LIC_VA, r"(^[A-Z]{1}[0-9]{8,11}$)|(^[0-9]{9}$)");
    /// 25051 Driver's License - WA
    (25051, REGEX_DRV_LIC_WA, r"\b[A-Z]{1,7}[\w|*]{12}\b");
    //(25051, REGEX_DRV_LIC_WA, r"(^[A-Z]{1,7}[A-Z0-9\\*]{4,11}$)");
    /// 25052 Driver's License - WV
    (25052, REGEX_DRV_LIC_WV, r"(^[0-9]{7}$)|(^[A-Z]{1,2}[0-9]{5,6}$)");
    /// 25053 Driver's License - WI
    (25053, REGEX_DRV_LIC_WI, r"^[A-Z]{1}[0-9]{13}$");
    /// 25054 Driver's License - WY
    (25054, REGEX_DRV_LIC_WY, r"^[0-9]{9,10}$");
    /// 25055 Gender
    (25055, REGEX_GENDER, r"/\bmale\b|\bfemale\b|\btransgender\b|\bgender\b/gim");
    /// 25056 Titles (e.g.: Mr, Mrs, Ms, etc.)
    (25056, REGEX_TITLE, r"/\bmr\b|\bmrs\b|\bms\b||\bmiss\b|\bdr\b|\brev\b|\bmstr\b|\bprof\b|\bcapt\b|\blady\b|\blord\b|\brabbi\b|\bsir\b|\bmadam\b|\bmx\b/gim");
    /// 25057 Ethnicity
    (25057, REGEX_ETHNICITY, r"/\brace\b|\bwhite\b|\bblack\b|\basian\b|\barab\b|\bhistpanic\b|\bnative\b|\bindian\b|\blatino\b/gim");
    /// 25058 Date of Birth
    (25058, REGEX_DOB, r"/(^[0-9]{1,2}.?[0-9]{1,2}.?[0-9]{2,4})|(^[0-9]{4}.?[0-9]{1,2}.?[0-9]{1,2})/gim");
    /// 25059 Passport
    (25059, REGEX_PASSPORT, r"^[A-Z0-9<]{9}[0-9]{1}[A-Z]{3}[0-9]{7}[A-Z]{1}[0-9]{7}[A-Z0-9<]{14}[0-9]{2}$");
    /// 25060 Access
    (25060, REGEX_ACCESS, r"/\baccess\b/gim");
    /// 25061 Authorize
    (25061, REGEX_AUTHORIZE, r"/authoriz/gim");
    /// 25062 Authenticate
    (25062, REGEX_AUTHENTICATE, r"/authent/gim");
    /// 25063 Auth
    (25063, REGEX_AUTH, r"/\bauth\b/gim");
    /// 25064 Biometric
    (25064, REGEX_BIOMETRIC, r"/biometric/gim");
    /// 25065 Cert
    (25065, REGEX_CERT, r"/certif|\bcert\b/gim");
    /// 25066 Confidential
    (25066, REGEX_CONFIDENTIAL, r"/confidential/gim");
    /// 25067 Config
    (25067, REGEX_CONFIG, r"/config|\bconf\b|\bcfg\b/gim");
    /// 25068 ID
    (25068, REGEX_ID, r"/ident|\bid\b|\buid\b|\buuuid\b/gim");

    /// 26000 Health Terms - On Forms
    (26000, REGEX_HEALTH_FORMS, r"/|health|history|symptom|care|patient|address|referred|chart|bites|insurance|father|mother|sibling|weight|height|age|dob|hospital|primary|treat|physic|phycholog|social|develop|mental|surger|condition|head|ear|nose|sinus|injur/gim");
    /// 26001 Health Terms - Common Symptoms
    (26001, REGEX_HEALTH_SYMPTOMS, r"/flu|vomit|diarrhea|itch|allerg|faint|asthma|neuro|confus|diabetes|pneumo|pregnan|letharg|somnol|amnesia|stupor|polyuria|polydipsia|sympath|seizure|diaphoresis|agitatn|tremor|palpitation|insomnia|sleep|toilet|hypertension|copd|tia|disorder|sickness|kidney|thyroid|skin|muscular|/gim");
    /// 26002 Health Terms - Common Immunization
    (26002, REGEX_HEALTH_IMMUNIZATION, r"/tetanus|pertussis|diphtheria|measles|mumps|rubella|polio|pox|hepatitus|meningitis|influenza|hib|covid/gim");
    /// 26003 Health Terms - Common Equipment
    (26003, REGEX_HEALTH_EQUIP, r"/inject|epipen|tablet|pill/gim");
    /// 26004 Health Terms - Senses
    (26004, REGEX_HEALTH_SENSE, r"/sense|hear|smell|touch|vision|taste|blur|oral/gim");
    /// 26005 Health Terms - Medications
    (26005, REGEX_HEALTH_MEDS, r"/medica|prescript|rx|drug|\bdose\b|\bdoses\b|\bdosage\b|react/gim");
    /// 26006 Health Terms - Blood
    (26006, REGEX_HEALTH_BLOOD, r"/blood|pressure|heart|stroke|disease|hyperglycemic|hemoglobin|cholesterol/gim");
    /// 26007 Health Terms - Respiratory
    (26007, REGEX_HEALTH_RESPIRATORY, r"/breath|mouth|lung|respirat/gim");
    /// 26008 Health Terms - DIET
    (26008, REGEX_HEALTH_DIET, r"/diet|food|restrict|allerg|carb|gluten|diary|peanut|lactose|\bshellfish\b|\bnut\b/gim");

    /// 27000 Credit Card Number - Visa
    (27000, REGEX_CREDIT_VISA, r"4[0-9]{12}(?:[0-9]{3})?");
    /// 27001 Credit Card Number - MasterCard
    (27001, REGEX_CREDIT_MASTER, r"(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}");
    /// 27002 Credit Card Number - AMEX
    (27002, REGEX_CREDIT_AMEX, r"3[47][0-9]{13}");
    /// 27003 Credit Card Number - Diners CLub
    (27003, REGEX_CREDIT_DINERS, r"3(?:0[0-5]|[68][0-9])[0-9]{11}");
    /// 27004 Credit Card Number - Discover
    (27004, REGEX_CREDIT_DISCOVER, r"6(?:011|5[0-9]{2})[0-9]{12}");
    /// 27005 Credit Card Number - JCB
    (27005, REGEX_CREDIT_JCB, r"(?:2131|1800|35\d{3})\d{11}");
    /// 27006 Bank Number - IBAN Czech
    (27006, REGEX_BANK_IBAN_CZECH, r"CZ\d{2}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}|CZ\d{22}");
    /// 27007 Bank Number - IBAN Slovak
    (27007, REGEX_BANK_IBAN_SLOVAK, r"SK\d{2}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}|SK\d{22}");
    /// 27008 Bank Number - IBAN Sweden
    (27008, REGEX_BANK_IBAN_SE, r"SE\d{2}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}|SE\d{22}");
    /// 27009 Bank Number - IBAN Switz
    (27009, REGEX_BANK_IBAN_CH, r"CH\d{2}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{1}|CH\d{19} ");
    /// 27010 Bank Number - IBAN Germany
    (27010, REGEX_BANK_IBAN_DE, r"DE\d{2}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{2}|DE\d{20}");
    /// 27011 Bank Number - IBAN Spain
    (27011, REGEX_BANK_IBAN_ES, r"ES\d{2}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}|ES\d{22}");
    /// 27012 Bank Number - IBAN Poland
    (27012, REGEX_BANK_IBAN_PL, r"PL\d{2}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}|PL\d{26}");
    /// 27013 Bank Number - IBAN Italy
    (27013, REGEX_BANK_IBAN_IT, r"IT\d{2}[ ][a-zA-Z]\d{3}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{4}[ ]\d{3}|IT\d{2}[a-zA-Z]\d{22}");
    /// 27014 Bank Routing Transit Number
    (27014, REGEX_BANK_RTN, r"^((0[0-9])|(1[0-2])|(2[1-9])|(3[0-2])|(6[1-9])|(7[0-2])|80)([0-9]{7})$");
    /// 27015 Bank Code - Germany
    (27015, REGEX_BANK_BLZ, r"[1-8][0-9]{2}[0-9]{5}");
    /// 27016 Bank Sort Code - UK
    (27016, REGEX_BANK_UK_SORT, r"^[0-9]{2}[-][0-9]{2}[-][0-9]{2}$");
    /// 27017 Bank Account Number - UK
    (27017, REGEX_BANK_UK_ACCNT, r"^(\d){8}$");
    /// 27018 Swift Number
    (27018, REGEX_BANK_SWIFT, r"^[a-zA-Z]{4}[a-zA-Z]{2}[a-zA-Z0-9]{2}[XXX0-9]{0,3}");
    /// 27019 Invoice
    (27019, REGEX_INVOICE, r"/invoic|receipt|bill/gim");


    /// 35000 Social Security Number with dashes
    (35000, PTTRN_SSN_DASHES, r"###@##@####");
    /// 35001 Social Security Number without dahses
    (35001, PTTRN_SSN_NODASHES, r"#########");
    /// 35002 Account Camel
    (35002, PTTRN_ACCOUNT_CAMEL, r"Vccvvcc");
    /// 35003 Account Upper
    (35003, PTTRN_ACCOUNT_UPPER, r"VCCVVCC");
    /// 35004 Account Lower
    (35004, PTTRN_ACCOUNT_LOWER, r"vccvvcc");
}

impl InvalidCode {
    fn new() -> InvalidCode {
        InvalidCode { _priv: () }
    }
}

impl fmt::Debug for InvalidCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidCode")
            // skip _priv noise
            .finish()
    }
}

impl fmt::Display for InvalidCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid code")
    }
}

impl Error for InvalidCode {}

/// The codes used in the DPI library are catalogued based on type of codes:
///
/// 1xxxx = Key Words for PII
/// 15xxx = Key Words for NPPI (Non-public Personal Information)
/// 2xxxx = Regular Expressions for PII
/// 25xxx = Regular Expressions for NPPI
/// 26xxx = Regular Expressions for Health
/// 27xxx = Regular Expressions for PCI
/// 3xxxx = Pattern Definitions for PII
/// 35xxx = Pattern Definitions for NPPI
///
impl Lib {
    /// Returns a &str representation of the `Code`
    ///
    /// The return value representation of the code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use pbd::dpi::reference::Lib;
    ///     
    /// let code = Lib::TEXT_SSN_ABBR;
    /// assert_eq!(code.as_str(), Some("SSN"));
    /// ```
    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        get_value(self.0.get())
    }

    /// Returns the `u16` corresponding to this `Code`.
    ///
    /// # Note
    ///
    /// This is the same as the `From<Lib>` implementation, but
    /// included as an inherent method because that implementation doesn't
    /// appear in rustdocs, as well as a way to force the type instead of
    /// relying on inference.
    ///
    /// # Example
    ///
    /// ```rust    
    /// use pbd::dpi::reference::Lib;
    ///     
    /// let code = Lib::TEXT_SSN_ABBR;
    /// assert_eq!(code.as_u16(), 15001);
    /// ```
    #[inline]
    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    /// Converts a &[u8] to a status code
    pub fn from_bytes(src: &[u8]) -> Result<Lib, InvalidCode> {
        let mut src_vec = Vec::new();

        if src.len() != 5 {
            return Err(InvalidCode::new());
        }

        for s in src {
            src_vec.push(s.wrapping_sub(b'0') as u16);
        }

        if src_vec[0] == 0
            || (src_vec[0] > 6
                && src_vec[1] > 5
                && src_vec[2] > 5
                && src_vec[3] > 3
                && src_vec[4] > 5)
        {
            return Err(InvalidCode::new());
        }

        let code = (src_vec[0] * 10000)
            + (src_vec[1] * 1000)
            + (src_vec[2] * 100)
            + (src_vec[3] * 10)
            + (src_vec[4]);
        NonZeroU16::new(code).map(Lib).ok_or_else(InvalidCode::new)
    }

    /// Converts a u16 to a library code.
    ///
    /// The function validates the correctness of the supplied u16. It must be
    /// greater or equal to 10000 and less than 65535.
    ///
    /// # Example
    ///
    /// ```rust
    /// use pbd::dpi::reference::Lib;
    ///
    /// let ssn = Lib::from_u16(15001).unwrap();
    /// assert_eq!(ssn, Lib::TEXT_SSN_ABBR);
    ///
    /// let err = Lib::from_u16(1000);
    /// assert!(err.is_err());
    /// ```
    #[inline]
    pub fn from_u16(src: u16) -> Result<Lib, InvalidCode> {
        if src < 10000 {
            return Err(InvalidCode::new());
        }

        NonZeroU16::new(src).map(Lib).ok_or_else(InvalidCode::new)
    }

    /// Get the standardised `reason-phrase` for this standard.
    ///
    /// This is mostly here for human readable understanding, but could potentially have application
    /// at other times.
    ///
    /// The reason phrase is defined as being exclusively for human readers. You should avoid
    /// deriving any meaning from it at all costs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use pbd::dpi::reference::Lib;
    ///
    /// let code = Lib::TEXT_SSN_ABBR;
    /// assert_eq!(code.get_value(), Some("SSN"));
    /// ```
    pub fn get_value(&self) -> Option<&'static str> {
        get_value(self.0.get())
    }
}

impl fmt::Debug for Lib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Formats the code, *including* the value.
///
/// # Example
///
/// ```rust
/// use pbd::dpi::reference::Lib;
///
/// assert_eq!(format!("{}", Lib::TEXT_SSN_ABBR), "SSN");
/// ```
impl fmt::Display for Lib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_value().unwrap_or("<unknown code>"))
    }
}

impl PartialEq<u16> for Lib {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<Lib> for u16 {
    #[inline]
    fn eq(&self, other: &Lib) -> bool {
        *self == other.as_u16()
    }
}

impl From<Lib> for u16 {
    #[inline]
    fn from(status: Lib) -> u16 {
        status.0.get()
    }
}

impl FromStr for Lib {
    type Err = InvalidCode;

    fn from_str(s: &str) -> Result<Lib, InvalidCode> {
        Lib::from_bytes(s.as_ref())
    }
}

impl<'a> From<&'a Lib> for Lib {
    #[inline]
    fn from(t: &'a Lib) -> Self {
        *t
    }
}

impl<'a> TryFrom<&'a [u8]> for Lib {
    type Error = InvalidCode;

    #[inline]
    fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        Lib::from_bytes(t)
    }
}

impl<'a> TryFrom<&'a str> for Lib {
    type Error = InvalidCode;

    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        t.parse()
    }
}

impl TryFrom<u16> for Lib {
    type Error = InvalidCode;

    #[inline]
    fn try_from(t: u16) -> Result<Self, Self::Error> {
        Lib::from_u16(t)
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        let code = Lib::TEXT_SSN_ABBR;
        assert_eq!(code.as_str(), Some("SSN"));
    }

    #[test]
    fn test_as_u16() {
        let code = Lib::TEXT_SSN_ABBR;
        assert_eq!(code.as_u16(), 15001);
    }

    #[test]
    fn test_debug_code() {
        assert_eq!(format!("{:?}", Lib::TEXT_SSN_ABBR), "15001");
    }

    #[test]
    fn test_display_code() {
        assert_eq!(format!("{}", Lib::TEXT_SSN_ABBR), "SSN");
    }

    #[test]
    fn test_from_bytes() {
        assert!(!Lib::from_bytes("15001".as_bytes()).is_err());
        assert!(!Lib::from_bytes("36666".as_bytes()).is_err());
        assert!(Lib::from_bytes("1000".as_bytes()).is_err());
        assert!(Lib::from_bytes("99999".as_bytes()).is_err());
    }

    #[test]
    fn test_from_str() {
        let ssn = Lib::from_str("15001").unwrap();
        assert_eq!(ssn, Lib::TEXT_SSN_ABBR);

        let err = Lib::from_str("ssn");
        assert!(err.is_err());
    }

    #[test]
    fn test_from_u16() {
        let ssn = Lib::from_u16(15001).unwrap();
        assert_eq!(ssn, Lib::TEXT_SSN_ABBR);

        let err = Lib::from_u16(1000);
        assert!(err.is_err());
    }

    #[test]
    fn test_invalid_code() {
        let invalid_code = InvalidCode::new();
        assert_eq!(format!("{:?}", invalid_code), "InvalidCode");
        assert_eq!(format!("{}", invalid_code), "invalid code");
    }

    #[test]
    fn test_nppi_code() {
        let code = Lib::TEXT_SSN_ABBR;
        assert_eq!(code.get_value(), Some(r"SSN"));
        assert_eq!(Lib::from_u16(15001).unwrap(), Lib::TEXT_SSN_ABBR);
    }

    #[test]
    fn test_basic_list() {
        struct Logic {}
        impl IdentifierLogic for Logic {}
        let lists = Logic::basic_list();

        assert_eq!(lists.len(), 3);
        assert_eq!(lists.get("words").unwrap().len(), 0);
        assert_eq!(lists.get("regexs").unwrap().len(), 17);
        assert_eq!(lists.get("patterns").unwrap().len(), 0);
    }

    #[test]
    fn test_health_list() {
        struct Logic {}
        impl IdentifierLogic for Logic {}
        let lists = Logic::health_list();

        assert_eq!(lists.len(), 3);
        assert_eq!(lists.get("words").unwrap().len(), 0);
        assert_eq!(lists.get("regexs").unwrap().len(), 9);
        assert_eq!(lists.get("patterns").unwrap().len(), 0);
    }

    #[test]
    fn test_nppi_list() {
        struct Logic {}
        impl IdentifierLogic for Logic {}
        let lists = Logic::nppi_list();

        assert_eq!(lists.len(), 3);
        assert_eq!(lists.get("words").unwrap().len(), 3);
        assert_eq!(lists.get("regexs").unwrap().len(), 69);
        assert_eq!(lists.get("patterns").unwrap().len(), 5);
    }

    #[test]
    fn test_pci_list() {
        struct Logic {}
        impl IdentifierLogic for Logic {}
        let lists = Logic::pci_list();

        assert_eq!(lists.len(), 3);
        assert_eq!(lists.get("words").unwrap().len(), 0);
        assert_eq!(lists.get("regexs").unwrap().len(), 20);
        assert_eq!(lists.get("patterns").unwrap().len(), 0);
    }

    #[test]
    fn test_try_from_lib() {
        let try_successful_lib = Lib::try_from(Lib::TEXT_SSN_ABBR);
        assert!(try_successful_lib.is_ok());
    }

    #[test]
    fn test_try_from_str() {
        let try_successful_str = Lib::try_from("15000");
        assert!(try_successful_str.is_ok());
    }

    #[test]
    fn test_try_from_u8() {
        let try_successful_u8 = Lib::try_from("15000".as_bytes());
        assert!(try_successful_u8.is_ok());
    }

    #[test]
    fn test_try_from_u16() {
        let try_successful_u16 = Lib::try_from(15000 as u16);
        assert!(try_successful_u16.is_ok());
    }
}
